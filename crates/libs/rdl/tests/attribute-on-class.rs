use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-on-class.rdl")
        .output("tests/attribute-on-class.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-on-class.winmd")
        .output("tests/attribute-on-class.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
