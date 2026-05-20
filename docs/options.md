# `windows-bindgen` options: consolidation plan

This document inventories every option `windows-bindgen` exposes today (CLI flag
plus matching `Bindgen` builder method) and tracks the remaining work to shrink
that surface. The goal is fewer knobs, clearer defaults, and no options that
only exist to combine with another option.

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

1. **`--no-deps` and `--specific-deps` describe the dep graph from opposite
   ends.** They live alongside `--sys`, `--link`, and `--reference` with no
   single conceptual home and no shared spelling, even though they're
   mutually exclusive picks on the same axis.
2. **`--no-toml` is a sub-flag of `--package`**, with no meaning otherwise.
   Today it silently no-ops when `--package` is absent.
3. **`--sys-fn-extern` is a sub-flag of `--sys`**, and is silently a no-op
   without it. The name also doesn't read well as a top-level CLI switch.
4. **`--sys` vs `--minimal` are sibling "drop ergonomics" modes** that are
   mutually exclusive and overlap heavily (no `Result<T>` wrappers, no
   `IntoIterator`, no per-class wrappers, bare handles). They differ on
   vtables/ABI/`_Impl` (kept by `--minimal`, dropped by `--sys`) and on
   event auto-revocation (`--minimal` only). They are documented as two
   flags; this is intentional but worth re-checking against usage data.

## 3. Proposed option set

Replace the current code-style/cosmetic options with the table below.
"Structural" and "layout" options stay as-is (modulo the `--no-toml` cleanup
in §4).

| Option | Replaces | Notes |
|--------|----------|-------|
| `--sys` | unchanged | Still emits `fn`-pointer typedefs alongside the `link!` (folded in from the removed `--sys-fn-ptrs`). Handle typedef behavior is implied (folded in from the removed `--typedef`). |
| `--extern` | `--sys-fn-extern` | Renamed and promoted to a top-level "use `extern { fn … }` instead of `link!`" switch. Still only meaningful with `--sys`; documented as such, validated at parse time. |
| `--minimal` | unchanged | Handle typedef behavior is implied (folded in from the removed `--typedef`). |
| `--deps <mode>` | `--no-deps`, `--specific-deps` | Three values: `core` (default — today's behavior, depend on `windows-core`), `specific` (today's `--specific-deps` — depend on `windows-result`, `windows-strings`, `windows-link` directly), `none` (today's `--no-deps`). One axis, one option, exhaustive. |
| `--link <crate>` | unchanged | Still overrides the `link!` macro source. |

## 4. Sub-option cleanups

- `--no-toml` stays but is documented strictly as a `--package` modifier and
  rejected at parse time if `--package` is absent (today it silently no-ops).
- `--extern` (renamed `--sys-fn-extern`) is rejected without `--sys` rather
  than silently ignored.
- The mutual-exclusion check between `--sys` and `--minimal` stays. Likewise
  `--package` vs `--flat`.

## 5. Net change

Remaining work removes: `--no-deps`, `--specific-deps`, `--sys-fn-extern`.
(3 to remove.)

Adds: `--extern`, `--deps`. (2 to add.)

Combined with the items in §7, the original 11 code-style/cosmetic options
shrink to 5, with each remaining option controlling one independent
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
- `--no-allow` / `--no-comment` removed (#4441).
- `--noexcept` removed (#4385).
- `--implement` and `--implements` folded into a single `--implement[=<filter>]`
  option (#4445): bare flag emits `_Impl` for every WinRT interface in scope,
  optional trailing patterns narrow emission to the listed types. The
  `Bindgen::implements(...)` builder method has been deleted; the surviving
  `Bindgen::implement(names)` accepts an iterable (empty for broad, non-empty
  for narrow).

## 8. Resolved design choices

These were previously open questions; recorded here so the next pass doesn't
relitigate them.

- **Keep `--extern` (renamed from `--sys-fn-extern`)** rather than dropping
  it. The downstream cost of forcing a post-processing step on `link!` output
  outweighs the savings of removing one flag, and the rename plus parse-time
  validation already addresses the discoverability complaint.
- **Keep `--sys` and `--minimal` as two flags** rather than merging into
  `--style <sys|minimal>`. They are mutually exclusive, but the common case
  (`--sys` / `--minimal`) is short and unambiguous; a single option with
  values would only make the common case noisier.
- **Leave `--index` as an option on the main command**, not a sub-command.
  It composes cleanly with `--package` / `--out` and a sub-command split
  would require its own duplicated argument parsing.
