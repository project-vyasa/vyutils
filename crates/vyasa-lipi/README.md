# vyasa-lipi

**Universal, Lossless, Vedic Multi-Script Transliteration Engine in Rust.**

Part of the [`vyutils`](https://github.com/project-vyasa/vyutils) project.

[![Documentation](https://img.shields.io/badge/docs-vyutils-blue)](https://project-vyasa.github.io/vyutils/reference/vyasa-lipi/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

---

## Overview

`vyasa-lipi` is a high-performance, zero-runtime-dependency transliteration engine built specifically for Vedic and Classical Sanskrit texts. Unlike existing transliteration packages that mutate or strip Vedic pitch accents (*svaras*) and conflate Roman transliteration standards, `vyasa-lipi` guarantees:

1. **Lossless Round-Trip Transliteration**: Transliterating A → B → A produces byte-identical output across all supported scripts.
2. **First-Class Vedic Accent (*Svara*) Preservation**: Uniquely handles *Udātta*, *Anudātta* (`\u0952`), *Svarita* (`\u0951`), and *DīrghaSvarita* across all Indic scripts and transliteration schemes.
3. **Strict ISO 15919 vs IAST Separation**:
   - **ISO 15919**: Preserves vocalic under-rings (`r̥`, `r̥̄`, `l̥`, `l̥̄`) and explicit Dravidian long/short macrons (`ē`, `ō` vs `e`, `o`).
   - **IAST**: Standard academic under-dots (`ṛ`, `ṝ`, `ḷ`, `ḹ`) and unmarked long dipthongs (`e`, `o`).
4. **Intermediate Representation (AST)**: Parsed into structural tokens (`AksharaToken` with initial consonants, medial vowels, ayogavahas, and svaras), completely avoiding brittle regex substitution cascades.

---

## Supported Scripts & Formats

`vyasa-lipi` natively supports 11 Indic scripts, Romanizations, and computational ASCII formats:

| Format Name | Script ID | Script Type | Key Notes |
|:---|:---|:---|:---|
| **Devanagari** | `devanagari`, `deva` | Indic Abugida | Standard Sanskrit, Vedic accents (`\u0951`, `\u0952`), flap ळ (`\u0933`), ळ्ह (`\u0934`) |
| **Telugu** | `telugu`, `telu` | Indic Abugida | South Indian Vedic tradition, native Dravidian short vowels (*e*, *o*), flap ళ (`\u0C33`) |
| **Kannada** | `kannada`, `knda` | Indic Abugida | Complete Vedic accent support, native Dravidian short vowels (*e*, *o*), flap ಳ (`\u0C83`) |
| **Grantha** | `grantha`, `gran` | Indic Abugida | Traditional Tamil Nadu Sanskrit script (Unicode block `11300..1137F`), fully distinguishes all 4 Sanskrit stops per varga |
| **Malayalam** | `malayalam`, `mlym` | Indic Abugida | Complete Sanskrit consonant inventory, native Dravidian vowels, Vedic *chillu* forms |
| **Bengali** | `bengali`, `beng` | Indic Abugida | Eastern Indian tradition, distinct *kṣa*, *jña*, candrabindu |
| **Strict ISO 15919** | `iso15919`, `iso` | Roman Diacritic | International standard: under-rings for liquids (`r̥`, `l̥`), macron `ē`/`ō`, retroflex flap `l̤` |
| **IAST** | `iast` | Roman Diacritic | Academic standard: under-dots (`ṛ`, `ḷ`), bare `e`/`o` |
| **SLP1** | `slp1` | Computational ASCII | 1:1 ASCII character mapping, standard in Cologne Digital Sanskrit Dictionaries |
| **Harvard-Kyoto** | `harvard-kyoto`, `hk` | Computational ASCII | Case-sensitive ASCII transliteration standard |
| **WX** | `wx` | Computational ASCII | Indian computational linguistics standard developed at IIT Kanpur/Hyderabad |

> [!NOTE]
> **Why Grantha and not unaugmented Tamil?**
> Classical Tamil merges all four Sanskrit stop registers into a single consonant (k, kh, g, gh → k), which violates `vyutils`' non-negotiable invariant of 100% round-trip lossless transliteration. For texts from the Tamil linguistic sphere, Grantha is the canonical, fully expressive script.

---

## Installation

### Building from Source

```bash
# Build the binary
cargo build --release -p vyasa-lipi

# Install globally to your cargo bin
cargo install --path crates/vyasa-lipi
```

---

## CLI Usage Guide

The `vyasa-lipi` command line interface provides fast, flexible transliteration for one-off commands, piped text processing, and batch files.

### Synopsis

```bash
vyasa-lipi [OPTIONS] [TEXT]
```

### Options

- `-f, --from <SCRIPT>`: Source script (auto-detected if omitted)
- `-t, --to <SCRIPT>`: Target script (default: `devanagari`)
- `-a, --accents <MODE>`: Accent handling mode:
  - `preserve` (default): Keep all Vedic pitch accents (*udātta*, *anudātta*, *svarita*)
  - `strip`: Strip all combining accent marks
  - `warn`: Preserve accents, but emit a warning on stderr if target format does not canonically support them
- `-h, --help`: Print help information
- `-V, --version`: Print version information

---

### Examples

#### 1. Transliterating Vedic Sanskrit with Pitch Accents
Rigveda 1.1.1 from Devanagari to Telugu:

```bash
vyasa-lipi --from devanagari --to telugu "अ॒ग्निमी॑ळे पु॒रोहि॑तम्"
# Output: అ॒గ్నిమీ॑ళే పు॒రోహి॑తమ్
```

Transliterating to Kannada:

```bash
vyasa-lipi --from devanagari --to kannada "अ॒ग्निमी॑ळे पु॒रोहि॑तम्"
# Output: ಅ॒ಗ್ನಿಮೀ॑ಳೆ ಪು॒ರೋಹಿ॑ತಮ್
```

#### 2. Transliterating to Grantha & Malayalam
```bash
vyasa-lipi --from devanagari --to grantha "धर्मक्षेत्रे कुरुक्षेत्रे"
# Output in Unicode Grantha: 𑌧𑌰𑍍𑌮𑌕𑍍𑌷𑍇𑌤𑍍𑌰𑍇 𑌕𑍁𑌰𑍁𑌕𑍍𑌷𑍇𑌤𑍍𑌰𑍇

vyasa-lipi --from devanagari --to malayalam "धर्मक्षेत्रे कुरुक्षेत्रे"
# Output: ധര്മക്ഷേത്രേ കുരുക്ഷേത്രേ
```

#### 3. Automatic Script Detection
If `--from` is omitted, `vyasa-lipi` automatically analyzes Unicode code points:

```bash
vyasa-lipi --to iast "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः"
# Output: dharmakṣētrē kurukṣētrē samavētā yuyutsavaḥ
```

#### 4. Strict ISO 15919 vs IAST

```bash
# Strict ISO 15919 (note under-ring r̥ and macron ē)
vyasa-lipi --from devanagari --to iso15919 "कृष्णः"
# Output: kr̥ṣṇaḥ

# IAST (note under-dot ṛ)
vyasa-lipi --from devanagari --to iast "कृष्णः"
# Output: kṛṣṇaḥ
```

#### 5. Computational ASCII (SLP1, Harvard-Kyoto, WX)

```bash
# Harvard-Kyoto
vyasa-lipi --from devanagari --to hk "धर्मक्षेत्रे कुरुक्षेत्रे"
# Output: dharmakSetre kurukSetre

# SLP1
vyasa-lipi --from devanagari --to slp1 "धर्मक्षेत्रे कुरुक्षेत्रे"
# Output: Darmakzetre kurukzetre

# Round-trip back to Devanagari
vyasa-lipi --from slp1 --to devanagari "Darmakzetre kurukzetre"
# Output: धर्मक्षेत्रे कुरुक्षेत्रे
```

#### 6. Piped Input & Batch Processing
`vyasa-lipi` seamlessly reads from `stdin`:

```bash
cat rigveda_mandala1.txt | vyasa-lipi --from devanagari --to telugu > rigveda_telugu.txt
```

#### 7. Stripping Vedic Accents
When generating search indexes or non-Vedic editions, strip accents with `--accents strip`:

```bash
vyasa-lipi --from devanagari --to devanagari --accents strip "अ॒ग्निमी॑ळे पु॒रोहि॑तम्"
# Output: अग्निमीळे पुरोहितम्
```

---

## Crate Developer Guide (Rust API)

Add `vyasa-lipi` to your `Cargo.toml`:

```toml
[dependencies]
vyasa-lipi = { path = "../crates/vyasa-lipi" }
```

### Basic Transliteration

```rust
use vyasa_lipi::{transliterate, Script};
use vyasa_lipi::model::AccentMode;

fn main() {
    let input = "अ॒ग्निमी॑ळे पु॒रोहि॑तम्";
    let output = transliterate(
        input,
        Script::Devanagari,
        Script::Telugu,
        AccentMode::Preserve,
    );
    assert_eq!(output, "అ॒గ్నిమీ॑ళే పు॒రోహి॑తమ్");
}
```

### Automatic Script Detection

```rust
use vyasa_lipi::{detect_script, transliterate, Script};
use vyasa_lipi::model::AccentMode;

fn main() {
    let text = "ధర్మక్షేత్రే కురుక్షేత్రే";
    let detected = detect_script(text);
    assert_eq!(detected, Some(Script::Telugu));

    let devanagari = transliterate(
        text,
        detected.unwrap(),
        Script::Devanagari,
        AccentMode::Preserve,
    );
    assert_eq!(devanagari, "धर्मक्षेत्रे कुरुक्षेत्रे");
}
```

### Working with the Phonemic AST

```rust
use vyasa_lipi::brahmic::parse_brahmic;
use vyasa_lipi::Script;

fn main() {
    let text = "अ॒ग्निम्";
    let tokens = parse_brahmic(text, Script::Devanagari);
    
    // Inspect parsed Akshara tokens, consonants, vowels, and Vedic pitch accents
    for token in tokens {
        println!("{:?}", token);
    }
}
```

---

## Documentation

Full architectural documentation and references are available at:
- **[Vedic Transliteration Explanation](https://project-vyasa.github.io/vyutils/explanation/vedic-transliteration/)**
- **[CLI Reference Manual](https://project-vyasa.github.io/vyutils/reference/vyasa-lipi/)**
- **[How-To Guide: Transliterating Sanskrit](https://project-vyasa.github.io/vyutils/guides/transliterate-sanskrit/)**
- **[RFC-0002: Sanskrit Processing Engine](https://project-vyasa.github.io/vyutils/rfcs/rfc-0002-sanskrit-processing-engine/)**
