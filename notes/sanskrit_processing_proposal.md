# Architecture Proposal & Ecosystem Survey: Vedic & Classical Sanskrit Processing Engine (`vyasa-sanskrit`)

**Repository:** `project-vyasa/vyutils`  
**Target Location:** `vyutils/crates/` (`vyasa-phonetics`, `vyasa-lipi`, `vyasa-patha`)  
**Scope:** Pre-Pāṇinian (Prātiśākhyas & Śikṣās), Pāṇinian, and Non-Pāṇinian Phonetics; Recitation Mode Generation (Prakṛti & Aṣṭa-Vikṛti Pāṭhas); and Vedic-aware Universal Multi-Script Transliteration (Indic scripts + strict ISO 15919 / IAST).

---

## 1. Executive Summary & Vision

Vedic and Classical Sanskrit computational linguistics requires a paradigm shift. Existing Sanskrit computational toolkits are almost exclusively designed around **Post-Pāṇinian (Classical) Sanskrit** using Pāṇini's *Aṣṭādhyāyī* and the *Māheśvara Sūtras*. They fail when applied to **Vedic literature**—specifically the *Prātiśākhyas* (Śaunaka’s *Ṛgveda-Prātiśākhya*, *Taittirīya-Prātiśākhya*, etc.)—which feature:
1. Unique phonetic inventories and sequence models (e.g. the Śaiśirīya vowel sequence: a, ṛ, i, u, e, o, ai, au).
2. Specialized phonetic classifications (*Nāmin*, *Samānākṣara*, *Sandhyakṣara*, *Rakta*, *Aghoṣa*).
3. Suprasegmental accentuation (*Udātta*, *Anudātta*, *Svarita*, *Kampa*) that must be preserved and dynamically recomputed during permutations.
4. Recitation algorithms (*Prakṛti* and *Aṣṭa-Vikṛti* pāṭhas) governed by strict *Parigraha* and *Pragṛhya* sandhi laws.

We are implementing **`vyasa-sanskrit`** as a suite of modular, zero-dependency **Rust crates inside the `vyutils` workspace**:
- **Native CLI tools** inside `vyutils` for high-throughput batch conversions and corpus transformations.
- **WebAssembly (WASM) components** with a minimal memory footprint for real-time client-side rendering and interactive transformations inside **`vyasa-viewer`** (`vyasav` / Svelte).

---

## 2. Comprehensive Survey of Existing Software & Gap Analysis

### 2.1 Comparative Matrix of Existing Libraries

| Library / Tool | Primary Language | Classical / Pāṇinian | Vedic / Prātiśākhya | Multi-Script Indic | Accent Preservation | Recitation Pāṭhas | WASM / Browser Ready |
|---|---|---|---|---|---|---|---|
| **Aksharamukha** (Vinodh Rajan) | Python (Offline / API) | N/A (Script only) | Basic character mapping | Excellent (120+ scripts) | Good (character level, no svara recomputation) | None | No (requires Pyodide or server) |
| **indic_transliteration / sanscript** | Python & JS | Basic | None | Good (Common scripts) | Poor (frequently strips or corrupts accents) | None | Partial (`sanscript.js`, no Vedic support) |
| **Vidyut** (Arun Prasad) | Rust | Outstanding (*Aṣṭādhyāyī*, Sandhi) | Minimal | Basic (Devanagari, SLP1, IAST) | Limited (strips Vedic svaras in sandhi) | None | Yes (native Rust / WASM) |
| **Heritage du Sanskrit** (Gérard Huet) | OCaml | World-class Classical Lexicon & Sandhi | None | Devanagari, IAST | None (Classical unaccented only) | None | No (CGI / Web server) |
| **UoH Sanskrit Parser** (Amba Kulkarni) | Shell / C / Perl | Advanced Dependency Parser | None | Devanagari, WX | None | None | No (Server-side) |
| **ITranslator / Omkarananda** | Windows C++ | Basic | Legacy Vedic font conversion | Limited | Partial (Legacy encoding to Unicode) | None | No |
| **Amateur GitHub Pāṭha Scripts** | Python / Perl | None | Naive string concatenation | Devanagari only | Broken | Flawed (no sandhi, no *iti* clauses, no svara rules) | No |

---

