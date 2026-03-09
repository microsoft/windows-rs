use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-field.rdl")
        .output("tests/attribute-field.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-field.winmd")
        .output("tests/attribute-field.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
