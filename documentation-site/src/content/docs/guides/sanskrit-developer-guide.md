---
title: Sanskrit Developer Guide (Rust & WASM)
description: Comprehensive developer guide for integrating Vyasa Sanskrit linguistic engines via direct Rust crates and WebAssembly (WASM).
---

Project Vyasa provides two primary integration pathways for developers building Sanskrit computational applications:

1. **Direct Rust Crates**: For native applications, backend services, high-throughput pipelines, and command-line tools.
2. **WebAssembly (WASM) Bridge (`@project-vyasa/sanskrit-wasm`)**: For web applications, browser extensions, Svelte/React user interfaces, and Node.js runtimes.

---

## 1. Architectural Comparison

| Dimension | Direct Rust Crates | WebAssembly Bridge (`@project-vyasa/sanskrit-wasm`) |
|:---|:---|:---|
| **Target Runtime** | Native OS (macOS, Linux, Windows, bare-metal `no_std`) | Web Browsers, SvelteKit, React, Electron, Node.js |
| **Distribution** | Cargo workspace / `crates.io` | NPM package (`@project-vyasa/sanskrit-wasm`) / `pkg/` |
| **Dependencies** | Zero external dependencies (`no_std` ready) | `wasm-bindgen`, `serde-wasm-bindgen` |
| **Binary Overhead** | Direct native compilation | **133 KB** `.wasm` binary (opt-level `s`) |
| **Execution Latency** | Sub-microsecond native execution | <1ms JIT-compiled in-browser execution |
| **Vedic Accents** | Lossless preservation across all 11 scripts | Lossless preservation across all 11 scripts |

---

## 2. Option A: Direct Rust Crates

If you are developing in Rust, add the crates directly to your `Cargo.toml`:

```toml
[dependencies]
vyasa-phonetics = { path = "../crates/vyasa-phonetics" }
vyasa-lipi = { path = "../crates/vyasa-lipi" }
vyasa-patha = { path = "../crates/vyasa-patha" }
```

### Direct Rust Examples

#### 1. Transliteration (`vyasa-lipi`)
```rust
use vyasa_lipi::{detect_script, transliterate, Script};

fn main() {
    let deva = "अ॒ग्निमी॑ळे पु॒रोहि॑तम्";
    
    // Auto-detect script
    let detected = detect_script(deva); // Some(Script::Devanagari)
    
    // Transliterate with full Vedic pitch accent preservation
    let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
    assert_eq!(telu, "అ॒గ్నిమీ॑ళే పు॒रोహి॑తమ్");
}
```

#### 2. Recitation: Krama & Jaṭā (`vyasa-patha`)
```rust
use vyasa_patha::{parse_pada_patha, generate_krama_patha, format_krama_patha, generate_jata, Script};

fn main() {
    let pada_text = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";
    let padas = parse_pada_patha(pada_text);
    
    // Generate Krama steps
    let steps = generate_krama_patha(&padas);
    let krama_output = format_krama_patha(&steps);
    println!("Krama:\n{}", krama_output);
    // => "अ॒ग्निमी॑ळे । ई॒ळे॒ पु॒रोहि॑तम् । पु॒रो-हि॑तम् । पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम् ॥"

    // Generate Jaṭā (1-2, 2-1, 1-2 with Abhinihita reverse Sandhi)
    let jata_output = generate_jata(pada_text);
    println!("Jaṭā:\n{}", jata_output);
    // => "अ॒ग्निमी॑ळ ई॒ळे॒ऽग्निर॒ग्निमी॑ळे । ई॒ळे॒ पु॒रोहि॑तं पु॒रोहि॑तमी॑ळ ई॒ळे॒ पु॒रोहि॑तं पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम् ।"
}
```

#### 3. Phonetics & Śiva Sūtras (`vyasa-phonetics`)
```rust
use vyasa_phonetics::{Pratyahara, SHIVA_SUTRAS, Varna, Consonant, sthana, abhyantara_prayatna, is_ghosha};

fn main() {
    // Resolve Pāṇinian Pratyāhāras
    let ac = Pratyahara::from_name("ac").unwrap();
    let hal = Pratyahara::from_name("hal").unwrap();
    
    // Articulatory classification
    let k = Varna::Consonant(Consonant::K);
    let places = sthana(&k); // [Sthana::Kantha]
    let effort = abhyantara_prayatna(&k); // AbhyantaraPrayatna::Sprshta
    let voiced = is_ghosha(&k); // false
}
```

