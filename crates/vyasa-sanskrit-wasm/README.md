# vyasa-sanskrit-wasm

> WebAssembly bindings for Project Vyasa's Sanskrit computing engines.

Bundles **`vyasa-phonetics`**, **`vyasa-lipi`**, and **`vyasa-patha`** into a lightweight (133 KB), zero-backend client-side WASM package distributed as **`@project-vyasa/sanskrit-wasm`**.

---

## Capabilities

- **Universal Transliteration**: Lossless conversion across 11 Indic and Roman scripts with complete Vedic pitch accent preservation.
- **Vedic Recitation**: Pada-pāṭha parser, forward Sandhi, Pragṛhya detection, and Krama recitation generator.
- **Articulatory Phonetics**: Ṛgveda-Prātiśākhya classifications, 14 Māheśvara Śiva Sūtras, and Pāṇinian Pratyāhāra queries.

---

## Build

```bash
# Build for modern browsers with wasm-pack
wasm-pack build --target web --scope project-vyasa
```

Output is generated into `./pkg/` with TypeScript declarations (`vyasa_sanskrit_wasm.d.ts`).

---

## Quick Example

```typescript
import init, { transliterate, generate_krama, inspect_varna } from '@project-vyasa/sanskrit-wasm';

await init();

// 1. Transliterate with Vedic accents
const telugu = transliterate("अ॒ग्निमी॑ळे पु॒रोहि॑तम्", "devanagari", "telugu");
// => "అ॒గ్నిమీ॑ళే పు॒रोహి॑తమ్"

// 2. Krama-pāṭha generation
const steps = generate_krama("अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।", "devanagari");

// 3. Phonetic inspection
const k = inspect_varna("k");
// => { glyph_deva: "क", sthana: ["Kantha (Velar)"], abhyantara_prayatna: "Sprshta", ... }
```

See the [Documentation Site](https://project-vyasa.github.io/vyutils/guides/sanskrit-developer-guide/) for complete function references.
