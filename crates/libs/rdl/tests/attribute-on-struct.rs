use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute-on-struct.rdl")
        .output("tests/attribute-on-struct.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute-on-struct.winmd")
        .output("tests/attribute-on-struct.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
