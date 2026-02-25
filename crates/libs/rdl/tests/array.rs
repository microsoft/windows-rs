use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/array.rdl")
        .output("tests/array.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/array.winmd")
        .output("tests/array.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
