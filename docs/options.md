# `windows-bindgen` options: consolidation plan

This document inventories every option `windows-bindgen` exposes today (CLI flag
plus matching `Bindgen` builder method), groups them by intent, and proposes a
smaller, more orthogonal set. The goal is fewer knobs, clearer defaults, and no
options that only exist to combine with another option.

## 1. Current options

Structural (required / always-on inputs):

| Option | Purpose |
|--------|---------|
| `--in` / `input` | Source `.winmd` (or `"default"`). |
| `--out` / `output` | Output file or package directory. |
| `--filter` / `filter` | Include / exclude rules (incl. method-level). |
| `--reference` / `reference` | External crate references. |
| `--implement` / `implement` | Per-type list to emit `_Impl` for (or all-in-scope when no list is provided). |
| `--rustfmt` / `rustfmt` | Override `rustfmt` config. |
| `--link` / `link` | Override the `link!` macro source crate. |
| `--derive` / `derive` | Extra `derive`s for selected types. |
| `--etc` | Inline-expand args from a file. |

Output layout:

| Option | Purpose |
|--------|---------|
| `--flat` | Drop namespace modules (single flat list). |
| `--package` | Emit one file per namespace + `Cargo.toml` features. |
| `--no-toml` | With `--package`, skip rewriting `Cargo.toml`. |
| `--index` | Also emit a `features.json` index. |

Code-style switches (the ones this plan targets):

| Option | Purpose |
|--------|---------|
| `--sys` | Raw / sys-style bindings (no wrappers, plain handles, etc.). Always emits `fn`-pointer typedefs alongside the `link!`. |
| `--sys-fn-extern` | Use `extern { fn … }` instead of `link!` (only with `--sys`). |
| `--minimal` | Drop class wrappers, inherited forwarders, handle ergonomics, etc.; auto-revoke events. Mutually exclusive with `--sys`. |
| `--no-deps` | Avoid pulling in `windows-*` crate references. |
| `--specific-deps` | Reference `windows-result` / `windows-strings` / `windows-link` directly instead of `windows-core`. |

## 2. Problems observed

1. **`--sys` vs `--minimal` are sibling "drop ergonomics" modes** that are
   mutually exclusive and overlap heavily:
   - Both: bare handles, no `Result<T>` free-function wrappers, no
     `IntoIterator`, no per-class wrappers (minimal explicitly; sys by
     construction).
   - `--sys` additionally: collapses enums/structs (no `derive`s, no traits,
     vararg, etc.), no vtables for interfaces, single `link!` per fn.
   - `--minimal` additionally: keeps vtables/ABI/`_Impl`, auto-revokes events.
   The user-facing axis is the same ("how stripped-down do you want it?"),
   but they're spelled with two flags and explicitly forbidden together.
2. **`--no-deps` and `--specific-deps` describe the dep graph from opposite
   ends** but live alongside `--sys`, `--link`, and `--reference` with no
   single conceptual home.
3. **`--no-toml` is a sub-flag of `--package`**, with no meaning otherwise.
4. **`--sys-fn-extern` is a sub-flag of `--sys`**, and is silently a no-op
   without it.

## 3. Proposed option set

Replace the current code-style/cosmetic options with the table below.
"Structural" and "layout" options stay as-is.

| Option | Replaces | Notes |
|--------|----------|-------|
| `--sys` | unchanged | Still emits `fn`-pointer typedefs alongside the `link!` (folded in from the removed `--sys-fn-ptrs`). Handle typedef behavior is implied (folded in from the removed `--typedef`). |
| `--extern` | `--sys-fn-extern` | Renamed and promoted to a top-level "use `extern { fn … }` instead of `link!`" switch. Still only meaningful with `--sys`; documented as such, validated at parse time. (Alternative: drop entirely and always emit `link!`; we keep it because some downstream users want raw `extern` blocks.) |
| `--minimal` | unchanged | Handle typedef behavior is implied (folded in from the removed `--typedef`). |
| `--deps <mode>` | `--no-deps`, `--specific-deps` | Three values: `core` (default — today's behavior, depend on `windows-core`), `specific` (today's `--specific-deps` — depend on `windows-result`, `windows-strings`, `windows-link` directly), `none` (today's `--no-deps`). One axis, one option, exhaustive. |
| `--link <crate>` | unchanged | Still overrides the `link!` macro source. |

## 4. Sub-option cleanups

- `--no-toml` stays but is documented strictly as a `--package` modifier and
  rejected at parse time if `--package` is absent (today it silently no-ops).
- The mutual-exclusion check between `--sys` and `--minimal` stays. Likewise
  `--package` vs `--flat`.
- `--extern` (renamed `--sys-fn-extern`) is rejected without `--sys` rather
  than silently ignored.

## 5. Net change

Remaining work removes: `--no-deps`, `--specific-deps`, `--sys-fn-extern`.
(3 to remove.)

Adds: `--extern`, `--deps`. (2 to add.)

Combined with earlier landings (`--sys-fn-ptrs`, `--typedef`, and the
`--implement` / `--implements` fold), the original 11 code-style/cosmetic
options shrink to 5, with each remaining option controlling one independent
dimension of the output.

## 6. Migration

| Old invocation | New invocation |
|----------------|----------------|
| `--sys --sys-fn-extern` | `--sys --extern` |
| `--specific-deps` | `--deps specific` |
| `--no-deps` | `--deps none` |

The builder API mirrors the CLI: `sys()`, `extern_fns()`, `minimal()`,
`deps(DepMode::{Core, Specific, None})`. Today's `no_deps()`,
`specific_deps()`, and `sys_fn_extern()` methods are deleted.

## 7. Already landed

- `--sys-fn-ptrs` folded into `--sys` (#4443).
- `--typedef` removed; behavior implied by `--sys` / `--minimal` (#4444).
- `--implement` and `--implements` folded into a single `--implement[=<filter>]`
  option: bare flag emits `_Impl` for every WinRT interface in scope, optional
  trailing patterns narrow emission to the listed types. The
  `Bindgen::implements(...)` builder method has been deleted; the surviving
  `Bindgen::implement(names)` accepts an iterable (empty for broad,
  non-empty for narrow).

## 8. Open questions

- Should `--extern` survive at all, or is the small set of users who want raw
  `extern` blocks better served by `--sys` plus a post-processing step? If we
  drop it we get to 4 style options.
- Should `--sys` and `--minimal` merge into a single `--style <sys|minimal>`
  (or even a single `--lean` flag with a documented "best-effort sys" filter
  recipe)? They are already mutually exclusive, so a single option with values
  is the natural shape; the cost is a slightly noisier CLI for the common case
  (`--style sys` vs `--sys`). Recommendation: keep them as two flags for
  ergonomics, but document them as "pick at most one of these style modes."
- `--index` is orthogonal but rarely used; consider promoting it to a separate
  `windows-bindgen index` sub-command in a follow-up.
