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
fn build_emits_static_global_function_signature() {
    let dir = scratch("global_function");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("api.rdl");
    let output = dir.join("api.winmd");
    std::fs::write(
        &input,
        "#[win32] mod Test { #[library(\"test.dll\")] extern fn GetValue(value: i32) -> i32; }",
    )
    .unwrap();

    assert!(
        riddle()
            .args(["build", "--no-default"])
            .arg(&input)
            .arg("--out")
            .arg(&output)
            .status()
            .unwrap()
            .success()
    );

    let index = windows_metadata::reader::Index::read(&output).unwrap();
    let method = index
        .expect("Test", "Apis")
        .methods()
        .find(|method| method.name() == "GetValue")
        .unwrap();
    assert!(
        method
            .flags()
            .contains(windows_metadata::MethodAttributes::Static)
    );
    assert!(
        !method
            .signature(&[])
            .flags
            .contains(windows_metadata::MethodCallAttributes::HASTHIS)
    );
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
fn check_renders_every_independent_error() {
    let dir = scratch("multiple_errors");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("api.rdl");
    std::fs::write(
        &input,
        "#[win32]\nmod Test {\n\
         struct First { value: i32, value: i32, }\n\
         struct Second { value: i32, value: i32, }\n\
         }\n",
    )
    .unwrap();

    let output = riddle().arg("check").arg(&input).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(stderr.matches("error[RDL0001]:").count(), 2);
    assert!(stderr.contains("aborting due to 2 previous errors"));
}

#[test]
fn check_reports_finalized_metadata_validation() {
    let dir = scratch("metadata_validation");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("api.rdl");
    std::fs::write(
        &input,
        "#[win32]\nmod Test {\n\
         attribute MarkerAttribute { fn(); Value: i32, }\n\
         #[Marker(Value = 1, Value = 2)]\n\
         struct Item {}\n\
         }\n",
    )
    .unwrap();

    let output = riddle().arg("check").arg(&input).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.starts_with("error[RDL0001]:"));
    assert!(stderr.contains("duplicate named field argument `Value`"));
    assert!(stderr.contains(&input.to_string_lossy().to_string()));
    assert!(stderr.contains("metadata row Attribute["));
    assert!(stderr.contains('^'));
}

#[test]
fn check_reports_invalid_overload_metadata() {
    let dir = scratch("overload_validation");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("api.rdl");
    std::fs::write(
        &input,
        "#[winrt]\nmod Test {\n\
         interface IValue {\n\
         #[overload(GetFirst)]\n\
         fn Get(&self, value: i32);\n\
         #[overload(GetSecond)]\n\
         fn Get(&self, value: i32);\n\
         }\n\
         }\n",
    )
    .unwrap();

    let output = riddle()
        .args(["check", "--no-default"])
        .arg(&input)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.starts_with("error[RDL0001]:"));
    assert!(stderr.contains("duplicate overload signature `Get`"));
    assert!(stderr.contains("first declared here"));
    assert!(stderr.matches(" --> ").count() >= 2);
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

#[test]
fn fmt_checks_and_updates_files_with_comments() {
    let dir = scratch("fmt");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("api.rdl");
    std::fs::write(
        &input,
        "/// API\n#[win32] mod Test { struct Value { field:i32, // Field\n} }",
    )
    .unwrap();

    let check = riddle()
        .args(["fmt", "--check"])
        .arg(&input)
        .output()
        .unwrap();
    assert_eq!(check.status.code(), Some(1));
    assert!(
        String::from_utf8(check.stderr)
            .unwrap()
            .contains("needs formatting")
    );

    assert!(riddle().arg("fmt").arg(&input).status().unwrap().success());
    let formatted = std::fs::read_to_string(&input).unwrap();
    assert!(formatted.contains("/// API"));
    assert!(formatted.contains("field: i32, // Field"));
    assert!(
        riddle()
            .args(["fmt", "--check"])
            .arg(&input)
            .status()
            .unwrap()
            .success()
    );
}

#[test]
fn fmt_does_not_modify_any_file_when_one_is_invalid() {
    let dir = scratch("fmt_invalid");
    std::fs::create_dir_all(&dir).unwrap();
    let valid = dir.join("a.rdl");
    let invalid = dir.join("b.rdl");
    let original = "#[win32] mod Test { struct Value { field:i32 } }";
    std::fs::write(&valid, original).unwrap();
    std::fs::write(&invalid, "#[win32] mod Test { struct Broken { field: } }").unwrap();

    let output = riddle().arg("fmt").arg(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(std::fs::read_to_string(valid).unwrap(), original);
}

#[test]
fn fmt_writes_standard_input_to_standard_output() {
    let mut child = riddle()
        .args(["fmt", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"#[win32] mod Test { struct Value {} }")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "#[win32]\nmod Test {\n    struct Value {}\n}\n"
    );
}
