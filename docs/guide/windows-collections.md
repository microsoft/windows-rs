# windows-collections

> Stock WinRT collection types backed by Rust containers.

- 📦 [crates.io](https://crates.io/crates/windows-collections)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-collections)
- 🛠 [Internals](../internals/windows-collections.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/collections)

## Overview

`windows-collections` provides ready-made implementations of the WinRT collection
interfaces — `IIterable`, `IVector`, `IVectorView`, `IMap`, and `IMapView` — so
you can hand a Rust `Vec` or `BTreeMap` to an API that expects a WinRT collection.

## Example

```rust
use windows_collections::IIterable;

let numbers = IIterable::<i32>::from(vec![1, 2, 3]);

let doubled: Vec<i32> = numbers.into_iter().map(|n| n * 2).collect();
assert_eq!(doubled, [2, 4, 6]);
```