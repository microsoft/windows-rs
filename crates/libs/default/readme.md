## windows-default

The [windows-default](https://crates.io/crates/windows-default) crate provides the default metadata
for Windows APIs as embedded byte slices. Build tools can use `WINRT` and `WIN32` without
locating or distributing separate `.winmd` files.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-default.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-default]
version = "0.100"
```

```rust
use windows_default::{WIN32, WINRT};

assert!(!WINRT.is_empty());
assert!(!WIN32.is_empty());
```
