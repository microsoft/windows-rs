fn main() {
    if !cfg!(windows) {
        return;
    }

    windows_bindgen::builder()
        .input("../json_validator_winrt/sample.winmd")
        .input(&format!(
            "{}\\System32\\WinMetadata",
            std::env::var("windir").unwrap()
        ))
        .output("src/bindings.rs")
        .filter("Sample")
        .flat()
        .write()
        .unwrap();
}
