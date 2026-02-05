use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/api.rdl")
        .output("tests/api.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/api.winmd")
        .output("tests/api.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
