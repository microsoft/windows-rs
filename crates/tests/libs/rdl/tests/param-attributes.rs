use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/param-attributes.rdl")
        .output("tests/param-attributes.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/param-attributes.winmd")
        .output("tests/param-attributes.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
