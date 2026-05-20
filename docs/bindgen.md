# `windows-bindgen` implementation simplification

This is a rolling list of internal simplifications for `crates/libs/bindgen/`.
The recently-landed option consolidation work (PRs #4441, #4443, #4444, #4445
and the `--deps`/`--extern` fold that retired the old `docs/options.md`) made
the user-facing surface small and orthogonal:

| Axis | Options |
|---|---|
| Input | `--in` |
| Output path / layout | `--out`, `--flat`, `--package` (+`--no-toml`), `--index` |
| Filter | `--filter`, `--implement[…]` |
| Code style | `--sys` (+`--extern`), `--minimal` |
| Dependencies | `--deps {core,specific,none}`, `--link` |
| Misc | `--derive`, `--rustfmt`, `--reference`, `--etc` |

The follow-up internal work to make the implementation match that surface
landed `DepMode`, `Option<&Implements>`, and the internal `Style`/`Layout`
enums with nested sub-flags (the four "set X without Y, panic at write()"
failure modes are now either unrepresentable or fail at the builder method
that introduced them). What's below is what's left.

## Outstanding simplifications

### A. Merge `Bindgen` and `Config` state

`Bindgen` (lib.rs, ~17 fields) and `Config` (config/mod.rs:11–30, 17 fields)
still hold parallel state. `Bindgen::write()` (lib.rs ~780–940) is mostly
plumbing:

- destructure the `Layout` / `Style` enums into the six derived booleans
  `flat`, `package`, `no_toml`, `sys`, `minimal`, `sys_fn_extern` (lib.rs
  ~795–800),
- copy each `Bindgen` field one-for-one into `Config` (lib.rs ~912–931),
- after which `Config` is purely read-only for the rest of the run.

Two designs to choose between, both better than the status quo:

1. **`Config<'a>` holds `&'a Bindgen`** — drops the 17 fields, keeps the
   transient state (`reader`, `types`, `references`, `filter`, `derive`,
   `warnings`, `namespace`, `link`, `implement: Option<&Implements>`) and
   exposes the rest via `&self.bindgen.deps`, `&self.bindgen.style`, etc.
2. **Merge them outright** — `Bindgen` *is* the only `Config` builder, and
   `Config` has no other constructor. Move the transient fields into
   `Bindgen` (or into a sibling `RunState` that `Config` owns) and let the
   emitters take `&Bindgen` directly.

Either way, this is also the point to drop the derived `flat` / `package` /
`no_toml` / `sys` / `minimal` / `sys_fn_extern` booleans on `Config` and have
the ~50 emitter call sites match on `Layout` / `Style` directly. That's the
substantive part of the diff and the reason this step was deferred until the
enums were in place.

A few care-abouts when doing this:

- `Config::with_namespace` (config/mod.rs:33–37) clones the whole struct on
  every namespace; reducing `Config`'s size with this merge is also the
  right time to make `with_namespace` not clone (e.g. by stashing
  `namespace` in a `Cell` or threading it as a parameter).
- `Bindgen::write` runs validation in the wrong order today (output / filter
  presence checks happen *after* the link string and input vec are built
  unnecessarily). When merging, lift validation into a single
  `Bindgen::validate()` (or `Bindgen::build_run()`) called at the top of
  `write` and have `bindgen()` call it too. The CLI parser at lib.rs
  ~360–445 should not need to repeat any of those checks.

### B. The `config/` module is mis-named

Five of the six files in `config/` are not configuration:

- `config/cpp_handle.rs` — `impl Config` block emitting handle types,
- `config/format.rs` — invokes `rustfmt`,
- `config/names.rs` — type-path / namespace path emission,
- `config/value.rs` — value-literal emission,
- `config/cfg.rs` — `cargo` feature / `#[cfg(...)]` emission for
  `--package`.

They were attached to `Config` for convenient field access, but
conceptually they belong with the other emitters under `types/`
(`types/cpp_handle.rs`, etc.) or as their own top-level module.

After Step A this becomes pure file moves: `Config` stops being the
`impl` host and the emitter functions take whatever subset of state they
actually need. Concrete tasks:

- Move `config/cpp_handle.rs` next to `types/cpp_const.rs`,
  `config/value.rs` next to the value emitters, `config/names.rs` into a
  new `paths.rs` (its functions are namespace path emission, not
  configuration).
- `config/cfg.rs::Cfg::write` early-returns unless `package` is the active
  layout (it's the only mode that emits cargo features at all). Push that
  guard into the call sites — or move the whole `Cfg` machinery under a
  `package_writer` module — so the non-package path stops allocating empty
  `Cfg` values.
- `config/format.rs` is just a `rustfmt` shell-out plus a fallback
  formatter; it can move to a top-level `format.rs` and stop being a
  `Config` method.

This step is best left until last so review diffs aren't tangled with the
state-merge in Step A.

## What inherently resists further simplification

Recording these so they aren't re-proposed:

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
- **`--package` vs `--flat` vs default modules** are three distinct output
  topologies driving different writer code paths. Already modelled as a
  `Layout` enum; collapsing further would force the writer to fork on
  every emit.

## Suggested order of attack

1. **Step A** — merge `Bindgen` and `Config`. This is the substantive
   change and the one that pays the most: ~30 fewer fields in flight,
   ~25 fewer lines of plumbing in `write`, and the emitter sites finally
   get to match on `Layout` / `Style` directly.
2. **Step B** — once `config/` is no longer the impl host, splitting it
   becomes mechanical file moves with no logic change.
