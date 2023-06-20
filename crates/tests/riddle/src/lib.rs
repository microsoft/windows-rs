use std::process::Command;

pub fn run_riddle(idl: &str) -> Vec<windows_metadata::File> {
    let before = std::fs::read_to_string(idl).expect("Failed to read input");
    let mut command = Command::new("cargo");

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
    command.args(["-in", idl, "-out", &winmd, "-filter", "Test"]);

    assert!(command.status().unwrap().success());

    let mut command = Command::new("riddle.exe");
    command.args(["-in", &winmd, "-out", idl, "-filter", "Test"]);
    assert!(command.status().unwrap().success());

    let after = std::fs::read_to_string(idl).expect("Failed to read output");
    assert_eq!(before, after);

    let mut files = tool_lib::default_metadata();
    files.push(
        windows_metadata::File::new(std::fs::read(winmd).expect("failed to read winmd"))
            .expect("failed to parse winmd"),
    );
    files
}
