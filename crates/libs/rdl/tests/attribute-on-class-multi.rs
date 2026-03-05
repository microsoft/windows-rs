use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-on-class-multi.rdl")
        .output("tests/attribute-on-class-multi.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-on-class-multi.winmd")
        .output("tests/attribute-on-class-multi.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
