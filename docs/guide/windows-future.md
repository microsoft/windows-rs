# windows-future

> Stock async types for Windows APIs.

- 📦 [crates.io](https://crates.io/crates/windows-future)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-future)
- 🛠 [Internals](../internals/windows-future.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/future)

## Overview

`windows-future` provides `IAsyncOperation` and the related WinRT async
interfaces, with conveniences for producing and consuming them from Rust. Use
`ready` to wrap an already-available value, `spawn` to run work on a background
thread, and `join` to block until a result is available. The types also
implement `IntoFuture`, so they can be `.await`ed.

## Example

```rust
use windows_future::IAsyncOperation;

// A value that is available immediately.
let ready = IAsyncOperation::ready(Ok(123));
assert_eq!(ready.join().unwrap(), 123);

// Run work on a background thread and wait for the result.
let computed = IAsyncOperation::spawn(|| Ok(456));
assert_eq!(computed.join().unwrap(), 456);
```