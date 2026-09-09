---
title: vyasa-phonetics Crate Reference
description: Rust developer API reference for vyasa-phonetics sound representations, Prātiśākhya functions, and Pāṇinian Pratyāhāras.
---

`vyasa-phonetics` provides foundational, zero-runtime-dependency Rust types and algorithms for Vedic and Classical Sanskrit phonetics.

---

## Crate Organization

```text
vyasa-phonetics
├── sound          # Varṇa, Vowel, Consonant, Ayogavāha, Svara representations
├── pratisakhya    # Śaunaka's Ṛgveda-Prātiśākhya (Śaiśirīya order, Nāmin, Mātrā)
├── panini         # 14 Māheśvara Sūtras & dynamic Pratyāhāra engine
├── articulatory   # Śikṣā Sthāna, Ābhyantara & Bāhya Prayatna classifications
└── varnamala      # Standard pan-Indic Laukika Varṇamālā sequences
```

---

## Module: `sound`

### Enums

#### `Varna`
Represents an individual Sanskrit phoneme:

```rust
pub enum Varna {
    Vowel(Vowel),
    Consonant(Consonant),
    Ayogavaha(Ayogavaha),
}
```

#### `Vowel`
Full inventory of short, long, and pluta vowels, plus Dravidian shorts:

```rust
pub enum Vowel {
    ShortA, LongA,
    ShortI, LongI,
    ShortU, LongU,
    ShortR, LongR,
    ShortL, LongL,
    DravidianShortE, LongE,
    Ai,
    DravidianShortO, LongO,
    Au,
}
```

#### `Consonant`
All 33 classical consonants, plus Vedic retroflex flaps:

```rust
pub enum Consonant {
    // Ka-varga (Velar)
    Ka, Kha, Ga, Gha, Nga,
    // Ca-varga (Palatal)
    Ca, Cha, Ja, Jha, Nya,
    // Ṭa-varga (Retroflex)
    Tta, Ttha, Dda, Ddha, Nna,
    // Ta-varga (Dental)
    Ta, Tha, Da, Dha, Na,
    // Pa-varga (Labial)
    Pa, Pha, Ba, Bha, Ma,
    // Antastha (Semivowels)
    Ya, Ra, La, Va,
    // Ūṣman (Sibilants & Glottal)
    Sha, Ssa, Sa, Ha,
    // Vedic flaps
    LVedic,  // ळ (ḍa-kāra-sthāne)
    LhVedic, // ळ्ह (ḍha-kāra-sthāne)
}
```

#### `Svara`
Vedic suprasegmental pitch accents:

```rust
pub enum Svara {
    Udatta,
    Anudatta,
    Svarita,
    DirghaSvarita,
}
```

#### `Ayogavaha`
Specialized carrying sounds:

```rust
pub enum Ayogavaha {
    Anusvara,
    Visarga,
    Jihvamuliya,
    Upadhmaniya,
    Gomukha,
    Dvibindu,
    Ardhavisarga,
}
```

---

## Module: `pratisakhya`

Implements phonological rules from Sage Śaunaka's *Ṛgveda-Prātiśākhya*.

### Constants

```rust
pub const SAISIRIYA_VOWELS: [Vowel; 8] = [
    Vowel::ShortA,
    Vowel::ShortR,
    Vowel::ShortI,
    Vowel::ShortU,
    Vowel::LongE,
    Vowel::LongO,
    Vowel::Ai,
    Vowel::Au,
];
```

### Functions

#### `is_namin(v: Vowel) -> bool`
Returns `true` if the vowel belongs to the *Nāmin* class (all vowels other than a and ā, per RPr. 1.65). *Nāmin* vowels trigger retroflexion (*Nati*) of dental s and n.

#### `is_samanaksara(v: Vowel) -> bool`
Returns `true` if the vowel is a simple, homogeneous vowel (a, ā, ṛ, ṝ, i, ī, u, ū, ḷ).

#### `is_sandhyaksara(v: Vowel) -> bool`
Returns `true` if the vowel is a composite diphthong (e, o, ai, au).

#### `matra_count(v: Vowel) -> u8`
Returns the moraic value:
- Short vowels: `1`
- Long vowels / diphthongs: `2`

---

## Module: `panini`

Implements Pāṇini's 14 *Māheśvara Sūtras* and dynamic Pratyāhāra decoder.

### Constants

```rust
pub const MAHESHWARA_SUTRAS: [&str; 14] = [
    "a-i-u-ṇ",
    "ṛ-ḷ-k",
    "e-o-ṅ",
    "ai-au-c",
    "ha-ya-va-ra-ṭ",
    "la-ṇ",
    "ña-ma-ṅa-ṇa-na-m",
    "jha-bha-ñ",
    "gha-ḍha-dha-ṣ",
    "ja-ba-ga-ḍa-da-ś",
    "kha-pha-cha-ṭha-tha-ca-ṭa-ta-v",
    "ka-pa-y",
    "śa-ṣa-sa-r",
    "ha-l",
];
```

### Functions

#### `evaluate_pratyahara(name: &str) -> Option<Vec<Varna>>`
Expands any valid Pāṇinian two-letter abbreviation per rule 1.1.71 (*ādir antyena sahetā*):

```rust
use vyasa_phonetics::panini::evaluate_pratyahara;

let ik = evaluate_pratyahara("ik").unwrap();
// Returns [Vowel(ShortI), Vowel(ShortU), Vowel(ShortR), Vowel(ShortL)]

let hal = evaluate_pratyahara("hal").unwrap();
// Returns all 33 consonants
```

---

## Module: `articulatory`

Implements the *Śikṣā* classification system.

### Traits

```rust
pub trait ArticulatoryFeatures {
    fn sthana(&self) -> Sthana;
    fn abhyantara_prayatna(&self) -> AbhyantaraPrayatna;
    fn bahya_prayatna(&self) -> Vec<BahyaPrayatna>;
}
```

Implemented for `Varna`, `Vowel`, and `Consonant`.

---

## Invariants & Guarantees

1. **`no_std` + `alloc`**: Zero external dependencies, enabling direct compilation to WebAssembly.
2. **Zero Heap Allocation in Hot Paths**: Phoneme inspection, classification, and table lookups operate on copyable stack enums.
