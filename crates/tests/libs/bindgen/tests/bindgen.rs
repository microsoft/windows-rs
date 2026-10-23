include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

fn run(name: &str) {
    let input_path = format!("input/{name}.rdl");
    let expected_path = format!("expected/{name}.rs");
    let scratch = format!("{}/{name}", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    // Extract args from `//!` comment lines at the top.
    let contents = std::fs::read_to_string(&input_path).unwrap();
    let mut args = Vec::new();

    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("//!") {
            for arg in rest.split_whitespace() {
                args.push(arg.to_string());
            }
        }
    }

    let winmd = format!("{scratch}/out.winmd");

    windows_rdl::reader()
        .input(&input_path)
        .output(&winmd)
        .write()
        .unwrap();

    let out_rs = format!("{scratch}/out.rs");
    let mut cli: Vec<String> = vec!["--in".into(), winmd];
    cli.extend(args);
    cli.push("--out".into());
    cli.push(out_rs.clone());

    windows_bindgen::bindgen(cli);

    let actual = std::fs::read_to_string(&out_rs).unwrap();
    std::fs::write(&expected_path, &actual).unwrap();
}

#[test]
fn nested_dependency_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("nested_dependency_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/struct_nested_dependency.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["default", "minimal", "sys"] {
        for (flat, filter) in [
            (false, "Consumer.Value"),
            (true, "Consumer.Value"),
            (false, "Consumer"),
        ] {
            let output = scratch.join(format!("{style}-{flat}-{filter}.rs"));
            let mut builder = windows_bindgen::builder();
            builder.input(&winmd).filter(filter).output(&output);
            match style {
                "minimal" => {
                    builder.minimal();
                }
                "sys" => {
                    builder.sys();
                }
                _ => {}
            }
            if flat {
                builder.flat();
            }
            builder.write();
            let result = std::process::Command::new("rustc")
                .args([
                    "--crate-name",
                    "nested_dependency",
                    "--crate-type=lib",
                    "--edition=2024",
                    "--emit=metadata",
                    "-Dwarnings",
                    "-Anonstandard_style",
                ])
                .arg(&output)
                .arg("-o")
                .arg(scratch.join("output.rmeta"))
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{style} {flat} {filter}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
    }
}

