use windows_rdl::*;

#[test]
pub fn write() {
    writer()
        .input("../bindgen/default/Windows.Win32.winmd")
        .output("tests/context.rdl")
        .filter("Windows.Win32.System.Diagnostics.Debug")
        .write()
        .unwrap();
}
