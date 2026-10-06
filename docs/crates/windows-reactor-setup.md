# windows-reactor-setup

> Build-time staging for self-contained Windows Reactor applications.

- 📦 [crates.io](https://crates.io/crates/windows-reactor-setup)
- 📖 [docs.rs](https://docs.rs/windows-reactor-setup)
- 🚀 [Getting started](../../crates/libs/reactor-setup/readme.md)
- 🧩 [Self-contained sample](../../crates/samples/reactor/self_contained)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor-setup)

## When to use it

Use `windows-reactor-setup` only when a
[`windows-reactor`](windows-reactor.md) executable must carry a private Windows App SDK runtime
beside the executable. It is a build dependency called from `build.rs`; it is not a runtime API and
does not belong in `[dependencies]`.

Do not use it for a framework-dependent Reactor application. `windows-reactor` already contains
the framework bootstrap that resolves an installed Windows App SDK framework package at startup.
That deployment model stages no private runtime files.

Do not use it for a plain `windows-webview` `HWND` host. The WebView2 Evergreen runtime supplies
that COM-only path. Reactor's XAML WebView2 control has an additional projection DLL requirement,
which this setup helper handles for self-contained Reactor apps.

## Prerequisites

- The Cargo target OS must be Windows.
- The build needs network access the first time each pinned NuGet package is staged.
- `%SystemRoot%\System32\curl.exe` and `tar.exe` must be available.
- The target must use MSVC, or the LLVM-based GNU ABI supported by the manifest linker arguments.
- Supported target architecture mappings are `x86` -> `x86`, `aarch64` -> `arm64`, and
  `x86_64` -> `x64`. Other architectures are rejected.

The README contains the build-dependency declaration and one-line `build.rs`.

## First workflow: produce a self-contained build

1. Add `windows-reactor-setup` under `[build-dependencies]`.
2. Create `build.rs` and call `windows_reactor_setup::as_self_contained()`.
3. Build the application normally with Cargo.
4. Run the executable from its Cargo profile output directory to confirm the staged runtime is
   used.
5. Package the executable together with all staged DLLs and runtime directories from that output
   directory. Preserve their relative layout.
6. Test the packaged directory on a machine that does not provide the framework package expected
   by a framework-dependent build.

The [`reactor/self_contained`](../../crates/samples/reactor/self_contained) sample is the reference
project layout. The [`webview/reactor`](../../crates/samples/webview/reactor) sample shows the same
setup for a Reactor app containing the XAML WebView2 control.

## What the build step does

`as_self_contained` performs these operations during the application build:

1. Resolve Cargo's profile output directory from `OUT_DIR` and `PROFILE`.
2. Download and cache the pinned `Microsoft.WindowsAppSDK.Runtime` NuGet package.
3. Extract the MSIX for the target architecture.
4. Copy the allow-listed Windows App Runtime files to the profile output directory.
5. Download and cache the pinned `Microsoft.Web.WebView2` NuGet package.
6. Copy the target architecture's `native_uap/Microsoft.Web.WebView2.Core.dll` beside the
   executable.
7. Write an application manifest containing the self-contained deployment marker.
8. Pass linker arguments that embed the manifest in binary targets.

Extracted packages are cached under `%LOCALAPPDATA%\windows-reactor-setup\temp\packages` when
`LOCALAPPDATA` is available. Cargo may rerun the build script, but completed package and
architecture-specific MSIX caches avoid downloading and extracting on every build.

## Deployment and shared target directories

The Cargo profile directory can contain outputs from several packages. Stage the application from
a clean, known build profile and copy every runtime file and subdirectory required beside the
executable. Copying only the `.exe` does not produce a self-contained deployment.

The embedded manifest includes a `windows-reactor-self-contained` description marker. Reactor reads
that marker to select the private runtime. A framework-dependent executable ignores private files
left by a self-contained build in the same Cargo target directory and uses its inlined framework
bootstrap instead.

`Microsoft.Web.WebView2.Core.dll` is always staged. This allows a self-contained app to add
`windows-webview`'s `reactor` feature without another deployment step. It is the WinRT projection
assembly used by the XAML control, not the `webview2loader.dll` used by COM-only hosting.

## Failures and cleanup

`as_self_contained` has no `Result` return. Unsupported target configuration, missing Cargo
environment variables, HTTP errors, extraction failures, missing required files, and copy failures
panic during the build. Failed restores do not publish a completed cache entry. Cleanup failures
are reported separately.

The first build therefore requires reliable NuGet access. In offline build environments, populate
the helper's package cache before disconnecting or use a build environment with the required cache.
Do not delete the cache while another build is using it.

Staged files live in the Cargo profile output directory, outside the package's `OUT_DIR`.
`cargo clean` removes the target output but not the package cache under `LOCALAPPDATA`. Remove that
specific cache directory manually only when forcing a fresh package download or recovering from a
bad partial download.

The helper copies fixed package versions. Update the crate rather than replacing staged files by
hand so the manifest, runtime allow-list, and WebView2 projection stay compatible.

---

## Internal documentation

This section is for contributors to `windows-reactor-setup`.

`as_self_contained` validates `CARGO_CFG_TARGET_OS`, derives the target directory from the nearest
ancestor matching `PROFILE`, and falls back to the conventional ancestor depth. The unit tests
cover standard and split-package `OUT_DIR` layouts.

`stage_pkg` preserves the NuGet package directory layout under `temp/packages/<name>-<version>`.
An archive named `<name>.<version>.nupkg` in `temp` can seed an offline restore. Downloads and
extractions use private staging directories and publish complete directories by rename.
Unmarked extraction directories outside this layout are not reused. Within each runtime package,
MSIX extraction uses `.msix/<architecture>` so cross-compilation cannot reuse another target's
binaries.

`copy_runtime_to` requires every selected entry in `assets/runtime.txt`, then recursively copies
selected directories. A bare filename applies to all supported architectures; optional whitespace-
separated architecture names restrict an entry. `Microsoft.Graphics.Imaging.dll` and
`SessionHandleIPCProxyStub.dll` are selected for x64 and arm64 only. The x86 package does not contain
them. `src/runtime.rs` implements this policy and is also compiled into the metadata tool.

`tool-reactor-metadata` owns the Windows App SDK pin. It resolves the Runtime dependency from the
umbrella package's nuspec and writes `assets/runtime-version.txt`, which both this crate and Reactor
CI use. It also reads the Runtime package's `include/WindowsAppSDK-VersionInfo.h` to generate
Reactor's bootstrap version constants and framework identity. It generates `runtime-package.txt`
for MSIX selection and `reactor/src/native/runtime.rs` for framework bootstrap. Do not edit these
generated files by hand.

The application manifests are generated as `assets/app.manifest` for x64/arm64 and
`assets/app-x86.manifest` for x86. The setup function selects the target manifest, inserts the
deployment marker after the opening assembly element, writes the result to `OUT_DIR`, and emits
binary-only manifest linker arguments for MSVC or LLVM GNU targets.

The metadata tool reads WinRT activation entries from the Foundation, InteractiveExperiences, and
WinUI `runtimes-framework/package.appxfragment` files. It reads COM proxy entries from each runtime
MSIX's `AppxManifest.xml`, selecting only DLLs in the runtime allow-list. The x86 manifest therefore
omits SessionHandle proxy registrations. The tool checks the allow-list against every supported
architecture's MSIX and requires each WinRT activation DLL to be staged, with WebView2's separately
staged projection as the exception. It also verifies that x64 and arm64 share the same registrations.
Review generated manifests and the handwritten allow-list together when updating the pin.

`deploy_webview2` copies `Microsoft.Web.WebView2.Core.dll` from the pinned WebView2 package's
per-architecture `native_uap` directory. `tool-webview` generates `assets/webview2-version.txt` from
the same pin as its COM bindings and Core WinRT metadata. Do not edit this version file by hand.
