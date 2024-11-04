//! Regenerates bindings sources for `crates/tests/bindgen`

use std::path::Path;
use windows_bindgen2::bindgen;

fn test(args: &str) {
    bindgen(args.split_whitespace());
}

fn main() {
    if !Path::new("crates/tests/bindgen/Cargo.toml").exists() {
        println!("This tool must be run from the root of the repo.");
        std::process::exit(1);
    }

    std::fs::remove_dir_all("crates/tests/bindgen/src").unwrap();
    std::fs::create_dir_all("crates/tests/bindgen/src").unwrap();
    std::env::set_current_dir("crates/tests/bindgen/src").unwrap();
    std::fs::write("lib.rs", "").unwrap();

    // Very minimal example of generating just a single item.
    test("--out iota.rs --filter GetTickCount --sys --flat --no-comment --no-allow");

    // Same as 'iota.rs' but without `--sys`.
    test("--out cpp_fn_return_none.rs --filter GetTickCount --flat --no-comment");

    test("--out cpp_fn_return_void.rs --filter GlobalMemoryStatus --flat --no-comment");

    test("--out cpp_fn_result_void.rs --filter SetComputerNameA --flat --no-comment");

    // Generate functions and include dependencies automatically.
    test("--out deps.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --sys --flat --no-deps --no-comment");

    // Same as 'deps.rs' but with namespace/module structure due to lack of "--flat" option.
    test("--out deps2.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --sys --no-deps --no-comment");

    // Same as 'deps2.rs' but `--no-deps` is implied.
    test("--out deps3.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --sys --no-comment");

    // TODO: for winrt we could have dedicated .idl files to test the various edge cases like dependencies and other
    // scenarios that are hard to come by.
    test("--out winrt_struct.rs --filter Windows.Foundation.Rect --flat --no-comment");
    test("--out winrt_struct_with_generic.rs --filter HttpProgress --flat --no-comment");

    test("--out winrt_enum.rs --filter Windows.Foundation.AsyncStatus --flat --no-comment");

    test("--out winrt_interface.rs --filter Windows.Foundation.IStringable --flat --no-comment");
    test("--out winrt_interface_generic.rs --filter Windows.Foundation.IAsyncOperation --flat --no-comment");
    test("--out winrt_interface_generic2.rs --filter Windows.Foundation.Collections.IVector --flat --no-comment");
    test("--out winrt_interface_required.rs --filter Windows.Foundation.IAsyncAction --flat --no-comment");

    test("--out winrt_delegate.rs --filter Windows.Foundation.DeferralCompletedHandler --flat --no-comment");
    test("--out winrt_delegate_generic.rs --filter Windows.Foundation.EventHandler --flat --no-comment");

    test("--out winrt_interface_no_status.rs --filter IAsyncInfo --flat --no-comment");
    test("--out winrt_interface_status.rs --filter IAsyncInfo AsyncStatus --flat --no-comment");

    test("--out winrt_class_with_handler.rs --filter Deferral DeferralCompletedHandler --flat --no-comment");
    test("--out winrt_class_without_handler.rs --filter Deferral --flat --no-comment");

    // TODO: need to test 3rd party package support and make sure we can compose code gen from different sources
    // test("--out package --filter WwwFormUrlDecoder --package");

    // test("--out winrt_class.rs --filter Windows.Foundation.Deferral --flat --no-comment");
    // test("--out winrt_class_static.rs --filter Windows.Foundation.GuidHelper --flat --no-comment");

    // test("--out winrt_class_deps.rs --filter WwwFormUrlDecoder --flat --no-comment");

    // // Tree version
    // test("--out winrt_class_deps2.rs --filter WwwFormUrlDecoder --no-comment");

    // // TODO: what does minimal do for methods?
    // test("--out winrt_class_deps3.rs --filter WwwFormUrlDecoder --flat --minimal --no-comment");

    // // This should exclude WwwFormUrlDecoder's dependencies on the collection interfaces
    // test("--out winrt_class_deps4.rs --filter WwwFormUrlDecoder !Windows.Foundation.Collections --flat --no-comment");

    // test("--out winrt_class_uri.rs --filter Uri --flat --no-comment");
    // test("--out winrt_class_uri_no_decoder.rs --filter Uri !WwwFormUrlDecoder --flat --no-comment");

    // TODO test class with generic default interface
    // TODO test class with async default interface
    // TODO test class with collection default interface

    // TODO: probably want to avoid generating dependencies on windows and windows-sys - just windows-core/result/string/targets
    // 1. so in that model --sys bindings imply --no-deps and the latter is only useful for removing the above dependencies for non-sys bindings

    // Same as 'deps3.rs' but with dependency on `windows-core` for core types. TODO: what about other types found in windows?
    // test("--out deps4.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --no-comment");

    // TODO: test with path using white space

    // TODO: test failure/panics

    write_lib();
}

// The tool_bindgen crate generates the modules for this crate
// while this build script generates the lib file that includes
// those modules.

fn write_lib() {
    let mods = std::fs::read_dir(".")
        .unwrap()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                let path = path.file_name().unwrap().to_string_lossy();

                if path == "lib.rs" {
                    None
                } else {
                    Some(format!("\npub mod {};", path.strip_suffix(".rs").unwrap()))
                }
            } else {
                None
            }
        });

    let mut lib_rs = "// This package was generated by `tool_bindgen`.\n".to_string();
    lib_rs.extend(mods);
    lib_rs.push('\n');
    std::fs::write("lib.rs", lib_rs).unwrap();

    let mut cmd = std::process::Command::new("rustfmt");
    cmd.arg("lib.rs");

    if !cmd.status().unwrap().success() {
        panic!("Failed to run rustfmt");
    }
}
