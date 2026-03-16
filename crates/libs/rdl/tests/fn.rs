use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/fn.rdl")
        .output("tests/fn.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/fn.winmd")
        .output("tests/fn.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
