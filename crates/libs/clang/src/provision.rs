//! On-demand provisioning of the pinned libclang and NuGet packages the header
//! scrapers depend on, shared by every clang consumer (`tool_win32`, `tool_wdk`, …)
//! so a fresh checkout regenerates without a manual `pip install` / `nuget restore`.
//!
//! libclang is a clang concern, so the version, wheel URLs, and resource-header
//! component are pinned here beside [`clang`](crate::clang) / [`Clang`](crate::Clang)
//! and the download cache is shared across tools (they all pin the same
//! [`LIBCLANG_VERSION`]). The NuGet helper is a generic *fetch a pinned package by
//! id + version* utility both scrapers need; the SDK/WDK version pins stay in each
//! tool since they diverge.

use std::path::{Path, PathBuf};

/// Pinned libclang version the corpora are generated against (the reproducible PyPI
/// `libclang` wheel; CI installs the same with `pip install libclang==<this>`). clang's
/// macro-capture behavior shifts between major versions, so the version is asserted at
/// startup against the loaded libclang to keep the scrape deterministic.
pub const LIBCLANG_VERSION: &str = "18.1.1";

/// Pinned LLVM release component that ships the clang builtin *resource headers*
/// (`intrin.h`, the `__stddef`/`__stdarg`/x86/arm intrinsic headers, …), version-matched
/// to [`LIBCLANG_VERSION`]. The reproducible `libclang` wheel ships only `libclang.dll`,
/// with no resource headers; on x64 nothing in the closure needs them, but on `aarch64`
/// MSVC's `intrin.h` declares `void __cdecl __prefetch(const void*)` which conflicts with
/// clang's aarch64 compiler builtin of the same name — clang's own `intrin.h` (absent from
/// the wheel) is what reconciles it. These headers are fetched on demand and cached, then
/// passed as `-resource-dir` for the non-x64 arch passes. The component tarball (~22 MB) is
/// pinned to the exact `llvmorg-<ver>` tag so the multi-arch scrape stays deterministic.
const CLANG_RESOURCE_URL: &str =
    "https://github.com/llvm/llvm-project/releases/download/llvmorg-18.1.1/clang-18.1.1.src.tar.xz";

/// Pinned `libclang` wheel URLs (PyPI `files.pythonhosted.org`, version-matched to
/// [`LIBCLANG_VERSION`]). These are the reproducible Windows wheels the corpora are generated
/// against — the same builds `pip install libclang=={LIBCLANG_VERSION}` resolves — pinned by
/// immutable content URL so [`ensure_libclang`] can fetch the host-arch `libclang.dll` on a fresh
/// checkout with no manual `pip install` / `LIBCLANG_PATH`. A wheel is a zip laying the DLL at
/// `clang/native/libclang.dll`; `tar` (ships with Windows) extracts it.
const LIBCLANG_WHEEL_URL_X64: &str = "https://files.pythonhosted.org/packages/0b/2d/3f480b1e1d31eb3d6de5e3ef641954e5c67430d5ac93b7fa7e07589576c7/libclang-18.1.1-py2.py3-none-win_amd64.whl";
const LIBCLANG_WHEEL_URL_ARM64: &str = "https://files.pythonhosted.org/packages/71/cf/e01dc4cc79779cd82d77888a88ae2fa424d93b445ad4f6c02bfc18335b70/libclang-18.1.1-py2.py3-none-win_arm64.whl";

/// Shared, untracked download cache under the workspace `target` directory. All clang
/// consumers pin the same [`LIBCLANG_VERSION`], so keying the cache by version (not by tool)
/// lets `tool_win32`, `tool_wdk`, … reuse one libclang wheel and one resource-header extract.
const CACHE_ROOT: &str = "target/windows-clang";