### 2.2 In-Depth Analysis of Critical Gaps

#### Gap 1: Inability to Preserve or Recompute Vedic Svaras during Transformations
- In Vedic literature, every vowel possesses a pitch accent (*svara*): *Udātta* (high pitch), *Anudātta* (low pitch, unmarked or underscored `\u0952`), and *Svarita* (falling pitch, vertical top stroke `\u0951`).
- When words are combined in sandhi or permuted into *Krama*, *Jaṭā*, or *Ghana* pāṭhas:
  - Two unaccented syllables can become *Svarita* (*Jātya*, *Kṣaipra*, *Abhinihita*).
  - An *Anudātta* preceding a *Svarita* changes character.
  - Independent accents combine according to Vedic rules (e.g., Udātta + Anudātta = Svarita).
- **Current reality:** Tools like `indic_transliteration` or `sanscript` treat Vedic accent codepoints as non-spacing combining characters or discard them entirely. No existing library dynamically recomputes Vedic tone changes during sandhi fusion.

#### Gap 2: Conflation of ISO 15919 with IAST
- Standard **IAST** does not distinguish between short and long e and o, nor does it provide clean distinctions for modern Dravidian vowels or Vedic sonorants.
- **ISO 15919** is the international standard:
  - Represents Sanskrit vocalic r and l with under-rings (r̥, r̥̄, l̥, l̥̄), whereas IAST uses under-dots (ṛ, ṝ, ḷ, ḹ).
  - Distinguishes short e, o from long ē, ō.
  - Supports the Vedic retroflex lateral flap ळ (ḷa / ḻa &rarr; ISO ḻ, IAST often confused with vocalic ḷ) and lateral aspirate ळ्ह (ḷha &rarr; ḻh).
- Existing libraries often claim "IAST/ISO" support but actually deliver a hybrid that breaks strict ISO 15919 round-trip conformance.

#### Gap 3: Absence of Prātiśākhya Phonetic Sequences in Software
- All modern phonetic libraries assume Pāṇini's *Śiva Sūtras* (a-i-u-ṇ, ṛ-ḷ-k...).
- In the *Ṛgveda-Prātiśākhya*, Śaunaka formulates rules around a sequence where ṛ directly follows a (a, ṛ, i, u, e, o, ai, au).
- The Prātiśākhya defines the **Nāmin** vowels (all vowels except a and ā) which trigger retroflexion (*Nati* - turning s &rarr; ṣ and n &rarr; ṇ).
- Without an engine that understands Prātiśākhya-specific groupings, computational analysis of texts like the *Ṛgveda-Prātiśākhya* is impossible.

#### Gap 4: Naive Recitation Pāṭha Generators
- Existing pāṭha scripts generate *Krama* or *Ghana* via string manipulation:
  $$\text{Word}_1 + \text{Word}_2, \quad \text{Word}_2 + \text{Word}_1, \quad \text{Word}_1 + \text{Word}_2 + \text{Word}_3 ...$$
- **Why this fails catastrophically:**
  1. **Sandhi application:** Word 1 and Word 2 must undergo euphonic fusion in the forward direction, but reverse combinations (Word 2 + Word 1) require reverse sandhi computation.
  2. **Pragṛhya & Parigraha:** In the Vedas, dual vowels and certain pronominal terminations (e.g. *harī*, *amū*) are *Pragṛhya* (immune to sandhi). In *Krama* and *Vikṛti* pāṭhas, they require an **"iti" clause** (*Parigraha* / *Sthita-Upasthita*): e.g., *harī* &rarr; *harī iti harī*. Current scripts completely omit *iti* insertions.
  3. **Loss of Svara:** Permuting words without re-accentuation violates Vedic recitation rules.

---

## 3. The Landscape of Indian Phonetic and Grammatical Traditions

To design a truly robust computational library, we must understand the full spectrum of Indian phonetic and grammatical traditions beyond just Śaunaka and Pāṇini:

