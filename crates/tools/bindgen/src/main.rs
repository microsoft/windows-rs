//! Regenerates bindings sources for `crates/tests/bindgen`

use std::path::Path;
use windows_bindgen::bindgen;

fn test(args: &str) {
    let mut expand = vec!["--no-comment", "--in", "default", "--flat"];
    expand.extend(args.split_whitespace());
    bindgen(expand);
}

fn test_raw(args: &str) 
{
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

    test_raw("--no-comment --in default --out core_win.rs --filter CoCreateGuid");
    test_raw("--no-comment --in default --out core_win_flat.rs --filter CoCreateGuid --flat");
    test_raw("--no-comment --in default --out core_sys.rs --filter CoCreateGuid --sys");
    test_raw("--no-comment --in default --out core_sys_flat.rs --filter CoCreateGuid --sys --flat");
    test_raw("--no-comment --in default --out core_sys_no_core.rs --filter CoCreateGuid --sys --no-core");
    test_raw("--no-comment --in default --out core_sys_flat_no_core.rs --filter CoCreateGuid --sys --flat --no-core");

    test("--out derive_struct.rs --filter DateTime TimeSpan --derive DateTime=PartialOrd");
    test("--out derive_cpp_struct.rs --filter POINT SIZE --derive POINT=PartialOrd");
    test("--out derive_cpp_struct_sys.rs --filter POINT SIZE --sys --derive POINT=Debug");
    test("--out derive_enum.rs --filter AsyncStatus CollectionChange --derive AsyncStatus=PartialOrd");
    test("--out derive_cpp_enum.rs --filter WAIT_EVENT WIN32_ERROR --derive WAIT_EVENT=PartialOrd");
    test("--out derive_edges.rs --filter POINT SIZE --sys --derive POINT=Debug,Eq,PartialEq,PartialOrd,Ord, SIZE=");

    test("--out enum_win.rs --filter AsyncStatus ");
    test("--out enum_sys.rs --filter AsyncStatus --sys");
    test("--out enum_flags_win.rs --filter ErrorOptions ");
    test("--out enum_flags_sys.rs --filter ErrorOptions --sys");
    test("--out enum_cpp_win.rs --filter WAIT_EVENT ");
    test("--out enum_cpp_sys.rs --filter WAIT_EVENT --sys");
    test("--out enum_cpp_flags_win.rs --filter GENERIC_ACCESS_RIGHTS ");
    test("--out enum_cpp_flags_sys.rs --filter GENERIC_ACCESS_RIGHTS --sys");
    test("--out enum_cpp_scoped_win.rs --filter SECURITY_LOGON_TYPE ");
    test("--out enum_cpp_scoped_sys.rs --filter SECURITY_LOGON_TYPE --sys");

    test("--out struct_win.rs --filter RectInt32 ");
    test("--out struct_sys.rs --filter RectInt32 --sys");
    test("--out struct_cpp_win.rs --filter RECT ");
    test("--out struct_cpp_sys.rs --filter RECT --sys");
    test("--out struct_disambiguate.rs --filter Windows.Foundation.Rect ");
    test("--out struct_with_generic.rs --filter HttpProgress ");
    test("--out struct_with_cpp_interface.rs --filter D3D12_RESOURCE_UAV_BARRIER ");
    test("--out struct_with_cpp_interface_sys.rs --filter D3D12_RESOURCE_UAV_BARRIER --sys");

    test("--out interface.rs --filter Windows.Foundation.IStringable ");
    test("--out interface_sys.rs --filter Windows.Foundation.IStringable --sys");
    test("--out interface_sys_no_core.rs --filter Windows.Foundation.IStringable --sys --no-core");
    test("--out interface_cpp.rs --filter IPersist ");
    test("--out interface_cpp_sys.rs --filter IPersist --sys");
    test("--out interface_cpp_sys_no_core.rs --filter IPersist --sys --no-core");
    test("--out interface_cpp_derive.rs --filter IPersistFile ");
    test("--out interface_cpp_derive_sys.rs --filter IPersistFile --sys");
    test("--out interface_cpp_return_udt.rs --filter ID2D1Bitmap D2D_SIZE_F ");
    test("--out interface_generic.rs --filter Windows.Foundation.IAsyncOperation ");
    test("--out interface_required.rs --filter Windows.Foundation.IAsyncAction ");
    test("--out interface_required_sys.rs --filter Windows.Foundation.IAsyncAction --sys");
    test("--out interface_required_with_method.rs --filter Windows.Foundation.IAsyncAction AsyncStatus ");
    test("--out interface_required_with_method_sys.rs --filter Windows.Foundation.IAsyncAction AsyncStatus --sys");
    test("--out interface_iterable.rs --filter IVector ");

    test("--out fn_win.rs --filter GetTickCount ");
    test("--out fn_sys.rs --filter GetTickCount --sys");
    test("--out fn_associated_enum_win.rs --filter CoInitializeEx ");
    test("--out fn_associated_enum_sys.rs --filter CoInitializeEx --sys ");
    test("--out fn_return_void_win.rs --filter GlobalMemoryStatus ");
    test("--out fn_return_void_sys.rs --filter GlobalMemoryStatus --sys");
    test("--out fn_no_return_win.rs --filter FatalExit ");
    test("--out fn_no_return_sys.rs --filter FatalExit --sys");
    test("--out fn_result_void_sys.rs --filter SetComputerNameA --sys");
    // TODO: this requires BOOL extensions which are currently only in the `windows` crate
    // test("--out fn_result_void_win.rs --filter SetComputerNameA ");

    test("--out delegate.rs --filter Windows.Foundation.DeferralCompletedHandler ");
    test("--out delegate_generic.rs --filter Windows.Foundation.EventHandler ");

    test("--out class_with_handler.rs --filter Deferral DeferralCompletedHandler ");
    test("--out class_without_handler.rs --filter Deferral ");

    test("--out multi.rs --filter HTTP_VERSION ");
    test("--out multi_sys.rs --filter HTTP_VERSION --sys");

    test("--out reference.rs --filter IMemoryBuffer --reference windows,skip-root,IMemoryBufferReference");

    test("--out deps.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --sys");

    // TODO: need to test 3rd party package support and make sure we can compose code gen from different sources
    // test("--out package --filter WwwFormUrlDecoder --package");

    // test("--out winrt_class.rs --filter Windows.Foundation.Deferral ");
    // test("--out winrt_class_static.rs --filter Windows.Foundation.GuidHelper ");

    // test("--out winrt_class_deps.rs --filter WwwFormUrlDecoder ");

    // // Tree version
    // test("--out winrt_class_deps2.rs --filter WwwFormUrlDecoder");

    // // TODO: what does minimal do for methods?
    // test("--out winrt_class_deps3.rs --filter WwwFormUrlDecoder --minimal");

    // // This should exclude WwwFormUrlDecoder's dependencies on the collection interfaces
    // test("--out winrt_class_deps4.rs --filter WwwFormUrlDecoder !Windows.Foundation.Collections ");

    // test("--out winrt_class_uri.rs --filter Uri ");
    // test("--out winrt_class_uri_no_decoder.rs --filter Uri !WwwFormUrlDecoder ");

    // TODO test class with generic default interface
    // TODO test class with async default interface
    // TODO test class with collection default interface

    // TODO: probably want to avoid generating dependencies on windows and windows-sys - just windows-core/result/string/targets
    // 1. so in that model --sys bindings imply and the latter is only useful for removing the above dependencies for non-sys bindings

    // Same as 'deps3.rs' but with dependency on `windows-core` for core types. TODO: what about other types found in windows?
    // test("--out deps4.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS");

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
