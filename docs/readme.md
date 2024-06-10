## Rust for Windows

The [windows](https://crates.io/crates/windows) and [windows-sys](https://crates.io/crates/windows-sys) crates let you call any Windows API past, present, and future using code generated on the fly directly from the [metadata describing the API](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen/default) and right into your Rust package where you can call them as if they were just another Rust module. The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)  <!-- link to latest samples on repo -->
* [Releases](https://github.com/microsoft/windows-rs/releases)
* [Feature search](https://microsoft.github.io/windows-rs/features/#/master)

This repo is the home of the following crates (and other supporting crates):

* [windows](https://crates.io/crates/windows) - Safer bindings including C-style APIs as well as COM and WinRT APIs.
* [windows-bindgen](https://crates.io/crates/windows-bindgen) - Windows metadata compiler library.
* [windows-core](https://crates.io/crates/windows-core) - Type support for the `windows` crate.
* [windows-implement](https://crates.io/crates/windows-implement) - The `implement` macro for the `windows` crate, for implementing COM interfaces.
* [windows-interface](https://crates.io/crates/windows-interface) - The `interface` macro for the `windows` crate, for declaring COM interfaces.
* [windows-metadata](https://crates.io/crates/windows-metadata) - Windows metadata reader.
* [windows-registry](https://crates.io/crates/windows-registry) - Windows registry.
* [windows-result](https://crates.io/crates/windows-result) - Windows error handling.
* [windows-sys](https://crates.io/crates/windows-sys) - Raw bindings for C-style Windows APIs.
* [windows-targets](https://crates.io/crates/windows-targets) - Import libs for Windows.
* [windows-version](https://crates.io/crates/windows-version) - Windows version information.

* [cppwinrt](https://crates.io/crates/cppwinrt) - Bundles the C++/WinRT compiler for use in Rust.
* [riddle](https://crates.io/crates/riddle) - Windows metadata compiler tool.
