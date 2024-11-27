//! Regenerates bindings sources for `crates/tests/bindgen`

use std::path::Path;
use windows_bindgen::bindgen;

fn test(args: &str) {
    let mut expand = vec!["--no-comment", "--in", "default", "--flat"];
    expand.extend(args.split_whitespace());
    bindgen(expand);
}

fn test_raw(args: &str) {
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

    // Test interactions with core types and namespaces
    test_raw("--out core_win.rs --no-comment --in default --filter CoCreateGuid");
    test_raw("--out core_win_flat.rs --no-comment --in default --filter CoCreateGuid --flat");
    test_raw("--out core_sys.rs --no-comment --in default --filter CoCreateGuid --sys");
    test_raw("--out core_sys_flat.rs --no-comment --in default --filter CoCreateGuid --sys --flat");
    test_raw(
        "--out core_sys_no_core.rs --no-comment --in default --filter CoCreateGuid --sys --no-core",
    );
    test_raw("--out core_sys_flat_no_core.rs --no-comment --in default --filter CoCreateGuid --sys --flat --no-core");

    // Tests adding custom derived traits for specific types
    test("--out derive_struct.rs --filter DateTime TimeSpan --derive DateTime=PartialOrd");
    test("--out derive_cpp_struct.rs --filter POINT SIZE --derive POINT=PartialOrd");
    test("--out derive_cpp_struct_sys.rs --filter POINT SIZE --sys --derive POINT=Debug");
    test("--out derive_enum.rs --filter AsyncStatus CollectionChange --derive AsyncStatus=PartialOrd");
    test("--out derive_cpp_enum.rs --filter WAIT_EVENT WIN32_ERROR --derive WAIT_EVENT=PartialOrd");
    test("--out derive_edges.rs --filter POINT SIZE --sys --derive POINT=Debug,Eq,PartialEq,PartialOrd,Ord, SIZE=");

    // Tests for enumerations
    test("--out enum_win.rs --filter AsyncStatus");
    test("--out enum_sys.rs --filter AsyncStatus --sys");
    test("--out enum_flags_win.rs --filter ErrorOptions");
    test("--out enum_flags_sys.rs --filter ErrorOptions --sys");
    test("--out enum_cpp_win.rs --filter WAIT_EVENT");
    test("--out enum_cpp_sys.rs --filter WAIT_EVENT --sys");
    test("--out enum_cpp_flags_win.rs --filter GENERIC_ACCESS_RIGHTS");
    test("--out enum_cpp_flags_sys.rs --filter GENERIC_ACCESS_RIGHTS --sys");
    test("--out enum_cpp_scoped_win.rs --filter SECURITY_LOGON_TYPE");
    test("--out enum_cpp_scoped_sys.rs --filter SECURITY_LOGON_TYPE --sys");

    // Tests for structs
    test("--out struct_win.rs --filter RectInt32");
    test("--out struct_sys.rs --filter RectInt32 --sys");
    test("--out struct_cpp_win.rs --filter RECT");
    test("--out struct_cpp_sys.rs --filter RECT --sys");
    test("--out struct_disambiguate.rs --filter Windows.Foundation.Rect");
    test("--out struct_with_generic.rs --filter HttpProgress");
    test("--out struct_with_cpp_interface.rs --filter D3D12_RESOURCE_UAV_BARRIER");
    test("--out struct_with_cpp_interface_sys.rs --filter D3D12_RESOURCE_UAV_BARRIER --sys");

    // Tests for interfaces
    test("--out interface.rs --filter IStringable");
    test("--out interface_sys.rs --filter IStringable --sys");
    test("--out interface_sys_no_core.rs --filter IStringable --sys --no-core");
    test("--out interface_cpp.rs --filter IPersist");
    test("--out interface_cpp_sys.rs --filter IPersist --sys");
    test("--out interface_cpp_sys_no_core.rs --filter IPersist --sys --no-core");
    test("--out interface_cpp_derive.rs --filter IPersistFile");
    test("--out interface_cpp_derive_sys.rs --filter IPersistFile --sys");
    test("--out interface_cpp_return_udt.rs --filter ID2D1Bitmap D2D_SIZE_F");
    test("--out interface_generic.rs --filter IAsyncOperation");
    test("--out interface_required.rs --filter IAsyncAction");
    test("--out interface_required_sys.rs --filter IAsyncAction --sys");
    test("--out interface_required_with_method.rs --filter IAsyncAction AsyncStatus");
    test("--out interface_required_with_method_sys.rs --filter IAsyncAction AsyncStatus --sys");
    test("--out interface_iterable.rs --filter IVector");

    // Tests for functions
    test("--out fn_win.rs --filter GetTickCount");
    test("--out fn_sys.rs --filter GetTickCount --sys");
    test("--out fn_associated_enum_win.rs --filter CoInitializeEx");
    test("--out fn_associated_enum_sys.rs --filter CoInitializeEx --sys");
    test("--out fn_return_void_win.rs --filter GlobalMemoryStatus");
    test("--out fn_return_void_sys.rs --filter GlobalMemoryStatus --sys");
    test("--out fn_no_return_win.rs --filter FatalExit");
    test("--out fn_no_return_sys.rs --filter FatalExit --sys");
    test("--out fn_result_void_sys.rs --filter SetComputerNameA --sys");
    // TODO: this requires BOOL extensions which are currently only in the `windows` crate
    // test("--out fn_result_void_win.rs --filter SetComputerNameA");

    // Tests for delegates
    test("--out delegate.rs --filter DeferralCompletedHandler");
    test("--out delegate_generic.rs --filter EventHandler");

    // Tests for classes
    test("--out class.rs --filter Deferral");
    test("--out class_with_handler.rs --filter Deferral DeferralCompletedHandler");
    test("--out class_static.rs --filter GuidHelper");
    test("--out class_dep.rs --filter WwwFormUrlDecoder");

    // Test for duplicate types
    test("--out multi.rs --filter HTTP_VERSION");
    test("--out multi_sys.rs --filter HTTP_VERSION --sys");

    // Tests for external references e.g. references to other crates
    test("--out reference_windows.rs --filter IMemoryBuffer --reference windows,skip-root,IMemoryBufferReference");
    test("--out reference_dependency.rs --filter IMemoryBufferReference");
    test("--out reference_dependent.rs --filter IMemoryBuffer --reference crate::reference_dependency,flat,IMemoryBufferReference");

    // Tests for dependency tracking
    test("--out deps.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --sys");

    // Tests for header elements
    test_raw("--out comment.rs --in default --filter GetTickCount --sys --flat");
    test_raw(
        "--out comment_no_allow.rs --in default --filter GetTickCount --sys --flat --no-allow",
    );

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
