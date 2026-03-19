use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/opt.rdl")
        .output("tests/opt.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/opt.winmd")
        .output("tests/opt.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
