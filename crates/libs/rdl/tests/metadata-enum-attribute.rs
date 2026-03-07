use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/metadata-enum-attribute.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/metadata-enum-attribute.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/metadata-enum-attribute.winmd")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/metadata-enum-attribute.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
