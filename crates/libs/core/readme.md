## windows-core

The [windows-core](https://crates.io/crates/windows-core) crate provides COM and WinRT types,
traits, and macros for the windows-* family of crates.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-core.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-core]
version = "0.100"
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
