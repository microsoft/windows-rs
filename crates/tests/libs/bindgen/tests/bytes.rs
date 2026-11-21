#[test]
fn output_paths() {
    const CASE: &str = "WINDOWS_BINDGEN_OUTPUT_TEST";
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("output_paths");

    if let Ok(case) = std::env::var(CASE) {
        let winmd = scratch.join("input.winmd");
        if case.starts_with("cli") {
            let mut args = vec!["--in", winmd.to_str().unwrap(), "--filter", "Test"];
            match case.as_str() {
                "cli-explicit" => args.extend(["--out", "explicit.rs"]),
                "cli-package" => args.push("--package"),
                "cli-missing-value" => args.push("--out"),
                "cli-missing-value-before-filter" => args.extend(["--out", "--filter", "Test"]),
                _ => {}
            }
            windows_bindgen::bindgen(args);
        } else {
            let mut builder = windows_bindgen::builder();
            builder.input(winmd).filter("Test");
            match case.as_str() {
                "explicit" | "explicit-no-env" => {
                    builder.output("explicit.rs");
                }
                "empty-explicit" => {
                    builder.output("");
                }
                "package" => {
                    builder.package();
                }
                _ => {}
            }
            builder.write();
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

    for (case, error) in [
        ("default", None),
        ("cli-default", None),
        ("explicit", None),
        ("cli-explicit", None),
        ("explicit-no-env", None),
        ("missing-env", Some("output is required")),
        ("empty-env", Some("output is required")),
        ("empty-explicit", Some("output is required")),
        ("package", Some("package output is required")),
        ("cli-package", Some("package output is required")),
        ("cli-missing-value", Some("`--out` requires a path")),
        (
            "cli-missing-value-before-filter",
            Some("`--out` requires a path"),
        ),
    ] {
        let directory = scratch.join(case);
        let out_dir = directory.join("cargo-out");
        std::fs::create_dir_all(&directory).unwrap();
        let default = out_dir.join("bindings.rs");
        let explicit = directory.join("explicit.rs");
        if default.exists() {
            std::fs::remove_file(&default).unwrap();
        }
        if explicit.exists() {
            std::fs::remove_file(&explicit).unwrap();
        }

        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "output_paths", "--nocapture"])
            .env(CASE, case)
            .env("OUT_DIR", &out_dir)
            .current_dir(&directory);
        if matches!(case, "missing-env" | "explicit-no-env") {
            command.env_remove("OUT_DIR");
        } else if case == "empty-env" {
            command.env("OUT_DIR", "");
        }
        let result = command.output().unwrap();
        let stderr = String::from_utf8_lossy(&result.stderr);
        let stdout = String::from_utf8_lossy(&result.stdout);
        assert!(!stdout.contains("cargo:"), "{case}: {stdout}");

        if let Some(error) = error {
            assert!(!result.status.success(), "{case} unexpectedly succeeded");
            assert!(stderr.contains(error), "{case}: {stderr}");
            assert!(!default.exists(), "{case} wrote default output");
            assert!(!explicit.exists(), "{case} wrote explicit output");
        } else {
            assert!(result.status.success(), "{case}: {stdout}\n{stderr}");
            let output = if case.contains("explicit") {
                assert!(!default.exists(), "{case} wrote default output");
                explicit
            } else {
                assert!(!explicit.exists(), "{case} wrote explicit output");
                default
            };
            assert_eq!(std::fs::read_to_string(output).unwrap(), expected, "{case}");
        }
    }
}

#[test]
fn bindgen_accepts_metadata_bytes() {
    let temp = std::env::temp_dir();
    let winmd = temp.join("windows_bindgen_bytes.winmd");
    let output = temp.join("windows_bindgen_bytes.rs");
    let filters = temp.join("windows_bindgen_bytes.txt");

    windows_rdl::reader()
        .input_texts([r#"
#[win32]
mod Test {
    #[library("test.dll")]
    extern fn Function() -> u32;
}
"#])
        .output(&winmd)
        .write()
        .unwrap();

    let bytes = std::fs::read(winmd).unwrap();
    std::fs::write(&filters, "  // comment\nTest\n").unwrap();
    windows_bindgen::builder()
        .input_byte_sets([bytes])
        .output(&output)
        .filter_files([filters])
        .write();

    assert!(
        std::fs::read_to_string(output)
            .unwrap()
            .contains("fn Function")
    );
}

#[test]
fn bindgen_accepts_command_files() {
    let temp = std::env::temp_dir();
    let winmd = temp.join("windows_bindgen_commands.winmd");
    let output = temp.join("windows_bindgen_commands.rs");
    let commands = temp.join("windows_bindgen_commands.txt");
    let filters = temp.join("windows_bindgen_commands_filters.txt");

    windows_rdl::reader()
        .input_text(
            r#"
#[win32]
mod Test {
    #[library("test.dll")]
    extern fn Function() -> u32;
}
"#,
        )
        .output(&winmd)
        .write()
        .unwrap();

    std::fs::write(&filters, "Test\n").unwrap();
    std::fs::write(
        &commands,
        format!(
            "// commands\n--in {}\n--out {}\n--filter-file {}\n",
            winmd.display(),
            output.display(),
            filters.display()
        ),
    )
    .unwrap();

    let commands = commands.to_string_lossy();
    windows_bindgen::bindgen(["--etc", commands.as_ref()]);

    assert!(
        std::fs::read_to_string(output)
            .unwrap()
            .contains("fn Function")
    );
}

#[test]
fn command_files_can_be_nested_and_combined() {
    let temp = std::env::temp_dir();
    let winmd = temp.join("windows_bindgen_nested_commands.winmd");
    let output = temp.join("windows_bindgen_nested_commands.rs");
    let input_commands = temp.join("windows_bindgen_input_commands.txt");
    let output_commands = temp.join("windows_bindgen_output_commands.txt");
    let nested_commands = temp.join("windows_bindgen_nested_commands.txt");

    windows_rdl::reader()
        .input_text(
            r#"
#[win32]
mod Test {
    #[library("test.dll")]
    extern fn Function() -> u32;
}
"#,
        )
        .output(&winmd)
        .write()
        .unwrap();

    std::fs::write(&input_commands, format!("--in {}\n", winmd.display())).unwrap();
    std::fs::write(
        &nested_commands,
        format!("--out {}\n--filter Test\n", output.display()),
    )
    .unwrap();
    std::fs::write(
        &output_commands,
        format!("--etc {}\n", nested_commands.display()),
    )
    .unwrap();

    let input_commands = input_commands.to_string_lossy();
    let output_commands = output_commands.to_string_lossy();
    windows_bindgen::bindgen(["--etc", input_commands.as_ref(), output_commands.as_ref()]);

    assert!(
        std::fs::read_to_string(output)
            .unwrap()
            .contains("fn Function")
    );
}
