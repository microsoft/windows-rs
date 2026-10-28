# windows-reactor-setup

Stage a private Windows App Runtime beside a Reactor executable. Use this build dependency only
for self-contained deployment; framework-dependent applications use an installed runtime.

- 📦 [crates.io](https://crates.io/crates/windows-reactor-setup)
- 📖 [docs.rs](https://docs.rs/windows-reactor-setup)
- 🧩 [Self-contained sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/self_contained)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor-setup)

## Set up the build

```toml
[build-dependencies]
windows-reactor-setup = "0.100"
```

Create `build.rs`:

```rust,no_run
use windows_reactor_setup::as_self_contained;

fn main() {
    as_self_contained();
}
```

Build normally with Cargo. The helper stages runtime files in the Cargo profile output directory
and embeds the deployment manifest. A staging failure stops the build.

The first build needs network access and Windows `curl.exe` and `tar.exe`. Use a supported Windows
target with the MSVC linker or supported LLVM GNU linker. Downloads are cached for later builds.

## Package the application

Distribute the executable with the staged DLLs and runtime subdirectories, preserving their
relative layout. Copying only the `.exe` is not a self-contained deployment. Use a clean build
output and try the packaged directory on a machine without the matching framework runtime.

The helper also stages the WebView2 projection DLL used by Reactor's XAML WebView2 control.
This does not replace the WebView2 browser runtime. A plain
[windows-webview](https://docs.rs/windows-webview) HWND host does not need this helper.

Keep runtime files matched to the crate instead of replacing individual staged DLLs by hand.
`cargo clean` removes build output, not the package cache under `%LOCALAPPDATA%`.
