# Dependencies

This page tracks every external SDK, header set, metadata file, and runtime the build and
tooling depend on: the version, where it is pinned, how it is obtained, and how the pin is
kept honest. It covers *build/tooling and runtime* dependencies (Windows SDK, WDK, WinRT
contracts, libclang, Windows App SDK, WebView2). It does **not** cover the crates.io Rust
dependencies in `Cargo.toml`, which are already centralized under `[workspace.dependencies]`.

## The model: one owner, validated by running

There is no central version registry. Every external dependency has a single **owning tool**
that declares the pin as a `const`, downloads/consumes the artifact at that exact version, and
runs in CI (`.github/workflows/gen.yml`) — so *running the tool proves the pin is current*, in
one of two ways:

- **Generators** (`tool_win32`, `tool_wdk`, `tool_winrt`, `tool_webview`, `tool_reactor`)
  regenerate committed artifacts; `gen.yml` runs each then `git diff --exit-code`, so a stale
  pin produces a diff and fails.
- **Validators** (`tool_clang`, and guards inside generators) assert invariants and write
  nothing; a violation panics loudly, failing the job with a clean tree.

When one crate must track a pin another crate owns, the owner reads the peer's constant
straight from source with [`helpers::read_str_const`](../crates/tools/helpers/src/lib.rs) and
asserts they agree. Pins are never copied — they are read back and checked. `windows-clang`
stays a clean libclang library and is **not** a shared home for SDK/runtime versions.

## At a glance

| Dependency | Version | Owner (pin) | Obtained by | Kept honest by |
| --- | --- | --- | --- | --- |
| libclang | `18.1.1` | `LIBCLANG_VERSION` — `crates/libs/clang/src/provision.rs` | download (PyPI wheel + LLVM headers) | `tool_clang` validator + runtime assert |
| Windows SDK | `10.0.28000.2270` | `SDK_VERSION` — `crates/tools/win32/src/main.rs` | download (NuGet) | `tool_win32` zero-diff regen |
| Windows WDK | `10.0.28000.1839` | `WDK_VERSION` — `crates/tools/wdk/src/main.rs` | download (NuGet) | `tool_wdk` zero-diff regen; reads `SDK_VERSION` from `tool_win32` |
| SDK Contracts (WinRT) | `10.0.28000.2270` | `CONTRACTS_VERSION` — `crates/tools/winrt/src/main.rs` | download (NuGet) | `tool_winrt` zero-diff regen |
| WebView2 SDK headers | `1.0.4022.49` | `WEBVIEW2_VERSION` — `crates/tools/webview/src/main.rs` | download (NuGet) | `tool_webview` zero-diff regen |
| WinUI / Windows App SDK metadata (`.winmd` corpus) | `2.1.3` | `WINDOWS_APP_SDK_VERSION` — `crates/tools/reactor/src/main.rs` | download (NuGet) | `tool_reactor` zero-diff regen of the committed corpus |
| Windows App SDK runtime | `2.1.3` | `RUNTIME_VER` — `crates/libs/reactor-setup/src/lib.rs` | download (NuGet) + committed bootstrap DLLs | `tool_reactor` guard: `== WINDOWS_APP_SDK_VERSION`, and `reactor.yml` matches |
| WebView2 runtime projection | `1.0.4022.49` | `WEBVIEW2_VER` — `crates/libs/reactor-setup/src/lib.rs` | download (NuGet) | `tool_reactor` guard: `== WEBVIEW2_VERSION` |
| LLVM / libclang (CI) | `18` (Windows), `20` (Linux) | `version:` — `clippy.yml`, `test.yml` | `KyleMayes/install-llvm-action` | `tool_clang`: Windows `version:` `== LIBCLANG_VERSION` major |

## Toolchain: libclang

The header scrapers (`tool_win32`, `tool_wdk`, `tool_webview`) parse C/C++ with libclang. The
version is pinned because clang's macro-capture behavior drifts across majors, which would
silently change the generated corpus.

- **Owner:** `provision.rs` declares `LIBCLANG_VERSION = "18.1.1"`; the PyPI wheel URLs and the
  LLVM `CLANG_RESOURCE_URL` all embed that string.
- **Obtained:** if `LIBCLANG_PATH` is unset, `ensure_libclang` downloads the pinned wheel from
  PyPI into `target/windows-clang/`; non-x64 passes also fetch version-matched LLVM resource
  headers. `LIBCLANG_PATH` / `CLANG_RESOURCE_DIR` override for offline machines. All three
  scrapers call it, so their `gen.yml` jobs need no LLVM install — they always parse with the
  pinned `18.1.1`, in CI and on a fresh checkout alike.
- **CI:** the `gen.yml` scrapers self-provision (above). `clippy.yml` and `test.yml` install LLVM
  via `KyleMayes/install-llvm-action@v2` to *build and test* the clang-based crates — `18` on
  Windows (matched to `LIBCLANG_VERSION`), `20` on Linux (consumes generated code only).
