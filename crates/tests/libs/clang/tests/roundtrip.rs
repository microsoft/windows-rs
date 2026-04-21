#![cfg(target_pointer_width = "64")]

use windows_rdl::*;

/// Tests that a multi-level namespace ("Foo.Bar") is emitted as nested `mod`
/// blocks in the generated RDL.  The main `roundtrip()` test hard-codes the
/// flat "Test" namespace, so this case needs its own test function.
///
/// `input_str` is used rather than a file so that the main `roundtrip()` loop
/// (which picks up every `.h` in the `roundtrip/` directory) does not also
/// process this header with the wrong namespace.
#[test]
fn multilevel_namespace() {
    let rdl = "roundtrip/multilevel_ns.rdl";
    let winmd = "roundtrip/multilevel_ns.winmd";
    let reference = "../../../libs/bindgen/default";

    clang()
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .input_str("struct MultiNsTestStruct { int x; int y; };")
        .input(reference)
        .output(rdl)
        .namespace("Foo.Bar")
        .library("test.dll")
        .write()
        .unwrap();

    reader()
        .input(rdl)
        .input(reference)
        .output(winmd)
        .write()
        .unwrap();

    writer()
        .input(winmd)
        .input(reference)
        .output(rdl)
        .filter("Foo.Bar")
        .write()
        .unwrap();
}

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
            .args([
                "-x",
                "c++",
                "--target=x86_64-pc-windows-msvc",
                "-fms-extensions",
            ])
            .input(&h)
            .input(reference)
            .output(&rdl)
            .namespace("Test")
            .library("test.dll")
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
