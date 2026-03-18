use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/const-underlying-rdl.rdl")
        .output("tests/const-underlying-rdl.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/const-underlying-rdl.winmd")
        .output("tests/const-underlying-rdl-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
