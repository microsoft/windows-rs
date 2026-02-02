use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/mod-type.rdl")
        .output("tests/mod-type.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input(r#"tests/mod-type.winmd"#)
        .output("tests/mod-type.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
