use windows_rdl::*;

/// Verify that the default interface (marked with `DefaultAttribute`) is listed
/// first when writing a class, regardless of how interfaces appear in the metadata.
#[test]
pub fn write() {
    writer()
        .input("../../../libs/bindgen/default/Windows.winmd")
        .output("tests/default-interface.rdl")
        .filter("Windows.ApplicationModel.Background.AppBroadcastTrigger")
        .write()
        .unwrap();
}
