//! Validator for the libclang pin owned by windows-clang-sys. Like the other
//! `gen`-matrix tools it is run by CI and "validates by running": it writes nothing, so the
//! tree stays clean, and any inconsistency in how libclang is pinned fails the run loudly.
//!
//! The metadata scrapers (`tool-win32`/`tool-winrt`/`tool-webview`) all parse with
//! the pinned libclang, so a drift between the loaded `libclang.dll` and the clang builtin
//! resource headers would silently change the generated metadata. Both are keyed off the single
//! `LIBCLANG_VERSION` const (the native SDK by version, the headers by the derived `llvmorg-<ver>`
//! git tag), so this tool confirms that pin loads and self-provisions rather than checking a
//! version literal for drift - there is only one to bump.

use helpers::LIBCLANG_VERSION;
use std::path::Path;

fn main() {
    helpers::ensure_libclang();
    helpers::assert_libclang_version();

    // `tool-clang path` prints the directory holding the pinned `libclang.dll` (respecting an
    // existing `LIBCLANG_PATH`). CI's `test.yml` captures it into `LIBCLANG_PATH` for the
    // `windows-clang` tests, so the multithreaded test runner never calls the unsafe `set_var`.
    if std::env::args().nth(1).as_deref() == Some("path") {
        let dir = std::env::var_os("LIBCLANG_PATH").unwrap();
        println!("{}", Path::new(&dir).display());
        return;
    }

    println!("clang pin OK: libclang {LIBCLANG_VERSION}");
}
