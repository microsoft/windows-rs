#![cfg(target_pointer_width = "64")]

use windows_rdl::*;

fn run_roundtrip(file: &str) {
    let h = format!("src/{file}.h");
    let rdl = format!("src/{file}.rdl");
    let winmd = format!("src/{file}.winmd");
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

// generated tests

#[test]
fn roundtrip_callback() {
    run_roundtrip("callback");
}
#[test]
fn roundtrip_clsid() {
    run_roundtrip("clsid");
}
#[test]
fn roundtrip_const() {
    run_roundtrip("const");
}
#[test]
fn roundtrip_empty() {
    run_roundtrip("empty");
}
#[test]
fn roundtrip_enum() {
    run_roundtrip("enum");
}
#[test]
fn roundtrip_enum_param() {
    run_roundtrip("enum_param");
}
#[test]
fn roundtrip_enum_repr() {
    run_roundtrip("enum_repr");
}
#[test]
fn roundtrip_extern_c() {
    run_roundtrip("extern_c");
}
#[test]
fn roundtrip_extern_c_macro() {
    run_roundtrip("extern_c_macro");
}
#[test]
fn roundtrip_extern_c_macro_dep() {
    run_roundtrip("extern_c_macro_dep");
}
#[test]
fn roundtrip_extern_c_macro_nested() {
    run_roundtrip("extern_c_macro_nested");
}
#[test]
fn roundtrip_flags_enum() {
    run_roundtrip("flags_enum");
}
#[test]
fn roundtrip_fn() {
    run_roundtrip("fn");
}
#[test]
fn roundtrip_interface() {
    run_roundtrip("interface");
}
#[test]
fn roundtrip_interface_empty() {
    run_roundtrip("interface_empty");
}
#[test]
fn roundtrip_interface_ptr() {
    run_roundtrip("interface_ptr");
}
#[test]
fn roundtrip_interface_ref() {
    run_roundtrip("interface_ref");
}
#[test]
fn roundtrip_lvalue_ref() {
    run_roundtrip("lvalue_ref");
}
#[test]
fn roundtrip_midl_enum() {
    run_roundtrip("midl_enum");
}
#[test]
fn roundtrip_midl_interface() {
    run_roundtrip("midl_interface");
}
#[test]
fn roundtrip_nested() {
    run_roundtrip("nested");
}
#[test]
fn roundtrip_propget() {
    run_roundtrip("propget");
}
#[test]
fn roundtrip_sal() {
    run_roundtrip("sal");
}
#[test]
fn roundtrip_struct() {
    run_roundtrip("struct");
}
#[test]
fn roundtrip_struct_packed() {
    run_roundtrip("struct_packed");
}
#[test]
fn roundtrip_typedef() {
    run_roundtrip("typedef");
}
#[test]
fn roundtrip_typedef_in_struct() {
    run_roundtrip("typedef_in_struct");
}
#[test]
fn roundtrip_typedef_in_struct_dep() {
    run_roundtrip("typedef_in_struct_dep");
}
#[test]
fn roundtrip_typedef_struct() {
    run_roundtrip("typedef_struct");
}
#[test]
fn roundtrip_types() {
    run_roundtrip("types");
}
#[test]
fn roundtrip_union() {
    run_roundtrip("union");
}
#[test]
fn roundtrip_vararg() {
    run_roundtrip("vararg");
}
#[test]
fn roundtrip_vararg_cb() {
    run_roundtrip("vararg_cb");
}
#[test]
fn roundtrip_winmd_ref() {
    run_roundtrip("winmd_ref");
}
