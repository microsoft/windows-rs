# windows-result

> Efficient Windows error handling and propagation for Win32, COM, and WinRT.

- 📦 [crates.io](https://crates.io/crates/windows-result)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-result)
- 🛠 [Internals](../internals/windows-result.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/result)

## Overview

`windows-result` is the error layer shared by the windows-* crates. Its building
blocks are:

- **`HRESULT`** — a Windows status code, with `.ok()` to convert into a `Result`.
- **`Error`** — a rich error carrying an `HRESULT` and a message.
- **`Result<T>`** — shorthand for `core::result::Result<T, Error>`.

It also wraps the other Windows status representations — `BOOL`, `WIN32_ERROR`,
and `NTSTATUS` — so they convert cleanly into an `Error`.

## Example

```rust
use windows_result::{Error, Result, HRESULT};

// `HRESULT::ok` maps a success code to `Ok` and a failure code to `Err`.
fn check(code: HRESULT) -> Result<()> {
    code.ok()
}

assert!(check(HRESULT(0)).is_ok());
assert!(check(HRESULT(-2147467259)).is_err());

// Errors carry both a code and a message.
let error = Error::new(HRESULT(-2147023673), "operation canceled");
assert_eq!(error.code(), HRESULT(-2147023673));
assert_eq!(error.message(), "operation canceled");
```