# Dependencies

This page tracks every external SDK, header set, metadata file, and runtime the
build and tooling depend on: what version is used, where that version is set, how
the artifact is obtained, and how to update it. It covers *build/tooling and
runtime* dependencies (the Windows SDK, WebView2, the Windows App SDK, libclang,
…) — not the crates.io Rust dependencies in `Cargo.toml`.

Dependencies come in two shapes:

- **Downloaded on demand, pinned by a version constant.** A version string in a
  tool's source drives an on-demand download that is cached locally, so a fresh
  checkout regenerates with no manual restore. Updating is a one-line version bump.
- **Vendored (committed to the repo).** The artifact itself is checked in. Updating
  means replacing the committed files and regenerating whatever derives from them.

## At a glance

| Dependency | Version | Set / vendored in | Obtained by |
| --- | --- | --- | --- |
| libclang | `18.1.1` | `LIBCLANG_VERSION` — `crates/libs/clang/src/provision.rs` | download (PyPI wheel) |
| clang resource headers (LLVM) | `18.1.1` | `CLANG_RESOURCE_URL` — `crates/libs/clang/src/provision.rs` | download (llvm-project release) |
| Windows SDK (C/C++ headers) | `10.0.28000.2270` | `SDK_VERSION` — `crates/tools/win32/src/main.rs`, `crates/tools/wdk/src/main.rs` | download (NuGet) |
| Windows WDK (kernel headers) | `10.0.28000.1839` | `WDK_VERSION` — `crates/tools/wdk/src/main.rs` | download (NuGet) |
| Windows SDK Contracts (WinRT) | `10.0.28000.2270` | `CONTRACTS_VERSION` — `crates/tools/windows/src/main.rs` | download (NuGet) |
| WebView2 SDK (C/C++ headers) | `1.0.4078.44` | vendored: `crates/tools/webview/WebView2*.h` | committed |
| WebView2 Core (WinRT metadata) | `1.0.4078.44` | `WEBVIEW2_VER` — `crates/tools/reactor/src/stage.rs` | download (NuGet) |
| WinUI / Windows App SDK metadata | `2.2.0` | `WINAPPSDK_VER` — `crates/tools/reactor/src/stage.rs` | download (NuGet) |
| Windows App SDK Runtime | `2.2.0` | `RUNTIME_VER` — `crates/libs/reactor-setup/src/lib.rs` | download (NuGet) + committed bootstrap DLLs |
| WebView2 runtime projection (`Core.dll`) | `1.0.4078.44` | `WEBVIEW2_VER` — `crates/libs/reactor-setup/src/lib.rs` | download (NuGet) |
| WebView2 Evergreen runtime (CI only) | Evergreen | `.github/workflows/webview.yml` | download (installer) |

