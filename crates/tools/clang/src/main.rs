//! Provision and validate the native DLL pair pinned by `windows_clang_sys::LIBCLANG_VERSION`.
//!
//! `cargo run -p tool-clang` checks loading, LLVM version, and companion ABI without changing
//! tracked files. `cargo run -q -p tool-clang -- path` prints LIBCLANG_PATH after validation.
//! Provisioning can rebuild the native cache; an explicit path is caller-owned.
//! After changing the LLVM pin, rebuild and regenerate bindings with tool-clang-sys first.

use helpers::LIBCLANG_VERSION;
use std::path::Path;

fn main() {
    helpers::ensure_libclang();
    helpers::assert_libclang_version();

    // CI captures this value before starting the multithreaded test runner, which cannot
    // safely set process environment variables itself.
    if std::env::args().nth(1).as_deref() == Some("path") {
        let dir = std::env::var_os("LIBCLANG_PATH").unwrap();
        println!("{}", Path::new(&dir).display());
        return;
    }

    println!("clang pin OK: libclang {LIBCLANG_VERSION}");
}
