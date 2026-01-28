use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/struct.rdl")
        .output("tests/struct.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input(r#"tests/struct.winmd"#)
        .output("tests/struct.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
