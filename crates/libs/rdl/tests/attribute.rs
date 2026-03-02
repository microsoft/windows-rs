use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute.rdl")
        .output("tests/attribute.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute.winmd")
        .output("tests/attribute.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
