fn main() {
    println!("cargo:rerun-if-changed=src/component.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));

    let mut command = std::process::Command::new("midlrt.exe");
    command
        .arg("/winrt")
        .arg("/nomidl")
        .arg("/h")
        .arg("nul")
        .arg("/metadata_dir")
        .arg(&metadata_dir)
        .arg("/reference")
        .arg(format!("{metadata_dir}\\Windows.Foundation.winmd"))
        .arg("/winmd")
        .arg("component.winmd")
        .arg("src/component.idl");

    if !command.status().unwrap().success() {
        panic!("Failed to run midlrt");
    }

    // TODO: this looks more complicated but soon the midlrt step above should disappear and then overall it should be simpler...

    let command: Vec<String> = vec![
        "--in".to_owned(),
        "component.winmd".to_owned(),
        metadata_dir.to_owned(),
        "--out".to_owned(),
        "src/bindings.rs".to_owned(),
        "--filter".to_owned(),
        "test_component".to_owned(),
        "--config".to_owned(),
        "implement".to_owned(),
        "no-bindgen-comment".to_owned(),
    ];

    windows_bindgen::bindgen(&command).unwrap();
}
