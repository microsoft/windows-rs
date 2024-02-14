#[test]
fn bindings() {
    let args = ["--etc", "tests/bindings.txt"];

    windows_bindgen::bindgen(args).unwrap();
}
