## windows-collections

The [windows-collections](https://crates.io/crates/windows-collections) crate implements Windows
collection interfaces for Rust collections.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-collections.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-collections]
version = "0.100"
```

```rust
use windows_collections::*;

let numbers = IIterable::<i32>::from(vec![1, 2, 3]);

for value in numbers {
    println!("{value}");
}
```
