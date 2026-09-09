---
title: vyasa-lipi CLI Reference
description: Command-line reference, script identifiers, flags, and character support matrices for vyasa-lipi.
---

`vyasa-lipi` is the command-line interface for the universal Sanskrit transliteration engine. It provides high-throughput, lossless transliteration of Vedic and Classical Sanskrit across Indic, Roman, and computational ASCII scripts.

---

## Installation

From the `vyutils` repository root:

```bash
cargo build --release -p vyasa-lipi
cargo install --path crates/vyasa-lipi
```

Binary location: `~/.cargo/bin/vyasa-lipi`

---

## Synopsis

```bash
vyasa-lipi [OPTIONS] [TEXT]
cat <file> | vyasa-lipi [OPTIONS]
```

If `[TEXT]` is provided as an argument, it is transliterated directly. If `[TEXT]` is omitted, `vyasa-lipi` reads from `stdin` until EOF.

---

## Options & Flags

| Flag | Long Option | Values | Default | Description |
|:---|:---|:---|:---|:---|
| `-f` | `--from` | `<SCRIPT>` | Auto-detect | Source script format. If omitted, script is inferred from Unicode codepoints. |
| `-t` | `--to` | `<SCRIPT>` | `devanagari` | Target script format. |
| `-a` | `--accents` | `preserve` \| `strip` \| `warn` | `preserve` | Handling of Vedic pitch accents (*udātta*, *anudātta*, *svarita*). |
| `-h` | `--help` | — | — | Display help information and exit. |
| `-V` | `--version` | — | — | Display version information and exit. |

---

## Supported Script Identifiers

The `--from` and `--to` arguments accept case-insensitive script names and standardized short aliases:

| Script Name | Recognized Identifiers / Aliases | Script Category | Primary Domain |
|:---|:---|:---|:---|
| **Devanagari** | `devanagari`, `deva` | Indic Abugida | Standard Sanskrit, Northern India, Vedic editions |
| **Telugu** | `telugu`, `telu` | Indic Abugida | Andhra Pradesh, Telangana, Krishna Yajurveda tradition |
| **Kannada** | `kannada`, `knda` | Indic Abugida | Karnataka, Rigveda & Sama Veda traditions |
| **Grantha** | `grantha`, `gran` | Indic Abugida | Traditional Sanskrit script of Tamil Nadu |
| **Malayalam** | `malayalam`, `mlym` | Indic Abugida | Kerala, Rigvedic Nambudiri tradition |
| **Bengali** | `bengali`, `beng` | Indic Abugida | Bengal, Assam, Eastern Indian traditions |
| **Strict ISO 15919** | `iso15919`, `iso` | Roman Diacritic | International Library standard (under-rings for liquids, macrons) |
| **Standard IAST** | `iast` | Roman Diacritic | Western academic standard (under-dots) |
| **SLP1** | `slp1` | Computational ASCII | Cologne Digital Sanskrit Dictionaries (1:1 ASCII mapping) |
| **Harvard-Kyoto** | `harvard-kyoto`, `hk` | Computational ASCII | Case-sensitive ASCII transliteration |
| **WX** | `wx` | Computational ASCII | Indian computational linguistics standard (IIT-K / UoH) |

---

## Accent Handling Modes

Vedic texts use combining pitch accents. The `--accents` flag controls how these are emitted:

- **`preserve` (Default)**:
  Retains all pitch accents. In Indic abugidas, combining Vedic marks `\u0951` (*svarita*), `\u0952` (*anudātta*), and `\u1CDA` (*dīrghasvarita*) are correctly positioned over the target script's aksharas.
- **`strip`**:
  Completely strips all Vedic pitch accent marks, leaving clean classical Sanskrit text suitable for search indexing or general readership.
- **`warn`**:
  Preserves accents where supported, but outputs a warning on `stderr` when targeting formats with limited native diacritic representation.

---

## Exit Codes

| Code | Meaning |
|:---:|:---|
| `0` | Success: Transliteration completed without errors. |
| `1` | Error: Unknown script identifier, invalid CLI argument, or I/O failure. |

---

## Examples

### 1. Direct Command Line Transliteration

```bash
# Transliterate Rigveda 1.1.1 to Telugu
vyasa-lipi --from devanagari --to telugu "अ॒ग्निमी॑ळे पु॒रोहि॑तम्"
# అ॒గ్నిమీ॑ళె పు॒రోహి॑తమ్

# Auto-detect script and convert to IAST
vyasa-lipi --to iast "ధర్మక్షేత్రే కురుక్షేత్రే"
# dharmakṣētrē kurukṣētrē
```

### 2. Unix Pipeline Processing

```bash
# Transliterate whole files via stdin/stdout
cat input_devanagari.txt | vyasa-lipi --to kannada > output_kannada.txt

# Strip accents during stream
cat vedic_corpus.txt | vyasa-lipi --to devanagari --accents strip > corpus_clean.txt
```

### 3. Computational ASCII Transformations

```bash
# Round-trip through Harvard-Kyoto
vyasa-lipi --from devanagari --to hk "धर्मक्षेत्रे" | vyasa-lipi --from hk --to devanagari
# धर्मक्षेत्रे
```
