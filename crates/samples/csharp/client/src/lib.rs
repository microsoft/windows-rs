#[test]
fn test() {
    let mut command = std::process::Command::new("dotnet.exe");
    command.arg("run");
    let output = command.output().expect("failed to run dotnet");

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    assert_eq!(result.trim(), "result = 9");
}
