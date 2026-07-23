use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RUNTIME_PKG: &str = "Microsoft.WindowsAppSDK.Runtime";
const RUNTIME_VER: &str = "2.3.1";
const RUNTIME_FILES: &str = include_str!("../assets/runtime.txt");
const APP_MANIFEST: &str = include_str!("../assets/app.manifest");
const NUGET_URL: &str = "https://www.nuget.org/api/v2/package/{name}/{version}";
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VER: &str = "1.0.4078.44";
const WEBVIEW2_CORE_DLL: &str = "Microsoft.Web.WebView2.Core.dll";

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
    let temp_dir = temp_dir();
    let runtime = stage_pkg(RUNTIME_PKG, RUNTIME_VER, &temp_dir);
    let extract = ensure_msix_extracted(&runtime);
    copy_runtime_to(&extract, &target_dir_from_out(&out_dir));
    deploy_webview2(&temp_dir, &target_dir_from_out(&out_dir));

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
    let _ = fs::create_dir_all(&temp);
    temp
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

fn ensure_msix_extracted(runtime: &Path) -> PathBuf {
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

fn copy_runtime_to(src: &Path, dest: &Path) {
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

fn stage_pkg(name: &str, ver: &str, temp: &Path) -> PathBuf {
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

fn target_dir_from_out(out: &Path) -> PathBuf {
    out.ancestors().nth(3).unwrap_or(out).to_path_buf()
}

fn target_arch() -> &'static str {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86") => "x86",
        _ => "x64",
    }
}
