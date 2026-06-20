// Negative tests for windows-rdl error reporting. The golden round-trip harness
// only ever feeds valid input, so these drive the failure paths in the reader
// and the resulting `Error` formatting. Unwrapping the `Err` panics with the
// `Debug` output, which defers to `Display`, so `should_panic` exercises all
// three `Display` branches in `src/error.rs`.

fn out_path(name: &str) -> String {
    std::env::temp_dir()
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