#### 4. Krishna Yajurveda Phonology (`vyasa-phonetics`)
```rust
use vyasa_phonetics::{
    karana, sthana, classify_taittiriya_svarita, should_double_in_taittiriya,
    Karana, Sthana, Svara, SvaritaJunctureContext, TaittiriyaSvarita,
    Varna, Consonant, Ayogavaha,
};

fn main() {
    // 1. Active Articulator (Karaṇa) vs Passive Place (Sthāna) under TPr Ch. 2
    let c = Varna::Consonant(Consonant::C);
    assert_eq!(karana(&c), Karana::Jihvopamadhya); // Tongue blade/edges
    assert_eq!(sthana(&c), vec![Sthana::Talu]); // Hard palate

    // 2. 8-fold Svarita classification under TPr Ch. 20
    let kshaipra = classify_taittiriya_svarita(
        Svara::Svarita,
        SvaritaJunctureContext::SemivowelSandhi,
    );
    assert_eq!(kshaipra, Some(TaittiriyaSvarita::Kshaipra));

    // 3. Consonant gemination (Dvitva) under TPr Ch. 14
    // In "arkaḥ", k preceded by r doubles before a vowel
    let r = Varna::Consonant(Consonant::R);
    assert!(should_double_in_taittiriya(Some(&r), Consonant::K, None));
}
```

---

## 3. Option B: WebAssembly (`@project-vyasa/sanskrit-wasm`)

The WASM bridge compiles the three Rust engines into an ultra-fast, 133 KB WebAssembly bundle that runs 100% in the client browser with zero backend requests.

### Installation

In your web application (e.g. `apps/sanskrit-studio` in `vyasa-apps`):

```json
{
  "dependencies": {
    "@project-vyasa/sanskrit-wasm": "file:../../../../vyutils/crates/vyasa-sanskrit-wasm/pkg"
  }
}
```

### Module Initialization

Before calling any WASM functions in the browser, initialize the module:

```typescript
import init from '@project-vyasa/sanskrit-wasm';

// Call once on application startup (e.g. onMount in Svelte)
await init();
```

---

## 4. WebAssembly Exported Functions Reference

Below is a complete reference with a dedicated code example for each of the 17 exported functions.

```typescript
import init, {
  transliterate,
  detect_script,
  get_supported_scripts,
  generate_krama,
  generate_krama_text,
  generate_jata,
  generate_jata_text,
  parse_padas,
  get_shiva_sutras,
  get_pratyahara_sounds,
  check_pratyahara_contains,
  inspect_varna,
  analyze_syllables,
  get_taittiriya_svaritas,
  inspect_taittiriya_varna,
  check_taittiriya_dvitva,
  classify_taittiriya_svarita_by_context
} from '@project-vyasa/sanskrit-wasm';
```

---

### Function 1: `transliterate`

Transliterates text between any of the 11 supported Indic, Roman, and ASCII scripts while maintaining 100% preservation of Vedic pitch accents (*Anudātta*, *Udātta*, *Svarita*, *Dīrgha Svarita*).

```typescript
const result = transliterate(
  "अ॒ग्निमी॑ळे पु॒रोहि॑तम्", // input text
  "devanagari",              // source script
  "telugu"                   // target script
);

console.log(result);
// Output: "అ॒గ్నిమీ॑ళే పు॒रोహి॑తమ్"
```

---

### Function 2: `detect_script`

Automatically detects the script of the provided Sanskrit text by analyzing Unicode codepoints.

```typescript
const script1 = detect_script("धर्मक्षेत्रे कुरुक्षेत्रे");
console.log(script1); // "Devanagari"

const script2 = detect_script("ధర్మక్షేత్రే కురుక్షేత్రే");
console.log(script2); // "Telugu"

const script3 = detect_script("dharmakṣetre kurukṣetre");
console.log(script3); // "IAST"
```

---

### Function 3: `get_supported_scripts`

Returns metadata for all 11 supported scripts, including family classification (`is_indic`) and Vedic pitch accent capabilities (`has_vedic_pitch`).

