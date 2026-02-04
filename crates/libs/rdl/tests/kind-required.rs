use windows_rdl::*;

#[test]
#[should_panic(
    expected = r#"{ message: "`winrt` or `win32` attribute required", file_name: "tests/kind-required.rdl", line: 2, column: 4 }"#
)]
pub fn parse() {
    Reader::new()
        .input("tests/kind-required.rdl")
        .output("tests/kind-required.winmd")
        .write()
        .unwrap();
}
