use std::process::Command;

pub fn run_riddle(idl: &str) -> String {
    let mut command = Command::new("cargo.exe");

    command
        .arg("install")
        .arg("--path")
        .arg("../../tools/riddle");

    if !command.status().unwrap().success() {
        panic!("Failed to install riddle");
    }

    let winmd = std::path::Path::new(idl)
        .with_extension("winmd")
        .to_string_lossy()
        .into_owned();

    let mut command = Command::new("riddle.exe");
    command.arg("-in").arg(idl).arg("-out").arg(&winmd);

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }

    let mut command = Command::new("riddle.exe");
    command.arg("-in").arg(&winmd).arg("-out").arg(&idl);

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }

    winmd
}