```typescript
const scripts = get_supported_scripts();

console.log(scripts);
/* Output:
[
  { id: "devanagari", name: "Devanagari (देवनागरी)", is_indic: true, has_vedic_pitch: true },
  { id: "telugu", name: "Telugu (తెలుగు)", is_indic: true, has_vedic_pitch: true },
  { id: "kannada", name: "Kannada (ಕನ್ನಡ)", is_indic: true, has_vedic_pitch: true },
  { id: "grantha", name: "Grantha (𑌗𑍍𑌰𑌨𑍍𑌥)", is_indic: true, has_vedic_pitch: true },
  { id: "malayalam", name: "Malayalam (മലയാളം)", is_indic: true, has_vedic_pitch: true },
  { id: "bengali", name: "Bengali (বাংলা)", is_indic: true, has_vedic_pitch: true },
  { id: "iast", name: "IAST (Roman Diacritics)", is_indic: false, has_vedic_pitch: true },
  { id: "iso15919", name: "ISO 15919", is_indic: false, has_vedic_pitch: true },
  { id: "slp1", name: "SLP1 (ASCII Phonetic)", is_indic: false, has_vedic_pitch: false },
  { id: "harvardkyoto", name: "Harvard-Kyoto (HK)", is_indic: false, has_vedic_pitch: false },
  { id: "wx", name: "WX Notation", is_indic: false, has_vedic_pitch: false }
]
*/
```

---

### Function 4: `generate_krama`

Generates structured step-by-step Krama recitation data. Designed for UI data tables (`DataGrid`) and interactive chanting flashcards.

```typescript
const steps = generate_krama(
  "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।",
  "devanagari" // target script
);

console.log(steps);
/* Output:
[
  {
    step_number: 1,
    formula: "1-2",
    first_index: 1,
    second_index: 2,
    raw_pada: "अ॒ग्निम् ई॒ळे॒",
    sandhied: "अ॒ग्निमी॒ळे॒",
    is_parigraha: false,
    pragrhya_detected: false
  },
  {
    step_number: 2,
    formula: "2-3",
    first_index: 2,
    second_index: 3,
    raw_pada: "ई॒ळे॒ पु॒रो-हि॑तम्",
    sandhied: "ई॒ळे॒ पु॒रो-हि॑तम्",
    is_parigraha: false,
    pragrhya_detected: false
  },
  {
    step_number: 3,
    formula: "3-iti-3",
    first_index: 3,
    second_index: null,
    raw_pada: "पु॒रो-हि॑तम्",
    sandhied: "पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम्",
    is_parigraha: true,
    pragrhya_detected: false
  }
]
*/
```

---

### Function 5: `generate_krama_text`

Generates continuous, traditional Krama-pāṭha recitation text directly in the requested target script.

```typescript
const kramaChant = generate_krama_text(
  "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।",
  "telugu"
);

console.log(kramaChant);
// Output: "అ॒గ్నిమీ॒ళే॒ । ఈ॒ళే॒ పు॒రో-హి॑తమ్ । పు॒రోహి॑తమితి॑ పు॒రో-హి॑తమ్ ॥"
```

---

### Function 6: `generate_jata`

Generates structured step-by-step Jaṭā recitation data ($1\text{-}2, 2\text{-}1, 1\text{-}2$). Combines forward and reverse Sandhi (including Pāṇini 6.1.109 Abhinihita avagraha elision) with compound and terminal Parigraha clauses.

```typescript
const jataSteps = generate_jata(
  "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।",
  "devanagari" // target script
);

console.log(jataSteps);
/* Output:
[
  {
    step_number: 1,
    formula: "1-2-2-1-1-2",
    first_index: 1,
    second_index: 2,
    forward_12: "अ॒ग्निमी॑ळे",
    reverse_21: "ई॒ळे॒ऽग्निम्",
    forward_return_12: "अ॒ग्निमी॑ळे",
    parigraha: null,
    full_step_text: "अ॒ग्निमी॑ळ ई॒ळे॒ऽग्निर॒ग्निमी॑ळे"
  },
  {
    step_number: 2,
    formula: "2-3-3-2-2-3",
    first_index: 2,
    second_index: 3,
    forward_12: "ई॒ळे॒ पु॒रोहि॑तम्",
    reverse_21: "पु॒रोहि॑तमी॑ळे",
    forward_return_12: "ई॒ळे॒ पु॒रोहि॑तम्",
    parigraha: "पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम्",
    full_step_text: "ई॒ळे॒ पु॒रोहि॑तं पु॒रोहि॑तमी॑ळ ई॒ळे॒ पु॒रोहि॑तं पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम्"
  }
]
*/
```

---

### Function 7: `generate_jata_text`

Generates continuous, traditional Jaṭā-pāṭha recitation text directly in the requested target script.

```typescript
const jataChant = generate_jata_text(
  "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।",
  "devanagari"
);

console.log(jataChant);
// Output: "अ॒ग्निमी॑ळ ई॒ळे॒ऽग्निर॒ग्निमी॑ळे । ई॒ळे॒ पु॒रोहि॑तं पु॒रोहि॑तमी॑ळ ई॒ळे॒ पु॒रोहि॑तं पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम् ।"
```

