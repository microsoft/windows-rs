use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/struct-generic-field.rdl")
        .output("tests/struct-generic-field.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/struct-generic-field.winmd")
        .output("tests/struct-generic-field.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
