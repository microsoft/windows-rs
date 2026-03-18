use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/enum-attribute-on-class.rdl")
        .output("tests/enum-attribute-on-class.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/enum-attribute-on-class.winmd")
        .output("tests/enum-attribute-on-class.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
