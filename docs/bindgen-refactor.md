# windows-bindgen: implementation overhaul report

This report focuses on **simplifying and making the `windows-bindgen`
implementation more manageable**, not on simplifying its CLI / builder
surface. The CLI is treated here as essentially fixed; the question is
whether the ~8.1 kLOC under `crates/libs/bindgen/src/` can be reorganised
so that the next person who has to fix a codegen bug, add a new metadata
attribute, or teach the emitter a new sugar can do so quickly and
confidently.

The shape of `crates/libs/bindgen/src/` today:

```
lib.rs                      1241   builder, arg parsing, references seeding
types/mod.rs                1071   Type enum + write_name/default/abi/sig
types/cpp_method.rs          793   Win32 free-fn + COM method emitter
types/method.rs              713   WinRT method emitter
types/interface.rs           695   WinRT interface + _Impl emitter
types/cpp_interface.rs       516   COM interface + _Impl emitter
filter.rs                    579   include/exclude/method/?-prefix rules
tokens/{mod,runtime,        1050   hand-rolled quote!/ToTokens/TokenStream
   to_tokens,token_stream}
types/cpp_struct.rs          396   nested Win32 structs + handle types
types/class.rs               386   WinRT runtime classes
types/cpp_fn.rs              347   Win32 free function wrappers
tokens/runtime.rs            275   ext traits supporting `quote!`
config/{mod,names,cfg,       620   Config struct, package writer, cfg
  format,value,cpp_handle}        emission, name resolution
types/{cpp_const, delegate,
  struct, cpp_enum, enum,
  cpp_delegate}              ~800  remaining variant emitters
type_map / type_tree         ~210  dependency closure + namespace tree
references.rs                 111  cross-crate reference table
winmd/reader.rs               201  static-lifetime wrapper over
                                   windows-metadata::reader::TypeIndex
... small modules            ~700  filter parsing, derive DSL, IO, etc.
                            -----
                            ~8100  total
```

The complexity is not in the volume — every file individually is
reasonable — it is in the **redundant abstractions, parallel hierarchies,
and the way state flows through the emitter**. The rest of this document
catalogues those, ordered by the ROI of cleaning them up.

## 1. The hand-rolled `tokens/` crate is a thousand-line accident

`crates/libs/bindgen/src/tokens/` (446 + 275 + 149 + 180 = **1050 lines**)
reimplements the `quote` crate from scratch on top of a
`TokenStream(pub String)`:

- `tokens/mod.rs` is a 446-line `macro_rules!` reimplementation of
  `quote!` (`quote_each_token`, `quote_each_token_with_context`,
  `pounded_var_names`, repetition handling, etc.).
- `tokens/runtime.rs` is a 275-line port of `quote::__private::ext`
  (`HasIterator` / `ThereIsNoIteratorInRepetition`, the `BitOr` lattice,
  the `RepIteratorExt` / `RepToTokensExt` / `RepAsIteratorExt` family).
  Its module docstring literally describes the same trick the upstream
  `quote` crate documents.
- `tokens/token_stream.rs` makes `TokenStream` a transparent wrapper
  around `String`. `combine` pushes a single space and then concatenates.
- `tokens/to_tokens.rs` is the `ToTokens` boilerplate for that string
  type.

The workspace already declares `proc-macro2 = "1"` and `quote = "1"` in
the root `Cargo.toml` for other crates, but `windows-bindgen/Cargo.toml`
does **not** depend on them; instead it keeps the bespoke clone.

What this costs us:

- **No actual token tree.** The emitter is a string builder. Bugs like
  "missing space between tokens" are real (`combine` exists precisely to
  prepend a space). A real `TokenStream` would make these
  unrepresentable.
- **Output goes through `rustfmt` as its only sanity check.** If the
  emitter mis-balances a brace, the failure mode is a parse error in the
  generated file, not at the bindgen layer.
- **The `quote!` macro is the most expensive part of this crate to read.**
  Reviewers have to mentally diff against upstream `quote` to know which
  features are supported.

