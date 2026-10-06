use super::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

/// Restores an exact NuGet package without modifying NuGet-managed cache entries.
///
/// `NUGET_PACKAGES` overrides the default `%USERPROFILE%\.nuget\packages` root. Completed NuGet
/// global-cache entries are reused directly. Archives in global or flat (`id.version`) layouts
/// can be restored offline. Other packages are downloaded from nuget.org.
///
/// Tool restores live under `.windows-rs` in that root and are published only after extraction
/// succeeds. An unmarked legacy directory without an archive must be downloaded again.
pub fn nuget_package(id: &str, version: &str) -> PathBuf {
    let root = if let Some(dir) = std::env::var_os("NUGET_PACKAGES") {
        PathBuf::from(dir)
    } else {
        let profile = std::env::var_os("USERPROFILE")
            .unwrap_or_else(|| panic!("neither `NUGET_PACKAGES` nor `USERPROFILE` is set"));
        PathBuf::from(profile).join(".nuget").join("packages")
    };
    restore(&root, id, version)
}

fn restore(root: &Path, id: &str, version: &str) -> PathBuf {
    let id = id.to_ascii_lowercase();
    let version = version.to_ascii_lowercase();
    let global = root.join(&id).join(&version);
    if global.join(".nupkg.metadata").is_file() {
        assert_package(&global, &id);
        return global;
    }

    let dest = root.join(".windows-rs").join(&id).join(&version);
    if dest.try_exists().unwrap() {
        assert_package(&dest, &id);
        return dest;
    }

    let staging = TempDir::new(dest.parent().unwrap());
    let flat = root.join(format!("{id}.{version}"));
    let filename = format!("{id}.{version}.nupkg");
    let archive = [global.join(&filename), flat.join(&filename)]
        .into_iter()
        .find(|path| path.is_file())
        .unwrap_or_else(|| {
            for path in [&global, &flat] {
                if path.is_dir() {
                    eprintln!(
                        "Ignoring unmarked NuGet cache `{}` without a package archive; \
                         restoring {id} {version} into the tool cache.",
                        path.display()
                    );
                }
            }
            let archive = staging.0.join("package.nupkg");
            download(
                &format!("https://www.nuget.org/api/v2/package/{id}/{version}"),
                &archive,
            );
            archive
        });
    extract(&archive, &staging.0.join("package"), &dest, &id);
    dest
}

fn download(url: &str, archive: &Path) {
    let status = system_tool("curl.exe")
        .args([
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            url,
            "-o",
        ])
        .arg(archive)
        .status()
        .unwrap_or_else(|error| panic!("failed to download `{url}`: {error}"));
    assert!(
        status.success(),
        "curl failed to download `{url}`: {status}"
    );
}

fn extract(archive: &Path, staging: &Path, dest: &Path, id: &str) {
    fs::create_dir(staging).unwrap();
    let status = system_tool("tar.exe")
        .arg("-xf")
        .arg(archive)
        .arg("-C")
        .arg(staging)
        .status()
        .unwrap_or_else(|error| panic!("failed to extract `{}`: {error}", archive.display()));
    assert!(
        status.success(),
        "tar failed to extract `{}`: {status}",
        archive.display()
    );
    assert_package(staging, id);
    if let Err(error) = fs::rename(staging, dest) {
        // Another tool may have finished restoring the same package while we extracted it.
        if dest.is_dir() {
            assert_package(dest, id);
        } else {
            panic!("failed to publish package `{}`: {error}", dest.display());
        }
    }
}

