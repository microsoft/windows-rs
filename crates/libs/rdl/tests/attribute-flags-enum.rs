use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-flags-enum.rdl")
        .output("tests/attribute-flags-enum.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-flags-enum.winmd")
        .output("tests/attribute-flags-enum.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
