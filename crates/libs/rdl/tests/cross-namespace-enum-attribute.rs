use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/cross-namespace-enum-attribute.rdl")
        .output("tests/cross-namespace-enum-attribute.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/cross-namespace-enum-attribute.winmd")
        .output("tests/cross-namespace-enum-attribute.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
