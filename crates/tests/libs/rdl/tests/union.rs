use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/union.rdl")
        .output("tests/union.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/union.winmd")
        .output("tests/union.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
