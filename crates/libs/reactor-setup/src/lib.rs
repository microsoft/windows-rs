#![doc = include_str!("../readme.md")]

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod cache;
use cache::*;
mod runtime;

const RUNTIME_PKG: &str = "Microsoft.WindowsAppSDK.Runtime";
const RUNTIME_VER: &str = include_str!("../assets/runtime-version.txt");
const APP_MANIFEST: &str = include_str!("../assets/app.manifest");
const APP_MANIFEST_X86: &str = include_str!("../assets/app-x86.manifest");
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VER: &str = include_str!("../assets/webview2-version.txt");
const WEBVIEW2_CORE_DLL: &str = "Microsoft.Web.WebView2.Core.dll";
const SELF_CONTAINED_MARKER: &str = "windows-reactor-self-contained";

fn assert_windows() {
    match env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("windows") => {}
        Ok(os) => panic!("unsupported target OS: {os}"),
        Err(_) => panic!("CARGO_CFG_TARGET_OS not set"),
    }
}

/// Configures the app to run completely self-contained.
pub fn as_self_contained() {
    assert_windows();

    let out_dir = out_dir();
    let temp_dir = temp_dir();
    let arch = target_arch();
    let runtime = stage_pkg(RUNTIME_PKG, RUNTIME_VER, &temp_dir);
    let extract = ensure_msix_extracted(&runtime, arch);
    let dest = target_dir_from_out(&out_dir);
    copy_runtime_to(&extract, &dest, arch);
    deploy_webview2(&temp_dir, &dest);

    let manifest_path = out_dir.join("app.manifest");
    let mut manifest = if arch == "x86" {
        APP_MANIFEST_X86
    } else {
        APP_MANIFEST
    }
    .to_string();
    let assembly = manifest.find("<assembly").unwrap();
    let opening = assembly + manifest[assembly..].find('>').unwrap() + 1;
    manifest.insert_str(
        opening,
        &format!("<description>{SELF_CONTAINED_MARKER}</description>"),
    );
    fs::write(&manifest_path, manifest).unwrap_or_else(|e| {
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

/// Deploys `Microsoft.Web.WebView2.Core.dll` next to the executable.
///
/// The XAML `WebView2` control hosted by `windows-webview`'s `reactor` feature
/// loads this WinRT projection assembly at runtime. Unlike the COM-only path
/// (`webview2loader.dll`, supplied by the Evergreen runtime), it is not present
/// on the machine by default, so a self-contained app must carry it alongside
/// the other runtime DLLs.
fn deploy_webview2(temp: &Path, dest: &Path) {
    let pkg = stage_pkg(WEBVIEW2_PKG, WEBVIEW2_VER, temp);
    let src = pkg
        .join("runtimes")
        .join(format!("win-{}", target_arch()))
        .join("native_uap")
        .join(WEBVIEW2_CORE_DLL);
    copy_file(&src, dest, WEBVIEW2_CORE_DLL);
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
}

fn temp_dir() -> PathBuf {
    let base = if let Some(p) = env::var_os("LOCALAPPDATA") {
        PathBuf::from(p)
    } else if let Some(p) = env::var_os("XDG_CACHE_HOME") {
        PathBuf::from(p)
    } else if let Some(p) = env::var_os("HOME") {
        PathBuf::from(p).join(".cache")
    } else {
        panic!(
            "could not determine cache directory: LOCALAPPDATA, XDG_CACHE_HOME, and HOME are all unset"
        );
    };
    let temp = base.join("windows-reactor-setup").join("temp");
    fs::create_dir_all(&temp)
        .unwrap_or_else(|error| panic!("cannot create cache `{}`: {error}", temp.display()));
    temp
}

fn copy_runtime_to(src: &Path, dest: &Path, arch: &str) {
    for name in runtime::files(arch) {
        let path = src.join(name);
        let metadata = fs::metadata(&path).unwrap_or_else(|error| {
            panic!(
                "required runtime entry `{}` is missing: {error}",
                path.display()
            )
        });
        if metadata.is_file() {
            copy_file(&path, dest, name);
        } else {
            assert!(
                metadata.is_dir(),
                "unsupported runtime entry `{}`",
                path.display()
            );
            let sub = dest.join(name);
            copy_dir_contents(&path, &sub);
        }
    }
}

fn copy_dir_contents(src: &Path, dest: &Path) {
    fs::create_dir_all(dest)
        .unwrap_or_else(|error| panic!("cannot create `{}`: {error}", dest.display()));
    let entries = fs::read_dir(src)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", src.display()));
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = entry.file_name();
        let ty = entry.file_type().unwrap();
        if ty.is_file() {
            copy_file(&path, dest, &name);
        } else {
            assert!(
                ty.is_dir(),
                "unsupported runtime entry `{}`",
                path.display()
            );
            let sub = dest.join(&name);
            copy_dir_contents(&path, &sub);
        }
    }
}

fn copy_file(src: &Path, base: &Path, name: impl AsRef<OsStr>) {
    fs::create_dir_all(base)
        .unwrap_or_else(|error| panic!("cannot create `{}`: {error}", base.display()));
    let dest = base.join(name.as_ref());
    fs::copy(src, &dest).unwrap_or_else(|error| {
        panic!(
            "cannot copy `{}` to `{}`: {error}",
            src.display(),
            dest.display()
        )
    });
}

fn target_dir_from_out(out: &Path) -> PathBuf {
    env::var_os("PROFILE")
        .and_then(|profile| target_dir_for_profile(out, &profile))
        .unwrap_or_else(|| out.ancestors().nth(3).unwrap_or(out).to_path_buf())
}

fn target_dir_for_profile(out: &Path, profile: &OsStr) -> Option<PathBuf> {
    out.ancestors()
        .find(|path| path.file_name() == Some(profile))
        .map(Path::to_path_buf)
}

fn target_arch() -> &'static str {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86") => "x86",
        Ok("x86_64") => "x64",
        Ok(arch) => panic!("unsupported target architecture: {arch}"),
        Err(error) => panic!("CARGO_CFG_TARGET_ARCH not set: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_profile_in_standard_cargo_out_dir() {
        let out = Path::new(r"C:\repo\target\debug\build\package-hash\out");
        assert_eq!(
            target_dir_for_profile(out, OsStr::new("debug")),
            Some(PathBuf::from(r"C:\repo\target\debug"))
        );
    }

    #[test]
    fn finds_profile_in_split_package_cargo_out_dir() {
        let out = Path::new(r"C:\repo\target\debug\build\package\hash\out");
        assert_eq!(
            target_dir_for_profile(out, OsStr::new("debug")),
            Some(PathBuf::from(r"C:\repo\target\debug"))
        );
    }
}
