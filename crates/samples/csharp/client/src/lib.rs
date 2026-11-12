#[test]
fn test() {
    // A separate build gives the native DLL a stable output path for the .NET loader.
    let target_dir = std::path::Path::new(env!("OUT_DIR")).join("component");
    let output = std::process::Command::new(env!("CARGO"))
        .args(["build", "--quiet", "--lib", "--manifest-path"])
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../component/Cargo.toml"
        ))
        .args(["--target", env!("TARGET"), "--target-dir"])
        .arg(&target_dir)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut paths = vec![target_dir.join(env!("TARGET")).join("debug")];
    paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap()));

    let mut command = std::process::Command::new("dotnet.exe");
    command
        .arg("run")
        .env("PATH", std::env::join_paths(paths).unwrap());

    #[cfg(target_arch = "x86")]
    command.args("-r win-x86 /p:PlatformTarget=x86".split_whitespace());

    let output = command.output().expect("failed to run dotnet");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let result = String::from_utf8_lossy(&output.stdout);
    let result: Vec<&str> = result.lines().collect();

    assert_eq!(result, ["Add: 9", "Concat: hello world"]);
}
