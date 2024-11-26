use test_bindgen::iota::*;

#[test]
fn test() {
    unsafe {
        assert!(GetTickCount() > 0);
    }
}
