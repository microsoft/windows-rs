#![cfg(windows)]

// Builds and runs the C#/WinRT consumer via `dotnet run`. cargo puts the component's
// `langperf.dll` (a cdylib dependency) on PATH, which the dotnet child process inherits,
// so WinRT activation resolves the class without registration. A tiny iteration count is
// used here just to verify the projection works end to end.
#[test]
fn main() {
    let mut command = std::process::Command::new("dotnet.exe");
    command.arg("run");

    #[cfg(target_arch = "x86")]
    command.args("-r win-x86 /p:PlatformTarget=x86".split_whitespace());

    command.args(["--", "--iterations", "200"]);

    let output = command.output().expect("failed to run dotnet");

    assert!(
        output.status.success(),
        "stdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let result = String::from_utf8_lossy(&output.stdout);

    for label in ["Create:", "Int32:", "String:", "Object:", "Cast:"] {
        assert!(
            result.contains(label),
            "missing {label} in stdout:\n{result}"
        );
    }
}
