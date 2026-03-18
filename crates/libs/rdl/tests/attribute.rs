use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute.rdl")
        .output("tests/attribute.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute.winmd")
        .output("tests/attribute.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
