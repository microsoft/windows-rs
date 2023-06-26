mod nested_struct;
mod params;
mod r#struct;

use std::process::Command;

pub fn run_riddle(name: &str) -> Vec<windows_metadata::File> {
    let idl = format!("tests/{name}.idl");
    let winmd = format!("tests/{name}.winmd");
    let rs = format!("src/{name}.rs");

    let before = std::fs::read_to_string(&idl).expect("Failed to read input");

    // Convert .idl to .winmd
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "-in", &idl, "-out", &winmd, "-filter", "Test",
    ]);
    assert!(command.status().unwrap().success());

    // Convert .winmd back to .idl
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "-in", &winmd, "-out", &idl, "-filter", "Test",
    ]);
    assert!(command.status().unwrap().success());

    // Check that .idl is unchanged
    let after = std::fs::read_to_string(&idl).expect("Failed to read output");
    assert_eq!(before, after);

    // Convert .idl to .rs
    let mut command = Command::new("cargo");
    command.args([
        "run", "-p", "riddle", "--", "-in", &idl, "-out", &rs, "-filter", "Test",
    ]); // TODO: -config FLATTEN doesn't work for namespaces
    assert!(command.status().unwrap().success());

    // Return winmd file for validation
    let mut files = tool_lib::default_metadata();
    files.push(
        windows_metadata::File::new(std::fs::read(&winmd).expect("failed to read winmd"))
            .expect("failed to parse winmd"),
    );
    files
}
