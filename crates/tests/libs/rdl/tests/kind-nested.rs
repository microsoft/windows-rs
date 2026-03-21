use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/kind-nested.rdl")
        .output("tests/kind-nested.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/kind-nested.winmd")
        .output("tests/kind-nested-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
