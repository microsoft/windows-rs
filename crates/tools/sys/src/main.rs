fn main() {
    let mut command = std::process::Command::new("cargo");

    command.args([
        "run",
        "-p",
        "riddle",
        "--",
        "--etc",
        "crates/tools/sys/bindings.txt",
    ]);

    assert!(command.status().unwrap().success());
}
