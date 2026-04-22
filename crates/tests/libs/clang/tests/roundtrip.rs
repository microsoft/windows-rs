#![cfg(target_pointer_width = "64")]

use windows_rdl::*;

fn run_roundtrip(file: &str) {
    let h = format!("roundtrip/{file}.h");
    let rdl = format!("roundtrip/{file}.rdl");
    let winmd = format!("roundtrip/{file}.winmd");
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

include!(concat!(env!("OUT_DIR"), "/roundtrip_tests.rs"));