---

### Function 8: `parse_padas`

Parses raw Pada-pāṭha strings into structured grammatical tokens, identifying compound boundaries (samāsa) and Pāṇinian Pragṛhya vowels (Pāṇini 1.1.11–19).

```typescript
const padas = parse_padas("अ॒ग्निम् । हरी॒ । पु॒रो-हि॑तम् ।");

console.log(padas);
/* Output:
[
  {
    raw: "अ॒ग्निम्",
    clean: "अग्निम्",
    compound_parts: [],
    is_compound: false,
    is_pragrhya: false,
    pragrhya_type: null
  },
  {
    raw: "हरी॒",
    clean: "हरी",
    compound_parts: [],
    is_compound: false,
    is_pragrhya: true,
    pragrhya_type: "DualI" // Pāṇini 1.1.11: dual ending in long ī
  },
  {
    raw: "पु॒रो-हि॑तम्",
    clean: "पुरोहितम्",
    compound_parts: ["पुरो", "हितम्"],
    is_compound: true,
    is_pragrhya: false,
    pragrhya_type: null
  }
]
*/
```

---

### Function 9: `get_shiva_sutras`

Returns all 14 Māheśvara Sūtras (Śiva Sūtras) with individual sound tokens and terminating *it*-markers in both Devanagari and IAST.

```typescript
const sutras = get_shiva_sutras();

// Inspect Sūtra 1 (अ इ उ ण्)
console.log(sutras[0]);
/* Output:
{
  index: 1,
  name: "aiuṇ",
  it_marker_deva: "ण्",
  it_marker_iast: "ṇ",
  sounds_deva: ["अ", "इ", "उ"],
  sounds_iast: ["a", "i", "u"]
}
*/
```

---

### Function 10: `get_pratyahara_sounds`

Resolves any canonical Pāṇinian Pratyāhāra (sound abbreviation) into its constituent phonemes with full articulatory properties.

```typescript
// Resolve 'yaṇ' (यण्) = semivowels (y, v, r, l)
const yanSounds = get_pratyahara_sounds("yaṇ");

console.log(yanSounds.map(s => s.glyph_iast));
// Output: ["y", "v", "r", "l"]
```

---

### Function 11: `check_pratyahara_contains`

A high-speed boolean predicate checking if a given sound belongs to a Pratyāhāra abbreviation.

```typescript
// Is 'i' in 'ac' (all vowels)?
console.log(check_pratyahara_contains("ac", "i")); // true

// Is 'k' in 'ac'?
console.log(check_pratyahara_contains("ac", "k")); // false

// Is 'k' in 'hal' (all consonants)?
console.log(check_pratyahara_contains("hal", "k")); // true

// Is 'y' in 'yaṇ' (semivowels)?
console.log(check_pratyahara_contains("yaṇ", "y")); // true
```

---

### Function 12: `inspect_varna`

Computes the classical articulatory phonetics (*Śikṣā* tradition) for any single Sanskrit sound.

```typescript
const analysis = inspect_varna("k");

console.log(analysis);
/* Output:
{
  glyph_deva: "क",
  glyph_iast: "k",
  varna_type: "consonant",
  sthana: ["Kantha (Velar)"],
  abhyantara_prayatna: "Sprshta (Complete contact)",
  is_ghosha: false,     // Aghoṣa (voiceless)
  is_alpaprana: true,   // Alpaprāṇa (unaspirated)
  matra: 0.5            // Ardhamātrā (consonant weight)
}
*/
```

---

### Function 13: `analyze_syllables`

Performs syllable-level analysis of an entire Sanskrit word or verse, decomposing each Akṣara into its consonants, vowels, accents, and total mātrā weights.

```typescript
const syllables = analyze_syllables("अ॒ग्निः", "devanagari");

console.log(syllables);
/* Output:
[
  {
    surface: "अ",
    consonants: [],
    vowel: { glyph_deva: "अ", glyph_iast: "a", matra: 1.0, ... },
    ayogavaha: null,
    svara: "Anudatta",
    total_matra: 1.0
  },
  {
    surface: "ग्निः",
    consonants: [
      { glyph_deva: "ग", glyph_iast: "g", matra: 0.5, ... },
      { glyph_deva: "न", glyph_iast: "n", matra: 0.5, ... }
    ],
    vowel: { glyph_deva: "इ", glyph_iast: "i", matra: 1.0, ... },
    ayogavaha: "Visarga",
    svara: null,
    total_matra: 2.5
  }
]
*/
```

