use windows_rdl::*;

#[test]
pub fn guid() {
    reader()
        .input("tests/guid.rdl")
        .output("tests/guid.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/guid.winmd")
        .output("tests/guid.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
