fn main() {
    let mut command = std::process::Command::new("cargo.exe");

    command.args([
        "run",
        "-p",
        "riddle",
        "--target-dir",
        "target",
        "--",
        "-in",
        "../component/component.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "-out",
        "src/bindings.rs",
        "-filter",
        "test_component",
    ]);

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }
}
