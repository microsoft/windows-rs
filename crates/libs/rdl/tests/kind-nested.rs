use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/kind-nested.rdl")
        .output("tests/kind-nested.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/kind-nested.winmd")
        .output("tests/kind-nested-out.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
