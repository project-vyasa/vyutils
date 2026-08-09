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

**mpt** (multi-part-text) is a plain-text **container** format. The **core parser** recognizes envelope lines, splits regions, and records optional **`format = <name>`** on envelope lines. **Format parsers** interpret payload bytes.

A ``[ header`` block **before the first part** is the **file header**. Document metadata lives in header **payloads** (toml by default).

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

## Envelope syntax (v1)

Every region uses the same bracket pattern:

| Line | Form |
|------|------|
| Open | ``[ `<label>` [ `format = <name>` ] |
| Close | ``] `<label>` |

**Label** is either:

- `header` — metadata block (file-level before first part, or part-level before body)
- `<part-id>` — body region (URL-safe id; `header` is reserved)

**Close rule:** close echoes the open label (```] synopsis`` matches ``[ synopsis``). Same for ``] header``.

## Part identifiers

Each part is named with a **part id** — a URL-safe handle chosen by the author or tool:

```ebnf
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
| Human navigation / grep | **Yes** — ``[ source`` easy to find |
| `pack` / app logic (“load part `metadata`”) | **Yes** — if schema or code names parts |
| URLs / APIs (`/records/foo/parts/metadata`) | **Yes** — if exposed externally |

**Swapping ids** (rename in source files) is safe when:

- No schema, script, or documentation hard-codes the old name.
- You update both ``[ <id>`` and ``] <id>`` together.

**Swapping ids** requires coordination when:

- A `pack` schema expects a part named `metadata`.
- Tests assert on part id.
- Cross-links in docs say “see part `source`”.

The grammar treats ids as opaque strings; **semantics are a consumer contract**, like HTML `id` attributes or zip entry names.

## Envelope sigil

**Double-backtick** (`` `` ``) at column 0, followed immediately by `[` or `]`.

Reserve in Vyasa where sources embed in mpt: `header`, and bracket lines at column 0.

## Document shape

```text
[ FileHeader? ]
Part+
```

```ebnf
FileHeader   ::= HeaderBlock
Part         ::= PartOpen PartInterior PartClose
PartOpen     ::= SIGIL "[" SPACE PartId FormatClause?
PartClose    ::= SIGIL "]" SPACE PartId
HeaderBlock  ::= HeaderOpen HeaderPayload HeaderClose
HeaderOpen   ::= SIGIL "[" SPACE "header" FormatClause?
HeaderClose  ::= SIGIL "]" SPACE "header"
FormatClause ::= SPACE "format" SPACE? "=" SPACE? FormatName
FormatName   ::= [a-z][a-z0-9]*
```

### File header

- ``[ header`` before the first part → file header.
- At most one pre-part ``[ header`` in v1.
- Default header payload: **toml**.

### `format = <name>` on envelope lines

Only **`format = <name>`** on envelope lines — **not a general map**. Other fields (including optional checksums) live in header **payloads**.

Omit when default (`toml` headers, `text` part bodies).

### Format registry (v1)

Built-in names only (`toml`, `text`, `yaml`, `json5`, `xml`, `vyasa`, `csv`, …). Unknown → error.

### Empty header payload

Valid (machine-generated shells):

```text
``[ header
``] header
```

### Canonical serialization

- LF on write; canonical round-trip.
- Omit default `format = …`.

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

### Machine-generated

```text
``[ header
``] header

``[ build-log
2026-08-08T12:00:00Z pack complete
``] build-log
```

## Validation rules (v1)

1. At most one pre-part ``[ header`` block.
2. Every ``[ <part-id>`` has matching ``] <part-id>``.
3. Part ids unique, URL-safe, not `header`; order preserved.
4. ``[ header`` / ``] header`` paired (empty payload allowed).
5. Envelope: only `format = <registered-name>` besides labels.
6. Unknown format name → error.

## Resolved decisions

| # | Topic | Decision |
|---|-------|----------|
| 1 | Envelope shape | Bracket open/close: ``[ label`` / ``] label`` |
| 2 | Close echoes identity | ``] header`` for metadata; ``] <part-id>`` for parts |
| 3 | `header` keyword | Retained as label (grepability); reserved from part ids |
| 4 | Format on envelope | `format = <name>` only |
| 5 | Format registry | Built-in v1 |
| 6 | Sigil | Double-backtick + `[` / `]` |
| 7 | Checksums | Optional in header **payload**; compile-time integrity in `pack` |

## Name review (2026-08-08)

Independent review of the keyword set.

| Item | Verdict |
|------|---------|
| `header` label | Keep — readable, grep-friendly |
| Bracket syntax | Adopted — uniform open/close; drops `*-end` suffix |
| `part-end <id>` | Superseded by ``] <id>`` |
| `format = name` | Keep — readable, not a general map |
| `file`, `part-open`, etc. | Rejected — vocabulary bloat |

## References

- [Vyasa grammar](https://project-vyasa.github.io/vyasa-docs/reference/grammar/)
- RFC 2046 (MIME)

## Changelog

| Date       | Change |
|------------|--------|
| 2026-08-08 | `end` → `part-end`; part id semantics; name review |
| 2026-08-09 | Bracket envelope syntax (``[ label`` / ``] label``); `header` retained as label |
