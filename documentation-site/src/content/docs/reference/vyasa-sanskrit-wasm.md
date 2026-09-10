---
title: vyasa-sanskrit-wasm API Reference
description: WebAssembly bindings and TypeScript API reference for @project-vyasa/sanskrit-wasm.
---

`vyasa-sanskrit-wasm` is the WebAssembly facade crate in `vyutils`. It bundles `vyasa-phonetics`, `vyasa-lipi`, and `vyasa-patha` into a single, high-performance browser module distributed as the scoped npm package `@project-vyasa/sanskrit-wasm`.

---

## Package Overview

- **NPM Package**: `@project-vyasa/sanskrit-wasm`
- **WASM Binary**: `vyasa_sanskrit_wasm_bg.wasm` (**133 KB**)
- **TypeScript Definitions**: Fully typed declarations in `pkg/vyasa_sanskrit_wasm.d.ts`
- **Target Environments**: Modern Browsers (ESM), SvelteKit, Vite, Next.js, Electron, Node.js

---

## TypeScript Interfaces (DTOs)

### `ScriptInfoDto`
```typescript
export interface ScriptInfoDto {
  id: string;
  name: string;
  is_indic: boolean;
  has_vedic_pitch: boolean;
}
```

### `KramaStepDto`
```typescript
export interface KramaStepDto {
  step_number: number;
  formula: string;
  first_index: number;
  second_index?: number;
  raw_pada: string;
  sandhied: string;
  is_parigraha: boolean;
  pragrhya_detected: boolean;
}
```

### `PadaInfoDto`
```typescript
export interface PadaInfoDto {
  raw: string;
  clean: string;
  compound_parts: string[];
  is_compound: boolean;
  is_pragrhya: boolean;
  pragrhya_type?: string;
}
```

### `ShivaSutraDto`
```typescript
export interface ShivaSutraDto {
  index: number;
  name: string;
  it_marker_deva: string;
  it_marker_iast: string;
  sounds_deva: string[];
  sounds_iast: string[];
}
```

### `VarnaAnalysisDto`
```typescript
export interface VarnaAnalysisDto {
  glyph_deva: string;
  glyph_iast: string;
  varna_type: 'vowel' | 'consonant' | 'ayogavaha';
  sthana: string[];
  abhyantara_prayatna: string;
  is_ghosha: boolean;
  is_alpaprana: boolean;
  matra: number;
}
```

### `AksharaAnalysisDto`
```typescript
export interface AksharaAnalysisDto {
  surface: string;
  consonants: VarnaAnalysisDto[];
  vowel?: VarnaAnalysisDto;
  ayogavaha?: string;
  svara?: string;
  total_matra: number;
}
```

---

## Exported Functions

### 1. Transliteration & Scripts
- `transliterate(text: string, from_script: string, to_script: string): string`
- `detect_script(text: string): string | undefined`
- `get_supported_scripts(): ScriptInfoDto[]`

### 2. Vedic Recitation & Krama
- `generate_krama(input_pada_text: string, target_script: string): KramaStepDto[]`
- `generate_krama_text(input_pada_text: string, target_script: string): string`
- `parse_padas(input_pada_text: string): PadaInfoDto[]`

### 3. Phonetics & Śiva Sūtras
- `get_shiva_sutras(): ShivaSutraDto[]`
- `get_pratyahara_sounds(pratyahara_name: string): VarnaAnalysisDto[]`
- `check_pratyahara_contains(pratyahara_name: string, sound_symbol: string): boolean`
- `inspect_varna(varna_symbol: string): VarnaAnalysisDto`
- `analyze_syllables(text: string, script_name: string): AksharaAnalysisDto[]`

---

## Detailed Code Examples

For comprehensive usage examples of every function, see the **[Sanskrit Developer Guide (Rust & WASM)](file:///Users/anand/Projects/project-vyasa/vyutils/documentation-site/src/content/docs/guides/sanskrit-developer-guide.md)**.
