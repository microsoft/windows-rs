## Windows reference type

The [windows-reference](https://crates.io/crates/windows-reference) crate implements
`IReference<T>` for boxed Windows values.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-reference]
version = "0.1"
```

```rust
use windows_reference::*;

let value = IReference::<i32>::from(42);
assert_eq!(value.Value().unwrap(), 42);
```
