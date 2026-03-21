use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/arches.rdl")
        .reference("../../../libs/bindgen/default/Windows.Win32.winmd")
        .output("tests/arches.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/arches.winmd")
        .input("../../../libs/bindgen/default/Windows.Win32.winmd")
        .output("tests/arches.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
