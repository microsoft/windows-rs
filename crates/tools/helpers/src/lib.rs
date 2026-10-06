use serde::Deserialize;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};

mod clang;
pub use clang::*;
mod nuget;
pub use nuget::nuget_package;
mod sdk;
pub use sdk::*;

/// Prefer Windows-bundled tools over shadowing tools on `PATH`.
fn system_tool(exe: &str) -> std::process::Command {
    let system32 = std::env::var_os("SystemRoot")
        .map(|root| Path::new(&root).join("System32").join(exe))
        .filter(|path| path.is_file());
    match system32 {
        Some(path) => std::process::Command::new(path),
        None => std::process::Command::new(exe),
    }
}

struct TempDir(PathBuf);

impl TempDir {
    fn new(parent: &Path) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};

        static NEXT: AtomicU64 = AtomicU64::new(0);
        std::fs::create_dir_all(parent).unwrap();
        loop {
            let path = parent.join(format!(
                ".restore-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match std::fs::create_dir(&path) {
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
        if let Err(error) = std::fs::remove_dir_all(&self.0) {
            eprintln!(
                "failed to remove staging directory `{}`: {error}",
                self.0.display()
            );
        }
    }
}

/// Target architecture settings shared by metadata generators.
pub struct Arch {
    pub name: String,
    pub triple: String,
    pub bits: i32,
    pub defines: Vec<String>,
}

impl Arch {
    pub fn known(name: &str) -> Option<Self> {
        let (triple, bits) = match name {
            "x64" => ("x86_64-pc-windows-msvc", 2),
            "arm64" => ("aarch64-pc-windows-msvc", 4),
            "x86" => ("i686-pc-windows-msvc", 1),
            _ => return None,
        };
        Some(Self {
            name: name.to_string(),
            triple: triple.to_string(),
            bits,
            defines: Vec::new(),
        })
    }

    pub fn canonical_plus(extra: &[String], build: impl Fn(&str) -> Self) -> Vec<Self> {
        let mut archs = vec![build("x64")];
        for name in extra {
            if name != "x64" {
                archs.push(build(name));
            }
        }
        archs
    }
}

pub fn find_in_dirs(name: &str, dirs: &[String]) -> Option<String> {
    dirs.iter()
        .map(|dir| Path::new(dir).join(name))
        .find(|path| path.is_file())
        .map(|path| path.to_string_lossy().replace('\\', "/"))
}

#[derive(Deserialize)]
pub struct Crate {
    pub package: Package,
    pub lints: Option<Lints>,
    pub path: Option<PathBuf>,
}

impl PartialEq for Crate {
    fn eq(&self, other: &Self) -> bool {
        self.package.name == other.package.name
    }
}

impl Eq for Crate {}

impl Ord for Crate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.package.name.cmp(&other.package.name)
    }
}

impl PartialOrd for Crate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Deserialize)]
pub struct Lints {
    pub workspace: bool,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub publish: Option<bool>,
    #[serde(rename = "rust-version")]
    pub rust_version: Option<String>,
    pub license: Option<String>,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub readme: Option<String>,
    pub categories: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
}

pub fn crates<P: AsRef<Path>>(path: P) -> Vec<Crate> {
    let mut crates = find(path);
    crates.sort();
    crates
}

fn find<P: AsRef<Path>>(path: P) -> Vec<Crate> {
    let mut crates = vec![];

    let path = path.as_ref();
    let files = std::fs::read_dir(path).unwrap_or_else(|error| {
        panic!("cannot read crate directory `{}`: {error}", path.display())
    });
    for file in files {
        let file = file.unwrap();
        if file.file_type().unwrap().is_dir() {
            crates.append(&mut find(file.path()));
        } else if file.file_name() == "Cargo.toml" {
            let text = std::fs::read_to_string(file.path()).expect("Cargo.toml");
            let mut entry: Crate = toml::from_str(&text).expect("toml");
            entry.path = Some(file.path());
            crates.push(entry);
        }
    }

    crates
}

/// SDK/WDK include and library folders use the package version with a zero revision.
pub fn marketing_dir(version: &str) -> String {
    let mut parts = version.split('.');
    let major = parts.next();
    let minor = parts.next();
    let build = parts.next();
    match (major, minor, build) {
        (Some(major), Some(minor), Some(build)) => format!("{major}.{minor}.{build}.0"),
        _ => panic!("`{version}` is not a `major.minor.build[.revision]` version"),
    }
}

pub fn set_thread_ui_language() {
    // Enables testing without pulling in a dependency on the `windows` crate.
    windows_link::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages(flags : u32, language : *const u16, _ : *mut u32) -> i32);
    pub const MUI_LANGUAGE_NAME: u32 = 8u32;

    let language: Vec<_> = "en-US".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        assert_eq!(
            1,
            SetThreadPreferredUILanguages(
                MUI_LANGUAGE_NAME,
                language.as_ptr(),
                std::ptr::null_mut()
            )
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn marketing_dir_zeroes_the_revision() {
        use super::marketing_dir;
        assert_eq!(marketing_dir("10.0.28000.2270"), "10.0.28000.0");
        assert_eq!(marketing_dir("10.0.28000.1839"), "10.0.28000.0");
        // A three-part version (no revision) is still normalized to a `.0` fourth component.
        assert_eq!(marketing_dir("10.0.22621"), "10.0.22621.0");
    }

    #[test]
    #[should_panic(expected = "cannot read crate directory")]
    fn crate_discovery_rejects_non_directories() {
        super::crates(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"));
    }
}
