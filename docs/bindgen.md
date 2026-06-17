# `windows-bindgen`

`windows-bindgen` generates Rust bindings from Windows metadata (.winmd files).
It powers the `windows` and `windows-sys` crates and can generate standalone
bindings for any Windows API.

## Quick start

### Raw Win32 bindings (zero sugar, minimal dependencies)

```rust
// build.rs
fn main() {
    windows_bindgen::bindgen([
        "--out", "src/bindings.rs",
        "--sys",
        "--flat",
        "--filter",
            "Windows.Win32.UI.WindowsAndMessaging.LoadCursorW",
    ]).unwrap();
}
```

Only dependency: `windows-link` (for the `link!` macro). Core types like
`PCWSTR`, `HRESULT`, and `GUID` are emitted as inline definitions — no
`windows-sys` or `windows-core` crate required.

### COM / WinRT bindings

```rust
// build.rs
fn main() {
    windows_bindgen::bindgen([
        "--out", "src/bindings.rs",
        "--flat",
        "--filter",
            "Windows.Win32.Graphics.Direct2D.ID2D1Factory1",
            "Windows.Win32.Graphics.Dxgi.IDXGIFactory2",
    ]).unwrap();
}
```

Default mode depends on `windows-core` and provides full COM/WinRT support
with smart pointers, `Interface` trait, vtable wrappers, etc.

### With an args file

```rust
// build.rs
fn main() {
    windows_bindgen::bindgen(["--etc", "bindings.txt"]).unwrap();
}
```

```text
// bindings.txt
--out src/bindings.rs
--flat

--filter
    Windows.Win32.UI.WindowsAndMessaging.CreateWindowExW
    Windows.Win32.UI.WindowsAndMessaging.DefWindowProcW
    Windows.Win32.UI.WindowsAndMessaging.RegisterClassW
```

## Code-style modes

Bindgen has three code-style tiers, each building on the previous:

| Mode | Flag | Dependencies | Use case |
|------|------|-------------|----------|
| **sys** | `--sys` | `windows-link` only | Raw C-style FFI, smallest footprint |
| **minimal** | `--minimal` | `windows-core` | COM/WinRT with lean vtables (unused methods become `usize` slots) |
| **default** | *(none)* | `windows-core` | Full COM/WinRT sugar (class wrappers, inherited method forwarders, etc.) |

### `--sys` mode

Produces the rawest possible bindings: `link!` macros (or `extern` blocks with
`--extern`), plain struct/enum/const definitions, and inline type aliases for
core types (`PCWSTR = *const u16`, `HRESULT = i32`, etc.).

No dependency on `windows-sys`, `windows-core`, or any other `windows-*` crate
beyond `windows-link`. COM interfaces become just vtable structs and GUIDs —
interface pointers are `*mut c_void`. This is the right choice when you want
to avoid the type system entirely and control your own type definitions.

### `--minimal` mode

Preserves COM vtable layout, `define_interface!`, `_Impl` traits, and
`RuntimeType` but demotes methods you don't call to `usize` vtable slots.
Uses bottom-up type closure (`MinimalTypeMap`) — only types reachable from
requested method signatures are emitted. Depends on `windows-core` (and
transitively `windows-result`, `windows-strings`). Used internally by
`windows-reactor` and `windows-canvas` for optimized build times.

Key differences from default:
- Bottom-up type closure (only types reachable from requested methods)
- Methods demoted by default unless explicitly requested
- No inherited method forwarders (callers use `cast()`)
- Vtable truncation (trailing unused slots removed)
- Handles emitted as raw type aliases (no newtype wrappers)
- No `#[doc(hidden)]` on vtables

### Default mode

Full-fidelity bindings with class wrappers, inherited interface method
forwarders, and all the convenience sugar. Uses top-down type inclusion
(`TypeMap::filter`) — all types matching the namespace/type filter are
included. Same `windows-core` dependency as minimal. This is what the
published `windows` crate uses.

Key differences from minimal:
- Top-down type inclusion (all matching types included)
- All methods included by default (skipped only if deps missing)
- Inherited methods forwarded onto derived interfaces
- Handle newtypes with `is_invalid()` and `Default`
- Struct `Debug`/`PartialEq` derives
- `IntoIterator` for `IIterable` types
- `#[doc(hidden)]` on vtable structs

## Options reference

