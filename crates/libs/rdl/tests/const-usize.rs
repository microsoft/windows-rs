use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/const-usize.rdl")
        .output("tests/const-usize.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/const-usize.winmd")
        .output("tests/const-usize.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
