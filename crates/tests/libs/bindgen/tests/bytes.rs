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
fn filter_files_multiline_groups() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("multiline_filters");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    let output = scratch.join("output.rs");
    let filters = scratch.join("filters.txt");
    let commands = scratch.join("commands.txt");
    windows_rdl::reader()
        .input("input/filter_contracts.rdl")
        .output(&winmd)
        .write()
        .unwrap();

    let single_line = [
        "Test::{ISource::{GetResult}, Color::{Red, Green}, IUnused::{}}",
        "!Test::Color::{Red}",
        "Native::{INativeSource::{GetNativeResult}, NativeColor::{NativeRed, NativeBlue}}",
        "!Native::NativeColor::{NativeRed}",
    ];
    let multiline = "\
Test::{
    ISource::{
        GetResult, // Returned interface remains a shell. }
    },

    // Enum and explicit shell:
    Color::{Red, Green, },
    IUnused::{
    },
}
!Test::Color::{
    Red,
}
Native::{
    INativeSource::{
        GetNativeResult,
    },
    NativeColor::{NativeRed, NativeBlue, },
}
!Native::NativeColor::{
    NativeRed,
} // No final newline";

    for mode in ["minimal", "full", "sys"] {
        let flag = match mode {
            "full" => Some("--full"),
            "sys" => Some("--sys"),
            _ => None,
        };
        let builder = || {
            let mut builder = windows_bindgen::builder();
            builder.input(&winmd).output(&output);
            if mode == "full" {
                builder.full();
            } else if mode == "sys" {
                builder.sys();
            }
            builder
        };
        builder().filters(single_line).write();
        let expected = std::fs::read_to_string(&output).unwrap();

        for newline in ["\n", "\r\n"] {
            let multiline = multiline.replace('\n', newline);
            std::fs::write(&filters, &multiline).unwrap();
            std::fs::write(&commands, format!("--filter {multiline}")).unwrap();
            for form in [
                "filter-file",
                "filter-files",
                "cli-filter-file",
                "command-file",
            ] {
                std::fs::remove_file(&output).unwrap();
                match form {
                    "filter-file" => builder().filter_file(&filters).write(),
                    "filter-files" => builder().filter_files([&filters]).write(),
                    _ => {
                        let mut args = vec![
                            "--in",
                            winmd.to_str().unwrap(),
                            "--out",
                            output.to_str().unwrap(),
                        ];
                        args.extend(flag);
                        if form == "cli-filter-file" {
                            args.extend(["--filter-file", filters.to_str().unwrap()]);
                        } else {
                            args.extend(["--etc", commands.to_str().unwrap()]);
                        }
                        windows_bindgen::bindgen(args);
                    }
                }
                assert_eq!(
                    std::fs::read_to_string(&output).unwrap(),
                    expected,
                    "{mode}, {form}, {newline:?}"
                );
            }
        }
    }
}

#[test]
fn filter_files_trailing_comments() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("filter_comments");
    std::fs::create_dir_all(&scratch).unwrap();
    let filters = scratch.join("filters.txt");
    let output = scratch.join("output.rs");
    windows_bindgen::builder()
        .filter("GetTickCount")
        .output(&output)
        .write();
    let expected = std::fs::read_to_string(&output).unwrap();
    std::fs::write(
        &filters,
        "// ignored {\nGetTickCount \t// ignored } and MissingType\n// final comment",
    )
    .unwrap();
    std::fs::remove_file(&output).unwrap();
    windows_bindgen::builder()
        .filter_file(&filters)
        .output(&output)
        .write();
    assert_eq!(std::fs::read_to_string(output).unwrap(), expected);
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
fn command_file_paths_with_literal_braces() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("command_path_braces");
    std::fs::create_dir_all(&scratch).unwrap();
    let expected = std::fs::read_to_string("expected/const.rs").unwrap();

    for brace in ['}', '{'] {
        let winmd = scratch.join(format!("input{brace}.winmd"));
        let output = scratch.join(format!("output{brace}.rs"));
        let filters = scratch.join(format!("filters{brace}.txt"));
        let nested = scratch.join(format!("nested{brace}.txt"));
        let commands = scratch.join("commands.txt");
        windows_rdl::reader()
            .input("input/const.rdl")
            .output(&winmd)
            .write()
            .unwrap();
        std::fs::write(&filters, "Test\n").unwrap();
        std::fs::write(&nested, "--full\n").unwrap();
        for separator in ["\n", " "] {
            std::fs::write(
                &commands,
                format!(
                    "--out {}{separator}--in {}{separator}--filter-file {}{separator}--etc {}",
                    output.display(),
                    winmd.display(),
                    filters.display(),
                    nested.display()
                ),
            )
            .unwrap();
            if output.exists() {
                std::fs::remove_file(&output).unwrap();
            }
            windows_bindgen::bindgen(["--etc", commands.to_str().unwrap()]);
            assert_eq!(std::fs::read_to_string(&output).unwrap(), expected);
        }
    }
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
