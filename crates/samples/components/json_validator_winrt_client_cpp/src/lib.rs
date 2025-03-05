#![cfg(target_env = "msvc")]

#[test]
fn test() {
    unsafe extern "system" {
        fn client();
    }
    unsafe {
        client();
    }
}
