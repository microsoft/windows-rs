use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/delegate.rdl")
        .output("tests/delegate.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/delegate.winmd")
        .output("tests/delegate.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
