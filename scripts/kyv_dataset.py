#!/usr/bin/env python3
"""Reusable utility script for inspecting, validating, and managing the Krishna Yajurveda

(Taittirīya-Prātiśākhya) phonological corpus.
"""

import argparse
import json
import os
import sys

DEFAULT_CORPUS_JSON = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "crates/vyasa-phonetics/tests/data/kyv_corpus.json"
)


def load_corpus(path=DEFAULT_CORPUS_JSON):
    if not os.path.exists(path):
        print(f"Error: Corpus file not found at {path}", file=sys.stderr)
        sys.exit(1)
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def inspect_corpus(corpus):
    print("=" * 70)
    print(f"  {corpus.get('collection', 'Krishna Yajurveda Corpus')}")
    print(f"  Authority: {corpus.get('pratisakhya', 'Taittiriya-Pratisakhya')}")
    print("=" * 70)

    total_sections = 0
    total_svaritas = 0
    total_dvitva = 0
    svarita_counts = {}

    for text in corpus.get("texts", []):
        text_name = text.get("text_name")
        abbr = text.get("abbreviation")
        sections = text.get("sections", [])
        num_sections = len(sections)
        total_sections += num_sections

        print(f"\n📖 Text: {text_name} ({abbr}) — {num_sections} passages:")
        for sec in sections:
            sec_id = sec.get("id")
            title = sec.get("title")
            svaritas = sec.get("svaritas", [])
            dvitva = sec.get("dvitva", [])
            total_svaritas += len(svaritas)
            total_dvitva += len(dvitva)

            for sv in svaritas:
                exp = sv.get("expected")
                svarita_counts[exp] = svarita_counts.get(exp, 0) + 1

            print(f"   • [{sec_id}] {title}")
            print(f"     Mantra: {sec.get('mantra')[:65]}...")
            print(f"     Svarita tests: {len(svaritas)}, Dvitva tests: {len(dvitva)}")

    print("\n" + "-" * 70)
    print("Corpus Summary:")
    print(f"  • Primary Texts: {len(corpus.get('texts', []))}")
    print(f"  • Total Liturgical Passages: {total_sections}")
    print(f"  • Total Svarita Test Cases: {total_svaritas}")
    print(f"  • Total Dvitva (Consonant Doubling) Test Cases: {total_dvitva}")
    print("\nSvarita Breakdown (Taittirīya-Prātiśākhya Ch. 20):")
    for stype, count in sorted(svarita_counts.items(), key=lambda x: -x[1]):
        print(f"  • {stype:16}: {count} occurrences")
    print("=" * 70)


def validate_corpus(corpus):
    valid_svarita_types = {
        "Jatya", "Kshaipra", "Abhinihita", "Prashlishta",
        "Tairovyanjana", "Tairovirama", "Padavrtta", "Tathabhavya"
    }
    valid_contexts = {
        "InternalSemivowelStem", "SemivowelSandhi", "AbhinihitaElision",
        "CoalescentLongVowel", "PostUdattaConsonant", "HiatusWithoutSandhi",
        "AcrossVirama"
    }

    errors = []
    texts = corpus.get("texts", [])
    if not texts:
        errors.append("No texts found in corpus.")

    for t_idx, text in enumerate(texts):
        tname = text.get("text_name", f"Text #{t_idx}")
        for s_idx, sec in enumerate(text.get("sections", [])):
            sid = sec.get("id", f"Section #{s_idx}")
            if not sec.get("mantra"):
                errors.append(f"{tname} -> {sid}: Missing 'mantra' text.")
            for sv in sec.get("svaritas", []):
                exp = sv.get("expected")
                ctx = sv.get("context")
                if exp not in valid_svarita_types:
                    errors.append(f"{tname} -> {sid}: Invalid expected svarita type '{exp}'.")
                if ctx not in valid_contexts:
                    errors.append(f"{tname} -> {sid}: Invalid context '{ctx}'.")

    if errors:
        print(f"Validation FAILED with {len(errors)} errors:", file=sys.stderr)
        for err in errors:
            print(f"  ❌ {err}", file=sys.stderr)
        return False
    else:
        print("✅ Corpus validation PASSED. Schema and references conform to TPr specifications.")
        return True


def main():
    parser = argparse.ArgumentParser(
        description="Inspect and validate the Krishna Yajurveda (Taittirīya) phonological corpus."
    )
    subparsers = parser.add_subparsers(dest="command", help="Available subcommands")

    inspect_p = subparsers.add_parser("inspect", help="Inspect corpus statistics and passages")
    inspect_p.add_argument("--file", default=DEFAULT_CORPUS_JSON, help="Path to kyv_corpus.json")

    validate_p = subparsers.add_parser("validate", help="Validate schema and phonological types")
    validate_p.add_argument("--file", default=DEFAULT_CORPUS_JSON, help="Path to kyv_corpus.json")

    args = parser.parse_args()

    if args.command == "validate":
        corpus = load_corpus(args.file)
        success = validate_corpus(corpus)
        sys.exit(0 if success else 1)
    else:
        file_path = getattr(args, "file", DEFAULT_CORPUS_JSON)
        corpus = load_corpus(file_path)
        inspect_corpus(corpus)


if __name__ == "__main__":
    main()