**Action:** delete `tokens/` entirely and depend on `proc-macro2` +
`quote` from crates.io. The replacement is mechanical (the call sites
already use `quote! { ... }` with the same `#var` / `#(...)*` syntax
upstream supports) and removes ~1 kLOC of code we own and test. The only
deliberate semantic difference today appears to be `TokenStream::combine`
inserting a space — that becomes free with a real token tree.

## 2. The `Type` enum is two enums in a trench coat

`types/mod.rs:31` declares one big enum with parallel COM/Win32-flavoured
and WinRT-flavoured variants for every category:

| Win32 / COM        | WinRT          | Shared (kind-neutral) |
|--------------------|----------------|-----------------------|
| `CppFn`            | —              |                       |
| `CppInterface`     | `Interface`    |                       |
| `CppStruct`        | `Struct`       |                       |
| `CppEnum`          | `Enum`         |                       |
| `CppDelegate`      | `Delegate`     |                       |
| `CppConst`         | —              |                       |
|                    | `Class`        |                       |
|                    |                | primitives, ptrs,     |
|                    |                | arrays, `Generic`, …  |

Each pair has its own `types/<name>.rs` file with substantially the same
shape (`type_name`, `write_name`, `write_cfg`, `write`, `dependencies`,
`PartialEq`/`Hash`/`Ord` boilerplate that always delegates to
`self.def`). Some of the duplication is genuinely warranted (a WinRT
method emission is very different from a Win32 free-fn emission), but a
lot of it is mechanical:

- `Hash`/`Ord`/`Eq` on every variant delegating to a `def: TypeDef` field.
- `type_name(&self) -> TypeName` returning `TypeName(def.namespace(),
  def.name())`.
- `combine(&self, deps, reader)` walking required interfaces / fields.

And in `types/mod.rs` we then pay for the duplication a second time:

- `write_name` is a 100-line `match` that dispatches by variant
  (`types/mod.rs:~330-480`).
- `write_default`, `write_abi`, `write_impl_name`, `runtime_signature`,
  `set_generics`, `type_name`, `write_no_deps`, `write`, `is_core`,
  `is_intrinsic` are all parallel giant matches.

What this costs us:

- Adding a new variant means touching ~10 matches in `types/mod.rs`,
  plus a new file, plus the `Reader` builder in `winmd/reader.rs`.
- Reviewers have to keep "what does Cpp* do that the WinRT counterpart
  doesn't" in their head while reading any emitter.
- The "kind-neutral" tail of `Type` (primitives + `Ptr*` + `Array*` +
  `PrimitiveOrEnum`) lives alongside the "real" variants, complicating
  ownership: those leaf types are values that can appear in signatures,
  not categories that get emitted.

**Action — two independently useful refactors:**

1. **Split `Type` into `Item` (emittable) and `Sig` (signature
   leaf).** `Item` would only hold `CppFn`, `Class`, `Interface`,
   `CppInterface`, `Struct`, `CppStruct`, `Enum`, `CppEnum`, `Delegate`,
   `CppDelegate`, `CppConst`. `Sig` would hold the primitives, pointer
   wrappers, array shapes, generic params, and remappable named handles
   (`PSTR`, `HRESULT`, etc.). Lifts the "leaves shouldn't appear in
   emission matches" invariant out of the comment in `is_intrinsic` and
   into the type system. Most of the giant `match` ladders in
   `types/mod.rs` lose a branch as a result.

2. **Replace the giant `match` ladders with a small `trait
   ItemEmitter`.** Each variant's `write_name` / `write_default` /
   `write_abi` / `write_impl_name` / `runtime_signature` lives once on
   the struct itself, not at the enum level. `Type::write_name` becomes a
   3-line `match self` that calls `ty.write_name(config)` for every
   variant — exactly mirroring what `write` already does
   (`types/mod.rs:878`). This is a mechanical refactor enforced by the
   compiler.

These two combined kill several hundred lines from `types/mod.rs` and
make each variant a self-contained unit.

