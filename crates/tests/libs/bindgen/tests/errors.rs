// Negative tests for windows-bindgen. The golden harness only feeds valid
// input, so this exercises the panic path in `src/io.rs::read_file_lines`,
// reached when a command file cannot be opened.

#[test]
fn nested_type_name_collisions() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("nested_type_name_collisions");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    for body in [
        "struct Outer { struct Inner { value: i32, } } struct Outer_Inner { value: u32, }",
        "struct Outer { struct A { struct B { value: i32, } } struct A_B { value: u32, } }",
        "struct Outer { value: struct { value: i32, }, } struct Outer_0 { value: u32, }",
    ] {
        windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{ struct Safe {{ value: i32, }} {body} }}"
            ))
            .output(&winmd)
            .write()
            .unwrap();
        windows_bindgen::builder()
            .input(&winmd)
            .filter("Test.Safe")
            .flat()
            .output(scratch.join("safe.rs"))
            .write();
        for flat in [false, true] {
            let error = std::panic::catch_unwind(|| {
                let mut builder = windows_bindgen::builder();
                builder
                    .input(&winmd)
                    .filter("Test")
                    .output(scratch.join("output.rs"));
                if flat {
                    builder.flat();
                }
                builder.write();
            })
            .unwrap_err();
            let message = error
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| error.downcast_ref::<&str>().copied())
                .unwrap();
            assert!(message.contains("nested type name collision"), "{message}");
        }
    }
}

#[test]
#[should_panic(expected = "duplicate anonymous field position")]
fn duplicate_anonymous_positions() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("duplicate_anonymous_positions");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Test {
                struct Outer {
                    #[anonymous(0)] struct First { value: i32, }
                    #[anonymous(0)] struct Second { value: u32, }
                }
            }",
        )
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Test.Outer")
        .flat()
        .output(scratch.join("output.rs"))
        .write();
}

#[test]
#[should_panic(expected = "failed to open file")]
fn missing_command_file_panics() {
    let missing = std::env::temp_dir()
        .join("test_bindgen_missing_response_file.rsp")
        .to_string_lossy()
        .into_owned();

    windows_bindgen::bindgen(["--etc", &missing]);
}

#[test]
fn fixed_pointer_callable_rejection() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("fixed_pointer_callable_rejection");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    for declaration in [
        "extern \"C\" fn Bad(value: ptr64<*mut u8>);",
        "extern \"C\" fn Bad() -> Address;",
        "extern \"C\" fn Bad(value: Storage);",
        "struct Bad { value: extern \"C\" fn(Address), }",
        "struct Bad { value: ptr64<*mut Callback>, }",
    ] {
        windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{
                    type Address = ptr64<*mut u8>;
                    struct Storage {{ value: Address, }}
                    extern \"C\" fn Callback();
                    {declaration}
                }}"
            ))
            .output(&winmd)
            .write()
            .unwrap();
        let result = std::panic::catch_unwind(|| {
            windows_bindgen::builder()
                .input(&winmd)
                .filter("Test.Bad")
                .flat()
                .output(scratch.join("output.rs"))
                .write();
        })
        .unwrap_err();
        let message = result
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| result.downcast_ref::<&str>().copied())
            .unwrap();
        assert!(message.contains("fixed-width"), "{declaration}: {message}");
    }
}

#[test]
fn invalid_bitfield_members() {
    let source = include_str!("../input/struct_typed_bitfield.rdl");
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("invalid_bitfield_members");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    let output = scratch.join("output.rs");
    for (replacement, expected) in [
        ("0, 4, MemberType = f32", "unsupported bitfield member type"),
        ("0, 9, MemberType = i8", "exceeds its member type"),
        ("0, 0, MemberType = i32", "exceeds its backing type"),
        ("15, 4, MemberType = i32", "exceeds its backing type"),
    ] {
        windows_rdl::reader()
            .input_text(&source.replace("0, 4, MemberType = Alias", replacement))
            .output(&winmd)
            .write()
            .unwrap();
        let result = std::panic::catch_unwind(|| {
            windows_bindgen::builder()
                .input(&winmd)
                .filter("Test.Bits")
                .flat()
                .output(&output)
                .write();
        })
        .unwrap_err();
        let message = result
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| result.downcast_ref::<&str>().copied())
            .unwrap();
        assert!(message.contains(expected), "{message}");
    }
    std::fs::remove_file(winmd).unwrap();
}

#[test]
fn invalid_named_bitfield_storage() {
    let source = include_str!("../input/struct_named_bitfield.rdl");
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("invalid_named_bitfield_storage");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    let output = scratch.join("output.rs");
    for replacement in ["Value: f32", "Value: *mut u8"] {
        windows_rdl::reader()
            .input_text(&source.replace("Value: u8", replacement))
            .output(&winmd)
            .write()
            .unwrap();
        let result = std::panic::catch_unwind(|| {
            windows_bindgen::builder()
                .input(&winmd)
                .filter("Test.Wrapped")
                .flat()
                .output(&output)
                .write();
        })
        .unwrap_err();
        let message = result
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| result.downcast_ref::<&str>().copied())
            .unwrap();
        assert!(
            message.contains("bitfield storage `Test.Wrapped._bitfield` is not an integer"),
            "{message}"
        );
    }
    std::fs::remove_file(winmd).unwrap();
}

