#[test]
fn test() {
    extern "system" {
        fn client();
    }
    unsafe {
        client();
    }
}
