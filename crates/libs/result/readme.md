# windows-result

Windows status types, errors, and `Result` for Win32, COM, and WinRT APIs.
Projected APIs often re-export these types; use this crate directly for low-level error handling.

- 📦 [crates.io](https://crates.io/crates/windows-result)
- 📖 [docs.rs](https://docs.rs/windows-result)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/result)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/result)

## Get started

```toml
[dependencies]
windows-result = "0.100"
```

```rust
use windows_result::*;

const ERROR_CANCELLED: WIN32_ERROR = WIN32_ERROR(1223);

fn main() -> Result<()> {
    HRESULT(0).ok()?;
    let error = Error::new(ERROR_CANCELLED.to_hresult(), "operation cancelled");
    assert_eq!(error.code(), ERROR_CANCELLED.to_hresult());
    assert_eq!(error.message(), "operation cancelled");
    Ok(())
}
```

## Check the API's success convention

| Type | Success |
| --- | --- |
| `HRESULT`, `NTSTATUS` | Any value >= 0 |
| `BOOL` | Nonzero |
| `WIN32_ERROR`, `RPC_STATUS` | Zero |

Use the convention documented for the API, then propagate failures with `.ok()?`.
`HRESULT(1)` is success, and not every zero returned by a Win32 function means failure.

For last-error APIs, call `BOOL::ok` or `Error::from_thread` immediately after failure.
Another Windows call can overwrite the thread's error.

`Error::from_hresult` stores only the code; converting with `Error::from` can capture COM error
information. `WIN32_ERROR::from_error` recovers a mapped Win32 code when handling a specific
condition. The default `std` feature adds standard error and I/O conversions.
