---
title: mpt CLI
description: Command-line reference for the mpt multi-part-text tool.
---

The `mpt` binary validates and edits [multi-part-text](/vyutils/rfcs/rfc-0001-mpt-format/) (`.mpt`) documents. Normative grammar lives in [RFC-0001](/vyutils/rfcs/rfc-0001-mpt-format/).

## Installation

From the [vyutils](https://github.com/project-vyasa/vyutils) repository:

```bash
cargo build -p mpt
# binary: target/debug/mpt

cargo run -p mpt -- validate path/to/file.mpt
```

## Command tree

Run `mpt tree` for a concise snapshot of every subcommand, positional argument, and flag. The tree is generated from the CLI definition, so it stays in sync with `mpt --help`.

```bash
mpt tree
```

Example output (abbreviated):

```text
mpt
├── validate <path>
├── canonicalize <path>
├── merge <input>…
│   └── -o, --output <output>
├── tree
└── part
    ├── extract <path> <id>
    │   ├── --full
    │   ├── --header
    │   └── -o, --output <output>
    ├── add <path> <id>
    │   └── …
    └── remove <path> <id>
        └── …
```

## Synopsis

```text
mpt validate <path>
mpt canonicalize <path>
mpt merge <input>… [-o <path>]
mpt part extract <path> <id> [-o <path>] [--header | --full]
mpt part add <path> <id> [options]
mpt part remove <path> <id> [options]
```

## Conventions

| Topic | Behavior |
|-------|----------|
| **Stdout default** | Mutating commands (`add`, `remove`, `merge` without `-o`) print the canonical document to stdout. |
| **In-place** | `--in-place` rewrites the input file (`add`, `remove` only). Conflicts with `-o`. |
| **Canonical output** | Serialize uses LF line endings and omits default `format = …` on envelope lines. |
| **Exit code** | `0` on success; non-zero on parse, validation, or edit errors. |
| **Errors** | Messages on stderr; format `path: reason`. |

## `mpt validate`

Parse and validate structure. Does not interpret payload formats (toml, yaml, etc.).

```text
mpt validate <path>
```

| Argument | Description |
|----------|-------------|
| `<path>` | Path to an `.mpt` file. |

## `mpt canonicalize`

Parse and print the canonical serialized document to stdout.

```text
mpt canonicalize <path>
```

| Argument | Description |
|----------|-------------|
| `<path>` | Path to an `.mpt` file. |

## `mpt merge`

Concatenate parts from multiple documents into one, preserving order within each input file.

```text
mpt merge <input>… [-o <path>]
```

| Argument / flag | Description |
|-----------------|-------------|
| `<input>…` | One or more `.mpt` files (at least one). |
| `-o`, `--output` | Write merged document to `<path>`. Default: stdout. |

### File headers on merge

| Situation | Result |
|-----------|--------|
| Only the first document has a file header | That header is kept. |
| Only a later document has a file header | Its header is adopted. |
| Two or more documents have a file header | **Error:** `cannot merge: more than one file header across inputs`. |
| No document has a file header | Merged document has no file header. |

Part ids must be unique across all inputs; duplicates produce `duplicate part id`.

## `mpt part extract`

Extract content from a named part.

```text
mpt part extract <path> <id> [-o <path>] [--header | --full]
```

| Argument / flag | Description |
|-----------------|-------------|
| `<path>` | Path to an `.mpt` file. |
| `<id>` | Part id. |
| `-o`, `--output` | Write to `<path>`. Default: stdout. |
| `--header` | Output the part-level **header payload** only. Error if the part has no header block. |
| `--full` | Output a standalone single-part `.mpt` document. Conflicts with `--header`. |

Default (no flags): part **body** bytes only.

## `mpt part add`

Insert a new part. Body is read from `--body-file` or **stdin** when omitted.

```text
mpt part add <path> <id> [options]
```

| Flag | Description |
|------|-------------|
| `--format <name>` | Body format on the part envelope. Default: `text`. |
| `--body-file <path>` | Body payload file. Default: stdin. |
| `--header-file <path>` | Optional part-level header payload file. |
| `--header-format <name>` | Header payload format when `--header-file` is set. Default: `toml`. |
| `--before <id>` | Insert before existing part `<id>`. |
| `--after <id>` | Insert after existing part `<id>`. |
| `-o`, `--output` | Write updated document to `<path>`. |
| `--in-place` | Rewrite `<path>`. Conflicts with `-o`. |

If neither `--before` nor `--after` is set, the part is appended at the end.

Registered format names: `toml`, `text`, `yaml`, `json5`, `xml`, `vyasa`, `csv`.

## `mpt part remove`

Remove a part by id.

```text
mpt part remove <path> <id> [options]
```

| Flag | Description |
|------|-------------|
| `-o`, `--output` | Write updated document to `<path>`. |
| `--in-place` | Rewrite `<path>`. Conflicts with `-o`. |

Cannot remove the last remaining part.

## Library

The same operations are available from the `mpt` Rust crate:

| Module / symbol | Role |
|-----------------|------|
| `parse`, `to_string` | Parse and canonical serialize |
| `merge` | Merge documents |
| `add_part`, `remove_part`, `find_part` | Part edits |
| `InsertPosition` | `End`, `Before(id)`, `After(id)` |

## See also

- [RFC-0001 — mpt format](/vyutils/rfcs/rfc-0001-mpt-format/)
- [How-to: edit parts](/vyutils/guides/edit-mpt-parts/)
