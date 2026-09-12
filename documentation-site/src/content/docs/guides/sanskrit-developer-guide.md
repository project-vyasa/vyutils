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

#### 2. Recitation & Krama-pāṭha (`vyasa-patha`)
```rust
use vyasa_patha::{parse_pada_patha, generate_krama_patha, format_krama_patha, Script};

fn main() {
    let pada_text = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";
    let padas = parse_pada_patha(pada_text);
    
    // Generate Krama steps
    let steps = generate_krama_patha(&padas);
    let output = format_krama_patha(&steps);
    println!("{}", output);
    // => "अ॒ग्निमी॑ळे । ई॒ळे॒ पु॒रोहि॑तम् । पु॒रो-हि॑तम् । पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम् ॥"
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

Below is a complete reference with a dedicated code example for each of the 11 exported functions.

```typescript
import init, {
  transliterate,
  detect_script,
  get_supported_scripts,
  generate_krama,
  generate_krama_text,
  parse_padas,
  get_shiva_sutras,
  get_pratyahara_sounds,
  check_pratyahara_contains,
  inspect_varna,
  analyze_syllables
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

### Function 6: `parse_padas`

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

### Function 7: `get_shiva_sutras`

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

### Function 8: `get_pratyahara_sounds`

Resolves any canonical Pāṇinian Pratyāhāra (sound abbreviation) into its constituent phonemes with full articulatory properties.

```typescript
// Resolve 'yaṇ' (यण्) = semivowels (y, v, r, l)
const yanSounds = get_pratyahara_sounds("yaṇ");

console.log(yanSounds.map(s => s.glyph_iast));
// Output: ["y", "v", "r", "l"]
```

---

### Function 9: `check_pratyahara_contains`

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

### Function 10: `inspect_varna`

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

### Function 11: `analyze_syllables`

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
Run the bundled dataset test (takes ~0.00s):
```bash
cargo test -p vyasa-patha --test data_driven_krama
```

### 2. Running Pre-Release Pipeline Verification
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
Processing Sukta file: .../001_001.json (9 verses)
Processing Sukta file: .../001_002.json (9 verses)
Processing Sukta file: .../001_003.json (12 verses)
Processing Sukta file: .../001_004.json (10 verses)
Processing Sukta file: .../001_005.json (10 verses)
✓ Validated 51 ṛks and generated 600+ Krama steps successfully!
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
