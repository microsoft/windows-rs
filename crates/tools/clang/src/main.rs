//! Validator for the libclang pin owned by the `windows-clang` crate. Like the other
//! `gen`-matrix tools it is run by CI and "validates by running": it writes nothing, so the
//! tree stays clean, and any inconsistency in how libclang is pinned fails the run loudly.
//!
//! The metadata scrapers (`tool_win32`/`tool_wdk`/`tool_winrt`/`tool_webview`) all parse with
//! the pinned libclang, so a drift between `LIBCLANG_VERSION` and the download URLs that fetch it
//! would silently change the generated corpus. This tool is the paired guardian of that pin,
//! exactly as `tool_win32` guards the SDK pin and `tool_wdk` reads it back from `tool_win32`.

use std::path::Path;
use windows_clang::LIBCLANG_VERSION;

/// The `windows-clang` source declaring the libclang pin (version + the download URLs that
/// fetch that exact build).
const PROVISION: &str = "crates/libs/clang/src/provision.rs";

/// The pinned download URL(s), which must embed [`LIBCLANG_VERSION`]. `libclang.dll` comes from the
/// `libclang.runtime.win-<arch>` NuGet packages fetched by version (no URL to drift); only the
/// clang resource-header component is a literal URL.
const PINNED_URLS: &[&str] = &["CLANG_RESOURCE_URL"];

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

    // Each pinned download URL must reference the pinned version, so a version bump that
    // forgets a URL fails here rather than silently fetching a stale libclang.
    for name in PINNED_URLS {
        let url = helpers::read_str_const(PROVISION, name);
        assert!(
            url.contains(LIBCLANG_VERSION),
            "libclang pin drift: `{name}` in `{PROVISION}` does not reference \
             LIBCLANG_VERSION `{LIBCLANG_VERSION}`:\n  {url}"
        );
    }

    println!("clang pin OK: libclang {LIBCLANG_VERSION}");
}
