use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/in.rdl")
        .output("tests/in.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/in.winmd")
        .output("tests/in.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
