# windows-result

> Efficient Windows error handling and propagation for Win32, COM, and WinRT.

- 📦 [crates.io](https://crates.io/crates/windows-result)
- 📖 [docs.rs](https://docs.rs/windows-result)
- 🚀 [Getting started](../../crates/libs/result/readme.md)
- 🧩 [Samples](../../crates/samples/result)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/result)

## When to use this crate

Use `windows-result` when code works directly with Windows status values or exposes Windows errors
without depending on the larger `windows-core` crate. Generated windows-* APIs normally return
these types through a re-export, so application code often does not need a separate dependency.

Choose this model when callers need the original `HRESULT`, Win32 error code, or COM error
information. Convert to a domain-specific error at your application boundary when callers do not
need the Windows details.

## Getting started

The crate [README](../../crates/libs/result/readme.md) contains the dependency declaration and a
minimal example. A typical first workflow is to keep `windows_result::Result<T>` as the return type,
convert the status value immediately after a Windows call, and use `?` for propagation:

```rust
use windows_result::{HRESULT, Result};

fn check_status(status: HRESULT) -> Result<()> {
    status.ok()?;
    Ok(())
}
```

On Windows, status-to-message conversion asks the operating system for descriptive text. The core
status and error types also compile without `std`; integrations such as `std::error::Error` and
`std::io::Error` require the default `std` feature.

## Core API model

| Type | Success rule | Typical source |
| --- | --- | --- |
| `HRESULT` | Any value >= 0 | COM and WinRT methods |
| `BOOL` | Any nonzero value | Win32 functions with `GetLastError` on failure |
| `WIN32_ERROR` | Zero | `GetLastError` and Win32 error constants |
| `NTSTATUS` | Any value >= 0 | Native system APIs |
| `RPC_STATUS` | Zero | RPC APIs |
| `Error` | Always represents an error | Rich Rust error with code and optional error info |
| `Result<T>` | `Result<T, Error>` | Windows operations exposed to Rust |

The wrappers preserve the status representation until it needs to become an `Error`. Their `ok`
methods apply the correct success rule, and conversions normalize failures to an `HRESULT`.

## Common tasks

### Convert a status and preserve its code

```rust
use windows_result::{Error, Result, WIN32_ERROR};

fn cancelled() -> Result<()> {
    let code = WIN32_ERROR(1223);
    Err(Error::from(code))
}

let error = cancelled().unwrap_err();
assert_eq!(WIN32_ERROR::from_error(&error), Some(WIN32_ERROR(1223)));
```

`WIN32_ERROR::to_hresult` uses the standard `FACILITY_WIN32` mapping. Use
`WIN32_ERROR::from_error` when you need to recover a mapped Win32 code.

### Attach context to a failure

Use `Error::new` when you are creating the failure and have a useful message:

```rust
use windows_result::{Error, WIN32_ERROR};

let code = WIN32_ERROR(1223).to_hresult();
let error = Error::new(code, "operation cancelled");

assert_eq!(error.code(), code);
assert_eq!(error.message(), "operation cancelled");
```

`Error::from_hresult` stores only the code. Converting an `HRESULT` with `Error::from` can capture
thread-local COM error information when it is available. `Error::message` falls back to the system
description for the code.

### Work with last-error APIs

Call `BOOL::ok`, `WIN32_ERROR::from_thread`, or `Error::from_thread` immediately after the failing
Windows call. Another Windows call may overwrite the thread's last-error value.

```rust
use windows_result::{BOOL, Result};

fn check_win32_result(value: BOOL) -> Result<()> {
    value.ok()
}
```

Only use this pattern for APIs whose documentation says failure details come from `GetLastError`.
Some Win32 functions use other return conventions.

## Important choices and pitfalls

- `HRESULT(1)` is success, not failure. Test it with `is_ok` or `ok` rather than comparing with
  zero.
- `BOOL(0).ok()` reads the current thread's last-error value. Do not construct a false `BOOL` later
  and expect it to retain the original error.
- `Error` equality and ordering compare the `HRESULT`, not the message or COM error information.
- Converting a mapped Win32 `Error` to `std::io::Error` recovers the underlying Win32 code.
  Other facilities retain the full `HRESULT` as the raw OS error.
- `--cfg=windows_slim_errors` is a whole-program policy that stores only the four-byte `HRESULT`.
  It reduces error size but discards extended COM and WinRT error information.

## Samples and next steps

The [result samples](../../crates/samples/result/samples/examples) contain two focused programs:
`error.rs` creates an error with a message, and `propagate.rs` propagates and inspects failures.
Run them from the workspace with:

```text
cargo run -p result_samples --example error
cargo run -p result_samples --example propagate
```

For error handling alongside projected COM types, continue with
[`windows-core`](windows-core.md).

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-result`**.

### How it's built

`src/bindings.rs` is generated by `tool_bindings` from `crates/tools/bindings/src/result.txt`. The
`com`, `bstr`, and `strings` modules add COM error-info support and are gated on `windows` (and
disabled under the `windows_slim_errors` cfg).

### Testing

Run `cargo test -p windows-result`; see also the workspace test crates.
