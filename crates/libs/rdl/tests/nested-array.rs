use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/nested-array.rdl")
        .output("tests/nested-array.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/nested-array.winmd")
        .output("tests/nested-array.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
