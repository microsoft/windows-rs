// Builds and runs the C#/WinRT consumer via `dotnet run`. cargo puts the component's
// cdylib (`langperf_rust.dll`) directory on PATH, which the dotnet child process inherits.
// WinRT activation probes for a module named after the type's namespace (`LangPerf.dll`),
// so the cdylib is staged under that name beside the test binary before launching. A tiny
// iteration count is used here just to verify the projection works end to end.
#[test]
fn main() {
    stage_component();

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

// Copy the component cdylib (`langperf_rust.dll`) to `LangPerf.dll` beside the test binary,
// which cargo places on PATH, so the C# consumer's WinRT activation can load it by the name
// it probes for without registration.
#[cfg(test)]
fn stage_component() {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let _ = std::fs::copy(dir.join("langperf_rust.dll"), dir.join("LangPerf.dll"));
    }
}
