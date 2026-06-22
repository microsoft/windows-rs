# windows-targets

> Import libraries and the `link!` macro for Windows.

- 📦 [crates.io](https://crates.io/crates/windows-targets)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-targets)
- 🚀 [Getting started](../../crates/libs/targets/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/targets)

`windows-targets` bundles the Windows import libraries and provides a `link!`
macro for declaring external functions. It supports semantic versioning and,
optionally, `raw-dylib`.

> **Prefer [`windows-link`](windows-link.md)** for new code (Rust 1.71+): it uses
> `raw-dylib` and needs no import `.lib` files. `windows-targets` remains for
> scenarios that require import libraries.

## How it's built

Hand-written shim that selects the matching `windows_<arch>_<abi>` import-lib
crate. The precursor to `windows-link`.

## Testing

Run `cargo test -p windows-targets`; see also the workspace test crates.
