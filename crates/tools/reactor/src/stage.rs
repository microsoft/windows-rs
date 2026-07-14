//! Fetches the reactor's WinUI / Windows App SDK / WebView2 `.winmd` codegen inputs from
//! NuGet by pinned version and stages them under `target/`, so the metadata is never
//! hand-vendored in the repository — exactly how `tool_win32`/`tool_windows`/`tool_wdk`
//! source their SDK metadata. Bumping a version constant here (and regenerating) is the
//! whole update: the matching packages are fetched into the shared NuGet cache on demand
//! and the winmd re-staged, so nothing can go stale or drift out of sync by hand.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use windows_clang::nuget_package;

/// Pinned Windows App SDK release. The three winmd-bearing sub-packages
/// (`WinUI`, `InteractiveExperiences`, `Foundation`) are resolved from *this* metapackage's
/// own nuspec ([`resolve_subpackages`]), so this single constant moves the whole WinUI
/// metadata surface in lockstep. Bumping it is a deliberate, reviewable change: set the
/// version, regenerate, commit.
const WINAPPSDK_VER: &str = "2.2.0";

/// Pinned WebView2 SDK. Supplies `Microsoft.Web.WebView2.Core.winmd`, the WinRT metadata the
/// XAML `WebView2` control's `CoreWebView2` bridge resolves against during codegen. This is a
/// distinct artifact from the COM `WebView2.h` surface `tool_webview` scrapes for
/// `windows-webview`; keep it aligned with the WebView2 version `windows-reactor-setup` stages.
const WEBVIEW2_VER: &str = "1.0.4078.44";

/// The Windows App SDK metapackage. Carries no metadata itself — its nuspec pins the exact
/// sub-package versions the winmd actually ship in.
const METAPACKAGE: &str = "Microsoft.WindowsAppSDK";

/// Sub-packages that carry the winmd, keyed by the nuspec dependency id.
const WINUI_PKG: &str = "Microsoft.WindowsAppSDK.WinUI";
const INTERACTIVE_PKG: &str = "Microsoft.WindowsAppSDK.InteractiveExperiences";
const FOUNDATION_PKG: &str = "Microsoft.WindowsAppSDK.Foundation";
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";

/// `InteractiveExperiences` ships two contract-folder variants of `Microsoft.Foundation`/
/// `Microsoft.Graphics`/`Microsoft.UI` winmd (`10.0.17763.0` and `10.0.18362.0`) that differ
/// only in embedded code-signing blobs, not metadata. Take the higher one, consistently.
const INTERACTIVE_CONTRACT: &str = "10.0.18362.0";

/// The single WebView2 winmd the reactor needs (the package also ships an unrelated
/// `tools/wv2winrt/winrt_winmd.winmd` build artifact that must not be picked up).
const WEBVIEW2_CORE_WINMD: &str = "Microsoft.Web.WebView2.Core.winmd";

/// Returns the staged winmd directory, fetching and populating it on first use. Idempotent
/// per process (`OnceLock`) and cheap after the initial download, since [`nuget_package`]
/// caches packages in the shared NuGet global cache.
pub fn winmd_dir() -> &'static Path {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(stage).as_path()
}

/// Downloads the pinned packages, copies their winmd into `target/reactor/winmd`, and
/// generates `extras.winmd` alongside them, returning the populated directory.
fn stage() -> PathBuf {
    let dir = repo_root().join("target/reactor/winmd");
    // Rebuild from scratch so a version bump never leaves a stale winmd behind.
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir)
        .unwrap_or_else(|e| panic!("failed to create `{}`: {e}", dir.display()));

    let (winui_ver, interactive_ver, foundation_ver) = resolve_subpackages();

    copy_winmd(&nuget_package(WINUI_PKG, &winui_ver).join("metadata"), &dir);
    copy_winmd(
        &nuget_package(INTERACTIVE_PKG, &interactive_ver)
            .join("metadata")
            .join(INTERACTIVE_CONTRACT),
        &dir,
    );
    copy_winmd(
        &nuget_package(FOUNDATION_PKG, &foundation_ver).join("metadata"),
        &dir,
    );
    copy_file(
        &nuget_package(WEBVIEW2_PKG, WEBVIEW2_VER)
            .join("lib")
            .join(WEBVIEW2_CORE_WINMD),
        &dir.join(WEBVIEW2_CORE_WINMD),
    );

    generate_extras(&dir);
    dir
}

