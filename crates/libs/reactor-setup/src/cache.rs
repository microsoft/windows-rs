use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub(super) fn stage_pkg(name: &str, version: &str, temp: &Path) -> PathBuf {
    let dest = temp.join("packages").join(format!("{name}-{version}"));
    cached_dir(&dest, |staging| {
        let cached_archive = temp.join(format!("{name}.{version}.nupkg"));
        let archive = if cached_archive.is_file() {
            cached_archive
        } else {
            let archive = staging.parent().unwrap().join("package.nupkg");
            let url = format!("https://www.nuget.org/api/v2/package/{name}/{version}");
            download(&url, &archive);
            archive
        };
        extract_tar(&archive, staging);
        assert!(
            staging.join(format!("{name}.nuspec")).is_file(),
            "package `{}` is missing its nuspec",
            archive.display()
        );
    })
}

pub(super) fn ensure_msix_extracted(runtime: &Path, arch: &str) -> PathBuf {
    let name = include_str!("../assets/runtime-package.txt");
    let msix = runtime.join(format!("tools/MSIX/win10-{arch}/{name}.msix"));
    let dest = runtime.join(".msix").join(arch);
    cached_dir(&dest, |staging| {
        extract_tar(&msix, staging);
        assert!(
            staging.join("AppxManifest.xml").is_file(),
            "missing runtime AppxManifest.xml"
        );
        for name in runtime::files(arch) {
            fs::metadata(staging.join(name)).unwrap_or_else(|error| {
                panic!("runtime `{}` is missing `{name}`: {error}", msix.display())
            });
        }
    })
}

fn cached_dir(dest: &Path, populate: impl FnOnce(&Path)) -> PathBuf {
    if dest.try_exists().unwrap() {
        assert!(
            dest.is_dir(),
            "cache entry is not a directory: {}",
            dest.display()
        );
        return dest.to_path_buf();
    }
    let staging = TempDir::new(dest.parent().unwrap());
    let contents = staging.0.join("contents");
    fs::create_dir(&contents).unwrap();
    populate(&contents);
    if let Err(error) = fs::rename(&contents, dest) {
        // Another build may have published the same complete cache entry.
        assert!(
            dest.is_dir(),
            "cannot publish cache `{}`: {error}",
            dest.display()
        );
    }
    dest.to_path_buf()
}

fn system_tool(name: &str) -> Command {
    let root = env::var_os("SystemRoot").expect("SystemRoot not set");
    Command::new(PathBuf::from(root).join("System32").join(name))
}

fn run(command: &mut Command) {
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("cannot run {command:?}: {error}"));
    assert!(
        output.status.success(),
        "{command:?} failed ({}): {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn extract_tar(src: &Path, dest: &Path) {
    run(system_tool("tar.exe")
        .arg("-xf")
        .arg(src)
        .arg("-C")
        .arg(dest));
}

fn download(url: &str, dest: &Path) {
    run(system_tool("curl.exe")
        .args([
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            url,
            "-o",
        ])
        .arg(dest));
}

struct TempDir(PathBuf);

