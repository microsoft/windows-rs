use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/cross-namespace-enum-attribute.rdl")
        .output("tests/cross-namespace-enum-attribute.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/cross-namespace-enum-attribute.winmd")
        .output("tests/cross-namespace-enum-attribute.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
