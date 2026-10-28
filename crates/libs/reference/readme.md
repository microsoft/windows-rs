# windows-reference

Box Rust values as WinRT `IReference<T>` objects. Use this at a Windows API boundary, not as a
substitute for Rust borrowing, `Box`, or `Option`.

- 📦 [crates.io](https://crates.io/crates/windows-reference)
- 📖 [docs.rs](https://docs.rs/windows-reference)
- 🧩 [Sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reference/reference)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reference)

## Get started

```toml
[dependencies]
windows-reference = "0.100"
```

```rust
use windows_reference::IReference;

let value = IReference::<i32>::from(42);
assert_eq!(value.Value().unwrap(), 42);
```

## Match the receiving signature

The generic type determines the interface identity: `IReference<i32>` and `IReference<u32>` are
different interfaces. An `IReference<T>` contains a present value; a surrounding nullable interface
represents absence.

Cloning the interface retains the same object. `Value()` retrieves the value through a fallible
interface call; propagate its result in application code. Read once at the boundary, then continue
with ordinary Rust data.

The crate requires Windows and the default `std` feature. `IReference<HSTRING>`
also accepts Rust strings; use [windows-strings](https://docs.rs/windows-strings) for the string
type. See [windows-collections](https://docs.rs/windows-collections) for collection interfaces.
