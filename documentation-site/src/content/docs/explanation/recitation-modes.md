---
title: Vedic Recitation Modes
description: Educational explanation of Prakṛti and Vikṛti recitation pāṭhas, Krama permutations, Pragṛhya sandhi immunity, and Parigraha formatting.
---

For over three millennia, the Vedic textual corpus—comprising tens of thousands of verses across the Ṛgveda, Yajurveda, Sāmaveda, and Atharvaveda—was transmitted entirely by oral tradition without the loss of a single syllable, vowel quantity, or pitch accent (*svara*).

This preservation was achieved not merely by rote repetition, but through an error-detecting checksum system: the **Prakṛti** (natural) and **Vikṛti** (permutational) **Pāṭhas**.

---

## 1. The Architectural Hierarchy

Vedic recitations fall into two overarching categories:

```text
Vedic Recitations
├── Prakṛti Pāṭhas (Natural forms)
│   ├── Saṃhitā-pāṭha (Continuous, sandhi-merged text)
│   ├── Pada-pāṭha    (Analyzed, word-by-word text)
│   └── Krama-pāṭha   (Stepped pairwise permutations)
└── Vikṛti Pāṭhas (Eight modified permutations based on Krama)
    ├── Jaṭā-pāṭha
    ├── Mālā-pāṭha
    ├── Śikhā-pāṭha
    ├── Rekhā-pāṭha
    ├── Dhvaja-pāṭha
    ├── Daṇḍa-pāṭha
    ├── Ratha-pāṭha
    └── Ghana-pāṭha
```

---

## 2. The Three Prakṛti Pāṭhas

### Saṃhitā-pāṭha (1 2 3 4 ... n)
The continuous, poetic form as chanted in ritual. Here, words are joined by natural euphonic combination (*sandhi*). Accents interact across word boundaries:

> अ॒ग्निमी॑ळे पु॒रोहि॑तं य॒ज्ञस्य॑ दे॒वमृ॒त्विज॑म् । होता॑रं रत्न॒धात॑मम् ॥
> (*agnim īḷe purohitaṃ yajñasya devam ṛtvijam | hotāraṃ ratnadhātamam ||*)

### Pada-pāṭha (1 | 2 | 3 | 4 ... n)
Attributed in the Ṛgveda tradition to the ancient grammarian Śākalya, the **Pada-pāṭha** isolates every word (*pada*) in its uncombined state. It resolves all external sandhi, reveals compound components using hyphens or avagrahah (`-` or `ऽ`), and isolates enclitics:

> अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् । य॒ज्ञस्य॑ । दे॒वम् । ऋ॒त्विज॑म् । होता॑रम् । रत्न॒-धात॑मम् ॥

### Krama-pāṭha (1-2 | 2-3 | 3-4 ... n-iti)
The **Krama-pāṭha** ("step recitation") is the vital bridge between Prakṛti and Vikṛti forms. In Krama, padas are recited in overlapping adjacent pairs:

- Word 1 + Word 2
- Word 2 + Word 3
- Word 3 + Word 4
- ...
- Word (n-1) + Word n
- Word n + *iti* + Word n (Terminal Parigraha)

Every internal word is recited exactly twice: first as the second member of a pair, then as the first member of the subsequent pair. This overlapping chain makes it impossible to drop, insert, or transpose a single word without breaking the recitation chain.

```text
Step 1: (1-2)  अ॒ग्निमी॒ळे॒
Step 2: (2-3)  ई॒ळे॒ पु॒रो-हि॑तम्
Step 3: (3-4)  पु॒रो-हि॑तं य॒ज्ञस्य॑
Step 4: (4-5)  य॒ज्ञस्य॑ दे॒वम्
Step 5: (5-6)  दे॒वमृ॒त्विज॑म्
Step 6: (6-7)  ऋ॒त्विजं॑ होता॑रम्
Step 7: (7-8)  होता॑रं रत्न॒-धात॑मम्
Step 8: (8)    रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्
```

---

## 3. The Parigraha Clause (*iti* / Sthita-Upasthita)

In Krama-pāṭha, certain words cannot simply be paired and left behind. They require an explicit closure clause known as **Parigraha** (or *Sthita-Upasthita* in the Prātiśākhyas).

Parigraha uses the particle **`इति॑`** (*iti*, accented with a canonical *svarita* on the second syllable):

### A. Terminal Words of a Verse
The final word (pada *n*) of a verse has no subsequent word to pair with. It is recited with Parigraha:
- Simple word: `पदम् + इति॑ + पदम्`
- Merged with *m*: `पु॒रोहि॑तम्` → `पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम्`

### B. Compound Words (*Samāsa*)
When a compound word occurs at the end of a section or receives Parigraha, it is recited in **unified form**, followed by `इति॑`, followed by its **split analytical form**:

> **Unified** + **इति॑** + **Split**

Example from Ṛgveda 1.1.1 (pada 8: `रत्न॒-धात॑मम्`):
> रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्

### C. The Vedic Particle *u*
Pāṇini (1.1.13 *oteḥ*) records that the unaccented monosyllabic particle *u* undergoes nasalization and prolongation in the Pada/Krama tradition:
> ऊँ॒ इति॑ उ

---

## 4. Pragṛhya Vowels & Sandhi Immunity

Normally in Sanskrit, when two vowels meet across a word boundary, they merge (*sandhi*):
- *a* + *i* → *e*
- *i* + *a* → *ya*
- *ī* + *e* → *ye*

However, an elite class of vowels called **Pragṛhya** are strictly immune to sandhi (*pluta-pragṛhyā aci nityam*, Pāṇini 6.1.125). Even before another vowel, they remain separate and unaltered (*hiatus*).

