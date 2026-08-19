# Dependencies

This page tracks every external SDK, header set, metadata file, and runtime the build and tooling
depend on: the version, where it is pinned, how it is obtained, and how the pin is validated. It
covers *build/tooling and runtime* dependencies (Windows SDK, WDK, WinRT contracts, libclang,
Windows App SDK, WebView2). It does **not** cover the crates.io Rust dependencies in `Cargo.toml`,
which are already centralized under `[workspace.dependencies]`.

## The model: one owner, validated by running

There is no central version registry. Every external dependency has one **owner** that declares the
pin as a `const`. Generators, validators, and tests consume that pin in CI:

- **Generators** (`tool_win32`, `tool_winrt`, `tool_webview`, and `tool_composition`) regenerate
  committed artifacts; `gen.yml` runs each then `git diff --exit-code`, so a stale pin produces a
  diff and fails.
- **Validators** (`tool_clang`, and guards inside generators) assert invariants and write nothing; a
  violation panics loudly, failing the job with a clean tree.

When one crate must track a pin another crate owns, the consumer reads the peer's constant straight
from source with [`helpers::read_str_const`](../crates/tools/helpers/src/lib.rs) and asserts they
agree. Pins are never copied - they are read back and checked. `windows-clang` stays a clean
libclang library and is **not** a shared home for SDK/runtime versions.

## At a glance

| Dependency | Version | Owner | Validation |
| --- | --- | --- | --- |
| libclang | `22.1.8` | `windows-clang` `LIBCLANG_VERSION` | `tool_clang` |
| Windows SDK | `10.0.28000.2270` | `tool_win32` `SDK_VERSION` | `tool_win32` |
| Windows WDK | `10.0.28000.1839` | `tool_win32` `WDK_VERSION` | `tool_win32` |
| SDK Contracts | `10.0.28000.2270` | `tool_winrt` `CONTRACTS_VERSION` | `tool_winrt` |
| WebView2 SDK | `1.0.4078.44` | `tool_webview` `WEBVIEW2_VERSION` | `tool_webview` |
| Windows App SDK | `2.4.0` | `reactor-setup` `RUNTIME_VER` | setup, Reactor, and binding tests |
| WebView2 projection | `1.0.4078.44` | `reactor-setup` `WEBVIEW2_VER` | `tool_webview` |

## Toolchain: libclang

The header scrapers (`tool_win32`, `tool_webview`) parse C/C++ with libclang. The
version is pinned because clang's macro-capture behavior drifts across majors, which would silently
change the generated metadata.

- **Owner:** `provision.rs` declares `LIBCLANG_VERSION = "22.1.8"`. `libclang.dll` comes from the
  `libclang.runtime.win-<arch>` NuGet packages (`dotnet/clangsharp`, .NET Foundation) fetched at
  that version. The paired clang builtin *resource headers* (needed only for the non-x64 arch
  passes, to reconcile the aarch64 `__prefetch` builtin) come from a blobless, shallow, sparse `git`
  checkout of `clang/lib/Headers` at the derived `llvmorg-<ver>` tag - so the DLL and headers are
  the *same* single-const pin and can never drift.
- **Obtained:** if `LIBCLANG_PATH` is unset, `ensure_libclang` fetches the pinned
  `libclang.runtime.win-<arch>` package via `nuget_package` (the shared NuGet global cache, same as
  the SDK/WDK/WebView2 pins) and points `LIBCLANG_PATH` at its `runtimes/<rid>/native/`; non-x64
  passes also fetch the pinned LLVM resource headers via `git`. `LIBCLANG_PATH` /
  `CLANG_RESOURCE_DIR` override for offline machines. All three scrapers call it, so their `gen.yml`
  jobs need no LLVM install - they always parse with the pinned `22.1.8`, in CI and on a fresh
  checkout alike.
