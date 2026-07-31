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

#[cfg(test)]
fn stage_component() {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        // WinRT activation probes the namespace-derived module name.
        let _ = std::fs::copy(dir.join("langperf_rust.dll"), dir.join("LangPerf.dll"));
    }
}
