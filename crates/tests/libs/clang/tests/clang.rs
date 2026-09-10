#![cfg(target_pointer_width = "64")]

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

#[test]
fn reference_rejects_non_winmd_input() {
    let error = windows_clang::clang()
        .reference("reference.rdl")
        .output("unused.rdl")
        .write()
        .unwrap_err();
    assert_eq!(error.message, "expected .winmd file");
}

#[test]
fn accepts_c_and_cpp_header_extensions() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("header_extensions");
    std::fs::create_dir_all(&scratch).unwrap();

    let headers = [
        ("one.H", "typedef int HeaderOne;"),
        ("two.HpP", "typedef int HeaderTwo;"),
        ("three.hXx", "typedef int HeaderThree;"),
        ("four.HH", "typedef int HeaderFour;"),
    ];
    let paths: Vec<_> = headers
        .iter()
        .map(|(name, contents)| {
            let path = scratch.join(name);
            std::fs::write(&path, contents).unwrap();
            path
        })
        .collect();

    let _guard = test_clang::libclang_guard();

    let explicit_output = scratch.join("explicit.rdl");
    windows_clang::clang()
        .inputs(&paths)
        .args(["-x", "c++"])
        .output(&explicit_output)
        .namespace("Test")
        .write()
        .unwrap();

    let directory_output = scratch.join("directory.rdl");
    windows_clang::clang()
        .input(&scratch)
        .args(["-x", "c++"])
        .output(&directory_output)
        .namespace("Test")
        .write()
        .unwrap();

    let explicit = std::fs::read_to_string(explicit_output).unwrap();
    let directory = std::fs::read_to_string(directory_output).unwrap();
    for name in ["HeaderOne", "HeaderTwo", "HeaderThree", "HeaderFour"] {
        assert!(explicit.contains(name));
        assert!(directory.contains(name));
    }
}

#[test]
fn rejects_non_header_input_extension() {
    let error = windows_clang::clang()
        .input("input.txt")
        .output("unused.rdl")
        .write()
        .unwrap_err();
    assert_eq!(error.message, "expected .h, .hpp, .hxx, or .hh file");
}

#[test]
fn terminals_require_output() {
    let write = windows_clang::clang().write().unwrap_err();
    assert_eq!(write.message, "output is required");

    let partition = windows_clang::clang().write_by_header().unwrap_err();
    assert_eq!(partition.message, "output is required");
}

#[test]
fn malformed_metadata_reports_its_role() {
    let reference = windows_clang::clang()
        .reference_bytes(b"not metadata")
        .output("unused.rdl")
        .write()
        .unwrap_err();
    assert_eq!(reference.message, "invalid reference");
    assert_eq!(reference.file_name, "<memory>");

    let resolution = windows_clang::clang()
        .resolution_bytes(b"not metadata")
        .output("unused")
        .write_by_header()
        .unwrap_err();
    assert_eq!(resolution.message, "invalid resolution input");
    assert_eq!(resolution.file_name, "<memory>");
}

