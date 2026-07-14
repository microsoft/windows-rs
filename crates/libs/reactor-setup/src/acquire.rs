//! The self-contained deployment model's build-time acquisition: downloading and
//! extracting the Windows App SDK runtime NuGet packages, unpacking the runtime
//! MSIX, and staging the files an app needs next to its executable. This is
//! deliberately dependency-free so it stays lightweight in a consumer's
//! `build.rs`.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RUNTIME_FILES: &str = include_str!("../assets/runtime.txt");
const NUGET_URL: &str = "https://www.nuget.org/api/v2/package/{name}/{version}";
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VER: &str = "1.0.4078.44";
const WEBVIEW2_CORE_DLL: &str = "Microsoft.Web.WebView2.Core.dll";

/// Downloads and extracts a NuGet package into the cache, returning its root.
pub(crate) fn stage_pkg(name: &str, ver: &str, temp: &Path) -> PathBuf {
    let nupkg = temp.join(format!("{name}.{ver}.nupkg"));
    let extract = temp.join(format!("{name}-{ver}"));
    if !nupkg.is_file() {
        dl_nupkg(name, ver, &nupkg);
    }
    if !extract.is_dir() {
        let _ = fs::create_dir_all(&extract);
        extract_tar(&nupkg, &extract, &["--strip-components=1"]);
    }
    extract
}

/// Extracts the runtime MSIX from a staged runtime package.
pub(crate) fn ensure_msix_extracted(runtime: &Path) -> PathBuf {
    let arch = format!("win10-{}", target_arch());
    let msix = runtime
        .join("MSIX")
        .join(&arch)
        .join("Microsoft.WindowsAppRuntime.2.msix");
    let extract = runtime.join(".msix_extract");
    if !extract.is_dir() {
        let _ = fs::create_dir_all(&extract);
        if !msix.is_file() {
            println!("MSIX not found at {}", msix.display());
        } else {
            extract_tar(&msix, &extract, &[]);
        }
    }
    extract
}

/// Copies the runtime files named in `runtime.txt` from `src` into `dest`.
pub(crate) fn copy_runtime_to(src: &Path, dest: &Path) {
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = entry.file_name().into_string().ok() else {
            continue;
        };
        if !RUNTIME_FILES
            .lines()
            .any(|l| l.trim().eq_ignore_ascii_case(&name))
        {
            continue;
        }
        if path.is_file() {
            copy_file(&path, dest, &name);
        } else if path.is_dir() {
            let sub = dest.join(&name);
            let _ = fs::create_dir_all(&sub);
            copy_dir_contents(&path, &sub);
        }
    }
}

/// Deploys `Microsoft.Web.WebView2.Core.dll` next to the executable.
///
/// The XAML `WebView2` control hosted by `windows-webview`'s `reactor` feature
/// loads this WinRT projection assembly at runtime. Unlike the COM-only path
/// (`webview2loader.dll`, supplied by the Evergreen runtime), it is not present
/// on the machine by default, so a self-contained app must carry it alongside
/// the other runtime DLLs.
pub(crate) fn deploy_webview2(temp: &Path, dest: &Path) {
    let pkg = stage_pkg(WEBVIEW2_PKG, WEBVIEW2_VER, temp);
    let src = pkg
        .join(format!("win-{}", target_arch()))
        .join("native_uap")
        .join(WEBVIEW2_CORE_DLL);
    copy_file(&src, dest, WEBVIEW2_CORE_DLL);
}

/// The cache directory for downloaded packages.
pub(crate) fn temp_dir() -> PathBuf {
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
    let _ = fs::create_dir_all(&temp);
    temp
}

pub(crate) fn target_arch() -> &'static str {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86") => "x86",
        _ => "x64",
    }
}

fn copy_dir_contents(src: &Path, dest: &Path) {
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = entry.file_name().into_string().ok() else {
            continue;
        };
        if path.is_file() {
            copy_file(&path, dest, &name);
        } else if path.is_dir() {
            let sub = dest.join(&name);
            let _ = fs::create_dir_all(&sub);
            copy_dir_contents(&path, &sub);
        }
    }
}

fn copy_file(src: &Path, base: &Path, name: &str) {
    if !src.is_file() {
        println!("{name} not found at {}", src.display());
        return;
    }
    let _ = fs::create_dir_all(base);
    let _ = fs::copy(src, base.join(name));
}

fn dl_nupkg(name: &str, ver: &str, dest: &Path) {
    let url = NUGET_URL.replace("{name}", name).replace("{version}", ver);
    println!("Downloading {name} {ver}");
    let curl = env::var_os("SystemRoot")
        .map(|r| PathBuf::from(r).join("System32\\curl.exe"))
        .filter(|p| p.is_file());
    match curl.and_then(|c| {
        Command::new(&c)
            .args(["-s", "-L", "-o", dest.to_str().unwrap(), &url])
            .output()
            .ok()
    }) {
        Some(out) if out.status.success() => {
            println!("Downloaded {name} {ver}");
        }
        _ => {
            println!("Download failed for {name} {ver}");
        }
    }
}

fn extract_tar(src: &Path, dst: &Path, extra: &[&str]) {
    println!("Extracting {} to {}", src.display(), dst.display());
    let tar = env::var_os("SystemRoot")
        .map(|r| PathBuf::from(r).join("System32\\tar.exe"))
        .filter(|p| p.is_file());
    if let Some(t) = tar {
        let _ = Command::new(&t)
            .args(["-xf", src.to_str().unwrap(), "-C", dst.to_str().unwrap()])
            .args(extra)
            .output();
    }
}
