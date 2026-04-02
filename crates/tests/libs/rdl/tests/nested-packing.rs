use windows_rdl::*;

/// Verify that un-nested (anonymous) inner types inherit the `packed` attribute
/// from their enclosing struct or union.
#[test]
pub fn write() {
    writer()
        .input("../../../libs/bindgen/default/Windows.Win32.winmd")
        .output("tests/nested-packing.rdl")
        .filter("Windows.Win32.Devices.Bluetooth.BTH_INFO_RSP")
        .write()
        .unwrap();
}
