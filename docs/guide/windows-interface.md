# windows-interface

> The `#[interface]` macro for declaring COM interfaces in Rust.

- 📦 [crates.io](https://crates.io/crates/windows-interface)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-interface)
- 🛠 [Internals](../internals/windows-interface.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/interface)

## Overview

`windows-interface` provides the `#[interface]` attribute macro used to declare
a COM interface as a Rust trait. It is re-exported by
[`windows-core`](windows-core.md), which is the dependency you normally take —
the generated code refers to `windows-core` for `IUnknown`, the vtable layout,
and `QueryInterface` support.

Annotate an `unsafe trait` that derives from `IUnknown` with the interface's
GUID. The macro generates the vtable and the plumbing needed to call and
implement the interface. Pair it with [`#[implement]`](windows-implement.md) to
supply a Rust implementation.

## Example

```rust
use windows_core::*;

// Declare a custom COM interface with its GUID.
#[interface("7e75ffe0-2f8c-4040-953e-b1f83a48f77b")]
unsafe trait IValue: IUnknown {
    unsafe fn value(&self) -> i32;
}

// Implement it with `#[implement]`.
#[implement(IValue)]
struct Value {
    value: i32,
}

impl IValue_Impl for Value_Impl {
    unsafe fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let object: IValue = Value { value: 42 }.into();
    assert_eq!(unsafe { object.value() }, 42);
}
```