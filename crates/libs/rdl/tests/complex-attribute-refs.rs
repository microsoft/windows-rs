use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/complex-attribute-refs.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/complex-attribute-refs.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/complex-attribute-refs.winmd")
        .output("tests/complex-attribute-refs.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
