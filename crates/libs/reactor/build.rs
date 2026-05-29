use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const WINAPPSDK_FOUNDATION_PACKAGE: &str = "Microsoft.WindowsAppSDK.Foundation";
const WINAPPSDK_FOUNDATION_VERSION: &str = "2.0.21";
const WINAPPSDK_INTERACTIVE_PACKAGE: &str = "Microsoft.WindowsAppSDK.InteractiveExperiences";
const WINAPPSDK_INTERACTIVE_VERSION: &str = "2.0.13";
const WINAPPSDK_RUNTIME_PACKAGE: &str = "Microsoft.WindowsAppSDK.Runtime";
const WINAPPSDK_RUNTIME_VERSION: &str = "2.1.3";

const NUGET_URL_TEMPLATE: &str = "https://www.nuget.org/api/v2/package/{name}/{version}";
const WINMD_DEST_DIR: &str = "winmd";

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

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let workspace_root = PathBuf::from("/git/windows-rs/");
    let temp_dir = workspace_root.join("temp");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");

    let foundation_dir = stage_package(
        WINAPPSDK_FOUNDATION_PACKAGE,
        WINAPPSDK_FOUNDATION_VERSION,
        &temp_dir,
    );
    let interactive_dir = stage_package(
        WINAPPSDK_INTERACTIVE_PACKAGE,
        WINAPPSDK_INTERACTIVE_VERSION,
        &temp_dir,
    );
    let runtime_pkg_dir = stage_package(
        WINAPPSDK_RUNTIME_PACKAGE,
        WINAPPSDK_RUNTIME_VERSION,
        &temp_dir,
    );
    let winmd_dest = workspace_root.join(WINMD_DEST_DIR);
    fs::create_dir_all(&winmd_dest).expect("Failed to create winmd destination directory");
    populate_winmd_files(&runtime_pkg_dir, &winmd_dest);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR set by cargo"));
    let target_dir = target_dir_from_out(&out_dir);
    let dest_dirs = [
        target_dir.clone(),
        target_dir.join("examples"),
        target_dir.join("deps"),
    ];

    let arch = format!("win-{}", target_arch());
    let bootstrap_src = foundation_dir
        .join(&arch)
        .join("native")
        .join("Microsoft.WindowsAppRuntime.Bootstrap.dll");

    if bootstrap_src.is_file() {
        println!("cargo:rerun-if-changed={}", bootstrap_src.display());
        for dest_dir in &dest_dirs {
            let dest = dest_dir.join("Microsoft.WindowsAppRuntime.Bootstrap.dll");
            if let Err(e) = fs::copy(&bootstrap_src, &dest) {
                println!(
                    "cargo:warning=Failed to copy bootstrap DLL to {}: {}",
                    dest.display(),
                    e
                );
            }
        }
    } else {
        println!(
            "cargo:warning=Microsoft.WindowsAppRuntime.Bootstrap.dll not found at {}",
            bootstrap_src.display()
        );
    }

    let pri_src = interactive_dir
        .join(&arch)
        .join("native")
        .join("Microsoft.UI.pri");

    if pri_src.is_file() {
        println!("cargo:rerun-if-changed={}", pri_src.display());
        for dest_dir in &dest_dirs {
            let dest = dest_dir.join("resources.pri");
            if let Err(e) = fs::copy(&pri_src, &dest) {
                println!(
                    "cargo:warning=Failed to copy framework PRI to {}: {}",
                    dest.display(),
                    e
                );
            }
        }
    } else {
        println!(
            "cargo:warning=Microsoft.UI.pri not found at {}. \
             XamlControlsResources will fail to load unless a resources.pri \
             file is present next to the executable.",
            pri_src.display()
        );
    }
}