#[test]
fn namespaced_hresult_survives_the_binding_round_trip() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("namespaced_hresult");
    std::fs::create_dir_all(&scratch).unwrap();

    let direct_header = scratch.join("direct.h");
    std::fs::write(
        &direct_header,
        "typedef long HRESULT;\n\
         typedef struct _GUID { unsigned long Data1; } GUID;\n\
         typedef const GUID *REFIID;\n\
         #define DIRECT_STATUS ((HRESULT)0)\n\
         HRESULT DirectStatus(void);\n\
         HRESULT CreateThing(REFIID iid, void **object);",
    )
    .unwrap();

    let included_header = scratch.join("dependency.h");
    std::fs::write(&included_header, "typedef long HRESULT;").unwrap();
    let main_header = scratch.join("included.h");
    std::fs::write(
        &main_header,
        "#include \"dependency.h\"\n\
         #define INCLUDED_STATUS ((HRESULT)0)\n\
         HRESULT IncludedStatus(void);",
    )
    .unwrap();

    let hostile_reference = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input_text("#[win32] mod Other { type HRESULT = i16; }")
        .output(&hostile_reference)
        .write()
        .unwrap();

    let direct_rdl = scratch.join("direct.rdl");
    let included_rdl = scratch.join("included.rdl");
    {
        let _guard = test_clang::libclang_guard();

        windows_clang::clang()
            .input(&direct_header)
            .output(&direct_rdl)
            .namespace("Direct")
            .library("test.dll")
            .write()
            .unwrap();

        windows_clang::clang()
            .input(&main_header)
            .reference(&hostile_reference)
            .output(&included_rdl)
            .namespace("Included")
            .library("test.dll")
            .write()
            .unwrap();
    }

    for (rdl, function, constant) in [
        (&direct_rdl, "DirectStatus", "DIRECT_STATUS"),
        (&included_rdl, "IncludedStatus", "INCLUDED_STATUS"),
    ] {
        let contents = std::fs::read_to_string(rdl).unwrap();
        assert!(!contents.contains("type HRESULT"));
        assert!(contents.contains(&format!("fn {function}() -> HRESULT")));
        assert!(contents.contains(&format!("const {constant}: HRESULT")));
    }
    let included = std::fs::read_to_string(&included_rdl).unwrap();
    assert!(!included.contains("Other::HRESULT"));
    let direct = std::fs::read_to_string(&direct_rdl).unwrap();
    assert!(direct.contains("#[iid_is] object: *mut *mut void"));

    let flat_dir = scratch.join("flat");
    std::fs::create_dir_all(&flat_dir).unwrap();
    {
        let _guard = test_clang::libclang_guard();
        windows_clang::clang()
            .input(&direct_header)
            .output(&flat_dir)
            .namespace("Windows.Win32")
            .write_by_header()
            .unwrap();
    }
    let flat = std::fs::read_to_string(flat_dir.join("direct.rdl")).unwrap();
    assert!(!flat.contains("type HRESULT"));
    assert!(flat.contains("fn DirectStatus() -> HRESULT"));
    assert!(flat.contains("const DIRECT_STATUS: HRESULT"));

    let winmd = scratch.join("out.winmd");
    windows_rdl::reader()
        .input(&direct_rdl)
        .input(&included_rdl)
        .output(&winmd)
        .write()
        .unwrap();

    let rich = scratch.join("rich.rs");
    windows_bindgen::bindgen([
        "--in",
        winmd.to_str().unwrap(),
        "--out",
        rich.to_str().unwrap(),
        "--filter",
        "Direct",
        "--filter",
        "Included",
    ]);
    let rich = std::fs::read_to_string(rich).unwrap();
    assert!(rich.contains("fn DirectStatus() -> windows_core::HRESULT"));
    assert!(rich.contains("fn IncludedStatus() -> windows_core::HRESULT"));

    let sys = scratch.join("sys.rs");
    windows_bindgen::bindgen([
        "--in",
        winmd.to_str().unwrap(),
        "--out",
        sys.to_str().unwrap(),
        "--filter",
        "Direct",
        "--filter",
        "Included",
        "--sys",
    ]);
    let sys = std::fs::read_to_string(sys).unwrap();
    assert!(sys.contains("pub type HRESULT = i32"));
    assert!(sys.contains("fn DirectStatus() -> super::HRESULT"));
    assert!(sys.contains("fn IncludedStatus() -> super::HRESULT"));
}

