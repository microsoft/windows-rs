use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-on-interface.rdl")
        .output("tests/attribute-on-interface.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-on-interface.winmd")
        .output("tests/attribute-on-interface.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
