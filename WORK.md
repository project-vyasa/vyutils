# vyutils — work queue

Actionable sequencing for this repo. Overview and motivation live in [notes/vyutils.md](notes/vyutils.md). Usability observations from using `.mpt` files: [notes/mpt-field-notes.md](notes/mpt-field-notes.md).

**Conventions (agreed):**

- Library crates first; **defer separate `*-cli` crates** — each tool ships as `[lib]` + `[[bin]]` in one crate until WASM or dependency isolation forces a split.
- Astro Starlight site lives in **`documentation-site/`** (not a Rust crate). Dev server: `bun run dev` → http://localhost:9800/vyutils/
- Crate names: **`mpt`**, **`walk`**, **`pack`**, **`render`** (replacing tentative `folder`, `mpt-db`, `report-doc-engine`).
- Compiled SQLite artifacts are **immutable at runtime**; authoritative edits happen in the source tree (`.mpt` files + config).
- **Shared CLI helpers deferred** — `mpt tree` is implemented in `crates/mpt/src/command_tree.rs` today. When a second tool needs it (`walk`, `pack`, …), extract to a small workspace crate (tentative name `cli-tree`; depends on `clap` only). Hold until we know what else belongs there (e.g. common output/in-place conventions, JSON tree export).

**Target repo layout (scaffold):**

```text
vyutils/
├── WORK.md
├── Cargo.toml
├── crates/
│   ├── mpt/
│   ├── walk/
│   ├── pack/
│   └── render/
├── documentation-site/      # Astro Starlight (RFCs, guides)
├── inventory/
└── notes/
```

---

## NOW

1. **`walk` crate** — directory traversal, per-folder config merge, ignore globs, path → metadata inference.
2. **`pack` crate** — schema TOML → validate records → SQLite; `pack compile <project>/` binary in the same crate.

---

## NEXT — opportunistically, after confirming

1. **`inventory/`** — dogfood feature inventory (`.mpt` records + schemas); proof point by recompiling `vyasa/inventory/` when `pack` is ready.
2. **CI** — `cargo fmt`, `clippy`, `test`; `documentation-site` build.
3. **GH Pages deploy** — `documentation-site` to `project-vyasa.github.io/vyutils`; add vyutils card on org landing page.

---

## LATER — for discussion

1. **Split `*-cli` crates** — when `wasm32` builds must not depend on `clap` or other binary-only deps.
2. **`render` crate** — narrative documents (Word, Google Docs, Markdown) from packed data; grouping, headers/footers, expression language for computed fields.
3. **WASM + npm distribution** — `@vyutils/*` packages for browser hosts.
4. **Vite/Bun integration** — dev server, watch mode, virtual modules built on `walk`.
5. **Turso / libSQL** — only if edge replication becomes a concrete requirement.
6. **Umbrella binary** — single installable `vyu` dispatching subcommands (optional).
7. **`cli-tree` crate** — extract `command_tree::render` from `mpt` when a second `[[bin]]` needs `tree`; keep tool-specific clap defs in each crate.

---

## ARCHIVE — do not process

| Item | Outcome |
|------|---------|
| RFC-0001 `mpt` format | Draft in [rfc-0001-mpt-format.md](documentation-site/src/content/docs/rfcs/rfc-0001-mpt-format.md); name review settled (`part-end <id>`, double-backtick sigil). |
| Workspace scaffold | Root `Cargo.toml`; crates `mpt`, `walk`, `pack`, `render`; `cargo test` green. |
| `mpt` crate (minimal) | Parse, validate, canonical serialize; CLI `validate` + `canonicalize`; RFC example integration tests. |
| `mpt` CLI part + merge | `mpt part extract|add|remove`; `mpt merge`; `mpt tree`; library `edit` + `command_tree` modules. |
