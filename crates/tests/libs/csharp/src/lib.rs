//! Test harness for the `windows-csharp` C# projection generator.
//!
//! The tests live in `tests/csharp.rs`. Each `input/*.rdl` fixture produces a self-updating golden
//! in `expected/*.cs` (the projection fragment), plus a combined compile check and an end-to-end
//! round trip against the real `test_bench_component` WinRT component. See `tests/csharp.rs` and
//! `build.rs` for details.
