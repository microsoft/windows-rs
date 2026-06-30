# windows-implement

> The `#[implement]` macro for implementing COM and WinRT interfaces in Rust.

- 📦 [crates.io](https://crates.io/crates/windows-implement)
- 📖 [docs.rs](https://docs.rs/windows-implement)
- 🚀 [Getting started](../../crates/libs/implement/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/implement)

`windows-implement` provides the `#[implement]` attribute macro that lets a Rust
type implement one or more COM or WinRT interfaces. It is **part of
[`windows-core`](windows-core.md)** and exists as a separate crate only because
Rust requires procedural macros to ship in their own `proc-macro` crate.
`windows-core` re-exports it behind the default `proc-macros` feature, so you
depend on `windows-core` and rarely name this crate directly.

See [`windows-core`](windows-core.md) for how to declare and implement interfaces —
including a worked example, the `cast`/identity model, and the `implement_decl!`
declarative alternative for builds without the proc macros.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-implement`**.

### How it's built

A `proc-macro` crate. The crate-level docs in `src/lib.rs` cover the generated
`*_Impl` vtable layout and how it pairs with `#[interface]`. Uses
`syn`/`quote`/`proc-macro2` to parse the attribute and emit the vtable plumbing.

### Testing

Run `cargo test -p windows-implement`; see also the workspace test crates.
