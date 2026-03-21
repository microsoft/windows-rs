use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/enum-flags.rdl")
        .output("tests/enum-flags.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/enum-flags.winmd")
        .output("tests/enum-flags.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
