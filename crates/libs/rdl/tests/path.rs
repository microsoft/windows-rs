use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/path.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/path.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/path.winmd")
        .output("tests/path-output.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
