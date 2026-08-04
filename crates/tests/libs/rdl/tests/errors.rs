// Negative tests for windows-rdl error reporting. The golden round-trip harness
// only ever feeds valid input, so these drive the failure paths in the reader
// and the resulting `Error` formatting. Unwrapping the `Err` panics with the
// `Debug` output, which defers to `Display`, so `should_panic` exercises all
// three `Display` branches in `src/error.rs`.

fn out_path(name: &str) -> String {
    std::path::Path::new(env!("OUT_DIR"))
        .join(format!("test_rdl_err_{name}.winmd"))
        .to_string_lossy()
        .into_owned()
}

#[test]
#[should_panic(expected = "-->")]
fn syntax_error_reports_line_and_column() {
    // `Display` branch 1: a parse error carries a `file:line:column` location.
    windows_rdl::reader()
        .input_str("#[winrt] mod Test { this is not valid rdl }")
        .output(&out_path("syntax"))
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: output is required")]
fn missing_output_is_rejected() {
    // `Display` branch 2: empty file name yields a bare message with no `-->`.
    windows_rdl::reader()
        .input_str("#[winrt] mod Test {}")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "expected .rdl or .winmd")]
fn unsupported_input_extension_is_rejected() {
    // `Display` branch 3: a file name but no source location (line/column 0).
    windows_rdl::reader()
        .input("definitely_not_here.txt")
        .output(&out_path("ext"))
        .write()
        .unwrap();
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
        .input_str(
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
        .output(&out_path("const_reinterpret"))
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
            .input_str(&source)
            .output(&out_path(name))
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
