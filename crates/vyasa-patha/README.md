# vyasa-patha

**Vedic & Classical Sanskrit Recitation Generator (Prakṛti and Vikṛti Pāṭhas) in Rust.**

Part of the [`vyutils`](https://github.com/project-vyasa/vyutils) project.

[![Documentation](https://img.shields.io/badge/docs-vyutils-blue)](https://project-vyasa.github.io/vyutils/reference/vyasa-patha/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

---

## Overview

For millennia, the Vedic tradition preserved its vast oral corpus with letter-perfect, pitch-accurate fidelity through a system of recitation modes: **Prakṛti** (natural) and **Vikṛti** (complex permutation) pāṭhas.

`vyasa-patha` provides a deterministic, high-performance engine for parsing, permuting, and synthesizing Vedic recitation forms:

1. **Prakṛti Recitations**:
   - **Pada-pāṭha**: Parses and normalizes sandhi-isolated padas, tracking compound splits (`-`, `ऽ`), boundary markers (`।`, `॥`), and pitch accents.
   - **Krama-pāṭha**: Generates canonical stepped pairs (1-2 | 2-3 | 3-4 ... (n-1)-n | n-iti-n), with terminal and Pragṛhya *iti* parigraha clauses.
2. **Pragṛhya Detection & Sandhi Immunity**:
   - Implements Pāṇini's *Aṣṭādhyāyī* 1.1.11–1.1.19: dual endings in *ī*, *ū*, *e* (*īdūded-dvivacanaṃ pragṛhyam*), archaic Vedic pronouns (*asme*, *yuvābhyām*, *tve*), particles *u*, *ā*, and demonstratives (*amī*, *amū*).
   - Guarantees strict sandhi immunity (*pluta-pragṛhyā aci nityam*, 6.1.125).
3. **Parigraha (Sthita-Upasthita / "iti" Clause) Engine**:
   - Synthesizes canonical Vedic *iti* clauses with Svarita accentuation (`इति॑`).
   - Automatically handles compound splits (*unified* + `इति॑` + *split*, e.g., `रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्`).
   - Merges final *m* with *iti* (`...मिति॑`).
4. **Vedic Forward Sandhi**:
   - Accurately joins adjacent padas: handles vowel sandhi, terminal *m* merging before vowels and converting to Anusvāra before consonants, visarga transformations, while respecting Unicode combining accent positions.
5. **Multi-Script Recitation**:
   - Seamlessly integrated with `vyasa-lipi` to generate recitations directly in **Devanagari**, **Telugu**, **Kannada**, **Grantha**, **Malayalam**, **Bengali**, **IAST**, and **ISO 15919**.
6. **Zero Dependencies & `no_std` Ready**:
   - Core library compiles with `#![no_std]` + `alloc`, suitable for embedded systems, WebAssembly, or low-latency servers.

---

## Theoretical Background: Vedic Recitation Modes

### 1. The Three Prakṛti Pāṭhas

| Pāṭha Mode | Permutation Formula | Description | Example (Rigveda 1.1.1) |
|:---|:---|:---|:---|
| **Saṃhitā-pāṭha** | 1 2 3 4 ... n | Continuous, sandhi-joined poetic verse. | अ॒ग्निमी॑ळे पु॒रोहि॑तं... |
| **Pada-pāṭha** | 1 | 2 | 3 | 4 ... n | Isolated words with internal sandhi resolved, compounds marked. Attributed to Śākalya. | अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् । |
| **Krama-pāṭha** | 1-2 | 2-3 | 3-4 ... n-iti | Step-by-step paired recitation where every internal word is recited twice: first as predicate, second as subject. | अ॒ग्निमी॒ळे॒ । ई॒ळे॒ पु॒रो-हि॑तम् । पु॒रोहि॑तम् इति॑... |

### 2. The Eight Vikṛti Pāṭhas

All eight complex recitation permutations build directly upon the pairs produced by **Krama-pāṭha**:

1. **Jaṭā-pāṭha**: 1-2, 2-1, 1-2 | 2-3, 3-2, 2-3 ...
2. **Mālā-pāṭha**: Intertwined garland chaining.
3. **Śikhā-pāṭha**: Jaṭā extended by forward linkage: 1-2, 2-1, 1-2-3 ...
4. **Rekhā-pāṭha**: Stepwise triangular inversion.
5. **Dhvaja-pāṭha**: Flag arrangement connecting start and end padas.
6. **Daṇḍa-pāṭha**: Staff progression: 1-2, 2-1, 2-3, 3-2, 3-4, 4-3 ...
7. **Ratha-pāṭha**: Chariot permutation pairing verses across hemistichs.
8. **Ghana-pāṭha**: The pinnacle of Vedic recitation:
   > 1-2, 2-1, 1-2-3, 3-2-1, 1-2-3 | 2-3, 3-2, 2-3-4, 4-3-2, 2-3-4 ...

### 3. Pragṛhya Vowels & Parigraha

In Vedic Sanskrit, certain vowels are phonologically immutable: they never undergo sandhi when followed by another vowel. Pāṇini classifies these as **Pragṛhya** (*Aṣṭādhyāyī* 1.1.11–1.1.19):
- Dual nominative/accusative forms ending in long *ī*, *ū*, or *e* (*harī*, *kavī*, *dhenū*, *etau*).
- Locatives in *ī* or *e* in the Veda (*somā somapāve*, *asme*).
- The particle *u* (recited with anudātta and nasalization: `ऊँ॒ इति॑ उ`).
- The demonstrative pronouns *amī* and *amū*.

When a Pragṛhya word occurs in Krama-pāṭha, it is followed by a **Parigraha** (the particle `इति॑`) to confirm that the absence of sandhi is intentional and not a transmission error.

Compounds (*samāsa*) receive Parigraha when terminal:
> रत्न-धातमम् → रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्

---

## Installation

### Building from Source

```bash
# Build the CLI tool
cargo build --release -p vyasa-patha

# Install globally
cargo install --path crates/vyasa-patha
```

### Adding to `Cargo.toml`

```toml
[dependencies]
vyasa-patha = { version = "0.1.0" }
vyasa-lipi = { version = "0.1.0" }
```

---

## CLI Usage

The `vyasa-patha` command-line tool reads Pada-pāṭha from arguments or standard input and emits recitations in any script.

```bash
vyasa-patha [OPTIONS] [TEXT]
```

### Options

- `-m, --mode <MODE>`: Recitation mode: `krama` (default) or `pada`
- `-t, --to <TO>`: Target script (`devanagari`, `telugu`, `kannada`, `grantha`, `malayalam`, `bengali`, `iast`, `iso15919`, etc.)
- `-s, --steps`: Emit numbered steps line-by-line rather than continuous recitation text

### Examples

#### 1. Rigveda 1.1.1 Full Krama-pāṭha (Devanagari)

```bash
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् । य॒ज्ञस्य॑ । दे॒वम् । ऋ॒त्विज॑म् । होता॑रम् । रत्न॒-धात॑मम् ॥" --steps
```

Output:
```text
1. (1-2) अ॒ग्निमी॒ळे॒
2. (2-3) ई॒ळे॒ पु॒रो-हि॑तम्
3. (3-4) पु॒रो-हि॑तं य॒ज्ञस्य॑
4. (4-5) य॒ज्ञस्य॑ दे॒वम्
5. (5-6) दे॒वमृ॒त्विज॑म्
6. (6-7) ऋ॒त्विजं॑ होता॑रम्
7. (7-8) होता॑रं रत्न॒-धात॑मम्
8. (8) रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्
```

#### 2. Recitation in Telugu Script

```bash
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।" --to telugu
```

Output:
```text
అ॒గ్నిమీ॒ళే॒ । ఈ॒ళే॒ పు॒रो-హి॑తమ్ । పు॒రోహి॑తమితి॑ పు॒रो-హి॑తమ్ ॥
```

#### 3. Recitation in Kannada Script

```bash
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।" --to kannada
```

Output:
```text
ಅ॒ಗ್ನಿಮೀ॒ಳೇ॒ । ಈ॒ಳೇ॒ ಪು॒ರೋ-ಹಿ॑ತಮ್ । ಪು॒ರೋಹಿ॑ತಮಿತಿ॑ ಪು॒ರೋ-ಹಿ॑ತಮ್ ॥
```

#### 4. Recitation in Scholarly Roman (IAST / ISO 15919)

```bash
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ ।" --to iast
```

Output:
```text
a̱gnimī̱ḷe̱ | ī̱ḷe̱ ití ī̱ḷe̱ ||
```

---

## Rust Library API

### Quickstart

```rust
use vyasa_patha::generate_krama;
use vyasa_lipi::Script;

let pada_input = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";

// 1. Generate continuous Krama text in Devanagari
let krama = generate_krama(pada_input);
println!("{}", krama);
// "अ॒ग्निमी॒ळे॒ । ई॒ळे॒ पु॒रो-हि॑तम् । पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम् ॥"

// 2. Generate Krama directly into Telugu script
let telugu_krama = vyasa_patha::generate_krama_in_script(pada_input, Script::Telugu);
println!("{}", telugu_krama);
// "అ॒గ్నిమీ॒ళే॒ । ఈ॒ళే॒ పు॒రో-హి॑తమ్ । పు॒రోహి॑తమితి॑ పు॒రో-హి॑తమ్ ॥"
```

### Low-Level Parsing & Step Inspection

```rust
use vyasa_patha::prakriti::{parse_pada_patha, generate_krama_patha};
use vyasa_patha::pragrhya::detect_pragrhya;
use vyasa_patha::parigraha::generate_parigraha;

let padas = parse_pada_patha("हरी । एतौ । विहरतः ॥");
assert_eq!(padas[0].is_pragrhya(), true);

let steps = generate_krama_patha(&padas);
for step in &steps {
    if step.is_parigraha {
        println!("Parigraha (Pada {}): {}", step.first_index, step.text);
    } else {
        println!("Step {}-{}: {}", step.first_index, step.second_index.unwrap(), step.text);
    }
}
```

---

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
