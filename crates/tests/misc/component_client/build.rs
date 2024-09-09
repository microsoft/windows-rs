fn main() {
    let command: Vec<String> = vec![
        "--in".to_owned(),
        "../component/component.winmd".to_owned(),
        format!("{}\\System32\\WinMetadata", env!("windir")),
        "--out".to_owned(),
        "src/bindings.rs".to_owned(),
        "--filter".to_owned(),
        "test_component".to_owned(),
        "--config".to_owned(),
        "no-bindgen-comment".to_owned(),
    ];

    windows_bindgen::bindgen(&command).unwrap();
}
