use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/delegate.rdl")
        .output("tests/delegate.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/delegate.winmd")
        .output("tests/delegate.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
