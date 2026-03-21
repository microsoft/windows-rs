use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute-everywhere.rdl")
        .output("tests/attribute-everywhere.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute-everywhere.winmd")
        .output("tests/attribute-everywhere.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
