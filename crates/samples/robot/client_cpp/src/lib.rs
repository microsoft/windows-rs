#![cfg(target_env = "msvc")]

#[test]
fn main() {
    unsafe extern "system" {
        fn main_cpp() -> windows_result::HRESULT;
    }
    unsafe {
        main_cpp().unwrap();
    }
}