- **CI:** every workflow self-provisions the pinned libclang from NuGet - no CI job installs LLVM.
  The `gen.yml` scrapers call `ensure_libclang`; `clippy.yml` loads no libclang at all
  (`cargo clippy` never parses); and `test.yml`, whose `test_clang` suite loads libclang at runtime,
  exports `LIBCLANG_PATH` from the same pin via `echo "LIBCLANG_PATH=$(cargo run -q -p tool_clang --
  path)"
  >> "$GITHUB_ENV"`. `tool_clang path` prints `windows_clang::libclang_dir()`, keeping the `unsafe`
`set_var` off the multithreaded test runner. The Linux CI jobs build code that needs no libclang.
- **Validated by `tool_clang`:** fetches, loads, and version-asserts the pin (the same provisioning
  the scrapers run). Writes nothing.
- **To update:** bump `LIBCLANG_VERSION` - a single const that drives both the NuGet DLL and the
  `llvmorg-<ver>` git tag for the headers, so there is nothing else to touch. Run `tool_clang` (must
  pass) and regenerate all metadata. No prebuilt-asset ceiling: the DLL comes from NuGet and the
  headers from a git tag, both of which track current LLVM.

## Windows SDK, WDK, and WinRT contracts

Each feeds an in-house generator producing a committed `.winmd` in `windows-default`. The metadata
is embedded for Rust build tools and remains available as files for external tools. Provenance is
documented in [`crates/libs/default/readme.md`](../crates/libs/default/readme.md).

`windows-bindgen`, `windows-rdl`, and `windows-clang` depend on `windows-default` so their builders
can select this metadata without filesystem paths. Binaries linking those crates include both
metadata files.

| Package | Owner (pin) | Produces |
| --- | --- | --- |
| `Microsoft.Windows.SDK.CPP[.<arch>]` | `SDK_VERSION` - `tool_win32` | um scrape |
| `Microsoft.Windows.WDK.x64` | `WDK_VERSION` - `tool_win32` | km scrape |
| `Microsoft.Windows.SDK.Contracts` | `CONTRACTS_VERSION` - `tool_winrt` | `Windows.winmd` |

- **Obtained:** `windows_clang::nuget_package(id, version)` restores from the NuGet global cache or
  downloads from nuget.org (`NUGET_PACKAGES` overrides the cache).
- **`tool_win32` runs both scrapes**, so the WDK km scrape reuses the same `SDK_VERSION` headers as
  the um scrape (they share the crate); `WDK_VERSION` is a separate pin in `km.rs`.
- The "marketing" include/lib folder nested in each package (e.g. `10.0.28000.0`) is **derived**
  from the version via `helpers::marketing_dir` (first three components + `.0`), so the version is
  the single edit - never a second folder constant to keep in sync.
- **To update:** bump the owning constant, run `cargo run -p tool_<win32|winrt>`, and commit the
  regenerated `.rdl` snapshot(s) and `.winmd`.

`CONTRACTS_VERSION` happens to share the `10.0.28000` build number with `SDK_VERSION`, but it is a
distinct NuGet package with its own pin - the two are not coupled and can diverge.

## WebView2

WebView2 ships only C/C++ SDK headers, so `windows-webview` is scraped from them. The WinRT `Core`
metadata and runtime projection DLL are separate artifacts.

- `WebView2.h` and `WebView2Interop.h` come from the `WEBVIEW2_VERSION` NuGet package and generate
  `crates/libs/webview/src/bindings.rs`.
- `Microsoft.Web.WebView2.Core.winmd` comes from the same package and generates the XAML bindings.
- `Microsoft.Web.WebView2.Core.dll` is staged by `reactor-setup` for self-contained applications.
- The Evergreen runtime installed by `.github/workflows/webview.yml` is the CI test host.

- **Headers are downloaded, not vendored:** `tool_webview` fetches the pinned NuGet package via
  `nuget_package` and parses the headers from it. A bump is a one-line `WEBVIEW2_VERSION` edit.
- **Pinned libclang:** like `tool_win32`, `tool_webview` calls `ensure_libclang` +
  `assert_libclang_version`, so it parses with the exact pinned `22.1.8` (its `gen.yml` job needs no
  LLVM install - only the SDK include paths for the system headers `WebView2.h` pulls in).
