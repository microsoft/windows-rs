#[allow(non_snake_case, non_camel_case_types, dead_code)]
mod bindings;
use bindings::*;
use windows_core::{PCWSTR, PWSTR, w};

fn main() {
    let expected = std::env::args_os().nth(1).unwrap();
    unsafe {
        let module = GetModuleHandleW(w!("WebView2Loader.dll"));
        assert!(!module.is_null());
        let mut path = vec![0; 32768];
        let length = GetModuleFileNameW(module, PWSTR(path.as_mut_ptr()), path.len() as u32) as usize;
        assert!(length > 0 && length < path.len());
        let actual = std::path::PathBuf::from(String::from_utf16(&path[..length]).unwrap());
        assert_eq!(
            actual.canonicalize().unwrap(),
            std::path::PathBuf::from(expected).canonicalize().unwrap(),
        );
        for (a, b, expected) in [
            (w!("1.0.0.0"), w!("1.0.0.0"), 0),
            (w!("1.0.0.0"), w!("2.0.0.0"), -1),
            (w!("2.0.0.0"), w!("1.0.0.0"), 1),
            (w!("100.0.0.0 dev"), w!("100.0.0.0"), 0),
        ] {
            let mut result = [0x12345678; 3];
            CompareBrowserVersions(a, b, &raw mut result[1]).ok().unwrap();
            assert_eq!(result, [0x12345678, expected, 0x12345678]);
        }
        let mut result = 0;
        let failures = [
            (w!("invalid"), w!("1.0.0.0"), &raw mut result),
            (w!("1.0.0.0"), w!("invalid"), &raw mut result),
            (PCWSTR::null(), w!("1.0.0.0"), &raw mut result),
            (w!("1.0.0.0"), PCWSTR::null(), &raw mut result),
            (w!("1.0.0.0"), w!("1.0.0.0"), std::ptr::null_mut()),
        ]
        .map(|(a, b, output)| CompareBrowserVersions(a, b, output).0);
        // The pinned DLL returns E_POINTER for a null result, unlike the documented E_INVALIDARG.
        let invalid = 0x80070057u32 as i32;
        assert_eq!(
            failures,
            [invalid, invalid, invalid, invalid, 0x80004003u32 as i32]
        );
    }
}
