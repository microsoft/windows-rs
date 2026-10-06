//! Provisions the pinned libclang DLL and resource headers used by `tool-win32` and `tool-webview`.
//! Change `LIBCLANG_VERSION` in `helpers/src/clang.rs`, then run this tool before regenerating.

use helpers::*;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        None => {
            ensure_libclang();
            clang_resource_dir();
            println!("clang pin OK: libclang {LIBCLANG_VERSION} and resource headers");
        }
        Some("path") if args.next().is_none() => {
            println!("{}", ensure_libclang().display());
        }
        _ => panic!("usage: tool-clang [path]"),
    }
}
