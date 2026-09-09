---
title: "RFC-0002: Sanskrit Processing Engine (vyasa-sanskrit)"
description: Architecture specification for Vedic & Classical Sanskrit phonetic models, multi-script transliteration, and recitation mode generators.
---

## Status

**Accepted / In Implementation** (Phases 1 & 2 Complete)

## Summary

This RFC specifies the architecture of **`vyasa-sanskrit`**, a suite of modular, zero-dependency Rust crates inside the `vyutils` workspace (`vyasa-phonetics`, `vyasa-lipi`, `vyasa-patha`). The suite supports:

1. **Pre-Pāṇinian & Pāṇinian Phonetics**: Śaunaka's *Ṛgveda-Prātiśākhya* (Śaiśirīya sequence), Pāṇini's 14 *Māheśvara Sūtras* with dynamic *Pratyāhāra* computation, and *Śikṣā* articulatory classifications.
2. **Universal Vedic Multi-Script Transliteration**: Lossless round-trip conversion across Indic abugidas (Devanagari, Telugu, Kannada, Grantha, Malayalam, Bengali), Roman transliteration schemes (Strict ISO 15919, Standard IAST), and ASCII formats (SLP1, Harvard-Kyoto, WX) with suprasegmental pitch accent (*svara*) preservation.
3. **Recitation Mode Generation**: Mathematical generation of *Prakṛti* (*Saṁhitā*, *Pada*, *Krama*) and *Aṣṭa-Vikṛti* (*Jaṭā*, *Ghana*, etc.) pāṭhas with strict *Pragṛhya* detection and *Parigraha* (*iti*) clauses.

---

## Motivation & Gap Analysis

Existing computational Sanskrit tools (e.g. *indic_transliteration*, *sanscript.js*, *Heritage du Sanskrit*) are almost exclusively designed around unaccented classical Sanskrit. They fail when applied to Vedic literature due to four critical gaps:

### Gap 1: Vedic Accent Corruption
In Vedic recitation, every vowel possesses a tone: *Udātta* (high), *Anudātta* (low, `\u0952`), or *Svarita* (falling, `\u0951`). Existing tools treat combining accent marks as non-spacing noise or strip them entirely. `vyasa-sanskrit` treats pitch accents as first-class phonetic properties of the syllable (*Akṣara*).

### Gap 2: Conflation of ISO 15919 with IAST
Standard IAST does not distinguish short and long *e* and *o*, nor does it accurately represent vocalic liquids or the Vedic retroflex lateral flap (ळ). Strict **ISO 15919** uses under-rings (`r̥`, `r̥̄`, `l̥`, `l̥̄`) and explicit macrons (`ē`, `ō`), whereas **IAST** uses under-dots (`ṛ`, `ṝ`, `ḷ`, `ḹ`) and bare vowels (`e`, `o`). `vyasa-lipi` enforces this distinction strictly.

### Gap 3: Absence of Prātiśākhya Sequences
Classical toolkits assume Pāṇini's *Śiva Sūtras* (*a-i-u-ṇ*...). The *Ṛgveda-Prātiśākhya* formulates rules around the Śaiśirīya order (*a, ṛ, i, u, e, o, ai, au*) and defines *Nāmin* vowels (all vowels except *a* and *ā*) which trigger retroflexion (*Nati*). `vyasa-phonetics` implements these pre-Pāṇinian models directly.

### Gap 4: Naive Recitation Generation
Existing pāṭha scripts perform naive string concatenation without applying forward/reverse sandhi, without recomputing Vedic accents, and without inserting canonical **"iti" clauses** (*Parigraha* / *Sthita-Upasthita*) for *Pragṛhya* vowels.

---

## Architecture of Workspace Crates

```text
crates/
├── vyasa-phonetics/     # Sound representations, Prātiśākhya & Pāṇinian models
├── vyasa-lipi/          # Universal multi-script transliteration engine
└── vyasa-patha/         # Recitation generator (Prakṛti & Aṣṭa-Vikṛti pāṭhas)
```

### Invariants
1. **Zero Runtime Dependencies**: All crates compile with `no_std` + `alloc` support to guarantee immediate compilation and WebAssembly export to `vyasa-viewer` (`vyasav`).
2. **Lossless Round-Trip Guarantee**: Transliterating text A → B → A produces byte-identical strings for all supported scripts.
3. **Akṣara Intermediate Representation (IR)**: Transliteration is performed via a structured phonemic AST, never ad-hoc regex cascades.

---

## Roadmap

- **Phase 1 (Complete)**: `vyasa-phonetics` (Śaiśirīya sequence, Pratyāhāras, Śikṣā) + `vyasa-lipi` (Devanagari, Telugu, Kannada, ISO 15919, IAST).
- **Phase 2 (Complete)**: Expanded Indic scripts (Grantha, Malayalam, Bengali) + ASCII schemes (SLP1, Harvard-Kyoto, WX).
- **Phase 3**: `vyasa-patha` for Prakṛti pāṭhas (*Saṁhitā*, *Pada*, *Krama*) with *Pragṛhya* detection and *Parigraha* (*iti*) clauses.
- **Phase 4**: Aṣṭa-Vikṛti generators (*Jaṭā*, *Ghana*, etc.) with forward/reverse Vedic sandhi.
- **Phase 5**: WASM build and Svelte UI components in `vyasa-viewer`.