> Known inconsistencies to reconcile: WebView2 is pinned by a `WEBVIEW2_VER`
> constant in three places (the `reactor` and `reactor-setup` copies, plus the
> vendored `tool_webview` headers, which carry the version only in a comment) that
> must be kept in step; and the `reactor-setup` `app.manifest`/`resources.pri`
> are generated assets with no committed generator (see below). The two NuGet
> download helpers (`windows_clang::nuget_package` and `reactor-setup`'s
> `stage_pkg`) are kept separate on purpose — see [Download
> mechanisms](#download-mechanisms).

## Toolchain: libclang

The header scrapers (`tool_win32`, `tool_wdk`, `tool_webview`) parse C/C++ with
libclang. The version is pinned because clang's macro-capture behavior drifts
across major versions, which would silently change the generated corpus; it is
asserted at startup against the loaded libclang.

- **Version:** `LIBCLANG_VERSION = "18.1.1"` in `crates/libs/clang/src/provision.rs`.
- **How it's obtained:** if `LIBCLANG_PATH` is unset, `ensure_libclang` downloads
  the pinned host-arch `libclang` wheel from PyPI and caches it under
  `target/windows-clang/`. Non-x64 passes also fetch the version-matched LLVM
  resource headers (`CLANG_RESOURCE_URL`). `LIBCLANG_PATH` and `CLANG_RESOURCE_DIR`
  override for offline machines.
- **CI:** installs LLVM `18` via `KyleMayes/install-llvm-action@v2`
  (`clippy.yml`, `gen.yml`).
- **To update:** bump `LIBCLANG_VERSION`, the two wheel URLs, and `CLANG_RESOURCE_URL`
  in `provision.rs`, and the `version:` in the two workflows; then regenerate all
  corpora and confirm no diff.

## Windows SDK, WDK, and WinRT contracts

These feed the in-house metadata generators. Each produces a committed `.winmd`
under `crates/libs/bindgen/default/` (see the provenance table there), which
`windows-bindgen` consumes as `--in default`.

| Package | Version constant | Location | Produces |
| --- | --- | --- | --- |
| `Microsoft.Windows.SDK.CPP[.<arch>]` | `SDK_VERSION` | `crates/tools/win32/src/main.rs` | `Windows.Win32.winmd` (via `tool_win32`) |
| `Microsoft.Windows.WDK.x64` | `WDK_VERSION` | `crates/tools/wdk/src/main.rs` | `Windows.Wdk.winmd` (via `tool_wdk`) |
| `Microsoft.Windows.SDK.Contracts` | `CONTRACTS_VERSION` | `crates/tools/windows/src/main.rs` | `Windows.winmd` (via `tool_windows`) |

- **How they're obtained:** `windows_clang::nuget_package(id, version)` restores the
  package from the NuGet global cache, or downloads it from nuget.org on first use.
  `NUGET_PACKAGES` overrides the cache location.
- **To update:** bump the version constant, run the corresponding
  `cargo run -p tool_<win32|wdk|windows>`, and commit the regenerated `.rdl`
  snapshot(s) and `.winmd`. The `gen` CI job re-runs each tool and fails on any diff.

`CONTRACTS_VERSION` is intentionally aligned to the same `10.0.28000` SDK build as
`SDK_VERSION` rather than the latest stable contracts package.

## WebView2

WebView2 ships only C/C++ SDK headers (no `.rdl`/`.winmd`), so the `windows-webview`
bindings are scraped from vendored headers. The WinRT `Core` metadata and the
runtime projection DLL are separate artifacts.

| Artifact | Where | Used by |
| --- | --- | --- |
| `WebView2.h`, `WebView2Interop.h`, `WebView2EnvironmentOptions.h` | `crates/tools/webview/` (committed) | `tool_webview` → `crates/libs/webview/src/bindings.rs` |
| `Microsoft.Web.WebView2.Core.winmd` | fetched by `tool_reactor` → staged in `target/reactor/winmd/` (pinned by `WEBVIEW2_VER` in `crates/tools/reactor/src/stage.rs`) | `tool_webview` (interop bridge), `tool_reactor` |
| `Microsoft.Web.WebView2` runtime (`Core.dll`), `WEBVIEW2_VER = 1.0.4078.44` | `crates/libs/reactor-setup/src/lib.rs` | self-contained apps (deploys `Core.dll`) |
| Evergreen runtime | `.github/workflows/webview.yml` | CI test host (COM `webview2loader.dll`) |

- **To update the headers:** drop the new `WebView2*.h` from the
  `Microsoft.Web.WebView2` NuGet package (`build/native/include/`) into
  `crates/tools/webview/`, run `cargo run -p tool_webview`, and commit the
  regenerated `bindings.rs`.
- **To update the runtime projection:** bump `WEBVIEW2_VER` in `reactor-setup`.
- Keep the WebView2 versions in step: the vendored headers (`crates/tools/webview/`),
  `Core.winmd` (`WEBVIEW2_VER` in `crates/tools/reactor/src/stage.rs`), and the
  runtime projection (`WEBVIEW2_VER` in `reactor-setup`).

For the full WebView2 build pipeline and the COM↔WinRT interop bridge, see
[windows-webview](crates/windows-webview.md).

## WinUI / Windows App SDK

The `windows-reactor` bindings are generated from WinUI / Windows App SDK `.winmd`
metadata that `tool_reactor` **downloads from NuGet** (pinned by version constant,
staged under `target/`, never vendored); the `windows-reactor-setup` crate stages the
matching runtime so reactor apps can run.

The reactor's two Windows App SDK dependencies are **independent** and updated by
separate steps:

- The `.winmd` **metadata** is a *codegen input* fetched by `tool_reactor` via
  `windows_clang::nuget_package`, exactly like the header scrapers fetch the SDK.
  A single `WINAPPSDK_VER` constant (`crates/tools/reactor/src/stage.rs`) pins the
  `Microsoft.WindowsAppSDK` metapackage; the tool reads that metapackage's nuspec to
  resolve the exact winmd-bearing sub-package versions, downloads them, and stages the
  winmd into `target/reactor/winmd`. Nothing is hand-copied, and `reactor-setup`'s
  runtime acquisition (`RUNTIME_VER`, below) never touches the winmd.
- The **runtime** (`RUNTIME_VER`) is downloaded by `reactor-setup` at the
  *consuming app's* build time to stage runtime binaries next to the executable
  (see [Download mechanisms](#download-mechanisms)). It ships no metadata.

Keep them on the same Windows App SDK version.

| Artifact | Where | Used by |
| --- | --- | --- |
| WinUI / Windows App SDK `.winmd` (~26) + `Microsoft.Web.WebView2.Core.winmd` | fetched by `tool_reactor` → staged in `target/reactor/winmd/` | `tool_reactor` → `crates/libs/reactor/src/bindings.rs` |
| `Microsoft.WindowsAppSDK.Runtime`, `RUNTIME_VER = 2.2.0` | `crates/libs/reactor-setup/src/lib.rs` | app runtime deploy (MSIX extract) |
| `Microsoft.WindowsAppRuntime.Bootstrap.dll` (x86/x64/arm64) | `crates/libs/reactor-setup/bootstrap/` (committed) | framework-dependent apps |
| `resources.pri`, `app.manifest`, `runtime.txt` | `crates/libs/reactor-setup/assets/` (committed) | runtime staging |

- **To update the metadata:** bump `WINAPPSDK_VER` (and, when the WebView2 SDK
  moves, `WEBVIEW2_VER`) in `crates/tools/reactor/src/stage.rs`, then run
  `cargo run -p tool_reactor` and `cargo run -p tool_bindings` and commit the
  regenerated bindings. The tool resolves and fetches everything automatically:
  - `Microsoft.WindowsAppSDK.WinUI` → `Microsoft.UI.Xaml.winmd`,
    `Microsoft.UI.Text.winmd` (from `metadata/`).
  - `Microsoft.WindowsAppSDK.InteractiveExperiences` → `Microsoft.UI.winmd`,
    `Microsoft.Foundation.winmd`, `Microsoft.Graphics.winmd` (the tool picks the
    `metadata/10.0.18362.0/` contract variant; the `10.0.17763.0` variant differs
    only in code-signing blobs, not metadata).
  - `Microsoft.WindowsAppSDK.Foundation` → `Microsoft.Security.Authentication.OAuth.winmd`
    and the `Microsoft.Windows.*.winmd` set (from `metadata/`).
  - `Microsoft.Web.WebView2` → `Microsoft.Web.WebView2.Core.winmd` (from `lib/`), the
    WinRT metadata the XAML `WebView2` control's `CoreWebView2` bridge resolves against
    — distinct from the COM `WebView2.h` surface `tool_webview` scrapes.

  (`extras.winmd` is generated by `tool_reactor` from `Windows.Win32.winmd`, not
  fetched from any package.)
- **To update the runtime:** bump `RUNTIME_VER`, refresh the committed bootstrap
  DLLs from the target Windows App SDK build's
  `Microsoft.WindowsAppSDK.Foundation` package
  (`runtimes/win-<arch>/native/Microsoft.WindowsAppRuntime.Bootstrap.dll`).
- `assets/app.manifest` and `assets/resources.pri` are **generated activation
  assets with no committed generator** — `app.manifest` is a transform of the
  Windows App SDK `runtimes-framework/package.appxfragment` files into the SxS
  fusion format (its source versions are recorded in its header comment). They
  map WinRT class names to runtime DLLs and are forward-compatible, so they are
  refreshed only when the curated reactor control set needs classes that a newer
  Windows App SDK moved or added — not on every version bump. Automating their
  regeneration is tracked as future work in
  [windows-reactor](crates/windows-reactor.md).

See [windows-reactor](crates/windows-reactor.md) and
[windows-reactor-setup](crates/windows-reactor-setup.md).

## Download mechanisms

There are two independent NuGet download paths:

- **`windows_clang::nuget_package`** (`crates/libs/clang/src/provision.rs`) — used
  by the metadata tools (`tool_win32`, `tool_wdk`, `tool_windows`, `tool_webview`,
  and `tool_reactor`) to fetch pinned SDK / Windows App SDK / WebView2 packages.
  Restores into the NuGet global cache (`%USERPROFILE%\.nuget\packages`, or
  `NUGET_PACKAGES`); falls back to a nuget.org download via bundled `curl`/`tar`.
- **`reactor-setup`'s `stage_pkg` / `dl_nupkg`** (`crates/libs/reactor-setup/src/acquire.rs`)
  — used at build time to stage the Windows App SDK runtime and WebView2 projection.
  Downloads to a per-user cache under `windows-reactor-setup/temp`.

Both use `https://www.nuget.org/api/v2/package/{id}/{version}`. They are kept
separate on purpose: `reactor-setup` runs in the `build.rs` of every consuming
app and is deliberately dependency-free (standard library plus the `curl.exe`/
`tar.exe` that ship with Windows), so it does not take on `windows-clang`,
`clang-sys`, and libclang provisioning just to fetch a package.
