# windows-bindgen: usage survey and simplification report

This report analyses how `windows-bindgen` (`crates/libs/bindgen`) is used to
generate the `bindings.rs` files that ship inside this workspace, identifies
what makes the crate complicated today, and proposes how it could be
simplified — with and without breaking changes — including a discussion of
whether a fresh implementation would be worthwhile.

The focus is deliberately on **in-repo usage** (the `crates/libs/*` crates
that ship to crates.io), not on the test/sample build scripts under
`crates/tests/*` or `crates/samples/*`. Those are also considered, but only
as a sanity check that the in-repo lib usage represents the dominant use
case.

## 1. What windows-bindgen does

`crates/libs/bindgen` is a ~8.1 kLOC code generator (across `src/`,
`src/types/`, `src/tokens/`, `src/config/`, `src/winmd/`) that takes:

1. One or more `.winmd` files (Windows metadata: ECMA‑335 with WinRT
   extensions), read by a small in-tree winmd reader in
   `src/winmd/reader.rs` (~200 lines). The crate also bundles a default
   metadata set under `crates/libs/bindgen/default/` (`Windows.winmd`,
   `Windows.Win32.winmd`, `Windows.Wdk.winmd`) via `include_bytes!`, so that
   `bindgen([...])` works without any external file references.
2. A `--filter` list with include/exclude rules at namespace, type, or
   `Type::Method` granularity, plus the `?Type` prefix that denotes
   "trait-only" emission.
3. An output destination — either a single file via `--out`, or a
   `--package` tree of `mod.rs` files plus a feature graph appended to a
   `Cargo.toml`.
4. Roughly 15 boolean shape switches that influence the emitted code.

It then walks the metadata into a `TypeMap`/`TypeTree`, emits a `TokenStream`
(through a hand-rolled mini quote/`ToTokens` in `src/tokens/`), and shells
out to `rustfmt`.

## 2. How `crates/libs/*` actually uses it

The orchestrator is `crates/tools/bindings/src/main.rs`, which runs
`bindgen(["--etc", "<file>.txt"])` once per argfile in
`crates/tools/bindings/src/`. There are 15 of these argfiles:
`collections.txt`, `core.txt`, `core_com.txt`, `future.txt`,
`future_impl.txt`, `metadata.txt`, `numerics.txt`, `reference.txt`,
`registry.txt`, `result.txt`, `services.txt`, `strings.txt`, `sys.txt`,
`threading.txt`, `version.txt`, `windows.txt`.

Reading all 15, every in-repo lib invocation falls into exactly **three
shapes**:

| Shape | Used for | Flags (besides `--out` / `--filter`) |
|---|---|---|
| **A. Single-file `sys` bindings** for the leaf "support" crates that `windows-core` itself depends on | `crates/libs/{core/imp/bindings, future/bindings_impl, metadata, registry, result, services, strings, threading, version}` | `--flat --sys --no-comment --no-allow --no-deps` (`strings.txt` omits `--no-allow`) |
| **B. Single-file WinRT bindings, flat, no comment, no allow** | `crates/libs/{collections, future, numerics, core/imp/com_bindings}` and `reference` (the only `--minimal` user) | `--flat --no-comment --no-allow` (+ `--minimal` for `reference.txt`, `--no-deps` for `core_com.txt`) |
| **C. Big package outputs** for the user-facing umbrella crates | `crates/libs/sys`, `crates/libs/windows` | `--package` + `--no-comment --no-allow --rustfmt max_width=800,newline_style=Unix`; `windows.txt` adds `--index`, `sys.txt` adds `--sys` |

Notable observations from this usage survey:

- **No in-repo lib uses `--specific-deps`.** It exists only so external
  downstream consumers can carve a smaller dependency tree.
- **No in-repo lib uses `--implement`** in its own `bindings.rs`.
  `--implement` only appears in `crates/tests/winrt/*/build.rs` (component
  scaffolding tests) and in external consumers; the in-tree libs implement
  WinRT interfaces through hand-written `implement_decl!` instead.
- **No in-repo lib uses `--implements`** (the finer-grained sibling of
  `--implement`).
- **No in-repo lib uses `--sys-fn-ptrs`, `--sys-fn-extern`, or `--typedef`.**
- **No in-repo lib uses `--link`.** The default (`windows_link` for
  `--sys`/`--specific-deps`, otherwise `windows_core`) suffices.
