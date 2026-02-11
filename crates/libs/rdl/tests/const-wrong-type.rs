use windows_rdl::*;

#[test]
#[should_panic(
    expected = r#"{ message: "value not valid", file_name: "tests/const-wrong-type.rdl", line: 3, column: 19 }"#
)]
pub fn parse() {
    Reader::new()
        .input("tests/const-wrong-type.rdl")
        .output("tests/const-wrong-type.winmd")
        .write()
        .unwrap();
}