## 3. `Config` is a 20-field bag threaded by reference everywhere

`config/mod.rs:11` defines:

```rust
pub struct Config<'a> {
    pub reader: &'a Reader,
    pub types: &'a TypeMap,
    pub references: &'a References,
    pub filter: &'a Filter,
    pub output: &'a str,
    pub flat: bool, pub no_allow: bool, pub no_comment: bool,
    pub no_deps: bool, pub no_toml: bool, pub package: bool,
    pub rustfmt: &'a str, pub sys: bool, pub minimal: bool,
    pub typedef: bool, pub sys_fn_ptrs: bool, pub sys_fn_extern: bool,
    pub implement: bool, pub implements: &'a Implements,
    pub specific_deps: bool, pub derive: &'a Derive,
    pub link: &'a str, pub warnings: &'a WarningBuilder,
    pub namespace: &'static str,
}
```

Every emitter — there are ~30 of them — takes `&Config` and inspects a
specific subset of its fields. This is the entire reason every
`types/*.rs` file ends up with `if config.sys`, `if config.minimal`,
`if config.package`, `if config.implement || …` scattered through it,
and why `config/names.rs::write_specific` (the namespace-to-path
resolver) crams six switches into ~20 lines.

What this costs us:

- **Behaviour is non-local.** Any flag in `Config` can be read by any
  emitter, so the only way to reason about "what does `--sys` do" is to
  grep for `config.sys`.
- **Three orthogonal concerns are conflated**: identity of inputs
  (`reader`, `types`, `references`, `filter`), output target
  (`output`, `package`, `flat`, `no_toml`, `rustfmt`), and emission mode
  (`sys`, `minimal`, `typedef`, `sys_fn_ptrs`, `sys_fn_extern`,
  `implement`, `implements`, `no_deps`, `specific_deps`, `no_allow`,
  `no_comment`, `link`, `derive`).

**Action — separate `Config` into three structs and pass each one only
where it's used:**

```text
Inputs<'a>    = { reader, types, references, filter, derive }
Output<'a>    = { output, package, flat, no_toml, rustfmt,
                  no_allow, no_comment, namespace }
Mode          = enum Sys | Safe | SafeMinimal
                + bitflags for the cross-cutting toggles that survive
                  (implement / implements / no_deps / specific_deps / link)
```

Inputs are immutable for the whole run. Output flows only to
`config/mod.rs` (the file/package writer). Mode flows into every
emitter. The crucial win is **the `Mode` enum** — it captures the
*intent* of `sys` / `minimal` / "plain safe" and lets each variant's
`ItemEmitter` impl decide its own branching, instead of every variant
reading raw booleans.

## 4. The `Reference` table is imperative configuration in `lib.rs`

`lib.rs:805-1032` (~230 lines) is one block of:

```rust
references.insert(0, ReferenceStage::new("windows_future",
    ReferenceStyle::Flat, "Windows.Foundation.Async*"));
references.insert(0, ReferenceStage::new("windows_future",
    ReferenceStyle::Flat, "Windows.Foundation.IAsync*"));
references.insert(0, ReferenceStage::new("windows_collections", ...));
...  // x ~30
```

This is *configuration*, hand-written as control flow, branching on
`!self.sys && !self.no_deps && reader.contains_key(<namespace>)`. Every
time a `windows-*` crate is split out, a contributor must add a stanza
here; every typo silently mis-routes references; the giant block is
hostile to review.

**Action:** replace with a static `[(crate, style, glob, requires_namespace)]`
table at the top of `references.rs`, e.g.:

```text
const DEFAULT_REFERENCES: &[Reference] = &[
    Reference { crate_: "windows_future", style: Flat,
        path: "Windows.Foundation.IAsync*",
        gated_on: Some("Windows.Foundation") },
    ...
];
```

`lib.rs` reduces to a single iteration over that table. The same change
makes adding a new split-crate a one-line edit and makes diffing the
*default* references trivial.

