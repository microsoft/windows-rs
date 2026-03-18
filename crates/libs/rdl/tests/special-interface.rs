use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/special-interface.rdl")
        .output("tests/special-interface.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/special-interface.winmd")
        .output("tests/special-interface.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
