#[test]
fn main() {
    let mut command = std::process::Command::new("dotnet.exe");
    command.arg("run");

    #[cfg(target_arch = "x86")]
    command.args("-r win-x86 /p:PlatformTarget=x86".split_whitespace());

    let output = command.output().expect("failed to run dotnet");

    if !output.status.success() {
        panic!(
            "stdout:\n{}\n\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let result = String::from_utf8_lossy(&output.stdout);
    let result: Vec<&str> = result.lines().collect();

    assert_eq!(result[0], "Hello from cs land");
    assert!(result[1].starts_with("Hello handy (0x"));
    assert_eq!(result[2], "interop handle: 0x1c8");
}
