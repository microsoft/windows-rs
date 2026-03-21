use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/const-ptr.rdl")
        .output("tests/const-ptr.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/const-ptr.winmd")
        .output("tests/const-ptr.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
