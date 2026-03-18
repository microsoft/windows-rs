use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/same-name.rdl")
        .output("tests/same-name.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/same-name.winmd")
        .output("tests/same-name.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
