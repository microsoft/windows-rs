fn main() {
    windows_bindgen::bindgen([
        "--in",
        "../activation/metadata.winmd",
        "default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_activation",
        "--no-comment",
        "--flat",
    ])
    .unwrap();
}
