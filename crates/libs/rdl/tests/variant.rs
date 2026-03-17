use windows_rdl::*;

#[test]
pub fn write() {
    writer()
        .input("../bindgen/default/Windows.Win32.winmd")
        .output("tests/variant.rdl")
        .namespace("Windows.Win32.System.Variant")
        .write()
        .unwrap();
}
