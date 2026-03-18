use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/struct.rdl")
        .output("tests/struct.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/struct.winmd")
        .output("tests/struct.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
