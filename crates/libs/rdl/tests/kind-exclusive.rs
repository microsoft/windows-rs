use windows_rdl::*;

#[test]
#[should_panic(
    expected = r#"{ message: "`winrt` and `win32` attributes are mutually exclusive", file_name: "tests/kind-exclusive.rdl", line: 4, column: 4 }"#
)]
pub fn parse() {
    Reader::new()
        .input("tests/kind-exclusive.rdl")
        .output("tests/kind-exclusive.winmd")
        .write()
        .unwrap();
}