---

### Function 14: `get_taittiriya_svaritas`

Returns the canonical 8-fold Svarita accent varieties defined in *Taittirīya-Prātiśākhya* Chapter 20, identifying whether each accent is *Nitya* (independent/inherent) or enclitic (contextual).

```typescript
const svaritas = get_taittiriya_svaritas();

console.log(svaritas);
/* Output:
[
  { id: "Jatya", name_deva: "जात्य", name_iast: "Jātya", is_nitya: true },
  { id: "Kshaipra", name_deva: "क्षैप्र", name_iast: "Kṣaipra", is_nitya: true },
  { id: "Abhinihita", name_deva: "अभिनिहित", name_iast: "Abhinihita", is_nitya: true },
  { id: "Prashlishta", name_deva: "प्रश्लिष्ट", name_iast: "Praśliṣṭa", is_nitya: true },
  { id: "Tairovyanjana", name_deva: "तैरोव्यञ्जन", name_iast: "Tairovyañjana", is_nitya: false },
  { id: "Tairovirama", name_deva: "तैरोविराम", name_iast: "Tairovirāma", is_nitya: false },
  { id: "Padavrtta", name_deva: "पादवृत्त", name_iast: "Pādavṛtta", is_nitya: false },
  { id: "Tathabhavya", name_deva: "तथाभाव्य", name_iast: "Tathābhāvya", is_nitya: false }
]
*/
```

---

### Function 15: `inspect_taittiriya_varna`

Computes the articulatory classification for a single sound symbol under the *Taittirīya-Prātiśākhya* Chapter 2 framework, calculating both the passive place (*Sthāna*) and active articulator (*Karaṇa*).

```typescript
const analysis = inspect_taittiriya_varna("t");

console.log(analysis);
/* Output:
{
  glyph_deva: "त",
  glyph_iast: "t",
  varna_type: "consonant",
  sthana: ["Danta (Dental)"],
  karana: "Jihvāgram (Tongue tip)",
  abhyantara_prayatna: "Spṛṣṭa (Complete contact)",
  is_ghosha: false,
  is_alpaprana: true,
  matra: 0.5
}
*/
```

---

### Function 16: `check_taittiriya_dvitva`

Evaluates whether a consonant geminates (doubles) in Taittirīya recitation according to the rules of *Taittirīya-Prātiśākhya* Chapter 14 (e.g. TPr 14.1 post-vocalic conjuncts, TPr 14.4 consonants following *r* or *h*).

```typescript
// In "arkaḥ" (अ॒र्कः॑): does 'k' preceded by 'r' double?
const doublesArka = check_taittiriya_dvitva("r", "k", null);
console.log(doublesArka); // true => "arkkaḥ"

// Does initial 'k' double before a vowel without preceding consonant?
const doublesSimple = check_taittiriya_dvitva(null, "k", "a");
console.log(doublesSimple); // false
```

---

### Function 17: `classify_taittiriya_svarita_by_context`

Classifies an accent into one of the 8 canonical *Taittirīya-Prātiśākhya* Svaritas given its phonological juncture context.

```typescript
// Semivowel Sandhi (e.g., ví + abravīt -> vyàbravīt)
const svarita1 = classify_taittiriya_svarita_by_context("SemivowelSandhi");
console.log(svarita1); // "Kshaipra"

// Avagraha elision (e.g., té + abruvan -> té 'bruvan)
const svarita2 = classify_taittiriya_svarita_by_context("AbhinihitaElision");
console.log(svarita2); // "Abhinihita"

// Post-Udātta enclitic across consonant
const svarita3 = classify_taittiriya_svarita_by_context("PostUdattaConsonant");
console.log(svarita3); // "Tairovyanjana"
```

---

## 5. SvelteKit Integration Example

Here is a complete, production-ready Svelte component using the WASM module:

