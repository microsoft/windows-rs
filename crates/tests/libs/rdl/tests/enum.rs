use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/enum.rdl")
        .reference("../../../libs/bindgen/default")
        .output("tests/enum.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/enum.winmd")
        .output("tests/enum.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
