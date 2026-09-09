---
title: How to Transliterate Sanskrit
description: Practical guide to transliterating Vedic and Classical Sanskrit across Indic, Roman, and ASCII formats using vyasa-lipi.
---

This guide demonstrates how to use **`vyasa-lipi`**—both as a standalone CLI tool and as a Rust library—to perform lossless transliteration of Vedic and Classical Sanskrit texts across 11 supported formats.

---

## 1. Installing the CLI

From the root of the `vyutils` workspace:

```bash
# Build release binary
cargo build --release -p vyasa-lipi

# Install directly to your PATH (~/.cargo/bin)
cargo install --path crates/vyasa-lipi
```

Verify the installation:

```bash
vyasa-lipi --version
```

---

## 2. Basic Transliteration

### Indic Abugida Conversion

Transliterate a verse from the Bhagavad Gita (1.1) from Devanagari to Telugu:

```bash
vyasa-lipi --from devanagari --to telugu "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः"
```

**Output:**
```text
ధర్మక్షేత్రే కురుక్షేత్రే సమవేతా యుయుత్సవః
```

Transliterate to Kannada:

```bash
vyasa-lipi --from devanagari --to kannada "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः"
```

**Output:**
```text
ಧರ್ಮಕ್ಷೇತ್ರೇ ಕುರುಕ್ಷೇತ್ರೇ ಸಮವೇತಾ ಯುಯುತ್ಸವಃ
```

---

## 3. Vedic Pitch Accent Preservation

By default, `vyasa-lipi` preserves all Vedic pitch accents (*Udātta*, *Anudātta*, *Svarita*, *DīrghaSvarita*).

Transliterate Ṛgveda 1.1.1 to Telugu:

```bash
vyasa-lipi --from devanagari --to telugu "अ॒ग्निमी॑ळे पु॒रोहि॑तम्"
```

**Output:**
```text
అ॒గ్నిమీ॑ళే పు॒రోహి॑తమ్
```

Notice:
1. The *Anudātta* under-bar (`\u0952`) on *a* and *ro* is preserved.
2. The *Svarita* vertical stroke (`\u0951`) on *mī* is preserved.
3. The Vedic flap ळ (`\u0933`) is mapped to Telugu ళ (`\u0C33`).

### Stripping Accents for Search Indexing

If you are preparing text for non-Vedic readers or generating search indexes where accents interfere with matching, use `--accents strip`:

```bash
vyasa-lipi --from devanagari --to devanagari --accents strip "अ॒ग्निमी॑ळे पु॒रोहि॑तम्"
```

**Output:**
```text
अग्निमीळे पुरोहितम्
```

---

## 4. Transliterating to Grantha & Malayalam

Grantha is the canonical Tamil-region Sanskrit script:

```bash
vyasa-lipi --from devanagari --to grantha "धर्मक्षेत्रे कुरुक्षेत्रे"
```

**Output:**
```text
𑌧𑌰𑍍𑌮𑌕𑍍𑌷𑍇𑌤𑍍𑌰𑍇 𑌕𑍁𑌰𑍁𑌕𑍍𑌷𑍇𑌤𑍍𑌰𑍇
```

Malayalam:

```bash
vyasa-lipi --from devanagari --to malayalam "धर्मक्षेत्रे कुरुक्षेत्रे"
```

**Output:**
```text
ധര്മക്ഷേത്രേ കുരുക്ഷേത്രേ
```

---

## 5. Automatic Script Detection

If you omit the `--from` flag, `vyasa-lipi` automatically inspects the Unicode codepoints of the input:

```bash
# Detected as Telugu -> Transliterated to Devanagari
vyasa-lipi --to devanagari "ధర్మక్షేత్రే కురుక్షేత్రే"
```

**Output:**
```text
धर्मक्षेत्रे कुरुक्षेत्रे
```

```bash
# Detected as Kannada -> Transliterated to IAST
vyasa-lipi --to iast "ಧರ್ಮಕ್ಷೇತ್ರೇ ಕುರುಕ್ಷೇತ್ರೇ"
```

**Output:**
```text
dharmakṣētrē kurukṣētrē
```

---

## 6. Strict ISO 15919 vs Academic IAST

Depending on whether you need strict international library standards or standard Indological transcription:

```bash
# ISO 15919 (under-ring for r̥, macron for ē)
vyasa-lipi --to iso15919 "कृष्णः"
# kr̥ṣṇaḥ

# Standard IAST (under-dot for ṛ)
vyasa-lipi --to iast "कृष्णः"
# kṛṣṇaḥ
```

---

## 7. Computational ASCII Formats (SLP1, HK, WX)

When feeding Sanskrit into computational pipelines, parsers, or search engines:

```bash
# Convert Devanagari to SLP1 (Cologne Sanskrit dictionary standard)
vyasa-lipi --from devanagari --to slp1 "धर्मक्षेत्रे कुरुक्षेत्रे"
# Darmakzetre kurukzetre

# Convert back from SLP1 to Devanagari
vyasa-lipi --from slp1 --to devanagari "Darmakzetre kurukzetre"
# धर्मक्षेत्रे कुरुक्षेत्रे
```

---

## 8. Piped Input & Batch Processing

`vyasa-lipi` accepts input from standard input (`stdin`), making it ideal for shell scripts and corpus pipelines:

```bash
# Process a full chapter from file
cat gita_chapter1_deva.txt | vyasa-lipi --to telugu > gita_chapter1_telu.txt

# Inspect difference after round-trip
diff <(cat text.txt) <(cat text.txt | vyasa-lipi --to telugu | vyasa-lipi --to devanagari)
# Zero output confirms byte-identical round-trip fidelity!
```

---

## 9. Rust API Integration

Add `vyasa-lipi` to your project's `Cargo.toml`:

```toml
[dependencies]
vyasa-lipi = { path = "../crates/vyasa-lipi" }
```

### Example Rust Program

```rust
use vyasa_lipi::{detect_script, transliterate, Script};
use vyasa_lipi::model::AccentMode;

fn main() {
    let input = "अ॒ग्निमी॑ळे पु॒रोहि॑तम्";

    // Auto-detect source script
    let src = detect_script(input).unwrap_or(Script::Devanagari);

    // Transliterate to Telugu with accents preserved
    let telugu = transliterate(input, src, Script::Telugu, AccentMode::Preserve);
    println!("Telugu: {}", telugu);

    // Transliterate to IAST with accents stripped
    let iast = transliterate(input, src, Script::Iast, AccentMode::Strip);
    println!("IAST (stripped): {}", iast);

    // Verify round-trip back to Devanagari
    let roundtrip = transliterate(&telugu, Script::Telugu, Script::Devanagari, AccentMode::Preserve);
    assert_eq!(roundtrip, input);
}
```

---

## Next Steps

- Consult the complete flag and script reference: **[vyasa-lipi Reference](file:///Users/anand/Projects/project-vyasa/vyutils/documentation-site/src/content/docs/reference/vyasa-lipi.md)**
- Learn about the underlying phonetic models: **[Sanskrit Phonetics](file:///Users/anand/Projects/project-vyasa/vyutils/documentation-site/src/content/docs/explanation/sanskrit-phonetics.md)**
