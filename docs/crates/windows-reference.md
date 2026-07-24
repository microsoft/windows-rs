# windows-reference

> A stock implementation of the WinRT `IReference<T>` interface.

- [crates.io](https://crates.io/crates/windows-reference)
- [docs.rs](https://docs.rs/windows-reference)
- [Getting started](../../crates/libs/reference/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reference)

Many Windows APIs use `IReference<T>` to box an optional or by-reference value (the WinRT equivalent
of `Nullable<T>`). `windows-reference` provides a ready implementation so you can wrap a Rust value
and hand it to such an API.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-reference`**.

### How it's built

`src/bindings.rs` is generated; the `IReference<T>` implementation is hand-written.

### Testing

Run `cargo test -p windows-reference`; see also the workspace test crates.