#[test]
fn semantic_scalars_are_universal() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("semantic_scalars");
    std::fs::create_dir_all(&scratch).unwrap();

    let header = scratch.join("semantic.h");
    std::fs::write(
        &header,
        "typedef unsigned char BOOLEAN;\n\
         typedef union _LARGE_INTEGER { long long QuadPart; } LARGE_INTEGER;\n\
         typedef union _ULARGE_INTEGER { unsigned long long QuadPart; } ULARGE_INTEGER;\n\
         typedef BOOLEAN *PBOOLEAN;\n\
         typedef LARGE_INTEGER *PLARGE_INTEGER;\n\
         typedef ULARGE_INTEGER *PULARGE_INTEGER;\n\
         typedef struct Holder {\n\
             BOOLEAN Boolean;\n\
             LARGE_INTEGER Signed;\n\
             ULARGE_INTEGER Unsigned;\n\
             union _LARGE_INTEGER DirectSigned;\n\
             union _ULARGE_INTEGER DirectUnsigned;\n\
         } Holder;\n\
         #define BOOLEAN_TRUE ((BOOLEAN)1)\n\
         #define BOOLEAN_COMPLEMENT (BOOLEAN)(~0)\n\
         BOOLEAN ReadBoolean(void);\n\
         LARGE_INTEGER ReadSigned(void);\n\
         ULARGE_INTEGER ReadUnsigned(void);\n\
         union _LARGE_INTEGER ReadDirectSigned(void);\n\
         union _ULARGE_INTEGER ReadDirectUnsigned(void);\n\
         void GetValues(PBOOLEAN boolean, PLARGE_INTEGER signed_value,\n\
                        PULARGE_INTEGER unsigned_value);",
    )
    .unwrap();

    let midl_header = scratch.join("midl.h");
    std::fs::write(
        &midl_header,
        "typedef unsigned char BOOLEAN;\n\
         typedef struct _LARGE_INTEGER { long long QuadPart; } LARGE_INTEGER;\n\
         typedef struct _ULARGE_INTEGER { unsigned long long QuadPart; } ULARGE_INTEGER;\n\
         typedef struct MidlHolder {\n\
             BOOLEAN Boolean;\n\
             LARGE_INTEGER Signed;\n\
             ULARGE_INTEGER Unsigned;\n\
             struct _LARGE_INTEGER DirectSigned;\n\
             struct _ULARGE_INTEGER DirectUnsigned;\n\
         } MidlHolder;\n\
         LARGE_INTEGER ReadSigned(void);\n\
         ULARGE_INTEGER ReadUnsigned(void);",
    )
    .unwrap();

    let forward_header = scratch.join("forward.h");
    std::fs::write(
        &forward_header,
        "typedef union _LARGE_INTEGER LARGE_INTEGER;\n\
         typedef union _ULARGE_INTEGER ULARGE_INTEGER;\n\
         void UseForward(union _LARGE_INTEGER *signed_value,\n\
                         union _ULARGE_INTEGER *unsigned_value);",
    )
    .unwrap();

    let hostile_reference = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Other {\n\
                 type BOOLEAN = u8;\n\
                 struct LARGE_INTEGER { LowPart: u32, HighPart: i32 }\n\
                 struct ULARGE_INTEGER { LowPart: u32, HighPart: u32 }\n\
             }",
        )
        .output(&hostile_reference)
        .write()
        .unwrap();

    let direct_rdl = scratch.join("direct.rdl");
    let referenced_rdl = scratch.join("referenced.rdl");
    let midl_rdl = scratch.join("midl.rdl");
    let forward_rdl = scratch.join("forward.rdl");
    let flat_dir = scratch.join("flat");
    std::fs::create_dir_all(&flat_dir).unwrap();
    {
        let _guard = test_clang::libclang_guard();

        windows_clang::clang()
            .input(&header)
            .output(&direct_rdl)
            .namespace("Direct")
            .library("test.dll")
            .write()
            .unwrap();

        windows_clang::clang()
            .input(&forward_header)
            .output(&forward_rdl)
            .namespace("Forward")
            .library("test.dll")
            .write()
            .unwrap();

        windows_clang::clang()
            .input(&midl_header)
            .output(&midl_rdl)
            .namespace("Midl")
            .library("test.dll")
            .write()
            .unwrap();

        windows_clang::clang()
            .input(&header)
            .reference(&hostile_reference)
            .output(&referenced_rdl)
            .namespace("Referenced")
            .library("test.dll")
            .write()
            .unwrap();

        windows_clang::clang()
            .input(&header)
            .output(&flat_dir)
            .namespace("Flat")
            .write_by_header()
            .unwrap();
    }

    for path in [
        direct_rdl.as_path(),
        referenced_rdl.as_path(),
        flat_dir.join("semantic.rdl").as_path(),
    ] {
        let contents = std::fs::read_to_string(path).unwrap();
        assert!(!contents.contains("type BOOLEAN"));
        for declaration in [
            "struct LARGE_INTEGER {",
            "union LARGE_INTEGER {",
            "struct ULARGE_INTEGER {",
            "union ULARGE_INTEGER {",
        ] {
            assert!(!contents.contains(declaration));
        }
        assert!(!contents.contains("Other::"));
        assert!(contents.contains("Boolean: bool"));
        assert!(contents.contains("Signed: i64"));
        assert!(contents.contains("Unsigned: u64"));
        assert!(contents.contains("DirectSigned: i64"));
        assert!(contents.contains("DirectUnsigned: u64"));
        assert!(contents.contains("const BOOLEAN_TRUE: bool = true"));
        assert!(contents.contains("const BOOLEAN_COMPLEMENT: bool = true"));
        assert!(contents.contains("fn ReadBoolean() -> bool"));
        assert!(contents.contains("fn ReadSigned() -> i64"));
        assert!(contents.contains("fn ReadUnsigned() -> u64"));
        assert!(contents.contains("fn ReadDirectSigned() -> i64"));
        assert!(contents.contains("fn ReadDirectUnsigned() -> u64"));
        assert!(contents.contains("boolean: *mut bool"));
        assert!(contents.contains("signed_value: *mut i64"));
        assert!(contents.contains("unsigned_value: *mut u64"));
    }

    let midl = std::fs::read_to_string(&midl_rdl).unwrap();
    for declaration in ["struct LARGE_INTEGER {", "struct ULARGE_INTEGER {"] {
        assert!(!midl.contains(declaration));
    }
    assert!(midl.contains("Boolean: bool"));
    assert!(midl.contains("Signed: i64"));
    assert!(midl.contains("Unsigned: u64"));
    assert!(midl.contains("DirectSigned: i64"));
    assert!(midl.contains("DirectUnsigned: u64"));
    assert!(midl.contains("fn ReadSigned() -> i64"));
    assert!(midl.contains("fn ReadUnsigned() -> u64"));

    let forward = std::fs::read_to_string(&forward_rdl).unwrap();
    for declaration in [
        "struct LARGE_INTEGER {",
        "union LARGE_INTEGER {",
        "struct ULARGE_INTEGER {",
        "union ULARGE_INTEGER {",
    ] {
        assert!(!forward.contains(declaration));
    }
    assert!(forward.contains("signed_value: *mut i64"));
    assert!(forward.contains("unsigned_value: *mut u64"));

    let winmd = scratch.join("out.winmd");
    windows_rdl::reader()
        .input(&direct_rdl)
        .input(&referenced_rdl)
        .output(winmd)
        .write()
        .unwrap();
}

