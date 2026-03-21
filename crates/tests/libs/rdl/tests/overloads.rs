use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/overloads.rdl")
        .output("tests/overloads.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/overloads.winmd")
        .output("tests/overloads.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
