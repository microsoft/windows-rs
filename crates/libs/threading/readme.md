# windows-threading

Safe access to the Windows thread pool, including scoped tasks that borrow local data.
Use it for CPU or blocking work that does not need a dedicated thread.

- 📦 [crates.io](https://crates.io/crates/windows-threading)
- 📖 [docs.rs](https://docs.rs/windows-threading)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/threading)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/threading)

## Get started

```toml
[dependencies]
windows-threading = "0.100"
```

```rust,no_run
let total = std::sync::Mutex::new(0);

windows_threading::for_each(0..10, |value| {
    *total.lock().unwrap() += value;
});

assert_eq!(*total.lock().unwrap(), 45);
```

## Choose lifetime and concurrency

| API | Behavior |
| --- | --- |
| `submit` | Submit a `'static` closure to the default pool without waiting. |
| `for_each` | Process iterator items in parallel and wait for completion. |
| `Pool::submit` | Submit a `'static` closure to a private pool. |
| `Pool::scope` | Submit borrowing closures and wait for the private pool. |
| `Pool::join` | Wait for outstanding work on that pool. |

Dropping a pool also waits. Avoid waiting on tasks queued to the same constrained pool, and handle
expected failures inside callbacks rather than panicking. Set thread limits only when the workload
needs them.

Use a dedicated thread for work requiring a stable thread identity or apartment.
The [pool sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/threading/pool)
shows scoped work and thread limits.