```svelte
<!-- SanskritWorkbench.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import init, { transliterate, generate_krama } from '@project-vyasa/sanskrit-wasm';

  let isReady = false;
  let input = 'अ॒ग्निमी॑ळे पु॒रोहि॑तम्';
  let targetScript = 'telugu';
  let transliterated = '';
  let kramaSteps: any[] = [];

  onMount(async () => {
    await init();
    isReady = true;
    update();
  });

  function update() {
    if (!isReady) return;
    transliterated = transliterate(input, 'devanagari', targetScript);
    kramaSteps = generate_krama(input, targetScript);
  }

  $: if (input || targetScript) update();
</script>

{#if !isReady}
  <div class="loading">Loading Sanskrit WASM engine...</div>
{:else}
  <div class="workbench">
    <textarea bind:value={input} rows="3" />
    
    <select bind:value={targetScript}>
      <option value="telugu">Telugu</option>
      <option value="grantha">Grantha</option>
      <option value="kannada">Kannada</option>
      <option value="iast">IAST</option>
    </select>

    <div class="output">{transliterated}</div>

    <div class="krama-list">
      {#each kramaSteps as step}
        <div class="krama-step">
          <span class="step-num">{step.step_number}.</span>
          <span class="step-formula">{step.formula}</span>
          <strong class="step-text">{step.sandhied}</strong>
        </div>
      {/each}
    </div>
  </div>
{/if}
```

---

## 6. Data-Driven Test Harness & Regression Suite

To ensure absolute fidelity to traditional Vedic recitation without continually modifying Rust test files when new verses are added, `vyasa-patha` includes a **data-driven test harness** (`crates/vyasa-patha/tests/data_driven_krama.rs`).

### Architecture: Everyday Regression vs. Pre-Release Verification

```text
Test Harness Architecture
├── Everyday Regression Suite (<1 ms)
│   └── crates/vyasa-patha/tests/data/rv_01_001.json
│       ├── Standalone bundled dataset (all 9 ṛks of Sūkta 1.1)
│       ├── Validates Pada parsing, Svarita shift, Ardharca limits, and Parigraha
│       └── Runs unconditionally in CI/CD without local filesystem dependencies
│
└── Pre-Release Verification Suite (~10 ms)
    └── Driven by local data pipeline: sa.wikisource.org/data/processed/rigveda
        ├── Iterates through full Sūktas (e.g. Mandala 1 Sūktas 001 - 020)
        ├── Tests hundreds of ṛks and thousands of Krama steps
        └── Enabled dynamically via environment variables
```

### 1. Running Everyday Regression Tests
Run the bundled regression tests (<30 ms for both RV 1.1 and the 169-verse curated tricky suite):
```bash
# Run both everyday and curated tricky suites
cargo test -p vyasa-patha --test data_driven_krama

# Run only the 169-verse curated tricky suite
cargo test -p vyasa-patha --test data_driven_krama test_curated_tricky_suite -- --nocapture
```

The test runner will output:
```text
✓ Curated Tricky Suite: Validated 169 verses and 2947 Krama steps across 11 canonical hymns in <20ms!
test test_curated_tricky_suite ... ok
```

### 2. Dataset Management with `scripts/rigveda_dataset.py`
To synchronize, inspect, or add verses from the Wikisource pipeline without writing ad-hoc scripts:
```bash
# Inspect current curated tricky dataset
python3 scripts/rigveda_dataset.py inspect

# Rebuild the curated tricky dataset from the pipeline
python3 scripts/rigveda_dataset.py build-tricky

# Add or replace a specific verse (e.g. RV 1.164.1)
python3 scripts/rigveda_dataset.py add -m 1 -s 164 -r 1 -c "Asya Vamasya Opening"
```

### 3. Running Pre-Release Pipeline Verification
Prior to a release or significant crate update, run the harness against the comprehensive Wikisource pipeline:
```bash
RIGVEDA_PIPELINE_PATH="/Users/anand/Projects/project-vyasa/sa.wikisource.org/data/processed/rigveda" \
RIGVEDA_SAMPLE_SUKTAS=20 \
cargo test -p vyasa-patha --test data_driven_krama -- --nocapture
```

The test runner will output:
```text
=== Pre-Release Krama Pipeline Test ===
Checking pipeline directory: .../processed/rigveda
Processing Sukta file: .../001.vy (9 verses)
Processing Sukta file: .../002.vy (9 verses)
Processing Sukta file: .../003.vy (13 verses)
Processing Sukta file: .../004.vy (10 verses)
Processing Sukta file: .../005.vy (10 verses)
✓ Validated 51 ṛks and generated 600+ Krama steps successfully!
```

### 4. Krishna Yajurveda Data-Driven Harness (`kyv_corpus.json`)

Modeled directly after the Rigveda harness, `vyasa-phonetics` includes a dedicated corpus-driven test suite (`crates/vyasa-phonetics/tests/data_driven_taittiriya.rs`) driven by a standalone, version-controlled JSON dataset (`crates/vyasa-phonetics/tests/data/kyv_corpus.json`).

