use windows_rdl::*;

#[test]
#[should_panic(
    expected = r#"{ message: "`repr` attribute not found", file_name: "tests/error-no-repr.rdl", line: 2, column: 4 }"#
)]
pub fn parse() {
    Reader::new()
        .input("tests/error-no-repr.rdl")
        .output("tests/error-no-repr.winmd")
        .write()
        .unwrap();
}
