# Vedic Krama-Pāṭha, Phonological Rules, and Test Datasets

This document captures the analysis of Vedic Krama-pāṭha mechanics, phonological failure modes, test datasets (including Vidyut and the Wikisource data pipeline), and the Ṛgveda-Prātīśākhya source text.

---

## 1. Why Krama Processing Is Reported as "Not Right"

Traditional Vedic reciters and scholars follow strict phonological, accentual, and structural rules codified in the *Prātīśākhyas* (particularly *Ṛgveda-Prātīśākhya* Paṭalas 10 and 11) and Pāṇini’s *Aṣṭādhyāyī* (Adhyāyas 6 and 8). Standard string concatenation and classical sandhi engines fail in Vedic Krama for four primary reasons:

### 1.1 Svara (Accent) Shift Across Pada Junctures (The Primary Failure Point)
In Pada-pāṭha, every word has independent lexical accents. When $P_n$ and $P_{n+1}$ combine in a Krama step, **their accents interact across the word boundary**:
1. **Udātta + Anudātta $\to$ Svarita**:
   - Pāṇini 8.4.66 (*udāttād anudāttasya svaritaḥ*) & RPr 3.1:
   - When a word ending in an **Udātta** (or Svarita) is followed by an initial syllable that is lexically **Anudātta**, that following syllable **must become a dependent (Jātya/Enclitic) Svarita** (marked with vertical line above, `U+0951` `॑`).
   - *Example from RV 1.1.1*:
     - Pada 1: `अ॒ग्निम्` (udātta on `ग्नि`)
     - Pada 2: `ई॒ळे॒` (lexical anudātta on `ई॒`)
     - Krama Step 1 (`1-2`): `अ॒ग्निमी॑ळे` — the `ई॒` changes from anudātta to **svarita** `मी॑`!
     - If an algorithm outputs `अ॒ग्निमी॒ळे॒`, a traditional reciter will immediately flag it as defective.
2. **Anudāttatara / Sannatara (Pre-accent lowering)**:
   - Pāṇini 1.2.39–40: Syllables preceding an Udātta or Svarita become Anudātta (marked with horizontal underline `॒`).
   - Subsequent unaccented syllables following a Svarita become unmarked (*pracaya* / *ekaśruti*).

### 1.2 Parigraha (`iti`) Rule for Pragṛhya Words
1. **Sandhi Immunity**:
   - Pāṇini 6.1.125 (*pluta-pragṛhyā aci nityam*) & RPr 1.68–74:
   - Words ending in dual $ī, ū, e$, vocative particle $o/aho$, or particle $u$ are **Pragṛhya**. They do **not** undergo vowel sandhi with any following vowel.
2. **Repetition with `iti` (Parigraha)**:
   - In Krama recitation, whenever a Pragṛhya pada finishes its sequence (or is introduced), it requires a separate **Parigraha step** where the word is repeated with the particle `इति`:
     $$\text{Pada } P_2 \implies P_2\ \text{इति}\ P_2$$
   - The particle `इति` itself takes an udātta on its initial syllable (`इति॑`), and the repeated word is pronounced with pausal (avasāna) phonetics.

### 1.3 Compound Word (*Samāsa*) Treatment & Avagraha
1. In Pada-pāṭha, compounds are marked with hyphens or avagrahas (`ऽ` / `-`):
   - *Example*: `पु॒रःऽहि॑तम्` (or `र॒त्न॒ऽधात॑मम्`).
2. In Krama-pāṭha, a compound word is processed in two stages:
   - First, the **entire compound** participates in the step sequence as a single pada:
     $$P_2\ P_3 = \text{ई॒ळे॒ पु॒रोहि॑तम्}$$
   - Then, the compound receives an **iti-parigraha** step that demonstrates its morphological division:
     $$\text{पु॒रोहि॑त॒मिति॑ पु॒रःऽहि॑तम्}$$

### 1.4 Hemistich / Half-Verse Boundary (*Ardharca-Avasāna*)
- Krama recitation **strictly respects the ardharca boundary (marked by single daṇḍa `।`)**.
- In an ardharca (half-verse) containing padas $P_1, P_2, P_3, P_4$:
  $$\text{Steps: } P_1 P_2 \to P_2 P_3 \to P_3 P_4 \to P_4\ \text{इति}\ P_4\ ।$$
- **Rule**: $P_4$ (the last pada before `।`) **never combines with $P_5$** (the first pada of the second half-verse)! It is closed with Parigraha (`iti`), followed by a pause. The second half-verse starts fresh with $P_5 P_6$.

---

## 2. Test Datasets & Comparative Crate Coverage

### 2.1 Vidyut vs Vyutils Crates (License: MIT)
Vidyut is licensed under the permissive **MIT License**, permitting code inspection, test vector porting, and adaptation.

