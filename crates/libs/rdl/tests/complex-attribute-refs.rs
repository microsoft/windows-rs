use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/complex-attribute-refs.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/complex-attribute-refs.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/complex-attribute-refs.winmd")
        .output("tests/complex-attribute-refs.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
