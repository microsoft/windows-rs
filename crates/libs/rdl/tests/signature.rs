use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/signature.rdl")
        .output("tests/signature.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/signature.winmd")
        .output("tests/signature.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
