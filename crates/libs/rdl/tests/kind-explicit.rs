use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/kind-explicit.rdl")
        .output("tests/kind-explicit.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/kind-explicit.winmd")
        .output("tests/kind-explicit-out.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
