use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/nested.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/nested.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/nested.winmd")
        .output("tests/nested.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
