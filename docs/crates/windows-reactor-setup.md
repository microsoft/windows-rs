# windows-reactor-setup

> Build-time staging for self-contained Windows Reactor applications.

- 📦 [crates.io](https://crates.io/crates/windows-reactor-setup)
- 📖 [docs.rs](https://docs.rs/windows-reactor-setup)
- 🚀 [Getting started](../../crates/libs/reactor-setup/readme.md)
- 🧩 [Self-contained sample](../../crates/samples/reactor/self_contained)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor-setup)

## When to use it

Use `windows-reactor-setup` when a [`windows-reactor`](windows-reactor.md) executable must carry a
private Windows App SDK runtime. It is a build dependency called from `build.rs`, not a runtime
API. Framework-dependent applications and plain `windows-webview` `HWND` hosts do not need it.

## Make a self-contained build

```toml
[build-dependencies]
windows-reactor-setup = "0.100"
```

```rust,ignore
fn main() {
    windows_reactor_setup::as_self_contained();
}
```

Build normally with Cargo. The build step downloads the pinned Windows App SDK runtime and WebView2
NuGet packages, copies the runtime files beside the executable, and embeds a manifest that tells
Reactor to use them. To deploy, copy the executable together with every staged file and directory
from the Cargo profile output directory, preserving their layout.

The [`reactor/self_contained`](../../crates/samples/reactor/self_contained) sample is the reference
layout, and [`webview/reactor`](../../crates/samples/webview/reactor) adds the XAML WebView2 control.

## Prerequisites

- A Windows target using MSVC or the LLVM-based GNU ABI.
- Network access for the first build; packages are then cached under
  `%LOCALAPPDATA%\windows-reactor-setup`.
- `%SystemRoot%\System32\curl.exe` and `tar.exe`.

Staging errors panic and fail the build. `cargo clean` does not remove the package cache; delete it
manually to force a fresh download.

## Contributing

| Item | Location |
| --- | --- |
| Runtime file allow-list | `assets/runtime.txt`; keep in sync with the pinned runtime |
| Manifest template | `assets/app.manifest` |
| Package versions | Pinned in the crate; keep WebView2 compatible with `windows-webview` |

Cache entries are populated under a per-entry file lock in a `.partial` directory and renamed when
complete. Copying into the shared Cargo profile directory is serialized by a destination lock.
Tests use local ZIP fixtures and do not download packages or launch WinUI.
