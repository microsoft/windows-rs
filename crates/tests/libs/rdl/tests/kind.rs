use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/kind.rdl")
        .output("tests/kind.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/kind.winmd")
        .output("tests/kind.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