/// Reads the metapackage's nuspec and returns the pinned `(WinUI, InteractiveExperiences,
/// Foundation)` sub-package versions.
fn resolve_subpackages() -> (String, String, String) {
    let meta = nuget_package(METAPACKAGE, WINAPPSDK_VER);
    let nuspec_path = std::fs::read_dir(&meta)
        .unwrap_or_else(|e| panic!("cannot read `{}`: {e}", meta.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .find(|p| {
            p.extension()
                .is_some_and(|x| x.eq_ignore_ascii_case("nuspec"))
        })
        .unwrap_or_else(|| panic!("no nuspec in `{METAPACKAGE}` {WINAPPSDK_VER}"));
    let nuspec = std::fs::read_to_string(&nuspec_path)
        .unwrap_or_else(|e| panic!("cannot read `{}`: {e}", nuspec_path.display()));

    let get = |id: &str| {
        dependency_version(&nuspec, id).unwrap_or_else(|| {
            panic!("dependency `{id}` not found in `{METAPACKAGE}` {WINAPPSDK_VER} nuspec")
        })
    };
    (get(WINUI_PKG), get(INTERACTIVE_PKG), get(FOUNDATION_PKG))
}

/// Extracts the `version` of the `<dependency id="..." version="..." />` entry for `id` from a
/// nuspec, stripping any NuGet version-range brackets (e.g. `[2.2.0]` → `2.2.0`).
fn dependency_version(nuspec: &str, id: &str) -> Option<String> {
    nuspec
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("<dependency "))
        .find(|line| attr(line, "id").as_deref() == Some(id))
        .and_then(|line| attr(line, "version"))
        .map(|v| v.trim_matches(['[', ']', '(', ')']).to_string())
}

/// Reads the value of the `name="value"` attribute from an XML element string.
fn attr(element: &str, name: &str) -> Option<String> {
    let key = format!("{name}=\"");
    let start = element.find(&key)? + key.len();
    let end = element[start..].find('"')? + start;
    Some(element[start..end].to_string())
}

/// Generates `extras.winmd` (the reactor's supplemental Win32 definitions) into `dir` from
/// `extras.rdl` and the committed default Win32 metadata.
fn generate_extras(dir: &Path) {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let extras_rdl = manifest.join("src/extras.rdl");
    let win32 = manifest.join("../../libs/bindgen/default/Windows.Win32.winmd");
    let out = dir.join("extras.winmd");
    windows_rdl::Reader::new()
        .input(&extras_rdl.to_string_lossy())
        .input(&win32.to_string_lossy())
        .output(&out.to_string_lossy())
        .write()
        .unwrap();
}

/// Copies every `*.winmd` directly under `src` into `dst`.
fn copy_winmd(src: &Path, dst: &Path) {
    let entries = std::fs::read_dir(src)
        .unwrap_or_else(|e| panic!("cannot read winmd source `{}`: {e}", src.display()));
    let mut copied = 0;
    for path in entries.filter_map(|e| e.ok().map(|e| e.path())) {
        if path
            .extension()
            .is_some_and(|x| x.eq_ignore_ascii_case("winmd"))
        {
            copy_file(&path, &dst.join(path.file_name().unwrap()));
            copied += 1;
        }
    }
    assert!(copied > 0, "no `.winmd` under `{}`", src.display());
}

fn copy_file(src: &Path, dst: &Path) {
    std::fs::copy(src, dst).unwrap_or_else(|e| {
        panic!(
            "failed to copy `{}` -> `{}`: {e}",
            src.display(),
            dst.display()
        )
    });
}

/// The repository root, located relative to this crate's manifest so staging paths resolve
/// the same whether the tool is run from the repo root or a test's crate directory.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
