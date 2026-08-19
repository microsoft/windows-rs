# windows-reactor-setup

> A build-script helper that stages the Windows App SDK runtime for reactor apps.

- [crates.io](https://crates.io/crates/windows-reactor-setup)
- [docs.rs](https://docs.rs/windows-reactor-setup)
- [Getting started](../../crates/libs/reactor-setup/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor-setup)

`windows-reactor-setup` is used from the `build.rs` of a
[`windows-reactor`](windows-reactor.md) application. It downloads and stages the Windows App SDK
runtime bootstrap files next to the built executable so the app can start WinUI 3. The
self-contained helper also writes the application manifest. Choose the helper that matches your
deployment model - for example a framework-dependent app or a self-contained one.

Call `windows_reactor_setup::as_self_contained()` or
`windows_reactor_setup::as_framework_dependent()` from `build.rs`. A framework-dependent
application also calls `windows_reactor::bootstrap()` at startup. A self-contained application
does not.

See the [samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
for complete project layouts.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-reactor-setup`**.

### How it's built

A small build-script helper crate that drives the Windows App Runtime installer/bootstrapper.

`as_self_contained()` also stages `Microsoft.Web.WebView2.Core.dll` from the
`Microsoft.Web.WebView2` NuGet package and copies the per-architecture `native_uap` build next to
the executable. The XAML `WebView2` control used by `windows-reactor`'s `webview` feature loads
that WinRT projection assembly at runtime. Unlike the COM-only `webview2loader.dll` supplied by
the Evergreen runtime, it is not present on the machine by default. Bundling it unconditionally
keeps Reactor apps that host a WebView2 working with no extra build step. The allow-list of
WindowsAppSDK runtime files lives in `assets/runtime.txt`.
The output directory is found by locating Cargo's enclosing `build` directory rather than assuming
a fixed `OUT_DIR` depth, since Cargo may include the package name as an extra path component.

The framework-dependent bootstrap DLLs under `bootstrap/<arch>/` match the runtime version pinned
by `RUNTIME_VER`. See [dependencies](../dependencies.md#winui--windows-app-sdk).

### Testing

Run `cargo test -p windows-reactor-setup`. The unit test stages the files for x86, x64, and arm64
and compares them byte-for-byte with the committed assets.
