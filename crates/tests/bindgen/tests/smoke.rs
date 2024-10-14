use test_bindgen2::smoke::*;

#[test]
fn smoke() {
    unsafe {
        assert!(GetTickCount() > 0);
    }
}
