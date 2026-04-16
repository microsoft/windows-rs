#![cfg(target_pointer_width = "64")]

use windows_rdl::*;

#[test]
fn roundtrip() {
    let paths = std::fs::read_dir("roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("h"));

    for h in paths {
        let rdl = h.with_extension("rdl").to_str().unwrap().to_string();
        let winmd = h.with_extension("winmd").to_str().unwrap().to_string();
        let h = h.to_str().unwrap().to_string();
        let reference = "../../../libs/bindgen/default";

        clang()
            .input(&h)
            .input(reference)
            .output(&rdl)
            .namespace("Test")
            .write()
            .unwrap();

        reader()
            .input(&rdl)
            .input(reference)
            .output(&winmd)
            .write()
            .unwrap();

        writer()
            .input(&winmd)
            .input(reference)
            .output(&rdl)
            .filter("Test")
            .write()
            .unwrap();
    }
}
