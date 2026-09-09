# Interlinked section items

Transient notes. Not an RFC.

## Need

Interactive documents need **data inside the document**, not only HTML/prose structure. Choosing a node should light up related nodes (different personas, different entry points).

## Draft model

A **block** has:

| Field | Role |
|-------|------|
| `id` | Unique in its container |
| `phrase` | 2–3 word caption / link text |
| `sentence` | ~120 characters |
| `para` | A few lines |
| `color`, `icon` | Optional display cues |

A **container** is a block whose body is an **ordered sequence** of blocks.

A **document** is the top-level container (a file), plus extras such as styles or namespace.

**Shape:** mostly a tree, plus links into short reference lists → a graph.

## Example: opportunity assessment

Beneficiary → problems (current state) → measures → target state → solutions, plus execution sequence, investment, risks, unknowns.

UI: pick a measure → related problems, solutions, steps. Pick a solution → problem, beneficiary, investment.

## mpt fit (open)

mpt is a **flat** container of named parts. Nesting and typed edges are not envelope features.

Likely **hybrid**, not one mapping for every block:

- **Typed tables** — repeating, queryable kinds (`beneficiaries`, `problems`, `measures`, …) with foreign keys. Pack/SQLite-friendly.
- **Nested payload** — section-like containers whose meaning is order and nesting (`vyasa` or a small block grammar inside a part). mpt stays the file envelope.

One part per block is probably too heavy; skip unless a kind is rare and prose-first.

## Appendix — opportunity assessment (narrative)

`Benefeciary` who will benefit from
`Problems` with the `current-state` for `benefeciaries`
How are the problems quantified `Meaures`
Descript the `target-state` from the point of view of `beneficiaries` i.e. how does their experience change?
What `solutions` must be implemented to address the `problems` and reach `target-state`?
Other data: `execution sequence`, `investment` required, `risks`, `unknowns`, and so on.

This a graph of blocks, a good subset is a tree with links to short "reference lists".

Interactive document for various personas (e.g. investor) with different starting points to explore the relationships and implicaitons
Choosing a metric for instance will show the relevant problems, solutions and perhaps execution steps associated with the metric.
Choosing a solution will light up the problem, the beneficiary and perhaps the investment required.