## 5. `winmd/reader.rs` duplicates `windows-metadata`'s `TypeIndex`

`crates/libs/bindgen/src/winmd/reader.rs` (~200 lines):

- Wraps `windows_metadata::reader::TypeIndex` in a `Reader`.
- `Box::leak`s the `TypeIndex` to extend its lifetime to `'static`
  (`reader.rs:27-28`), explicitly so that no lifetime parameter has to
  ride along with `Reader` / `Type` / `TypeMap`.
- Re-categorises every `TypeDef` into the local `Type` enum
  (`reader.rs:51-…`), duplicating the categorisation that
  `windows-metadata` already does (`TypeCategory::{Class, Delegate,
  Enum, Interface, Struct}`).

The unsafe `'static`-leak is load-bearing for almost every type in the
crate: `TypeName`, `MethodDef`, `TypeDef`, `Field` are all `'static`
references into the leaked `TypeIndex`.

What this costs us:

- **Running bindgen twice in the same process leaks the metadata
  arena.** For build-script consumers this is fine; for library use
  (and for our own `crates/tests/libs/bindgen/tests/fixtures.rs`, which
  invokes bindgen many times) it is a real, measurable leak.
- **There are now two sources of truth for the metadata model**:
  `windows-metadata` and `windows-bindgen::winmd`. Anyone fixing a
  metadata bug has to know which file to look at.

**Action — two-phase:**

1. **Short term**: thread an `Arena<'a>` lifetime through `Reader`,
   `Type`, and `TypeMap`. This is invasive but mechanical (the leak only
   exists to avoid the lifetime parameter; once `Reader` carries it,
   `'a` propagates by signature). Removes one `unsafe impl Send/Sync`
   and the leak.

2. **Longer term**: collapse `winmd/reader.rs` into a small wrapper that
   only adds bindgen-specific indexing (`nested` map,
   `Type::remap`-aware filtering) and reuses `windows-metadata`'s
   `TypeIndex` API directly. The 12-arm `match` over `TypeCategory` in
   `reader.rs:60-…` should not exist twice.

## 6. The `_Impl` story is decided in three different places

The "should this type emit a Rust trait that downstream `#[implement]`
crates can implement?" question is decided by:

- `Config::should_implement` (`config/mod.rs:52`) — reconciles the
  `--implement` and `--implements <list>` flags.
- The `?Ns.Type` prefix in `--filter`, parsed in `filter.rs` into a
  `trait_only: BTreeSet<…>` and consulted from
  `Filter::is_trait_only`.
- The per-emitter checks: `types/interface.rs`'s `kind` analysis,
  `types/cpp_interface.rs`'s `is_exclusive` check, and the base-class
  default-interface bound chain
  (`types/interface.rs::base_class_default_interfaces`).

What this costs us:

- Three places to look when an `_Impl` is emitted that shouldn't be
  (or vice-versa).
- Three places where regressions can land.

**Action:** centralise the decision into a single
`fn implement_policy(name: TypeName, kind: InterfaceKind, …) ->
ImplementMode` (where `ImplementMode = { None, TraitOnly, Full }`),
compute it once per type at `TypeMap` time, store it on the type, and
have emitters consult `self.implement_mode` instead of recomputing.
Emitters become smaller; the policy is testable in isolation.

## 7. Per-namespace `cfg` emission is duplicated in every emitter

Every type emitter has its own `write_cfg` method that follows the same
pattern:

```rust
fn write_cfg(&self, config: &Config) -> TokenStream {
    if !config.package { return quote! {}; }
    Cfg::new(&self.dependencies(config.reader), config).write(config, false)
}
```

`types/mod.rs` already has helpers (`write_simple_cfg`, `write_full_cfg`,
`types/mod.rs:1053-1071`) acknowledging the duplication, but every
emitter still calls its own `write_cfg` and threads `cfg` tokens through
its own emission.

What this costs us:

- The `cfg` annotation logic lives inside each emitter, so a bug there
  (e.g. mis-handling `Windows.Foundation` exclusion) needs ~12 fixes.
