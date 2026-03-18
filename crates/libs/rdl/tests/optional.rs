use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/optional.rdl")
        .output("tests/optional.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/optional.winmd")
        .output("tests/optional.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
