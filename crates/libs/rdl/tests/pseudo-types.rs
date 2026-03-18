use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/pseudo-types.rdl")
        .output("tests/pseudo-types.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/pseudo-types.winmd")
        .output("tests/pseudo-types.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
