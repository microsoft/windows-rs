## Rust for Windows

This repo provides a comprehensive set of Rust crates for building Windows applications — from low-level API access to high-level declarative UI.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

This repo is the home of the following crates:

* [windows-animation](https://crates.io/crates/windows-animation) - Variable interpolation via the Windows Animation Manager. [Docs](animation.md)
* [windows-bindgen](https://crates.io/crates/windows-bindgen) - Code generator that produces bindings from Windows metadata. [Docs](bindgen.md)
* [windows-canvas](https://crates.io/crates/windows-canvas) - Idiomatic 2D graphics built on Direct2D. [Docs](canvas.md)
* [windows-collections](https://crates.io/crates/windows-collections) - WinRT collection types.
* [windows-core](https://crates.io/crates/windows-core) - Fundamental COM/Windows types.
* [windows-future](https://crates.io/crates/windows-future) - WinRT async bridged to Rust futures.
* [windows-implement](https://crates.io/crates/windows-implement) - Proc macro for implementing COM interfaces.
* [windows-interface](https://crates.io/crates/windows-interface) - Proc macro for defining COM interfaces.
* [windows-link](https://crates.io/crates/windows-link) - Raw-dylib linking support.
* [windows-metadata](https://crates.io/crates/windows-metadata) - Low-level ECMA-335 metadata reader/writer.
* [windows-numerics](https://crates.io/crates/windows-numerics) - Graphics math types.
* [windows-reactor](https://crates.io/crates/windows-reactor) - Declarative UI library backed by WinUI 3. [Docs](reactor.md)
* [windows-reference](https://crates.io/crates/windows-reference) - Stock `IReference<T>` implementation.
* [windows-registry](https://crates.io/crates/windows-registry) - Safe Windows registry access.
* [windows-result](https://crates.io/crates/windows-result) - Windows error handling.
* [windows-services](https://crates.io/crates/windows-services) - Windows service control and runtime.
* [windows-strings](https://crates.io/crates/windows-strings) - Windows string types.
* [windows-sys](https://crates.io/crates/windows-sys) - Raw bindings for C-style Windows APIs.
* [windows-targets](https://crates.io/crates/windows-targets) - Import libs for older compilers.
* [windows-threading](https://crates.io/crates/windows-threading) - Thread pool work items, timers, and waits.
* [windows-time](https://crates.io/crates/windows-time) - Windows time types. [Docs](time.md)
* [windows-version](https://crates.io/crates/windows-version) - Windows version information.
* [windows](https://crates.io/crates/windows) - Safer bindings for C-style, COM, and WinRT APIs.
* [cppwinrt](https://crates.io/crates/cppwinrt) - Bundles the C++/WinRT compiler for use in Rust.

See [crates.md](crates.md) for detailed usage examples and API descriptions.
