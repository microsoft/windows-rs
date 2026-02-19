use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/nested.rdl")
        .output("tests/nested.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/nested.winmd")
        .output("tests/nested.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
