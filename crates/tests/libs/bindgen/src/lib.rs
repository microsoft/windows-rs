//! Compile every fixture's `expected.rs` so stale goldens break the build.
//! `build.rs` emits the include file. Test-execution logic lives in
//! `tests/fixtures.rs`. Gated on `cfg(windows)` because some goldens
//! reference Windows-only items.

#[cfg(windows)]
include!(concat!(env!("OUT_DIR"), "/compile_fixtures.rs"));
