# `windows-bindgen` implementation simplification

This is a review of `crates/libs/bindgen/` (~10.5 kLOC; `lib.rs` alone is 1098
lines) following the recently-landed option consolidation work (PRs #4441,
#4443, #4444, #4445 and the `--deps`/`--extern` fold that finally deleted the
old `docs/options.md`). It records what's in good shape, what can still be
simplified, what inherently resists further simplification, and a suggested
order of attack.

## What's in good shape

The user-facing surface is now genuinely small and orthogonal:

| Axis | Options |
|---|---|
| Input | `--in` |
| Output path / layout | `--out`, `--flat`, `--package` (+`--no-toml`), `--index` |
| Filter | `--filter`, `--implement[…]` |
| Code style | `--sys` (+`--extern`), `--minimal` |
| Dependencies | `--deps {core,specific,none}`, `--link` |
| Misc | `--derive`, `--rustfmt`, `--reference`, `--etc` |

The CLI-level cleanup is done. The internal representation, however, still
carries the *shape* of the pre-cleanup options. The simplifications below are
about making the implementation match the surface.

## Concrete simplifications

### 1. Collapse the duplicate state on `Bindgen` and `Config`

`Bindgen` (lib.rs:481–500) and `Config` (config/mod.rs:11–32) hold an almost
identical set of ~15 fields. `Bindgen::write()` (lib.rs:733–910, ~180 lines)
does little more than:

- re-run the same validation that `bindgen()` already runs (lib.rs:444–458 vs
  770–784 — exact duplicates),
- copy every field one-for-one into a new `Config`.

Two cheap wins:

- **Make `Config` hold `&Bindgen`** (or merge them — `Bindgen` is the only
  `Config` builder). All the `config.sys`, `config.minimal`, `config.no_deps`,
  … field accesses become method calls on a single struct.
- **Move validation into one place** (`Bindgen::validate()` or
  `Bindgen::build_config()`). The CLI parser shouldn't be repeating
  mutual-exclusion checks the builder already enforces.

### 2. Replace the `(no_deps: bool, specific_deps: bool)` pair with a single `DepMode` field

`DepMode` (the new public enum, lib.rs:929–939) is already the user-facing
model. Internally, `Bindgen::deps()` (lib.rs:622–638) takes a `DepMode` and
immediately decomposes it back into two booleans that are then forwarded to
`Config`. Every consumer (`config/names.rs:17–35`, the references block at
`lib.rs:794–863`, `types/mod.rs:822–823`) re-derives the mode from the two
booleans. Store it once.

### 3. Replace `(implement: bool, implements: Vec<String>)` with `Option<Vec<String>>`

Today `implement=true && implements.is_empty()` means "all";
`implement=true && !implements.is_empty()` means "narrow"; `implement=false`
means "off". The empty-vs-not-set distinction is encoded with a parallel
bool. A single `Option<Vec<String>>` (`None` = off, `Some([])` = all,
`Some([…])` = narrow) removes the invalid state
(`implement=false && !implements.is_empty()`) and lets `should_implement`
(config/mod.rs:48–57) become a straight three-arm match. `Implements::is_empty()`
(implements.rs:16) — currently ambiguous between "implement nothing" and
"implement everything" — disappears entirely.

### 4. Promote "lean modes" to a single enum

`sys: bool` and `minimal: bool` are mutually exclusive (panic-checked at
`lib.rs:448–450` and `774–776`). Internally there are a fair number of
`config.sys || config.minimal` sites (e.g. `config/cpp_handle.rs:9`,
`types/mod.rs` typedef paths). A `Style { Default, Sys, Minimal }` enum makes
the invariant unrepresentable, removes the runtime panic, and reads well at
the call sites:

```text
match config.style { Style::Sys | Style::Minimal => …, Style::Default => … }
```

The user-facing flags `--sys` / `--minimal` stay (the docs/options.md
"resolved design choices" section explicitly decided not to merge them) — this
is purely internal.

Similarly `flat: bool` and `package: bool` are a mutually exclusive layout
axis (panic at `lib.rs:444–446` and `770–772`). A
`Layout { Modules, Flat, Package }` enum has the same shape and benefit.

### 5. Make sub-flags structurally sub-flags

`--no-toml` is only meaningful with `--package`, and `--extern` is only
meaningful with `--sys`. Both are validated by runtime panic today. Inside the
model, `no_toml` could live as a field on a `Layout::Package { no_toml: bool }`
variant and `sys_fn_extern` as `Style::Sys { extern_fns: bool }`. After that:

- `--no-toml` without `--package` becomes structurally impossible to set in
  the builder API (today: `Bindgen::no_toml()` happily sets the field, and
  the panic fires only inside `write()`).
