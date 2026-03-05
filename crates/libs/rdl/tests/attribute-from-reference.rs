use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-from-reference.rdl")
        .reference("../bindgen/default/windows.winmd")
        .output("tests/attribute-from-reference.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-from-reference.winmd")
        .output("tests/attribute-from-reference-out.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
