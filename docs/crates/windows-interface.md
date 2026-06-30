# windows-interface

> The `#[interface]` macro for declaring COM interfaces in Rust.

- 📦 [crates.io](https://crates.io/crates/windows-interface)
- 📖 [docs.rs](https://docs.rs/windows-interface)
- 🚀 [Getting started](../../crates/libs/interface/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/interface)

`windows-interface` provides the `#[interface]` attribute macro used to declare a
COM interface as a Rust trait. It is **part of [`windows-core`](windows-core.md)**
and exists as a separate crate only because Rust requires procedural macros to
ship in their own `proc-macro` crate. `windows-core` re-exports it behind the
default `proc-macros` feature, so you depend on `windows-core` and rarely name
this crate directly. The generated code refers to `windows-core` for `IUnknown`,
the vtable layout, and `QueryInterface` support.

See [`windows-core`](windows-core.md) for how to declare an interface and pair it
with [`#[implement]`](windows-implement.md) — including a worked example and the
`interface_decl!` declarative alternative for builds without the proc macros.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-interface`**.

### How it's built

A `proc-macro` crate. The crate-level docs in `src/lib.rs` cover what the macro
generates (the interface struct, `*_Vtbl`, and `*_Impl` trait) and how it must
agree with `#[implement]`. Uses `syn`/`quote`/`proc-macro2`.

### Testing

Run `cargo test -p windows-interface`; see also the workspace test crates.
