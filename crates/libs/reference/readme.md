## windows-reference

The [windows-reference](https://crates.io/crates/windows-reference) crate implements
`IReference<T>` for boxed Windows values.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reference.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-reference]
version = "0.100"
```

```rust
use windows_reference::*;

let value = IReference::<i32>::from(42);
assert_eq!(value.Value().unwrap(), 42);
```