fn assert_package(path: &Path, id: &str) {
    let nuspec = format!("{id}.nuspec");
    let found = fs::read_dir(path)
        .unwrap_or_else(|error| panic!("cannot read package `{}`: {error}", path.display()))
        .any(|entry| {
            let entry = entry.unwrap();
            entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.eq_ignore_ascii_case(&nuspec))
                && entry.path().is_file()
        });
    assert!(
        found,
        "package `{}` is missing `{nuspec}`; remove the invalid cache entry and restore it again",
        path.display()
    );
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
                Err(error) => panic!(
                    "cannot create staging directory `{}`: {error}",
                    path.display()
                ),
            }
        }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.0) {
            eprintln!(
                "failed to remove staging directory `{}`: {error}",
                self.0.display()
            );
        }
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::panic::catch_unwind;
    use std::time::{Duration, Instant};

    const ID: &str = "test.package";
    const VERSION: &str = "1.2.3";

    fn package_archive(root: &Path, archive: &Path, id: &str) {
        let source = TempDir::new(root);
        fs::write(source.0.join(format!("{id}.nuspec")), "<package />").unwrap();
        fs::write(source.0.join("payload.txt"), "test payload").unwrap();
        fs::create_dir_all(archive.parent().unwrap()).unwrap();
        let status = system_tool("tar.exe")
            .args(["--format", "zip", "-cf"])
            .arg(archive)
            .arg("-C")
            .arg(&source.0)
            .arg(".")
            .status()
            .unwrap();
        assert!(status.success());
    }

    fn tool_cache(root: &Path) -> PathBuf {
        root.join(".windows-rs").join(ID).join(VERSION)
    }

    #[test]
    fn reuses_completed_nuget_cache() {
        let root = TempDir::new(&std::env::temp_dir());
        let global = root.0.join(ID).join(VERSION);
        fs::create_dir_all(&global).unwrap();
        fs::write(global.join("Test.Package.nuspec"), "<package />").unwrap();
        fs::write(global.join(".nupkg.metadata"), "{}").unwrap();
        assert_eq!(restore(&root.0, "Test.Package", VERSION), global);
        assert!(!root.0.join(".windows-rs").exists());
    }

    #[test]
    fn rejects_invalid_completed_cache() {
        let root = TempDir::new(&std::env::temp_dir());
        let global = root.0.join(ID).join(VERSION);
        fs::create_dir_all(&global).unwrap();
        fs::write(global.join(".nupkg.metadata"), "{}").unwrap();
        assert!(catch_unwind(|| restore(&root.0, ID, VERSION)).is_err());
        assert!(!root.0.join(".windows-rs").exists());
    }

    #[test]
    fn restores_cached_archives_offline() {
        for flat in [false, true] {
            let root = TempDir::new(&std::env::temp_dir());
            let source = if flat {
                root.0.join(format!("{ID}.{VERSION}"))
            } else {
                root.0.join(ID).join(VERSION)
            };
            let archive = source.join(format!("{ID}.{VERSION}.nupkg"));
            package_archive(&root.0, &archive, "Test.Package");
            fs::write(source.join("unrelated.txt"), "preserve").unwrap();

            let dest = restore(&root.0, ID, VERSION);
            assert_eq!(dest, tool_cache(&root.0));
            assert_eq!(
                fs::read_to_string(dest.join("payload.txt")).unwrap(),
                "test payload"
            );
            assert_eq!(
                fs::read_to_string(source.join("unrelated.txt")).unwrap(),
                "preserve"
            );
            fs::remove_file(archive).unwrap();
            assert_eq!(restore(&root.0, ID, VERSION), dest);
            assert_eq!(fs::read_dir(dest.parent().unwrap()).unwrap().count(), 1);
        }
    }

    #[test]
    fn failed_extraction_does_not_publish_and_can_be_retried() {
        let root = TempDir::new(&std::env::temp_dir());
        let source = root.0.join(ID).join(VERSION);
        fs::create_dir_all(&source).unwrap();
        let archive = source.join(format!("{ID}.{VERSION}.nupkg"));
        fs::write(&archive, "not a package").unwrap();

        assert!(catch_unwind(|| restore(&root.0, ID, VERSION)).is_err());
        let dest = tool_cache(&root.0);
        assert!(!dest.exists());
        assert_eq!(fs::read_dir(dest.parent().unwrap()).unwrap().count(), 0);

        package_archive(&root.0, &archive, ID);
        assert_eq!(restore(&root.0, ID, VERSION), dest);
    }

    #[test]
    fn rejects_archive_for_another_package() {
        let root = TempDir::new(&std::env::temp_dir());
        let archive = root
            .0
            .join(ID)
            .join(VERSION)
            .join(format!("{ID}.{VERSION}.nupkg"));
        package_archive(&root.0, &archive, "other.package");
        assert!(catch_unwind(|| restore(&root.0, ID, VERSION)).is_err());
        assert!(!tool_cache(&root.0).exists());
    }

    #[test]
    fn concurrent_extractions_publish_one_complete_package() {
        let root = TempDir::new(&std::env::temp_dir());
        let archive = root.0.join("source.nupkg");
        package_archive(&root.0, &archive, ID);
        let dest = tool_cache(&root.0);
        let barrier = std::sync::Barrier::new(4);
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| {
                    let staging = TempDir::new(dest.parent().unwrap());
                    barrier.wait();
                    extract(&archive, &staging.0.join("package"), &dest, ID);
                });
            }
        });
        assert_eq!(
            fs::read_to_string(dest.join("payload.txt")).unwrap(),
            "test payload"
        );
        assert_eq!(fs::read_dir(dest.parent().unwrap()).unwrap().count(), 1);
    }

    #[test]
    fn downloads_successful_responses_and_rejects_http_errors() {
        let root = TempDir::new(&std::env::temp_dir());
        for (status, succeeds) in [("200 OK", true), ("404 Not Found", false)] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            listener.set_nonblocking(true).unwrap();
            let url = format!("http://{}/package", listener.local_addr().unwrap());
            std::thread::scope(|scope| {
                scope.spawn(|| {
                    let deadline = Instant::now() + Duration::from_secs(10);
                    let mut stream = loop {
                        match listener.accept() {
                            Ok((stream, _)) => break stream,
                            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                                assert!(Instant::now() < deadline, "no download request received");
                                std::thread::sleep(Duration::from_millis(10));
                            }
                            Err(error) => panic!("cannot accept download request: {error}"),
                        }
                    };
                    stream
                        .set_read_timeout(Some(Duration::from_secs(10)))
                        .unwrap();
                    let mut request = Vec::new();
                    while !request.ends_with(b"\r\n\r\n") {
                        let mut byte = [0];
                        stream.read_exact(&mut byte).unwrap();
                        request.push(byte[0]);
                    }
                    write!(
                        stream,
                        "HTTP/1.1 {status}\r\nContent-Length: 7\r\nConnection: close\r\n\r\npayload"
                    )
                    .unwrap();
                });
                let archive = root.0.join("download.nupkg");
                let result = catch_unwind(|| download(&url, &archive));
                assert_eq!(result.is_ok(), succeeds);
                if succeeds {
                    assert_eq!(fs::read_to_string(&archive).unwrap(), "payload");
                }
            });
        }
    }
}
