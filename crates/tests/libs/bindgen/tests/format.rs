use std::path::Path;
use std::process::Command;

#[test]
fn formatting() {
    const CASE: &str = "WINDOWS_BINDGEN_FORMAT_TEST";
    let scratch = Path::new(env!("OUT_DIR")).join("format");

    if let Ok(case) = std::env::var(CASE) {
        let winmd = scratch.join("input.winmd");
        let output = scratch.join(format!("{case}.rs"));
        let config = match case.as_str() {
            "invalid" => "newline_style=Invalid",
            "windows" => "newline_style=Windows",
            _ => "newline_style=Unix",
        };

        if case == "windows" {
            windows_bindgen::bindgen([
                "--in",
                winmd.to_str().unwrap(),
                "--out",
                output.to_str().unwrap(),
                "--filter",
                "Test",
                "--flat",
                "--rustfmt",
                config,
            ]);
        } else {
            let mut builder = windows_bindgen::builder();
            builder.input(winmd).output(output).filter("Test").flat();
            if !matches!(case.as_str(), "missing" | "default") {
                builder.rustfmt(config);
            }
            builder.write();
        }
        return;
    }

    std::fs::create_dir_all(&scratch).unwrap();
    windows_rdl::reader()
        .input("input/const.rdl")
        .output(scratch.join("input.winmd"))
        .write()
        .unwrap();
    let expected = std::fs::read_to_string("expected/const.rs").unwrap();

    for case in ["missing", "invalid", "default", "unix", "windows"] {
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "formatting", "--nocapture"])
            .env(CASE, case)
            .current_dir(&scratch);
        if case == "missing" {
            command.env("PATH", "");
        }
        let result = command.output().unwrap();
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert!(result.status.success(), "{case}: {stderr}");
        assert!(stderr.is_empty(), "{case}: {stderr}");

        let output = scratch.join(format!("{case}.rs"));
        let actual = std::fs::read_to_string(&output).unwrap();
        match case {
            "default" | "unix" => assert_eq!(actual, expected),
            "windows" => assert_eq!(actual, expected.replace('\n', "\r\n")),
            "invalid" => assert_eq!(
                actual,
                std::fs::read_to_string(scratch.join("missing.rs")).unwrap()
            ),
            _ => assert!(!actual.is_empty()),
        }

        let result = Command::new("rustc")
            .args([
                "--crate-type",
                "lib",
                "--edition",
                "2024",
                "--emit",
                "metadata",
            ])
            .arg(&output)
            .arg("-o")
            .arg(scratch.join(format!("{case}.rmeta")))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{case}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}
