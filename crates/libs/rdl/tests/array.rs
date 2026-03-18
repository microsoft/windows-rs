use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/array.rdl")
        .output("tests/array.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/array.winmd")
        .output("tests/array.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
