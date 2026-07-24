//! On-demand provisioning of the pinned libclang and NuGet packages the header
//! scrapers depend on, shared by every clang consumer (`tool_win32`, `tool_wdk`, ...)
//! so a fresh checkout regenerates without a manual `nuget restore`.
//!
//! libclang is a clang concern, so the version, package ids, and resource-header
//! component are pinned here beside [`clang`](crate::clang) / [`Clang`](crate::Clang).
//! libclang itself is fetched through the shared NuGet helper (the
//! `libclang.runtime.win-<arch>` package), the same *fetch a pinned package by id +
//! version* utility the scrapers use for the SDK/WDK/WebView2 pins; those version pins
//! stay in each tool since they diverge.

use std::path::{Path, PathBuf};

/// Pinned libclang version the corpora are generated against (the reproducible
/// `libclang.runtime.win-<arch>` NuGet package from the .NET Foundation's `dotnet/clangsharp`
/// project). clang's macro-capture behavior shifts between major versions, so the version is
/// asserted at startup against the loaded libclang to keep the scrape deterministic.
pub const LIBCLANG_VERSION: &str = "22.1.8";

/// LLVM source repository supplying the clang builtin *resource headers* (`intrin.h`, the
/// `__stddef`/`__stdarg`/x86/arm intrinsic headers, ...). The reproducible `libclang` NuGet
/// package ships only `libclang.dll` with no resource headers; on x64 nothing in the closure
/// needs them, but on `aarch64` MSVC's `intrin.h` declares `void __cdecl __prefetch(const void*)`
/// which conflicts with clang's aarch64 compiler builtin of the same name - clang's own
/// `intrin.h` (absent from the package) is what reconciles it. These headers are fetched on
/// demand and cached, then passed as `-resource-dir` for the non-x64 arch passes.
///
/// LLVM stopped publishing the small per-component `clang-<ver>.src.tar.xz` after 21.x (22.x
/// ships only the ~130 MB `llvm-project` monorepo tarball), so the headers are pulled with a
/// blobless, shallow, sparse `git` checkout of just `clang/lib/Headers` at the tag
/// `llvmorg-{LIBCLANG_VERSION}` (~8 MB). The tag is derived from [`LIBCLANG_VERSION`], so a bump
/// is a single-const edit and the DLL and headers can never drift apart.
const CLANG_RESOURCE_REPO: &str = "https://github.com/llvm/llvm-project";

/// Pinned `libclang.dll` NuGet packages (`dotnet/clangsharp`'s `libclang.runtime.win-<arch>`,
/// version-matched to [`LIBCLANG_VERSION`]), one per host arch. These are the reproducible
/// Windows builds the corpora are generated against, fetched through [`nuget_package`] - the same
/// pinned-package path the SDK/WDK/WebView2 dependencies use - so [`ensure_libclang`] can stage
/// the host-arch `libclang.dll` on a fresh checkout with no manual install or `LIBCLANG_PATH`. The
/// package lays the DLL at `runtimes/<rid>/native/libclang.dll`. This replaces the PyPI `libclang`
/// wheel, which is unmaintained (frozen at 18.1.1 since 2024); the .NET Foundation packages track
/// current LLVM.
const LIBCLANG_PKG_X64: &str = "libclang.runtime.win-x64";
const LIBCLANG_PKG_ARM64: &str = "libclang.runtime.win-arm64";

/// Shared, untracked download cache under the workspace `target` directory for the clang
/// resource-header checkout. Keyed by [`LIBCLANG_VERSION`] (not by tool) so `tool_win32`,
/// `tool_wdk`, ... reuse one resource-header extract. (`libclang.dll` itself lives in the shared
/// NuGet global cache, fetched by [`ensure_libclang`] via [`nuget_package`].)
const CACHE_ROOT: &str = "target/windows-clang";

/// Ensures libclang is loadable without manual setup. An explicit `LIBCLANG_PATH` is always
/// respected (offline machines, custom builds); otherwise the pinned host-arch
/// `libclang.runtime.win-<arch>` NuGet package is fetched into the shared NuGet global cache and
/// `LIBCLANG_PATH` is pointed at its `libclang.dll`. Must run before the first
/// libclang load ([`assert_libclang_version`]) - `clang-sys` reads `LIBCLANG_PATH` at load time.
pub fn ensure_libclang() {
    if std::env::var_os("LIBCLANG_PATH").is_some() {
        return;
    }
    let native = libclang_dir();
    // SAFETY: called at the very start of `main`, before any libclang load or worker thread
    // is spawned, so no other thread can be reading the environment concurrently.
    unsafe {
        std::env::set_var("LIBCLANG_PATH", &native);
    }
}

