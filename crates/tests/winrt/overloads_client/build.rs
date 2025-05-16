fn main() {
    windows_bindgen::bindgen([
        "--in",
        "../overloads/metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_overloads",
        "--no-comment",
        "--flat",
    ])
    .unwrap();
}
