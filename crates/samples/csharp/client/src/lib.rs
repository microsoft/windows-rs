#[test]
fn test() {
    let mut command = std::process::Command::new("dotnet.exe");
    command.arg("run");

    #[cfg(target_arch = "x86")]
    command.args("-r win-x86 /p:PlatformTarget=x86".split_whitespace());

    let output = command.output().expect("failed to run dotnet");

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let result = String::from_utf8_lossy(&output.stdout);
    let result: Vec<&str> = result.lines().collect();

    assert_eq!(result, ["Add: 9", "Concat: hello world"]);
}
