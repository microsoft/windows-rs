use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/attribute-zero-args.rdl")
        .output("tests/attribute-zero-args.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/attribute-zero-args.winmd")
        .output("tests/attribute-zero-args.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