impl TempDir {
    fn new(parent: &Path) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        fs::create_dir_all(parent).unwrap();
        loop {
            let path = parent.join(format!(
                ".restore-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("cannot create `{}`: {error}", path.display()),
            }
        }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.0) {
            eprintln!(
                "cannot remove staging directory `{}`: {error}",
                self.0.display()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_restore_is_not_published() {
        let root = TempDir::new(&env::temp_dir());
        let dest = root.0.join("cache");
        assert!(
            std::panic::catch_unwind(|| cached_dir(&dest, |stage| {
                fs::write(stage.join("partial"), "partial").unwrap();
                panic!("interrupted extraction");
            }))
            .is_err()
        );
        assert!(!dest.exists());
        assert_eq!(fs::read_dir(&root.0).unwrap().count(), 0);
        cached_dir(&dest, |stage| {
            fs::write(stage.join("complete"), "done").unwrap();
        });
        cached_dir(&dest, |_| panic!("completed cache must be reused"));
        assert!(dest.join("complete").is_file());
    }

    #[test]
    fn concurrent_restores_publish_complete_contents() {
        let root = TempDir::new(&env::temp_dir());
        let dest = root.0.join("cache");
        let barrier = std::sync::Barrier::new(2);
        std::thread::scope(|scope| {
            for _ in 0..2 {
                scope.spawn(|| {
                    cached_dir(&dest, |stage| {
                        fs::write(stage.join("complete"), "done").unwrap();
                        barrier.wait();
                    })
                });
            }
        });
        assert_eq!(fs::read_to_string(dest.join("complete")).unwrap(), "done");
        assert_eq!(fs::read_dir(&root.0).unwrap().count(), 1);
    }

    #[test]
    fn copy_failures_are_reported() {
        let root = TempDir::new(&env::temp_dir());
        let missing = root.0.join("missing");
        assert!(std::panic::catch_unwind(|| copy_file(&missing, &root.0, "dest")).is_err());
        assert!(std::panic::catch_unwind(|| copy_dir_contents(&missing, &root.0)).is_err());
        assert!(
            std::panic::catch_unwind(|| copy_runtime_to(&root.0, &root.0.join("out"), "x86"))
                .is_err()
        );
    }

    #[cfg(windows)]
    #[test]
    fn invalid_archive_is_not_cached() {
        let root = TempDir::new(&env::temp_dir());
        let archive = root.0.join("Test.1.0.nupkg");
        fs::write(&archive, "invalid").unwrap();
        assert!(std::panic::catch_unwind(|| stage_pkg("Test", "1.0", &root.0)).is_err());
        assert!(!root.0.join("packages/Test-1.0").exists());
        let source = root.0.join("source");
        fs::create_dir(&source).unwrap();
        fs::write(source.join("Test.nuspec"), "<package />").unwrap();
        fs::create_dir(source.join("tools")).unwrap();
        fs::write(source.join("tools/payload"), "complete").unwrap();
        run(system_tool("tar.exe")
            .args(["--format", "zip", "-cf"])
            .arg(&archive)
            .arg("-C")
            .arg(&source)
            .arg("."));
        let package = stage_pkg("Test", "1.0", &root.0);
        assert_eq!(
            fs::read_to_string(package.join("tools/payload")).unwrap(),
            "complete"
        );
    }

    #[cfg(windows)]
    #[test]
    fn runtime_cache_is_architecture_specific() {
        let root = TempDir::new(&env::temp_dir());
        let package = root.0.join("runtime");
        for arch in ["x64", "arm64", "x86"] {
            let source = root.0.join(arch);
            fs::create_dir(&source).unwrap();
            fs::write(source.join("AppxManifest.xml"), "<Package />").unwrap();
            for name in runtime::files(arch) {
                fs::write(source.join(name), arch).unwrap();
            }
            let msix_dir = package.join(format!("tools/MSIX/win10-{arch}"));
            fs::create_dir_all(&msix_dir).unwrap();
            let name = include_str!("../assets/runtime-package.txt");
            run(system_tool("tar.exe")
                .args(["--format", "zip", "-cf"])
                .arg(msix_dir.join(format!("{name}.msix")))
                .arg("-C")
                .arg(&source)
                .arg("."));
            let extract = ensure_msix_extracted(&package, arch);
            assert_eq!(extract, package.join(".msix").join(arch));
            assert_eq!(
                fs::read_to_string(extract.join("microsoft.ui.xaml.dll")).unwrap(),
                arch
            );
        }
        for arch in ["x64", "arm64", "x86"] {
            let extract = ensure_msix_extracted(&package, arch);
            assert_eq!(
                fs::read_to_string(extract.join("microsoft.ui.xaml.dll")).unwrap(),
                arch
            );
        }
    }

    #[test]
    fn selected_contents_are_copied_recursively() {
        let root = TempDir::new(&env::temp_dir());
        let source = root.0.join("source");
        let dest = root.0.join("dest");
        fs::create_dir(&source).unwrap();
        for name in runtime::files("x86") {
            fs::write(source.join(name), name).unwrap();
        }
        fs::remove_file(source.join("en-us")).unwrap();
        fs::create_dir(source.join("en-us")).unwrap();
        fs::write(source.join("en-us/resource.pri"), "resource").unwrap();
        fs::write(source.join("unselected.dll"), "excluded").unwrap();
        copy_runtime_to(&source, &dest, "x86");
        assert_eq!(
            fs::read_to_string(dest.join("en-us/resource.pri")).unwrap(),
            "resource"
        );
        assert!(!dest.join("unselected.dll").exists());
        assert_eq!(
            fs::read_dir(dest).unwrap().count(),
            runtime::files("x86").count()
        );
    }

    #[cfg(windows)]
    #[test]
    fn http_errors_fail_the_restore() {
        use std::io::{Read, Write};
        use std::net::TcpListener;
        use std::time::{Duration, Instant};

        let root = TempDir::new(&env::temp_dir());
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/missing.nupkg", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            let start = Instant::now();
            let mut socket = loop {
                match listener.accept() {
                    Ok((socket, _)) => break socket,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(
                            start.elapsed() < Duration::from_secs(10),
                            "HTTP request timed out"
                        );
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("{error}"),
                }
            };
            socket.set_nonblocking(false).unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(10)))
                .unwrap();
            let mut request = Vec::new();
            while !request.ends_with(b"\r\n\r\n") {
                let mut byte = [0];
                socket.read_exact(&mut byte).unwrap();
                request.push(byte[0]);
            }
            socket
                .write_all(
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .unwrap();
        });
        let dest = root.0.join("cache");
        let result = std::panic::catch_unwind(|| {
            cached_dir(&dest, |staging| {
                download(&url, &staging.join("package.nupkg"));
            })
        });
        server.join().unwrap();
        assert!(result.is_err());
        assert!(!dest.exists());
        assert_eq!(fs::read_dir(&root.0).unwrap().count(), 0);
    }
}
