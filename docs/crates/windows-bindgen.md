# windows-bindgen

> The code generator that turns Windows metadata (`.winmd`) into Rust bindings.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [docs.rs](https://docs.rs/windows-bindgen)
- 🚀 [Getting started](../../crates/libs/bindgen/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

`windows-bindgen` generates Rust bindings from Windows metadata. It powers the
`windows` and `windows-sys` crates and is also usable directly — typically from a
`build.rs` — to generate a minimal, project-specific set of bindings for just the
APIs you call. The generator ships with the standard Windows metadata bundled, so
you usually only need to choose an output file and a filter.

`windows-bindgen` is the back half of the metadata pipeline: it expects a
`.winmd` and produces Rust. When an API has no metadata yet — because it ships
only as a C/C++ header, or because you are authoring types by hand — use
[`windows-rdl`](windows-rdl.md) to manufacture a `.winmd` first, then point
`windows-bindgen` at it. `tool_webview` and `tool_reactor` chain the two crates
exactly this way.

## Getting started

Add `windows-bindgen` as a build dependency, and `windows-link` (or `windows-core`)
as the runtime dependency the generated code links against:

```toml
[dependencies]
windows-link = "0.2"

[build-dependencies]
windows-bindgen = "0.66"
```

Generate bindings from `build.rs`. There are two equivalent entry points — a
command-line-style `bindgen(args)` function and a fluent `Bindgen` builder:

```rust,no_run
// Command-line style: the same arguments the CLI accepts.
windows_bindgen::bindgen([
    "--out", "src/bindings.rs",
    "--flat",
    "--sys",
    "--filter", "GetTickCount",
]);
```

```rust,no_run
// Builder style: the same options as method calls.
windows_bindgen::Bindgen::new()
    .output("src/bindings.rs")
    .flat()
    .sys()
    .filter("GetTickCount")
    .write();
```

Then `include!` or `mod` the generated file and call into it:

```rust,ignore
mod bindings;

unsafe { println!("{}", bindings::GetTickCount()); }
```

## Filters

A filter selects which APIs end up in the output. The **specificity** of a rule
decides how much of a type you get, inspired by Rust's `use` declarations — name
something bare to get the whole thing; add braces to narrow it:

- a **namespace** (`Windows.Win32.System.Com`) — pulls in every type under it,
  each with its full surface,
- a **type**, by bare or fully-qualified name (`HWND`, `OSVERSIONINFOEXW`,
  `Windows.Win32.Foundation.HWND`) — the whole type: an interface projects all
  its methods, a struct all its fields, an enum all its variants, a class its
  default interface,
- a **name-only shell** (`Namespace.Type::{}`) — the type is projected so it can
  be named in signatures, but none of its own methods are (their vtable slots
  stay opaque). Use this for a dependency you only need to pass through,
- **specific methods** (`Namespace.Type::{Method1, Method2}`, or a single
  `Namespace.Type::Method`) — only the named methods are full; the rest of the
  type's methods become name-only. `Property` / `Event` names are sugar that
  expand to their accessor pairs (`get_`/`put_`, `add_`/`remove_`),
- a **class activation** (`Namespace.Class::CreateInstance`) — mark a class as
  activatable so its `new()` constructor is emitted. Activation is always
  explicit: a bare class projects its default interface but no constructor.

Prefix any rule with `!` to **exclude** rather than include. Pulling in a type
automatically pulls in everything it transitively requires — those dependency
types come in as shells — so you list only the entry points you call.

For anything beyond a handful of names, keep the arguments in a response file and
pass it with `--etc`. Lines starting with `//` are comments:

```text
--out crates/libs/version/src/bindings.rs
--flat --sys

--filter
    RtlGetVersion
    OSVERSIONINFOEXW
    VER_NT_WORKSTATION
```

```rust,no_run
windows_bindgen::bindgen(["--etc", "bindings.txt"]);
```

This is exactly how the in-repo crates are generated — `tool_bindings` runs
`bindgen(["--etc", "crates/tools/bindings/src/<crate>.txt"])` for each library.

## Choosing the output shape

Two independent choices control what the generated code looks like — its *style*
(how rich the bindings are) and its *layout* (how they're organized).

Style:

- **Default** — full-fidelity bindings: class wrappers, inherited-interface
  forwarders, ergonomic handle types, and free-function wrappers. This is what the
  `windows` crate ships.
- **`--sys` / `.sys()`** — raw, sys-style FFI: bare `extern` functions and plain
  structs, linked via `link!` macros. Add **`--extern` / `.extern_fns()`** to emit
  `extern { fn … }` blocks instead of `link!`. This is what `windows-sys` ships.
- **`--minimal` / `.minimal()`** — like the default style but drops per-class
  wrappers, inherited forwarders, handle ergonomics, and free-function wrappers.
  Ideal for small, hand-curated binding sets (used by `windows-canvas` and
  `windows-reactor`). Mutually exclusive with `--sys`.

WinRT event accessors (`add_*`/`remove_*` pairs) are always collapsed into a single
auto-revoking `Event` wrapper, regardless of style or layout — see *Event accessors*
below.

Layout:

- **Default** — one Rust module per metadata namespace.
- **`--flat` / `.flat()`** — a single flat list of items with no namespace modules.
- **`--package` / `.package()`** — one file per namespace plus a `Cargo.toml` with
  per-namespace features; this is how the published `windows`/`windows-sys` crates
  are produced. Mutually exclusive with `--flat`.

The two axes are independent, but only a few combinations are meaningful in practice.
Every in-repo crate pairs its style with `--flat` or `--package`; the default module
layout is for *external* consumers generating their own namespace-organized bindings.

| Style + layout       | Purpose                                              | Examples                                  |
| -------------------- | ---------------------------------------------------- | ----------------------------------------- |
| default + `--flat`   | Full-fidelity helper crate, one bindings file        | `windows-collections`, `windows-future`   |
| default + `--package`| The published umbrella crate                          | `windows`                                 |
| `--sys` + `--flat`   | Raw FFI helper crate, one bindings file              | `windows-result`, `windows-registry`, …   |
| `--sys` + `--package`| The published raw-FFI crate                           | `windows-sys`                             |
| `--minimal` + `--flat`| Small, hand-curated binding set                     | `windows-core`, `windows-canvas`, `windows-reactor` |
| *any* + *modules*    | Namespace-per-module output for external consumers   | *(not used in-repo)*                       |

`--minimal` + `--package` is never used (minimal targets small curated sets, packages
are the full API surface), and `--package` only ever pairs with default or `--sys` — the
two crates the repo publishes.

**Empty-module suppression (`--sys` + `--package`).** A namespace whose entire surface is COM
interfaces (e.g. a WinRT-interop header like `windowsuicompositioninterop`, or a pure-interface
Win32 header like `servprov`) emits *nothing* in `windows-sys`, since the raw-FFI style renders no
interfaces. In package mode such a namespace would otherwise leave an empty module file and a dead
Cargo feature, so `write_package` prunes it: the module declaration, its file, its Cargo feature,
and every reference to it from other features' dependency lists are all suppressed. Pruning is
recursive (a parent is pruned only when it and all its descendants are empty) and applies only to
`--sys`; the full `windows` crate emits interfaces, so no module there is empty.

**Future work — decouple module path from Cargo feature (`--package`).** Today `--package`
derives the module *and* the feature from the same winmd namespace, a rigid 1:1:1
(`package_writer::write_package` uses `tree.namespace` for both the directory and
`tree.feature()`). This is why `tool_package`'s `remap.rs` synthesises ~570 per-header
`Windows.<stem>` namespaces from the flat `Windows.Win32` winmd: it is the only way to get
per-header Cargo features, but it also forces ~570 flat lowercase modules under
`windows::`/`windows_sys::` (e.g. the direct2d sample imports from 15 of them: `windows::d2d`,
`windows::dxgi`, `windows::windef`, …). We should let package mode map an item's *feature*
(from its defining header) independently of its *module* (a coarser, friendlier grouping), so
the Win32 surface can live under a small number of modules while keeping granular build-time
gating. The per-item `#[cfg(feature=…)]` machinery already exists (`package_writer::Cfg`); the
coupled part is the module split. Note the original win32metadata sub-namespaces
(`Win32::Graphics::Direct2D`, `Win32::System::WinRT`, …) are *not* recoverable — the in-house
flat winmd dropped them — so the grouping must come from a curated header→module map (a coarser
sibling of `remap.rs`'s `FOLD_PREFIXES`), not from the original namespaces.

**In progress — flattening the Win32/Wdk module surface (`--package`).** A first step toward the
grouping above: rather than ~570 flat lowercase modules directly under `windows::`/`windows_sys::`,
the per-header stems are nested under a single `Win32` (and `Wdk`) module whose `mod.rs` privately
declares each stem (`#[cfg(feature = "<stem>")] mod <stem>;`) and re-exports it flat
(`pub use <stem>::*;`). The public path collapses to `windows::Win32::*` while per-header Cargo
features and file layout are preserved. Because every stem is glob-re-exported into one namespace,
any bare name defined by two stems becomes an ambiguous re-export (`E0659` when a consumer names it).

Auditing that flat namespace surfaced 21 cross-stem bare-name collisions. 20 were scraper artifacts —
loose macro constants duplicating a typed enumerator (`D3DFMT_*`, `OLEMISC_*`) — since removed by the
`windows-clang` const/enum-member dedup pass (see `docs/crates/windows-clang.md`). The **sole**
remaining flat-`Win32` collision is `Network`: two genuinely distinct enum members that happen to
share the bare name — `ConnectorType::Network = 5` (`devicetopology`) and
`SECURITY_LOGON_TYPE::Network = 3` (`ntsecapi`). Options: (a) scope enum members as associated
constants (`ConnectorType::Network`) instead of free `pub const Network` — eliminates this class of
collision entirely but is a large projection change; (b) place the two stems under different
sub-modules via the curated header→module grouping above so the bare name is never in one namespace;
(c) leave the ambiguous glob (compiles until a consumer names `windows::Win32::Network`). Beyond
`Network`, the only other names shared between the flat `Win32` surface and the crate root are the
core types `NTSTATUS`, `WIN32_ERROR`, and `RPC_STATUS`, resolved by routing Win32 references to the
`windows_core` canonicals (`types/mod.rs::remap`).

Other useful options:

- **`--in` / `.input(..)` / `.inputs(..)`** — add your own `.winmd` files or
  directories. Use the literal `"default"` to include the bundled Windows metadata.
- **`--derive` / `.derive(..)`** — derive extra traits on generated types.
- **`--implement` / `.implement(..)`** — emit `_Impl` scaffolding so you can
  implement WinRT interfaces in Rust (pass names/namespaces to scope it).
- **`--rustfmt` / `.rustfmt(..)`** — override the formatter used on the output.
- **`--dead-code` / `.dead_code()`** — emit `pub(crate)` instead of `pub` so the
  compiler flags any binding you generated but never used.

## Committing generated bindings

The `build.rs` approach above regenerates bindings on every build and adds
`windows-bindgen` as a build dependency of your crate. For a *published* crate you
usually want the opposite: commit `src/bindings.rs` as ordinary source and depend
only on the tiny [`windows-link`](windows-link.md) crate at runtime. Consumers then
build with no code generation, no metadata, and no `windows-bindgen` in their
dependency graph, and the exact bindings are visible in the published source and in
code review. This is how the crates in this repository that use `windows-bindgen`
are built.

The pattern has three parts.

**1. The published crate depends only on `windows-link`** and includes the
committed bindings:

```toml
# tickcount/Cargo.toml
[dependencies]
windows-link = "0.2"
```

```rust,ignore
// tickcount/src/lib.rs
mod bindings;

/// Milliseconds elapsed since the system was started.
pub fn get_tick_count() -> u64 {
    unsafe { bindings::GetTickCount64() }
}
```

**2. A separate, unpublished binary owns code generation.** Keep it as a workspace
member so it never becomes a dependency of the published crate:

```toml
# gen/Cargo.toml
[package]
name = "gen"
publish = false

[dependencies]
windows-bindgen = "0.66"
```

```rust,no_run
// gen/src/main.rs
windows_bindgen::bindgen([
    "--out", "tickcount/src/bindings.rs",
    "--flat",
    "--sys",
    "--filter", "GetTickCount64",
]);
```

`--out` is resolved relative to the current directory, so run the tool from the
workspace root:

```sh
cargo run -p gen
```

**3. A CI check keeps the committed bindings honest.** Regenerate, then fail if the
result differs from what's checked in:

```yaml
- run: cargo run -p gen
- run: git diff --exit-code
```

If someone edits the filter — or a new `windows-bindgen` changes its output — but
forgets to commit the regenerated file, `git diff --exit-code` returns non-zero and
the build fails. This repository uses exactly this arrangement: [`tool_bindings`](https://github.com/microsoft/windows-rs/tree/master/crates/tools/bindings)
regenerates each crate's `bindings.rs` from a `.txt` filter, and the
[`gen.yml`](https://github.com/microsoft/windows-rs/blob/master/.github/workflows/gen.yml)
workflow runs the tools and rejects any resulting diff.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-bindgen`**.

### How it's built

`windows-bindgen` is hand-written — it is the generator the other crates depend
on, not generated itself. It reads ECMA-335 metadata through
[`windows-metadata`](windows-metadata.md); the default `.winmd` inputs live in
`crates/libs/bindgen/default`. It is driven by `tool_bindings` (the per-crate
`.txt` filters in `crates/tools/bindings/src`) and by `tool_package` (which
produces the published `windows` and `windows-sys` crates).

### Testing

Verified by the dedicated test crates `test_bindgen`, `test_rdl`, and
`test_clang` (`crates/tests/libs/{bindgen,rdl,clang}`).

### Output-mode consolidation

The generated output is shaped by two axes — *style* (`Default` / `Sys` /
`Minimal`) and *layout* (modules / `--flat` / `--package`) — plus a handful of
booleans (`--dead-code`, `--implement`, `--derive`). Historically the per-style
divergences were expressed as ad-hoc `is_minimal()` / `is_sys()` checks scattered
across the type writers, which made it hard to see *which* policies actually
differ between modes and easy to introduce accidental inconsistencies. This is an
ongoing effort to remove divergence that exists for no good reason and to make the
remaining, intentional differences explicit.

**Current work — name the style policies (done).** The individual
code-generation policies that distinguish the styles are now named predicates on
`Style`/`Config` rather than inline `is_minimal()` checks, so call sites read by
intent:

- `Style::emit_class_methods` — emit per-class wrapper methods.
- `Style::emit_inherited_forwarders` — emit forwarders to inherited-interface methods.
- `Style::emit_iterable_into_iterator` — emit the `IntoIterator` bridge to an inherited `IIterable<T>`.
- `Style::minimal_string_input` / `minimal_string_return` — expose HSTRING params/returns as `&str` / `String`.
- `Config::emit_runtime_name` — emit the WinRT `NAME` runtime-name constant.

These are behavior-preserving: regenerating every in-repo crate produces zero
diff. `MethodNames::for_style` was the precedent for this style-keyed centralization.

**`--dead-code` visibility (centralized; broadening investigated and rejected).**
The `--dead-code` workaround emits `pub(crate)` instead of `pub` so the compiler
flags unused bindings ([rust-lang/rust#157961](https://github.com/rust-lang/rust/issues/157961)
means `pub` items in a non-public module are never linted). The duplicated
`if dead_code { pub(crate) } else { pub }` checks are now a single
`Config::item_vis()` helper, and the existing callable sites (class/WinRT/COM
methods, delegate `new`) route through it.

We investigated **broadening** the workaround to all nameable items (structs,
enums, consts, interfaces, …) to catch more dead code. This is **not viable as a
blanket policy**: generated items are frequently part of a curated crate's
*public surface*, and `pub(crate)` then fails to compile.

- `windows-reactor` re-exports ~22 generated WinUI enums/structs as its public API
  (`pub use bindings::Orientation;`, `Color`, `Thickness`, …). A `pub(crate)`
  definition cannot be re-exported (`E0364`/`E0365`).
- `windows-core`'s `imp` module is a *macro-export* surface: the exported
  `implement_decl!` / interface macros reference `$crate::imp::E_POINTER` and
  `E_NOINTERFACE`, so those generated consts must stay `pub` (`E0603`).

The generator cannot know which items a hand-written crate re-exports or
references from an exported macro, so it cannot safely demote them. The workaround
therefore stays scoped to callables (which are invoked, not re-exported as types) —
the original `#4609` scope. A future opt-in (e.g. a filter directive marking the
re-exported types to keep `pub`) could revisit this, but it is not worth the
complexity today.

**Event accessors (universal).** Every WinRT `add_X`/`remove_X` accessor pair is
collapsed into a single method `X<F>(handler) -> Result<EventRevoker>`: the closure
is taken directly (no `TypedEventHandler::new` wrapper), and the returned
[`EventRevoker`] auto-calls the paired `remove_X` slot on drop (call `.forget()` or
`.into_token()` to opt out). This was originally gated to non-`--package` builds —
the published `windows` crate kept the raw `add_X`/`remove_X` + token pattern — but
that gate has been removed so all layouts share one event-accessor shape. The `_Impl`
producer side is unaffected: implementing an event source still requires both
`add_X` and `remove_X`. Making this universal is a breaking change to the `windows`
crate's public event API (e.g. `widget.Click(&TypedEventHandler::new(|s, a| { …;
Ok(()) }))?` becomes `let _revoker = widget.Click(|s, a| { … })?;`), but it removes
the last layout-driven divergence in method emission.

[`EventRevoker`]: https://docs.rs/windows-core/latest/windows_core/struct.EventRevoker.html

**Name the sys policies (started).** As with the minimal predicates, the recurring
`is_sys()` divergences are becoming named `Style` predicates so the FFI policy reads
by intent. Done so far:

- `Style::derive_std_traits` — derive `Default`/`Debug`/`PartialEq` (on top of the
  always-emitted `Copy`/`Clone`); sys emits bare value types. Used by the WinRT
  value-type writers (`types/struct.rs`, `types/enum.rs`) and the Win32 `cpp_enum.rs`.
- `Style::emit_core_traits` — emit the `windows-core` trait block (type-kind,
  runtime signature, `NAME`); sys has no `windows-core` dependency so it omits them.
  Used by `types/struct.rs` and `types/enum.rs`.
- `Style::emit_bare_typedef` (`is_sys() || is_minimal()`) — emit handle structs and
  unscoped enums as a bare `pub type X = <underlying>` alias rather than a newtype
  wrapper. This is a non-obvious *cross-style* policy that was previously spelled out
  inline (with an explanatory comment) in `cpp_handle.rs`, `cpp_enum.rs`, and
  `cpp_const.rs`; naming it makes the relationship explicit and greppable.

Behavior-preserving: regenerating every in-repo crate produces zero diff. The rest of
the Win32 `cpp_*` family still uses inline `is_sys()` checks where the condition is
compound and site-specific (struct copyability/`Drop` wrapping, flag ops,
`link!`-vs-`extern` function emission, raw-pointer interface representation) — a
larger, separate follow-up.

**Deduplicate recurring layout/minimal idioms (done).** Beyond the style predicates,
two code blocks recurred *verbatim* across the type writers and are now single helpers:

- `Config::doc_hidden_in_package` — the `#[doc(hidden)]`-when-`--package` attribute on
  raw vtbl structs, previously copied identically into `types/interface.rs`,
  `types/cpp_interface.rs`, and `types/delegate.rs`.
- `Config::write_value_name_const` — the `RuntimeType::NAME` constant for value types
  (skipped in minimal mode), previously duplicated — comment and all — between
  `types/struct.rs` and `types/enum.rs`.

**Review conclusion.** A full sweep of the remaining `is_sys()` / `is_minimal()` /
`is_package()` / `is_flat()` branches confirms the *cleanly-recurring, identical*
divergences have now been named or deduplicated. What remains is intentional and
site-specific: structural layout dispatch (`Config::write`, `paths.rs`,
`package_writer.rs`), per-type-kind dependency-closure computation (the `--package`
`cfg` blocks in `class`/`interface`/`cpp_interface`, which differ by type kind), and
one-off behavioral differences (delegate invoke signatures, struct field
snake-casing, class `Deref` target, the compound `cpp_*` `is_sys()` sites noted above).
These are not duplication-for-no-reason; collapsing them further would obscure genuine
behavioral intent rather than clarify it.

**Follow-up — `Eq` derive unification (done).** A later pass found one more divergence
that existed for no good reason: value-type structs derived `Eq` (in addition to the
always-present `PartialEq`) only in `Minimal` mode. `Eq` is a purely additive marker
trait, gated on all fields being comparable (`Type::is_eq` — no floats, recursively),
so there was no reason to withhold it from the default `windows` crate. The
`is_minimal()` gate was dropped in `types/struct.rs` (WinRT structs) and the same
`is_eq` check added in `types/cpp_struct.rs` (Win32 structs), so both derive `Eq`
whenever eligible in every non-`sys` style. This also fixed a latent bug in
`Type::is_eq`, which lacked a `CppStruct` arm and so fell through to `_ => true` for
nested Win32 struct fields — wrongly reporting a nested float-containing struct as
`Eq`-eligible.

**Future work.**

- *Filter syntax simplification (done).* The filter grammar had accumulated
  redundant and unused syntax. Removed: the explicit-prefix accessor sugar
  (`get:Prop` / `set:Prop` / `add:Evt` / `remove:Evt`), which only ever appeared
  in a test; the recursive namespace glob `**`; and the namespace-level `*`
  (`Ns::*`). A bare namespace is already recursive — `match_type_name` matches it
  as a prefix — so `Windows`, `Windows::*`, and `Windows::**` all resolved to the
  identical rule. `tool_package`'s `windows.txt` used `Windows::**` while `sys.txt`
  used bare `Windows`; both are now bare `Windows`, and the corpus regenerates
  byte-for-byte unchanged. A later pass (below) removed the remaining
  member-glob (`Type::*`) and name-glob (`Prefix*`) syntax as well. What remains
  is one uniform, specificity-driven model: bare namespaces (recursive),
  fully-qualified types (bare = full), `Type::{}` name-only shells,
  `Type::member` / `Type::{a, b}` method selection, bare-name `Property`/`Event`
  accessor sugar, `Class::CreateInstance` activation, and `!` exclusions.
- *`--extern` is a deliberate escape hatch — keep it.* The `--extern`
  (`Style::Sys { extern_fns: true }`) option emits `extern` declarations instead of
  `link!` macros. It is unused in-repo because every windows-rs crate links via
  `raw-dylib`, but [#3828](https://github.com/microsoft/windows-rs/pull/3828) added it
  specifically *"if your build does not yet support `raw-dylib` and requires implicit
  linking with lib files"* — i.e. for external consumers on toolchains without
  `raw-dylib`. That PR also unified three copies of the function-signature codegen into
  one, so `--sys`, function-pointer types, `link!` macros, and `extern` blocks all share
  one ABI/FFI signature implementation. Removing `--extern` would regress that use case
  and is **not** recommended; "unused in-repo" is expected, not dead.
- *Default module layout.* Likewise unused in-repo (every caller passes `--flat` or
  `--package`), but it is the original namespace-to-module output and the natural shape
  an external consumer generating their own bindings would want. Retiring it is a
  breaking change to the published API with a plausible external audience — a maintainer
  decision, not an in-repo cleanup.
- *Finish naming the sys policies.* Extend the named-predicate treatment to the
  remaining compound `cpp_*` `is_sys()` sites (struct copyability/`Drop`, flag ops,
  `link!`-vs-`extern` function emission, raw-pointer interface representation).
- *Referenced-type inclusion is unified across styles (done).* The seed +
  dependency-closure type selection (`TypeClosure::build`, formerly `MinimalTypeMap`)
  runs for **every style** on precise filters — the dispatch at `lib.rs` is gated on
  `!has_broad_filter && !is_package()`. A filter no longer hand-lists the types its
  methods *reference*: the closure pulls them in transitively and back-fills the filter,
  so a requested method stays callable in every style instead of collapsing to a
  non-callable name-only slot. Dependency interfaces are pulled in *shallowly* (their own
  methods may stay name-only when they reach still-further-out types — see the
  `type_closure*` fixtures). Core types (`GUID`, `HRESULT`, `BOOL`, `PCWSTR`, `IUnknown`,
  …) are seeded even with an empty namespace, so a standalone `--sys` crate still emits its
  local definitions (gated on `uses_inline_core_types()`). `--minimal` no longer affects
  inclusion at all — it is a pure rendering style.

  Inclusion granularity is now **specificity-driven, like a Rust `use` declaration** — a
  bare mention gives you the whole thing; braces narrow it:

  - a **namespace** (`Windows`) → every type, full;
  - a **bare type** (`Ns.Type`) → full (a bare interface projects all its methods, seeded
    as `MethodSet::All`);
  - **`Ns.Type::{}`** → a name-only shell;
  - **`Ns.Type::m`** / **`Ns.Type::{a, b}`** → only the named methods;
  - a type reached only as a **dependency** of a seed → a shell (present and named so the
    API stays usable, own methods name-only);
  - **`Ns.Class::CreateInstance`** (or a composable factory method) → activation; a bare
    class projects its default interface but no constructor.

  The two type-map builders stay **deliberately separate** — `TypeClosure` (bottom-up
  closure, precise filters) and `TypeMap::filter` (top-down namespace scan, broad filters /
  `--package`) — the two ends of the specificity spectrum; `has_broad_filter` is a
  meaningful strategy selector (also consumed by event-only delegate detection). `--sys`,
  `--implement`, and `--package`/`--flat` remain orthogonal flags. Verified byte-for-byte
  across `windows`, `windows-sys`, every `tool_bindings` crate, and `tool_reactor`.

  Changelog (each step cleared a whole-corpus byte-for-byte regen):

  1. Introduced a per-type **role** (`TypeRole::Named(MethodSet)` vs `Shell`); routed
     `includes_method`'s no-method-filter fallback through `Filter::type_role`.
  2. Replaced `method_is_skipped`'s `is_minimal()` guard **and** the transitive
     `dependencies.included()` check with one universal **shell-aware direct-dependency**
     check (demote only when a *direct* signature type is absent entirely; a name-only shell
     satisfies it).
  3. Dropped `type_role`'s transitional `minimal` parameter for a `Filter::uses_closure`
     flag (a type with no method filter is `Named(All)` on a scan build, `Shell` on a
     closure build); `--minimal` stops affecting inclusion.
  4. Removed the last inclusion use of `minimal` (overload-name matching, unified to
     overload-exact for every style); renamed `MinimalTypeMap` → **`TypeClosure`**.
  5. Made the grammar read like Rust `use`: **bare = full, `Ns.Type::{}` = shell**. Removed
     the `::*` member-glob and `Prefix*` name-glob (parser rejects `*` with a migration
     hint); migrated the corpus off globs and pinned reactor/animation dependency interfaces
     to `::{}` to stay lean. The `type_closure*` / `filter_shell` fixtures cover both ends.
  6. Re-keyed the last inclusion-pruning guards on the build strategy, not the style flag.
     The `required_hierarchy!` / `interface_hierarchy!` emitters in `class.rs` / `interface.rs`
     prune references to types the closure dropped; they gated on `style.is_minimal()` but the
     concept is *closure build*, so they now gate on `filter.uses_closure`. This also covers
     the non-minimal closure builds (`collections`, `future` are `--flat` default style with
     precise filters), closing a latent trap where a pruned reference would only have been
     caught by a compile error. Whole corpus regenerates byte-for-byte identical.

  **Remaining (optional polish, not blocking):** route broad filters and `--package` through
  the same closure path (currently on `TypeMap::filter`; a no-op unification); move the three
  rendering cosmetics that are neither inclusion nor FFI-shape (snake_case struct fields
  `struct.rs`, the extra `Eq` derive, and the `&str`/`String` sugar) behind an explicitly
  named rendering flag if `--minimal` is ever split; and possibly rename the `--minimal` CLI
  flag to advertise that it is a rendering style, not an inclusion lever (deferred: it would
  churn ~16 `.txt` filters for a naming-only gain).
- *Preserve success `HRESULT` codes without the `-> HRESULT` ergonomic tax.* A void
  COM/Win32 method whose `HRESULT` can be a non-`S_OK` success (`S_FALSE`,
  `DXGI_STATUS_OCCLUDED`, …) trades off two options: `-> Result<()>` throws the
  success code away (`.ok()` maps every success to `()`), while `-> HRESULT` keeps it
  but pushes error handling back onto every caller (they must remember to `.ok()?`,
  losing the `?`-on-`Result` ergonomics that make the projection pleasant). Non-WinRT
  non-`[retval]` `HRESULT` methods currently take the second option — they project as
  raw `-> HRESULT` so the success code survives — which is the safer default but is
  more work at the call site. Explore a design that keeps a `Result`-shaped return
  *and* the success code — e.g. a success-carrying `Result` (an `Ok` payload that
  preserves the original `HRESULT`, so `?` still works but `S_FALSE`/`DXGI_STATUS_*`
  remain inspectable), or a projection that only emits `-> HRESULT` for the specific
  methods that can actually return multiple success values rather than for every void
  `HRESULT`. The goal is to stop trading success-code fidelity against caller
  ergonomics.

### Unifying the WinRT and Win32/COM type system fork

Above the output-mode axes sits a second, deeper fork: WinRT vs. non-WinRT
(`cpp_*`) code generation. There is exactly **one** classification point —
`winmd/reader.rs` keying on `TypeAttributes::WindowsRuntime` — and everything below
the `Type` enum (`types/mod.rs`) is shared substrate: `TypeMap`, `TypeTree`,
`Signature`, `Param`, `Dependencies`, and crucially `remap()`, which already
collapses many Win32 metadata types onto WinRT-equivalent primitives
(BSTR/HSTRING → `String`, `HRESULT`, `IUnknown`, `GUID`, `IInspectable` → `Object`,
`EventRegistrationToken` → `i64`, `D2D_MATRIX_3X2_F` → `Matrix3x2`, …).

The fork shows up as **parallel `Type` variants** — `Interface`/`CppInterface`,
`Enum`/`CppEnum`, `Struct`/`CppStruct`, `Delegate`/`CppDelegate`, plus COM-only
`CppFn`/`CppConst`/`CppHandle` — each forcing an N-arm match across every dispatch
site (`sort_key`, `write_name`, `write`, `combine`, …). The conceptual key is that
**WinRT is COM + `IInspectable` + metadata signatures**: a WinRT `Interface` is a
`CppInterface` plus `{ IInspectable base, forced-HRESULT return policy, generics,
SIGNATURE, activation }`. That argues for *configuration, not forking*.

**Essential divergence (must stay distinct):**

- *Return policy.* WinRT vtbl methods are always `HRESULT` and `method.rs` always
  wraps `Result`; COM preserves raw return types, which is why `cpp_method.rs`
  carries a `ReturnHint` enum (Query / ResultValue / ResultVoid / ReturnValue /
  ReturnStruct). This is the deepest real axis.
- *Generics + SIGNATURE/RuntimeType* — WinRT-only.
- *`Delegate` vs `CppDelegate`* — a WinRT delegate is a GUID'd COM interface with
  `Invoke`; a COM "delegate" is a bare `extern fn` pointer. Different ABI shapes;
  do not merge.
- *`CppFn`/`CppConst`/`CppHandle`* — no WinRT analog (free exports, freestanding
  constants, `DECLARE_HANDLE` newtypes).

**Incidental duplication (safe to unify — measured, smaller than first estimated).**
A first pass already landed the cheap, decisive part: `CppMethodOrName` was deleted in
favor of a generic `MethodOrName<M>` shared by both interface writers, and the
slot-skip and per-method `#[cfg]` policies were hoisted into the shared free functions
`method_is_skipped()` and `write_method_cfg()` (so "is this slot opaque?" and "does
this method need its own gate?" now live in exactly one place each). What remains is
narrower than the original "~600–800 line" guess; a close per-section reading of the
two `write()` bodies gives the realistic numbers:

- *Interface (`interface.rs` ~509 lines vs `cpp_interface.rs` ~353).* Only the vtable
  method-pointer emission and the `_Impl` field/impl/trait-method iteration are truly
  near-identical — about **90 lines** extractable into shared helpers
  (`emit_vtbl_methods`, `emit_impl_methods`, `emit_trait_methods`). The rest is
  genuinely platform-specific: WinRT-only RuntimeType/`SIGNATURE`, generics, inherited
  forwarders, `IntoIterator`/IIterable, RuntimeName (~145 lines); COM-only
  `has_unknown_base` branching, `ScopedHeap` non-`IUnknown` wrapping, and
  `Deref`-to-base (~135 lines).
- *Enum (`enum.rs` vs `cpp_enum.rs`).* ~**60%** unifiable behind a small parameterized
  writer (the bitwise-operator block and scalar-constant emission are effectively
  identical); the remainder is WinRT trait impls vs COM scoped/`#[repr]` attribute
  handling.
- *Struct (`struct.rs` vs `cpp_struct.rs`).* **Not** "only WinRT extras" — only ~15%
  overlaps. The COM side carries unions (explicit layout, `ManuallyDrop`), `DECLARE_HANDLE`
  newtypes, nested types, arch-specific `#[cfg]`, and bespoke `Default`/`Clone` derive
  logic that WinRT structs never have. Leave this fork alone.

**The proposals, in priority order:**

1. *Extract the interface vtable/`_Impl` iteration and the enum overlap into shared
   helpers (DONE — pure refactor, output-preserving).* Implemented. `interface.rs` now
   hosts a small `MethodItem` trait (`def()` / `dependencies()`, implemented for both
   `Method` and `CppMethod`) plus three shared emitters called from *both* interface
   writers:
   - `write_vtbl_methods` — the `_Vtbl` method-pointer fields (slot-name allocation, the
     per-method `#[cfg]` gate with its `#[cfg]`-out `usize` fallback, and the opaque
     name-only slot). The one real difference — WinRT appends `-> HRESULT` while COM's
     `write_abi` already carries the native return — is supplied by the caller as a
     closure.
   - `write_impl_field_methods` — the `name: name::<..>,` initializer entries (the
     opaque `name: 0,` fallback is shared; the turbofish args, which carry WinRT generics
     or branch on COM's `OFFSET`, come from the caller).
   - `write_impl_trait_methods` — the `fn name signature;` producer-trait entries (only
     the `write_impl_signature` arity differs).

   The per-method extern thunks (`impl_methods`) were intentionally left per-side: their
   generics, `this`-extraction (`OFFSET` deref vs `ScopedHeap`), and `write_upcall`
   shapes genuinely diverge, so a shared helper would be all parameters and no body.
   `enum.rs` likewise gained `write_enum_constants` (the filtered `pub const X = Self(v)`
   variants) and `write_enum_flags` (the `BitOr`/`BitAnd`/`*Assign`/`Not`/`contains`
   block); the WinRT and COM enum writers now call both, gating on their own
   (`u32`-underlying vs `FlagsAttribute`) guards. Net bindgen source ≈ −22 lines, but the
   point is single-sourcing: four near-identical blocks now live in one place each. Proven
   correct by regenerating `windows`/`windows-sys` (`tool_package`), `windows-reactor`,
   `windows-webview`, and the `tool_bindings` crates to a **zero-byte diff**; bindgen
   tests + clippy clean.

   Deliberately *not* attempted (essential forks — a unified writer would be mostly
   feature flags): `Struct`/`CppStruct` (COM unions/handles/nested types/arch layout),
   `Delegate`/`CppDelegate` (GUID'd interface vs bare `extern fn`), the COM-only
   `CppFn`/`CppConst`/`CppHandle`, and the method **return model** (next item).
2. *Return-model axis (studied — fork is mostly essential).* A close reading of the
   caller- and producer-side return handling in `method.rs` (WinRT) and
   `cpp_method.rs` (COM, driven by the `ReturnHint` enum) shows the two return models
   **overlap only in the `ResultVoid` / `ResultValue` shapes**, and that overlap is
   *already* shared:

   - The success-code folding lives in `windows-core`: `HRESULT::ok` / `map` /
     `and_then` all gate on `is_ok()` (`self.0 >= 0`, i.e. `SUCCEEDED`). The
     value-side mapping is centralized once in `Type::write_result_map`
     (`types/mod.rs`) and called by *both* writers. The void-side fold is `vcall.ok()`
     in both.
   - Everything else in each model is exclusive to one side and genuinely different:
     WinRT always returns `HRESULT` on the vtable with the logical return appended as a
     trailing out-param, plus WinRT-only axes (noexcept, WinRT arrays, `IReference<T>`
     sugar, minimal `&str`/`String`). COM preserves the **native** vtable return and
     adds COM-only `ReturnHint` shapes — `None` (raw return passes through),
     `ReturnValue` (non-`HRESULT` return + retval out-param), `ReturnStruct` (by-value
     struct via a hidden first out-param), and `Query` / `QueryOptional` (the
     `(REFIID, void**)` QI pattern folded into `Result<T>`).

   Because the strategies are ~80% disjoint, a single shared `ReturnStrategy` enum
   would be mostly non-overlapping variants and would not reduce real complexity;
   merging the two `write()` bodies is therefore **not** a worthwhile dedup — the fork
   is essential, and the part that *should* be shared already is.

   *Success-code (`S_FALSE`) finding.* `SUCCEEDED` codes are correctly treated as
   `Ok` on both paths (so `S_FALSE` is never an error), but projecting to
   `Result<()>` / `Result<T>` **discards the specific success code** — a method that
   returns `S_FALSE` meaningfully loses that signal. For the *void* case this is now
   solved on the COM path: a non-`[retval]` `HRESULT` method classifies as
   `ReturnHint::HResult` and projects raw `-> HRESULT`, so callers observe `S_FALSE` /
   `DXGI_STATUS_*` directly and `.ok()` when they only want a `Result<()>`. The
   remaining gap is a *shared* limitation, not a fork: it still applies to WinRT (a
   WinRT method can return `S_FALSE`, just more rarely, and always projects
   `Result`), and to the COM *value* case (`[retval]`), and there is no metadata
   distinguishing meaningful-success methods. `CppMethod::new` classifies a `[retval]`
   `HRESULT` return as `ResultValue`, so there is currently no way to opt a specific
   value-returning method out on either side. The worthwhile follow-up here is
   therefore a *feature*: an opt-in (e.g. a filter directive, since metadata can't
   express it) to preserve the raw `HRESULT` for those cases too — applicable
   uniformly to WinRT and COM. The deeper reason it can't be uniform is an
   **information-capacity mismatch**, not a codegen choice: a vtable method
   `HRESULT M([out] T*)` produces up
   to three distinct states — a failure code, a *success* code (`S_OK` vs `S_FALSE`),
   and the out-param value — but `Result<T>` has only two slots (`Ok(T)` | `Err(code)`).
   Collapsing always drops one: the `Ok` arm keeps the value and drops the
   success code; the `Err` arm keeps the code and drops the value. So `S_FALSE` cannot
   ride along "for free" in a `Result<T>` projection.

   *Void vs. value asymmetry.* The void case is solvable losslessly and is now handled
   by the raw `-> HRESULT` projection above; a richer alternative would be
   `Result<bool>` (`Ok(true)` = `S_OK`, `Ok(false)` = `S_FALSE`, `Err` = `FAILED`)
   instead of the lossy `ResultVoid` → `Result<()>`. The value case
   (`HRESULT M([out] T*)`) is genuinely lossy and needs a heavier shape
   (`Result<Option<T>>` with `S_FALSE` → `Ok(None)`, or raw `HRESULT` + caller reads
   the out-param).

   *Relation to the `Error::empty()` "success-but-empty" hack.* `windows-result`
   already faces the same fork for the success-with-no-value case: `Error.code` is a
   `NonZeroI32` (so `Result<(), Error>` stays niche-sized as `Option<Error>`, with the
   zero/`Ok` niche), which means an `Error` can never hold `S_OK` (0). `Error::empty()`
   therefore stores the sentinel `S_EMPTY_ERROR` (`b"S_OK"` as `i32`), and
   `Type::from_abi` for an interface maps *success HRESULT + null out-param* to
   `Err(Error::empty())` — i.e. a missing object becomes an error so `let x = foo()?;`
   stays ergonomic. Note this is a **policy choice** for the exact ABI situation an
   `S_FALSE`-aware value method cares about: same bits on the wire (success + null
   out-param), but `Error::empty()` projects it as `Err` while an `S_FALSE`-aware
   method would want `Ok(None)` / `Ok(false)`. That is precisely why `S_FALSE` support
   must be opt-in — it requests the *other* policy. The machinery already exists in
   one form: `ReturnHint::QueryOptional` models "optional out-param" by writing to a
   caller-provided `*mut Option<T>` and returning `Result<()>` (preserving both the
   HRESULT-as-`Result` and present/absent), so a generalized "optional/`S_FALSE`
   out-param" hint would follow an established shape rather than a new one.
3. *COM event transform + closure-ctor (prototyped, deliberately not kept).* COM
   already gets most WinRT ergonomics through genuinely *shared* code — property
   naming sugar (`get_X` → `X()`, `put_X` → `SetX()`), `_Impl` traits, IID-based
   `cast`, the `Param` conversion trait, HRESULT → `Result`. Two further WinRT
   ergonomics were prototyped for the COM path: collapsing `add_`/`remove_` event
   pairs into a single `X(handler) -> Result<EventRevoker>`, and giving
   delegate-shaped handler interfaces (`IUnknown` base, single `Invoke`) a
   closure-accepting `new()` mirroring `Delegate::write`. Both were reverted.

   The reason is the priority order: these mirrored the WinRT behavior by adding a
   **parallel COM codegen path** (`CppEvent` detection + event-collapse emit, plus
   `delegate_method`/`write_closure_ctor` and closure signature/upcall helpers) that
   *duplicated* `method.rs`/`delegate.rs` rather than sharing it — only the runtime
   types (`EventRevoker`, `DelegateBox`) were reused. That is more bindgen edge
   cases, not fewer, and the only consumer was `windows-webview`: the published
   `windows`/`windows-sys` crates have no `EventRegistrationToken`-based COM events
   (their one COM event, `ISpellChecker`, uses a `u32` cookie), and no other crate
   has delegate-shaped COM handlers. Mirroring-by-duplication to serve a single crate
   runs against the goal of *fewer* edge cases, so `windows-webview` keeps its own
   small `event_handler!`/`EventRegistration` glue instead — the complexity stays in
   the one crate that needs it. If these are revisited, the bar is a *single* shared
   event/delegate generator driven from both type systems, not a second copy.
4. *Extend remaining minimal-mode ergonomics to COM.* `IntoIterator` for COM
   enumerators (`IEnumXxx`'s Next/Skip/Reset/Clone) mirroring `IIterable` →
   `BufferedIterator`, and broader string in/out sugar, leveraging the existing
   `remap()` substrate.
