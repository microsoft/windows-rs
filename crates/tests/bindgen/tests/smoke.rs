use test_bindgen2::smoke::*;

#[test]
fn test() {
    unsafe {
        assert!(GetTickCount() > 0);
    }
}
