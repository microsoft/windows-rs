use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/enum.rdl")
        .output("tests/enum.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/enum.winmd")
        .output("tests/enum.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
