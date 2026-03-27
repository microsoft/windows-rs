#![cfg(test)]
#![cfg(target_env = "msvc")]

use windows_result::*;

#[test]
fn main() {
    unsafe extern "system" {
        fn main_cpp() -> HRESULT;
    }
    unsafe {
        main_cpp().unwrap();
    }
}
