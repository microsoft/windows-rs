# windows-implement

Expose Rust objects through COM or WinRT interfaces with `#[implement]`. Use it for components
and callbacks; calling an existing interface does not require the macro.

- 📦 [crates.io](https://crates.io/crates/windows-implement)
- 📖 [docs.rs](https://docs.rs/windows-implement)
- 🧩 [Robot sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/robot)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/implement)

## Get started

```toml
[dependencies]
windows-core = "0.100"
```

```rust
use windows_core::{IUnknown, implement, interface};

#[interface("57f61c8b-b400-4035-a7c2-abbddf9e3559")]
unsafe trait IValue: IUnknown {
    fn get(&self) -> u32;
}

#[implement(IValue)]
struct Value(u32);

impl IValue_Impl for Value_Impl {
    unsafe fn get(&self) -> u32 {
        self.0
    }
}

let value: IValue = Value(42).into();
assert_eq!(unsafe { value.get() }, 42);
```

`windows-core` re-exports this macro through its default `proc-macros` feature.
A direct `windows-implement` dependency is not needed for application code.
Implement required base-interface traits as well as the declared interface.

## Threading and ownership

Objects are agile by default. Synchronize shared mutable state and ensure destruction is valid on
the thread that releases the final reference. Use `Agile = false` in the attribute for an
apartment-affine object. Agility does not make thread-affine fields safe to share.

Interface values retain the object through COM reference counting. Avoid ownership cycles or
provide an explicit teardown path.

Follow the generated trait signatures, returning `Result` or HRESULT errors as required.
Panics are not COM errors and must not escape an FFI callback. Raw pointers and out parameters
still require the interface's safety and ownership rules.
