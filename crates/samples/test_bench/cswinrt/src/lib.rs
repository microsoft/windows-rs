// Focused probe for the CsWinRT 3 preview's delegate marshalling. The package currently builds but
// fails at the first event subscription because it tries to load an assembly named `WinRT.Interop`
// that is not included in the package. Keep this ignored until a preview ships that passes it.
#[cfg(test)]
fn stage_component() {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let _ = std::fs::copy(dir.join("bench_component.dll"), dir.join("Bench.dll"));
    }
}

#[test]
#[ignore = "CsWinRT 3 preview omits WinRT.Interop and cannot marshal delegates"]
fn main() {
    stage_component();

    let manifest = env!("CARGO_MANIFEST_DIR");

    let mut command = std::process::Command::new("dotnet.exe");
    command.arg("run");
    command.args(["--project", manifest]);
    let output = command.output().expect("failed to run dotnet");

    assert!(
        output.status.success(),
        "stdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("CsWinRT 3 delegate marshalling succeeded.")
    );
}
