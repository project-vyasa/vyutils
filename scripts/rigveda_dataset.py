#!/usr/bin/env python3
"""Reusable utility script for extracting, updating, and validating Rigveda test datasets

from the Wikisource processing pipeline (sa.wikisource.org/data/processed/rigveda).
"""

import argparse
import glob
import json
import os
import re
import sys

DEFAULT_PIPELINE = "/Users/anand/Projects/project-vyasa/sa.wikisource.org/data/processed/rigveda"
DEFAULT_OUTPUT_JSON = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "crates/vyasa-patha/tests/data/rv_curated_tricky.json"
)

CURATED_SUKTAS = [
    {"mandala": "01", "sukta": "001", "name": "Agni Sūkta", "category": "Phonological Accent Shift & Parigraha"},
    {"mandala": "01", "sukta": "002", "name": "Vāyu & Indra-Vāyu", "category": "Dual Pragṛhyas & Compound Parigraha"},
    {"mandala": "01", "sukta": "032", "name": "Indra Vṛtra-vadha", "category": "Epic Narrative & Vedic Consonants (ळ/ळ्ह)"},
    {"mandala": "01", "sukta": "164", "name": "Asya Vāmasya", "category": "Deep Multi-Member Compounds & Phonological Riddles"},
    {"mandala": "02", "sukta": "012", "name": "Indra Sūkta (Gṛtsamada)", "category": "Pronoun Visarga Dropping Refrain (स जनास इन्द्रः)"},
    {"mandala": "03", "sukta": "062", "name": "Viśvāmitra (incl. Gāyatrī)", "category": "Sacred Metrical Cadence & Relative Pronoun Sandhi"},
    {"mandala": "06", "sukta": "069", "name": "Indrā-Viṣṇū Dual Hymn", "category": "High-Density Dual Pragṛhya Combinations"},
    {"mandala": "07", "sukta": "059", "name": "Maruts (incl. Mahāmṛtyuñjaya)", "category": "Metrical Transitions & Boundary Parigraha"},
    {"mandala": "10", "sukta": "090", "name": "Puruṣa Sūkta", "category": "Liturgical Crown Jewel & Neuter Virāma Sandhi"},
    {"mandala": "10", "sukta": "125", "name": "Devī Sūkta (Vāk Āmbhṛṇī)", "category": "Consonant Assimilation & Sovereign Accentuation"},
    {"mandala": "10", "sukta": "129", "name": "Nāsadīya Sūkta", "category": "Negative Particle Sandhi & Interrogative Pluta"}
]


def parse_vy_file(file_path):
    """Parses a .vy file and returns a dictionary of rik_number -> normalized text."""
    if not os.path.exists(file_path):
        return {}
    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()
    
    verses = {}
    for v_num_str, v_body in re.findall(r"`v\s+(\d+)\s+\[(.*?)\]", content, re.DOTALL):
        v_num = int(v_num_str)
        clean = re.sub(r"॥\d+\s*$", "", v_body.strip()).strip()
        normalized = " ".join(clean.split())
        verses[v_num] = normalized
    return verses


def build_tricky_dataset(pipeline_dir, output_path):
    """Builds the complete curated tricky dataset from the pipeline."""
    padapatha_root = os.path.join(pipeline_dir, "content/padapatha")
    samhitapatha_root = os.path.join(pipeline_dir, "content/samhita")

    dataset = []
    for target in CURATED_SUKTAS:
        m = target["mandala"]
        s = target["sukta"]
        name = target["name"]
        cat = target["category"]

        pada_file = os.path.join(padapatha_root, m, f"{s}.vy")
        samhita_file = os.path.join(samhitapatha_root, m, f"{s}.vy")

        pada_verses = parse_vy_file(pada_file)
        samhita_verses = parse_vy_file(samhita_file)

        for rik_num in sorted(pada_verses.keys()):
            p_text = pada_verses[rik_num]
            if not p_text.endswith("।") and not p_text.endswith("॥"):
                p_text += " ॥"

            dataset.append({
                "mandala": int(m),
                "sukta": int(s),
                "rik": rik_num,
                "hymn_name": name,
                "category": cat,
                "padapatha": p_text,
                "samhitapatha": samhita_verses.get(rik_num, "")
            })

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(dataset, f, ensure_ascii=False, indent=2)

    print(f"✓ Built {len(dataset)} curated verses across {len(CURATED_SUKTAS)} Sūktas into {output_path}")
    return dataset


