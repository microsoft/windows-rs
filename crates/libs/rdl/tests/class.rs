use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/class.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/class.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/class.winmd")
        .output("tests/class.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
