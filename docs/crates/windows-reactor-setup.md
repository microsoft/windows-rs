# windows-reactor-setup

> A build-script helper that stages the Windows App SDK runtime for reactor apps.

- 📦 [crates.io](https://crates.io/crates/windows-reactor-setup)
- 📖 [docs.rs](https://docs.rs/windows-reactor-setup)
- 🚀 [Getting started](../../crates/libs/reactor-setup/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor-setup)

`windows-reactor-setup` is used from the `build.rs` of a [`windows-reactor`](windows-reactor.md)
application to configure a self-contained deployment: it stages a private copy of the Windows App
SDK runtime next to the executable and writes the application manifest.

Call `windows_reactor_setup::as_self_contained()` from `build.rs`. A framework-dependent app
does not depend on `windows-reactor-setup`; the bootstrap is inlined into `windows-reactor`,
which resolves the installed framework package at startup.

See the [samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
for complete project layouts.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-reactor-setup`**.

### How it's built

A small build-script helper crate that stages the Windows App Runtime.

`as_self_contained()` also stages `Microsoft.Web.WebView2.Core.dll` from the
`Microsoft.Web.WebView2` NuGet package and copies the per-architecture `native_uap` build next to
the executable. The XAML `WebView2` control used by [`windows-webview`](windows-webview.md)'s
`reactor` feature loads that WinRT projection assembly at runtime, and - unlike the COM-only
`webview2loader.dll` supplied by the Evergreen runtime - it is not present on the machine by
default. Bundling it unconditionally keeps reactor apps that host a WebView2 working with no extra
build step. The allow-list of WindowsAppSDK runtime files lives in `assets/runtime.txt`.

The self-contained manifest includes a deployment marker. Reactor uses the marker to choose
between the staged runtime and the inlined framework bootstrap, which keeps cached
framework-dependent and self-contained executables usable from one Cargo target directory.

### Testing

Run `cargo test -p windows-reactor-setup`. The unit tests check the Cargo `OUT_DIR` to target
directory resolution.
