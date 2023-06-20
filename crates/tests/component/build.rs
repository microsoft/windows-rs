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

    let mut command = std::process::Command::new("cargo.exe");

    command.args([
        "run",
        "-p",
        "riddle",
        "--target-dir",
        "target", // TODO: workaround for https://github.com/rust-lang/cargo/issues/6412
        "--",
        "-in",
        "component.winmd",
        &metadata_dir,
        "-out",
        "src/bindings.rs",
        "-filter",
        "test_component",
        "-config",
        "IMPLEMENT",
    ]);

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }
}
