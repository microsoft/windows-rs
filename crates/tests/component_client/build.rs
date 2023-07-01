fn main() {
    let mut command = std::process::Command::new("cargo");

    command.args([
        "run",
        "-p",
        "riddle",
        "--target-dir",
        "../../../target/test_component_client", // TODO: workaround for https://github.com/rust-lang/cargo/issues/6412
        "--",
        "--in",
        "../component/component.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_component",
    ]);

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }
}
