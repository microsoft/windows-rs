//! Regenerates bindings sources for `crates/tests/bindgen`

use std::path::Path;
use windows_bindgen::bindgen;

fn test(args: &str) {
    let mut expand = vec!["--no-comment", "--in", "default"];
    expand.extend(args.split_whitespace());
    bindgen(expand);
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

    test("--out core_win.rs --filter CoCreateGuid");
    test("--out core_win_flat.rs --filter CoCreateGuid --flat");
    test("--out core_sys.rs --filter CoCreateGuid --sys");
    test("--out core_sys_flat.rs --filter CoCreateGuid --sys --flat");
    test("--out core_sys_no_core.rs --filter CoCreateGuid --sys --no-core");
    test("--out core_sys_flat_no_core.rs --filter CoCreateGuid --sys --flat --no-core");

    // TODO: test derive on different types and sys/win style
    test("--out derive.rs --filter DateTime TimeSpan --sys --flat --derive DateTime=PartialOrd");

    test("--out class_factory.rs --filter IClassFactory --flat");
    test("--out class_factory_sys.rs --filter IClassFactory --sys --flat");

    //test("--out class_factory_no_deps.rs --filter IClassFactory --flat");
    //test("--out class_factory_sys_no_deps.rs --filter IClassFactory --sys --flat");

    test("--out multi.rs --filter HTTP_VERSION  --flat");

    // Very minimal example of generating just a single item.
    test("--out iota.rs --filter GetTickCount --sys --flat --no-allow");

    // Same as 'iota.rs' but without `--sys`.
    test("--out cpp_fn_return_none.rs --filter GetTickCount --flat");

    test("--out cpp_fn_associated_enum_sys.rs --filter CoInitializeEx --sys --flat --no-allow");
    test("--out cpp_fn_associated_enum_win.rs --filter CoInitializeEx --flat");

    test("--out cpp_fn_return_void.rs --filter GlobalMemoryStatus --flat");

    test("--out cpp_interface.rs --filter IPersist --flat");
    test("--out cpp_interface2.rs --filter IPersistFile --flat");

    test("--out udt_return_interface.rs --filter ID2D1Bitmap D2D_SIZE_F --flat");

    // test("--out cpp_interface_sys.rs --filter IPersist --flat --sys");

    // test("--out cpp_fn_result_void.rs --filter SetComputerNameA --flat");

    // Generate functions and include dependencies automatically.
    test("--out deps.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --sys --flat");

    // Same as 'deps.rs' but with namespace/module structure due to lack of "--flat" option.
    test("--out deps2.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --sys");

    // TODO: for winrt we could have dedicated .idl files to test the various edge cases like dependencies and other
    // scenarios that are hard to come by.
    test("--out winrt_struct.rs --filter Windows.Foundation.Rect --flat");
    test("--out winrt_struct_with_generic.rs --filter HttpProgress --flat");

    test("--out winrt_enum.rs --filter Windows.Foundation.AsyncStatus --flat");

    test("--out winrt_interface.rs --filter Windows.Foundation.IStringable --flat");
    test("--out winrt_interface_generic.rs --filter Windows.Foundation.IAsyncOperation --flat");
    //test("--out winrt_interface_generic2.rs --filter Windows.Foundation.Collections.IVector --flat");
    test("--out winrt_interface_required.rs --filter Windows.Foundation.IAsyncAction --flat");

    test("--out winrt_delegate.rs --filter Windows.Foundation.DeferralCompletedHandler --flat");
    test("--out winrt_delegate_generic.rs --filter Windows.Foundation.EventHandler --flat");

    test("--out winrt_interface_no_status.rs --filter IAsyncInfo --flat");
    test("--out winrt_interface_status.rs --filter IAsyncInfo AsyncStatus --flat");

    test("--out winrt_class_with_handler.rs --filter Deferral DeferralCompletedHandler --flat");
    test("--out winrt_class_without_handler.rs --filter Deferral --flat");

    test("--out reference.rs --filter IMemoryBuffer --flat --reference name=windows,style=skip-root,path=IMemoryBufferReference");

    // TODO: need to test 3rd party package support and make sure we can compose code gen from different sources
    // test("--out package --filter WwwFormUrlDecoder --package");

    // test("--out winrt_class.rs --filter Windows.Foundation.Deferral --flat");
    // test("--out winrt_class_static.rs --filter Windows.Foundation.GuidHelper --flat");

    // test("--out winrt_class_deps.rs --filter WwwFormUrlDecoder --flat");

    // // Tree version
    // test("--out winrt_class_deps2.rs --filter WwwFormUrlDecoder");

    // // TODO: what does minimal do for methods?
    // test("--out winrt_class_deps3.rs --filter WwwFormUrlDecoder --flat --minimal");

    // // This should exclude WwwFormUrlDecoder's dependencies on the collection interfaces
    // test("--out winrt_class_deps4.rs --filter WwwFormUrlDecoder !Windows.Foundation.Collections --flat");

    // test("--out winrt_class_uri.rs --filter Uri --flat");
    // test("--out winrt_class_uri_no_decoder.rs --filter Uri !WwwFormUrlDecoder --flat");

    // TODO test class with generic default interface
    // TODO test class with async default interface
    // TODO test class with collection default interface

    // TODO: probably want to avoid generating dependencies on windows and windows-sys - just windows-core/result/string/targets
    // 1. so in that model --sys bindings imply and the latter is only useful for removing the above dependencies for non-sys bindings

    // Same as 'deps3.rs' but with dependency on `windows-core` for core types. TODO: what about other types found in windows?
    // test("--out deps4.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS");

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
