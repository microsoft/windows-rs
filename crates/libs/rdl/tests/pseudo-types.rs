use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/pseudo-types.rdl")
        .output("tests/pseudo-types.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/pseudo-types.winmd")
        .output("tests/pseudo-types.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
