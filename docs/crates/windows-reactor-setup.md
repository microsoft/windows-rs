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
- Supported target architecture mappings are `x86` -> `x86`, `aarch64` -> `arm64`, and other
  Rust target architectures -> `x64`.

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
project layout. The [`reactor/webview`](../../crates/samples/reactor/webview) sample shows the same
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

Downloads and extracted packages are cached under
`%LOCALAPPDATA%\windows-reactor-setup\temp` when `LOCALAPPDATA` is available. Cargo may rerun the
build script, but the cached package and extraction directories avoid downloading on every build.

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

`as_self_contained` has no `Result` return. Unsupported target configuration and missing Cargo
environment variables panic during the build. Download, extraction, and individual copy failures
are printed by the helper; inspect build output and the profile directory if a staged file is
missing.

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

`stage_pkg` caches `.nupkg` files and extracted package directories. Runtime MSIX extraction uses a
separate `.msix_extract` directory. `copy_runtime_to` copies only top-level entries named in
`assets/runtime.txt`, then recursively preserves any selected directory. Keep that allow-list in
sync with the pinned Windows App SDK runtime.

The application manifest template is `assets/app.manifest`. The function inserts the deployment
marker after the opening assembly element, writes the result to `OUT_DIR`, and emits binary-only
manifest linker arguments for MSVC or LLVM GNU targets.

`deploy_webview2` copies `Microsoft.Web.WebView2.Core.dll` from the pinned WebView2 package's
per-architecture `native_uap` directory. Keep its package version compatible with the WinRT
metadata and XAML WebView2 bridge used by `windows-webview` and Reactor.
