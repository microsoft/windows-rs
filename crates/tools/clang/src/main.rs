//! Validator for the libclang pin owned by the `windows-clang` crate. Like the other
//! `gen`-matrix tools it is run by CI and "validates by running": it writes nothing, so the
//! tree stays clean, and any inconsistency in how libclang is pinned fails the run loudly.
//!
//! The metadata scrapers (`tool_win32`/`tool_wdk`/`tool_winrt`/`tool_webview`) all parse with
//! the pinned libclang, so a drift between the loaded `libclang.dll` and the clang builtin
//! resource headers would silently change the generated corpus. Both are keyed off the single
//! `LIBCLANG_VERSION` const (the NuGet DLL by version, the headers by the derived `llvmorg-<ver>`
//! git tag), so this tool confirms that pin loads and self-provisions rather than checking a
//! version literal for drift — there is only one to bump.

use std::path::Path;
use windows_clang::LIBCLANG_VERSION;

fn main() {
    // `tool_clang path` prints the directory holding the pinned `libclang.dll` (respecting an
    // existing `LIBCLANG_PATH`). CI's `test.yml` captures it into `LIBCLANG_PATH` for the
    // `test_clang` suite, so the multithreaded test runner never has to call the `unsafe` `set_var`.
    if std::env::args().nth(1).as_deref() == Some("path") {
        if let Some(dir) = std::env::var_os("LIBCLANG_PATH") {
            println!("{}", Path::new(&dir).display());
        } else {
            println!("{}", windows_clang::libclang_dir().display());
        }
        return;
    }

    // Fetch + load the pinned `libclang.dll` and assert it reports `LIBCLANG_VERSION`. This is the
    // same provisioning the scrapers run, so a broken/missing pin fails here rather than mid-scrape.
    windows_clang::ensure_libclang();
    windows_clang::assert_libclang_version();

    println!("clang pin OK: libclang {LIBCLANG_VERSION}");
}
