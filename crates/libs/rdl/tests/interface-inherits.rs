use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/interface-inherits.rdl")
        .output("tests/interface-inherits.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/interface-inherits.winmd")
        .output("tests/interface-inherits.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
