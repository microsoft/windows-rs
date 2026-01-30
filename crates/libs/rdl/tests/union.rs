use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/union.rdl")
        .output("tests/union.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input(r#"tests/union.winmd"#)
        .output("tests/union.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