- **No in-repo lib uses non-`--flat` single-file output.** Single-file
  output is always `--flat`. Module-nested output is exclusive to
  `--package`.
- **Only `reference.txt` uses `--minimal`**, to drop wrapper methods and
  `interface_hierarchy!` machinery while keeping vtables, GUIDs, and
  `RuntimeType` signatures intact.
- **Both `--no-comment` and `--no-allow` are toggled on for every in-repo
  invocation.** The defaults are tuned for one-shot external consumers, not
  for the workspace itself.
- `--no-deps` only appears together with `--sys`. Code-side,
  `Type::write_no_deps` (`src/types/mod.rs:822`) is gated by
  `!config.no_deps || !config.sys`, so the flag's only practical meaning is
  "when in `--sys` mode, additionally inline `HRESULT` / `BOOL` / `PWSTR` /
  `HSTRING` / `GUID` / `IUnknown` typedefs."
- The two `--package` callers (`sys.txt`, `windows.txt`) are the only ones
  that exercise the namespace-feature-graph emitter, the Cargo.toml feature
  appender, and `--index`.

## 3. What makes windows-bindgen complicated

Walking the code, the 45 distinct `config.<flag>` branch sites across
`src/types/*.rs` and `src/tokens/runtime.rs` cluster into a handful of
overlapping concerns, none of which is inherently large but whose
combination is.

### 3.1 The "shape" axis is really 3–4 modes pretending to be N booleans

`Config` carries 14 booleans (`flat`, `no_allow`, `no_comment`, `no_deps`,
`no_toml`, `package`, `sys`, `minimal`, `typedef`, `sys_fn_ptrs`,
`sys_fn_extern`, `implement`, `specific_deps`, plus `implements:
&Implements`). In practice they collapse to a small set of **modes**:

- `sys` vs `safe`. This is the dominant axis. It changes
  `write_name` / `write_default` / `write_abi`, the `cpp_fn`, `cpp_enum`,
  `cpp_struct`, `cpp_interface`, `cpp_const`, `enum`, `struct`,
  `interface`, and `method` emitters, and the namespace lookup in
  `config/names.rs::write_specific`. Roughly half of all flag branches are
  `if config.sys`.
