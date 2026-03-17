use windows_rdl::*;

fn roundtrip(winmd: &str, rdl: &str) {
    writer()
        .input(winmd)
        .reference("crates/libs/bindgen/default/Windows.winmd") // for Windows.Foundation.HRESULT
        .output(rdl)
        .namespace("Windows")
        .split()
        .write()
        .unwrap();

    reader().input(rdl).output(winmd).write().unwrap();
}

fn main() {
    roundtrip("crates/libs/bindgen/default/Windows.winmd", "rdl/Windows");

    roundtrip(
        "crates/libs/bindgen/default/Windows.Win32.winmd",
        "rdl/Windows.Win32",
    );

    roundtrip(
        "crates/libs/bindgen/default/Windows.Wdk.winmd",
        "rdl/Windows.Wdk",
    );
}
