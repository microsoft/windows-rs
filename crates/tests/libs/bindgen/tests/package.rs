#[test]
fn package_architecture_host() {
    run(&[None]);
}

#[test]
#[ignore = "requires x86, x64, and ARM64 MSVC Rust targets"]
fn package_architecture_cross_targets() {
    run(&[
        Some("i686-pc-windows-msvc"),
        Some("x86_64-pc-windows-msvc"),
        Some("aarch64-pc-windows-msvc"),
    ]);
}

fn run(targets: &[Option<&str>]) {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join(if targets.len() == 1 {
        "package-architecture-host"
    } else {
        "package-architecture-cross"
    });
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input("input/package_arches.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    let libraries = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\libs");
    for style in ["rich", "minimal", "sys", "extern"] {
        let output = scratch.join(style);
        std::fs::create_dir_all(output.join("src")).unwrap();
        let manifest = output.join("Cargo.toml");
        let dependencies = match style {
            "rich" | "minimal" => format!(
                "windows-core = {{ path = '{}' }}\n",
                libraries.join("core").display()
            ),
            "sys" => format!(
                "windows-link = {{ path = '{}' }}\n",
                libraries.join("link").display()
            ),
            _ => String::new(),
        };
        std::fs::write(
            &manifest,
            format!(
                "[package]\nname = \"package-architecture\"\nversion = \"0.0.0\"\n\
                 edition = \"2024\"\n[workspace]\n[dependencies]\n\
                 {dependencies}\
                 [lints.rust]\nwarnings = \"deny\"\nnonstandard_style = \"allow\"\n\
                 [features]\n# generated features\n"
            ),
        )
        .unwrap();
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Windows.Win32")
            .package()
            .output(&output);
        match style {
            "minimal" => {
                builder.minimal();
            }
            "sys" => {
                builder.sys();
            }
            "extern" => {
                builder.sys().extern_fns();
            }
            _ => {}
        }
        builder.write();
        std::fs::write(output.join("src\\lib.rs"), r#"
pub mod Windows;
pub use Windows::Win32::{narrow as narrow_module, wide as wide_module, constants as constants_module};

#[cfg(target_arch = "x86")]
pub unsafe fn aliases() {
    use Windows::Win32::*;
    let _: i32 = unsafe { native::GetWindowLongPtrA() };
    let _: i32 = unsafe { native::GetWindowLongPtrW() };
    let _: i32 = unsafe { native::SetWindowLongPtrA() };
    let _: i32 = unsafe { native::SetWindowLongPtrW() };
    let _: i32 = unsafe { legacy::GetWindowLongPtrA() };
    let _: i32 = unsafe { split_legacy::SetWindowLongPtrW() };
    let _: i32 = unsafe { complete::GetWindowLongPtrW() };
    let _: Narrow = Narrow { value: 1 };
    assert_eq!(X86, 1);
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub unsafe fn aliases() {
    use Windows::Win32::*;
    let _: isize = unsafe { native::GetWindowLongPtrA() };
    let _: isize = unsafe { native::GetWindowLongPtrW() };
    let _: isize = unsafe { native::SetWindowLongPtrA() };
    let _: isize = unsafe { native::SetWindowLongPtrW() };
    let _: isize = unsafe { legacy::GetWindowLongPtrA() };
    let _: isize = unsafe { split_legacy::SetWindowLongPtrW() };
    let _: isize = unsafe { complete::GetWindowLongPtrW() };
    let _: Wide = Wide { value: 1 };
}

#[cfg(target_arch = "x86_64")]
pub const CHECK: u32 = Windows::Win32::X64;
#[cfg(target_arch = "aarch64")]
pub const CHECK: u32 = Windows::Win32::ARM + Windows::Win32::ARM64;
"#).unwrap();
        for target in targets {
            let mut command = std::process::Command::new("cargo");
            command
                .args([
                    "check",
                    "--quiet",
                    "--offline",
                    "--all-features",
                    "--manifest-path",
                ])
                .arg(&manifest)
                .arg("--target-dir")
                .arg(scratch.join("build"));
            if let Some(target) = target {
                command.args(["--target", target]);
            }
            let result = command.output().unwrap();
            assert!(
                result.status.success(),
                "{style} {target:?}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
    }
}
