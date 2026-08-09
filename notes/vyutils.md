# vyutils Overview

## Motivation
Common primitives are required to process structured text in a versioned filesystem,
and package them into efficient formats for use as an in-memory datastore with efficient
query language to analyze data. 
Project Vyasa publication view takes a similar approach to build compelling search and 
visualization of structured corpora like scriptural text.

vyutils is a **general-purpose** pipeline for custom read-only applications (e.g. mid-market
CRM-style tools): infrequent writes in a versioned source tree, compile to an immutable SQLite
package, deliver via static hosting.

## Approach
Model after GNU coreutils: composable tools, optional single binary, WASM for browser hosts.

- **Rust** — library crates consumed by CLI (`[[bin]]` in-crate for now; separate `*-cli` crates deferred).
- **Svelte** — web UI when needed.
- **Astro Starlight** — project docs in `documentation-site/`.
- **Dogfooding** — feature inventory cross-linked to tests and docs (see `inventory/` when scaffolded).
- **Quality bar** — coreutils-level rigor in design, delivery, and user feedback.
- **CLI discoverability** — each tool exposes `tree` (command/option snapshot from clap). Implementation lives in `mpt` for now; promote to a shared `cli-tree` crate when a second binary needs it.

Work sequencing: [WORK.md](../WORK.md).

## Crates (names settled)

| Crate | Role |
|-------|------|
| `mpt` | Multi-part-text format: parse, validate, manipulate |
| `walk` | Folder hierarchy: config inheritance, ignores, path inference |
| `pack` | Compile `.mpt` records + schema → immutable SQLite for delivery |
| `render` | Narrative documents (Word, Google Docs, Markdown) from structured data |

## Format

Multi-part-text (`.mpt`) — ``[ header`` / ``[ <part-id>`` / ``] header`` / ``] <part-id>`` (double-backtick sigil). Pre-part ``[ header`` = file metadata. See [RFC-0001](documentation-site/src/content/docs/rfcs/rfc-0001-mpt-format.md).

Replaces informal `.mpx` naming in vyasa; existing inventory files are compatible starting points.

## Invariant

**The compiled database is immutable at runtime.** All authoritative edits occur in the source tree (`.mpt` files + config). Recompilation produces a new artifact version.
