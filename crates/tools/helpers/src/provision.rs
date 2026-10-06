//! On-demand provisioning for pinned generator dependencies.

use super::*;
use clang_sys::{clang_getCString, clang_getClangVersion, load};
use std::path::PathBuf;

/// Pinned libclang version; macro capture changes across major versions.
pub const LIBCLANG_VERSION: &str = "22.1.8";

/// LLVM repo used to fetch version-matched clang resource headers for non-x64 passes.
const CLANG_RESOURCE_REPO: &str = "https://github.com/llvm/llvm-project";

/// Host-arch `libclang.dll` NuGet packages from dotnet/clangsharp.
const LIBCLANG_PKG_X64: &str = "libclang.runtime.win-x64";
const LIBCLANG_PKG_ARM64: &str = "libclang.runtime.win-arm64";

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
pub fn assert_libclang_version() {
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
