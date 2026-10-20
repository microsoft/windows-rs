#![doc = include_str!("../readme.md")]

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

const RUNTIME_PKG: &str = "Microsoft.WindowsAppSDK.Runtime";
const RUNTIME_VER: &str = "2.5.1";
const RUNTIME_FILES: &str = include_str!("../assets/runtime.txt");
const APP_MANIFEST: &str = include_str!("../assets/app.manifest");
const NUGET_URL: &str = "https://www.nuget.org/api/v2/package/{name}/{version}";
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VER: &str = "1.0.4078.44";
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
    stage_self_contained().unwrap_or_else(|error| panic!("runtime staging failed: {error}"));
}

fn stage_self_contained() -> io::Result<()> {
    assert_windows();

    let out_dir = out_dir();
    let temp_dir = temp_dir()?;
    let arch = target_arch();
    let runtime = stage_pkg(RUNTIME_PKG, RUNTIME_VER, &temp_dir, &runtime_msix(arch))?;
    let extract = ensure_msix_extracted(&runtime, arch)?;
    let dest = target_dir_from_out(&out_dir);
    fs::create_dir_all(&dest)
        .map_err(|error| file_error("create output directory", &dest, error))?;
    let _destination_lock = lock_file(&dest.join(".windows-reactor-setup.lock"))?;
    copy_runtime_to(&extract, &dest)?;
    deploy_webview2(&temp_dir, &dest, arch)?;

    let manifest_path = out_dir.join("app.manifest");
    let mut manifest = APP_MANIFEST.to_string();
    let assembly = manifest.find("<assembly").unwrap();
    let opening = assembly + manifest[assembly..].find('>').unwrap() + 1;
    manifest.insert_str(
        opening,
        &format!("<description>{SELF_CONTAINED_MARKER}</description>"),
    );
    fs::write(&manifest_path, manifest)
        .map_err(|error| file_error("write manifest", &manifest_path, error))?;
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
    Ok(())
}

/// Deploys `Microsoft.Web.WebView2.Core.dll` next to the executable.
///
/// The XAML `WebView2` control hosted by `windows-webview`'s `reactor` feature
/// loads this WinRT projection assembly at runtime. Unlike the COM-only path
/// (`webview2loader.dll`, supplied by the Evergreen runtime), it is not present
/// on the machine by default, so a self-contained app must carry it alongside
/// the other runtime DLLs.
fn deploy_webview2(temp: &Path, dest: &Path, arch: &str) -> io::Result<()> {
    let relative = PathBuf::from(format!("win-{arch}"))
        .join("native_uap")
        .join(WEBVIEW2_CORE_DLL);
    let pkg = stage_pkg(WEBVIEW2_PKG, WEBVIEW2_VER, temp, &relative)?;
    copy_entry(&pkg.join(relative), &dest.join(WEBVIEW2_CORE_DLL))
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
}

fn temp_dir() -> io::Result<PathBuf> {
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
    // Keep caches published by this implementation separate from unchecked legacy entries.
    let temp = base.join("windows-reactor-setup").join("temp").join("v1");
    fs::create_dir_all(&temp).map_err(|error| file_error("create cache", &temp, error))?;
    Ok(temp)
}

fn runtime_msix(arch: &str) -> PathBuf {
    PathBuf::from("MSIX")
        .join(format!("win10-{arch}"))
        .join("Microsoft.WindowsAppRuntime.2.msix")
}

fn ensure_msix_extracted(runtime: &Path, arch: &str) -> io::Result<PathBuf> {
    cached_dir(runtime, &format!(".msix_extract-{arch}"), |partial| {
        extract_tar(&runtime.join(runtime_msix(arch)), partial, &[])?;
        require_file(&partial.join("Microsoft.WindowsAppRuntime.dll"))?;
        require_file(&partial.join("Microsoft.ui.xaml.dll"))
    })
}

fn copy_runtime_to(src: &Path, dest: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src).map_err(|error| file_error("read directory", src, error))? {
        let entry = entry.map_err(|error| file_error("read directory entry", src, error))?;
        let name = entry.file_name();
        if name.to_str().is_some_and(|name| {
            RUNTIME_FILES
                .lines()
                .any(|line| line.trim().eq_ignore_ascii_case(name))
        }) {
            copy_entry(&entry.path(), &dest.join(name))?;
        }
    }
    Ok(())
}

