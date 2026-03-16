use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/kind-override.rdl")
        .output("tests/kind-override.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/kind-override.winmd")
        .output("tests/kind-override-out.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
