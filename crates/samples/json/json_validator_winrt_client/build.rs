fn main() {
    let Ok(windir) = std::env::var("windir") else {
        return;
    };

    windows_bindgen::bindgen([
        "--in",
        "../json_validator_winrt/sample.winmd",
        &format!("{}\\System32\\WinMetadata", windir),
        "--out",
        "src/bindings.rs",
        "--filter",
        "Sample",
        "--flat",
    ])
    .unwrap();
}