#### Curated Benchmark Corpus
The dataset bundles 7 canonical passages across the 3 core Taittirīya texts:
1. **Taittirīya Saṃhitā**:
   - TS 1.1.1 (Opening: *iṣe tvorje tvā...*)
   - TS 4.5.1 (*Śrī Rudram / Namakam*)
   - TS 4.7.1 (*Camakam*)
2. **Taittirīya Upaniṣad**:
   - *Śīkṣāvallī* 1.1.1 (*śam no mitraḥ śam varuṇaḥ...*)
   - *Ānandavallī* 2.1.1 (*brahmavid āpnoti param...*)
3. **Taittirīya Āraṇyaka**:
   - TA 3.12.1 (*Puruṣa Sūkta*: *sahasraśīrṣā puruṣaḥ...*)
   - TA 3.12.16 (*Puruṣa Sūkta* Phalaśruti)

#### Running the KYV Test Harness
```bash
# Run the complete Taittirīya data-driven regression suite (<5 ms)
cargo test -p vyasa-phonetics --test data_driven_taittiriya -- --nocapture
```

The harness automatically executes:
- **Karaṇa & Sthāna exhaustive check**: Validates active articulator coordinates across all Sanskrit varṇas.
- **Svarita taxonomy verification**: Evaluates 25 distinct Svarita context tokens across the 7 corpus passages.
- **Dvitva gemination verification**: Validates 12 consonant doubling occurrences under TPr 14.1, 14.4, and 14.8.

#### Corpus Management with `scripts/kyv_dataset.py`
```bash
# Validate JSON structure and phonological schemas
python3 scripts/kyv_dataset.py validate

# Inspect all 7 benchmark passages with detailed phonetic annotations
python3 scripts/kyv_dataset.py inspect
```

---

## 7. Recommended Unicode Font Stack

The WASM/CLI emit Unicode. Tofu (empty boxes) means the **page never loaded a font that covers that script**, not that Lipi failed. Longer field note in this repo: `notes/indic-fonts.md` (Studio + Starlight incidents, cmap vs Google Fonts slices, whether a coverage CLI is worth it).

**Rule:** every family you list in `font-family` must be installed or fetched (`<link>` / `@font-face`). A CSS name with no file is a no-op. Google Fonts also **slices** WOFF2 by `unicode-range`; Noto Sans Mono’s Latin slice often omits Vedic `U+0331`, so prefer Menlo/Monaco/Courier for monospace IAST pitch.

Load at least: Noto Sans Devanagari, Telugu, Kannada, Malayalam, Bengali, Noto Serif Grantha, and Noto Sans (Latin). Then:

```css
.font-sanskrit {
  font-family:
    "Noto Sans Devanagari", "Noto Sans Telugu", "Noto Sans Kannada", "Noto Serif Grantha",
    "Noto Sans Malayalam", "Noto Sans Bengali",
    "Noto Sans", "Gentium Plus", sans-serif;
  font-feature-settings: "kern" 1, "liga" 1;
}

.font-sanskrit-mono {
  font-family: Menlo, Monaco, "Courier New", monospace;
}
```

Apply that class (or the same stack on `--font-sans`) to **tables and selects**, not only `<textarea>`. Inter/`system-ui` alone will box Malayalam and Bengali in Chromium.

---

## 8. Appendix: Traditional Phonological Assessment & The "Tricky" Ṛgveda Suite

A comprehensive survey of all **10,547 verses across all 1,028 Sūktas (10 Maṇḍalas)** in the pipeline reveals why certain Vedic hymns are regarded by traditional Vaidikas, Ghanapāṭhins, and phonologists as the ultimate benchmark for recitation engines.

### The 6 Categories of Phonological Complexity

