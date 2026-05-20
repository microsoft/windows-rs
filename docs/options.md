# `windows-bindgen` options

This document inventories every option `windows-bindgen` exposes today (CLI
flag plus matching `Bindgen` builder method) and records the consolidation
work that produced today's surface. The goal of that consolidation was fewer
knobs, clearer defaults, and no options that only exist to combine with
another option.

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
| `--no-toml` | With `--package`, skip rewriting `Cargo.toml`. Rejected at parse time without `--package`. |
| `--index` | Also emit a `features.json` index. |

Code-style switches:

| Option | Purpose |
|--------|---------|
| `--sys` | Raw / sys-style bindings (no wrappers, plain handles, etc.). Always emits `fn`-pointer typedefs alongside the `link!`. Mutually exclusive with `--minimal`. |
| `--extern` | Use `extern { fn … }` instead of `link!`. Rejected at parse time without `--sys`. |
| `--minimal` | Drop class wrappers, inherited forwarders, handle ergonomics, etc.; auto-revoke events. Mutually exclusive with `--sys`. |
| `--deps <mode>` | Selects how generated bindings depend on the `windows-*` crates. `core` (default — depend on `windows-core`), `specific` (depend on `windows-result`, `windows-strings`, `windows-link` directly), or `none` (no `windows-*` dependencies). |

The builder API mirrors the CLI: `sys()`, `extern_fns()`, `minimal()`,
`deps(DepMode::{Core, Specific, None})`.

## 2. Already landed

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
- `--no-deps`, `--specific-deps`, and `--sys-fn-extern` removed in favor of
  `--deps <core|specific|none>` and `--extern`. The corresponding
  `Bindgen::no_deps()`, `Bindgen::specific_deps()`, and
  `Bindgen::sys_fn_extern()` builder methods are replaced with
  `Bindgen::deps(DepMode)` and `Bindgen::extern_fns()`. `--no-toml` without
  `--package`, and `--extern` without `--sys`, are now parse-time errors
  rather than silent no-ops.

### Migration table

| Old invocation | New invocation |
|----------------|----------------|
| `--sys --sys-fn-extern` | `--sys --extern` |
| `--specific-deps` | `--deps specific` |
| `--no-deps` | `--deps none` |

## 3. Resolved design choices

Recorded here so the next pass doesn't relitigate them.

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
