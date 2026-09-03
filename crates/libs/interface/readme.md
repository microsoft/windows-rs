## windows-interface

The [windows-interface](https://crates.io/crates/windows-interface) crate provides the
`#[interface]` macro for declaring COM interfaces in Rust.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-interface.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows-core = "0.100"
windows-interface = "0.100"
```

```rust
use windows_core::IUnknown;
use windows_interface::interface;

#[interface("57f61c8b-b400-4035-a7c2-abbddf9e3559")]
unsafe trait IValue: IUnknown {
    fn get(&self) -> u32;
}
```
