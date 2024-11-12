fn main() {
    windows_bindgen2::bindgen([
        "--in",
        "../component/component.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_component",
        "--no-comment",
        "--flat",
    ]);
}
