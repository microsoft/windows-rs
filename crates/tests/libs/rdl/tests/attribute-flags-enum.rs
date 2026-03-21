use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/attribute-flags-enum.rdl")
        .output("tests/attribute-flags-enum.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/attribute-flags-enum.winmd")
        .output("tests/attribute-flags-enum.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