/// Resolves (fetching on demand) the directory holding the pinned host-arch `libclang.dll`,
/// *without* mutating the environment. [`ensure_libclang`] wraps this to set `LIBCLANG_PATH` for
/// the scrapers; CI's `test.yml` instead reads it via `tool_clang path` to export `LIBCLANG_PATH`
/// from a workflow step, keeping the `unsafe` [`set_var`](std::env::set_var) off the multithreaded
/// `cargo test` runner.
pub fn libclang_dir() -> PathBuf {
    let (id, rid) = if cfg!(target_arch = "x86_64") {
        (LIBCLANG_PKG_X64, "win-x64")
    } else if cfg!(target_arch = "aarch64") {
        (LIBCLANG_PKG_ARM64, "win-arm64")
    } else {
        // Only x64/arm64 `libclang.runtime.win-<arch>` packages are pinned; a 32-bit host would
        // be handed a 64-bit `libclang.dll` it cannot load. Fail loudly instead of guessing - the
        // scrapers only ever run host-arch, and `LIBCLANG_PATH` can still override for exotic hosts.
        panic!(
            "windows-clang provisions the pinned libclang only for x86_64 and aarch64 Windows \
             hosts; set `LIBCLANG_PATH` to a libclang {LIBCLANG_VERSION} build to run elsewhere."
        );
    };
    // `nuget_package` fetches + caches on demand in the shared NuGet global cache, and the
    // package lays `libclang.dll` at `runtimes/<rid>/native/`.
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

/// Asserts the loaded libclang matches the pinned [`LIBCLANG_VERSION`]. clang's
/// macro-capture behavior drifts across versions, so an unpinned libclang silently
/// produces different output; failing fast here keeps the scrape reproducible.
pub fn assert_libclang_version() {
    let version = crate::clang_version().unwrap_or_else(|e| {
        panic!(
            "failed to load libclang: {e}\n\
             Point `LIBCLANG_PATH` at a libclang {LIBCLANG_VERSION} build, or let the tool fetch \
             the pinned `libclang.runtime.win-<arch>` NuGet package automatically."
        )
    });
    assert!(
        version_is_pinned(&version, LIBCLANG_VERSION),
        "libclang version mismatch: the corpus is pinned to {LIBCLANG_VERSION} but the loaded \
         libclang reports `{version}`.\nUnset `LIBCLANG_PATH` to use the pinned \
         `libclang.runtime.win-<arch>` NuGet build, or point it at a matching libclang."
    );
}

/// Returns `true` when `reported` (e.g. `"clang version 21.1.8"`) contains `pinned` as a whole
/// version token - bounded by non-`[0-9.]` on both sides - so a prefix like `21.1.8` does **not**
/// spuriously match `21.1.80`.
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

/// Resolves a clang `-resource-dir` whose `include/` holds the [`LIBCLANG_VERSION`] builtin
/// headers, fetching and caching them on first use. Used only for the non-x64 arch passes (x64
/// has no builtin conflict, so its committed output is generated without a resource dir and stays
/// byte-identical). A `CLANG_RESOURCE_DIR` environment override is honored for air-gapped/offline
/// machines that pre-stage the headers; otherwise they are pulled with a blobless, shallow, sparse
/// `git` checkout of the LLVM repo into `target/windows-clang/clang-resource/<ver>`. The
/// cache is keyed by the pinned version, so a populated cache short-circuits.
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

/// Pulls clang's `lib/Headers` subtree for the pinned `llvmorg-<ver>` tag straight into
/// `<cache>/include` via a blobless, shallow, sparse `git` checkout - nothing but that one
/// directory is materialized (~8 MB), and the whole monorepo history is filtered out. Pure
/// tooling glue around `git`; any failure panics with the step that failed so a developer can
/// run it by hand.
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

/// The NuGet global-packages folder: `NUGET_PACKAGES` if set, else
/// `%USERPROFILE%\.nuget\packages`. The pinned SDK/WDK packages are restored here.
fn nuget_root() -> PathBuf {
    if let Ok(dir) = std::env::var("NUGET_PACKAGES") {
        return PathBuf::from(dir);
    }
    let profile = std::env::var("USERPROFILE")
        .unwrap_or_else(|_| panic!("neither `NUGET_PACKAGES` nor `USERPROFILE` is set"));
    PathBuf::from(profile).join(".nuget").join("packages")
}

/// Resolves a restored NuGet package directory for `id` at the exact `version`, accepting either
/// layout so the same pinned output regenerates from a global-cache restore
/// (`<root>/<id>/<version>/`, what `dotnet`/PackageReference and a local cache produce) or a
/// flat restore (`<root>/<Id>.<Version>/`, what `nuget restore -PackagesDirectory` produces). The
/// content under each is identical; callers index into the package-specific subtree they need
/// (e.g. the SDK's `c/Include/...` or WebView2's `build/native/include/...`). When neither is
/// present the pinned nupkg is fetched from nuget.org into the
/// global-cache layout on demand, so a fresh checkout needs no prior `nuget restore`.
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

/// Downloads the pinned nupkg from nuget.org and extracts it into `dest`
/// (`<nuget-root>/<id>/<version>/`). A nupkg is a zip; `tar` (ships with Windows) extracts it,
/// preserving the `c/...` trees [`nuget_package`] reads. Pure tooling glue around `curl` +
/// `tar`; any failure panics with the command that failed and a pointer at the manual
/// `nuget restore` step so a developer can run it by hand.
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

/// Resolves a bundled command-line tool (`curl`/`tar`) to its absolute `System32` path when
/// present, falling back to a bare `PATH` lookup otherwise. Pinning to `System32` matches
/// `windows-reactor-setup` and avoids picking up a shadowing tool on `PATH` - e.g. a
/// Git-bundled `tar` whose `bsdtar` semantics differ from the Windows-bundled `libarchive`
/// build the extraction paths here were written against.
fn system_tool(exe: &str) -> std::process::Command {
    let system32 = std::env::var_os("SystemRoot")
        .map(|r| Path::new(&r).join("System32").join(exe))
        .filter(|p| p.is_file());
    match system32 {
        Some(path) => std::process::Command::new(path),
        None => std::process::Command::new(exe),
    }
}

/// A uniquely-named path in the system temp directory whose file is removed on drop. The
/// per-process name (pid + high-resolution timestamp) keeps concurrent runs from sharing -
/// and clobbering - an in-flight download, and the `Drop` cleanup avoids leaving stale
/// archives behind.
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
