use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/arches.rdl")
        .reference("../bindgen/default/Windows.Win32.winmd")
        .output("tests/arches.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/arches.winmd")
        .input("../bindgen/default/Windows.Win32.winmd")
        .output("tests/arches.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
