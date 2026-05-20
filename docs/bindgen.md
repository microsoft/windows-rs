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
that introduced them). Step A landed the first half of the `Bindgen`/`Config`
state-merge: `Config<'a>` now holds `&'a Bindgen` (design 1), the six
derived booleans (`flat`/`package`/`no_toml`/`sys`/`minimal`/`sys_fn_extern`)
are gone, and emitter call sites query the `Layout` / `Style` enums
directly through predicate helpers (`is_sys`, `is_package`, `no_toml`, …).
Validation has been lifted to the top of `Bindgen::write` so it runs
before any expensive plumbing. The outright merge (design 2) is still
available as a follow-up — see "Follow-up to Step A" below. What's below
is what's left.

## Outstanding simplifications

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

Now that Step A has landed, this becomes pure file moves: `Config` no
longer needs to be the `impl` host (its remaining state is small and
exposed via `&self.bindgen`), and the emitter functions can take whatever
subset of state they actually need. Concrete tasks:

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

### Follow-up to Step A

Step A picked design 1 from the original doc (`Config<'a>` holds
`&'a Bindgen`) because it landed cleanly without churning the emitter
signatures. Design 2 — merge `Bindgen` and `Config` outright, so the
emitters take `&Bindgen` directly and the `bindgen` indirection
disappears — is still on the table as a further refinement:

- `Config<'a>` today is essentially a sidecar of run-state references
  (`reader`, `types`, `references`, `filter`, `derive`, `warnings`,
  `namespace`, `link`, `implement`) plus `&Bindgen`. Folding the run-state
  into `Bindgen` (or into a sibling `RunState` that `Bindgen` owns for the
  duration of `write`) removes the last duplicate type.
- That move is also the right time to make `Config::with_namespace` not
  clone (it currently clones the whole struct on every namespace; even
  though `Config` is now small (~10 reference fields), stashing
  `namespace` in a `Cell` or threading it as a parameter avoids the clone
  entirely).

Doing this *now* would mostly be churn — every emitter call site would
flip from `config: &Config` to `bindgen: &Bindgen` (or similar) with no
behavioural change. It makes more sense to fold this in alongside Step
B, when the `config/` files are being moved out anyway.

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