- **Runtime projection:** bump `WEBVIEW2_VER` in `reactor-setup` with `WEBVIEW2_VERSION`.
  `tool_webview` asserts that they match.
- **`Core.winmd`:** `tool_webview` reads it from the same pinned NuGet package as the headers.

For the full pipeline and COM<->WinRT bridge, see [windows-webview](crates/windows-webview.md).

## WinUI / Windows App SDK

`windows-reactor` is hand-written and has no semantic generator. Its private WinUI binding layer is
generated by `tool_bindings` from a fixed filter. `windows-reactor-setup` owns the Windows App SDK
runtime pin used by applications and by tools that need matching WinUI metadata.

`helpers::windows_app_sdk_metadata` reads `RUNTIME_VER = "2.4.0"` from `reactor-setup`, downloads
the `Microsoft.WindowsAppSDK` umbrella package, resolves the Foundation,
InteractiveExperiences, and WinUI component versions from its nuspec, and returns their metadata
paths. `tool_bindings`, `tool_composition`, and `tool_webview` use those paths when generating
lifted bindings.

- WinUI metadata is downloaded from components resolved through `RUNTIME_VER` by
  `tool_bindings`, `tool_composition`, and `tool_webview`.
- `Microsoft.WindowsAppSDK.Runtime` is staged by `reactor-setup` for self-contained applications.
- Bootstrap DLLs under `crates/libs/reactor-setup/bootstrap/` support framework-dependent apps and
  are refreshed by `tool_bindings` from the matching Foundation package.
- `app.manifest` and `runtime.txt` support runtime staging.
- `.github/workflows/reactor.yml` installs the runtime for native CI.

`RUNTIME_VER` drives self-contained runtime staging and metadata resolution. The native Reactor
workflow installs the same version for tests. `WEBVIEW2_VER` must equal the `tool_webview`
`WEBVIEW2_VERSION`; the tool checks that relation before generation.

- **To update the Windows App SDK:** bump `RUNTIME_VER`, replace the committed bootstrap DLLs with
  the matching Foundation package files, update the `reactor.yml` installer URLs, then run
  `cargo run -p tool_composition`, `cargo run -p tool_webview`,
  `cargo test -p windows-reactor-setup`, and the Reactor checks.
- `assets/app.manifest` is a **generated activation asset with no committed generator**. It
  transforms the App SDK `package.appxfragment` files into SxS fusion format (source versions in
  its header). It is forward-compatible, so it is refreshed only when the Reactor control set
  needs newly-moved classes, not on every bump.

See [windows-reactor](crates/windows-reactor.md) and
[windows-reactor-setup](crates/windows-reactor-setup.md).

## Download mechanisms

Two independent NuGet paths, both using `https://www.nuget.org/api/v2/package/{id}/{version}`:

- **`windows_clang::nuget_package`** - used by the scraping and binding tools (`tool_win32`,
  `tool_winrt`, `tool_webview`, `tool_composition`). Restores into the NuGet global cache
  (`NUGET_PACKAGES` overrides), else downloads via bundled `curl`/`tar`. Layout-agnostic - each
  caller indexes the subtree it needs.
- **`reactor-setup`'s staging** - runs in the `build.rs` of every consuming app to stage the App SDK
  runtime and WebView2 projection. Dependency-free (std + the `curl.exe`/`tar.exe` shipped with
  Windows), which is why it does not reuse `nuget_package`.

## The `tool_*` / pin pairing

| Tool | Proves |
| --- | --- |
| `tool_win32` | `SDK_VERSION`, `WDK_VERSION`, and `Windows.Win32.winmd` |
| `tool_winrt` | `CONTRACTS_VERSION` and `Windows.winmd` |
| `tool_clang` | `LIBCLANG_VERSION` |
| `tool_composition` | `RUNTIME_VER` metadata selection and lifted bindings |
| `tool_webview` | WebView2 pins, Windows App SDK metadata selection, and both binding sets |

All cross-file reads go through `helpers::read_str_const`, so each pin is declared once by its owner
and read back everywhere else.