#[test]
#[ignore = "requires the x86 MSVC Rust target"]
fn invalid_target_bitfield_width() {
    let source = include_str!("../input/struct_named_bitfield.rdl");
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("invalid_target_bitfield_width");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    let output = scratch.join("output.rs");
    for (filter, from, to, expected) in [
        (
            "Test.Pointer",
            "High: 28",
            "High: 60",
            "bitfield exceeds target storage width",
        ),
        (
            "Test.PointerMember",
            "16, 32, MemberType = isize",
            "16, 40, MemberType = isize",
            "bitfield exceeds target member width",
        ),
    ] {
        windows_rdl::reader()
            .input_text(&source.replace(from, to))
            .output(&winmd)
            .write()
            .unwrap();
        windows_bindgen::builder()
            .input(&winmd)
            .filter(filter)
            .flat()
            .output(&output)
            .write();
        let result = std::process::Command::new("rustc")
            .args([
                "--crate-type=lib",
                "--edition=2024",
                "--emit=metadata",
                "--target=i686-pc-windows-msvc",
                "-A",
                "nonstandard_style",
            ])
            .arg(&output)
            .arg("-o")
            .arg(scratch.join("output.rmeta"))
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert!(
            !result.status.success(),
            "{filter} must reject the target width"
        );
        assert!(stderr.contains(expected), "{stderr}");
    }
    std::fs::remove_file(winmd).unwrap();
}

#[test]
#[should_panic(expected = "failed to open file")]
fn missing_filter_file_panics() {
    let missing = std::env::temp_dir().join("test_bindgen_missing_filter_file.txt");
    windows_bindgen::builder().filter_file(missing);
}

#[test]
#[should_panic(expected = "invalid option `--unknown`")]
fn invalid_option_panics() {
    windows_bindgen::bindgen(["--unknown"]);
}

#[test]
#[should_panic(expected = "output is required")]
fn missing_output_panics() {
    windows_bindgen::bindgen(["--filter", "GetTickCount"]);
}

#[test]
#[should_panic(expected = "cannot combine `--sys` and `--minimal`")]
fn conflicting_styles_panic() {
    windows_bindgen::bindgen([
        "--out",
        "unused.rs",
        "--filter",
        "GetTickCount",
        "--sys",
        "--minimal",
    ]);
}

#[test]
#[should_panic(expected = "`compose` requires `minimal`")]
fn compose_without_minimal_panics() {
    windows_bindgen::builder()
        .output("unused.rs")
        .filter("GetTickCount")
        .compose("Test.Class")
        .write();
}

#[test]
#[should_panic(expected = "`compose` requires a fully qualified class name")]
fn unqualified_compose_target_panics() {
    windows_bindgen::builder().compose("Class");
}

#[test]
#[should_panic(expected = "`--compose` requires a class name")]
fn missing_compose_target_panics() {
    windows_bindgen::bindgen(["--compose", "--minimal"]);
}

#[test]
#[should_panic(
    expected = "composition target `Test.Base` has no composable factory method selected by the \
                filter"
)]
fn implemented_factory_does_not_select_composition_factory() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("compose_filter");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("out.winmd");
    windows_rdl::reader()
        .input("input/minimal_compose_target.rdl")
        .output(&winmd)
        .write()
        .unwrap();

    windows_bindgen::builder()
        .input(winmd)
        .output(scratch.join("out.rs"))
        .filter("Test.Base")
        .implement("Test.IBaseFactory")
        .compose("Test.Base")
        .minimal()
        .flat()
        .write();
}

fn author_variadic(name: &str) -> (String, String) {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join(name);
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("out.winmd");
    windows_rdl::reader()
        .input("input/variadic_fn_sys.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    (
        winmd.to_string_lossy().into_owned(),
        scratch.join("out.rs").to_string_lossy().into_owned(),
    )
}

#[test]
#[should_panic(
    expected = "windows-bindgen: selected variadic function `Test.VariadicFunc` cannot be projected \
                by rich or minimal bindings; use `--sys` for its raw declaration"
)]
fn exact_rich_variadic_selection_panics() {
    let (winmd, output) = author_variadic("exact_rich_variadic");
    windows_bindgen::bindgen([
        "--in",
        &winmd,
        "--out",
        &output,
        "--flat",
        "--filter",
        "Test.VariadicFunc",
    ]);
}

#[test]
#[should_panic(
    expected = "windows-bindgen: selected variadic function `Test.VariadicFunc` cannot be projected \
                by rich or minimal bindings; use `--sys` for its raw declaration"
)]
fn exact_minimal_variadic_selection_panics() {
    let (winmd, output) = author_variadic("exact_minimal_variadic");
    windows_bindgen::bindgen([
        "--in",
        &winmd,
        "--out",
        &output,
        "--minimal",
        "--flat",
        "--filter",
        "Test.VariadicFunc",
    ]);
}

#[test]
#[should_panic(
    expected = "windows-bindgen: selected variadic function `Test.VariadicFastcall` uses a calling \
                convention that stable Rust cannot represent for C variadics"
)]
fn exact_fastcall_variadic_selection_panics() {
    let (winmd, output) = author_variadic("exact_fastcall_variadic");
    windows_bindgen::bindgen([
        "--in",
        &winmd,
        "--out",
        &output,
        "--sys",
        "--flat",
        "--filter",
        "Test.VariadicFastcall",
    ]);
}

#[test]
#[should_panic(expected = "windows-bindgen: selected variadic function \
                `Windows.Win32.AuthzReportSecurityEvent` cannot be projected by rich or minimal \
                bindings; use `--sys` for its raw declaration")]
fn exact_real_variadic_selection_panics() {
    let output = std::path::Path::new(env!("OUT_DIR")).join("real_variadic.rs");
    windows_bindgen::bindgen([
        "--in",
        "default",
        "--out",
        output.to_str().unwrap(),
        "--flat",
        "--filter",
        "Windows.Win32.AuthzReportSecurityEvent",
    ]);
}
