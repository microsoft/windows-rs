# Dependencies

This page tracks every external SDK, header set, metadata file, and runtime that the
build and tooling depend on: what version is used, where that version is pinned, how the
artifact is obtained, and how the pin is kept honest. It covers *build/tooling and
runtime* dependencies (the Windows SDK, WDK, WinRT contracts, libclang, the Windows App
SDK, WebView2, …) — **not** the crates.io Rust dependencies in `Cargo.toml`, which are
already centralized under `[workspace.dependencies]`.

## The model: one owner, validated by running

There is no central version registry. Instead every external dependency has a single
**owning tool** that:

1. **declares** the pin as a `const` in its own source,
2. **downloads / consumes** the artifact at that exact version, and
3. is **run by CI** (`.github/workflows/gen.yml`), so *running the tool proves the pin is
   current*.

"Proven by running" takes one of two shapes:

- **Generators** (`tool_win32`, `tool_wdk`, `tool_winrt`, `tool_webview`, `tool_reactor`,
  `tool_bindings`, `tool_package`) regenerate committed artifacts. `gen.yml` runs each
  tool and then `git diff --exit-code`: a stale pin produces a diff and fails the job.
- **Pure-check validators** (`tool_clang`, and the guards embedded in generators)
  assert invariants and write nothing. A violated invariant panics loudly, failing the
  job while leaving the tree clean.

When one crate must stay in step with a pin that another crate owns, the owner (or a
validator) **reads the peer's declared constant straight from source and asserts they
agree**, using the shared primitive
[`helpers::read_str_const(path, name)`](../crates/tools/helpers/src/lib.rs). This is the
one mechanism that replaces duplicated constants: pins are never copied, they are read
back and checked. `windows-clang` stays a clean libclang library and is **not** used as a
shared home for unrelated SDK/runtime versions.

## At a glance

