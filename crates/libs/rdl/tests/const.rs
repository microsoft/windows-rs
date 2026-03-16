use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/const.rdl")
        .output("tests/const.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/const.winmd")
        .output("tests/const.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
