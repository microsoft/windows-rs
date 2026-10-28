# windows-sys

Raw Windows function declarations, constants, and ABI types. This crate does not add resource
ownership, COM wrappers, or automatic error conversion.

- 📦 [crates.io](https://crates.io/crates/windows-sys)
- 📖 [API reference](https://microsoft.github.io/windows-docs-rs/)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows-sys)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/sys)

Prefer a [focused crate][crates] when one covers the task. Reusable libraries should generate
narrow private bindings with [windows-bindgen](https://docs.rs/windows-bindgen).
Use `windows-sys` when a binary application needs a broad, pre-generated raw API.

## Get started

Find the native function in the API reference and enable its feature:

```toml
[dependencies]
windows-sys = { version = "0.100", features = ["sysinfoapi"] }
```

```rust,no_run
let milliseconds = unsafe { windows_sys::Win32::GetTickCount64() };
println!("{milliseconds} ms since startup");
```

Win32 features follow header groups. Types used in a signature may need supporting features.
Check OS availability separately from compile-time features.

## Keep the FFI boundary small

Raw signatures preserve pointers, lengths, integer status codes, and native callbacks.

| Concern | Caller responsibility |
| --- | --- |
| Ownership | Pair every owned handle or allocation with its documented release operation. |
| Failure | Distinguish null, `INVALID_HANDLE_VALUE`, zero, and status-code conventions. |
| Last error | Read `GetLastError` immediately after a documented last-error failure. |
| Buffers | Supply valid storage and the correct length unit: bytes, code units, or elements. |
| Callbacks | Match the ABI and retain callback state for the required lifetime. |

Validate inputs before entering unsafe code, capture results immediately, and put owned resources
in guards before further fallible work.

The `w!` and `s!` macros supply null-terminated literals. For dynamic UTF-16 input, encode into a
buffer, append a zero, and keep it alive for the call. Narrow Windows strings are not necessarily
UTF-8. Output buffers follow the function's size and allocation rules.

[windows-result](https://docs.rs/windows-result) can help translate native errors.
Choose [windows](https://crates.io/crates/windows) when typed interfaces and projected results fit
better.

[crates]: https://github.com/microsoft/windows-rs/blob/master/docs/readme.md
