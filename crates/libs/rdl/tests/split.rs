use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/split")
        .output("tests/split.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/split.winmd")
        .output("tests/split")
        .namespace("Test")
        .split()
        .write()
        .unwrap();
}
