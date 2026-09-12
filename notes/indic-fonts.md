# Indic fonts for Lipi / Pāṭha / docs

The engines emit Unicode. Missing boxes (tofu) are almost always a **font load or fallback** problem, not a bad transliteration. We hit this twice: Starlight docs in this repo, then Sanskrit Studio in `vyasa-apps`.

**Public CSS snippet:** [Sanskrit developer guide §6](../documentation-site/src/content/docs/guides/sanskrit-developer-guide.md). **Studio:** `apps/sanskrit-studio/src/app.html` + `packages/sanskrit/src/sanskrit.css`.

## Scripts the engine actually emits

| Script id | Block (typical) | Google Fonts family to **fetch** |
|-----------|-----------------|----------------------------------|
| `devanagari` | U+0900–097F + Vedic marks | Noto Sans Devanagari |
| `telugu` | U+0C00–0C7F | Noto Sans Telugu |
| `kannada` | U+0C80–0CFF | Noto Sans Kannada |
| `grantha` | U+11300–1137F (SMP) | Noto Serif Grantha |
| `malayalam` | U+0D00–0D7F | Noto Sans Malayalam |
| `bengali` | U+0980–09FF | Noto Sans Bengali |
| `iast` / `iso15919` | Latin + combining | Noto Sans **and** a mono that contains U+0331 in one file (Menlo / Courier New). Do not rely on sliced Noto Sans Mono. |

Naming a family in `font-family` does nothing unless that file is installed or loaded as a webfont. macOS may still paint Telugu/Kannada via `* MN` system fonts; Malayalam and Bengali often will not, especially in Chromium / Cursor Simple Browser.

## Failure modes we have actually seen

1. **CSS names a Noto family that `app.html` / `@import` never fetches.** Studio listed Malayalam and Kannada in `.font-sanskrit` but only loaded Devanagari, Telugu, and Grantha. Malayalam/Bengali → tofu; Telugu/Grantha looked fine. Fix: every Indic family in the stack must appear in the Google Fonts (or self-hosted) stylesheet.
2. **Docs site same class of bug.** `documentation-site/src/styles/custom.css` still `@import`s only Devanagari, Kannada, Telugu (plus Noto Serif Telugu). Grantha, Malayalam, and Bengali examples in the guides will box on machines without those system fonts.
3. **Google Fonts unicode-range slicing.** WOFF2 subsets often drop Vedic combining marks. Classic: Noto Sans Mono Latin slice omits U+0331 (combining macron below), so IAST `a॒` / `ḷe॒` gets a box under the letter even though the base glyph exists. Prefer Menlo/Monaco/Courier for monospace pitch, or a **full** (unsliced) font file.
4. **First font in the stack has `.notdef` instead of falling through.** `Inter` / `system-ui` on the DataGrid (no `.font-sanskrit`) can show tofu for scripts Inter does not cover. Put the Indic Noto families on `--font-sans` as well, or put `.font-sanskrit` on every node that shows engine output (tables included).
5. **Cmap holes vs tofu.** Bengali has no ळ; the engine still emits a codepoint at the Devanagari offset. That is a mapping question, not a missing webfont. Tofu for an *entire* string (every akshara boxed) is fonts; one odd box in a sea of real glyphs may be an unassigned codepoint.

## What to load (web)

Studio’s current Google Fonts URL (keep in sync if you add a script):

```
Noto Sans (Latin)
Noto Sans Bengali
Noto Sans Devanagari
Noto Sans Kannada
Noto Sans Malayalam
Noto Sans Telugu
Noto Serif Grantha
```

System fallbacks worth listing after the Noto names: `Malayalam MN`, `Malayalam Sangam MN`, `Bangla MN`, `Kohinoor Bangla`, `Kannada MN`, `Telugu MN`.

UI chrome (`Inter`) can stay first **if** the Noto families are actually loaded; Google’s `@font-face` `unicode-range` then picks the right file per character.

## Quick checks (no new tool)

Browser, after load:

```js
await document.fonts.load('24px "Noto Sans Malayalam"', 'അ');
document.fonts.check('24px "Noto Sans Malayalam"', 'അ'); // want true
```

Local TTF/OTF cmap (codepoints only, not conjunct shaping):

```bash
# fonttools
python3 -c "from fontTools.ttLib import TTFont; print('0D05' in {hex(c)[2:].upper() for c in TTFont('NotoSansMalayalam-Regular.ttf').getBestCmap()})"

# HarfBuzz: missing glyphs show as .notdef in the trace
hb-shape NotoSansMalayalam-Regular.ttf 'അഗ്നി'
```

Golden sample: `अ॒ग्निमी॑ळे पु॒रोहि॑तम्` (and the pada-pāṭha form with `।`) through `--to malayalam` / `--to bengali`. If the CLI prints real Unicode and the web UI boxes, the font stack is wrong.

## Should vyutils ship a font-coverage CLI?

**Useful as a small diagnostic, not as part of the linguistic engine.** Lipi/Pāṭha should stay Unicode-in / Unicode-out. Coverage is a property of a **font file** (or of the WOFF2 slice the browser actually downloaded).

Worth doing later if we keep hitting tofu in docs and apps:

- Input: a string (or `--to <script>` plus the golden pada) and a font path.
- Output: codepoints with no cmap entry; call out Vedic combiners (U+0951, U+0952, U+0331, U+1CD0–1CF9) separately.
- Exit non-zero for CI (Starlight visual pages, Studio).
- **Do not** put this in WASM. Optional `ttf-parser` on a `vyasa-lipi check-font` subcommand (or a tiny `vyasa-font-check` bin) keeps `vyasa-lipi`’s core crate lean.

What it will **not** catch, so do not oversell it:

| Layer | Example | Needs |
|-------|---------|--------|
| cmap | Font file lacks U+0D05 | CLI vs TTF |
| GSUB / conjuncts | Letters exist, ക്ഷ does not form | `hb-shape` |
| Google slice | Full TTF is fine; served WOFF2 `unicode-range` omits U+0331 | Inspect the CSS `@font-face` or the downloaded WOFF2 |
| Terminal | `vyasa-lipi --to malayalam` prints correctly, Terminal.app shows boxes | Terminal profile font, not the engine |

Until we write that, `document.fonts.check`, `hb-shape`, and “did we actually `<link>` this family?” are enough.

## Follow-ups

- [ ] Docs `custom.css`: load Malayalam, Bengali, Grantha the same way Studio does (or stop showing those scripts in the guides).
- [ ] Optional `check-font` CLI (see above).
- [ ] Self-host Noto WOFF2 if we want GH Pages to render Indic offline / without fonts.googleapis.com.
