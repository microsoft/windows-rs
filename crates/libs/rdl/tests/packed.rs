use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/packed.rdl")
        .output("tests/packed.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/packed.winmd")
        .output("tests/packed.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
