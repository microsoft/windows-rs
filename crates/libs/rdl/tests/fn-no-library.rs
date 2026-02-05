use windows_rdl::*;

#[test]
#[should_panic(
    expected = r#"{ message: "`library` attribute not found", file_name: "tests/fn-no-library.rdl", line: 2, column: 4 }"#
)]
pub fn parse() {
    Reader::new()
        .input("tests/fn-no-library.rdl")
        .output("tests/fn-no-library.winmd")
        .write()
        .unwrap();
}
