use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute-on-interface.rdl")
        .output("tests/attribute-on-interface.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute-on-interface.winmd")
        .output("tests/attribute-on-interface.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
