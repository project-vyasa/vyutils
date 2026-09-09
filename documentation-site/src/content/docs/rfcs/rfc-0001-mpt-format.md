---
title: RFC-0001 — mpt format
description: Draft specification for the multi-part-text (mpt) plain-text format.
---

| Field      | Value                          |
|------------|--------------------------------|
| Status     | **Draft** — open for iteration |
| Authors    | vyutils                        |
| Supersedes | — (fresh design; not mpx-compatible) |

## Summary

**mpt** (multi-part-text) is a plain-text **container** format. The **core parser** recognizes envelope lines, splits regions, and records optional **`format = <name>`** plus an optional **format option-map** on part open envelopes. **Format parsers** interpret payload bytes.

A **file-header block** (open envelope with label `header` before the first part) holds document metadata. Payloads use **TOML** by default (including `#` line comments).

```text
``[ header
classification = pii
``] header

``[ synopsis format = yaml
``[ header
title = "Annotation patterns"
``] header
summary: |
  A brief overview.
``] synopsis

``[ source format = vyasa
``[ header
stream = "mula"
``] header
  `chapter 1 [ … ]
``] source
```

## Design principle: one syntax per feature

> **There is exactly one canonical way to express a given structure.** Alternate spellings or “equivalent” forms are not permitted unless an exception is documented below in **bold** with explicit rationale.

## Terminology: SIGIL and envelopes

**SIGIL** — two grave-accent characters (U+0060, the `` ` `` key) at **column 0**. Written literally only inside fenced `text` examples below; in prose we say **SIGIL** to avoid Markdown parsing issues.

**Open envelope** — SIGIL, then `[`, then a space, then a **label**, then an optional `format = <name>` clause. Part opens may then take an optional **option-map** (TOML inline table) of format-scoped scalars.

**Close envelope** — SIGIL, then `]`, then a space, then the same **label** as the matching open. Close lines have no format clause and no option-map.

| Line | Form (prose) |
|------|----------------|
| Open | SIGIL + `[` + label + optional `format = <name>` + optional `{ … }` (parts only) |
| Close | SIGIL + `]` + label |

**Label** is either:

- `header` — metadata block (file-level before first part, or part-level before body)
- `<part-id>` — body region (URL-safe id; `header` is reserved)

**Close rule:** the close label must echo the open label (`synopsis` open → `synopsis` close; `header` open → `header` close).

## Part identifiers

Each part is named with a **part id** — a URL-safe handle chosen by the author or tool:

```text
PartId ::= [a-z0-9_] ( [a-z0-9_-]* [a-z0-9_] )?
```

`header` is **reserved** and cannot be used as a part id.

Examples: `synopsis`, `source`, `metadata`, `build-log`, `states_psv`.

### What part ids are for

Part ids are **grammar-independent labels**. They are not envelope keywords and can be renamed freely (e.g. `synopsis` → `abstract`) without changing the mpt grammar — only the author's naming and any **consumer references** to that id.

| Concern | Tied to part id? |
|---------|------------------|
| Envelope syntax (`[`, `]`, `header`, …) | **No** — fixed grammar |
| Parser state machine | **No** — only structure matters |
| Human navigation / grep | **Yes** — open envelope with part id is easy to find |
| `pack` / app logic (“load part `metadata`”) | **Yes** — if schema or code names parts |
| URLs / APIs (`/records/foo/parts/metadata`) | **Yes** — if exposed externally |

**Swapping ids** (rename in source files) is safe when:

- No schema, script, or documentation hard-codes the old name.
- You update both the open and close envelope for that id.

**Swapping ids** requires coordination when:

- A `pack` schema expects a part named `metadata`.
- Tests assert on part id.
- Cross-links in docs say “see part `source`”.

The grammar treats ids as opaque strings; **semantics are a consumer contract**, like HTML `id` attributes or zip entry names.

## Envelope sigil

SIGIL is **two grave-accent characters at column 0**, immediately followed by `[` (open) or `]` (close).

Reserve in Vyasa where sources embed in mpt: the `header` label, and any column-0 line beginning with SIGIL.

## Document shape

```text
[ FileHeader? ]
Part+
```

```text
FileHeader   ::= HeaderBlock
Part         ::= PartOpen PartInterior PartClose
PartOpen     ::= SIGIL "[" SPACE PartId FormatClause? OptionMap?
PartClose    ::= SIGIL "]" SPACE PartId
HeaderBlock  ::= HeaderOpen HeaderPayload HeaderClose
HeaderOpen   ::= SIGIL "[" SPACE "header" FormatClause?
HeaderClose  ::= SIGIL "]" SPACE "header"
FormatClause ::= SPACE "format" SPACE? "=" SPACE? FormatName
FormatName   ::= [a-z][a-z0-9]*
OptionMap    ::= SPACE TomlInlineTable
# TomlInlineTable — TOML 1.0 inline table, same line as the open envelope
```