- **Validated by `tool_clang`:** asserts the wheel/resource URLs embed `LIBCLANG_VERSION` and
  that each Windows-job `version:` (`clippy.yml`, `test.yml`) equals its major. Writes nothing.
- **To update:** bump `LIBCLANG_VERSION`, the wheel URLs, `CLANG_RESOURCE_URL`, and the Windows
  `version:` in the workflows; run `tool_clang` (must pass) and regenerate all corpora.

## Windows SDK, WDK, and WinRT contracts

Each feeds an in-house generator producing a committed `.winmd` under
`crates/libs/bindgen/default/`, consumed by `windows-bindgen` as `--in default`. Provenance is
documented in [`crates/libs/bindgen/default/readme.md`](../crates/libs/bindgen/default/readme.md).

| Package | Owner (pin) | Produces |
| --- | --- | --- |
| `Microsoft.Windows.SDK.CPP[.<arch>]` | `SDK_VERSION` — `tool_win32` | `Windows.Win32.winmd` |
| `Microsoft.Windows.WDK.x64` | `WDK_VERSION` — `tool_wdk` | `Windows.Wdk.winmd` |
| `Microsoft.Windows.SDK.Contracts` | `CONTRACTS_VERSION` — `tool_winrt` | `Windows.winmd` |

- **Obtained:** `windows_clang::nuget_package(id, version)` restores from the NuGet global cache
  or downloads from nuget.org (`NUGET_PACKAGES` overrides the cache).
- **`tool_wdk` needs the SDK headers too**, so it reads `SDK_VERSION` back from `tool_win32`
  rather than carry its own copy; `WDK_VERSION` is its own pin.
- The "marketing" include/lib folder nested in each package (e.g. `10.0.28000.0`) is **derived**
  from the version via `helpers::marketing_dir` (first three components + `.0`), so the version
  is the single edit — never a second folder constant to keep in sync.
- **To update:** bump the owning constant, run `cargo run -p tool_<win32|wdk|winrt>`, and commit
  the regenerated `.rdl` snapshot(s) and `.winmd`.

`CONTRACTS_VERSION` happens to share the `10.0.28000` build number with `SDK_VERSION`, but it is
a distinct NuGet package with its own pin — the two are not coupled and can diverge.

## WebView2

WebView2 ships only C/C++ SDK headers, so `windows-webview` is scraped from them. The WinRT
`Core` metadata and runtime projection DLL are separate artifacts.

| Artifact | Owner / location | Used by |
| --- | --- | --- |
| `WebView2.h`, `WebView2Interop.h` | `WEBVIEW2_VERSION` — `tool_webview` (downloaded, not vendored) | `tool_webview` → `webview/src/bindings.rs` |
| `Microsoft.Web.WebView2.Core.winmd` | regenerated into `crates/tools/reactor/winmd/` by `tool_reactor` at `WEBVIEW2_VERSION` | `tool_webview`, `tool_reactor` |
| Runtime projection (`Core.dll`), `WEBVIEW2_VER` | `crates/libs/reactor-setup/src/lib.rs` | self-contained apps |
| Evergreen runtime | `.github/workflows/webview.yml` | CI test host |

- **Headers are downloaded, not vendored:** `tool_webview` fetches the pinned NuGet package via
  `nuget_package` and parses the headers from it. A bump is a one-line `WEBVIEW2_VERSION` edit.
- **Pinned libclang:** like `tool_win32`/`tool_wdk`, `tool_webview` calls `ensure_libclang` +
  `assert_libclang_version`, so it parses with the exact pinned `18.1.1` (its `gen.yml` job needs
  no LLVM install — only the SDK include paths for the system headers `WebView2.h` pulls in).
- **Runtime projection:** bump `WEBVIEW2_VER` in `reactor-setup`; it must equal
  `WEBVIEW2_VERSION` — `tool_reactor` asserts this.
- **`Core.winmd`** is refreshed into the reactor corpus by `tool_reactor` at `WEBVIEW2_VERSION`
  (see below).

For the full pipeline and COM↔WinRT bridge, see [windows-webview](crates/windows-webview.md).

## WinUI / Windows App SDK

`windows-reactor` is generated from WinUI / Windows App SDK `.winmd`; `windows-reactor-setup`
stages the matching runtime so reactor apps run. Metadata and runtime are two faces of one
release, tied to a single number.

**Metadata is regenerated, not hand-copied.** `tool_reactor` owns
`WINDOWS_APP_SDK_VERSION = "2.1.3"`. On every run it downloads the umbrella
`Microsoft.WindowsAppSDK` metapackage at that version, reads the exact component versions
(Foundation / InteractiveExperiences / WinUI) pinned in its nuspec, downloads each component,
and copies their `.winmd` — plus `Microsoft.Web.WebView2.Core.winmd` at `WEBVIEW2_VERSION` —
into the committed corpus at `crates/tools/reactor/winmd/`. `gen.yml` re-runs the tool and fails
on any diff, so the corpus provably matches the pin. (`extras.winmd` in that directory is
*generated* by `tool_reactor` from `Windows.Win32.winmd`, not a package.) The corpus stays
committed because `tool_webview` and `tool_composition` also read it.

