# Crate Reference

This document covers the smaller utility crates in `windows-rs` that don't have
their own dedicated documentation. For the main crates, see:

- [reactor.md](reactor.md) — `windows-reactor` (declarative UI)
- [canvas.md](canvas.md) — `windows-canvas` (2D graphics)
- [animation.md](animation.md) — `windows-animation` (variable interpolation)
- [bindgen.md](bindgen.md) — `windows-bindgen` (code generation)
- [time.md](time.md) — `windows-time` (TimeSpan and DateTime)

---

## `windows` and `windows-sys`

The two main projection crates:

- **`windows`** — Safer bindings including C-style APIs, COM, and WinRT.
  Provides smart pointers, `Deref` to default interfaces, `Param` trait
  for automatic conversions, and full WinRT support.

- **`windows-sys`** — Raw bindings for C-style Windows APIs. Zero overhead,
  no COM/WinRT support. Use when you need minimal dependencies and are
  calling only flat Win32 functions.

Both crates use code generated on the fly from Windows metadata. Select APIs
via Cargo feature flags — see the
[feature search](https://microsoft.github.io/windows-rs/features) tool.

---

## `windows-core`

Core type support for COM and Windows. Provides the fundamental traits and
types that all other `windows-*` crates build on:

- `IUnknown`, `IInspectable`, `Interface` trait
- `HSTRING` interop, `GUID`, `VARIANT`
- `ComObject`, `Ref`, `OutRef` for COM object lifecycle
- `RuntimeType`, `RuntimeName` for WinRT reflection
- `Param` trait for automatic parameter conversions
- `implement` and `interface` proc macros (via `windows-implement` and
  `windows-interface`)

---

## `windows-result`

Windows error handling types:

```rust
use windows_result::*;

// HRESULT-based error handling
let hr = HRESULT(0x80070005_u32 as i32); // E_ACCESSDENIED
let err = Error::from(hr);
println!("{err}"); // prints the system error message

// Result<T> = core::result::Result<T, Error>
fn example() -> Result<()> {
    hr.ok()?; // converts HRESULT to Result
    Ok(())
}
```

Key types: `Error`, `HRESULT`, `Result<T>`, `BOOL`, `WIN32_ERROR`, `NTSTATUS`.

---

## `windows-strings`

Windows string types for interop:

```rust
use windows_strings::*;

// HSTRING — immutable, reference-counted UTF-16 string (WinRT)
let s = HSTRING::from("Hello");
let s = h!("compile-time HSTRING");

// BSTR — COM string (Windows only)
let b = BSTR::from("Hello");

// Null-terminated pointer wrappers
let p: PCWSTR = w!("wide string literal");
let p: PCSTR = s!("narrow string literal");
```

Key types: `HSTRING`, `HStringBuilder`, `BSTR`, `PCSTR`, `PCWSTR`, `PSTR`, `PWSTR`.

Macros: `s!("...")` (PCSTR literal), `w!("...")` (PCWSTR literal), `h!("...")` (HSTRING literal).

---

## `windows-numerics`

Windows numeric types for graphics math:

- `Vector2`, `Vector3`, `Vector4`
- `Matrix3x2`, `Matrix4x4`

These are `#[repr(C)]` POD types with inherent methods (`new`, `zero`, `dot`,
`length`, etc.) and operator trait impls (`Add`, `Sub`, `Mul`, `Neg`, `Display`).
They match the `Windows.Foundation.Numerics` namespace and are ABI-compatible
with WinRT.

---

## `windows-registry`

Safe Windows registry access:

```rust
use windows_registry::*;

// Read from HKEY_CURRENT_USER
let key = CURRENT_USER.open(r"Software\MyApp")?;
let name: String = key.get_string("Name")?;
let count: u32 = key.get_u32("Count")?;

// Create and write
let key = CURRENT_USER.create(r"Software\MyApp")?;
key.set_string("Name", "Hello")?;
key.set_u32("Count", 42)?;

// Iterate subkeys and values
for subkey in key.keys()? { /* ... */ }
for (name, value) in key.values()? { /* ... */ }

// Transactional writes
let tx = Transaction::new()?;
let key = CURRENT_USER.open_with_transaction(r"Software\MyApp", &tx)?;
key.set_string("Name", "Updated")?;
tx.commit()?;
```

Predefined roots: `CURRENT_USER`, `LOCAL_MACHINE`, `CLASSES_ROOT`, `USERS`, `CURRENT_CONFIG`.

---

## `windows-version`

Query the Windows version at runtime:

```rust
use windows_version::OsVersion;

let version = OsVersion::current();
println!("Windows {}.{} build {}", version.major, version.minor, version.build);

// Compare against a known version
if OsVersion::current() >= OsVersion::new(10, 0, 0, 22000) {
    println!("Windows 11 or later");
}

// Check if running on Windows Server
if windows_version::is_server() {
    println!("Server edition");
}
```

---

## `windows-collections`

Stock implementations of WinRT collection interfaces (`IIterable<T>`,
`IIterator<T>`, `IVectorView<T>`, `IMapView<K,V>`). Used internally by the
`windows` crate to provide Rust-friendly collection semantics for WinRT APIs.

---

## `windows-future`

Windows async type support. Implements `IAsyncAction`, `IAsyncOperation<T>`,
and their `WithProgress` variants as Rust `Future`s, bridging WinRT async
patterns with `async`/`await`.

---

## `windows-reference`

Stock implementation of `IReference<T>` — the WinRT equivalent of
`Nullable<T>`. Enables passing optional value types (primitives, enums, structs)
through WinRT APIs that expect `IReference<T>`.

---

## `windows-link`

Low-level linking support for Windows APIs. Provides the `link!` macro that
handles `raw-dylib` imports with fallback for older compilers. Used internally
by most `windows-*` crates; not typically used directly by applications.

---

## `windows-targets`

Import libraries for Windows. The precursor to `windows-link` for older Rust
compilers. Provides arch-specific import lib crates
(`windows_x86_64_msvc`, `windows_aarch64_msvc`, etc.).

---

## `windows-threading`

Windows threading primitives:

- Thread pool work items
- Timer-based callbacks
- Wait-based callbacks

A thin safe wrapper over the Win32 thread pool API.

---

## `windows-services`

Windows service control and runtime support. Provides types for implementing
Windows services (daemons) in Rust, including service registration, start/stop
lifecycle, and control handler dispatch.

---

## `windows-metadata`

Low-level ECMA-335 metadata reader/writer. Parses `.winmd` files and provides
access to type definitions, method signatures, attributes, and cross-references.
Used by `windows-bindgen` to generate bindings. Not typically used directly
unless building custom code generators.

---

## `cppwinrt`

Bundles the C++/WinRT compiler (`cppwinrt.exe`) for use from Rust build scripts.
Useful when a Rust project needs to generate C++ projection headers for WinRT
components alongside Rust code.

---

## Unpublished Crates

These crates exist in the repo but are not published to crates.io:

| Crate | Description |
|-------|-------------|
| `windows-reactor` | Declarative UI library (v0.0.0, pre-release) |
| `windows-canvas` | 2D graphics library (v0.0.0, pre-release) |
| `windows-animation` | Animation manager wrapper (v0.0.0, pre-release) |
| `windows-reactor-setup` | Windows App Runtime dependency installer |
| `windows-rdl` | RDL parser / ECMA-335 generator |
| `riddle` | Windows metadata compiler |
