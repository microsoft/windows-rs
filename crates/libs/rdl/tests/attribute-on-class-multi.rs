use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute-on-class-multi.rdl")
        .output("tests/attribute-on-class-multi.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute-on-class-multi.winmd")
        .output("tests/attribute-on-class-multi.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
