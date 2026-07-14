# windows-reactor-setup

> A build-script helper that stages the Windows App SDK runtime for reactor apps.

- 📦 Not published to crates.io
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor-setup)

`windows-reactor-setup` runs in the `build.rs` of a
[`windows-reactor`](windows-reactor.md) application. At **build time** it stages
the files the app needs to locate the Windows App SDK runtime — the runtime WinUI
3 is built on — when it starts. The app itself activates that runtime at startup
through `windows-reactor`'s `bootstrap()`, which calls into the staged
`Microsoft.WindowsAppRuntime.Bootstrap.dll`; nothing in this crate runs at app
runtime.

Pick the helper that matches your deployment model:

- `as_framework_dependent()` / `as_example()` — the app relies on a
  machine-installed Windows App SDK runtime, so only the bootstrapper shim, the
  application manifest, and `resources.pri` are staged.
- `as_self_contained()` — the app carries its own copy of the runtime and runs
  with nothing pre-installed, so the full runtime is also downloaded and staged
  next to the executable.

```rust,ignore
// build.rs
fn main() {
    windows_reactor_setup::as_self_contained();
}
```

See the
[samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
for complete project layouts.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-reactor-setup`**.

### How it's built

Everything in this crate happens at **build time** — nothing runs when the app
executes. Its single responsibility is to make a reactor app able to find the
Windows App SDK runtime at startup; the two deployment models differ only in how
much of that runtime the app carries. Downloading and staging the runtime for the
self-contained model therefore lives here rather than in a separate "installer":
it is the implementation of `as_self_contained()`, has no other consumer, and it
shares the manifest and bootstrapper staging with the framework-dependent path.

It is intentionally **dependency-free** (standard library only) because it runs
in the `build.rs` of every consuming app; it shells out to the `curl.exe` and
`tar.exe` that ship with Windows rather than pulling in a download/unzip crate.
For the same reason it does **not** reuse `windows_clang::nuget_package` (the
downloader used by the metadata scrapers) — doing so would drag `windows-clang`,
`clang-sys`, and libclang provisioning into every reactor app's build just to
fetch a NuGet package.

The crate separates its two concerns into modules:

- `src/lib.rs` — the public API that configures an app for a deployment model
  (`as_framework_dependent`, `as_example`, `as_self_contained`): embedding the
  application manifest and copying the committed bootstrap DLL.
- `src/acquire.rs` — the self-contained model's build-time acquisition: downloading
  and extracting the pinned NuGet packages, unpacking the runtime MSIX, and staging
  the files listed in `assets/runtime.txt` next to the executable.

`as_self_contained()` also stages `Microsoft.Web.WebView2.Core.dll` from the
`Microsoft.Web.WebView2` NuGet package and copies the per-architecture
`native_uap` build next to the executable. The XAML `WebView2` control used by
[`windows-webview`](windows-webview.md)'s `reactor` feature loads that WinRT
projection assembly at runtime, and — unlike the COM-only `webview2loader.dll`
supplied by the Evergreen runtime — it is not present on the machine by default.
Bundling it unconditionally keeps reactor apps that host a WebView2 working with
no extra build step.

### Testing

Run `cargo test -p windows-reactor-setup`; see also the workspace test crates.
