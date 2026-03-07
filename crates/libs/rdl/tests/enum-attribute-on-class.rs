use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/enum-attribute-on-class.rdl")
        .output("tests/enum-attribute-on-class.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/enum-attribute-on-class.winmd")
        .output("tests/enum-attribute-on-class.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
