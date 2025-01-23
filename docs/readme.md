## Rust for Windows

The [windows](https://crates.io/crates/windows) and [windows-sys](https://crates.io/crates/windows-sys) crates let you call any Windows API past, present, and future using code generated on the fly directly from the [metadata describing the API](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen/default) and right into your Rust package where you can call them as if they were just another Rust module. The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)
* [Feature search](https://microsoft.github.io/windows-rs/features)

This repo is the home of the following crates (and other supporting crates):

* [windows-sys](https://crates.io/crates/windows-sys) - Raw bindings for C-style Windows APIs.
* [windows](https://crates.io/crates/windows) - Safer bindings including C-style APIs as well as COM and WinRT APIs.

* [windows-bindgen](https://crates.io/crates/windows-bindgen) - Code generator for Windows metadata.
* [windows-core](https://crates.io/crates/windows-core) - Core type support for the Windows bindings.
* [windows-registry](https://crates.io/crates/windows-registry) - Windows registry.
* [windows-result](https://crates.io/crates/windows-result) - Windows error handling.
* [windows-strings](https://crates.io/crates/windows-strings) - Windows string types.
* [windows-link](https://crates.io/crates/windows-link) - Direct linking for Windows.
* [windows-targets](https://crates.io/crates/windows-targets) - Import libs for Windows (and the precursor to `windows-link` for older compilers).
* [windows-version](https://crates.io/crates/windows-version) - Windows version information.

* [cppwinrt](https://crates.io/crates/cppwinrt) - Bundles the C++/WinRT compiler for use in Rust.