### File header

- Open envelope with label `header` before the first part → **file header**.
- At most one pre-part file header in v1.
- Default header payload format: **toml** (supports `#` line comments; no block comments).

**Document-level metadata** (provenance, license, catalog ids, attribution) belongs in the **file header**, not in a separate part — analogous to source banners in plain-text corpora.

**Tool-maintained fields:** `parts = <n>` is written by `mpt` on canonical serialize and after `part add` / `part remove`. Authors should not hand-edit it. Other commands (`validate`, `merge`, `part extract`, …) emit a **warning** if declared `parts` does not match the actual part count.

### `format = <name>` and option-map on envelope lines

**`format = <name>`** names the payload parser family. Omit when default (`toml` headers, `text` part bodies).

**Option-map** — optional TOML **inline table** on the **part open** line, after the format clause. It holds **format-scoped scalar knobs** (parser dispatch), not descriptive metadata.

```text
``[ states format = csv { delimiter = "|" }
```

Grammar and quoting are **TOML 1.0 inline tables** ([toml.io](https://toml.io/en/v1.0.0#inline-table)):

- Single line only (matches this parser’s line-oriented envelopes; TOML 1.0 also forbids newlines inside `{ … }`).
- Comma-separated `key = value` pairs; **no trailing comma**.
- Empty `{ }` is invalid.
- Keys are bare TOML keys: `[A-Za-z0-9_-]+`.
- v1 values are **strings** (TOML basic `"…"` or literal `'…'`). Tab delimiter uses a basic string: `delimiter = "\t"`.
- Unknown key for the **effective** format (explicit `format` or default `text`) → error.
- Option-map is illegal on **header** opens and on **all close** envelopes.
- `format` itself stays a clause, not a key inside `{ … }`.

| Kind of data | Where it lives |
|--------------|----------------|
| Parser family | `format = <name>` on the open envelope |
| Scalar parser knobs (`delimiter`, …) | Option-map on the **part** open line |
| Multi-line descriptive metadata (title, stream, chapter, checksums) | Header **payload** (file- or part-level) |

**Rationale:** Bare `k = v` pairs after `format` have no enclosing context. A TOML inline table supplies that context and reuses quoting/escaping already used in header payloads. Nested part-header blocks remain for real header *content*; they are not required for a single delimiter.

Canonical serialize: omit the map when every value is the format default; emit keys in whitelist order; use TOML basic strings (`"…"`) so escapes such as `\t` stay visible.

### Format registry (v1)

Built-in names only (`toml`, `text`, `yaml`, `json5`, `xml`, `vyasa`, `csv`, …). Unknown → error.

### Tabular data (`csv` format)

Delimiter-separated **rows of fields** (the DSV family) use the single registry name **`csv`** — not separate entries per delimiter (`tsv`, `psv`, `dsv`, etc.).

| Concern | Where it lives |
|---------|----------------|
| Parser family | `format = csv` on the part open envelope |
| Field separator | Option-map: `delimiter` (default: `","`) |
| Part id | Semantic name (`states`, `inventory`) — **not** the delimiter |
| Row/schema notes | Part-header payload if needed (`pack`) |

**v1 `csv` option-map keys**

| Key | Type | Default | Notes |
|-----|------|---------|-------|
| `delimiter` | string | `","` | Field separator; omit when comma. One character in v1. |

`encoding` is **not** a v1 key. UTF-8 is the implicit default for all payload bytes; a future key can be added to this table without changing the map grammar.

**Rationale:** CSV is the de facto name for this shape of data even when the separator is not a comma (common with `;` in European locales). Pipe- or tab-separated tables are the same *kind* of payload; only the delimiter differs. Extensions like `.psv` are uncommon — delimiter belongs in the option-map, not in part ids, the format registry, or a nested header block.

When a `csv` format parser is implemented (in `pack` or a dedicated module), it should:

- Read `delimiter` from the part open option-map (default `,`).
- Apply RFC 4180-style quoting rules where applicable.
- Treat the first row as a header row when the schema or part header says so (future `pack` concern).

Until then, `mpt` stores tabular bodies as opaque bytes; `format = csv` and the option-map declare intent for downstream tools.

### Payload newlines

Header and part bodies are **line-oriented**: each source line between open and close envelopes is stored with its terminating LF. The close envelope line is not part of the payload.

- A single-line header payload `title = "x"` is stored as `title = "x"\n`.
- A body whose last row is `CO|Colorado|…` is stored with a trailing `\n` after that row — no blank line before the part close is required.
- An **empty** payload is open followed immediately by close on the next envelope line (no content lines in between). A blank line between open and close is stored as `\n` (one empty line of content), not as “empty header.”

`mpt part extract` emits the stored payload bytes as-is. Canonical serialize may still add a final LF before a close envelope when a programmatically built payload omits one.

### Empty header payload

Valid (machine-generated shells):

```text
``[ header
``] header
```

### Canonical serialization

- LF on write; canonical round-trip.
- Omit default `format = …` and default option-map keys.
- Insert or update `parts = <n>` in the file header when serializing.

## Examples

### Multi-format document

```text
``[ header
classification = research
``] header

``[ abstract format = yaml
``[ header
title = "Inline annotations"
``] header
summary: >
  Compares inline and out-of-band metadata patterns.
``] abstract

``[ source format = vyasa
``[ header
stream = "mula"
chapter = 1
``] header
  `chapter 1 [
    `verse 1 [ dhṛtarāṣṭra uvāca ]
  ]
``] source

``[ references format = toml
[[cite]]
key = "rfc-019"
title = "Extended semantic annotations"
``] references
```

### Feature inventory

```text
``[ header
classification = internal
``] header

``[ metadata
``[ header
name = "Text node rendering"
roles = ["publisher"]
tests = ["vyasac-simple-tests"]
``] header
``] metadata

``[ description
Granular feature: Text node rendering
``] description
```

### Tabular data (pipe-delimited)

Provenance in the file header; `csv` plus option-map `delimiter` on the part open line.

```text
``[ header
# Provenance
[source]
institution = "Example corpus"
license = "CC-BY-NC-4.0"
``] header

``[ states format = csv { delimiter = "|" }
code|name|sentence
CA|California|Golden State
TX|Texas|Lone Star State
``] states
```

### Machine-generated

```text
``[ header
``] header

``[ build-log
2026-08-08T12:00:00Z pack complete
``] build-log
```

## Validation rules (v1)

1. At most one pre-part file-header block.
2. Every part open envelope has a matching close with the same part id.
3. Part ids unique, URL-safe, not `header`; order preserved.
4. File- and part-level `header` open/close pairs (empty payload allowed).
5. Envelope: `format = <registered-name>` plus, on part opens only, an optional TOML inline-table option-map whose keys are whitelisted for that format.
6. Unknown format name or unknown option-map key → error.

## Resolved decisions

| # | Topic | Decision |
|---|-------|----------|
| 1 | Envelope shape | SIGIL + bracket open/close; label echoed on close |
| 2 | Close echoes identity | `header` label on close; part id on close |
| 3 | `header` keyword | Retained as label (grepability); reserved from part ids |
| 4 | Format on envelope | `format = <name>`; optional TOML inline-table option-map on **part** opens |
| 5 | Format registry | Built-in v1 |
| 6 | Sigil | Double grave-accent at column 0, then `[` or `]` |
| 7 | Checksums | Optional in header **payload**; compile-time integrity in `pack` |
| 8 | Provenance | File header (not a separate part) |
| 9 | `parts` count | Tool-maintained in file header; warn on mismatch |
| 10 | Tabular payloads | One registry name (`csv`); `delimiter` in the part option-map |

## Name review (2026-08-08)

Independent review of the keyword set.

| Item | Verdict |
|------|---------|
| `header` label | Keep — readable, grep-friendly |
| Bracket syntax | Adopted — uniform open/close; drops `*-end` suffix |
| `part-end <id>` | Superseded by close envelope with part id |
| `format = name` | Keep — parser family on the envelope |
| Option-map `{ … }` | Adopted — TOML 1.0 inline table; format-scoped scalars; not a free-form header |
| `file`, `part-open`, etc. | Rejected — vocabulary bloat |

## References

- [Vyasa grammar](https://project-vyasa.github.io/vyasa-docs/reference/grammar/)
- RFC 2046 (MIME)

## Changelog

| Date       | Change |
|------------|--------|
| 2026-08-08 | `end` → `part-end`; part id semantics; name review |
| 2026-08-09 | Bracket envelope syntax; `header` retained as label |
| 2026-08-09 | File-header provenance; tool-maintained `parts`; SIGIL docs use prose + fenced examples |
| 2026-08-09 | Tabular data: `csv` format + `delimiter` in part header; no per-delimiter registry names |
| 2026-08-09 | Payload newlines: store each content line’s LF; empty = open then close with no lines between |
| 2026-08-14 | Part open option-map: TOML 1.0 inline table; `csv` `delimiter` moves off the part header |
