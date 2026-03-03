use windows_rdl::*;

#[test]
pub fn error_display() {
    let e = Error::new("message", "file_name.rdl", 2, 3);

    let s = format!("{e}");

    assert_eq!(s, "error: message\n  --> file_name.rdl:2:4");
}

#[test]
#[should_panic(
    expected = r#"{ message: "`repr` attribute not found", file_name: ".rdl", line: 4, column: 4 }"#
)]
pub fn enum_repr_not_found() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    enum AsyncStatus {
        Canceled = 2,
        Completed = 1,
        Error = 3,
        Started = 0,
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(
    expected = r#"{ message: "value not valid", file_name: ".rdl", line: 4, column: 19 }"#
)]
pub fn const_value_not_valid() {
    Reader::new()
        .input_str(
            r#"
#[win32]
mod Test {
    const U8: u8 = -1;
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(
    expected = r#"{ message: "`link` attribute not found", file_name: ".rdl", line: 3, column: 4 }"#
)]
pub fn fn_link_not_found() {
    Reader::new()
        .input_str(
            r#"
mod Test {
    fn GetTickCount() -> u32;
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(
    expected = r#"{ message: "`winrt` and `win32` attributes are mutually exclusive", file_name: ".rdl", line: 5, column: 4 }"#
)]
pub fn winrt_win32_exclusive() {
    Reader::new()
        .input_str(
            r#"
mod Test {
    #[winrt]
    #[win32]
    struct A {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(
    expected = r#"{ message: "`winrt` or `win32` attribute required", file_name: ".rdl", line: 3, column: 4 }"#
)]
pub fn winrt_win32_required() {
    Reader::new()
        .input_str(
            r#"
mod Test {
    struct A {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