| Vyutils Crate | Vidyut Counterpart | Overlap & Usable Assets | Gaps to Address in Vyutils |
| :--- | :--- | :--- | :--- |
| **`vyasa-lipi`** | `vidyut-lipi` | High overlap on Sanskrit/Pali scripts. Excellent test vectors in `tests/basic.rs` for ZWJ/ZWNJ, Malayalam chillus (ൺ, ൻ, ർ), Tamil superscripts, and Latin schemes (SLP1, IAST, ISO-15919, HK). | Vidyut drops or ignores **Vedic Svara accents** and Rigvedic consonants (ळ `U+0933`, ळ्ह `U+0934`). |
| **`vyasa-phonetics`** | `vidyut-prakriya` | Pāṇinian phonology: Śiva Sūtras, Pratyāhāras (`ac`, `hal`, `ik`, `yaṇ`, etc.), Savarṇa (homorganic) classification, it-markers. | Pre-Pāṇinian phonology: Śikṣā and Prātīśākhya traditions (Śaiśirīya ordering, archaic 4-manner articulation, Jihvāmūlīya/Upadhmānīya sandhi states). |
| **`vyasa-patha`** | *None* | Zero overlap. Vidyut does not implement Vedic Krama-pāṭha or accent transformation rules. | We must implement authoritative Vedic rules ourselves using the Prātīśākhya specifications. |

### 2.2 Preprocessed Rigveda Pipeline (`sa.wikisource.org`)
In `/Users/anand/Projects/project-vyasa/sa.wikisource.org/data/processed/rigveda`:
- **`content/padapatha/<mandala>/<sukta>.vy`**: Full accent-marked Pada-pāṭha for all 10 Mandalas (e.g. `01/001.vy` contains `अ॒ग्निम् । ई॒ळे॒ । पु॒रःऽहि॑तम् । ...`).
- **`content/samhita/<mandala>/<sukta>.vy`**: Full accent-marked continuous Saṁhitā-pāṭha (e.g. `01/001.vy` contains `अ॒ग्निमी॑ळे पु॒रोहि॑तं ...`).
- **Significance**: Comparing the pairwise sandhi of Padapatha directly against the corresponding words in Samhitapatha yields an automated, verified, ground-truth oracle for all 10,552 ṛks!

### 2.3 GRETIL & Digital Prātīśākhya Sources
- **GRETIL (Göttingen)**: Complete electronic texts of the *Ṛgveda-Prātīśākhya*, *Taittirīya-Prātīśākhya*, and *Vājasaneyi-Prātīśākhya*. Clean, proofread UTF-8 with zero OCR artifacts.
- **TITUS / Van Nooten & Holland**: Metrically restored Rigveda text and accent matrices.

---

## 3. The Scanned Book: *Ṛgveda-Prātīśākhya* (`2015.409802`)

### 3.1 Edition Identification
- **File**: `rigveda/2015.409802.Rgveda-Pratisakhya.pdf` (976 pages).
- **Text**: **`ऋग्वेद-प्रातिशाख्यम्` (Ṛgveda-Prātīśākhya) of Śaunaka**.
- **Commentary**: Includes **Uvvaṭa’s classic *Bhāṣya*** with Hindi exposition by **Dr. Virendra Kumar Verma** (Chaukhamba Sanskrit Pratishthan / Chaukhamba Surbharati Prakashan, Varanasi).
- **Key Chapters for Vyutils**:
  - **Paṭala 1**: *Saṁjñā-paṭala* (phonetic inventory, Sthāna, Prayatna, Aghoṣa/Ghoṣa).
  - **Paṭalas 2–5**: *Sandhi-paṭalas* (vowel, consonant, and visarga euphonic combinations).
  - **Paṭala 10**: **`क्रमहेतु` (Krama-hetu)** — The rationale and theory of Krama recitation.
  - **Paṭala 11**: **`क्रमपाठ-विधि` (Krama-pāṭha-vidhi)** — The canonical rules and algorithm for Krama generation!

### 3.2 Evaluation of OCR for Test Suites
- **Verdict**: OCR on 100-year-old scanned books with bleed-through, complex Sanskrit conjunct ligatures, and Vedic accent diacritics introduces substantial noise (misrecognized accents, lost halantas).
- **Recommended Workflow**:
  1. Use clean digital sources (**GRETIL** and the **`sa.wikisource.org` pipeline**) for automated CI test fixtures.
  2. Use the scanned Chaukhamba edition and Uvvaṭa's commentary via [rigveda/ocr_extract.py](file:///Users/anand/Projects/project-vyasa/vyutils/rigveda/ocr_extract.py) as an authoritative reference to resolve difficult grammatical disputes.
