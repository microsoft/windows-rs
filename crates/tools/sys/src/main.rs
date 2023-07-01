fn main() {
    let mut command = std::process::Command::new("cargo");

    command.args([
        "run",
        "-p",
        "riddle",
        "--",
        "-@",
        "crates/tools/sys/bindings.txt",
    ]);

    assert!(command.status().unwrap().success());
}
