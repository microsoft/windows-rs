#[test]
fn bindings() {
    windows_bindgen::bindgen(["--etc", "tests/bindings.txt"]).unwrap();
}
