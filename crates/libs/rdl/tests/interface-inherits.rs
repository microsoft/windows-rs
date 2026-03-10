use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/interface-inherits.rdl")
        .output("tests/interface-inherits.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/interface-inherits.winmd")
        .output("tests/interface-inherits.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
