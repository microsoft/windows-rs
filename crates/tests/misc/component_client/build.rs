fn main() {
    let Ok(windir) = std::env::var("windir") else {
        return;
    };

    windows_bindgen::bindgen([
        "--in",
        "../component/component.winmd",
        &format!("{}\\System32\\WinMetadata", windir),
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_component",
        "--no-comment",
        "--flat",
        "--reference",
        "windows",
    ])
    .unwrap();
}
