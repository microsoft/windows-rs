use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const INTERACTIVE_PKG: &str = "Microsoft.WindowsAppSDK.InteractiveExperiences";
const INTERACTIVE_VER: &str = "2.0.13";
const RUNTIME_PKG: &str = "Microsoft.WindowsAppSDK.Runtime";
const RUNTIME_VER: &str = "2.1.3";

const NUGET_URL: &str = "https://www.nuget.org/api/v2/package/{name}/{version}";
const WINMD_OUT: &str = "winmd";

const WINMD_FILES: &[&str] = &[
    "Microsoft.Foundation.winmd",
    "Microsoft.Graphics.winmd",
    "Microsoft.Security.Authentication.OAuth.winmd",
    "Microsoft.UI.Text.winmd",
    "Microsoft.UI.Xaml.winmd",
    "Microsoft.UI.winmd",
    "Microsoft.Windows.AppLifecycle.winmd",
    "Microsoft.Windows.AppNotifications.Builder.winmd",
    "Microsoft.Windows.AppNotifications.winmd",
    "Microsoft.Windows.ApplicationModel.Background.UniversalBGTask.winmd",
    "Microsoft.Windows.ApplicationModel.Background.winmd",
    "Microsoft.Windows.ApplicationModel.DynamicDependency.winmd",
    "Microsoft.Windows.ApplicationModel.Resources.winmd",
    "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.winmd",
    "Microsoft.Windows.BadgeNotifications.winmd",
    "Microsoft.Windows.Foundation.winmd",
    "Microsoft.Windows.Globalization.winmd",
    "Microsoft.Windows.Management.Deployment.winmd",
    "Microsoft.Windows.Media.Capture.winmd",
    "Microsoft.Windows.PushNotifications.winmd",
    "Microsoft.Windows.Security.AccessControl.winmd",
    "Microsoft.Windows.Storage.Pickers.winmd",
    "Microsoft.Windows.Storage.winmd",
    "Microsoft.Windows.System.Power.winmd",
    "Microsoft.Windows.System.winmd",
];

/// Configures the app to run with a Windows App Runtime dependency.
pub fn as_framework_dependent() {
    let out_dir = out_dir();
    copy_bootstrap_to(&out_dir);
    let (temp_dir, _) = staging_dirs(&out_dir);
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
    let (temp_dir, winmd_dest) = staging_dirs(&out_dir);
    let runtime = stage_pkg(RUNTIME_PKG, RUNTIME_VER, &temp_dir);
    let extract = ensure_msix_extracted(&runtime);
    copy_winmds_to(&extract, &winmd_dest);
    copy_dir_contents(&extract, &target_dir_from_out(&out_dir));
}

#[doc(hidden)]
pub fn stage_runtime() {
    let out_dir = out_dir();
    let (temp_dir, winmd_dest) = staging_dirs(&out_dir);
    let runtime = stage_pkg(RUNTIME_PKG, RUNTIME_VER, &temp_dir);
    let extract = ensure_msix_extracted(&runtime);
    copy_winmds_to(&extract, &winmd_dest);
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
}

fn staging_dirs(out_dir: &Path) -> (PathBuf, PathBuf) {
    let root = find_workspace_root(out_dir);
    let temp = root.join("temp");
    let winmd = root.join(WINMD_OUT);
    let _ = fs::create_dir_all(&temp);
    let _ = fs::create_dir_all(&winmd);
    (temp, winmd)
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

fn copy_winmds_to(extract: &Path, dest: &Path) {
    let mut count = 0usize;
    for name in WINMD_FILES {
        let src = extract.join(name);
        if src.is_file() {
            let _ = fs::copy(&src, dest.join(name));
            count += 1;
        }
    }
    println!("Copied {count} winmd files to {}", dest.display());
}

fn copy_dir_contents(src: &Path, dest: &Path) {
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                copy_file(&path, dest, name);
            }
        }
    }
}

fn copy_file(src: &Path, base: &Path, name: &str) {
    if !src.is_file() {
        println!("{name} not found at {}", src.display());
        return;
    }
    for sub in ["", "examples", "deps"] {
        let mut d = base.to_path_buf();
        if !sub.is_empty() {
            d.push(sub);
        }
        let _ = fs::create_dir_all(&d);
        let _ = fs::copy(src, d.join(name));
    }
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
    for a in out.ancestors() {
        if a.join("Cargo.lock").is_file() {
            return a.to_path_buf();
        }
    }
    for a in out.ancestors() {
        if a.join("Cargo.toml").is_file() {
            return a.to_path_buf();
        }
    }
    out.ancestors().nth(2).unwrap_or(out).to_path_buf()
}

fn target_arch() -> &'static str {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86") => "x86",
        _ => "x64",
    }
}
