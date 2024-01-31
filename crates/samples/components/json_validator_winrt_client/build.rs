fn main() {
    if let Err(error) = windows_bindgen::bindgen([
        "--in",
        "../json_validator_winrt/sample.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "--out",
        "src/bindings.rs",
        "--filter",
        "Sample",
    ]) {
        panic!("{error}");
    }
}