/// Ensures libclang is loadable without manual setup. An explicit `LIBCLANG_PATH` is always
/// respected (offline machines, custom builds); otherwise the pinned host-arch `libclang`
/// wheel is downloaded and cached under `target/windows-clang/libclang/<ver>`, and
/// `LIBCLANG_PATH` is pointed at the extracted `libclang.dll`. Must run before the first
/// libclang load ([`assert_libclang_version`]) — `clang-sys` reads `LIBCLANG_PATH` at load time.
pub fn ensure_libclang() {
    if std::env::var_os("LIBCLANG_PATH").is_some() {
        return;
    }
    let cache = PathBuf::from(CACHE_ROOT)
        .join("libclang")
        .join(LIBCLANG_VERSION);
    let native = libclang_native_dir(&cache);
    if !native.join("libclang.dll").is_file() {
        fetch_libclang(&cache);
    }
    // SAFETY: called at the very start of `main`, before any libclang load or worker thread
    // is spawned, so no other thread can be reading the environment concurrently.
    unsafe {
        std::env::set_var("LIBCLANG_PATH", &native);
    }
}

/// The directory holding `libclang.dll` inside an extracted wheel: the wheel lays its data
/// payload under `libclang-<ver>.data/platlib/clang/native/`. Kept deterministic (pinned by
/// version) so [`ensure_libclang`] can both probe the cache and set `LIBCLANG_PATH` without a
/// filesystem search.
fn libclang_native_dir(cache: &Path) -> PathBuf {
    cache
        .join(format!("libclang-{LIBCLANG_VERSION}.data"))
        .join("platlib")
        .join("clang")
        .join("native")
}

/// Downloads the pinned host-arch [`libclang`](LIBCLANG_WHEEL_URL_X64) wheel and extracts its
/// `native` payload (`libclang.dll` and the sibling files clang-sys expects next to it) into
/// `<cache>`. Pure tooling glue around `curl` + `tar` (both ship with Windows); any failure
/// panics with the command that failed so a developer can run it by hand.
fn fetch_libclang(cache: &Path) {
    std::fs::create_dir_all(cache)
        .unwrap_or_else(|e| panic!("failed to create `{}`: {e}", cache.display()));

    let url = if cfg!(target_arch = "aarch64") {
        LIBCLANG_WHEEL_URL_ARM64
    } else {
        LIBCLANG_WHEEL_URL_X64
    };
    let archive = TempFile::new(&format!("libclang-{LIBCLANG_VERSION}.whl"));
    let status = system_tool("curl.exe")
        .args(["-sSL", url, "-o"])
        .arg(archive.path())
        .status()
        .unwrap_or_else(|e| panic!("failed to run `curl` to fetch the libclang wheel: {e}"));
    assert!(status.success(), "curl failed to download {url}");

    // A wheel is a zip; extract just the `native` payload. `--strip-components` is unreliable
    // for a single deep member under Windows `bsdtar`, so extract the member at its natural
    // (version-pinned) path and read the DLL from `libclang_native_dir`.
    let member = format!("libclang-{LIBCLANG_VERSION}.data/platlib/clang/native");
    let status = system_tool("tar.exe")
        .arg("-xf")
        .arg(archive.path())
        .arg("-C")
        .arg(cache)
        .arg(&member)
        .status()
        .unwrap_or_else(|e| panic!("failed to run `tar` to extract the libclang wheel: {e}"));
    assert!(
        status.success() && libclang_native_dir(cache).join("libclang.dll").is_file(),
        "tar failed to extract `{member}/libclang.dll` into `{}`",
        cache.display()
    );
}

/// Asserts the loaded libclang matches the pinned [`LIBCLANG_VERSION`]. clang's
/// macro-capture behavior drifts across versions, so an unpinned libclang silently
/// produces a different corpus; failing fast here keeps the scrape reproducible.
pub fn assert_libclang_version() {
    let version = crate::clang_version().unwrap_or_else(|e| {
        panic!(
            "failed to load libclang: {e}\n\
             Install the pinned libclang and point `LIBCLANG_PATH` at it:\n  \
             pip install libclang=={LIBCLANG_VERSION}"
        )
    });
    assert!(
        version_is_pinned(&version, LIBCLANG_VERSION),
        "libclang version mismatch: the corpus is pinned to {LIBCLANG_VERSION} but the loaded \
         libclang reports `{version}`.\nPoint `LIBCLANG_PATH` at the pinned build:\n  \
         pip install libclang=={LIBCLANG_VERSION}"
    );
}

