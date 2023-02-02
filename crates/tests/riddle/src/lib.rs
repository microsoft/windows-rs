use std::process::Command;

pub fn run_riddle(input: &str) -> String {
    let mut command = Command::new("cargo.exe");
    command.arg("install").arg("--path").arg("../../tools/riddle");

    if !command.status().unwrap().success() {
        panic!("Failed to install riddle");
    }

    // ildasm.exe %temp%\test_riddle.winmd
    let output = std::env::temp_dir().join("test_riddle.winmd").to_string_lossy().into_owned();

    let mut command = Command::new("riddle.exe");
    command.arg("-input").arg(input).arg("-output").arg(&output);

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }

    output
}
