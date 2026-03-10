use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/interface-requires.rdl")
        .output("tests/interface-requires.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/interface-requires.winmd")
        .output("tests/interface-requires.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
