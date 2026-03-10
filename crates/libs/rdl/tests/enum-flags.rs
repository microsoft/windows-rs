use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/enum-flags.rdl")
        .output("tests/enum-flags.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/enum-flags.winmd")
        .output("tests/enum-flags.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
