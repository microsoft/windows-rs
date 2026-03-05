use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-on-struct.rdl")
        .output("tests/attribute-on-struct.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-on-struct.winmd")
        .output("tests/attribute-on-struct.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
