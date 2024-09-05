fn main() {
    windows_bindgen::bindgen([
        "--in",
        "../constructors/metadata.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_constructors",
        "--config",
        "no-bindgen-comment",
    ])
    .unwrap();
}
