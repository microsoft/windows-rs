## windows-implement

The [windows-implement](https://crates.io/crates/windows-implement) crate provides the
`#[implement]` macro for implementing COM and WinRT interfaces in Rust.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-implement.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows-core = "0.100"
windows-implement = "0.100"
```

```rust
use windows_core::{IUnknown, interface};
use windows_implement::implement;

#[interface("57f61c8b-b400-4035-a7c2-abbddf9e3559")]
unsafe trait IValue: IUnknown {
    fn get(&self) -> u32;
}

#[implement(IValue)]
struct Value(u32);

impl IValue_Impl for Value_Impl {
    fn get(&self) -> u32 {
        self.0
    }
}

let value: IValue = Value(42).into();
assert_eq!(unsafe { value.get() }, 42);
```
