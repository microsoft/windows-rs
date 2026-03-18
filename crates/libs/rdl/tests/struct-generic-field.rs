use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/struct-generic-field.rdl")
        .output("tests/struct-generic-field.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/struct-generic-field.winmd")
        .output("tests/struct-generic-field.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
