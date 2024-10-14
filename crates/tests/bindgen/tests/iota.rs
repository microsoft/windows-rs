use test_bindgen2::iota::*;

#[test]
fn smoke() {
    unsafe {
        assert!(GetTickCount() > 0);
    }
}
