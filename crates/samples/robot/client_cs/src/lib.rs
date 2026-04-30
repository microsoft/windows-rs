#![cfg(windows)]
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

    assert!(result.contains("Hello from cs land"), "stdout:\n{result}");
    assert!(result.contains("Hello handy (0x"), "stdout:\n{result}");
    assert!(
        result.contains("interop handle: 0x1c8"),
        "stdout:\n{result}"
    );
}