```
                          Sanskrit Linguistic Traditions
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
Vedic Traditions               General Śikṣā Texts             Post-Vedic Systems
(Branch-Specific)              (Articulatory Phonetics)        (Classical Grammars)
       |                                |                                |
 • Ṛgveda-Prātiśākhya            • Pāṇinīya Śikṣā                • Pāṇinian (Aṣṭādhyāyī)
 • Taittirīya-Prātiśākhya        • Yājñavalkya Śikṣā             • Kātantra / Kalāpa
 • Vājasaneyi-Prātiśākhya        • Māṇḍūkī Śikṣā                 • Cāndra (Buddhist)
 • Atharvaveda-Prātiśākhya       • Nārada Śikṣā (Musical Svaras) • Jainendra & Śākaṭāyana
 • Ṛktantra / Sāmatantra         • 8 Sthānas, Karaṇa,            • Sārasvata & Mugdhabodha
                                   Ābhyantara/Bāhya Prayatnas    • Harināmāmṛta
                                                                 • Laukika Varṇamālā
```

### 3.1 Branch-Specific Prātiśākhyas (Vedic)
1. **Ṛgveda-Prātiśākhya (Śaunaka / Śaiśirīya)**:
   - Alphabet sequence: a, ṛ, i, u, e, o, ai, au.
   - Core concepts: *Samānākṣara* (monophthongs), *Sandhyakṣara* (diphthongs), *Nāmin* vowels, *Aghoṣa/Ghoṣa*, *Nati* (retroflexion).
2. **Taittirīya-Prātiśākhya (Kṛṣṇa Yajurveda)**:
   - Detailed treatment of syllable weight (*mātrā*), transition consonants (*ch*, *dh* augments), and intricate accent rules (*Svarita* varieties: *Nityasvarita*, *Kṣaipra*, *Abhinihita*, *Praśliṣṭa*).
   - Major commentaries: *Tribhāṣyaratna* and *Vaidikābharaṇa*.
3. **Vājasaneyi-Prātiśākhya (Kātyāyana - Śukla Yajurveda)**:
   - Bridges the Prātiśākhya and Pāṇinian traditions; introduces sutras very close to Pāṇinian formulations while retaining Vedic accentual precision.
4. **Atharvaveda-Prātiśākhya & Śaunakiya Caturādhyāyikā**:
   - Focuses on the unique phonetic variations of the Śaunakiya Atharvaveda recension.
5. **Ṛktantra and Sāmatantra (Sāmaveda)**:
   - Details the phonetic transition of spoken Vedic accents (Udātta, Anudātta, Svarita) into the musical scale of seven notes (Kruṣṭa, Prathama, Dvitīya, Tṛtīya, Caturtha, Mandra, Atisvārya).

### 3.2 The General Śikṣā Tradition (Articulatory Phonetics)
Texts like the *Pāṇinīya Śikṣā*, *Yājñavalkya Śikṣā*, and *Māṇḍūkī Śikṣā* define the physiological mechanics of speech production:
- **8 Places of Articulation (*Aṣṭau Sthānāni*)**: Chest (*Uras*), Throat (*Kaṇṭha*), Head/Crown (*Mūrdhan*), Tongue root (*Jihvāmūla*), Teeth (*Danta*), Nose (*Nāsikā*), Lips (*Oṣṭha*), Palate (*Tālu*).
- **Active Organ (*Karaṇa*)**: The tongue blade, tip, center, or edges.
- **Internal Effort (*Ābhyantara Prayatna*)**: *Spṛṣṭa* (complete stop/contact), *Īṣat-spṛṣṭa* (semi-contact / semivowels), *Īṣad-vivṛta* (slight opening / sibilants), *Vivṛta* (open / vowels), *Saṁvṛta* (closed / short *a* in actual speech).
- **External Effort (*Bāhya Prayatna*)**: 11 acoustic properties:
  - *Vivāra, Śvāsa, Aghoṣa* (Unvoiced stops and sibilants).
  - *Saṁvāra, Nāda, Ghoṣa* (Voiced sounds).
  - *Alpaprāṇa* (Unaspirated) vs. *Mahāprāṇa* (Aspirated).
  - *Udātta, Anudātta, Svarita* (Pitch accents).

