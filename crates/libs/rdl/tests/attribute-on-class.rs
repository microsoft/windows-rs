use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute-on-class.rdl")
        .output("tests/attribute-on-class.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute-on-class.winmd")
        .output("tests/attribute-on-class.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
