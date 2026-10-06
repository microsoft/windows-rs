//! Shared libclang pin and provisioning for the header generators.

use super::*;
use clang_sys::{clang_getCString, clang_getClangVersion, load};

/// Pinned libclang version; macro capture changes across major versions.
pub const LIBCLANG_VERSION: &str = "22.1.8";

/// LLVM repo used to fetch version-matched clang resource headers for every architecture.
const CLANG_RESOURCE_REPO: &str = "https://github.com/llvm/llvm-project";

/// Host-arch `libclang.dll` NuGet packages from dotnet/clangsharp.
const LIBCLANG_PKG_X64: &str = "libclang.runtime.win-x64";
const LIBCLANG_PKG_ARM64: &str = "libclang.runtime.win-arm64";

/// Shared cache for clang resource-header checkouts, keyed by [`LIBCLANG_VERSION`].
const CACHE_ROOT: &str = "target/tool-clang";

/// Provision and version-check libclang, returning the validated `LIBCLANG_PATH`.
///
/// Call before spawning threads. An override can name either a library file or its directory.
pub fn ensure_libclang() -> PathBuf {
    let path = std::env::var_os("LIBCLANG_PATH").map_or_else(libclang_dir, PathBuf::from);
    assert!(
        path.to_str().is_some_and(|path| !path.is_empty()) && path.exists(),
        "LIBCLANG_PATH must name an existing library or directory with a UTF-8 path: `{}`",
        path.display()
    );
    // SAFETY: called before any libclang load or worker thread is spawned.
    unsafe {
        std::env::set_var("LIBCLANG_PATH", &path);
    }
    assert_libclang_version();
    path
}

/// Resolve or fetch the pinned host-arch `libclang.dll` directory without setting env vars.
fn libclang_dir() -> PathBuf {
    let (id, rid) = if cfg!(target_arch = "x86_64") {
        (LIBCLANG_PKG_X64, "win-x64")
    } else if cfg!(target_arch = "aarch64") {
        (LIBCLANG_PKG_ARM64, "win-arm64")
    } else {
        // Only x64/arm64 packages are pinned; `LIBCLANG_PATH` can override other hosts.
        panic!(
            "windows-clang provisions the pinned libclang only for x86_64 and aarch64 Windows \
             hosts; set `LIBCLANG_PATH` to a libclang {LIBCLANG_VERSION} build to run elsewhere."
        );
    };
    let native = nuget_package(id, LIBCLANG_VERSION)
        .join("runtimes")
        .join(rid)
        .join("native");
    assert!(
        native.join("libclang.dll").is_file(),
        "`{}` is missing `libclang.dll`",
        native.display()
    );
    native
}

/// Assert the loaded libclang matches [`LIBCLANG_VERSION`].
fn assert_libclang_version() {
    let version = clang_version().unwrap_or_else(|e| {
        panic!(
            "failed to load libclang: {e}\n\
             Point `LIBCLANG_PATH` at a libclang {LIBCLANG_VERSION} build, or let the tool fetch \
             the pinned `libclang.runtime.win-<arch>` NuGet package automatically."
        )
    });
    assert!(
        version_is_pinned(&version, LIBCLANG_VERSION),
        "libclang version mismatch: the tooling is pinned to {LIBCLANG_VERSION} but the loaded \
         libclang reports `{version}`.\nUnset `LIBCLANG_PATH` to use the pinned \
         `libclang.runtime.win-<arch>` NuGet build, or point it at a matching libclang."
    );
}

fn clang_version() -> Result<String, String> {
    load().map_err(|error| format!("failed to load libclang: {error}"))?;
    let version = unsafe { clang_getClangVersion() };
    let value = unsafe { std::ffi::CStr::from_ptr(clang_getCString(version)) }
        .to_string_lossy()
        .into_owned();
    unsafe {
        clang_sys::clang_disposeString(version);
    }
    Ok(value)
}

/// True when `reported` contains `pinned` as a whole version token.
fn version_is_pinned(reported: &str, pinned: &str) -> bool {
    reported.split_whitespace().any(|version| version == pinned)
}

/// Resolve the version-matched clang `-resource-dir`, honoring `CLANG_RESOURCE_DIR`.
pub fn clang_resource_dir() -> String {
    if let Some(dir) = std::env::var_os("CLANG_RESOURCE_DIR") {
        return resource_dir(Path::new(&dir));
    }
    let cache = PathBuf::from(CACHE_ROOT)
        .join("clang-resource")
        .join(LIBCLANG_VERSION);
    if !cache.join("include").try_exists().unwrap() {
        fetch_clang_resource_headers(&cache);
    }
    resource_dir(&cache)
}

fn resource_dir(path: &Path) -> String {
    assert!(
        path.join("include").join("intrin.h").is_file(),
        "clang resource directory `{}` is missing `include/intrin.h`; \
         provide headers matching libclang {LIBCLANG_VERSION} or remove the invalid cache",
        path.display()
    );
    path.to_str()
        .unwrap_or_else(|| {
            panic!(
                "clang resource directory is not a UTF-8 path: `{}`",
                path.display()
            )
        })
        .replace('\\', "/")
}

/// Fetch clang's `lib/Headers` subtree for the pinned LLVM tag into `<cache>/include`.
fn fetch_clang_resource_headers(cache: &Path) {
    let staging = TempDir::new(cache);
    let include = cache.join("include");
    let work = staging.0.join("git");
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
    assert!(
        headers.join("intrin.h").is_file(),
        "clang resource headers missing `intrin.h` in {tag}"
    );
    if let Err(error) = std::fs::rename(&headers, &include) {
        // A concurrent restore can publish the same pinned headers first.
        assert!(
            include.is_dir(),
            "failed to publish clang resource headers `{}`: {error}",
            include.display()
        );
    }
    resource_dir(cache);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_only_the_exact_release_version() {
        let pin = LIBCLANG_VERSION;
        for reported in [pin.to_string(), format!("clang version {pin} (LLVM build)")] {
            assert!(version_is_pinned(&reported, pin));
        }
        for reported in [
            String::new(),
            format!("clang version 1{pin}"),
            format!("clang version {pin}0"),
            format!("clang version {pin}.1"),
            format!("clang version {pin}git"),
            format!("clang version {pin}-rc1"),
        ] {
            assert!(!version_is_pinned(&reported, pin));
        }
    }

    #[test]
    fn validates_resource_directory_layout() {
        let root = TempDir::new(&std::env::temp_dir());
        assert!(std::panic::catch_unwind(|| resource_dir(&root.0)).is_err());
        let include = root.0.join("include");
        std::fs::create_dir(&include).unwrap();
        assert!(std::panic::catch_unwind(|| resource_dir(&root.0)).is_err());
        std::fs::write(include.join("intrin.h"), "").unwrap();
        assert_eq!(
            resource_dir(&root.0),
            root.0.to_str().unwrap().replace('\\', "/")
        );
        assert!(std::panic::catch_unwind(|| resource_dir(&include)).is_err());
    }
}
