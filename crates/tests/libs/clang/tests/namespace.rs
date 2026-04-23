#![cfg(target_pointer_width = "64")]

use windows_rdl::*;

#[test]
fn namespace() {
    let h = "tests/namespace.h";
    let rdl = "tests/namespace.rdl";
    let winmd = "tests/namespace.winmd";
    let reference = "../../../libs/bindgen/default";

    clang()
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .input(h)
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
