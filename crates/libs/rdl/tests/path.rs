use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/path.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/path.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/path.winmd")
        .output("tests/path-output.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
