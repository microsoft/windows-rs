use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/vararg.rdl")
        .output("tests/vararg.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/vararg.winmd")
        .output("tests/vararg.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
