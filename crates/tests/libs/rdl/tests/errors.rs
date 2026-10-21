fn out_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("OUT_DIR")).join(format!("test_rdl_err_{name}.winmd"))
}

#[test]
fn missing_output_is_rejected() {
    let error = windows_rdl::reader()
        .input_text("#[winrt] mod Test {}")
        .write()
        .unwrap_err();
    assert_eq!(error.message, "output is required");
    assert_eq!(error.file_name, "");
    assert_eq!((error.line, error.column), (0, 0));
    assert_eq!(error.to_string(), "\nerror: output is required");
    assert_eq!(format!("{error:?}"), error.to_string());
}

#[test]
fn writer_missing_output_is_rejected() {
    let error = windows_rdl::writer().split().write().unwrap_err();
    assert_eq!(error.message, "output is required");
}

#[test]
fn malformed_metadata_reports_its_role() {
    let reference_error = windows_rdl::reader()
        .input_text("#[winrt] mod Test {}")
        .reference_bytes(b"not metadata")
        .output(out_path("invalid_reference"))
        .write()
        .unwrap_err();
    assert_eq!(reference_error.message, "invalid reference");
    assert_eq!(reference_error.file_name, "<memory>");

    let input_error = windows_rdl::writer()
        .input_bytes(b"not metadata")
        .output(out_path("invalid_input").with_extension("rdl"))
        .write()
        .unwrap_err();
    assert_eq!(input_error.message, "invalid input");
    assert_eq!(input_error.file_name, "<memory>");
}

#[test]
fn writer_rejects_non_winmd_input() {
    let error = windows_rdl::writer()
        .input("input.rdl")
        .output(out_path("writer_extension").with_extension("rdl"))
        .write()
        .unwrap_err();
    assert_eq!(error.message, "expected .winmd file");
    assert_eq!(error.file_name, "input.rdl");
}

#[test]
fn unsupported_input_extension_is_rejected() {
    let error = windows_rdl::reader()
        .input("definitely_not_here.txt")
        .output(out_path("ext"))
        .write()
        .unwrap_err();
    assert_eq!(error.message, "expected .rdl file");
    assert_eq!(error.file_name, "definitely_not_here.txt");
    assert_eq!((error.line, error.column), (0, 0));
    assert_eq!(
        error.to_string(),
        "\nerror: expected .rdl file\n --> definitely_not_here.txt"
    );
}

#[test]
fn integer_constants_reinterpret_bits_across_partitions() {
    // Real const idioms surfaced by merging the full `windows.h` closure. Each
    // exercises a reader path that previously rejected a valid value:
    //   * `(UINT)-1` style sentinel into an unsigned typedef (MCI_ALL_DEVICE_ID)
    //   * `(ATOM)0` cast routed to a cross-partition typedef whose body names a
    //     bare sibling (`type ATOM = WORD`)
    //   * an `HRESULT` (`i32`) whose macro evaluates to a positive literal beyond
    //     `i32::MAX`, the two's-complement negative (OLE_E_FIRST)
    //   * `(LPCSTR)2` MAKEINTRESOURCE pointer constant
    //   * a constant typed by an enum, encoded against its `#[repr]` integer
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Test {\n\
             mod Win {\n\
             type WORD = u16;\n\
             type ATOM = WORD;\n\
             type HRESULT = i32;\n\
             type CHAR = i8;\n\
             type LPCSTR = *const CHAR;\n\
             #[repr(i32)] enum SHSTOCKICONID { DOCNOASSOC = 0, }\n\
             }\n\
             mod Api {\n\
             const ALL_BITS: u32 = -1;\n\
             const INVALID_ATOM: super::Win::ATOM = -1;\n\
             const OLE_E_FIRST: super::Win::HRESULT = 2147745792;\n\
             const CERT_POLICY: super::Win::LPCSTR = 2;\n\
             const SIID_INVALID: super::Win::SHSTOCKICONID = -1;\n\
             }\n\
             }",
        )
        .output(out_path("const_reinterpret"))
        .write()
        .unwrap();
}

#[test]
fn mixed_pointer_constness_is_rejected() {
    for (name, ty, column) in [
        ("mut_const", "*mut *const i32", 32),
        ("const_mut", "*const *mut i32", 34),
        ("deep_mut_const", "*mut *mut *const i32", 37),
        ("deep_const_mut", "*const *const *mut i32", 41),
        ("behind_reference", "&mut *mut *const i32", 37),
    ] {
        let source = format!(
            "#[win32]\nmod Test {{\n    #[library(\"test.dll\")]\n    extern fn Mixed(value: {ty});\n}}\n"
        );
        let error = windows_rdl::reader()
            .input_text(&source)
            .output(out_path(name))
            .write()
            .unwrap_err();

        assert_eq!(
            error.message,
            "mixed `*mut` and `*const` pointer chains are not representable"
        );
        assert_eq!(error.file_name, ".rdl");
        assert_eq!(error.line, 4);
        assert_eq!(error.column, column);
    }
}

#[test]
fn reference_base_class_must_be_unsealed() {
    let compile = |base: &str| {
        windows_rdl::reader()
            .input_text(&format!(
                "#[winrt]\nmod Test {{\n    interface IWidget {{}}\n    class Widget: {base} {{\n        IWidget,\n    }}\n}}\n"
            ))
            .output(out_path("reference_base"))
            .reference_default()
            .write()
    };

    compile("Windows::UI::Xaml::DependencyObject").unwrap();

    let error = compile("Windows::Foundation::Uri").unwrap_err();
    assert_eq!(error.message, "base class must be `#[unsealed]`");
}
