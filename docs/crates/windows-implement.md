# windows-implement

> The `#[implement]` macro for implementing COM and WinRT interfaces in Rust.

- 📦 [crates.io](https://crates.io/crates/windows-implement)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-implement)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/implement)

`windows-implement` provides the `#[implement]` attribute macro that lets a Rust
type implement one or more COM or WinRT interfaces. It is re-exported by
[`windows-core`](windows-core.md), which is the dependency you normally take.

Apply `#[implement(...)]` to a struct, listing the interfaces it implements. The
macro generates a companion `*_Impl` type; you write the interface methods in an
`impl <Interface>_Impl for <Struct>_Impl` block. The struct's fields are
accessible through `self`. Converting the struct `.into()` an interface type
produces a reference-counted COM object.

```rust
use windows_core::*;

#[interface("7e75ffe0-2f8c-4040-953e-b1f83a48f77b")]
unsafe trait IValue: IUnknown {
    unsafe fn value(&self) -> i32;
}

// `MyValue` implements `IValue`.
#[implement(IValue)]
struct MyValue {
    value: i32,
}

impl IValue_Impl for MyValue_Impl {
    unsafe fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let object: IValue = MyValue { value: 42 }.into();
    assert_eq!(unsafe { object.value() }, 42);

    // COM identity: `cast` queries for another interface on the same object.
    let unknown: IUnknown = object.cast().unwrap();
    let again: IValue = unknown.cast().unwrap();
    assert_eq!(unsafe { again.value() }, 42);
}
```

## How it's built

A `proc-macro` crate. The crate-level docs live inline in `src/lib.rs`. Uses
`syn`/`quote`/`proc-macro2` to parse the attribute and emit the vtable plumbing.

## Testing

Run `cargo test -p windows-implement`; see also the workspace test crates.
