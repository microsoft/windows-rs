mod module_attributes;
mod nested_module;
mod nested_struct;
mod params;
mod r#struct;

use std::process::Command;

pub fn run_riddle(name: &str, etc: &[&str]) -> Vec<windows_metadata::File> {
    let rd = format!("tests/{name}.rd");
    let winmd = format!("tests/{name}.winmd");
    let rs = format!("src/{name}.rs");

    let before = std::fs::read_to_string(&rd).expect("Failed to read input");

    // Convert .rd to .winmd
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "--in", &rd, "--out", &winmd, "--filter", "Test",
    ]);
    assert!(command.status().unwrap().success());

    // Convert .winmd back to .rd
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "--in", &winmd, "--out", &rd, "--filter", "Test",
    ]);
    assert!(command.status().unwrap().success());

    // Check that .rd is unchanged
    let after = std::fs::read_to_string(&rd).expect("Failed to read output");
    assert_eq!(before, after);

    // Convert .rd to .rs
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "--in", &rd, "--out", &rs, "--filter", "Test",
    ]);
    command.args(etc);
    assert!(command.status().unwrap().success());

    // Return winmd file for validation
    let mut files = tool_lib::default_metadata();
    files.push(
        windows_metadata::File::new(std::fs::read(&winmd).expect("failed to read winmd"))
            .expect("failed to parse winmd"),
    );
    files
}