#[test]
fn namespaced_record_dependencies_preserve_layout() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("record_dependency");
    std::fs::create_dir_all(&scratch).unwrap();

    let rdl = scratch.join("record_dependency.rdl");
    let symbols_rdl = scratch.join("record_dependency_symbols.rdl");
    let flat = scratch.join("flat");
    std::fs::create_dir_all(&flat).unwrap();
    {
        let _guard = test_clang::libclang_guard();
        windows_clang::clang()
            .input("input/record_dependency.hpp")
            .output(&rdl)
            .namespace("RecordDependency")
            .library("test.dll")
            .write()
            .unwrap();

        windows_clang::clang()
            .input("input/record_dependency.hpp")
            .output(&symbols_rdl)
            .namespace("RecordDependencySymbols")
            .library("test.dll")
            .symbol("ReturnRemote")
            .write()
            .unwrap();

        windows_clang::clang()
            .input("input/record_dependency.hpp")
            .output(&flat)
            .namespace("RecordDependency")
            .write_by_header()
            .unwrap();
    }

    let contents = std::fs::read_to_string(&rdl).unwrap();
    assert!(contents.contains("struct RemoteValue"));
    assert!(contents.contains("payload: i64"));
    assert!(contents.contains("nested: NestedValue"));
    assert!(contents.contains("struct NestedValue"));
    assert!(contents.contains("code: i16"));
    assert!(contents.contains("struct PointerValue"));
    assert!(contents.contains("payload: u32"));
    assert!(contents.contains("type RemoteAlias = RemoteValue"));
    assert!(contents.contains("alias_value: RemoteAlias"));
    assert!(contents.contains("fn ReturnDirect() -> RemoteValue"));
    assert!(contents.contains("fn ReturnAlias() -> RemoteAlias"));
    assert!(contents.contains("fn ReturnPointer() -> *mut PointerValue"));

    windows_rdl::reader()
        .input(&rdl)
        .output(scratch.join("record_dependency.winmd"))
        .write()
        .unwrap();

    let symbols = std::fs::read_to_string(&symbols_rdl).unwrap();
    assert!(symbols.contains("fn ReturnRemote() -> RemoteValue"));
    assert!(symbols.contains("struct RemoteValue"));
    assert!(symbols.contains("payload: i64"));
    assert!(symbols.contains("nested: NestedValue"));
    assert!(symbols.contains("struct NestedValue"));
    assert!(!symbols.contains("struct Envelope"));
    windows_rdl::reader()
        .input(&symbols_rdl)
        .output(scratch.join("record_dependency_symbols.winmd"))
        .write()
        .unwrap();

    let flat_main = flat.join("record_dependency.rdl");
    let flat_dependency = flat.join("record_dependency_inc.rdl");
    let main_contents = std::fs::read_to_string(&flat_main).unwrap();
    let dependency_contents = std::fs::read_to_string(&flat_dependency).unwrap();
    assert!(!main_contents.contains("struct RemoteValue"));
    assert!(!main_contents.contains("struct NestedValue"));
    assert!(!main_contents.contains("struct PointerValue"));
    assert!(dependency_contents.contains("struct RemoteValue"));
    assert!(dependency_contents.contains("payload: i64"));
    assert!(dependency_contents.contains("nested: NestedValue"));
    assert!(dependency_contents.contains("struct NestedValue"));
    assert!(dependency_contents.contains("code: i16"));
    assert!(dependency_contents.contains("struct PointerValue"));
    assert!(dependency_contents.contains("payload: u32"));
    assert!(dependency_contents.contains("type RemoteAlias = RemoteValue"));

    windows_rdl::reader()
        .input(flat_main)
        .input(flat_dependency)
        .output(scratch.join("record_dependency_flat.winmd"))
        .write()
        .unwrap();
}

