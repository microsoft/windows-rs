## Core type support for COM and Windows

The [windows-core](https://crates.io/crates/windows-core) crate provides core type support for the windows-* family of crates.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

It is the foundation that nearly every other windows-* crate builds on,
providing the COM/WinRT machinery - `IUnknown`, `IInspectable`, the `Interface`
trait, reference counting, and agile and weak references - along with `GUID`
and re-exports of the result and string types. The
[`#[implement]`](https://docs.rs/windows-implement) and
[`#[interface]`](https://docs.rs/windows-interface) macros are re-exported here
behind the default `proc-macros` feature.

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-core]
version = "0.62"
```

Use the core types as needed:

```rust
use windows_core::{h, Result, HRESULT, HSTRING};

// WinRT reference-counted strings.
let name: &HSTRING = h!("Windows.Foundation.Uri");
assert_eq!(name, &HSTRING::from("Windows.Foundation.Uri"));

// HRESULT-based results.
fn check(code: HRESULT) -> Result<()> {
    code.ok()
}

assert!(check(HRESULT(0)).is_ok());
assert!(check(HRESULT(-2147467259)).is_err());
```