| Category | Phonological Phenomenon | Traditional Authority | Canonical Hymns & Triggers |
|:---|:---|:---|:---|
| **1. Dual Pragṛhyas & Compound Parigraha** | Duals in `-ī`, `-ū`, `-e` block sandhi; compound names require *Unified + इति + Split* parigraha. | Pāṇini 1.1.11 (*īdūded dvivacanam*) | **RV 1.2** (`वायो॒ इति॑`, `इन्द्र॑वायू॒ इति॑`, `वाजिनीवसू`), **RV 6.69** (Indrā-Viṣṇū, 7 dual pragṛhyas in 8 verses). |
| **2. Pronoun Sandhi & Refrains** | Pronoun `सः` (*saḥ*) obligatorily drops visarga before consonants, but retains it before vowels. | Pāṇini 6.1.132 (*eta-tadoḥ sulopo 'kor anañ-sve hali*) | **RV 2.12** (Indra Sūkta): 15 verses each ending with refrain `स जना॑स॒ इन्द्रः॑` (`सः` + `जनासः` $\to$ `स जनास`). |
| **3. Svarita Accent Shift & Exceptions** | Udātta + Anudātta $\to$ Svarita ($U + A \to S$), but suppressed if following syllable is accented. | Pāṇini 8.4.66 & 8.4.67 (*nodātta-svaritodātta-pade*) | **RV 1.1** (`अ॒ग्निमी॑ळे` shift vs. `दे॒वमृ॒त्विज॑म्` non-shift). |
| **4. Deep Multi-Member Compounds (*Samāsa*)** | Multi-avagraha compounds, nested Bahuvrīhis, and riddle terms requiring clean sandhi in steps. | Śākalya Padapāṭha & Ṛk-Prātīśākhya | **RV 1.164** (*Asya Vāmasya*, 52 verses, 121 compounds, 800+ lateral flaps `ळ`/`ळ्ह`), **RV 9.86 / 9.97** (Pavamāna Soma). |
| **5. Liturgical Core ("Crown Jewels")** | Ubiquitous ritual hymns scrutinized by traditionalists for zero-error sandhi, virāma, and neuter forms. | Śrauta & Smārta Liturgical Tradition | **RV 10.90** (Puruṣa Sūkta, 16 verses), **RV 10.125** (Devī Sūkta, 8 verses), **RV 10.129** (Nāsadīya Sūkta, 7 verses). |
| **6. Sacred Metrical Transitions** | Metrical pauses, relative pronoun sandhi (`धियो यो नः`), and boundary Parigrahas. | Gāyatrī, Triṣṭubh, and Jagatī Chandas | **RV 3.62** (incl. Gāyatrī 3.62.10), **RV 7.59** (incl. Mahāmṛtyuñjaya 7.59.12), **RV 1.32** (Indra-Vṛtra epic). |

### The 11 Canonical Hymns in `rv_curated_tricky.json`

The curated test dataset (`crates/vyasa-patha/tests/data/rv_curated_tricky.json`) bundles **169 verses (2,947 Krama steps)** covering all 6 categories:

1. **RV 1.1 (Agni Sūkta - 9 verses)**: Universal opening; tests Pāṇini 8.4.66/67 accent shift, compound unification (`ई॒ळे॒ पु॒रोहि॑तम्`), and ardharca boundaries.
2. **RV 1.2 (Vāyu & Indra-Vāyu - 9 verses)**: Canonical textbook introduction to dual Pragṛhyas (`-ū`), vocative `-o`, and compound Parigraha.
3. **RV 1.32 (Indra Vṛtra-vadha - 15 verses)**: Classical epic narrative phonology, dense consonant clusters, and aspirated lateral flaps (`वृ॒ळ्हम्`).
4. **RV 1.164 (*Asya Vāmasya* - 52 verses)**: Supreme phonological test of Maṇḍala 1; 121 compounds, 800+ lateral flaps, and philosophical riddle constructions.
5. **RV 2.12 (Indra Sūkta / Gṛtsamada - 15 verses)**: Celebrated refrain hymn testing pronoun visarga drop `स जनास इन्द्रः`.
6. **RV 3.62 (Viśvāmitra - 18 verses)**: Contains the Gāyatrī mantra (3.62.10); tests relative pronoun sandhi (`धियो॒ यो नः॑`).
7. **RV 6.69 (Indrā-Viṣṇū - 8 verses)**: Highest concentration of dual Pragṛhyas in the Ṛgveda (7 pragṛhya `iti` clauses in 8 verses).
8. **RV 7.59 (Maruts - 12 verses)**: Contains the Mahāmṛtyuñjaya mantra (7.59.12); tests metrical transitions.
9. **RV 10.90 (Puruṣa Sūkta - 16 verses)**: Chanted across all Vedic rituals; dense with compounds (`स॒हस्र॑-शीर्षा`, `स॒हस्र॑-अक्षः`, `स॒हस्र॑-पात्`) and neuter virāma sandhi.
10. **RV 10.125 (Devī Sūkta / Vāk - 8 verses)**: Sovereign first-person affirmations with intense consonant assimilation.
11. **RV 10.129 (Nāsadīya Sūkta - 7 verses)**: The Hymn of Creation; negative particle sandhi (`नास॑दासी॒न्नो सदा॑सीत्`), interrogatives, and pluta.

