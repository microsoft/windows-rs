//! Regenerates bindings sources for `crates/tests/libs/bindgen`

use std::path::Path;
use windows_bindgen::bindgen;

fn test(args: &str) {
    let mut expand = vec!["--no-comment", "--in", "default", "--flat"];
    expand.extend(args.split_whitespace());
    _ = bindgen(expand);
}

fn test_raw(args: &str) {
    _ = bindgen(args.split_whitespace());
}

fn main() {
    let time = std::time::Instant::now();

    if !Path::new("crates/tests/libs/bindgen/Cargo.toml").exists() {
        println!("This tool must be run from the root of the repo.");
        std::process::exit(1);
    }

    std::fs::remove_dir_all("crates/tests/libs/bindgen/src").unwrap();
    std::fs::create_dir_all("crates/tests/libs/bindgen/src").unwrap();
    std::env::set_current_dir("crates/tests/libs/bindgen/src").unwrap();
    std::fs::write("lib.rs", "").unwrap();

    // Test interactions with core types and namespaces
    test_raw("--out core_win.rs --no-comment --in default --filter CoCreateGuid");
    test_raw("--out core_win_flat.rs --no-comment --in default --filter CoCreateGuid --flat");
    test_raw("--out core_sys.rs --no-comment --in default --filter CoCreateGuid --sys");
    test_raw("--out core_sys_flat.rs --no-comment --in default --filter CoCreateGuid --sys --flat");
    test_raw(
        "--out core_sys_no_core.rs --no-comment --in default --filter CoCreateGuid --sys --no-deps",
    );
    test_raw("--out core_sys_flat_no_core.rs --no-comment --in default --filter CoCreateGuid --sys --flat --no-deps");

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
    test("--out struct_arch_a.rs --filter SP_POWERMESSAGEWAKE_PARAMS_A");
    test("--out struct_arch_w.rs --filter SP_POWERMESSAGEWAKE_PARAMS_W");
    test("--out struct_arch_a_sys.rs --sys --filter SP_POWERMESSAGEWAKE_PARAMS_A");
    test("--out struct_arch_w_sys.rs --sys --filter SP_POWERMESSAGEWAKE_PARAMS_W");

    // Tests for interfaces
    test("--out interface.rs --filter IStringable");
    test("--out interface_sys.rs --filter IStringable --sys");
    test("--out interface_sys_no_core.rs --filter IStringable --sys --no-deps");
    test("--out interface_cpp.rs --filter IPersist");
    test("--out interface_cpp_sys.rs --filter IPersist --sys");
    test("--out interface_cpp_sys_no_core.rs --filter IPersist --sys --no-deps");
    test("--out interface_cpp_derive.rs --filter IPersistFile");
    test("--out interface_cpp_derive_sys.rs --filter IPersistFile --sys");
    test("--out interface_cpp_return_udt.rs --filter ID2D1Bitmap D2D_SIZE_F");
    test("--out interface_generic.rs --filter IAsyncOperation --no-deps");
    test("--out interface_required.rs --filter IAsyncAction --no-deps");
    test("--out interface_required_sys.rs --filter IAsyncAction --sys");
    test("--out interface_required_with_method.rs --filter IAsyncAction AsyncStatus --no-deps");
    test("--out interface_required_with_method_sys.rs --filter IAsyncAction AsyncStatus --sys");
    test("--out interface_iterable.rs --filter IVector --no-deps");
    test("--out interface_array_return.rs --filter IDispatch");

    // Tests for functions
    test("--out fn_win.rs --filter GetTickCount");
    test("--out fn_sys.rs --filter GetTickCount --sys");
    test("--out fn_sys_targets.rs --filter GetTickCount --sys --link windows_targets");
    test("--out fn_associated_enum_win.rs --filter CoInitializeEx");
    test("--out fn_associated_enum_sys.rs --filter CoInitializeEx --sys");
    test("--out fn_return_void_win.rs --filter GlobalMemoryStatus");
    test("--out fn_return_void_sys.rs --filter GlobalMemoryStatus --sys");
    test("--out fn_no_return_win.rs --filter FatalExit");
    test("--out fn_no_return_sys.rs --filter FatalExit --sys");
    test("--out fn_result_void_sys.rs --filter SetComputerNameA --sys");

    // Tests for delegates
    test("--out delegate.rs --filter DeferralCompletedHandler");
    test("--out delegate_generic.rs --filter EventHandler");
    test("--out delegate_cpp.rs --filter GetProcAddress EnumWindows");
    test("--out delegate_cpp_ref.rs --filter PFN_D3D12_CREATE_DEVICE PFNGETACTIVATIONFACTORY S_OK CLASS_E_CLASSNOTAVAILABLE");
    test("--out delegate_param.rs --filter SetConsoleCtrlHandler");

    // Tests for classes
    test("--out class.rs --filter Deferral");
    test("--out class_with_handler.rs --filter Deferral DeferralCompletedHandler");
    test("--out class_static.rs --filter GuidHelper");
    test("--out class_dep.rs --filter WwwFormUrlDecoder --no-deps");

    // Tests for duplicate types
    test("--out multi.rs --filter HTTP_VERSION");
    test("--out multi_sys.rs --filter HTTP_VERSION --sys");

    // Tests for "window long" overloads
    test("--out window_long_get_a.rs --filter GetWindowLongPtrA");
    test("--out window_long_get_w.rs --filter GetWindowLongPtrW");
    test("--out window_long_set_a.rs --filter SetWindowLongPtrA");
    test("--out window_long_set_w.rs --filter SetWindowLongPtrW");

    test("--out window_long_get_a_sys.rs --filter GetWindowLongPtrA --sys");
    test("--out window_long_get_w_sys.rs --filter GetWindowLongPtrW --sys");
    test("--out window_long_set_a_sys.rs --filter SetWindowLongPtrA --sys");
    test("--out window_long_set_w_sys.rs --filter SetWindowLongPtrW --sys");

    // Tests for struct references
    test("--out reference_struct_filter.rs --filter InkTrailPoint");
    test("--out reference_struct_reference_type.rs --filter InkTrailPoint --reference windows,skip-root,Point");
    test("--out reference_struct_reference_namespace.rs --filter InkTrailPoint --reference windows,skip-root,Windows.Foundation");
    test("--out reference_struct_sys_filter.rs --sys --filter GAMING_DEVICE_MODEL_INFORMATION");
    test("--out reference_struct_sys_reference_type.rs --sys --filter GAMING_DEVICE_MODEL_INFORMATION --reference windows_sys,skip-root,GAMING_DEVICE_VENDOR_ID");
    test("--out reference_struct_sys_reference_namespace.rs --sys --filter GAMING_DEVICE_MODEL_INFORMATION --reference windows_sys,skip-root,Windows.Win32.Gaming");

    // Tests for BOOL and BOOLEAN parameters
    test("--out bool.rs --filter EnableMouseInPointer");
    test("--out bool_sys.rs --filter EnableMouseInPointer --sys");
    test("--out bool_sys_no_core.rs --filter EnableMouseInPointer --sys --no-deps");
    test("--out bool_event.rs --filter CreateEventW SetEvent NtWaitForSingleObject WaitForSingleObjectEx --reference windows,skip-root,Windows");
    test("--out bool_event_sans_reference.rs --filter CreateEventW SetEvent NtWaitForSingleObject WaitForSingleObjectEx");

    // Tests Ref and OutRef for COM interfaces since we can't use MIDLRT for that.
    test("--out ref_params.rs --filter IDynamicConceptProviderConcept IModelObject IKeyStore");

    // Tests simulating reference dependency and dependent
    test_raw(
        "--no-comment --out reference_dependency_flat.rs --filter IMemoryBufferReference --flat",
    );
    test_raw("--no-comment --out reference_dependency_full.rs --filter IMemoryBufferReference");
    test_raw(
        "--no-comment --out reference_dependency_skip_root.rs --filter IMemoryBufferReference",
    );
    test_raw("--no-comment --out reference_dependent_flat.rs --filter IMemoryBuffer --reference crate::reference_dependency_flat,flat,IMemoryBufferReference");
    test_raw("--no-comment --out reference_dependent_full.rs --filter IMemoryBuffer --reference crate::reference_dependency_full,full,IMemoryBufferReference");
    test_raw("--no-comment --out reference_dependent_skip_root.rs --filter IMemoryBuffer --reference crate::reference_dependency_skip_root::Windows,skip-root,IMemoryBufferReference");

    // Tests for dependency tracking
    test("--out deps.rs --filter FreeLibrary GetProcAddress LoadLibraryExA LOAD_LIBRARY_SEARCH_DEFAULT_DIRS --sys");

    // Tests for sort order
    test("--out sort.rs --filter E_FAIL S_FALSE E_OUTOFMEMORY ERROR_OUTOFMEMORY Windows.Foundation.Rect RECT GetTickCount FatalExit --sys --no-deps");

    // Tests input defaults
    test_raw(
        "--out default_default.rs --filter GetTickCount --sys --flat --no-comment --in default",
    );
    test_raw("--out default_assumed.rs --filter GetTickCount --sys --flat --no-comment");

    // Tests for header elements
    test_raw("--out comment.rs --in default --filter GetTickCount --sys --flat");
    test_raw(
        "--out comment_no_allow.rs --in default --filter GetTickCount --sys --flat --no-allow",
    );

    // Tests for rustfmt
    bindgen("--out rustfmt_25.rs --rustfmt max_width=25,newline_style=Unix --filter POINT --flat --no-comment --no-allow".split_whitespace()).unwrap();

    write_lib();
    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

// The tool_bindgen crate generates the modules for this crate
// while this build script generates the lib file that includes
// those modules.

fn write_lib() {
    let mut mods: Vec<String> = std::fs::read_dir(".")
        .unwrap()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                let path = path.file_name().unwrap().to_string_lossy();

                if path == "lib.rs" {
                    None
                } else if path.starts_with("rustfmt") {
                    Some(format!(
                        "\n#[rustfmt::skip] pub mod {};",
                        path.strip_suffix(".rs").unwrap()
                    ))
                } else {
                    Some(format!("\npub mod {};", path.strip_suffix(".rs").unwrap()))
                }
            } else {
                None
            }
        })
        .collect();

    mods.sort();

    let mut lib_rs = "// This package was generated by `tool_bindgen`.\n".to_string();
    lib_rs.extend(mods);
    lib_rs.push('\n');
    std::fs::write("lib.rs", lib_rs).unwrap();

    let mut cmd = std::process::Command::new("rustfmt");
    cmd.arg("lib.rs");

    if !cmd.status().unwrap().success() {
        panic!("failed to run rustfmt");
    }
}