- The "is packaging enabled" guard appears 12+ times.

**Action:** lift `cfg` annotation out of emitters entirely. The package
writer (`config/mod.rs::write_package`) already wraps emission per
namespace; have it emit the `cfg` attribute by *post*-decorating the
generated `TokenStream` for each `Item`, using a single
`fn cfg_for(item: &Item, config) -> TokenStream`. Emitters never see
`config.package` again.

## 8. Token writing is one big string-build with stitched-in helpers

Beyond the bespoke `quote!` (issue #1), the actual emission path is
shaped like:

```text
Config::write
  ├── if package: write_package -> for each tree -> ty.write(&config)
  └── else:       write_file    -> ty.write(&config)
                                    ├── write_flat
                                    └── write_modules (recursive)
```

But the per-`ty.write()` path calls back into `config.write_core()`,
`config.write_specific(...)`, `config.write_namespace(...)`,
`config.write_strings()`, `config.write_result()`,
`config.write_generic_phantoms()`, etc. (`config/names.rs`). These
helpers each compute *paths* into other crates based on `sys` /
`flat` / `specific_deps` / `package` / `no_deps`. They're invoked
from dozens of places.

What this costs us:

- The path-resolution policy is split between
  `config/names.rs::write_specific` (6 booleans → 4 outcomes) and
  `config/names.rs::write_namespace` (`flat` vs nested + ancestor
  walk). Both are stringly-typed; both are read by every emitter.
- Renaming a sibling crate (e.g. `windows_core::IUnknown` →
  `windows_com::IUnknown`) requires understanding the matrix.

**Action:** model "where do we name this type from?" as a single
`enum PathOrigin { Core, Result, Strings, Sibling(&'static str),
LocalReference(&References), LocalNamespace }` computed once per
reference at `References::new` time, and use it as a typed input to a
single path renderer. The renderer's *only* output is a `Path`, no
booleans involved.

## 9. The argument expander + builder is duplicated state

`lib.rs:56-1241` declares both a `pub fn builder() -> Bindgen` returning
a builder struct with 23 fields, **and** a `pub fn bindgen<I, S>(args)`
free-function that parses string arguments (`expand_args`, lib.rs:1093)
into the same 23 fields by hand. Every flag has two parallel
implementations: one on `Bindgen::<flag>(&mut self)` and one on the
`ArgKind` state machine.

Adding a new flag means updating both code paths and the builder docs
table at the top of `lib.rs`.

**Action — keep both entry points, but make one the source of truth:**

- Define `Bindgen` as a plain (serde-derivable) settings struct.
- `bindgen(args)` becomes "parse `args` into `Bindgen`, then call
  `bindgen.write()`."
- The builder methods become thin setters.

This drops `expand_args`'s panic-on-error semantics in favour of a real
parse-result, and removes the dual-maintenance burden for every new
flag.

## 10. Other smaller knots worth picking apart

The above eight items are the bulk of the win. These are smaller pieces
that are still worth addressing once the big refactors land:

- **`for_each` in `config/mod.rs:180`** is a one-off
  `std::thread::scope` wrapper used only by `write_package`. Should
  either be inlined or replaced with `rayon` (which the workspace does
  not currently use; this is the only parallelism in the crate).
- **`derive` and `derive_writer`** are isolated mini-DSL files that are
  fine on their own, but `derive_writer::write_derive` exists only to
  format the result of `derive::parse` and could fold into one file.
- **`io.rs` (34 lines)** wraps `std::fs::read_to_string` with `panic!`
  on failure. It exists because the top-level API panics on user error
  anyway; once the builder returns `Result`, this can collapse to a
  `std::io::Result` re-export.
- **`warnings.rs` is a non-fatal side channel** that is fine, but it is
  threaded through `&Config` rather than returned by emitters.
  Returning it through the call graph (as the entry-point already does
  via `Warnings`) would let `&Config` drop one more field.
- **`signature.rs`, `param.rs`, `value.rs`, `guid.rs`, `libraries.rs`**
  are all small leaf modules — they don't need refactoring but they do
  expose internals via `pub use` at the crate root, which makes the
  module map look bigger than it is. Tighter `pub(crate)` would help.
- **`method_names.rs`** is a static map of overload-disambiguation
  names. Fine as data, but it lives at the crate root with no obvious
  owner; would sit better next to the method emitter.

## 11. Refactor in place, do not rewrite

The hard parts of `windows-bindgen` are not the structures listed
above — they are the **semantic decisions** encoded in the emitters
(WinRT method ABI rules, the `IReference<T>` sugar paths, the
exclusive-interface vs default-interface logic on classes, the
agility/factory/composable detection, fixed buffers, calling
conventions, vararg, `repr(C)` layout rules, COM vs WinRT inheritance,
`PrimitiveOrEnum` propagation through generics, …). These are pinned
down by the golden fixtures under
`crates/tests/libs/bindgen/data/bindgen/**/expected.rs` — thousands of
lines of "this exact input must produce this exact output."

A fresh implementation would have to reproduce all of that semantic
behaviour bit-for-bit (otherwise every downstream `bindings.rs` in the
workspace and in external consumers churns), or accept that downstream
review costs explode. The fixture suite makes incremental refactoring
**very safe**, but offers no leverage for a rewrite.

The recommended path is therefore strictly incremental, ordered roughly
by independence:

1. **Delete `tokens/`; depend on `proc-macro2` + `quote` from
   crates.io** (#1). Largest single LOC reduction (~1000 lines) and the
   refactor with the smallest semantic surface — golden tests catch any
   spacing/ordering regressions.
2. **Data-drive the references table** (#4). Local, low-risk, ~230
   lines of `lib.rs` collapse to ~30. Test by re-running the workspace
   bindings.
3. **Centralise `_Impl` policy** (#6). Three call sites become one; the
   golden tests catch any miscategorisation immediately.
4. **Per-mode `Mode` enum + split `Config`** (#3). Mechanical but
   touches every emitter; trivial to validate via fixtures.
5. **Lift `cfg` emission out of emitters** (#7). Local to
   `config/mod.rs` once mode is split.
6. **Typed `PathOrigin`** (#8). Replaces `config/names.rs` policy code.
7. **`ItemEmitter` trait + `Item`/`Sig` split** (#2). The largest
   internal restructuring; do this *after* steps 1, 3, 4, because each
   of those simplifies what the trait has to abstract over.
8. **Lifetime through `Reader`** (#5 phase 1). Removes the
   `Box::leak`/`unsafe impl Send`. Confined to `winmd/` + `type_map.rs`
   + a few signatures.
9. **Settings-struct entry point** (#9). Last because it changes the
   public API surface; do it after the internal cleanups so the new
   surface reflects the new internals.

After steps 1–4 alone the crate should drop to roughly 5.5–6 kLOC, with
most emitters losing their `if config.<flag>` ladders, and `lib.rs`
shrinking by ~25%. Steps 5–9 are gravy: they make the model coherent
but each is independently small.

## 12. Validation strategy throughout

For every step above:

- The golden fixtures
  (`cargo test -p test_bindgen --test fixtures`) cover the emitted
  output bit-for-bit. Any change that does not move the diff is by
  definition behaviour-preserving.
- Linux cross-compile (`rustup target add x86_64-pc-windows-gnu &&
  cargo check --all --target x86_64-pc-windows-gnu --tests --exclude
  windows_*_msvc/gnu/gnullvm`) catches consumer-side breakage in the
  workspace's own `bindings.rs` files that the Linux-native check
  hides.
- `cargo fmt --all` is enforced by CI.
- The package-writer path is only exercised by `crates/libs/sys` and
  `crates/libs/windows`; rebuilding both is the regression test for
  step 5.

No rewrite is justified: the existing code is correct, the fixtures
make refactoring strictly bounded, and every item above is a localised
change with mechanical validation.
