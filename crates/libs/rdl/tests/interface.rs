use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/interface.rdl")
        .output("tests/interface.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input(r#"tests/interface.winmd"#)
        .output("tests/interface.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
