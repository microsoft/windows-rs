use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/metadata-enum-attribute.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/metadata-enum-attribute.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/metadata-enum-attribute.winmd")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/metadata-enum-attribute.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
