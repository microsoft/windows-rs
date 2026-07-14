//! Build-script helper that configures a [`windows-reactor`] app to run against
//! the Windows App SDK runtime. The public functions pick a deployment model;
//! acquiring the runtime bits (NuGet download, MSIX extraction, file staging)
//! lives in the [`acquire`] module.
//!
//! [`windows-reactor`]: https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor

mod acquire;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const RUNTIME_PKG: &str = "Microsoft.WindowsAppSDK.Runtime";
const RUNTIME_VER: &str = "2.2.0";
const APP_MANIFEST: &str = include_str!("../assets/app.manifest");

fn assert_windows() {
    match env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("windows") => {}
        Ok(os) => panic!("unsupported target OS: {os}"),
        Err(_) => panic!("CARGO_CFG_TARGET_OS not set"),
    }
}

/// Configures the app to run with a Windows App Runtime dependency.
pub fn as_framework_dependent() {
    assert_windows();
    as_framework_dependent_impl("");
}

/// Configures an example to run with a Windows App Runtime dependency.
pub fn as_example() {
    assert_windows();
    as_framework_dependent_impl("examples");
}

fn as_framework_dependent_impl(subdir: &str) {
    let out_dir = out_dir();
    let dest = if subdir.is_empty() {
        target_dir_from_out(&out_dir)
    } else {
        target_dir_from_out(&out_dir).join(subdir)
    };
    copy_bootstrap_to(&dest);
    fs::write(
        dest.join("resources.pri"),
        include_bytes!("../assets/resources.pri"),
    )
    .unwrap_or_else(|e| panic!("failed to write resources.pri to {}: {e}", dest.display()));
}

/// Configures the app to run completely self-contained.
pub fn as_self_contained() {
    assert_windows();

    let out_dir = out_dir();
    let temp_dir = acquire::temp_dir();
    let runtime = acquire::stage_pkg(RUNTIME_PKG, RUNTIME_VER, &temp_dir);
    let extract = acquire::ensure_msix_extracted(&runtime);
    let target_dir = target_dir_from_out(&out_dir);
    acquire::copy_runtime_to(&extract, &target_dir);
    acquire::deploy_webview2(&temp_dir, &target_dir);

    let manifest_path = out_dir.join("app.manifest");
    fs::write(&manifest_path, APP_MANIFEST).unwrap_or_else(|e| {
        panic!(
            "failed to write manifest to {}: {e}",
            manifest_path.display()
        )
    });
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("CARGO_CFG_TARGET_ENV not set");
    let target_abi = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default();
    match (target_env.as_str(), target_abi.as_str()) {
        ("msvc", _) => {
            println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
                manifest_path.display()
            );
        }
        ("gnu", "llvm") => {
            println!("cargo:rustc-link-arg-bins=-Wl,/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg-bins=-Wl,/MANIFESTINPUT:{}",
                manifest_path.display()
            );
        }
        _ => panic!("unsupported target environment: {target_env}{target_abi}"),
    }
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
}

fn copy_bootstrap_to(dest: &Path) {
    let bytes: &[u8] = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86") => include_bytes!("../bootstrap/x86/Microsoft.WindowsAppRuntime.Bootstrap.dll"),
        Ok("x86_64") => {
            include_bytes!("../bootstrap/x64/Microsoft.WindowsAppRuntime.Bootstrap.dll")
        }
        Ok("aarch64") => {
            include_bytes!("../bootstrap/arm64/Microsoft.WindowsAppRuntime.Bootstrap.dll")
        }
        Ok(arch) => panic!("Unsupported bootstrap target architecture: {arch}"),
        Err(_) => panic!("CARGO_CFG_TARGET_ARCH not set"),
    };
    let _ = fs::create_dir_all(dest);
    let _ = fs::write(
        dest.join("microsoft.windowsappruntime.bootstrap.dll"),
        bytes,
    );
}

fn target_dir_from_out(out: &Path) -> PathBuf {
    out.ancestors().nth(3).unwrap_or(out).to_path_buf()
}
