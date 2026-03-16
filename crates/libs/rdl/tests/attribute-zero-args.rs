use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute-zero-args.rdl")
        .output("tests/attribute-zero-args.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute-zero-args.winmd")
        .output("tests/attribute-zero-args.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
