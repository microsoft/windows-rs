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
windows-bindgen = "0.63"
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

fn main() {
    unsafe { println!("{}", bindings::GetTickCount()); }
}
```

## Filters

A filter selects which APIs end up in the output. Each rule may be:

- a function or type name (`GetTickCount`, `OSVERSIONINFOEXW`),
- a namespace prefix (`Windows.Win32.System.Com`) that pulls in everything under it,
- a fully-qualified name (`Windows.Win32.Foundation.HWND`), or
- a method-level entry of the form `Namespace.Type::Method` — with `Property` /
  `Event` sugar for accessor pairs.

Prefix any rule with `!` to **exclude** rather than include. Pulling in a type
automatically pulls in everything it transitively requires, so you list only the
entry points you call.

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

Other useful options:

- **`--in` / `.input(..)` / `.inputs(..)`** — add your own `.winmd` files or
  directories. Use the literal `"default"` to include the bundled Windows metadata.
- **`--derive` / `.derive(..)`** — derive extra traits on generated types.
- **`--implement` / `.implement(..)`** — emit `_Impl` scaffolding so you can
  implement WinRT interfaces in Rust (pass names/namespaces to scope it).
- **`--rustfmt` / `.rustfmt(..)`** — override the formatter used on the output.
- **`--dead-code` / `.dead_code()`** — emit `pub(crate)` instead of `pub` so the
  compiler flags any binding you generated but never used.

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

**Future work.**

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

**Incidental duplication (safe to unify, ~600–800 lines):**
`MethodOrName`/`CppMethodOrName`, `get_methods`/`has_skipped_methods`, the vtable
struct writer, and the `_Impl` trait + `Vtbl::new()` thunk writer are near-identical
between `interface.rs` and `cpp_interface.rs`; the Enum/Struct generators differ only
in WinRT extras.

**The proposals, in priority order:**

1. *Unify the incidental duplication behind a model struct (do first; pure refactor,
   low risk).* Extract one interface writer parameterized by an `InterfaceModel
   { base: IUnknown | IInspectable, return_policy, runtime: Option<Signature>,
   generics }`; collapse `Interface`/`CppInterface`, `Enum`/`CppEnum`,
   `Struct`/`CppStruct`. The safety net is decisive: generated output is committed and
   CI fails on any diff, so the refactor is proven correct by regenerating to an empty
   diff. Keep `Delegate`/`CppDelegate` and the COM-only variants separate.
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
   returns `S_FALSE` meaningfully loses that signal. This is a *shared* limitation,
   not a fork: it applies to WinRT too (a WinRT method can return `S_FALSE`, just more
   rarely), and there is no metadata distinguishing meaningful-success methods. The
   only escape hatch today is to project the raw `HRESULT` (COM `ReturnHint::None`),
   but `CppMethod::new` always classifies an `HRESULT` return as `ResultVoid` /
   `ResultValue`, so there is currently no way to opt a specific method out on either
   side. The worthwhile follow-up here is therefore a *feature*, not a dedup: an
   opt-in (e.g. a filter directive, since metadata can't express it) to preserve the
   raw `HRESULT` so callers can observe `S_FALSE` — applicable uniformly to WinRT and
   COM. The deeper reason it can't be uniform is an **information-capacity
   mismatch**, not a codegen choice: a vtable method `HRESULT M([out] T*)` produces up
   to three distinct states — a failure code, a *success* code (`S_OK` vs `S_FALSE`),
   and the out-param value — but `Result<T>` has only two slots (`Ok(T)` | `Err(code)`).
   Collapsing always drops one: the `Ok` arm keeps the value and drops the
   success code; the `Err` arm keeps the code and drops the value. So `S_FALSE` cannot
   ride along "for free" in a `Result<T>` projection.

   *Void vs. value asymmetry.* The void case is solvable losslessly: `HRESULT M()`
   with meaningful `S_FALSE` has no out-param, so the free slot can carry the success
   code — project it as `Result<bool>` (`Ok(true)` = `S_OK`, `Ok(false)` = `S_FALSE`,
   `Err` = `FAILED`) instead of the lossy `ResultVoid` → `Result<()>`. The value case
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
3. *Apply the event transform to COM (implemented for i64-token events).* COM already
   gets most WinRT ergonomics through shared code — property naming sugar (`get_X` → `X()`,
   `put_X` → `SetX()`), `_Impl` traits, IID-based `cast`, the `Param` conversion
   trait, HRESULT → `Result`. The one big missing piece was the event transform:
   `method.rs` detects `add_`/`remove_` pairs and emits a single
   `X(handler) -> Result<EventRevoker>`, and `windows_core::EventRevoker` is already
   COM-agnostic (it stores an `IUnknown`, an `i64` token, and a
   `remove: fn(*mut c_void, i64) -> HRESULT`). This is now mirrored in `cpp_method.rs`.

   *Structural equivalence (confirmed).* The COM `add_`/`remove_` shape is identical
   to the WinRT one. A WebView2 accessor pair generates
   `add_X(handler: P0) -> Result<i64>` (HRESULT with the `EventRegistrationToken`
   token as an `[out, retval]`) plus `remove_X(token: i64) -> Result<()>` — the same
   token-in/token-out vtable layout WinRT uses, and exactly the `(source, i64,
   fn(*mut c_void, i64) -> HRESULT)` triple `EventRevoker::new` already consumes. The
   *only* shape difference is the handler argument: WinRT's `add_` takes a
   `Type::Delegate`, COM's takes a handler *interface* (e.g.
   `ICoreWebView2NavigationStartingEventHandler`).

   *What COM gets and does not get.* The realized win is the **revoker collapse**:
   `cpp_method.rs` detects the pair, suppresses `remove_X` in the caller surface
   (the vtable struct and `_Impl` trait still carry every slot), and emits
   `X(handler: P0) -> Result<EventRevoker>` reusing `windows_core::EventRevoker`. This
   removes the per-event token/`remove` plumbing entirely. What it does *not* get is
   WinRT's closure sugar: COM consumers still pass a COM object implementing the
   handler interface, because the closure-to-handler adapter (`event_handler!`) and the
   args-wrapper newtypes (`NavigationStartingArgs`, etc.) encode app-level semantics
   bindgen cannot synthesize (the handler `Invoke` signature varies — `sender`+`args`,
   `errorcode`+`result`, read-from-sender). So `event_handler!` and the wrapper types
   stay hand-written; only the registration plumbing is absorbed.

   *Detection (structural, post-remap).* Win32 metadata sets no `SpecialName` on these
   methods, and the strongest signal — the `EventRegistrationToken` out-param type — is
   remapped to `i64` early (in `Type::to_type`, before `CppMethod::new` runs), so the
   classifier only sees `i64`. The implementation uses a purely structural detector
   (`is_event_add_shape`): `add_` prefix + HRESULT return + two params + `[retval]` +
   first param is an interface (the handler) + second derefs to `i64` (the token),
   paired with a sibling `remove_<suffix>(i64)` that itself satisfies the add shape.
   Keeping the early remap (rather than sniffing the pre-remap type name) avoids
   disrupting WinRT `_Impl`/vtable output; the structural shape is deterministic in
   practice.

   *Blast radius (measured).* Regenerating every binding showed the transform touches
   **only** `windows-webview` (its `add_`/`remove_` pairs collapse; `bindings.rs`
   −246/+172). The published `windows`/`windows-sys` crates are **unchanged**: their one
   COM `add_`/`remove_` event (`ISpellChecker::add_SpellCheckerChanged`) uses a `u32`
   *cookie*, not an `EventRegistrationToken`/`i64` token, so its `remove` slot
   `fn(…, u32)` is incompatible with `EventRevoker`'s `i64` and the detector correctly
   skips it. The `u32`-cookie idiom was deliberately *not* handled (it is not "just like
   WinRT"). canvas/time/numerics/reactor have no such events. So universal rollout is
   not a breaking change to the `windows` crate today; only `--minimal` COM crates with
   `EventRegistrationToken` events (webview, future DirectComposition-style APIs) are
   affected.

   *Consumer effect (windows-webview).* The collapse let the crate drop its entire
   hand-written subscription layer: the `EventRegistration` RAII type is deleted and
   every `on_*` returns `windows_core::EventRevoker` directly. The `subscription!` macro
   shrank to "build the `handler.rs` adapter, call `self.0.X(&handler)`". The one compound
   case — `on_web_resource_requested`, which also manages a URI filter — moved the filter
   lifetime into the handler adapter (register on create, remove in its `Drop`), so it too
   returns a plain `EventRevoker` and a single revoke tears down both handler and filter.

   *Next step: generating the closure adapter (the harder half).* The revoker collapse
   removed the registration plumbing but left the **closure adapter** (`event_handler!`
   in `handler.rs`) hand-written: the per-event `implement_decl!` struct holding a
   `Box<dyn FnMut>` whose `Invoke` forwards to the closure. The question is whether
   bindgen can synthesize that too. Surveying every WebView2 handler `Invoke` signature
   shows they fall into exactly **two metadata-recognizable families**: (a) *event
   handlers*, uniformly `Invoke(sender: Ref<S>, args: Ref<A>) -> HRESULT` (`S` is the
   raising object; `A` is a real args interface or `IUnknown`), and (b) *completion
   handlers*, `Invoke(errorcode: HRESULT, result: Ref<T>) -> HRESULT` (the async-result
   shape). Both are single-method interfaces, so a delegate-shaped detector (one
   `Invoke` method, `IUnknown` base) recognizes them — the same structural-detection
   approach used for the add/remove pair.

   *Why it is generatable.* WinRT delegates already get a closure constructor:
   `Delegate::write` emits `new<F>(invoke: F) -> Self` backed by the reusable
   `windows_core::imp::DelegateBox<I, F>` runtime helper, which supplies the shared
   `QueryInterface`/`AddRef`/`Release` and boxes the closure. COM handler interfaces are
   modeled as plain `CppInterface`s, so they never get it — but `DelegateBox` is generic
   over any `Interface`, and crucially its `QueryInterface` claims `IAgileObject`/
   `IMarshal` *exactly as `implement_decl!` does*, so reusing it for a COM handler is a
   faithful drop-in, not an agility change. Under `--minimal` the generated `new` is
   `F: Fn + 'static` (no `Send`), matching webview's single-threaded UI usage.

   *The passthrough-vs-projection boundary (the wall).* What bindgen can synthesize
   *correctly* from metadata is only the **raw passthrough** closure — `Fn(Ref<S>,
   Ref<A>) -> Result<()>` — handing the user the sender and args interfaces verbatim.
   That is exactly what WinRT auto-generates, and it works there because WinRT events are
   *uniformly* args-bearing (`TypedEventHandler<TSender, TArgs>`), so passthrough is
   always meaningful. WebView2 breaks the uniformity: several events carry **no args
   interface** (`IUnknown`), so the payload must be recovered from the *sender* by
   reading a specific property — `DocumentTitleChanged` → `sender.DocumentTitle()`
   (`String`), `ContainsFullScreenElementChanged` → `sender.ContainsFullScreenElement()`
   (`bool`) — or the *sender itself* is the payload (`DownloadOperation`'s
   `StateChanged`/`BytesReceivedChanged`). Metadata gives no signal about which property
   to read or that the sender is the value, so those projections are irreducibly
   hand-written. The ergonomic args newtypes (`NavigationStartingArgs`, …) are likewise
   a style choice bindgen can't infer.

   *So the realistic landing is two-tier.* (1) bindgen generates the **adapter
   plumbing** — a `new<F: Fn(sender, args) -> Result<()>>` closure constructor for every
   delegate-shaped handler interface, eliminating the `implement_decl!` boilerplate; (2)
   webview keeps only thin per-event **shaping** — a one-line closure for the args-less /
   sender-projection events, and nothing at all for events happy to expose the raw args
   interface. `event_handler!` shrinks from ~12 full adapter structs to a handful of
   projection shims plus whatever wrapper newtypes the crate chooses to keep.

   *Attempt + measured effectiveness (implemented).* Prototyped end to end. The
   codegen lives in `cpp_interface.rs`: `delegate_method()` detects a delegate-shaped
   handler (`--minimal`, base is exactly `[IUnknown]`, a single method named `Invoke`
   that classifies as `ReturnHint::ResultVoid`) and `write_closure_ctor()` emits the
   closure constructor — `IXHandler::new<F: Fn(..) + 'static>(invoke: F)` plus the
   `XBox` struct / `VTABLE` / `Invoke` thunk — reusing `windows_core::imp::DelegateBox`,
   exactly mirroring `Delegate::write`'s minimal path. Two small `CppMethod` helpers
   (`write_closure_fn_signature` / `write_closure_upcall`) supply the `Fn(..)` argument
   list and the upcall, sharing the existing produce-type/invoke-arg logic. For a
   delegate-shaped interface the `_Impl` producer trait is suppressed (the closure box is
   the better producer), but the interface stays in `--implement` so its `Invoke` method
   survives dead-code elimination. The transform is inert outside this exact shape:
   regenerating `windows`/`windows-sys` (`tool_package`), `windows-reactor`, and the
   `tool_bindings` crates (canvas/time/numerics/…) produced a zero-byte diff.

   On the consumer side every WebView2 handler interface — 24 of them, event and
   completion alike — now gets a generated `new`, so `windows-webview` deleted all of its
   `implement_decl!`-based adapter structs and `_Impl` impls. `handler.rs` shrank from a
   struct + `implement_decl!` + `create` + `Invoke` impl per handler to a thin `create`
   that bridges the public `FnMut`/`FnOnce` closure onto the generated `Fn` constructor
   (a `RefCell` for the repeatable events, a `Cell<Option<_>>` for the one-shot
   completion handlers) and projects the sender/args as before. Even the compound
   `WebResourceRequested` handler — captured environment plus a URI filter removed on
   release — moved onto the generated `new`, with the filter lifetime carried by a
   `FilterGuard` captured in the closure (its `Drop` runs when the box is released). The
   `event_handler!` macro survives only as a ~10-line projection-shim generator; the
   sender-read / sender-as-payload / args-newtype cases stay hand-written exactly as the
   "projection wall" predicted. `test_webview` passes 27/27 (including the three
   `Protocol_*` filter-lifetime fixtures).

   *Honest tradeoff.* This is a **unification** win, not a line-count win. Hand-written
   webview glue dropped ~100 lines and lost its entire COM-implementing layer, but the
   generated `bindings.rs` grew ~456 lines: the `DelegateBox` box (struct + full
   QI/AddRef/Release vtable + `Invoke` thunk + `new`, with the `F: Fn(..) + 'static`
   bound repeated and rustfmt-expanded) is more verbose per handler than the `_Impl`
   trait it replaces. That is the same cost WinRT already pays for every delegate, so the
   COM path now simply conforms to it. The payoff is that closure ergonomics for COM
   handlers are no longer hand-rolled per crate — any `--minimal` COM handler interface
   gets them for free, generated identically to WinRT delegates.
4. *Extend remaining minimal-mode ergonomics to COM.* `IntoIterator` for COM
   enumerators (`IEnumXxx`'s Next/Skip/Reset/Clone) mirroring `IIterable` →
   `BufferedIterator`, and broader string in/out sugar, leveraging the existing
   `remap()` substrate.