- `minimal` is a strict *subset* of `safe` (it's "safe-mode without
  wrappers"), explicitly rejected as combinable with `sys`
  (`src/lib.rs:799-801`), and used today by exactly one consumer.
- `package` vs `single-file` (and `flat` only makes sense without
  `package`). The combinations `package && flat` and `sys && minimal` are
  explicit panics.
- `sys_fn_ptrs` and `sys_fn_extern` only mean anything under `sys`. They
  produce branches like `if config.sys && config.sys_fn_ptrs`.
- `typedef` is documented as "sys-style type aliases" — yet
  `grep config\.typedef` finds it referenced only in `lib.rs` and
  `config/mod.rs`, not consumed inside the emitters anywhere. It is
  effectively dead in this workspace and likely only kept for one external
  pattern.
- `no_deps` only changes behaviour with `sys` (see `Type::write_no_deps`).
- `no_allow`, `no_comment`, `no_toml` are formatter/wrapper-only concerns
  confined to `config/format.rs` and `config/mod.rs::write_package`.

So the *real* shape matrix is much smaller than 2¹⁴:
**`{sys, safe, minimal-safe} × {single-file-flat, package}`** =
~6 meaningful combinations, plus a handful of orthogonal preamble switches.

### 3.2 Emission-time conditionals scattered across files

Every emitter (`types/interface.rs`, `types/cpp_struct.rs`,
`types/cpp_fn.rs`, `types/cpp_interface.rs`, `types/enum.rs`,
`types/cpp_enum.rs`, `types/class.rs`, …) inlines its own `if config.sys`,
`if config.minimal`, `if config.package` ladder, often interleaved with
metadata-driven branches (`is_exclusive`, `is_factory`, `is_trait_only`,
`is_agile`, `has_explicit_layout`, …). Result: each "what does sys mode
actually emit" question requires reading 10+ files. There is no single
place where the shape contract is stated.

The naming helpers (`config/names.rs::write_specific`, the `write_core` /
`write_result` / `write_strings` family) bake the cartesian product of
`sys × package × no_deps × flat × specific_deps × namespace-relative-paths`
into one ~20-line function that is invoked from dozens of places. This is
one of the densest knots.

### 3.3 The `--package` path duplicates the single-file path

`config/mod.rs::write_package` re-walks the tree, splices a Cargo.toml
feature graph, and reformulates `cfg` attributes against per-namespace
boundaries (`Cfg::new`, `class_cfg`). Many emitters carry a
`let cfg = if config.package { … } else { … };` shim. Combined with
`--specific-deps` namespacing rules in `write_specific`, this is the
second-biggest source of branching.

### 3.4 The `--implement` / `--implements` / `?Type` "trait-only" axis

Three knobs control `_Impl` emission:

1. `--implement` (emit `_Impl` for all WinRT interfaces in the filter set);
2. `--implements <name…>` (emit `_Impl` only for matching types);
3. the `?` prefix on filter entries (`Filter::is_trait_only`) which
   suppresses the inherent `impl IFoo` wrapper block but keeps the `_Impl`
   trait.

`Config::should_implement` (`src/config/mod.rs:52`) reconciles (1) and (2).
The `?` axis is essentially "don't emit caller-side wrappers for this
required-but-uncalled interface" and is used in only one in-repo place
(`reference.txt`'s `?Windows.Foundation.IPropertyValue`). Conceptually
these knobs are answering one question — "for which types should we emit
which of {ABI vtable, caller wrappers, `_Impl` trait}?" — but they are
spread across three input syntaxes and reconciled in two files.

### 3.5 References / cross-crate gluing

`src/references.rs` (`ReferenceStage` / `ReferenceStyle`) and the giant
`references.insert(0, …)` block in `src/lib.rs:805-1032` hard-code the
layout knowledge of `windows_future`, `windows_collections`,
`windows_reference`, `windows_numerics`, `windows_core`, `windows_result`.
This block is ~230 lines of literally enumerated namespace-to-crate-name
mappings, conditionally added unless the user passed `--no-deps`/`--sys`.
It is configuration-as-code in the worst sense: every new `windows-*`
split-crate has to be added here, and every breaking change to one of
those crates has to ripple through.

### 3.6 Side concerns

- `src/derive.rs` / `src/derive_writer.rs` (a small derives DSL parsed from
  `--derive` strings).
- `src/warnings.rs` (a side-channel `WarningBuilder` for non-fatal
  messages).
- `src/index.rs` (the `features.json` writer, only used by `windows.txt`).
- The bundled `default/Windows*.winmd` files (~MB of metadata embedded into
  the binary via `include_bytes!`).
- The `--etc <file>` argfile expander (`expand_args`), which is the only
  reason the libs orchestration works at all.
- `expand_args` panics rather than returning errors. The entire CLI surface
  panics on user error.

## 4. What could be consolidated, even with breaking changes

Working from "what does the workspace actually need" + "what do external
builds in this repo demonstrate":

### 4.1 High-confidence simplifications (low / no behaviour loss)

1. **Collapse `--sys-fn-ptrs`, `--sys-fn-extern`, `--typedef` into a single
   explicit mode (or remove).** None of them are exercised by any
   `crates/libs/*` argfile, nor by any in-repo build script.
   `--typedef` appears to be effectively dead (no `config.typedef` reads in
   `types/`). These three switches are pure tax on the matrix.
2. **Make `--flat` implicit when not using `--package`, and forbid the
   other combination instead of panicking.** Every in-repo single-file
   output uses `--flat`. The non-flat single-file path is preserved only
   for backwards compatibility with downstream users and adds a separate
   `write_modules` recursion path in `config/mod.rs`.
3. **Merge `--no-allow` / `--no-comment` / `--no-toml` into one
   `--preamble=<minimal|default>`.** Every in-repo invocation passes both
   `--no-allow --no-comment`. Most of the matrix has only two used corners.
4. **Fold `--implement` into `--implements <pattern>`.** `--implement` is
   exactly `--implements <every WinRT interface in the filter set>`. A
   single switch with namespace wildcards (e.g. `--implements Windows.**`,
   the empty-list default = today's behaviour) subsumes it and removes the
   `config.implement` vs `config.implements` reconciliation in
   `Config::should_implement`.
5. **Merge the `?Ns.Type` "trait-only" syntax into `--implements`.**
   Conceptually they answer the same question (which types get caller
   wrappers vs trait-only scaffolding). One combined option matrix removes
   a parser branch in `Filter` and an extra call site.
6. **Drop `--no-deps`'s overload of "also inline primitive typedefs."**
   Make that an explicit `--sys-prelude=embed` mode (or just always embed
   for `--sys`, conditionally use externally-imported names otherwise).
   Today `--no-deps` is silently a no-op outside `--sys` because of
   `write_no_deps`'s gate.
7. **Hide the cross-crate references table behind data, not code.** The
   230-line block of `references.insert(0, …)` should be a single
   `&'static [(crate_name, style, namespace)]` array (or read from a small
   TOML next to the crate). This both removes most of
   `src/lib.rs:805-1032` and makes adding/removing a `windows-*` support
   crate a one-line change.

These ~7 cleanups alone would remove most of the option-permutation
problem without losing any capability the workspace itself uses.

### 4.2 Plausible-but-larger simplifications (with breaking changes)

8. **Promote `sys` / `safe` / `minimal` to a single enum** (`--mode=sys |
   safe | safe-minimal`), and forbid the orthogonal-boolean style
   entirely. Most of the 45 `config.<flag>` branch sites today can become
   `match config.mode { Mode::Sys => …, Mode::Safe => …, Mode::Minimal =>
   … }` (or, better, dispatched through a small `Mode` trait so each
   emitter has one impl per mode). This is what the code already does
   emergently via panics on incompatible flag combinations.
9. **Move `--package` into a separate `bindgen-package` entry point** (or
   even a thin wrapper crate). It is a fundamentally different operation
   (writes many files + mutates `Cargo.toml` + emits `cfg(feature = "…")`
   per type) and lives behind a few `if config.package` branches that
   bleed into emitters that do not otherwise care. Separating it would let
   `Config` drop `package`, `no_toml`, and the Cargo.toml feature graph
   code; the package writer would invoke the single-file emitter per
   namespace.
10. **Replace the `&str`-everywhere CLI builder with a structured `Config`
    literal** and downgrade `bindgen(args)` to a thin parser sitting on
    top. The current `pub struct Bindgen { 23 fields }` plus mirror
    `pub struct Config<'a> { 21 fields }` plus the `ArgKind` state machine
    is three representations of the same data. A `serde`-able `Config`
    struct (TOML or in-process literal) would let
    `crates/tools/bindings/src/*.txt` become `*.toml` with strong typing,
    and would obviate `expand_args`.
11. **Type-driven dispatch in emitters.** Replace the central `Type`
    enum's `write_name` / `write_default` / `write_abi` /
    `write_impl_name` family — which inline `if config.sys` branches across
    ~1,000 lines in `types/mod.rs` — with a small trait per mode
    (`SysEmitter`, `SafeEmitter`) so each primitive's rules live in one
    place per mode. This is mechanical refactoring but pays back in every
    emitter file.
12. **Consider splitting "metadata loading + filtering + type-tree" from
    "rendering."** Today both live in one crate. The metadata side
    (`src/winmd/`, `src/index.rs`, `src/type_map.rs`, `src/references.rs`,
    `src/filter.rs`) is essentially a winmd parser specialized for
    windows-rs. The rendering side (`src/types/`, `src/tokens/`,
    `src/config/`) is the actual codegen. There is already a
    `crates/libs/metadata` crate (separate from bindgen's `winmd/`!), so
    there is some duplication worth investigating.

### 4.3 Things that look complicated but probably should stay

- The `Filter` grammar, including `Type::Method`, `Type::Property`,
  `Type::Event`, and `?Type`. The grammar is genuinely needed because
  in-repo argfiles use it heavily and the API surface of WinRT/Win32 is
  too coarse-grained for whole-type-only filters.
- The `derive` mini-DSL — small, isolated to `derive.rs` /
  `derive_writer.rs`, and used by external consumers.
- The bundled `Windows*.winmd` in `default/`. Distributing them inside the
  crate is the only thing that makes one-line `bindgen([...])` calls work
  without external dependencies. The size cost is real but the UX wins.
- The `WarningBuilder` channel — small and useful.

## 5. Fresh implementation vs. refactor

Once the "important options" are pared down to roughly:

```text
windows_bindgen::Config {
    inputs: [WinMd | "default"],
    outputs: SingleFile { path, preamble } | Package { root, index },
    filter: FilterSet,         // include/exclude + method-level + trait-only
    mode: Sys | Safe | SafeMinimal,
    derive: [String],
    implements: Pattern,       // wildcard set; empty = legacy default
    references: ReferenceTable, // data-driven, not code
    rustfmt: Option<String>,
    link_override: Option<String>,
}
```

…the case for a green-field rewrite is **weaker than it first looks**,
because the *hard* parts of bindgen are not the option matrix:

- **The winmd reader and the type-tree builder are correct and
  battle-tested.** Rewriting them risks regressions in WinRT/Win32 edge
  cases (generic instantiations, fixed buffers, agility, exclusive
  interfaces, factory/composable detection, `IReference` sugar, scoped vs
  flag enums, calling conventions, vararg, COM vs WinRT inheritance, …)
  that are encoded across thousands of test fixtures
  (`crates/tests/libs/bindgen/data/bindgen/**/expected.rs`).
- **The emitter's behaviour is locked down by those golden fixtures.** A
  rewrite would either have to reproduce them bit-for-bit (in which case
  it's a refactor) or accept large diffs (in which case every downstream
  consumer rebuilds and re-reviews their bindings).
- **The option-matrix complexity is concentrated in three places**:
  `config/names.rs::write_specific`, the cross-crate `references.insert`
  block in `lib.rs`, and the ~45 `if config.<flag>` branches in
  `types/*.rs`. Each is independently refactorable in place behind the
  golden tests.

So the more durable plan is:

- **(a) First pass — *breaking-but-mechanical* cleanup.** Drop unused
  flags (`--typedef`, `--sys-fn-ptrs`, `--sys-fn-extern` if no consumers
  depend on them; `--no-toml` if `--package` always emits the same file
  shape), merge `--implement` + `--implements` + `?` into one wildcard,
  collapse `--no-allow` / `--no-comment` into a preamble enum, make
  `--flat` implicit, data-drive the references table. Bindgen golden tests
  (`cargo test -p test_bindgen --test fixtures`) catch regressions
  per-mode.
- **(b) Second pass — *internal refactor behind the new surface*.**
  Replace `Config`'s flat booleans with a `Mode` enum + a `Preamble`
  struct; collapse `write_specific` to a single per-mode strategy; split
  `--package` writer into a thin caller of the single-file writer.
- **(c) Only then**, if the per-emitter `if mode` dispatch is still
  painful, consider the emitter-trait extraction in 4.2(11). That is the
  one change that is almost a rewrite, but it is still strictly local to
  `types/` and `tokens/`.

A *fresh* implementation only really pays off if the goal is to also
rethink the metadata layer (a new IR, a different filter language, a
typed config schema, an asynchronous incremental codegen pipeline, …).
Without those goals, the existing 8 kLOC is small enough and constrained
enough by golden tests that incremental simplification will land more
reliably and with less risk than a parallel implementation.

## 6. Concrete first cut, ordered by ROI

If you wanted to act on this, the suggested sequencing is:

1. Audit external usage of `--typedef`, `--sys-fn-ptrs`, `--sys-fn-extern`,
   and the non-`--flat` single-file path; delete the ones with no
   consumers.
2. Replace the `references.insert(0, …)` block in `lib.rs` with a static
   table; this alone shrinks `lib.rs` by ~20%.
3. Collapse `--no-allow` / `--no-comment` to a `--preamble` enum and make
   `--flat` the default for non-package output.
4. Unify `--implement` / `--implements` / `?` filter prefix into a single
   `--implements <pattern>` switch.
5. Introduce `enum Mode { Sys, Safe, SafeMinimal }` on `Config`, route all
   current `if config.sys` / `if config.minimal` through it, and remove
   the now-unused booleans.
6. Lift the `--package` writer into its own module (or sub-crate) that
   calls into the single-file pipeline namespace by namespace, and remove
   `config.package` from emitters.
7. (Optional / largest) split per-mode emission into trait impls
   (`SysEmitter` / `SafeEmitter`) so each `Type` variant's rules live in
   one place per mode.

Items 1–4 are roughly a day each and are mostly mechanical against the
existing fixture tests
(`cargo test -p test_bindgen --test fixtures`). Items 5–6 are a week or
two of careful refactoring. Item 7 is the only one that approaches
"partial rewrite."

The net effect would be roughly: ~14 boolean flags → 1 mode enum + 3–4
booleans, `Config` shrinks from 21 fields to ~10, the 45 branch sites
collapse toward a single mode-dispatch in each emitter, and `lib.rs`
drops by ~40%, all without losing any capability that any in-repo lib
(or, judging from how the existing flags compose, any external consumer
the repo's tests demonstrate) actually uses.
