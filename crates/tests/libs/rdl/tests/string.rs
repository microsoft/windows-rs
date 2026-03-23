use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/string.rdl")
        .reference("../../../libs/bindgen/default/Windows.Win32.winmd")
        .output("tests/string.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/string.winmd")
        .output("tests/string.rdl")
        .write()
        .unwrap();
}