Pāṇini defines Pragṛhya vowels in *Aṣṭādhyāyī* 1.1.11–1.1.19:

| Sūtra | Condition | Examples |
|:---|:---|:---|
| **1.1.11** (*īdūded-dvivacanam*) | Dual endings in long *-ī*, *-ū*, or *-e* | `हरी एतौ` (*harī etau*, dual "two Haris"), `विष्णू इमौ` (*viṣṇū imau*) |
| **1.1.12** (*adaso māt*) | Forms of pronoun *adas* after *m* | `अमी ईशाः` (*amī īśāḥ*) |
| **1.1.13** (*śe*) | Vedic locatives/nominatives ending in *-e* | `अस्मे` (*asme*), `त्वे` (*tve*) |
| **1.1.14** (*nipāta ekājanāṅ*) | Monosyllabic particles (except *ā*) | `इ इन्द्रम्`, `उ उमेशम्` |
| **1.1.15** (*ot*) | Particles ending in *-o* | `अथो`, `उतो` |

Whenever a Pragṛhya pada appears in Krama-pāṭha, it is followed by a Parigraha `इति॑` clause to confirm that the hiatus was deliberate:

> हरी । एतौ । विहरतः ॥
> 
> **Krama:**
> 1. `हरी एतौ` (sandhi blocked!)
> 2. `हरी इति॑ हरी` (Parigraha confirming Pragṛhya status)
> 3. `एतौ विहरतः`
> 4. `विहरत इति॑ विहरतः`

---

## 5. Forward Vedic Sandhi in Recitation

When adjacent padas are paired in Krama (1-2, 2-3 ...), forward sandhi rules apply between them:

1. **Terminal *m* (*म्*) Assimilation**:
   - Before a vowel: merges into the syllable (*m* + vowel → *m*-vowel).
     - Example: `अ॒ग्निम्` + `ई॒ळे॒` → `अ॒ग्निमी॒ळे॒`
   - Before a consonant: converts to Anusvāra (`ं`).
     - Example: `ऋ॒त्विज॑म्` + `होता॑रम्` → `ऋ॒त्विजं॑ होता॑रम्`
   - **Unicode Mark Ordering**: In Unicode Devanagari encoding, the Anusvāra `\u0902` must strictly precede pitch accents (`\u0951` Svarita, `\u0952` Anudatta). `vyasa-patha` automatically maintains correct Unicode combining sequences.

2. **Visarga Transformations**:
   - *aḥ* before voiced consonants: shifts to *o* (`-ो`).
   - *aḥ* before *a-*: shifts to *o* with avagraha (`-ोऽ`).
   - *āḥ* before voiced sounds: visarga drops (`-ा`).

---

## 6. The Eight Vikṛti Pāṭhas

With Krama pairs established, the eight Vikṛti recitations permute the sequence through forward, reverse, and interwoven patterns:

1. **Jaṭā-pāṭha** ("Matted hair"):
   - Pattern: 1-2, 2-1, 1-2 | 2-3, 3-2, 2-3 ...
   - Chants every pair forward, backward, and forward again.
2. **Mālā-pāṭha** ("Garland"):
   - Woven chain linking odd and even padas across the verse.
3. **Śikhā-pāṭha** ("Tuft"):
   - Pattern: 1-2, 2-1, 1-2-3 | 2-3, 3-2, 2-3-4 ...
4. **Rekhā-pāṭha** ("Streak"):
   - Triangular stepping across triples and triplets.
5. **Dhvaja-pāṭha** ("Banner"):
   - Links the first padas directly with the terminal padas.
6. **Daṇḍa-pāṭha** ("Staff"):
   - Progressive linear chaining with reverse echo.
7. **Ratha-pāṭha** ("Chariot"):
   - Cross-hemistich wheels pairing quarter-verses.
8. **Ghana-pāṭha** ("Bell" or "Dense"):
   - The most intricate and revered Vedic recitation mode:
   - Pattern:
     > 1-2, 2-1, 1-2-3, 3-2-1, 1-2-3 | 2-3, 3-2, 2-3-4, 4-3-2, 2-3-4 ...

Because `vyasa-patha` produces structured `KramaStep` tokens with parsed padas and exact Sandhi boundaries, higher-order Vikṛti generators (like `vyasa-ghana`) can be layered directly on top of `vyasa-patha`.

---

## 7. Multi-Script Recitation with `vyasa-lipi`

A critical feature of `vyasa-patha` is script-agnostic generation. Using `vyasa-lipi`, recitations can be generated directly in the native script of any traditional Vedic lineage:

```bash
# Rigveda 1.1.1 in Telugu script (Krishna Yajurveda & South Indian Rigveda tradition)
vyasa-patha "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।" --to telugu
```

Output:
```text
అ॒గ్నిమీ॒ళే॒ । ఈ॒ళే॒ పు॒రో-హి॑తమ్ । పు॒రోహి॑తమితి॑ పు॒రో-హి॑తమ్ ॥
```

Or in Kannada script:
```text
ಅ॒ಗ್ನಿಮೀ॒ಳೇ॒ । ಈ॒ಳೇ॒ ಪು॒ರೋ-ಹಿ॑ತಮ್ । ಪು॒ರೋಹಿ॑ತಮಿತಿ॑ ಪು॒ರೋ-ಹಿ॑ತಮ್ ॥
```

Or in Academic Roman (IAST):
```text
a̱gnimī̱ḷe̱ | ī̱ḷe̱ pu̱ro-hítam | pu̱rohítamití pu̱ro-hítam ||
```