#[test]
fn fixed_pointer_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("fixed_pointer_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/struct_fixed_pointer.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["default", "minimal", "sys"] {
        for package in [false, true] {
            for selection in ["fixture", "fixture.DeepOnly"] {
                let output = scratch.join(format!("{style}-{package}-{selection}"));
                std::fs::create_dir_all(output.join("src")).unwrap();
                let manifest = output.join("Cargo.toml");
                std::fs::write(
                    &manifest,
                    "[package]\nname = \"fixed-pointer-styles\"\nversion = \"0.0.0\"\n\
                 edition = \"2024\"\n[workspace]\n\
                 [lints.rust]\nwarnings = \"deny\"\nnonstandard_style = \"allow\"\n\
                 [features]\n# generated features\n",
                )
                .unwrap();
                let library = output.join("src\\lib.rs");
                let mut builder = windows_bindgen::builder();
                builder
                    .input(&winmd)
                    .filter(&format!("Windows.Win32.{selection}"));
                match style {
                    "minimal" => {
                        builder.minimal();
                    }
                    "sys" => {
                        builder.sys();
                    }
                    _ => {}
                }
                if package {
                    builder.package().output(&output);
                    std::fs::write(&library, "pub mod Windows;\n").unwrap();
                } else {
                    builder.output(&library);
                }
                builder.write();
                let mut command = std::process::Command::new("cargo");
                command
                    .args(["check", "--quiet", "--manifest-path"])
                    .arg(&manifest);
                if package {
                    command.args(["--no-default-features", "--features", "fixture"]);
                }
                let result = command.output().unwrap();
                assert!(
                    result.status.success(),
                    "{style} {package} {selection}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }
    }
}

#[test]
fn named_bitfield_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("named_bitfield_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/struct_named_bitfield.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    let core = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\libs\\core");
    let manifest = scratch.join("Cargo.toml");
    for style in ["minimal", "sys"] {
        let output = scratch.join(format!("{style}.rs"));
        let mut builder = windows_bindgen::builder();
        builder.input(&winmd).filter("Test").flat().output(&output);
        if style == "minimal" {
            builder.minimal();
        } else {
            builder.sys();
        }
        builder.write();
        std::fs::write(
            &manifest,
            format!(
                "[package]\nname = \"named-bitfield-styles\"\nversion = \"0.0.0\"\n\
                 edition = \"2024\"\n[workspace]\n[lib]\npath = \"{style}.rs\"\n\
                 [dependencies]\nwindows-core = {{ path = '{}' }}\n\
                 [lints.rust]\nwarnings = \"deny\"\nnonstandard_style = \"allow\"\n",
                core.display()
            ),
        )
        .unwrap();
        let result = std::process::Command::new("cargo")
            .args(["check", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(scratch.join("target"))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{style}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
#[ignore = "requires the x86, x64, and ARM64 MSVC Rust targets"]
fn enum_constant_shadow_styles() {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join("enum_constant_shadow_styles");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/enum_constant_shadow.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    for style in ["default", "minimal", "sys"] {
        for filter in [
            "Test",
            "Test.Pool",
            "Test.Pool::{Default,Aligned,Unshadowed,Replaced}",
            "Test.Default",
        ] {
            let output = scratch.join("bindings.rs");
            let mut builder = windows_bindgen::builder();
            builder.input(&winmd).filter(filter).flat().output(&output);
            match style {
                "minimal" => {
                    builder.minimal();
                }
                "sys" => {
                    builder.sys();
                }
                _ => {}
            }
            builder.write();
            let mut rust = std::fs::read_to_string(&output).unwrap();
            rust.push_str("\nconst _: () = { assert!(Default == if cfg!(target_arch = \"aarch64\") { 512 } else { 0 }); };\n");
            if filter != "Test.Default" {
                rust.push_str(
                    "\nconst _: () = {\n\
                    assert!(Aligned == if cfg!(target_arch = \"aarch64\") { 516 } else { 4 });\n\
                    assert!(Unshadowed == 8 && Replaced == 10);\n\
                };\n",
                );
            }
            if filter == "Test" {
                rust.push_str(
                    "\n#[cfg(target_arch = \"x86\")] const PartialValue: i32 = -1;\n\
                    const _: () = {\n\
                    #[cfg(target_arch = \"x86_64\")] assert!(PartialValue == 20);\n\
                    #[cfg(target_arch = \"aarch64\")] assert!(PartialValue == 21);\n\
                    #[cfg(target_arch = \"x86\")] assert!(PartialValue == -1);\n\
                    assert!(ScopedValue == 41 && Scoped::ScopedValue.0 == 40);\n\
                };\n",
                );
            }
            std::fs::write(&output, rust).unwrap();
            for target in [
                "i686-pc-windows-msvc",
                "x86_64-pc-windows-msvc",
                "aarch64-pc-windows-msvc",
            ] {
                let result = std::process::Command::new("rustc")
                    .args([
                        "--crate-name",
                        "enum_constant_shadow",
                        "--crate-type=lib",
                        "--emit=metadata",
                        "--edition=2024",
                        "--target",
                        target,
                    ])
                    .arg(&output)
                    .arg("-o")
                    .arg(scratch.join("output.rmeta"))
                    .output()
                    .unwrap();
                assert!(
                    result.status.success(),
                    "{style} {filter} {target}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }
    }
}

#[test]
fn real_variadic_exports_are_raw_sys_only() {
    let rich =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/authz/mod.rs").unwrap();
    assert!(!rich.contains("pub unsafe fn AuthzReportSecurityEvent("));

    let sys = std::fs::read_to_string("../../../libs/sys/src/Windows/Win32/authz/mod.rs").unwrap();
    assert!(sys.contains(
        "windows_link::link!(\"authz.dll\" \"C\" fn AuthzReportSecurityEvent(dwflags : super::DWORD, \
         heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : super::DWORD, pusersid : \
         super::PSID, dwcount : super::DWORD, ...) -> windows_sys::core::BOOL);"
    ));
}

#[test]
fn real_inout_parameters_remain_output_capable() {
    let wingdi =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/wingdi/mod.rs").unwrap();
    assert!(wingdi.contains(
        "pub unsafe fn DPtoLP(hdc: super::HDC, lppt: &mut [super::POINT]) -> \
         windows_core::BOOL"
    ));

    let dpapi =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/dpapi/mod.rs").unwrap();
    assert!(dpapi.contains(
        "pub unsafe fn CryptProtectMemory(pdatain: super::LPVOID, cbdatain: super::DWORD, \
         dwflags: super::DWORD) -> windows_core::BOOL"
    ));
    let minwindef =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/minwindef/mod.rs")
            .unwrap();
    assert!(minwindef.contains("pub type LPVOID = *mut core::ffi::c_void;"));
}
