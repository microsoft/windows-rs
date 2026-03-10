use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/special-interface.rdl")
        .output("tests/special-interface.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/special-interface.winmd")
        .output("tests/special-interface.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
