use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const INTERACTIVE_PKG: &str = "Microsoft.WindowsAppSDK.InteractiveExperiences";
const INTERACTIVE_VER: &str = "2.0.13";
const RUNTIME_PKG: &str = "Microsoft.WindowsAppSDK.Runtime";
const RUNTIME_VER: &str = "2.1.3";
const RUNTIME_FILES: &str = include_str!("../assets/runtime.txt");
const APP_MANIFEST: &str = include_str!("../assets/app.manifest");
const NUGET_URL: &str = "https://www.nuget.org/api/v2/package/{name}/{version}";

/// Configures the app to run with a Windows App Runtime dependency.
pub fn as_framework_dependent() {
    let out_dir = out_dir();
    copy_bootstrap_to(&out_dir);
    let temp_dir = temp_dir(&out_dir);
    let arch = format!("win-{}", target_arch());
    let interactive = stage_pkg(INTERACTIVE_PKG, INTERACTIVE_VER, &temp_dir);
    copy_file(
        &interactive
            .join(&arch)
            .join("native")
            .join("Microsoft.UI.pri"),
        &target_dir_from_out(&out_dir),
        "resources.pri",
    );
}

/// Configures the app to run completely self-contained.
pub fn as_self_contained() {
    let out_dir = out_dir();
    let temp_dir = temp_dir(&out_dir);
    let runtime = stage_pkg(RUNTIME_PKG, RUNTIME_VER, &temp_dir);
    let extract = ensure_msix_extracted(&runtime);
    copy_runtime_to(&extract, &target_dir_from_out(&out_dir));

    let manifest_path = out_dir.join("app.manifest");
    fs::write(&manifest_path, APP_MANIFEST).unwrap_or_else(|e| {
        panic!(
            "failed to write manifest to {}: {e}",
            manifest_path.display()
        )
    });
    println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
        manifest_path.display()
    );
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
}

fn temp_dir(out_dir: &Path) -> PathBuf {
    let temp = find_workspace_root(out_dir).join("temp");
    let _ = fs::create_dir_all(&temp);
    temp
}

fn copy_bootstrap_to(out_dir: &Path) {
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
    let target = target_dir_from_out(out_dir);
    let _ = fs::create_dir_all(&target);
    let _ = fs::write(
        target.join("Microsoft.WindowsAppRuntime.Bootstrap.dll"),
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

fn find_workspace_root(out: &Path) -> PathBuf {
    out.ancestors()
        .find(|a| a.join("Cargo.lock").is_file())
        .or_else(|| out.ancestors().find(|a| a.join("Cargo.toml").is_file()))
        .unwrap_or_else(|| out.ancestors().nth(2).unwrap_or(out))
        .to_path_buf()
}

fn target_arch() -> &'static str {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86") => "x86",
        _ => "x64",
    }
}