| Option | Description |
|--------|-------------|
| `--in <path>` | Input .winmd file(s) (default: bundled Windows metadata) |
| `--out <path>` | Output file path |
| `--filter <types>` | Which types/methods to include (see [Filter syntax](#filter-syntax)) |
| `--flat` | Emit all types in a single module (no namespace hierarchy) |
| `--sys` | Generate raw C-style bindings (no `windows-core` dependency) |
| `--extern` | Emit `extern` blocks instead of `link!` macros (only with `--sys`) |
| `--minimal` | Lean COM/WinRT bindings with method-level filtering |
| `--implement <ifaces>` | Generate `_Impl` traits for listed COM/WinRT interfaces |
| `--derive <traits>` | Additional derives on generated types |
| `--link <macro>` | Override the default `windows-link` link macro |
| `--rustfmt` | Format output with rustfmt |
| `--reference <path>` | Reference an external binding crate to avoid re-emitting shared types |
| `--etc <file>` | Read additional arguments from a response file |

### Filter syntax

Filters accept Rust-style `::` paths that map to Windows metadata names:

| Form | Effect |
|------|--------|
| `Windows::Win32::Graphics::Direct2D::ID2D1Factory` | Include a type and its dependencies |
| `!Windows::Win32::Storage::FileSystem::WIN32_FIND_DATAW` | Exclude a type |
| `Windows::Foundation::{DateTime, TimeSpan}` | Include multiple types with brace grouping |
| `Windows::UI::Xaml::Controls::TextBlock::put_Text` | Include a specific method (with `--minimal`) |
| `Windows::UI::Xaml::Controls::TextBlock::Property.FontSize` | Include getter/setter pair |
| `Windows::UI::Xaml::UIElement::Event.PointerPressed` | Include event add/remove pair |
| `Windows::UI::Xaml::Controls::TextBlock::*` | Include all methods on a type |

Dotted namespace paths (`Windows.Win32.Graphics.Direct2D.ID2D1Factory`) are
also accepted for backward compatibility.

With `--minimal`, unlisted methods become `usize` vtable slots — the layout is
preserved but no code is generated for methods you don't call. Dependencies
(parameter/return types) are pulled in automatically.

---

## Design notes

### Code-style tiers

`--sys`, `--minimal`, and default are three points on a single axis (the `Style`
enum internally). They are mutually exclusive. Each tier adds more sugar and
expressiveness on top of the previous one:

```text
--sys          raw FFI          depends on: windows-link
--minimal      lean COM/WinRT   depends on: windows-core
(default)      full COM/WinRT   depends on: windows-core
```

The long-term goal is to share optimizations (method-level filtering, dead-code
elimination, vtable slot demotion) across tiers so that each mode produces
optimal output by default, and the choice between tiers is purely about which
level of abstraction you want — not about which optimizations you get.

### Type closure: MinimalTypeMap vs TypeMap::filter

The two non-sys modes use different strategies for determining which types to
include in the output:

**Default mode (`TypeMap::filter`)** — top-down. Iterates all types in the
metadata that match the namespace/type filter rules and includes them. For each
included type, computes its full dependency set (`Dependencies::combine`) and
checks that none of those dependencies are explicitly excluded. Methods on
interfaces are then individually checked: if a method's parameter/return types
aren't in the final `types` map (or references), it gets demoted to a `usize`
vtable slot with a warning.

**Minimal mode (`MinimalTypeMap::build`)** — bottom-up. Starts from explicitly
requested methods (via `Interface::Method` filter syntax) and types, then walks
method signatures recursively to discover *only* the types that are actually
referenced. Types reachable through signatures get pulled in; interfaces
encountered as dependencies get their struct/IID but their methods are *not*
recursively walked (avoiding an explosion of transitive types). After building
the closure, method-level inclusion is determined by `includes_method` — methods
not in the filter's `requested_interfaces` map are demoted by default.

**Why two approaches?** Top-down is appropriate when the filter is broad (entire
namespaces or the full `Windows::**` for the published crate) — walking every
method signature in the entire Windows API would be prohibitively expensive.
Bottom-up is appropriate when the filter is specific (dozens/hundreds of known
methods) and you want zero extraneous code.

**Future direction:** The bottom-up approach produces strictly better output for
targeted use cases. If both modes used `MinimalTypeMap`, default-mode users
would benefit from: no spurious method skipping (no dependency-check heuristic
needed), no transitive type bloat, and deterministic output regardless of what
other types happen to be in the filter. The blocker is performance — running
`MinimalTypeMap` over `Windows::**` would need to walk ~100K+ method signatures.
A possible hybrid: use `MinimalTypeMap` when the filter is specific (method-level
entries, individual types) and fall back to `TypeMap::filter` for broad namespace
filters.

### Method-level filtering

Method-level filtering (the `Interface::Method` syntax) already works in both
default and minimal modes, but with different defaults:

| | Default mode | Minimal mode |
|---|---|---|
| Type has no method filter entries | All methods included | All methods demoted |
| Type has explicit `::Method` entries | Listed methods included, rest included too | Listed methods included, rest demoted |
| Type has `::*` entry | All methods included | All methods included |
| Method deps missing from types map | Method demoted (with warning) | N/A (MinimalTypeMap ensures deps exist) |

In default mode, method-level filters are *additive allow-list on top of include-all*.
In minimal mode, they are *the mechanism that determines what gets included at all*.

The `default_demote` flag on `Filter` (set to `true` in minimal mode) controls
this behavior in `includes_method()`.

### `--deps` (internal)

The `--deps` option exists for internal bootstrapping (e.g., `windows-core`
generating its own sys-style bindings with `--deps none`). It controls whether
core types reference external crates or are emitted inline.

For `--sys` mode, `--deps none` is always the effective default — users should
not need to specify it. For default/minimal modes, `--deps core` is the
default. The `--deps specific` value is used internally by crates that depend
on `windows-result`/`windows-strings` directly rather than through
`windows-core`.

### `--package` (internal)

The `--package` option generates a complete crate with `Cargo.toml` feature
gates and per-namespace module files. It is used exclusively by `tool_package`
for generating the published `windows` and `windows-sys` crates and is not
intended for external use.