/// Returns `true` when `reported` (e.g. `"clang version 18.1.1"`) contains `pinned` as a whole
/// version token — bounded by non-`[0-9.]` on both sides — so a prefix like `18.1.1` does **not**
/// spuriously match `18.1.10`.
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

/// Resolves a clang `-resource-dir` whose `include/` holds the version-matched
/// [`LIBCLANG_VERSION`] builtin headers, fetching and caching them on first use. Used only
/// for the non-x64 arch passes (x64 has no builtin conflict, so its committed corpus is
/// generated without a resource dir and stays byte-identical). A `CLANG_RESOURCE_DIR`
/// environment override is honored for air-gapped/offline machines that pre-stage the
/// headers; otherwise the pinned `CLANG_RESOURCE_URL` component is downloaded with `curl`
/// and extracted with `tar` into `target/windows-clang/clang-resource/<ver>` (both tools ship
/// with Windows). The cache is keyed by the pinned version, so a populated cache short-circuits.
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

/// Downloads the pinned clang resource-header component and extracts its `lib/Headers`
/// subtree straight into `<cache>/include`. Pure tooling glue around `curl` + `tar`; any
/// failure panics with the command that failed so a developer can run it by hand.
fn fetch_clang_resource_headers(cache: &Path) {
    let include = cache.join("include");
    std::fs::create_dir_all(&include)
        .unwrap_or_else(|e| panic!("failed to create `{}`: {e}", include.display()));

    let archive = TempFile::new(&format!("clang-{LIBCLANG_VERSION}.src.tar.xz"));
    let status = system_tool("curl.exe")
        .args(["-sSL", CLANG_RESOURCE_URL, "-o"])
        .arg(archive.path())
        .status()
        .unwrap_or_else(|e| panic!("failed to run `curl` to fetch clang resource headers: {e}"));
    assert!(
        status.success(),
        "curl failed to download {CLANG_RESOURCE_URL}"
    );

    // The component lays its builtin headers under `clang-<ver>.src/lib/Headers`; strip
    // those three leading path components so they land directly in `<cache>/include`.
    let subdir = format!("clang-{LIBCLANG_VERSION}.src/lib/Headers");
    let status = system_tool("tar.exe")
        .arg("-xf")
        .arg(archive.path())
        .arg("-C")
        .arg(&include)
        .arg("--strip-components=3")
        .arg(&subdir)
        .status()
        .unwrap_or_else(|e| panic!("failed to run `tar` to extract clang resource headers: {e}"));
    assert!(
        status.success() && include.join("intrin.h").is_file(),
        "tar failed to extract `{subdir}` into `{}`",
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
/// layout so the same pinned corpus regenerates from a global-cache restore
/// (`<root>/<id>/<version>/`, what `dotnet`/PackageReference and a local cache produce) or a
/// flat restore (`<root>/<Id>.<Version>/`, what `nuget restore -PackagesDirectory` produces). The
/// extracted content under each is identical (the SDK header packages carry a `c/Include/...`
/// tree, the WinUI / WebView2 metadata packages a `metadata/` or `lib/` tree). When neither is
/// present the pinned nupkg is fetched from nuget.org into the global-cache layout on demand, so
/// a fresh checkout needs no prior `nuget restore`.
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
/// preserving the package tree [`nuget_package`] reads. Pure tooling glue around `curl` +
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
    assert!(
        status.success() && dest.join("[Content_Types].xml").is_file(),
        "tar failed to extract the pinned NuGet package `{id}` {version} into `{}`.\n\
         Restore it manually into the NuGet global cache:\n  \
         nuget install {id} -Version {version} -OutputDirectory \"{}\"",
        dest.display(),
        nuget_root().display()
    );
}

/// Resolves a bundled command-line tool (`curl`/`tar`) to its absolute `System32` path when
/// present, falling back to a bare `PATH` lookup otherwise. Pinning to `System32` matches
/// `windows-reactor-setup` and avoids picking up a shadowing tool on `PATH` — e.g. a
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
/// per-process name (pid + high-resolution timestamp) keeps concurrent runs from sharing —
/// and clobbering — an in-flight download, and the `Drop` cleanup avoids leaving stale
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