fn stage_package(name: &str, version: &str, temp_dir: &Path) -> PathBuf {
    let nupkg_path = temp_dir.join(format!("{name}.{version}.nupkg"));
    let extract_dir = temp_dir.join(format!("{name}-{version}"));

    if !nupkg_path.is_file() {
        download_nupkg(name, version, &nupkg_path);
    }

    if !extract_dir.is_dir() {
        fs::create_dir_all(&extract_dir).expect("Failed to create extract directory");
        extract_archive(&nupkg_path, &extract_dir, &["--strip-components=1"]);
    }

    if !extract_dir.is_dir() || fs::read_dir(&extract_dir).map_or(true, |r| r.count() == 0) {
        println!(
            "cargo:warning=Extraction of {} to {} produced no files",
            nupkg_path.display(),
            extract_dir.display()
        );
    }

    extract_dir
}

fn download_nupkg(name: &str, version: &str, dest: &Path) {
    let url = NUGET_URL_TEMPLATE
        .replace("{name}", name)
        .replace("{version}", version);

    println!("cargo:warning=Downloading {name} version {version} from {url}");

    let curl_path = windows_system32().join("curl.exe");
    if !curl_path.is_file() {
        println!(
            "cargo:warning=curl.exe not found at {}",
            curl_path.display()
        );
        return;
    }

    let status = Command::new(&curl_path)
        .args([
            "-s",
            "-L",
            "-o",
            dest.to_str().expect("invalid dest path"),
            &url,
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Downloaded {name} version {version} successfully");
        }
        Ok(s) => {
            println!(
                "cargo:warning=Failed to download {name} version {version}: exit code {}",
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            println!("cargo:warning=Failed to run curl: {e}");
        }
    }
}

fn target_dir_from_out(out_dir: &Path) -> PathBuf {
    out_dir.ancestors().nth(3).unwrap_or(out_dir).to_path_buf()
}

fn windows_system32() -> PathBuf {
    PathBuf::from(env::var("SystemRoot").unwrap()).join("System32")
}

fn target_arch() -> &'static str {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "arm64",
        Ok("x86") => "x86",
        _ => "x64",
    }
}

fn populate_winmd_files(runtime_dir: &Path, winmd_dest: &Path) {
    let msix_arch = format!("win10-{}", target_arch());
    let msix_path = runtime_dir
        .join("MSIX")
        .join(msix_arch)
        .join("Microsoft.WindowsAppRuntime.2.msix");

    if !msix_path.is_file() {
        println!(
            "cargo:warning=Microsoft.WindowsAppRuntime.2.msix not found at {}",
            msix_path.display()
        );

        return;
    }

    let msix_extract_dir = runtime_dir.join(".msix_extract");
    if !msix_extract_dir.is_dir() {
        println!("cargo:warning=Processing MSIX: {}", msix_path.display());

        fs::create_dir_all(&msix_extract_dir).expect("Failed to create MSIX extract directory");
        extract_archive(&msix_path, &msix_extract_dir, &[]);

        for winmd_name in WINMD_FILES {
            let src = msix_extract_dir.join(winmd_name);
            let dest = winmd_dest.join(winmd_name);

            if !src.is_file() {
                println!("cargo:warning=Winmd not found in MSIX: {winmd_name}");
                continue;
            }

            if let Err(e) = fs::copy(&src, &dest) {
                println!(
                    "cargo:warning=Failed to copy {} to {}: {e}",
                    src.display(),
                    dest.display()
                );
            }
        }

        println!(
            "cargo:warning=Copied winmd files to {}",
            winmd_dest.display()
        );
    }
}

fn extract_archive(archive_path: &Path, dest_path: &Path, extra_args: &[&str]) {
    println!(
        "cargo:warning=Extracting archive {} to {}",
        archive_path.display(),
        dest_path.display()
    );

    let tar_path = windows_system32().join("tar.exe");
    if !tar_path.is_file() {
        println!("cargo:warning=tar.exe not found at {}", tar_path.display());
        return;
    }

    let status = Command::new(&tar_path)
        .args([
            "-xf",
            archive_path.to_str().expect("invalid archive path"),
            "-C",
            dest_path.to_str().expect("invalid destination path"),
        ])
        .args(extra_args)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Extracted archive successfully");
        }
        Ok(s) => {
            println!(
                "cargo:warning=Failed to extract archive: {}",
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            println!("cargo:warning=Failed to run tar: {e}");
        }
    }
}