fn copy_entry(src: &Path, dest: &Path) -> io::Result<()> {
    let metadata = fs::metadata(src).map_err(|error| file_error("read metadata", src, error))?;
    if metadata.is_dir() {
        fs::create_dir_all(dest).map_err(|error| file_error("create directory", dest, error))?;
        for entry in fs::read_dir(src).map_err(|error| file_error("read directory", src, error))? {
            let entry = entry.map_err(|error| file_error("read directory entry", src, error))?;
            copy_entry(&entry.path(), &dest.join(entry.file_name()))?;
        }
    } else {
        let parent = dest.parent().unwrap();
        fs::create_dir_all(parent)
            .map_err(|error| file_error("create directory", parent, error))?;
        fs::copy(src, dest).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("copy {} to {}: {error}", src.display(), dest.display()),
            )
        })?;
    }
    Ok(())
}

fn stage_pkg(name: &str, ver: &str, temp: &Path, required: &Path) -> io::Result<PathBuf> {
    let extract = cached_dir(temp, &format!("{name}-{ver}"), |partial| {
        let nupkg = partial.join("package.nupkg");
        dl_nupkg(name, ver, &nupkg)?;
        extract_tar(&nupkg, partial, &["--strip-components=1"])?;
        require_file(&partial.join(required))?;
        fs::remove_file(&nupkg).map_err(|error| file_error("remove archive", &nupkg, error))
    })?;
    require_file(&extract.join(required))?;
    Ok(extract)
}