| Artifact | Owner / location | Used by |
| --- | --- | --- |
| WinUI / Windows App SDK `.winmd` + `WebView2.Core.winmd` | regenerated into `crates/tools/reactor/winmd/` at `WINDOWS_APP_SDK_VERSION` | `tool_reactor`, `tool_webview`, `tool_composition` |
| `Microsoft.WindowsAppSDK.Runtime`, `RUNTIME_VER` | `crates/libs/reactor-setup/src/lib.rs` | app runtime deploy |
| Bootstrap DLLs (x86/x64/arm64) | `crates/libs/reactor-setup/bootstrap/` (committed) | framework-dependent apps |
| `resources.pri`, `app.manifest`, `runtime.txt` | `crates/libs/reactor-setup/assets/` (committed) | runtime staging |
| Runtime installer | `.github/workflows/reactor.yml` | CI test host |

`reactor-setup` is a published runtime crate with no generated artifact, so its pins can't be
proven by regen. Instead **`tool_reactor` guards them** on every run, asserting (and failing
loudly on drift) that:

- `WINDOWS_APP_SDK_VERSION` (metadata) equals `RUNTIME_VER` (runtime) — one number drives both;
- `WEBVIEW2_VER` equals `WEBVIEW2_VERSION`, so the staged WebView2 runtime matches the ABI the
  bindings target; and
- `reactor.yml`'s installer URL installs `RUNTIME_VER` (`.../windowsappsdk/<major.minor>/<ver>/`),
  so CI's self-tests exercise the runtime apps ship.

- **To update metadata + runtime:** bump `WINDOWS_APP_SDK_VERSION` (`tool_reactor`) and
  `RUNTIME_VER` (`reactor-setup`) together, refresh the committed bootstrap DLLs from the target
  build's Foundation package, update the `reactor.yml` installer URL, then run
  `cargo run -p tool_reactor` and commit the refreshed corpus. The guard enforces the version
  agreement; the regen enforces the corpus.
- `assets/app.manifest` / `assets/resources.pri` are **generated activation assets with no
  committed generator** — `app.manifest` transforms the App SDK `package.appxfragment` files
  into SxS fusion format (source versions in its header). They are forward-compatible, so
  refreshed only when the reactor control set needs newly-moved classes, not on every bump.
  Automating them is tracked in [windows-reactor](crates/windows-reactor.md).

See [windows-reactor](crates/windows-reactor.md) and
[windows-reactor-setup](crates/windows-reactor-setup.md).

## Download mechanisms

Two independent NuGet paths, both using `https://www.nuget.org/api/v2/package/{id}/{version}`:

- **`windows_clang::nuget_package`** — used by the scraping/codegen tools (`tool_win32`,
  `tool_wdk`, `tool_winrt`, `tool_webview`, `tool_reactor`). Restores into the NuGet global cache
  (`NUGET_PACKAGES` overrides), else downloads via bundled `curl`/`tar`. Layout-agnostic — each
  caller indexes the subtree it needs.
- **`reactor-setup`'s staging** — runs in the `build.rs` of every consuming app to stage the App
  SDK runtime and WebView2 projection. Deliberately dependency-free (std + the `curl.exe`/`tar.exe`
  shipped with Windows), which is why it does not reuse `nuget_package`.

## The `tool_*` / pin pairing

| Tool | Proves | How |
| --- | --- | --- |
| `tool_win32` | `SDK_VERSION` | zero-diff regen of `Windows.Win32.winmd` |
| `tool_wdk` | `WDK_VERSION`; SDK sync | zero-diff regen; reads `tool_win32`'s `SDK_VERSION` |
| `tool_winrt` | `CONTRACTS_VERSION` | zero-diff regen of `Windows.winmd` |
| `tool_webview` | `WEBVIEW2_VERSION` | zero-diff regen of `webview/src/bindings.rs` |
| `tool_clang` | `LIBCLANG_VERSION` ↔ URLs ↔ CI LLVM | pure-check assertions |
| `tool_reactor` | `WINDOWS_APP_SDK_VERSION`; reactor-setup sync | zero-diff regen of the winmd corpus + bindings; guard reads reactor-setup constants |

All cross-file reads go through `helpers::read_str_const`, so each pin is declared once by its
owner and read back everywhere else.

## Known gaps

1. **`app.manifest` / `resources.pri` have no committed generator** — generated activation assets
   checked in without their transform.
2. **Two NuGet download helpers.** `nuget_package` and `reactor-setup`'s staging duplicate the
   download/extract logic. The split is intentional (`reactor-setup` must stay dependency-free),
   but the shared glue is not factored out.
3. **Minor naming drift.** The NuGet package id is a named const in `tool_winrt`
   (`CONTRACTS_ID`) but inlined in `tool_win32`/`tool_wdk`.