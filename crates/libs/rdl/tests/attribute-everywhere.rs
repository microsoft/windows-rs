use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-everywhere.rdl")
        .output("tests/attribute-everywhere.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-everywhere.winmd")
        .output("tests/attribute-everywhere.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
