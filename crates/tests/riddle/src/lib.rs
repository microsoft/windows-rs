mod generic_interfaces;
mod module_attributes;
mod nested_module;
mod nested_struct;
mod params;
mod r#struct;
mod win32_struct;
mod winrt_struct;

use std::process::Command;

pub fn run_riddle(name: &str, dialect: &str, etc: &[&str]) -> Vec<windows_metadata::File> {
    let rdl = format!("tests/{name}.rdl");
    let winmd = format!("tests/{name}.winmd");
    let rs = format!("src/{name}.rs");

    let before = std::fs::read_to_string(&rdl).expect("Failed to read input");

    // Convert .rdl to .winmd
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "--in", &rdl, "--out", &winmd, "--filter", "Test",
    ]);
    assert!(command.status().unwrap().success());

    // Convert .winmd back to .rdl
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "--in", &winmd, "--out", &rdl, "--filter", "Test", "--config",
    ]);
    command.arg(format!("TYPE={dialect}"));
    assert!(command.status().unwrap().success());

    // Check that .rdl is unchanged
    let after = std::fs::read_to_string(&rdl).expect("Failed to read output");
    assert_eq!(before, after, "no equal {}", rdl);

    // Convert .rdl to .rs
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "--in", &rdl, "--out", &rs, "--filter", "Test",
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