- The four panics at lib.rs:444–458 / 770–784 reduce to one (sys vs minimal) —
  and that one disappears too if simplification 4 is taken.

### 6. The `config/` module is mis-named

Five of the six files in `config/` (`cfg.rs`, `cpp_handle.rs`, `format.rs`,
`names.rs`, `value.rs`) are not configuration at all — they're
`impl Config<'_>` blocks that emit `TokenStream`s for handles, type paths,
value literals, derive-cfg attributes, etc. They were attached to `Config` for
convenient field access, but conceptually belong with the emitters
(`types/cpp_handle.rs` next to `types/cpp_const.rs`, etc.). Moving them out
leaves `config/mod.rs` and the genuine `Cfg` machinery, and lets `Config`
shrink to a data struct with a few orchestration methods (`write`,
`with_namespace`).

While moving things, `Cfg::write` (config/cfg.rs:76–141) early-returns unless
`config.package`, which is the only mode that emits cargo features at all.
Lifting that check into the call sites (or pushing the whole `Cfg` code under
`config::write_package`) would let the non-package path stop allocating empty
`Cfg` values.

### 7. Smaller leftovers

- `Bindgen` has both singular and plural builders for each multi-value option
  (`input`/`inputs`, `filter`/`filters`, `derive`/`derives`,
  `reference`/`references`, lib.rs:509–595, ~85 lines). The singular form is
  `iter::once(x)` of the plural. Worth keeping as a one-liner each but the
  boilerplate could halve.
- `rustfmt: String` and `link: String` (lib.rs:488–489) default to `""` with
  sentinel-string semantics ("empty means use the default"). `Option<String>`
  would match the public surface (`Bindgen::rustfmt` / `Bindgen::link` always
  set a value, never clear it).
- `prepend_default_refs` (lib.rs:1066–1073) uses repeated `Vec::insert(0, …)`
  to preserve order. An `extend` into a temporary + `splice(0..0, …)` is one
  allocation and one O(n) shift rather than n shifts.
- The hand-rolled state-machine arg parser (lib.rs:356–442) plus `ArgKind`
  (913–924) is fine but is a place where the `--etc` recursion lives
  undocumented in the public `bindgen` arg table at lib.rs:75–88. Worth
  either documenting or removing.

## What inherently resists further simplification

A few options look noisy but earn their keep, and shouldn't be conflated:

- **`--sys` vs `--minimal`**: same "lean mode" axis but different ABI
  contract — `--minimal` preserves vtable/`_Impl`/`RuntimeType`, `--sys` does
  not. Merging them would force every consumer to also encode "and which lean
  variant" everywhere. Two flags, one internal enum, is the right shape.
- **`--deps none` vs `--sys`**: orthogonal. `--deps none` is *also* meaningful
  with `--minimal` (for the few `windows-*` crates that bootstrap themselves).
  Cannot fold one into the other.
- **`--reference` / `--link` / `--deps`** all touch dependencies but at three
  different layers (per-type reroute / link! macro source / which crate hosts
  shared types). Not foldable.
- **`--filter` method-level grammar** (`?Ns.Type`, `Ns.Type::Method`, etc.,
  documented at lib.rs:126–176): rich on purpose. A real DSL with documented
  escape hatches is better than an expanding set of top-level flags.
- **`--package` vs `--flat` vs default modules**: three distinct output
  topologies driving different writer code paths. Best modelled as a `Layout`
  enum (above), not collapsed.

## Suggested order of attack

1. **Steps 2 + 3** (collapse `DepMode` and `Option<Vec<String>>` for
   implement) — mechanical, isolated, immediate readability win, and
   `should_implement`/dep-mode call sites simplify on the spot.
2. **Step 5** (move sub-flags into enum variants) — small but removes two of
   the four runtime panics.
3. **Step 4** (Style/Layout enums) — touches more files (every
   `config.sys || config.minimal` site, every `config.flat`/`config.package`
   site) but is purely a `&str` → enum substitution.
4. **Step 1** (merge `Bindgen` and `Config` state) — depends on 2–4 being
   done first, otherwise you're moving boilerplate around.
5. **Step 6** (split the `config/` directory) — pure file moves once the rest
   is done, and best left until last so review diffs aren't tangled with
   logic changes.

Steps 1–4 should cut `lib.rs` roughly in half (the validation duplication +
the field-copying + the `DepMode` re-decomposition + the
`implement`/`implements` parallel state account for most of `Bindgen::write`'s
180 lines), and remove three of the four "if you set X without Y, panic at
runtime" failure modes by making them unrepresentable.
