use test_bindgen2::iota::*;

#[test]
fn test() {
    unsafe {
        assert!(GetTickCount() > 0);
    }
}
