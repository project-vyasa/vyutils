---
title: vyasa-patha CLI & API Reference
description: Command-line reference, options, flags, and Rust API documentation for vyasa-patha.
---

`vyasa-patha` is the recitation engine in `vyutils` for generating Vedic and Classical Sanskrit recitation modes (Prakṛti and Vikṛti pāṭhas). It parses Pada-pāṭha texts, detects Pāṇinian Pragṛhya vowels, formats Parigraha (*iti*) clauses, applies forward Sandhi, and outputs multi-script recitations.

---

## Installation

### Building from Source

```bash
cargo build --release -p vyasa-patha
cargo install --path crates/vyasa-patha
```

Binary location: `~/.cargo/bin/vyasa-patha`

---

## CLI Synopsis

```bash
vyasa-patha [OPTIONS] [TEXT]
cat <pada_file.txt> | vyasa-patha [OPTIONS]
```

If `[TEXT]` is provided as an argument, it is processed directly. If omitted, `vyasa-patha` reads Pada-pāṭha from `stdin` until EOF.

---

## CLI Options & Flags

| Flag | Long Option | Values | Default | Description |
|:---|:---|:---|:---|:---|
| `-m` | `--mode` | `krama` \| `pada` | `krama` | Recitation mode to generate. |
| `-t` | `--to` | `<SCRIPT>` | `devanagari` | Target script format (e.g. `telugu`, `kannada`, `grantha`, `iast`, `iso15919`). |
| `-s` | `--steps` | flag | off | Emit numbered steps line-by-line rather than continuous text. |
| `-h` | `--help` | — | — | Print help information and exit. |
| `-V` | `--version` | — | — | Print version information and exit. |

---

## CLI Examples

### 1. Generating Continuous Krama-pāṭha

```bash
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् । य॒ज्ञस्य॑ । दे॒वम् । ऋ॒त्विज॑म् ।"
```

Output:
```text
अ॒ग्निमी॒ळे॒ । ई॒ळे॒ पु॒रो-हि॑तम् । पु॒रो-हि॑तं य॒ज्ञस्य॑ । य॒ज्ञस्य॑ दे॒वम् । दे॒वमृ॒त्विज॑म् । ऋ॒त्विग् इति॑ ऋ॒त्विज॑म् ॥
```

### 2. Stepped Pairwise Output (`--steps`)

```bash
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।" --steps
```

Output:
```text
1. (1-2) अ॒ग्निमी॒ळे॒
2. (2-3) ई॒ळे॒ पु॒रो-हि॑तम्
3. (3) पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम्
```

### 3. Multi-Script Generation

Generate Krama-pāṭha directly in Telugu script:
```bash
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।" --to telugu --steps
```

Output:
```text
1. (1-2) అ॒గ్నిమీ॒ళే॒
2. (2-3) ఈ॒ళే॒ పు॒రో-హి॑తమ్
3. (3) పు॒రోహి॑తమితి॑ పు॒రో-హి॑తమ్
```

Generate in Western Academic Roman (IAST):
```bash
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ ।" --to iast
```

Output:
```text
a̱gnimī̱ḷe̱ | ī̱ḷe̱ ití ī̱ḷe̱ ||
```

---

## Rust Library API

Add `vyasa-patha` and `vyasa-lipi` to your `Cargo.toml`:

```toml
[dependencies]
vyasa-patha = { version = "0.1.0" }
vyasa-lipi = { version = "0.1.0" }
```

### 1. High-Level Functions

#### `generate_krama(pada_text: &str) -> String`
Generates continuous Krama-pāṭha text in Devanagari with daṇḍa delimiters.

```rust
use vyasa_patha::generate_krama;

let output = generate_krama("अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।");
```

#### `generate_krama_in_script(pada_text: &str, script: Script) -> String`
Generates continuous Krama-pāṭha in any target script supported by `vyasa-lipi`.

```rust
use vyasa_patha::generate_krama_in_script;
use vyasa_lipi::Script;

let telugu = generate_krama_in_script("अ॒ग्निम् । ई॒ळे॒ ।", Script::Telugu);
```

---

### 2. Mid-Level Prakṛti Functions

#### `parse_pada_patha(input: &str) -> Vec<Pada>`
Parses a raw Pada-pāṭha string into individual `Pada` structs, resolving accents and compound hyphenation.

#### `generate_krama_patha(padas: &[Pada]) -> Vec<KramaStep>`
Permutes a slice of `Pada` elements into canonical `KramaStep` tokens with Pragṛhya and terminal Parigraha clauses.

#### `format_krama_patha(steps: &[KramaStep]) -> String`
Formats a slice of `KramaStep` tokens into continuous recitation text separated by single daṇḍas (`।`) and terminated by a double daṇḍa (`॥`).

---

### 3. Low-Level Core Engines

#### `detect_pragrhya(pada: &Pada) -> Option<PragrhyaType>`
Evaluates whether a word is Pragṛhya according to Pāṇinian sūtras:
- `DualEnding` (Pāṇini 1.1.11)
- `PronounAdas` (Pāṇini 1.1.12)
- `ArchaicPronoun` (Pāṇini 1.1.13)
- `ParticleU` (Pāṇini 1.1.13–14)
- `ParticleO` (Pāṇini 1.1.15)

#### `generate_parigraha(pada: &Pada) -> String`
Constructs a canonical Parigraha (*iti*) clause for a `Pada`:
- Simple pada: `पदम् + इति॑ + पदम्`
- Terminal *m* assimilation: `पदम्` → `पदमिति॑ पदम्`
- Compound decomposition: `रत्न॒-धात॑मम्` → `रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्`
- Particle *u*: `ऊँ॒ इति॑ उ`

#### `apply_forward_sandhi(p1: &Pada, p2: &Pada) -> String`
Applies euphonic combination across the boundary of two adjacent padas:
- Pragṛhya immunity (sandhi blocked if `p1.is_pragrhya()`)
- Final *m* assimilation before vowels and consonants
- Visarga assimilation before voiced consonants and vowels
- Preservation of Unicode combining accent positions (`\u0951`, `\u0952`, `\u1CDA`)

---

## Data Models

### `Pada`

```rust
pub struct Pada {
    pub raw: String,
    pub clean: String,
    pub compound_parts: Vec<String>,
    pub pragrhya: Option<PragrhyaType>,
}
```

- `raw`: The original pada text preserving all Vedic pitch accents and hyphens.
- `clean`: Punctuation- and accent-stripped Devanagari text for dictionary lookup and grammatical classification.
- `compound_parts`: Individual members if the pada is an analyzed compound (*samāsa*).
- `pragrhya`: `Some(PragrhyaType)` if the word has Pragṛhya status.

### `KramaStep`

```rust
pub struct KramaStep {
    pub first_index: usize,
    pub second_index: Option<usize>,
    pub text: String,
    pub is_parigraha: bool,
}
```

- `first_index`: 1-based index of the first pada.
- `second_index`: 1-based index of the second pada (`None` for Parigraha steps).
- `text`: Sandhi-joined recitation text for this step.
- `is_parigraha`: `true` if this step represents an *iti* closure clause.
