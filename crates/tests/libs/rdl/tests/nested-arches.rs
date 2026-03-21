use windows_rdl::*;

/// Verify that un-nested (anonymous) inner types inherit the
/// `SupportedArchitecture` attribute from their enclosing struct or union.
#[test]
pub fn write() {
    writer()
        .input("../../../libs/bindgen/default/Windows.Win32.winmd")
        .output("tests/nested-arches-out.rdl")
        .filter("Windows.Win32.System.Kernel")
        .write()
        .unwrap();
}