### 3.3 Non-Pāṇinian Post-Vedic Grammars (*Aṣṭa-Vyākaraṇa*)
1. **Aindra Vyākaraṇa**: Pre-dating Pāṇini, attributed to Indra; it influenced early Prakrit grammarians and the Tamil *Tolkāppiyam*.
2. **Kātantra (or Kalāpa)** (Śarvavarman, c. 1st cent. CE): Designed to teach Sanskrit quickly without Pāṇini's cryptic *Māheśvara Pratyāhāras*; arranges the alphabet in the natural, intuitive order (*Varṇamālā*). Widely used for centuries in Kashmir and Bengal.
3. **Cāndra Vyākaraṇa** (Candragomin, Buddhist tradition): Stripped Pāṇini of Vedic accents and religious rules to create a secular, pan-Asian Sanskrit grammar.
4. **Jainendra Vyākaraṇa** (Pūjyapāda / Devanandin) & **Śākaṭāyana Vyākaraṇa**: Influential Jain grammatical systems.
5. **Sārasvata Vyākaraṇa** (Anubhūti Svarūpācārya): Extremely popular in medieval India for its simplified sandhi and declension rules.
6. **Harināmāmṛta Vyākaraṇa** (Jīva Gosvāmī, Gauḍīya Vaiṣṇava): Replaces all technical grammatical symbols with sacred names (e.g., vowels are *Sarveśvara*, consonants are *Viṣṇujana*, cases are *Viṣṇubhakti*).

### 3.4 The Standard Laukika *Varṇamālā* (Pan-Indic Orthographic Order)
The alphabet taught across Indian schools (a, ā, i, ī, u, ū, ṛ, ṝ, ḷ, ॡ, e, ai, o, au, aṁ, aḥ, followed by ka through ha) is a post-Vedic pedagogical synthesis that differs from both Pāṇini's *Śiva Sūtras* and Śaunaka's *Prātiśākhya*.

---

## 4. Architecture of the `vyutils` Crates

We will implement this library as three focused crates inside the `vyutils` workspace:

```
/Users/anand/Projects/project-vyasa/vyutils/crates/
├── vyasa-phonetics/     # Pre- & Post-Pāṇinian sound models, character sets, and classifications
├── vyasa-lipi/          # Universal Vedic transliteration (Indic + strict ISO 15919 / IAST + Vedic accents)
└── vyasa-patha/         # Sandhi, Parigraha ("iti"), and Recitation generator (Krama, Ghana, etc.)
```

```
vyutils Workspace Layout
├── Cargo.toml (workspace members updated)
├── crates/
│   ├── mpt/
│   ├── walk/
│   ├── pack/
│   ├── render/
│   ├── vyasa-phonetics/   <-- [NEW]
│   ├── vyasa-lipi/        <-- [NEW]
│   └── vyasa-patha/       <-- [NEW]
```

### 4.1 Crate 1: `vyasa-phonetics`
- **Zero external dependencies**, `no_std` compatible.
- **Sound inventories**:
  - `PratisakhyaSequence`: Shaunaka's order (a, ṛ, i, u, e, o, ai, au).
  - `PaniniShivaSutras`: 14 Maheshvara sutras and dynamic pratyahara generator.
  - `LaukikaVarnamala`: Standard 16 vowels + 33 consonants.
- **Vedic Character Support**:
  - Jihvāmūlīya (Unicode `U+1CF5` ᳵ) & Upadhmānīya (`U+1CF6` ᳶ).
  - Ardhavisarga.
  - Vedic Anusvāra variations: Gomukha (`U+1CE9`), Dvi-bindu (`U+1CEA`), Chandrabindu (`U+0303`, `U+0901`).
  - Pluta notation (3 / ३).
  - Vedic Dāḍha/Vaḍhava consonants: Lateral flap ळ (`U+0933`) and aspirated ळ्ह (`U+0934`).
- **Phonetic classifiers**:
  - `is_namin(c)`: Checks if a vowel triggers retroflexion (*Nati*).
  - `is_samanakshara(c)`, `is_sandhyakshara(c)`.
  - `is_aghosha(c)`, `is_ghosha(c)`, `is_alpaprana(c)`, `is_mahaprana(c)`.

### 4.2 Crate 2: `vyasa-lipi` (Universal Vedic Transliteration)
- **Zero external dependencies**, compiled to native CLI and WASM.
- **Bidirectional script support**:
  - Indic scripts: Devanagari, Telugu, Kannada, Grantha, Malayalam, Tamil, Bengali, Odia, Gujarati, Gurmukhi, Sharada, Nandinagari.
  - Romanization schemes:
    - **Strict ISO 15919**: Under-rings for liquids (r̥, r̥̄, l̥, l̥̄), explicit macron for long vowels (ē, ō vs e, o), retroflex lateral ḻ.
    - **Standard IAST**: Under-dots (ṛ, ṝ, ḷ, ḹ).
    - **Machine formats**: SLP1, Harvard-Kyoto (HK), WX, Velthuis, ITRANS.
