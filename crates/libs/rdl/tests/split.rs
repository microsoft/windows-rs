use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/split")
        .output("tests/split.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/split.winmd")
        .output("tests/split")
        .namespace("Test")
        .split()
        .write()
        .unwrap();
}
