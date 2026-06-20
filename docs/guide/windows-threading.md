# windows-threading

> Simple, safe access to the Windows thread pool.

- 📦 [crates.io](https://crates.io/crates/windows-threading)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-threading)
- 🛠 [Internals](../internals/windows-threading.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/threading)

## Overview

`windows-threading` exposes the Windows thread pool through a small, safe API.
`submit` runs a closure on a pool thread, `for_each` runs a closure over an
iterator in parallel and waits for completion, and the `Pool` type offers
direct control including thread limits and scoped submission.

## Example

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);

// Run the closure over each value in parallel, then wait for all to finish.
windows_threading::for_each(0..10, |value| {
    counter.fetch_add(value, Ordering::Relaxed);
});

assert_eq!(counter.load(Ordering::Relaxed), 45);
```