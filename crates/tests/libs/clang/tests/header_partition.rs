#![cfg(target_pointer_width = "64")]

fn read(dir: &str, leaf: &str) -> String {
    std::fs::read_to_string(format!("{dir}/{leaf}.rdl")).unwrap()
}

#[test]
fn partition_by_defining_header() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_partition", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .library("test.dll")
        .input("partition_input/a.h")
        .input("partition_input/b.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    let shared = read(&scratch, "shared");
    let a = read(&scratch, "a");
    let b = read(&scratch, "b");

    assert!(shared.contains("mod Test {"), "shared.rdl:\n{shared}");
    assert!(a.contains("mod Test {"), "a.rdl:\n{a}");

    assert!(
        shared.contains("type HFOO = *mut void"),
        "shared.rdl:\n{shared}"
    );
    assert!(shared.contains("type PSHARED"), "shared.rdl:\n{shared}");

    assert!(!a.contains("type HFOO"), "a.rdl:\n{a}");
    assert!(!a.contains("type PSHARED"), "a.rdl:\n{a}");
    assert!(!b.contains("type HFOO"), "b.rdl:\n{b}");
    assert!(!b.contains("type PSHARED"), "b.rdl:\n{b}");

    assert!(a.contains("fn AThing"), "a.rdl:\n{a}");
    assert!(b.contains("fn BThing"), "b.rdl:\n{b}");

    assert!(a.contains("PSHARED"), "a.rdl:\n{a}");
    assert!(b.contains("HFOO"), "b.rdl:\n{b}");
    assert!(!a.contains("super::"), "a.rdl:\n{a}");
    assert!(!b.contains("super::"), "b.rdl:\n{b}");

    assert!(
        shared.contains("type LRESULT = i32"),
        "shared.rdl:\n{shared}"
    );
    assert!(b.contains("-> LRESULT"), "b.rdl:\n{b}");
    assert!(!b.contains("type LRESULT"), "b.rdl:\n{b}");

    // Pointer-sized ABI typedefs must remain architecture-neutral primitives.
    assert!(!shared.contains("ULONG_PTR"), "shared.rdl:\n{shared}");
    assert!(!shared.contains("SIZE_T"), "shared.rdl:\n{shared}");
    assert!(!b.contains("SIZE_T"), "b.rdl:\n{b}");
    assert!(b.contains("count: usize"), "b.rdl:\n{b}");
    assert!(b.contains("-> usize"), "b.rdl:\n{b}");
}

#[test]
fn duplicate_typedef_prefers_direct_alias() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_duplicate_typedef", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .input("partition_input/typedef_a.h")
        .input("partition_input/typedef_b.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    let a = read(&scratch, "typedef_a");
    let b = read(&scratch, "typedef_b");
    assert!(
        a.contains("type PUNICODE_STRING = *mut UNICODE_STRING"),
        "typedef_a.rdl:\n{a}"
    );
    assert!(!b.contains("type PUNICODE_STRING"), "typedef_b.rdl:\n{b}");
}

#[test]
fn duplicate_typedef_ignores_excluded_owner() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_duplicate_typedef_excluded", env!("OUT_DIR"));
    if std::path::Path::new(&scratch).exists() {
        std::fs::remove_dir_all(&scratch).unwrap();
    }
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .exclude_header("duplicate_a.h")
        .input("partition_input/duplicate_a.h")
        .input("partition_input/duplicate_b.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    let b = read(&scratch, "duplicate_b");
    assert!(b.contains("type DUPLICATE = i32"), "duplicate_b.rdl:\n{b}");
    assert!(
        !std::path::Path::new(&format!("{scratch}/duplicate_a.rdl")).exists(),
        "excluded header `duplicate_a.h` must not produce `duplicate_a.rdl`"
    );
}

#[test]
fn definition_suppresses_cross_header_opaque_placeholder() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_opaque_definition", env!("OUT_DIR"));
    if std::path::Path::new(&scratch).exists() {
        std::fs::remove_dir_all(&scratch).unwrap();
    }
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .input("partition_input/opaque_forward.h")
        .input("partition_input/opaque_definition.h")
        .input("partition_input/opaque_definition_copy.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    let forward = read(&scratch, "opaque_forward");
    let definition = read(&scratch, "opaque_definition_copy");

    assert!(
        forward.contains("type PSHARED_RECORD = *mut SHARED_RECORD"),
        "opaque_forward.rdl:\n{forward}"
    );
    assert!(
        !forward.contains("struct SHARED_RECORD"),
        "opaque_forward.rdl:\n{forward}"
    );
    assert!(
        definition.contains("struct SHARED_RECORD {"),
        "opaque_definition_copy.rdl:\n{definition}"
    );
    assert!(
        !std::path::Path::new(&format!("{scratch}/opaque_definition.rdl")).exists(),
        "earlier duplicate definition should not produce a partition"
    );
}

