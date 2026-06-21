# windows-reference

> A stock implementation of the WinRT `IReference<T>` interface.

- 📦 [crates.io](https://crates.io/crates/windows-reference)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-reference)
- 🛠 [Internals](../internals/windows-reference.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reference)

## Overview

Many Windows APIs use `IReference<T>` to box an optional or by-reference value
(the WinRT equivalent of `Nullable<T>`). `windows-reference` provides a ready
implementation so you can wrap a Rust value and hand it to such an API.

## Example

```rust
use windows_reference::IReference;

let value = IReference::<i32>::from(42);
assert_eq!(value.Value().unwrap(), 42);
```