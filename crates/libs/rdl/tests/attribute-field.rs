use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute-field.rdl")
        .output("tests/attribute-field.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute-field.winmd")
        .output("tests/attribute-field.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
