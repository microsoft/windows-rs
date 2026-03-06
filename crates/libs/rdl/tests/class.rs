use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/class.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/class.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/class.winmd")
        .output("tests/class.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
