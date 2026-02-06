use windows_rdl::*;

#[test]
#[should_panic(
    expected = r#"{ message: "`link` attribute not found", file_name: "tests/fn-no-link.rdl", line: 2, column: 4 }"#
)]
pub fn parse() {
    Reader::new()
        .input("tests/fn-no-link.rdl")
        .output("tests/fn-no-link.winmd")
        .write()
        .unwrap();
}
