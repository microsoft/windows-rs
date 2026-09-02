# windows-reference

> A stock implementation of the WinRT `IReference<T>` interface.

- 📦 [crates.io](https://crates.io/crates/windows-reference)
- 📖 [docs.rs](https://docs.rs/windows-reference)
- 🚀 [Getting started](../../crates/libs/reference/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reference)

## When to use this crate

Use `windows-reference` when a WinRT signature explicitly requires `IReference<T>` and you need to
box a Rust value for that call or component boundary. WinRT uses this interface for boxed value
types, similar to `Nullable<T>` in other projections.

Do not use it for ordinary Rust borrowing. `&T`, `Box<T>`, and `Option<T>` express Rust ownership
and optionality without creating a WinRT object. Add this crate only at an interop boundary that
names `IReference<T>`.

## Getting started

The crate [README](../../crates/libs/reference/readme.md) has the dependency declaration and
minimal boxing example. The public implementation is available on Windows with the default `std`
feature. The value type must implement `RuntimeType + Clone + 'static`.

The first workflow is to convert a value with `IReference::<T>::from`, pass the resulting projected
interface to the WinRT API, and call `Value` when reading one received from an API:

```rust
use windows_reference::IReference;

fn round_trip() -> i32 {
    let boxed = IReference::<i32>::from(42);
    boxed.Value().unwrap()
}
```

In application code, propagate the `Result` from `Value` instead of unwrapping when the interface
call can fail.

## Core API model

`IReference<T>` is a reference-counted WinRT object. Creating one clones or moves the Rust value
into a stock implementation, and each call to `Value` returns a clone. Cloning the
`IReference<T>` itself clones the interface reference, not the boxed value.

The object implements the WinRT identity and runtime type information for
`Windows.Foundation.IReference<T>`. It also supports `IPropertyValue` for scalar value types, but
the public Rust wrapper exposes `Value` as the supported way to retrieve the boxed value.

## Common tasks

### Box scalar and string values

```rust
use windows_reference::IReference;
use windows_strings::HSTRING;

let count = IReference::<u32>::from(3);
assert_eq!(count.Value().unwrap(), 3);

let label = IReference::<HSTRING>::from("ready");
assert_eq!(label.Value().unwrap(), "ready");
```

`IReference<HSTRING>` accepts `HSTRING`, `&HSTRING`, `String`, and `&str`. Other supported value
types use their normal `From<T>` conversion. Add `windows-strings` directly when the surrounding
crate does not already provide its string types.

### Use a boxed value in a component

Keep `IReference<T>` in the component signature when the metadata declares a boxed value. Accept
the interface by the projected parameter form, call `Value` once, and continue with the plain Rust
value. When returning one, construct it at the return boundary:

```rust
use windows_reference::IReference;

fn boxed_count(value: u32) -> IReference<u32> {
    IReference::from(value)
}
```

This keeps WinRT reference counting at the boundary and avoids repeated ABI calls in the rest of
the implementation.

## Important choices and pitfalls

- `IReference<T>` boxes a present value. Optionality is represented by whether the interface
  itself is present in the surrounding projected signature.
- `Value` is fallible because it is a WinRT interface call, even for an object created locally.
- The value is cloned into and out of the stock object. Avoid using it as a high-frequency wrapper
  for large values when a different API shape is under your control.
- The generic type determines the interface IID and must match the receiving signature exactly.
  `IReference<i32>` and `IReference<u32>` are different WinRT interface types.
- Array accessors on the object's internal `IPropertyValue` implementation are not supported.
  Use the typed `Value` method.

## Samples and next steps

There is no dedicated `windows-reference` sample group. The
[README](../../crates/libs/reference/readme.md) is the smallest complete example. For related
interop patterns, see [`windows-collections`](windows-collections.md) for collection interfaces and
the [`robot` sample](../../crates/samples/robot) for a component boundary implemented in Rust.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-reference`**.

### How it's built

`src/bindings.rs` is generated; the `IReference<T>` implementation is hand-written.

### Testing

Run `cargo test -p windows-reference`; see also the workspace test crates.
