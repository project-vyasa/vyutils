# vyasa-phonetics

**Pre-Pāṇinian, Pāṇinian, and Śikṣā phonetic models for Classical & Vedic Sanskrit in Rust.**

Part of the [`vyutils`](https://github.com/project-vyasa/vyutils) project.

[![Documentation](https://img.shields.io/badge/docs-vyutils-blue)](https://project-vyasa.github.io/vyutils/reference/vyasa-phonetics/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

---

## Overview

Existing computational Sanskrit tools almost universally assume Classical Pāṇinian phonology and strip or ignore Vedic pitch accents and pre-Pāṇinian phonetic sequences. `vyasa-phonetics` provides foundational, zero-runtime-dependency Rust types and algorithms that model:

1. **Pre-Pāṇinian Phonetics (Śaunaka's *Ṛgveda-Prātiśākhya*)**:
   - The Śaiśirīya vowel sequence: `a, ṛ, i, u, e, o, ai, au` (reflecting vocal tract expansion from velar to labial).
   - Identification of *Samānākṣara* (simple vowels) vs *Sandhyakṣara* (diphthongs).
   - *Nāmin* vowels: all vowels except `a` and `ā`, which trigger retroflexion (*Nati*, s ⟶ ṣ and n ⟶ ṇ) per RPr. 1.65.
   - Mātrā (moraic timing) calculation: *Hrasva* (1), *Dīrgha* (2), *Pluta* (3).

2. **Pāṇinian Phonology (*Aṣṭādhyāyī* & *Śiva Sūtras*)**:
   - The 14 *Māheśvara Sūtras* (*a-i-u-ṇ*, *ṛ-ḷ-k*, *e-o-ṅ*, *ai-au-c*, etc.).
   - Dynamic *Pratyāhāra* decoder (Pāṇini 1.1.71 *ādir antyena sahetā*), evaluating standard abbreviations such as `ac` (all vowels), `hal` (all consonants), `al` (all sounds), `ik`, `uk`, `aṇ`, `yaṇ`, `khar`, `jaś`, and `val`.

3. **Śikṣā Articulatory Phonetics (*Pāṇinīya Śikṣā*)**:
   - **Sthāna** (Places of articulation): *Kaṇṭha* (velar), *Tālu* (palatal), *Mūrdhan* (retroflex), *Danta* (dental), *Oṣṭha* (labial), *Kaṇṭhatālu*, *Kaṇṭhoṣṭha*, *Dantoṣṭha*, and *Nāsikā*.
   - **Ābhyantara Prayatna** (Internal effort): *Spṛṣṭa* (stops), *Īṣatspṛṣṭa* (semivowels), *Vivṛta* (vowels), *Saṁvṛta* (closed short *a*).
   - **Bāhya Prayatna** (External effort): *Śvāsa*, *Nāda*, *Aghoṣa*, *Ghoṣa*, *Alpaprāṇa*, *Mahāprāṇa*, and Vedic pitch accents (*Udātta*, *Anudātta*, *Svarita*).

4. **Vedic Sound Inventory**:
   - Complete support for Vedic flaps: ळ (*LVedic* / ḍa-kāra-sthāne) and ळ्ह (*LhVedic* / ḍha-kāra-sthāne).
   - Specialized Vedic *Ayogavāhas*: *Jihvāmūlīya* (≍ k), *Upadhmānīya* (≍ p), *Gomukha*, *Dvibindu*, and *Ardhavisarga*.

---

## Architectural Invariants

- **Zero External Dependencies**: Compiles cleanly with `no_std` + `alloc` support for immediate WebAssembly portability.
- **Strict Distinction of Homoglyphs**: Clear differentiation between vocalic liquids (`L`, `ऌ`) and retroflex consonants (`LVedic`, `ळ`), avoiding the ambiguous conflations present in older toolkits.
- **Suprasegmental Pitch Accents**: Vedic accents (*Svara*) are first-class phonetic properties attached to vowels and aksharas, not treated as extraneous non-spacing diacritics.

---

## Quick Start

Add `vyasa-phonetics` to your `Cargo.toml`:

```toml
[dependencies]
vyasa-phonetics = { path = "../crates/vyasa-phonetics" }
```

### 1. Pāṇinian Pratyāhāras

```rust
use vyasa-phonetics::panini::{MAHESHWARA_SUTRAS, evaluate_pratyahara};
use vyasa-phonetics::sound::{Varna, Vowel};

// Check if a vowel belongs to the 'ik' pratyāhāra (i, u, ṛ, ḷ)
let ik_vowels = evaluate_pratyahara("ik").expect("Valid pratyahara");
assert!(ik_vowels.contains(&Varna::Vowel(Vowel::ShortI)));
assert!(ik_vowels.contains(&Varna::Vowel(Vowel::ShortU)));
assert!(!ik_vowels.contains(&Varna::Vowel(Vowel::ShortA)));

// 'ac' matches all vowels
let ac = evaluate_pratyahara("ac").unwrap();
assert_eq!(ac.len(), 9); // a, i, u, ṛ, ḷ, e, o, ai, au
```

### 2. Pre-Pāṇinian Ṛgveda-Prātiśākhya Rules

```rust
use vyasa-phonetics::pratisakhya::{is_namin, is_sandhyaksara, matra_count, SAISIRIYA_VOWELS};
use vyasa-phonetics::sound::Vowel;

// Nāmin vowels trigger retroflexion (Nati) of s -> ṣ and n -> ṇ
assert!(is_namin(Vowel::ShortI));
assert!(is_namin(Vowel::ShortU));
assert!(!is_namin(Vowel::ShortA)); // 'a' is not a Nāmin vowel

// Moraic count
assert_eq!(matra_count(Vowel::ShortA), 1);
assert_eq!(matra_count(Vowel::LongA), 2);

// Śaiśirīya acoustic vowel sequence
assert_eq!(SAISIRIYA_VOWELS[0], Vowel::ShortA);
assert_eq!(SAISIRIYA_VOWELS[1], Vowel::ShortR);
assert_eq!(SAISIRIYA_VOWELS[2], Vowel::ShortI);
```

### 3. Articulatory Phonetics (Śikṣā)

```rust
use vyasa-phonetics::articulatory::{AbhyantaraPrayatna, BahyaPrayatna, Sthana};
use vyasa-phonetics::sound::{Consonant, Varna};

let ka = Varna::Consonant(Consonant::Ka);

// Sthāna (Place of articulation)
assert_eq!(ka.sthana(), Sthana::Kantha); // Velar / Throat

// Ābhyantara Prayatna (Internal effort)
assert_eq!(ka.abhyantara_prayatna(), AbhyantaraPrayatna::Sprsta); // Contact / Stop

// Bāhya Prayatna (External effort)
let bahya = ka.bahya_prayatna();
assert!(bahya.contains(&BahyaPrayatna::Aghosa));    // Unvoiced
assert!(bahya.contains(&BahyaPrayatna::Alpapranah)); // Unaspirated
```

---

## Documentation

Full architectural documentation, linguistic treatises, and API specifications are available at:
- **[Sanskrit Phonetics Explanation](https://project-vyasa.github.io/vyutils/explanation/sanskrit-phonetics/)**
- **[RFC-0002: Sanskrit Processing Engine](https://project-vyasa.github.io/vyutils/rfcs/rfc-0002-sanskrit-processing-engine/)**
- **[Crate API Reference](https://project-vyasa.github.io/vyutils/reference/vyasa-phonetics/)**
