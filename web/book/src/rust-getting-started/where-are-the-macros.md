# Where's my favorite macro from the Windows SDK?

The [windows](https://crates.io/crates/windows) and [windows-sys](https://crates.io/crates/windows-sys) crates are [generated from metadata](how-are-crates-built.md). This metadata only includes type definitions and function signatures, not macros, header-only functions, or function bodies. You may find some equivalents of common C/C++ helper macros and functions in the `windows` crate, but in general the macros don't have direct equivalents in the `windows` or `windows-sys` crates. 
