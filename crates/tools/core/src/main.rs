fn main() {
    let mut command = std::process::Command::new("cargo");

    command.args([
        "run",
        "-p",
        "riddle",
        "--",
        "-@",
        "crates/tools/core/bindings.txt",
    ]);

    assert!(command.status().unwrap().success());

    let mut command = std::process::Command::new("cargo");

    command.args([
        "run",
        "-p",
        "riddle",
        "--",
        "-@",
        "crates/tools/core/com_bindings.txt",
    ]);

    assert!(command.status().unwrap().success());
}
