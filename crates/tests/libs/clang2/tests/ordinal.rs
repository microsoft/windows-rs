#![cfg(target_env = "msvc")]
#![allow(dead_code, non_snake_case)]

use std::path::Path;

mod rich {
    include!(concat!(env!("OUT_DIR"), "/ordinal.rs"));
}
mod minimal {
    include!(concat!(env!("OUT_DIR"), "/ordinal_minimal.rs"));
}
mod sys {
    include!(concat!(env!("OUT_DIR"), "/ordinal_sys.rs"));
}

windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(name: *const u16) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn GetProcAddress(module: *mut core::ffi::c_void, name: *const u8) -> *mut core::ffi::c_void);
windows_core::link!("clang2_ordinal.dll" "C" ordinal(17) fn DifferentName(value: i32) -> i32);

#[test]
fn noname_export_is_callable_in_every_binding_style() {
    unsafe {
        let module = GetModuleHandleW(windows_core::w!("clang2_ordinal.dll").0);
        assert!(!module.is_null());
        assert!(GetProcAddress(module, c"OrdinalOnly".as_ptr().cast()).is_null());
        assert!(!GetProcAddress(module, 17usize as *const u8).is_null());
        for value in [-17, 0, 29] {
            let expected = value * 3 + 7;
            assert_eq!(rich::OrdinalOnly(value), expected);
            let minimal: minimal::OrdinalOnly = minimal::OrdinalOnly;
            assert_eq!(minimal(value), expected);
            let sys: sys::OrdinalOnly = sys::OrdinalOnly;
            assert_eq!(sys(value), expected);
            let renamed: DifferentName = DifferentName;
            assert_eq!(renamed(value), expected);
        }
    }
}

#[test]
fn ordinal_contract_roundtrips_through_metadata() {
    let out = Path::new(env!("OUT_DIR"));
    let original = std::fs::read(out.join("ordinal.winmd")).unwrap();
    let file = windows_metadata::reader::File::new(original.clone()).unwrap();
    let index = windows_metadata::reader::Index::new(vec![file]);
    let method = index.expect("Test", "Apis").methods().next().unwrap();
    let map = method.impl_map().unwrap();
    assert_eq!(map.import_scope().name(), "clang2_ordinal.dll");
    assert_eq!(map.import_name(), "#17");
    assert_eq!(map.import_ordinal().unwrap(), Some(17));
    let rdl = out.join("ordinal_roundtrip.rdl");
    windows_rdl::writer()
        .input_bytes(&original)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(&rdl).unwrap(),
        std::fs::read_to_string(out.join("ordinal.rdl")).unwrap()
    );
    let folder = out.join("ordinal_roundtrip");
    std::fs::create_dir_all(&folder).unwrap();
    let roundtrip = folder.join("ordinal.winmd");
    windows_rdl::reader()
        .input(rdl)
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(original, std::fs::read(roundtrip).unwrap());
}

#[test]
#[should_panic(expected = "--extern cannot preserve it")]
fn bare_extern_rejects_ordinal_import() {
    let out = Path::new(env!("OUT_DIR"));
    windows_bindgen::bindgen([
        "--in",
        out.join("ordinal.winmd").to_str().unwrap(),
        "--out",
        out.join("ordinal_extern.rs").to_str().unwrap(),
        "--filter",
        "Test",
        "--sys",
        "--extern",
    ]);
}
