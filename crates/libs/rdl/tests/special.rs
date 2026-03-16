use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/special.rdl")
        .output("tests/special.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/special.winmd")
        .output("tests/special.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