fn run(name: &str) {
    let input_path = format!("input/{name}.h");
    let expected_path = format!("expected/{name}.rdl");
    let scratch = format!("{}/{name}", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    // Extract directives from `//!` comment lines at the top of the .h file.
    // Supported directives:
    //   //! namespace <Name>       - sets the RDL namespace (default: "Test")
    //   //! library <name.dll>     - sets the library name
    //   //! map <symbol>=<dll>      - overrides the DLL for one symbol (may repeat)
    //   //! filter <suffix>        - adds a filter (may repeat)
    //   //! args <arg> ...         - extra clang args
    //   //! include <path>         - adds an include directory (-I)
    //   //! reference <file>       - compiles `input/<file>.rdl` to a winmd and
    //                               feeds it as a reference (cross-namespace
    //                               resolution; the target namespace is excluded)
    //   //! flat                    - use the source-based per-header (flat) scrape
    //                               (`write_by_header`, as `tool-win32`) instead of
    //                               the namespaced scrape (`write`, as `tool-webview`).
    //                               Enables the flat-mode collapses/normalizations
    //                               (`header_root.is_some()`); references/filters/library
    //                               do not apply.
    //   //! symbols <a>,<b>,...     - restricts emission to an allowlist of function
    //                               names (comma-separated); only those functions and
    //                               their transitive type/const closure are emitted,
    //                               every other root is suppressed.
    let contents = std::fs::read_to_string(&input_path).unwrap();
    let mut namespace = "Test".to_string();
    let mut library = String::new();
    let mut filters: Vec<String> = vec![];
    let mut extra_args: Vec<String> = vec![];
    let mut map: Vec<(String, String)> = vec![];
    let mut references: Vec<String> = vec![];
    let mut flat = false;
    let mut symbols: Vec<String> = vec![];

    for line in contents.lines() {
        let Some(rest) = line.strip_prefix("//!") else {
            continue;
        };
        let rest = rest.trim();
        if let Some(ns) = rest.strip_prefix("namespace ") {
            namespace = ns.trim().to_string();
        } else if let Some(lib) = rest.strip_prefix("library ") {
            library = lib.trim().to_string();
        } else if let Some(m) = rest.strip_prefix("map ") {
            if let Some((symbol, dll)) = m.trim().split_once('=') {
                map.push((symbol.trim().to_string(), dll.trim().to_string()));
            }
        } else if let Some(f) = rest.strip_prefix("filter ") {
            filters.push(f.trim().to_string());
        } else if let Some(a) = rest.strip_prefix("args ") {
            extra_args.extend(a.split_whitespace().map(String::from));
        } else if let Some(inc) = rest.strip_prefix("include ") {
            extra_args.push("-I".to_string());
            extra_args.push(inc.trim().to_string());
        } else if let Some(r) = rest.strip_prefix("reference ") {
            references.push(r.trim().to_string());
        } else if let Some(s) = rest.strip_prefix("symbols ") {
            symbols.extend(s.split(',').map(|t| t.trim().to_string()));
        } else if rest == "flat" {
            flat = true;
        }
    }

    // Compile any reference RDL fixtures to winmd so they can be fed to the
    // scraper as cross-namespace references.
    let mut reference_winmds: Vec<Vec<u8>> = vec![];
    for reference in &references {
        let ref_rdl = format!("input/{reference}.rdl");
        let ref_winmd = format!("{scratch}/{reference}.winmd");
        windows_rdl::reader()
            .input(&ref_rdl)
            .output(&ref_winmd)
            .write()
            .unwrap();
        reference_winmds.push(std::fs::read(ref_winmd).unwrap());
    }

    let rdl_out = format!("{scratch}/{name}.rdl");

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .args(extra_args.iter().map(|s| s.as_str()))
        .input(&input_path)
        .output(&rdl_out)
        .namespace(&namespace);

    clang.resolution_default();

    clang.reference_byte_sets(&reference_winmds);

    if !library.is_empty() {
        clang.library(&library);
    }

    if !map.is_empty() {
        clang.libraries(map.iter().map(|(s, d)| (s.as_str(), d.as_str())));
    }

    for f in &filters {
        clang.filter(f);
    }

    if !symbols.is_empty() {
        clang.symbols(symbols.iter().map(|s| s.as_str()));
    }

    // libclang's process-global state is not safe under the harness's parallel test threads;
    // serialize the scrape (see `test_clang::libclang_guard`).
    let _guard = test_clang::libclang_guard();

    if flat {
        // Source-based per-header (flat) scrape, as `tool-win32`: one flat root namespace,
        // `header_root.is_some()`. Emits every defining header in the parse into `scratch`;
        // a self-contained fixture yields a single `<stem>.rdl` (the lowercased header stem,
        // which matches `rdl_out`).
        clang
            .namespace(&namespace)
            .output(&scratch)
            .write_by_header()
            .unwrap();
    } else {
        // Namespaced scrape, as `tool-webview`: `header_root.is_none()`, resolves external
        // types via the reference winmds.
        clang.write().unwrap();
    }

    let actual = std::fs::read_to_string(&rdl_out).unwrap();
    std::fs::write(&expected_path, &actual).unwrap();
}
