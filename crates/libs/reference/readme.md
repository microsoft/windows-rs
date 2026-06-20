## Windows reference type

The [windows-reference](https://crates.io/crates/windows-reference) crate provides a stock implementation of the `IReference<T>` interface used by Windows APIs to box and pass optional or reference values.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-reference]
version = "0.1"
```

Use the Windows reference type as needed:

```rust
use windows_reference::*;

let value = IReference::<i32>::from(42);
assert_eq!(value.Value().unwrap(), 42);
```
