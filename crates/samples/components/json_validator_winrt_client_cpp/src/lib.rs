#![cfg(target_env = "msvc")]

#[test]
fn test() {
    extern "system" {
        fn client();
    }
    unsafe {
        client();
    }
}
