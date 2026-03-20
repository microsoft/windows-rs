use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/arches.rdl")
        .output("tests/arches.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/arches.winmd")
        .output("tests/arches.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
