//! On-demand provisioning for pinned generator dependencies.

use std::path::{Path, PathBuf};
use windows_clang_sys::{clang_getCString, clang_getClangVersion, load};

/// Pinned libclang version; macro capture changes across major versions.
pub use windows_clang_sys::LIBCLANG_VERSION;

/// LLVM repo used to fetch version-matched clang resource headers for non-x64 passes.
const CLANG_RESOURCE_REPO: &str = "https://github.com/llvm/llvm-project";

/// Shared cache for clang resource-header checkouts, keyed by [`LIBCLANG_VERSION`].
const CACHE_ROOT: &str = "target/tool-clang";

/// Ensure libclang is loadable, respecting an existing `LIBCLANG_PATH`.
pub fn ensure_libclang() {
    if std::env::var_os("LIBCLANG_PATH").is_some() {
        return;
    }
    let native = libclang_dir();
    // SAFETY: called before any libclang load or worker thread is spawned.
    unsafe {
        std::env::set_var("LIBCLANG_PATH", &native);
    }
}

/// Resolve or fetch the pinned host-arch `libclang.dll` directory without setting env vars.
pub fn libclang_dir() -> PathBuf {
    let triple = if cfg!(target_arch = "x86_64") {
        "x86_64-pc-windows-msvc"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64-pc-windows-msvc"
    } else {
        panic!(
            "windows-clang provisions native libraries only for x86_64 and aarch64 Windows hosts"
        );
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..");
    let native = root
        .join("target\\tool-clang\\native")
        .join(LIBCLANG_VERSION)
        .join(triple)
        .join("bin");
    if !native.join("libClangSharp.dll").is_file() || !native.join("libclang.dll").is_file() {
        let output = std::process::Command::new("powershell.exe")
            .args(["-NoProfile", "-File"])
            .arg(root.join("crates\\libs\\clang-sys\\native\\build.ps1"))
            .args([
                "-Architecture",
                if cfg!(target_arch = "aarch64") {
                    "Arm64"
                } else {
                    "X64"
                },
            ])
            .output()
            .unwrap();
        eprint!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        assert!(output.status.success(), "windows-clang native build failed");
    }
    assert!(
        native.join("libclang.dll").is_file(),
        "`{}` is missing `libclang.dll`",
        native.display()
    );
    native
}

/// Assert the loaded libclang matches [`LIBCLANG_VERSION`].
pub fn assert_libclang_version() {
    let version = clang_version().unwrap_or_else(|e| {
        panic!(
            "failed to load libclang: {e}\n\
             Point `LIBCLANG_PATH` at a libclang {LIBCLANG_VERSION} build, or let the tool fetch \
             the pinned windows-clang native libraries automatically."
        )
    });
    assert!(
        version_is_pinned(&version, LIBCLANG_VERSION),
        "libclang version mismatch: the tooling is pinned to {LIBCLANG_VERSION} but the loaded \
         libclang reports `{version}`.\nUnset `LIBCLANG_PATH` to use the pinned \
         windows-clang native build, or point it at a matching native library pair."
    );
}

fn clang_version() -> Result<String, String> {
    load().map_err(|error| format!("failed to load libclang: {error}"))?;
    let version = unsafe { clang_getClangVersion() };
    let value = unsafe { std::ffi::CStr::from_ptr(clang_getCString(version)) }
        .to_string_lossy()
        .into_owned();
    unsafe {
        windows_clang_sys::clang_disposeString(version);
    }
    Ok(value)
}

/// True when `reported` contains `pinned` as a whole version token.
fn version_is_pinned(reported: &str, pinned: &str) -> bool {
    reported.match_indices(pinned).any(|(i, _)| {
        let before_ok = reported[..i]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_ascii_digit() && c != '.');
        let after_ok = reported[i + pinned.len()..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_ascii_digit() && c != '.');
        before_ok && after_ok
    })
}

/// Resolve the version-matched clang `-resource-dir`, honoring `CLANG_RESOURCE_DIR`.
pub fn clang_resource_dir() -> String {
    if let Ok(dir) = std::env::var("CLANG_RESOURCE_DIR") {
        return dir.replace('\\', "/");
    }
    let cache = PathBuf::from(CACHE_ROOT)
        .join("clang-resource")
        .join(LIBCLANG_VERSION);
    if !cache.join("include").join("intrin.h").is_file() {
        fetch_clang_resource_headers(&cache);
    }
    cache.to_string_lossy().replace('\\', "/")
}

