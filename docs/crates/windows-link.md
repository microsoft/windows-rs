# windows-link

> The `link!` macro for linking to Windows functions without import libraries.

- [crates.io](https://crates.io/crates/windows-link)
- [docs.rs](https://docs.rs/windows-link)
- [Getting started](../../crates/libs/link/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/link)

`windows-link` provides the `link!` macro for declaring external Windows
functions. It uses `raw-dylib`, so it does not require import `.lib` files.
The macro declares the function and also emits a `pub type` alias of the same
name describing the function-pointer signature.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-link`**.

### How it's built

Entirely hand-written - a single `link!` macro with per-architecture expansions.
No generated code.

### Testing

Run `cargo test -p windows-link`; see also the workspace test crates.
