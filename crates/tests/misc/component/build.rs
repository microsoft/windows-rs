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

    windows_bindgen::bindgen([
        "--in",
        "component.winmd",
        &metadata_dir,
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_component",
        "--flat",
        "--implement",
        "--no-comment",
        "--reference",
        "windows",
    ])
    .unwrap();
}
