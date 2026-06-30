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
