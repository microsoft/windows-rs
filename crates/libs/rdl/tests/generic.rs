use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/generic.rdl")
        .output("tests/generic.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/generic.winmd")
        .output("tests/generic.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
