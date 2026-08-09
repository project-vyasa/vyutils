---
title: Edit parts in an mpt file
description: How to extract, add, remove, and merge parts using the mpt CLI.
---

Goal: change the contents of an `.mpt` file without hand-editing envelope lines.

Prerequisites: [mpt CLI reference](/vyutils/reference/mpt/), a valid `.mpt` file, and the `mpt` binary (`cargo build -p mpt` from the vyutils repo).

## Extract a part body

```bash
mpt part extract records/states.mpt states_psv
```

Pipe-friendly: the default output is **body bytes only** (no envelope).

## Extract with metadata

Part-level header payload (toml/yaml/etc. inside the part):

```bash
mpt part extract records/states.mpt states_psv --header
```

Standalone single-part document (useful for templates):

```bash
mpt part extract records/states.mpt states_psv --full -o /tmp/states-only.mpt
```

## Add a part

Append from a body file and rewrite in place:

```bash
mpt part add records/states.mpt washington \
  --body-file rows/wa.psv \
  --header-file rows/wa-header.toml \
  --in-place
```

Insert before an existing part:

```bash
mpt part add records/states.mpt intro --body-file intro.txt --before states_psv --in-place
```

Without `--in-place` or `-o`, the updated document is written to **stdout** (safe for scripts).

## Remove a part

```bash
mpt part remove records/states.mpt washington --in-place
```

A document must keep at least one part.

## Merge two files

```bash
mpt merge chapter-01.mpt chapter-02.mpt -o book.mpt
```

See [merge file headers](/vyutils/reference/mpt/#merge) in the reference for header rules.

## Validate after edits

```bash
mpt validate records/states.mpt
mpt canonicalize records/states.mpt   # optional: inspect normalized form
```
