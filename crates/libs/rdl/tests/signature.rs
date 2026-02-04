use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/signature.rdl")
        .output("tests/signature.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/signature.winmd")
        .output("tests/signature.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
