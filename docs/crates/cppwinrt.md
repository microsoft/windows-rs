# cppwinrt

> The C++/WinRT compiler, bundled for use from Rust.

- 📦 [crates.io](https://crates.io/crates/cppwinrt)
- 📖 [API reference (docs.rs)](https://docs.rs/cppwinrt)
- 🚀 [Getting started](../../crates/libs/cppwinrt/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/cppwinrt)

`cppwinrt` packages the [C++/WinRT](https://github.com/microsoft/cppwinrt)
compiler so it can be invoked from a Rust build. It is a thin wrapper that runs
the bundled `cppwinrt.exe` with the arguments you provide, returning its output.
This is primarily useful for interop scenarios that also generate C++/WinRT
projection headers.

## How it's built

Packages the prebuilt `cppwinrt.exe` and exposes a path to it from build scripts.
No generated Rust bindings.

## Testing

Run `cargo test -p cppwinrt`; see also the workspace test crates.
