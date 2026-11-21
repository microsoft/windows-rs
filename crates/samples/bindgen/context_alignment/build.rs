fn main() {
    windows_bindgen::builder()
        .filter("CONTEXT")
        .filter("M128A")
        .sys()
        .write();
}
