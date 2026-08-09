---
title: Overview
description: Motivation, approach, and invariants for the vyutils project.
---

## Motivation

Common primitives are required to process structured text in a versioned filesystem and package it into efficient formats for use as an in-memory datastore with a query language to analyze data.

vyutils is a **general-purpose** pipeline for custom read-only applications (for example, mid-market CRM-style tools): infrequent writes in a versioned source tree, compile to an immutable SQLite package, deliver via static hosting.

Project Vyasa publication view is one possible consumer, not the primary driver.

## Approach

Model after GNU coreutils: composable tools, optional single binary, WASM for browser hosts.

- **Rust** — library crates with `[[bin]]` in-crate for now; separate `*-cli` crates deferred.
- **Svelte** — web UI when needed.
- **Astro Starlight** — this documentation site (`documentation-site/`).
- **Dogfooding** — feature inventory cross-linked to tests and docs (`inventory/` when scaffolded).

Work sequencing lives in the repository root [WORK.md](https://github.com/project-vyasa/vyutils/blob/main/WORK.md).

## Crates

| Crate | Role |
|-------|------|
| `mpt` | Multi-part-text format: parse, validate, manipulate |
| `walk` | Folder hierarchy: config inheritance, ignores, path inference |
| `pack` | Compile `.mpt` records + schema → immutable SQLite for delivery |
| `render` | Narrative documents from structured data (later) |

## Invariant

**The compiled database is immutable at runtime.** All authoritative edits occur in the source tree (`.mpt` files + config). Recompilation produces a new artifact version.

## Documentation map

This site follows [Diátaxis](https://diataxis.fr/):

| Quadrant | Section | Purpose |
|----------|---------|---------|
| Explanation | [Explanation](/vyutils/explanation/overview/) | Context and background (you are here) |
| How-to | [Guides](/vyutils/guides/edit-mpt-parts/) | Goal-oriented recipes |
| Reference | [Reference](/vyutils/reference/mpt/) | CLI and format facts |
| Specification | [RFCs](/vyutils/rfcs/rfc-0001-mpt-format/) | Normative format definitions |
