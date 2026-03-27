#![cfg(target_env = "msvc")]

use windows_result::*;

fn main() {
    unsafe extern "system" {
        fn main_cpp() -> HRESULT;
    }
    unsafe {
        main_cpp().unwrap();
    }
}
