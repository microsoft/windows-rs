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
