use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/fn.rdl")
        .output("tests/fn.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/fn.winmd")
        .output("tests/fn.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