- **Accent preservation engine**:
  - Maps combining marks (`U+0951` Svarita, `U+0952` Anudatta) into correct South Indian script diacritics (e.g., Telugu combining marks or Unicode Vedic extensions) and Roman diacritics (acute, grave, circumflex).
  - Guarantees **100% loss-free round-trip conversion**.

### 4.3 Crate 3: `vyasa-patha` (Recitation Mode Engine)
- Implements the complete mathematical permutation algorithms for:
  - **Prakṛti Pāṭhas**: Saṁhitā-pāṭha, Pada-pāṭha, Krama-pāṭha.
  - **Aṣṭa-Vikṛti Pāṭhas**: Jaṭā, Mālā, Śikhā, Rekhā, Dhvaja, Daṇḍa, Ratha, Ghana.
- **Parigraha Engine**:
  - Detects *Pragṛhya* vowels (dual endings in ī, ū, e) and indeclinables.
  - Automatically inserts canonical *iti* clauses (*Sthita-Upasthita*): e.g., *harī* &rarr; *harī iti harī*.
- **Vedic Sandhi & Svara Engine**:
  - Forward sandhi on paired tokens.
  - Reverse sandhi on inverted pairs (e.g. Word 2 + Word 1 in Jaṭā/Ghana).
  - Svara recomputation on the resulting compound tokens.

---

## 5. Integration Plan with `vyasa-viewer` & CLI

### 5.1 In `vyutils` (Native CLI):
Expose high-performance CLI binaries directly inside `vyutils`:
```bash
# Transliterate entire corpora preserving Vedic accents and strict ISO 15919
vyutils lipi convert rigveda.txt --from devanagari --to telugu --scheme iso15919 -o rigveda_telugu.txt

# Generate Ghana-patha with full Vedic sandhi and "iti" clauses from Padapatha
vyutils patha generate --mode ghana --input rigveda_01_001_pada.txt --script telugu
```

### 5.2 In `vyasav` (WASM / Vyasa Viewer):
`vyasav/Cargo.toml` in `project-vyasa/vyasa` can reference these crates directly:
```toml
[dependencies]
vyasa-lipi = { path = "../../vyutils/crates/vyasa-lipi", default-features = false }
vyasa-patha = { path = "../../vyutils/crates/vyasa-patha", default-features = false }
```
Exposing ultra-fast client-side functions to `vyasa-ui` (Svelte):
```typescript
// Instant script switching without network roundtrips
const teluguText = vyasa.transliterate(samhitaDevanagari, "Devanagari", "Telugu", { accents: true, scheme: "ISO15919" });

// Real-time recitation view toggle (Samhita -> Pada -> Krama -> Ghana)
const ghanaText = vyasa.generatePatha(padaTokens, "Ghana", { script: "Telugu" });
```

---

## 6. Implementation Roadmap

- **Phase 1 (Immediate)**: Initialize `vyasa-phonetics` and `vyasa-lipi` inside `vyutils/crates/`. Implement bidirectional Devanagari &harr; Telugu &harr; Kannada &harr; ISO 15919 &harr; IAST with Vedic svara preservation and comprehensive round-trip tests.
- **Phase 2**: Add remaining Indic scripts (Grantha, Malayalam, Bengali, Sharada, etc.) and ASCII formats (SLP1, HK, WX). Add CLI commands in `vyutils`.
- **Phase 3**: Implement `vyasa-patha` for Prakṛti pāṭhas (Saṁhitā, Pada, Krama) with *Pragṛhya* detection and *iti* (*Parigraha*) insertion.
- **Phase 4**: Implement the Aṣṭa-Vikṛti generators (Jaṭā, Ghana, etc.) with forward/reverse Vedic sandhi and svara recomputation.
- **Phase 5**: Hook crates into `vyasav` for WASM export and integrate interactive UI toggles into `vyasa-ui`.
