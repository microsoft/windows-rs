# cppwinrt

> The C++/WinRT compiler, bundled for use from Rust.

- 📦 [crates.io](https://crates.io/crates/cppwinrt)
- 📖 [API reference (docs.rs)](https://docs.rs/cppwinrt)
- 🛠 [Internals](../internals/cppwinrt.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/cppwinrt)

## Overview

`cppwinrt` packages the [C++/WinRT](https://github.com/microsoft/cppwinrt)
compiler so it can be invoked from a Rust build. It is a thin wrapper that runs
the bundled `cppwinrt.exe` with the arguments you provide, returning its output.
This is primarily useful for interop scenarios that also generate C++/WinRT
projection headers.

## Example

```rust,ignore
println!("{}", cppwinrt::cppwinrt(["-help"]));
```