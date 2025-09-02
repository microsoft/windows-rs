# Choosing between the windows and windows-sys crates

The [windows](https://crates.io/crates/windows) crate provides bindings for the Windows API, including C-style APIs like `CreateThreadpool` as well as COM and WinRT APIs like DirectX. This crate provides the most comprehensive API coverage for the Windows operating system. Where possible, the `windows` crate also attempts to provide a more idiomatic and safe programming model for Rust developers.

The [windows-sys](https://crates.io/crates/windows-sys) crate provides raw bindings for the C-style Windows APIs. It lacks support for COM and WinRT APIs. The `windows-sys` crate was born out of the realization that the most expensive aspect of the `windows` crate, in terms of build time, is the cost of compiling function bodies. The Rust compiler just spends a great deal of effort compiling function bodies, so a version of the `windows` crate that only includes declarations is both much smaller and faster by comparison. The trouble is that COM-style virtual function calls require extra code gen in Rust (unlike C++) and this in turn leads to slower compile times. Enter the `windows-sys` crate.

Of course, we continue to work hard at improving performance both in terms of the underlying Rust compiler toolchain as well as the efficiency of the code generated for these crates. We are thus confident that the compile-time will continue to improve.

| What do you need? | `windows` | `windows-sys`|
| --- | --- | --- |
| Fast compile times are one of your top concerns | | ✅ |
| You need `no_std` support | | ✅ |
| You need COM or WinRT support | ✅ | |
| You would prefer to use APIs that feel idiomatic to Rust | ✅ | |
| Minimum supported Rust version | 1.56 | 1.56 |