| Dependency | Version | Owner (pin location) | Obtained by | Kept honest by |
| --- | --- | --- | --- | --- |
| libclang | `18.1.1` | `LIBCLANG_VERSION` — `crates/libs/clang/src/provision.rs` | download (PyPI wheel + LLVM release headers) | `tool_clang` (validator) + runtime assert |
| Windows SDK (C/C++ headers) | `10.0.28000.2270` | `SDK_VERSION` — `crates/tools/win32/src/main.rs` | download (NuGet) | `tool_win32` zero-diff regen |
| Windows WDK (kernel headers) | `10.0.28000.1839` | `WDK_VERSION` — `crates/tools/wdk/src/main.rs` | download (NuGet) | `tool_wdk` zero-diff regen; reads `SDK_VERSION` back from `tool_win32` |
| Windows SDK Contracts (WinRT) | `10.0.28000.2270` | `CONTRACTS_VERSION` — `crates/tools/winrt/src/main.rs` | download (NuGet) | `tool_winrt` zero-diff regen |
| WebView2 SDK (C/C++ headers) | `1.0.4022.49` | `WEBVIEW2_VERSION` — `crates/tools/webview/src/main.rs` | download (NuGet) | `tool_webview` zero-diff regen |
| WinUI / Windows App SDK metadata (~26 `.winmd`) + `Microsoft.Web.WebView2.Core.winmd` | not pinned by a constant | vendored: `crates/tools/reactor/winmd/*.winmd` | committed | — (see [Known gaps](#known-gaps)) |
| Windows App SDK Runtime | `2.1.3` | `RUNTIME_VER` — `crates/libs/reactor-setup/src/lib.rs` | download (NuGet) + committed bootstrap DLLs | `tool_reactor` guard asserts `reactor.yml` matches |
| WebView2 runtime projection (`Core.dll`) | `1.0.4022.49` | `WEBVIEW2_VER` — `crates/libs/reactor-setup/src/lib.rs` | download (NuGet) | `tool_reactor` guard asserts `== tool_webview` pin |
| Windows App SDK Runtime (CI test host) | `2.1.3` | installer URL — `.github/workflows/reactor.yml` | download (installer) | `tool_reactor` guard asserts `== RUNTIME_VER` |
| WebView2 Evergreen runtime (CI test host) | Evergreen | `.github/workflows/webview.yml` | download (installer) | — |
| LLVM / libclang (CI) | `18` (Windows), `20` (Linux) | `version:` — `clippy.yml`, `gen.yml`, `test.yml` | `KyleMayes/install-llvm-action` | `tool_clang` asserts Windows `version:` `== LIBCLANG_VERSION` major |

## Toolchain: libclang

The header scrapers (`tool_win32`, `tool_wdk`, `tool_webview`) parse C/C++ with libclang.
The version is pinned because clang's macro-capture behavior drifts across major
versions, which would silently change the generated corpus; it is asserted at startup
against the loaded libclang.

- **Owner:** `crates/libs/clang/src/provision.rs` declares `LIBCLANG_VERSION = "18.1.1"`.
  The PyPI wheel URLs and the LLVM `CLANG_RESOURCE_URL` all embed that same version
  string.
- **How it's obtained:** if `LIBCLANG_PATH` is unset, `ensure_libclang` downloads the
  pinned host-arch `libclang` wheel from PyPI and caches it under `target/windows-clang/`.
  Non-x64 passes also fetch the version-matched LLVM resource headers
  (`CLANG_RESOURCE_URL`). `LIBCLANG_PATH` / `CLANG_RESOURCE_DIR` override for offline
  machines.
- **CI:** installs LLVM via `KyleMayes/install-llvm-action@v2` — version `18` on the
  Windows jobs (the corpus-generating scrapers, matched to `LIBCLANG_VERSION`) and `20`
  on the Linux jobs (which only consume already-generated code), in `clippy.yml`,
  `gen.yml`, and `test.yml`.
- **Validated by `tool_clang`** (`crates/tools/clang`, a pure-check validator run from
  `gen.yml`): it asserts the wheel/resource URLs in `provision.rs` all embed
  `LIBCLANG_VERSION`, and that every Windows-job `version:` in the three workflows equals
  `LIBCLANG_VERSION`'s major (the Linux `20` is intentionally skipped). It writes nothing;
  it fails loudly on drift.
- **To update:** bump `LIBCLANG_VERSION`, the wheel URLs, and `CLANG_RESOURCE_URL` in
  `provision.rs`, and the Windows `version:` in the workflows; then run `tool_clang`
  (must pass) and regenerate all corpora (must be diff-free).

## Windows SDK, WDK, and WinRT contracts

These feed the in-house metadata generators. Each produces a committed `.winmd` under
`crates/libs/bindgen/default/`, which `windows-bindgen` consumes as `--in default`. The
provenance of each output winmd (and the human-reviewable `metadata/*.rdl` snapshot it
round-trips through) is documented in
[`crates/libs/bindgen/default/readme.md`](../crates/libs/bindgen/default/readme.md).

| Package | Owner (pin) | Produces |
| --- | --- | --- |
| `Microsoft.Windows.SDK.CPP[.<arch>]` | `SDK_VERSION` — `crates/tools/win32/src/main.rs` | `Windows.Win32.winmd` (via `tool_win32`) |
| `Microsoft.Windows.WDK.x64` | `WDK_VERSION` — `crates/tools/wdk/src/main.rs` | `Windows.Wdk.winmd` (via `tool_wdk`) |
| `Microsoft.Windows.SDK.Contracts` | `CONTRACTS_VERSION` — `crates/tools/winrt/src/main.rs` | `Windows.winmd` (via `tool_winrt`) |

- **How they're obtained:** `windows_clang::nuget_package(id, version)` restores the
  package from the NuGet global cache, or downloads it from nuget.org on first use.
  `NUGET_PACKAGES` overrides the cache location.
- **`tool_wdk` needs the SDK headers too**, so instead of carrying its own copy of the
  SDK version it **reads `SDK_VERSION` back from `tool_win32`** via
  `helpers::read_str_const` — `tool_win32` is the single owner, and `tool_wdk` fails
  loudly if that read ever stops resolving. (`WDK_VERSION` is `tool_wdk`'s own pin.)
- **To update:** bump the owning constant, run the corresponding
  `cargo run -p tool_<win32|wdk|winrt>`, and commit the regenerated `.rdl` snapshot(s) and
  `.winmd`. The `gen` CI job re-runs each tool and fails on any diff.

`CONTRACTS_VERSION` is intentionally aligned to the same `10.0.28000` SDK build as
`SDK_VERSION` rather than tracking the latest stable contracts package; it is a distinct
NuGet package owned by `tool_winrt`.

## WebView2

WebView2 ships only C/C++ SDK headers (no `.rdl`/`.winmd`), so the `windows-webview`
bindings are scraped from those headers. The WinRT `Core` metadata and the runtime
projection DLL are separate artifacts.

| Artifact | Owner / location | Used by |
| --- | --- | --- |
| `WebView2.h`, `WebView2Interop.h` | `WEBVIEW2_VERSION` — `crates/tools/webview/src/main.rs` (downloaded from NuGet, **not** vendored) | `tool_webview` → `crates/libs/webview/src/bindings.rs` |
| `Microsoft.Web.WebView2.Core.winmd` | `crates/tools/reactor/winmd/` (committed, part of the vendored reactor metadata set) | `tool_webview` (interop bridge), `tool_reactor` |
| `Microsoft.Web.WebView2` runtime projection (`Core.dll`), `WEBVIEW2_VER = 1.0.4022.49` | `crates/libs/reactor-setup/src/lib.rs` | self-contained apps (deploys `Core.dll`) |
| Evergreen runtime | `.github/workflows/webview.yml` | CI test host (COM `webview2loader.dll`) |

- **The headers are downloaded, not vendored.** `tool_webview` fetches the pinned
  `Microsoft.Web.WebView2` NuGet package via `windows_clang::nuget_package` and parses
  `build/native/include/WebView2.h` and `build/native/include-winrt/WebView2Interop.h`
  straight from the restored package. A version bump is a one-line `WEBVIEW2_VERSION`
  edit; `gen.yml` re-runs `tool_webview` and fails if `bindings.rs` changes.
- **To update the runtime projection:** bump `WEBVIEW2_VER` in `reactor-setup`. It must
  match `tool_webview`'s `WEBVIEW2_VERSION` — **`tool_reactor` asserts this** (see
  [WinUI / Windows App SDK](#winui--windows-app-sdk)).
- The vendored `Core.winmd` under `crates/tools/reactor/winmd/` is the one WebView2
  artifact still without a version constant (see [Known gaps](#known-gaps)).

For the full WebView2 build pipeline and the COM↔WinRT interop bridge, see
[windows-webview](crates/windows-webview.md).

## WinUI / Windows App SDK

The `windows-reactor` bindings are generated from WinUI / Windows App SDK `.winmd`
metadata; the `windows-reactor-setup` crate stages the matching runtime so reactor apps
can run. These are two **independent** artifacts, updated by separate steps.

- The `.winmd` **metadata** is a *codegen input* consumed by `tool_reactor`. Today the
  ~26 WinUI / Windows App SDK winmd (plus `Microsoft.Web.WebView2.Core.winmd`) are
  **vendored** under `crates/tools/reactor/winmd/` and loaded from there
  (`MetadataResolver::load("crates/tools/reactor/winmd")`). There is no version constant
  recording which Windows App SDK release they came from — the version is implicit in the
  committed blobs. `extras.winmd` in that directory is *generated* by `tool_reactor` from
  `Windows.Win32.winmd`, not vendored from a package.
- The **runtime** (`RUNTIME_VER`) is downloaded by `reactor-setup` at the *consuming
  app's* build time to stage runtime binaries next to the executable (see
  [Download mechanisms](#download-mechanisms)). It ships no metadata.

| Artifact | Owner / location | Used by |
| --- | --- | --- |
| WinUI / Windows App SDK `.winmd` (~26) + `Microsoft.Web.WebView2.Core.winmd` | `crates/tools/reactor/winmd/` (committed) | `tool_reactor` → `crates/libs/reactor/src/bindings.rs` |
| `Microsoft.WindowsAppSDK.Runtime`, `RUNTIME_VER = 2.1.3` | `crates/libs/reactor-setup/src/lib.rs` | app runtime deploy (MSIX extract) |
| `Microsoft.WindowsAppRuntime.Bootstrap.dll` (x86/x64/arm64) | `crates/libs/reactor-setup/bootstrap/` (committed) | framework-dependent apps |
| `resources.pri`, `app.manifest`, `runtime.txt` | `crates/libs/reactor-setup/assets/` (committed) | runtime staging |
| Windows App SDK runtime installer `2.1.3` | `.github/workflows/reactor.yml` | CI test host |

`reactor-setup` is a published runtime dependency with no generated artifact of its own,
so its pins can't be proven by a zero-diff regen. Instead **`tool_reactor` (which already
runs in `gen.yml`) guards them** — on every run it reads `reactor-setup`'s constants via
`helpers::read_str_const` and asserts, failing loudly on drift, that:

- `WEBVIEW2_VER` (reactor-setup) equals `WEBVIEW2_VERSION` (tool_webview), so the staged
  WebView2 runtime matches the ABI the bindings target; and
- the WinAppSDK runtime installer URL in `.github/workflows/reactor.yml` installs
  `RUNTIME_VER`, so CI's self-tests exercise the runtime apps actually ship. The aka.ms
  installer URL is `.../windowsappsdk/<major.minor>/<version>/...`, derived from
  `RUNTIME_VER`.

- **To update the metadata:** replace the vendored winmd under
  `crates/tools/reactor/winmd/` with the matching files from the target Windows App SDK
  and WebView2 NuGet packages, then run `cargo run -p tool_reactor` and
  `cargo run -p tool_bindings` and commit the regenerated bindings.
- **To update the runtime:** bump `RUNTIME_VER`, refresh the committed bootstrap DLLs
  from the target Windows App SDK build's `Microsoft.WindowsAppSDK.Foundation` package
  (`runtimes/win-<arch>/native/Microsoft.WindowsAppRuntime.Bootstrap.dll`), and update the
  CI installer URL in `reactor.yml` to the same version (the `tool_reactor` guard enforces
  this last step).
- `assets/app.manifest` and `assets/resources.pri` are **generated activation assets with
  no committed generator** — `app.manifest` is a transform of the Windows App SDK
  `runtimes-framework/package.appxfragment` files into the SxS fusion format (its source
  versions are recorded in its header comment). They map WinRT class names to runtime DLLs
  and are forward-compatible, so they are refreshed only when the curated reactor control
  set needs classes that a newer Windows App SDK moved or added — not on every version
  bump. Automating their regeneration is tracked as future work in
  [windows-reactor](crates/windows-reactor.md).

See [windows-reactor](crates/windows-reactor.md) and
[windows-reactor-setup](crates/windows-reactor-setup.md).

## Download mechanisms

There are two independent NuGet download paths:

- **`windows_clang::nuget_package`** (`crates/libs/clang/src/provision.rs`) — used by the
  header-scraping tools (`tool_win32`, `tool_wdk`, `tool_winrt`, `tool_webview`) to fetch
  pinned SDK / WebView2 packages. Restores into the NuGet global cache
  (`%USERPROFILE%\.nuget\packages`, or `NUGET_PACKAGES`); falls back to a nuget.org
  download via bundled `curl`/`tar`. It is layout-agnostic — each caller indexes into the
  package subtree it needs (`c/Include/...` for the SDK, `build/native/include/...` for
  WebView2).
- **`reactor-setup`'s package staging** (`crates/libs/reactor-setup/src/lib.rs`) — used at
  build time to stage the Windows App SDK runtime and WebView2 projection. Downloads to a
  per-user cache.

Both use `https://www.nuget.org/api/v2/package/{id}/{version}`. They are separate because
`reactor-setup` runs in the `build.rs` of every consuming app and is deliberately
dependency-free (standard library plus the `curl.exe`/`tar.exe` that ship with Windows),
so it does not take on `windows-clang`, `clang-sys`, and libclang provisioning just to
fetch a package.

## The `tool_*` / pin pairing

Each pin has a tool that runs in `gen.yml` and proves it:

| Tool | Proves | How |
| --- | --- | --- |
| `tool_win32` | `SDK_VERSION` | zero-diff regen of `Windows.Win32.winmd` |
| `tool_wdk` | `WDK_VERSION`; SDK sync | zero-diff regen; reads `tool_win32`'s `SDK_VERSION` |
| `tool_winrt` | `CONTRACTS_VERSION` | zero-diff regen of `Windows.winmd` |
| `tool_webview` | `WEBVIEW2_VERSION` | zero-diff regen of `webview/src/bindings.rs` |
| `tool_clang` | `LIBCLANG_VERSION` ↔ URLs ↔ CI LLVM | pure-check assertions, writes nothing |
| `tool_reactor` | `reactor-setup` `RUNTIME_VER`/`WEBVIEW2_VER` sync | zero-diff regen + guard reads reactor-setup constants |

The cross-file reads all go through `helpers::read_str_const`, so a pin is declared once
by its owner and read back everywhere else — never copied.

## Known gaps

Remaining places where a version is not yet pinned by a constant, or where cleanup is
deferred:

1. **The reactor WinUI / WebView2 `.winmd` are vendored with no version constant.** The
   ~26 committed winmd under `crates/tools/reactor/winmd/` (~2.5 MB) carry no record of
   which Windows App SDK / WebView2 release they came from; updating means hand-copying
   new winmd from NuGet. This is the last WebView2 artifact still lacking a pin.
2. **`app.manifest` / `resources.pri` have no committed generator.** They are generated
   activation assets checked in without the transform that produces them.
3. **Two NuGet download helpers.** `windows_clang::nuget_package` and `reactor-setup`'s
   staging duplicate the download/extract logic. This split is intentional (`reactor-setup`
   must stay dependency-free), but the shared glue is not factored out.
4. **Minor naming drift.** The SDK include-dir constant is `SDK_INCLUDE_DIR` in
   `tool_win32` but `INCLUDE_DIR` in `tool_wdk`; the NuGet package id is a named const in
   `tool_winrt` (`CONTRACTS_ID`) but inlined in `tool_win32`/`tool_wdk`.
