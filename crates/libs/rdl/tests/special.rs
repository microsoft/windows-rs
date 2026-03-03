use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/special.rdl")
        .output("tests/special.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/special.winmd")
        .output("tests/special.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
