//! Regenerate `crates/libs/clang-sys/src/bindings.rs` from the pinned Clang C API and
//! `native/extension.h`.
//!
//! Run `cargo run -p tool-clang-sys` from the repo root. Unless LIBCLANG_PATH is set, the native
//! DLL pair for `windows_clang_sys::LIBCLANG_VERSION` is built first when missing or stale.
//! After changing the pin, validate loading with tool-clang, then regenerate header-derived
//! metadata with tool-win32 and tool-webview and refresh their consumers.

use std::path::Path;
use windows_clang::{Input, Target, ValueKind, capture, merge, projection};

fn main() {
    helpers::ensure_libclang();
    let triple = if cfg!(target_arch = "aarch64") {
        "aarch64-pc-windows-msvc"
    } else {
        "x86_64-pc-windows-msvc"
    };
    let sdk = Path::new("target\\tool-clang\\native")
        .join(helpers::LIBCLANG_VERSION)
        .join(triple)
        .join("llvm");
    let include = std::path::absolute(sdk.join("include")).unwrap();
    let native = std::path::absolute("crates\\libs\\clang-sys\\native").unwrap();
    let scratch = Path::new("target\\tool-clang\\bindings");
    std::fs::create_dir_all(scratch).unwrap();
    let rdl = scratch.join("bindings.rdl");
    let winmd = scratch.join("bindings.winmd");
    let input = Input {
        name: "bindings.h".into(),
        source: "#include <clang-c/Index.h>\n#include \"extension.h\"\n".into(),
        arguments: [
            "-x",
            "c++",
            "-std=c++20",
            "-I",
            &include.display().to_string(),
            "-I",
            &native.display().to_string(),
        ]
        .map(String::from)
        .into(),
        root_paths: Some(vec![include.join("clang-c"), native.join("extension.h")]),
        macros: [
            "CINDEX_VERSION",
            "CINDEX_VERSION_MAJOR",
            "CINDEX_VERSION_MINOR",
        ]
        .map(String::from)
        .into(),
        ..Default::default()
    };
    let snapshots = capture(&[input], &[Target::X64, Target::Arm64, Target::X86]).unwrap();
    // Functions declared by extension.h are exported by the companion DLL.
    let imports = snapshots
        .iter()
        .flat_map(|snapshot| {
            snapshot.values.iter().filter_map(|value| {
                let observation = &value.observations[0];
                matches!(observation.kind, ValueKind::Function { .. }).then(|| {
                    let extension = Path::new(&snapshot.files[observation.location.file.0])
                        .file_name()
                        .is_some_and(|name| name == "extension.h");
                    let library = if extension {
                        "windows_clang.dll"
                    } else {
                        "libclang.dll"
                    };
                    (
                        value.name.clone(),
                        projection::Import {
                            library: library.into(),
                            name: None,
                        },
                    )
                })
            })
        })
        .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Clang", &imports).unwrap();
    std::fs::write(
        &rdl,
        output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect::<String>(),
    )
    .unwrap();
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .input(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Clang")
        .sys()
        .flat()
        .output("crates\\libs\\clang-sys\\src\\bindings.rs")
        .write();
}