def inspect_dataset(dataset_path, sample_count=5):
    """Prints a summary and sample entries from the dataset."""
    if not os.path.exists(dataset_path):
        print(f"Dataset not found at {dataset_path}")
        return

    with open(dataset_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    print(f"=== Dataset Inspection: {dataset_path} ===")
    print(f"Total Verses: {len(data)}")

    # Distinct hymns
    hymns = {}
    for item in data:
        key = (item["mandala"], item["sukta"], item["hymn_name"], item["category"])
        hymns[key] = hymns.get(key, 0) + 1

    print("\nHymns included:")
    for (m, s, name, cat), count in hymns.items():
        print(f"  - RV {m:02d}.{s:03d} {name:30} ({count:2d} verses) [{cat}]")

    step = max(1, len(data) // sample_count)
    print(f"\nSamples (every {step}th verse):")
    for i in range(0, len(data), step)[:sample_count]:
        e = data[i]
        print(f"\n  [RV {e['mandala']}.{e['sukta']}.{e['rik']} - {e['hymn_name']}]")
        print(f"    Pada:    {e['padapatha'][:65]}...")
        if e.get("samhitapatha"):
            print(f"    Samhita: {e['samhitapatha'][:65]}...")


def add_or_replace_verse(pipeline_dir, dataset_path, mandala, sukta, rik=None, category="Custom Addition"):
    """Adds or replaces specific verses from the pipeline into the target dataset."""
    padapatha_file = os.path.join(pipeline_dir, f"content/padapatha/{mandala:02d}/{sukta:03d}.vy")
    samhita_file = os.path.join(pipeline_dir, f"content/samhita/{mandala:02d}/{sukta:03d}.vy")

    pada_verses = parse_vy_file(padapatha_file)
    sam_verses = parse_vy_file(samhita_file)

    if not pada_verses:
        print(f"Error: No padapatha verses found at {padapatha_file}")
        return

    dataset = []
    if os.path.exists(dataset_path):
        with open(dataset_path, "r", encoding="utf-8") as f:
            dataset = json.load(f)

    # Filter which riks to extract
    target_riks = [rik] if rik is not None else sorted(pada_verses.keys())

    added_count = 0
    updated_count = 0

    for r in target_riks:
        if r not in pada_verses:
            print(f"Warning: Rik {r} not found in Sukta {mandala}.{sukta}")
            continue

        p_text = pada_verses[r]
        if not p_text.endswith("।") and not p_text.endswith("॥"):
            p_text += " ॥"

        entry = {
            "mandala": mandala,
            "sukta": sukta,
            "rik": r,
            "hymn_name": f"RV {mandala}.{sukta}",
            "category": category,
            "padapatha": p_text,
            "samhitapatha": sam_verses.get(r, "")
        }

        # Check if already exists
        existing_idx = next(
            (i for i, x in enumerate(dataset) if x["mandala"] == mandala and x["sukta"] == sukta and x["rik"] == r),
            None
        )

        if existing_idx is not None:
            dataset[existing_idx] = entry
            updated_count += 1
        else:
            dataset.append(entry)
            added_count += 1

    # Sort dataset by mandala, sukta, rik
    dataset.sort(key=lambda x: (x["mandala"], x["sukta"], x["rik"]))

    with open(dataset_path, "w", encoding="utf-8") as f:
        json.dump(dataset, f, ensure_ascii=False, indent=2)

    print(f"✓ Success: Added {added_count}, Updated {updated_count} verses in {dataset_path} (Total: {len(dataset)})")


def main():
    parser = argparse.ArgumentParser(description="Rigveda Test Dataset Sync Utility")
    subparsers = parser.add_subparsers(dest="command", required=True)

    # build-tricky
    p_build = subparsers.add_parser("build-tricky", help="Rebuild the curated tricky dataset")
    p_build.add_argument("--pipeline", default=DEFAULT_PIPELINE, help="Pipeline processed root directory")
    p_build.add_argument("--output", default=DEFAULT_OUTPUT_JSON, help="Output JSON path")

    # inspect
    p_inspect = subparsers.add_parser("inspect", help="Inspect an existing dataset")
    p_inspect.add_argument("--dataset", default=DEFAULT_OUTPUT_JSON, help="Dataset JSON path")
    p_inspect.add_argument("--count", type=int, default=5, help="Number of samples to print")

    # add
    p_add = subparsers.add_parser("add", help="Add or update verse(s) from pipeline into dataset")
    p_add.add_argument("--pipeline", default=DEFAULT_PIPELINE, help="Pipeline processed root directory")
    p_add.add_argument("--dataset", default=DEFAULT_OUTPUT_JSON, help="Target dataset JSON path")
    p_add.add_argument("-m", "--mandala", type=int, required=True, help="Mandala number (1-10)")
    p_add.add_argument("-s", "--sukta", type=int, required=True, help="Sukta number (1-191)")
    p_add.add_argument("-r", "--rik", type=int, default=None, help="Specific Rik number (omitted = all riks in sukta)")
    p_add.add_argument("-c", "--category", default="Curated Addition", help="Phonological category description")

    args = parser.parse_args()

    if args.command == "build-tricky":
        build_tricky_dataset(args.pipeline, args.output)
    elif args.command == "inspect":
        inspect_dataset(args.dataset, args.count)
    elif args.command == "add":
        add_or_replace_verse(args.pipeline, args.dataset, args.mandala, args.sukta, args.rik, args.category)


if __name__ == "__main__":
    main()
