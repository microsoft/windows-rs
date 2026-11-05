fn out_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("OUT_DIR")).join(format!("test_rdl_err_{name}.winmd"))
}

#[test]
fn invalid_import_ordinals() {
    for ordinal in [
        "#", "#0", "#65536", "#-1", "#+1", "#0x12", "#12x", "# 12", "#12 ",
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{ #[library(\"example.dll\", import = \"{ordinal}\")] extern fn F(); }}"
            ))
            .output(out_path("invalid_import_ordinal"))
            .write()
            .unwrap_err();
        assert_eq!(
            error.message, "import ordinal must be decimal digits between 1 and 65535",
            "{ordinal}"
        );
    }
}

#[test]
fn returned_pointer_short_forms_are_rejected() {
    for attribute in ["out_ptr", "out_ptr(null_on_failure)"] {
        let error = windows_rdl::reader()
            .input(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..\\..\\..\\..\\metadata\\metadata.rdl"),
            )
            .input_text(&format!(
                "#[win32] mod Test {{ extern fn F(#[out] #[{attribute}] data: *mut *mut u8); }}"
            ))
            .output(out_path("returned_pointer"))
            .write()
            .unwrap_err();
        assert_eq!(error.message, "attribute type not found");
    }
}

#[test]
fn nullable_short_forms_are_rejected() {
    for declaration in [
        "extern fn F(#[out] #[nullable] value: *mut *mut u8);",
        "extern fn F() -> #[nullable] *mut u8;",
        "struct Value { #[nullable] value: *mut u8, }",
    ] {
        let error = windows_rdl::reader()
            .input(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..\\..\\..\\..\\metadata\\metadata.rdl"),
            )
            .input_text(&format!("#[win32] mod Test {{ {declaration} }}"))
            .output(out_path("nullable"))
            .write()
            .unwrap_err();
        assert_eq!(error.message, "attribute type not found");
    }
}

#[test]
fn fixed_pointer_arguments() {
    for ty in [
        "ptr64",
        "ptr64<i32>",
        "ptr64<*mut i32, *const u8>",
        "ptr64<'a>",
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{ struct Value {{ address: {ty}, }} }}"
            ))
            .output(out_path("fixed_pointer_arguments"))
            .write()
            .unwrap_err();
        assert!(
            error
                .message
                .contains("ptr64 requires one explicit pointer type argument"),
            "{error}"
        );
    }
    let error = windows_rdl::reader()
        .input_text("#[winrt] mod Test { struct Value { address: ptr64<*mut i32>, } }")
        .output(out_path("fixed_pointer_winrt"))
        .write()
        .unwrap_err();
    assert_eq!(error.message, "fixed-width pointers require Win32 metadata");
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
fn reference_base_class_must_be_unsealed() {
    let compile = |base: &str| {
        windows_rdl::reader()
            .input_text(&format!(
                "#[winrt]\nmod Test {{\n    #[guid(auto)] interface IWidget {{}}\n    class Widget: {base} {{\n        IWidget,\n    }}\n}}\n"
            ))
            .output(out_path("reference_base"))
            .reference_default()
            .write()
    };

    compile("Windows::UI::Xaml::DependencyObject").unwrap();

    let error = compile("Windows::Foundation::Uri").unwrap_err();
    assert_eq!(error.message, "base class must be `#[unsealed]`");
}