// `tool_win32` uses this path to exclude `intsafe.h` from metadata.
#[test]
fn exclude_headers_drops_partition() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_exclude", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .library("test.dll")
        .exclude_header("a.h")
        .input("partition_input/a.h")
        .input("partition_input/b.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    assert!(
        !std::path::Path::new(&format!("{scratch}/a.rdl")).exists(),
        "excluded header `a.h` must not produce `a.rdl`"
    );

    let b = read(&scratch, "b");
    let shared = read(&scratch, "shared");
    assert!(b.contains("fn BThing"), "b.rdl:\n{b}");
    assert!(
        shared.contains("type HFOO = *mut void"),
        "shared.rdl:\n{shared}"
    );
}

// Out-of-scope declarations survive only when referenced by an in-scope declaration.
#[test]
fn scope_sweeps_unreferenced_out_of_scope() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_scope", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .library("test.dll")
        .scope("scope_api")
        .input("partition_input/scope_api/api.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    let api = read(&scratch, "api");
    let crt = read(&scratch, "crt");

    assert!(api.contains("fn ApiCall"), "api.rdl:\n{api}");
    assert!(api.contains("APITYPE"), "api.rdl:\n{api}");

    assert!(crt.contains("type APITYPE = i32"), "crt.rdl:\n{crt}");
    assert!(!crt.contains("CRTNOISE"), "crt.rdl:\n{crt}");
    assert!(!crt.contains("CrtOnly"), "crt.rdl:\n{crt}");
}

#[test]
fn preferred_duplicate_typedef_keeps_pointee_through_scope_sweep() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_scope_duplicate", env!("OUT_DIR"));
    if std::path::Path::new(&scratch).exists() {
        std::fs::remove_dir_all(&scratch).unwrap();
    }
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .scope("scope_api")
        .input("partition_input/scope_api/z_api.h")
        .input("partition_input/scope_crt/a_crt.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    let crt = read(&scratch, "a_crt");
    assert!(crt.contains("type PFOO = *mut FOO"), "a_crt.rdl:\n{crt}");
    assert!(crt.contains("struct FOO"), "a_crt.rdl:\n{crt}");
}

// Dotted WinRT interop header names must map to one flat partition leaf.
#[test]
fn dotted_header_flattens_to_single_partition() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_dotted", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .library("test.dll")
        .input("partition_input/Dotted.Name.Interop.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    let dotted = read(&scratch, "dottednameinterop");
    assert!(
        !std::path::Path::new(&format!("{scratch}/dotted.name.interop.rdl")).exists(),
        "a dotted-leaf rdl must not be produced"
    );

    assert!(dotted.contains("mod Test {"), "dotted.rdl:\n{dotted}");
    assert!(dotted.contains("fn DottedThing"), "dotted.rdl:\n{dotted}");
}

// Canonical ABI projection aliases must map to their `Windows.winmd` references before sweeping.
#[test]
fn abi_projection_type_maps_and_sweeps() {
    let _guard = test_clang::libclang_guard();
    let scratch = format!("{}/header_abi", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .library("test.dll")
        .scope("abi_interop")
        .input("partition_input/abi_interop/interop.h");

    clang
        .namespace("Test")
        .output(&scratch)
        .write_by_header()
        .unwrap();

    let interop = read(&scratch, "interop");
    assert!(
        interop.contains("fn InteropCall"),
        "interop.rdl:\n{interop}"
    );
    assert!(
        !interop.contains("ProjStatus"),
        "the ABI projection type must not leak into the in-scope partition:\n{interop}"
    );

    let proj_path = format!("{scratch}/proj.rdl");
    if let Ok(proj) = std::fs::read_to_string(&proj_path) {
        assert!(!proj.contains("ProjThing"), "proj.rdl:\n{proj}");
    }
}
