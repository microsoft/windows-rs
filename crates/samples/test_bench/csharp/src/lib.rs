// Stages the Rust WinRT component cdylib as `Bench.dll` -- the module name WinRT activation
// probes for the `Bench` namespace -- beside the test binary (cargo puts that directory on
// PATH), then runs the windows-csharp consumer via `dotnet run` with a tiny iteration count to
// confirm the generated projection activates and calls the component end to end.
#[cfg(test)]
fn stage_component() {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let _ = std::fs::copy(dir.join("bench_component.dll"), dir.join("Bench.dll"));
    }
}

#[test]
fn main() {
    stage_component();

    let manifest = env!("CARGO_MANIFEST_DIR");

    let mut command = std::process::Command::new("dotnet.exe");
    command.arg("run");
    command.args(["--project", manifest]);
    #[cfg(target_arch = "x86")]
    command.args(["-r", "win-x86", "/p:PlatformTarget=x86"]);
    #[cfg(target_arch = "x86_64")]
    command.args(["-r", "win-x64", "/p:PlatformTarget=x64"]);
    #[cfg(target_arch = "aarch64")]
    command.args(["-r", "win-arm64", "/p:PlatformTarget=arm64"]);
    command.args(["--", "--iterations", "1000", "--generated-com"]);

    let output = command.output().expect("failed to run dotnet");

    assert!(
        output.status.success(),
        "stdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let result = String::from_utf8_lossy(&output.stdout);

    for label in [
        "Create:",
        "Int32:",
        "String:",
        "Add:",
        "Cast:",
        "Object:",
        "Event:",
        "AddRemove:",
        "Vector:",
        "IterateVector:",
        "GetMany:",
        "Map:",
        "Lookup:",
        "VectorView:",
        "MapView:",
        "Reference:",
        "Async:",
        "Error:",
        "OwnerInt32:",
        "OwnerString:",
        "OwnerAdd:",
        "GeneratedComInt32:",
        "GeneratedComString:",
        "GeneratedComAdd:",
        "GeneratedComRawInt32:",
        "GeneratedComRawAdd:",
    ] {
        assert!(
            result.contains(label),
            "missing {label} in stdout:\n{result}"
        );
    }

    assert!(
        result.contains("Leak: 0"),
        "windows-csharp leaked native references (expected Leak: 0):\n{result}"
    );
    assert!(
        result.contains("GeneratedComLeak: 0"),
        "generated COM leaked native references (expected GeneratedComLeak: 0):\n{result}"
    );
}
