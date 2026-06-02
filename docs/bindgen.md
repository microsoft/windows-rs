# `windows-bindgen` implementation simplification

The user-facing CLI surface, after the option-consolidation work (PRs #4441,
#4443, #4444, #4445, and the `--deps`/`--extern` fold), is small and orthogonal:

| Axis | Options |
|---|---|
| Input | `--in` |
| Output path / layout | `--out`, `--flat`, `--package` (+`--no-toml`), `--index` |
| Filter | `--filter`, `--implement[…]` |
| Code style | `--sys` (+`--extern`), `--minimal` |
| Dependencies | `--deps {core,specific,none}`, `--link` |
| Misc | `--derive`, `--rustfmt`, `--reference`, `--etc` |

The internal follow-up to match that surface landed `DepMode`,
`Option<&Implements>`, and the `Layout` / `Style` enums with nested sub-flags
(the "set X without Y, panic at write()" failure modes are now either
unrepresentable or fail at the builder method that introduced them). Step A
merged the derived-boolean state of `Bindgen` into `Config<'a>` and lifted
validation to the top of `Bindgen::write`. Step B retired the `config/`
sub-module by moving each emitter file to its natural home (`cpp_handle.rs`
under `types/`, `format.rs` / `paths.rs` / `value.rs` at the top level, and
the new `package_writer.rs` that owns `write_package`).

This document is now mostly a record of design decisions so that they aren't
re-proposed.

## What inherently resists further simplification

- **`--sys` vs `--minimal`** are the same "lean mode" axis but produce
  different ABI contracts — `--minimal` preserves
  vtable/`_Impl`/`RuntimeType`, `--sys` does not. Two flags, one internal
  enum (`Style`), is the right shape.
- **`--deps none` vs `--sys`** are orthogonal. `--deps none` is also
  meaningful with `--minimal` (for the few `windows-*` crates that
  bootstrap themselves). Cannot fold one into the other.
- **`--reference` / `--link` / `--deps`** all touch dependencies but at
  three different layers (per-type reroute / `link!` macro source / which
  crate hosts shared types). Not foldable.
- **`--filter` method-level grammar** (`?Ns.Type`, `Ns.Type::Method`,
  etc., documented at lib.rs ~115–165): rich on purpose. A real DSL with
  documented escape hatches is better than an expanding set of top-level
  flags.
- **`--minimal` + COM interfaces**: In minimal mode, the dependency check
  for method emission is skipped. The type closure computed by
  `MinimalTypeMap::build` is authoritative — if a method is explicitly in
  the filter, its types are already in the map. The `dependencies.included()`
  guard only applies in non-minimal mode where the full transitive closure
  may not cover every method on every included interface.
- **`--package` vs `--flat` vs default modules** are three distinct output
  topologies driving different writer code paths. Already modelled as a
  `Layout` enum; collapsing further would force the writer to fork on
  every emit.
- **`Config<'a>` as a sidecar of run-state references next to `&'a Bindgen`**.
  Folding `Config` into `Bindgen` (or a sibling `RunState` owned by `Bindgen`)
  was considered (design 2 in the original Step A notes) and rejected: it
  would flip every emitter call site from `&Config` to `&Bindgen`/`&RunState`
  with no behavioural change, and `Bindgen` is a builder that exists before
  the run-state references can be constructed. Likewise, the `with_namespace`
  clone in `write_modules` / `write_package` copies a ~80-byte
  ten-reference struct a handful of times per namespace and is not worth
  threading `namespace` as a separate parameter through every emitter.
