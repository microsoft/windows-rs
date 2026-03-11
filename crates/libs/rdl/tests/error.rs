use windows_rdl::*;

#[test]
pub fn error_display() {
    let e = Error::new("message", "file_name.rdl", 2, 3);

    let s = format!("{e}");

    assert_eq!(s, "\nerror: message\n --> file_name.rdl:2:4");
}

#[test]
#[should_panic(expected = "error: `repr` attribute not found\n --> .rdl:4:5")]
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
#[should_panic(expected = "error: value not valid\n --> .rdl:4:20")]
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
#[should_panic(expected = "error: `link` attribute not found\n --> .rdl:3:5")]
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
    expected = "error: `winrt` and `win32` attributes are mutually exclusive\n --> .rdl:5:5"
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
#[should_panic(expected = "error: `winrt` or `win32` attribute required\n --> .rdl:3:5")]
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
