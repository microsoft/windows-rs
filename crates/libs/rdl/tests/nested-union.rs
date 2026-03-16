use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/nested-union.rdl")
        .output("tests/nested-union.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/nested-union.winmd")
        .output("tests/nested-union.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