fn cached_dir(
    root: &Path,
    name: &str,
    populate: impl FnOnce(&Path) -> io::Result<()>,
) -> io::Result<PathBuf> {
    let lock_path = root.join(format!("{name}.lock"));
    let _lock = lock_file(&lock_path)?;
    let dest = root.join(name);
    match fs::metadata(&dest) {
        Ok(metadata) if metadata.is_dir() => return Ok(dest),
        Ok(_) => {
            return Err(io::Error::other(format!(
                "not a directory: {}",
                dest.display()
            )));
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(file_error("read cache metadata", &dest, error)),
    }

    let partial = root.join(format!("{name}.partial"));
    if partial
        .try_exists()
        .map_err(|error| file_error("read incomplete cache metadata", &partial, error))?
    {
        fs::remove_dir_all(&partial)
            .map_err(|error| file_error("remove incomplete cache", &partial, error))?;
    }
    fs::create_dir(&partial)
        .map_err(|error| file_error("create staging directory", &partial, error))?;
    populate(&partial)?;
    fs::rename(&partial, &dest).map_err(|error| file_error("publish cache", &dest, error))?;
    Ok(dest)
}

fn lock_file(path: &Path) -> io::Result<fs::File> {
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
        .map_err(|error| file_error("open lock", path, error))?;
    file.lock()
        .map_err(|error| file_error("lock", path, error))?;
    Ok(file)
}

fn require_file(path: &Path) -> io::Result<()> {
    if fs::metadata(path)
        .map_err(|error| file_error("read required file", path, error))?
        .is_file()
    {
        Ok(())
    } else {
        Err(io::Error::other(format!("not a file: {}", path.display())))
    }
}

fn file_error(action: &str, path: &Path, error: io::Error) -> io::Error {
    io::Error::new(
        error.kind(),
        format!("{action} {}: {error}", path.display()),
    )
}

fn dl_nupkg(name: &str, ver: &str, dest: &Path) -> io::Result<()> {
    let url = NUGET_URL.replace("{name}", name).replace("{version}", ver);
    println!("Downloading {name} {ver}");
    run(Command::new(system_tool("curl.exe")?)
        .args([
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            "--output",
        ])
        .arg(dest)
        .arg(url))
}

fn extract_tar(src: &Path, dst: &Path, extra: &[&str]) -> io::Result<()> {
    println!("Extracting {} to {}", src.display(), dst.display());
    run(Command::new(system_tool("tar.exe")?)
        .arg("-xf")
        .arg(src)
        .arg("-C")
        .arg(dst)
        .args(extra))
}

fn system_tool(name: &str) -> io::Result<PathBuf> {
    env::var_os("SystemRoot")
        .map(|root| PathBuf::from(root).join("System32").join(name))
        .ok_or_else(|| io::Error::other("SystemRoot is not set"))
}

fn run(command: &mut Command) -> io::Result<()> {
    let output = command
        .output()
        .map_err(|error| io::Error::new(error.kind(), format!("run {command:?}: {error}")))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "{command:?} failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
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
        _ => "x64",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            loop {
                let path = env::temp_dir().join(format!(
                    "windows-reactor-setup-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self(path),
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                    Err(error) => panic!("{error}"),
                }
            }
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn write(path: &Path, contents: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    #[cfg(windows)]
    fn archive(src: &Path, dest: &Path) {
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        run(Command::new(system_tool("tar.exe").unwrap())
            .args(["--format=zip", "-cf"])
            .arg(dest)
            .arg("-C")
            .arg(src)
            .arg("."))
        .unwrap();
    }

    #[cfg(windows)]
    fn runtime_archive(root: &Path, arch: &str) {
        let payload = root.join(format!("payload-{arch}"));
        write(&payload.join("Microsoft.WindowsAppRuntime.dll"), arch);
        write(&payload.join("Microsoft.ui.xaml.dll"), arch);
        archive(&payload, &root.join(runtime_msix(arch)));
    }

    #[test]
    #[cfg(windows)]
    fn runtime_caches_are_architecture_specific() {
        let temp = TempDir::new();
        runtime_archive(&temp.0, "x64");
        runtime_archive(&temp.0, "x86");
        runtime_archive(&temp.0, "arm64");
        write(
            &temp.0.join(".msix_extract").join("Microsoft.ui.xaml.dll"),
            "legacy",
        );

        let x64 = ensure_msix_extracted(&temp.0, "x64").unwrap();
        let x86 = ensure_msix_extracted(&temp.0, "x86").unwrap();
        let arm64 = ensure_msix_extracted(&temp.0, "arm64").unwrap();
        assert_ne!(x64, x86);
        assert_ne!(x86, arm64);
        assert_ne!(x64, arm64);
        assert_eq!(
            fs::read_to_string(x64.join("Microsoft.ui.xaml.dll")).unwrap(),
            "x64"
        );
        assert_eq!(
            fs::read_to_string(x86.join("Microsoft.ui.xaml.dll")).unwrap(),
            "x86"
        );
        assert_eq!(
            fs::read_to_string(arm64.join("Microsoft.ui.xaml.dll")).unwrap(),
            "arm64"
        );
        fs::remove_file(temp.0.join(runtime_msix("x64"))).unwrap();
        assert_eq!(ensure_msix_extracted(&temp.0, "x64").unwrap(), x64);
    }

    #[test]
    #[cfg(windows)]
    fn failed_extraction_can_be_retried() {
        let temp = TempDir::new();
        write(&temp.0.join(runtime_msix("x64")), "not an archive");
        let error = ensure_msix_extracted(&temp.0, "x64").unwrap_err();
        assert!(error.to_string().contains("tar.exe"));
        assert!(!temp.0.join(".msix_extract-x64").exists());

        runtime_archive(&temp.0, "x64");
        let extract = ensure_msix_extracted(&temp.0, "x64").unwrap();
        assert_eq!(
            fs::read_to_string(extract.join("Microsoft.ui.xaml.dll")).unwrap(),
            "x64"
        );
    }

    #[test]
    #[cfg(windows)]
    fn extraction_requires_runtime_files_before_publication() {
        let temp = TempDir::new();
        let payload = temp.0.join("payload");
        write(&payload.join("unrelated.txt"), "not a runtime");
        archive(&payload, &temp.0.join(runtime_msix("x64")));
        let error = ensure_msix_extracted(&temp.0, "x64").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Microsoft.WindowsAppRuntime.dll")
        );
        assert!(!temp.0.join(".msix_extract-x64").exists());
    }

    #[test]
    fn failed_population_is_not_reused() {
        let temp = TempDir::new();
        let error = cached_dir(&temp.0, "package", |partial| {
            write(&partial.join("incomplete.nupkg"), "partial download");
            Err(io::Error::other("interrupted"))
        })
        .unwrap_err();
        assert_eq!(error.to_string(), "interrupted");
        assert!(!temp.0.join("package").exists());

        let cache = cached_dir(&temp.0, "package", |partial| {
            assert!(!partial.join("incomplete.nupkg").exists());
            assert!(!temp.0.join("package").exists());
            fs::write(partial.join("complete"), "ready")
        })
        .unwrap();
        assert_eq!(fs::read_to_string(cache.join("complete")).unwrap(), "ready");
        assert_eq!(
            cached_dir(&temp.0, "package", |_| panic!("cache was rebuilt")).unwrap(),
            cache
        );
    }

    #[test]
    fn concurrent_population_publishes_once() {
        let temp = TempDir::new();
        let starts = std::sync::Barrier::new(2);
        let calls = AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for _ in 0..2 {
                scope.spawn(|| {
                    starts.wait();
                    let cache = cached_dir(&temp.0, "package", |partial| {
                        calls.fetch_add(1, Ordering::Relaxed);
                        fs::write(partial.join("complete"), "ready")
                    })
                    .unwrap();
                    assert_eq!(fs::read_to_string(cache.join("complete")).unwrap(), "ready");
                });
            }
        });
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    #[cfg(windows)]
    fn concurrent_deployment() {
        use std::os::windows::fs::OpenOptionsExt;
        use std::time::{Duration, Instant};

        const WORKER_ROOT: &str = "WINDOWS_REACTOR_SETUP_TEST_ROOT";
        const FILE_SHARE_READ: u32 = 1;
        if let Some(root) = env::var_os(WORKER_ROOT) {
            let root = PathBuf::from(root);
            write(&root.join(format!("ready-{}", std::process::id())), "");
            let deadline = Instant::now() + Duration::from_secs(30);
            while !root.join("start").is_file() {
                assert!(
                    Instant::now() < deadline,
                    "deployment workers did not start"
                );
                std::thread::sleep(Duration::from_millis(10));
            }
            for _ in 0..4 {
                as_self_contained();
            }
            return;
        }

        struct Worker(std::process::Child);
        impl Drop for Worker {
            fn drop(&mut self) {
                if self.0.try_wait().unwrap().is_none() {
                    if let Err(error) = self.0.kill()
                        && self.0.try_wait().unwrap().is_none()
                    {
                        eprintln!("failed to stop deployment worker: {error}");
                        return;
                    }
                    self.0.wait().unwrap();
                }
            }
        }

        let temp = TempDir::new();
        let cache = temp.0.join("windows-reactor-setup").join("temp").join("v1");
        let runtime = cache.join(format!("{RUNTIME_PKG}-{RUNTIME_VER}"));
        write(&runtime.join(runtime_msix("x64")), "cached archive");
        let payload = "runtime".repeat(256 * 1024);
        for name in ["Microsoft.WindowsAppRuntime.dll", "Microsoft.ui.xaml.dll"] {
            write(&runtime.join(".msix_extract-x64").join(name), &payload);
        }
        write(
            &cache
                .join(format!("{WEBVIEW2_PKG}-{WEBVIEW2_VER}"))
                .join("win-x64")
                .join("native_uap")
                .join(WEBVIEW2_CORE_DLL),
            &payload,
        );
        let dest = temp.0.join("target").join("debug");
        write(&dest.join("Microsoft.ui.xaml.dll"), "previous deployment");
        let destination_lock = lock_file(&dest.join(".windows-reactor-setup.lock")).unwrap();
        // An active writer keeps its output unavailable until it releases the destination lock.
        let busy_file = fs::OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(dest.join("Microsoft.ui.xaml.dll"))
            .unwrap();
        let mut workers = Vec::new();
        for index in 0..8 {
            let out = dest
                .join("build")
                .join(format!("worker-{index}"))
                .join("out");
            fs::create_dir_all(&out).unwrap();
            let log = fs::File::create(temp.0.join(format!("worker-{index}.log"))).unwrap();
            workers.push(Worker(
                Command::new(env::current_exe().unwrap())
                    .args(["--exact", "tests::concurrent_deployment", "--nocapture"])
                    .env(WORKER_ROOT, &temp.0)
                    .env("LOCALAPPDATA", &temp.0)
                    .env("OUT_DIR", out)
                    .env("PROFILE", "debug")
                    .env("CARGO_CFG_TARGET_OS", "windows")
                    .env("CARGO_CFG_TARGET_ARCH", "x86_64")
                    .env("CARGO_CFG_TARGET_ENV", "msvc")
                    .stdout(log.try_clone().unwrap())
                    .stderr(log)
                    .spawn()
                    .unwrap(),
            ));
        }
        let deadline = Instant::now() + Duration::from_secs(30);
        while !workers
            .iter()
            .all(|worker| temp.0.join(format!("ready-{}", worker.0.id())).is_file())
        {
            assert!(
                Instant::now() < deadline,
                "deployment workers were not ready"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
        write(&temp.0.join("start"), "");
        let blocked = Instant::now() + Duration::from_secs(1);
        while Instant::now() < blocked {
            for worker in &mut workers {
                assert!(
                    worker.0.try_wait().unwrap().is_none(),
                    "deployment did not wait for the destination lock"
                );
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        drop(busy_file);
        drop(destination_lock);
        for (index, worker) in workers.iter_mut().enumerate() {
            let status = loop {
                if let Some(status) = worker.0.try_wait().unwrap() {
                    break status;
                }
                assert!(Instant::now() < deadline, "deployment worker timed out");
                std::thread::sleep(Duration::from_millis(10));
            };
            assert!(
                status.success(),
                "worker {index}: {status}\n{}",
                fs::read_to_string(temp.0.join(format!("worker-{index}.log"))).unwrap(),
            );
        }
        for name in [
            "Microsoft.WindowsAppRuntime.dll",
            "Microsoft.ui.xaml.dll",
            WEBVIEW2_CORE_DLL,
        ] {
            assert_eq!(fs::read_to_string(dest.join(name)).unwrap(), payload);
        }
    }

    #[test]
    fn copy_failures_are_reported() {
        let temp = TempDir::new();
        let src = temp.0.join("source");
        let dest = temp.0.join("destination");
        let name = "Microsoft.ui.xaml.dll";
        write(&src.join(name), "runtime");
        fs::create_dir_all(dest.join(name)).unwrap();
        let error = copy_runtime_to(&src, &dest).unwrap_err().to_string();
        assert!(error.contains(&src.join(name).display().to_string()));
        assert!(error.contains(&dest.join(name).display().to_string()));

        assert!(copy_runtime_to(&temp.0.join("missing"), &dest).is_err());
        assert!(copy_entry(&temp.0.join("missing"), &dest.join("file")).is_err());
    }

    #[test]
    fn copy_preserves_selected_directories_and_filters_other_files() {
        let temp = TempDir::new();
        let src = temp.0.join("source");
        let dest = temp.0.join("destination");
        write(&src.join("Microsoft.ui.xaml.dll"), "runtime");
        let resource = Path::new("Microsoft.UI.Xaml")
            .join("nested")
            .join("resource");
        write(&src.join(&resource), "resource");
        write(&src.join("unrelated.txt"), "excluded");
        copy_runtime_to(&src, &dest).unwrap();
        assert_eq!(
            fs::read_to_string(dest.join("Microsoft.ui.xaml.dll")).unwrap(),
            "runtime"
        );
        assert_eq!(fs::read_to_string(dest.join(resource)).unwrap(), "resource");
        assert!(!dest.join("unrelated.txt").exists());
    }

    #[test]
    fn missing_tool_is_reported() {
        let temp = TempDir::new();
        let tool = temp.0.join("missing.exe");
        let error = run(&mut Command::new(&tool)).unwrap_err().to_string();
        assert!(error.contains("missing.exe"));
    }

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
