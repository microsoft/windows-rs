# windows-future

WinRT asynchronous operations that Rust can await, consume with callbacks, or produce from closures.
This crate is not an async executor.

- 📦 [crates.io](https://crates.io/crates/windows-future)
- 📖 [docs.rs](https://docs.rs/windows-future)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/future)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/future)

## Get started

```toml
[dependencies]
windows-future = "0.101"
```

```rust,no_run
use windows_future::IAsyncOperation;

let operation = IAsyncOperation::spawn(|| Ok(456));
assert_eq!(operation.join().unwrap(), 456);
```

## Choose a completion style

| API | Use |
| --- | --- |
| `.await` | Wait within a Rust async function. |
| `join()` | Block a thread that does not need to pump UI work. |
| `when(callback)` | Receive a result without blocking. |

These styles also apply to `IAsyncAction` and the progress interfaces. Each delivers a Windows
`Result`. Choose one style per operation: WinRT permits assigning its completion handler only
once. Do not block a UI thread with `join`.

Consume the successful result of an operation created with `ready` or `spawn` only once.
Native operations retain their own result-retrieval contracts.

`spawn` runs a `Send + 'static` closure on the Windows thread pool. It does not make apartment-bound
objects safe to use there. Cancelling an operation created with `spawn` does not interrupt the
closure.

`spawn` does not initialize COM. To deliver apartment-bound native completion callbacks from its
workers, establish an MTA (for example, with `windows_core::init_mta()`) and keep the handler's
apartment initialized and dispatching calls.

The default `std` feature supplies `.await`, `ready`, and `spawn`.
