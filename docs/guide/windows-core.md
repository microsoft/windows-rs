# windows-core

> Core COM and WinRT type support shared by the windows-* crates.

- 📦 [crates.io](https://crates.io/crates/windows-core)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-core)
- 🛠 [Internals](../internals/windows-core.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/core)

## Overview

`windows-core` is the foundation that nearly every other crate builds on. It
provides the COM/WinRT runtime machinery — `IUnknown`, `IInspectable`, the
`Interface` trait, reference counting, agile references, and weak references —
along with `GUID` and re-exports of the result and string types. The
[`#[implement]`](windows-implement.md) and [`#[interface]`](windows-interface.md)
macros are re-exported here as well.

## Example

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