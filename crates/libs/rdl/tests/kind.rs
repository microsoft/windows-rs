use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/kind.rdl")
        .output("tests/kind.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input(r#"tests/kind.winmd"#)
        .output("tests/kind.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
