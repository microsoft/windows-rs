use std::io::Write;
use std::process::{Command, Stdio};

fn scratch(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("riddle_cli_{}_{}", std::process::id(), name))
}

fn riddle() -> Command {
    Command::new(env!("CARGO_BIN_EXE_riddle"))
}

#[test]
fn check_and_build_valid_rdl() {
    let dir = scratch("valid");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("api.rdl");
    let output = dir.join("api.winmd");
    std::fs::write(&input, "#[win32] mod Test { struct Value { field: i32 } }").unwrap();

    assert!(
        riddle()
            .arg("check")
            .arg(&input)
            .status()
            .unwrap()
            .success()
    );
    assert!(
        riddle()
            .arg("build")
            .arg(&input)
            .arg("--out")
            .arg(&output)
            .status()
            .unwrap()
            .success()
    );
    assert!(output.is_file());
}

#[test]
fn invalid_rdl_uses_terminal_diagnostic() {
    let dir = scratch("invalid");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("api.rdl");
    std::fs::write(&input, "#[win32] mod Test { struct Value { field: } }").unwrap();

    let output = riddle().arg("check").arg(&input).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.starts_with("error:"));
    assert!(stderr.contains(&input.to_string_lossy().to_string()));
    assert!(stderr.contains(" --> "));
    assert!(stderr.contains(" | "));
    assert!(stderr.contains('^'));
}

#[test]
fn invalid_arguments_use_exit_code_two() {
    let output = riddle().arg("build").arg("api.rdl").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("requires `--out <path>`")
    );
}

#[test]
fn duplicate_diagnostic_renders_both_labels() {
    let dir = scratch("duplicate");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("api.rdl");
    std::fs::write(
        &input,
        "#[win32]\nmod Test {\n    struct Value {}\n    struct Value {}\n}\n",
    )
    .unwrap();

    let output = riddle().arg("check").arg(&input).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.starts_with("error[RDL0001]:"));
    assert!(stderr.contains("first declared here"));
    assert!(stderr.matches(" --> ").count() >= 2);
    assert!(stderr.contains('^'));
    assert!(stderr.contains('-'));
}

#[test]
fn check_accepts_standard_input() {
    let mut child = riddle()
        .args(["check", "-"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"#[win32] mod Test { struct Value {} }")
        .unwrap();
    assert!(child.wait().unwrap().success());
}

#[test]
fn build_accepts_directory_and_reference_inputs() {
    let dir = scratch("references");
    let dependency_dir = dir.join("dependency");
    let source_dir = dir.join("source");
    std::fs::create_dir_all(&dependency_dir).unwrap();
    std::fs::create_dir_all(&source_dir).unwrap();
    let dependency = dir.join("dependency.winmd");
    let output = dir.join("api.winmd");
    std::fs::write(
        dependency_dir.join("dependency.rdl"),
        "#[win32] mod Dependency { struct Value { field: i32 } }",
    )
    .unwrap();
    std::fs::write(
        source_dir.join("api.rdl"),
        "#[win32] mod Test { struct UsesValue { value: Dependency::Value } }",
    )
    .unwrap();

    assert!(
        riddle()
            .args(["build", "--no-default"])
            .arg(&dependency_dir)
            .arg("--out")
            .arg(&dependency)
            .status()
            .unwrap()
            .success()
    );
    assert!(
        riddle()
            .args(["build", "--no-default", "--reference"])
            .arg(&dependency)
            .arg(&source_dir)
            .arg("--out")
            .arg(&output)
            .status()
            .unwrap()
            .success()
    );
    assert!(output.is_file());
}
