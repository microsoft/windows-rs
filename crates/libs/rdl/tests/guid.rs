use windows_rdl::*;

#[test]
pub fn guid() {
    Reader::new()
        .input("tests/guid.rdl")
        .output("tests/guid.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/guid.winmd")
        .output("tests/guid.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
