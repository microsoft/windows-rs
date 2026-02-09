use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/const.rdl")
        .output("tests/const.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/const.winmd")
        .output("tests/const.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