/// Fetch clang's `lib/Headers` subtree for the pinned LLVM tag into `<cache>/include`.
fn fetch_clang_resource_headers(cache: &Path) {
    std::fs::create_dir_all(cache)
        .unwrap_or_else(|e| panic!("failed to create `{}`: {e}", cache.display()));
    let include = cache.join("include");
    let work = cache.join("_git");
    if work.exists() {
        std::fs::remove_dir_all(&work).ok();
    }
    let tag = format!("llvmorg-{LIBCLANG_VERSION}");

    let status = system_tool("git.exe")
        .args([
            "clone",
            "--filter=blob:none",
            "--no-checkout",
            "--depth",
            "1",
            "--branch",
            &tag,
            CLANG_RESOURCE_REPO,
        ])
        .arg(&work)
        .status()
        .unwrap_or_else(|e| panic!("failed to run `git clone` for clang resource headers: {e}"));
    assert!(
        status.success(),
        "git clone of {CLANG_RESOURCE_REPO} @ {tag} failed"
    );

    for args in [
        &["sparse-checkout", "set", "--no-cone", "clang/lib/Headers"][..],
        &["checkout"][..],
    ] {
        let status = system_tool("git.exe")
            .arg("-C")
            .arg(&work)
            .args(args)
            .status()
            .unwrap_or_else(|e| panic!("failed to run `git {}`: {e}", args.join(" ")));
        assert!(status.success(), "git {} failed", args.join(" "));
    }

    let headers = work.join("clang").join("lib").join("Headers");
    if include.exists() {
        std::fs::remove_dir_all(&include).ok();
    }
    std::fs::rename(&headers, &include).unwrap_or_else(|e| {
        panic!(
            "failed to move `{}` -> `{}`: {e}",
            headers.display(),
            include.display()
        )
    });
    std::fs::remove_dir_all(&work).ok();
    assert!(
        include.join("intrin.h").is_file(),
        "clang resource headers missing `intrin.h` after checkout into `{}`",
        include.display()
    );
}

/// NuGet global-packages folder used by the pinned package restores.
fn nuget_root() -> PathBuf {
    if let Ok(dir) = std::env::var("NUGET_PACKAGES") {
        return PathBuf::from(dir);
    }
    let profile = std::env::var("USERPROFILE")
        .unwrap_or_else(|_| panic!("neither `NUGET_PACKAGES` nor `USERPROFILE` is set"));
    PathBuf::from(profile).join(".nuget").join("packages")
}

/// Resolve or fetch an exact NuGet package, accepting global-cache or flat restore layouts.
pub fn nuget_package(id: &str, version: &str) -> PathBuf {
    let root = nuget_root();
    let global = root.join(id).join(version);
    if global.is_dir() {
        return global;
    }
    let flat = root.join(format!("{id}.{version}"));
    if flat.is_dir() {
        return flat;
    }
    fetch_nuget_package(id, version, &global);
    global
}

/// Download and extract the pinned nupkg into the global-cache layout.
fn fetch_nuget_package(id: &str, version: &str, dest: &Path) {
    std::fs::create_dir_all(dest)
        .unwrap_or_else(|e| panic!("failed to create `{}`: {e}", dest.display()));

    let url = format!("https://www.nuget.org/api/v2/package/{id}/{version}");
    let archive = TempFile::new(&format!("{id}.{version}.nupkg"));
    let status = system_tool("curl.exe")
        .args(["-sSL", &url, "-o"])
        .arg(archive.path())
        .status()
        .unwrap_or_else(|e| panic!("failed to run `curl` to fetch `{id}` {version}: {e}"));
    assert!(status.success(), "curl failed to download {url}");

    let status = system_tool("tar.exe")
        .arg("-xf")
        .arg(archive.path())
        .arg("-C")
        .arg(dest)
        .status()
        .unwrap_or_else(|e| panic!("failed to run `tar` to extract `{id}` {version}: {e}"));
    let extracted_any = std::fs::read_dir(dest).is_ok_and(|mut entries| entries.next().is_some());
    assert!(
        status.success() && extracted_any,
        "tar failed to extract the pinned NuGet package `{id}` {version} into `{}`.\n\
         Restore it manually into the NuGet global cache:\n  \
         nuget install {id} -Version {version} -OutputDirectory \"{}\"",
        dest.display(),
        nuget_root().display()
    );
}

/// Prefer Windows-bundled `curl`/`tar` in `System32` over shadowing tools on `PATH`.
fn system_tool(exe: &str) -> std::process::Command {
    let system32 = std::env::var_os("SystemRoot")
        .map(|r| Path::new(&r).join("System32").join(exe))
        .filter(|p| p.is_file());
    match system32 {
        Some(path) => std::process::Command::new(path),
        None => std::process::Command::new(exe),
    }
}

/// Per-process temporary download path removed on drop.
struct TempFile(PathBuf);

impl TempFile {
    fn new(name: &str) -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        Self(std::env::temp_dir().join(format!("{}-{nanos}-{name}", std::process::id())))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
