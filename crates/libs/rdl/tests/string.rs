use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/string")
        .output("tests/string.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/string.winmd")
        .output("tests/string.rdl")
        .write()
        .unwrap();
}
