use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/kind-explicit.rdl")
        .output("tests/kind-explicit.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/kind-explicit.winmd")
        .output("tests/kind-explicit-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
