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

macro_rules! roundtrip_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            run_roundtrip(stringify!($name));
        }
    };
    ($name:ident, $file:expr) => {
        #[test]
        fn $name() {
            run_roundtrip($file);
        }
    };
}

roundtrip_test!(callback);
roundtrip_test!(r#const, "const");
roundtrip_test!(empty);
roundtrip_test!(r#enum, "enum");
roundtrip_test!(enum_param);
roundtrip_test!(extern_c);
roundtrip_test!(r#fn, "fn");
roundtrip_test!(interface);
roundtrip_test!(interface_ptr);
roundtrip_test!(interface_ref);
roundtrip_test!(lvalue_ref);
roundtrip_test!(midl_interface);
roundtrip_test!(nested);
roundtrip_test!(propget);
roundtrip_test!(r#struct, "struct");
roundtrip_test!(typedef);
roundtrip_test!(typedef_in_struct);
roundtrip_test!(typedef_in_struct_dep);
roundtrip_test!(typedef_struct);
roundtrip_test!(types);
roundtrip_test!(r#union, "union");
roundtrip_test!(vararg);
roundtrip_test!(winmd_ref);
