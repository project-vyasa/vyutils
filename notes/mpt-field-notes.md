# mpt field notes

Observations from real use of the `.mpt` format and `mpt` tooling (`n=1` is fine).

**Workflow**

| Stage | Where |
|-------|--------|
| Notice friction | This file (append an entry) |
| Decide spec change | [RFC-0001](../documentation-site/src/content/docs/rfcs/rfc-0001-mpt-format.md) + Changelog |
| Schedule work | [WORK.md](../WORK.md) NOW / NEXT (one line; link back here) |

**How to add entries:** jot prose in chat or inline below the template; ask the agent to turn it into a structured entry.

**Disposition** (set when triaged):

| Value | Meaning |
|-------|---------|
| `open` | Not yet triaged |
| `ack` | Understood; no change planned |
| `rfc-change?` | May need RFC update — discuss |
| `tooling` | Format OK; improve CLI / errors / docs |
| `done` | Addressed (link commit or RFC entry) |

---

## Entry template

```markdown
### YYYY-MM-DD — short title

- **Friction:**
- **Example:** `notes/mpt-test/…` or `cargo run -p mpt -- validate …`
- **Expected:**
- **Actual:**
- **Disposition:** open | ack | rfc-change? | tooling | done
- **Notes:**
```

---

## Entries

### 2026-08-08 — part id vs filename extension

- **Friction:** Part id `file01.psv` felt natural (mirrors a pipe-delimited filename); validator rejected it.
- **Example:** `notes/mpt-test/tabular-data.mpt` (originally ``part file01.psv``; now ``part states_psv``)
- **Expected:** Dots in part ids, like URL path segments or `file.psv`.
- **Actual:** RFC `PartId` allows only `[a-z0-9-]`; `mpt validate` → `invalid part id 'file01.psv'`.
- **Disposition:** ack
- **Notes:** `.` is URL-safe (RFC 3986 unreserved); mpt uses a stricter slug grammar. Use `states-psv` or `states_psv` instead of extension-style ids. Underscore `_` is allowed; dot `.` is not.

### 2026-08-09 — bracket envelope syntax

- **Friction:** `part-end`, `header-end` felt verbose; wanted uniform open/close from the user's perspective.
- **Example:** [RFC-0001](../documentation-site/src/content/docs/rfcs/rfc-0001-mpt-format.md) (updated)
- **Expected:** ``[ label`` / ``] label`` — same pattern for headers and parts; keep `header` keyword for grepability.
- **Actual:** Adopted in RFC + `mpt` parser. `header` reserved as label; part ids cannot be `header`.
- **Disposition:** done
- **Notes:** Legacy ``part`` / ``part-end`` / ``header-end`` syntax rejected. Checksums stay optional in header payload; compile-time integrity deferred to `pack`.
