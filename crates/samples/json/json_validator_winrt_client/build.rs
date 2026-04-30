fn main() {
    windows_bindgen::builder()
        .input("../json_validator_winrt/sample.winmd")
        .input(&format!("{}\\System32\\WinMetadata", env!("windir")))
        .output("src/bindings.rs")
        .filter("Sample")
        .flat()
        .write()
        .unwrap();
}
