# windows-reactor-setup

> A build-script helper that stages the Windows App SDK runtime for reactor apps.

- 📦 Not published to crates.io
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor-setup)

`windows-reactor-setup` is used from the `build.rs` of a
[`windows-reactor`](windows-reactor.md) application. It downloads and stages the
Windows App SDK runtime bootstrap files next to the built executable so the app
can start WinUI 3, and writes the required application manifest. Choose the helper
that matches your deployment model — for example a framework-dependent app or a
self-contained one.

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

A small unpublished helper crate that drives the Windows App Runtime
installer/bootstrapper.

`as_self_contained()` also stages `Microsoft.Web.WebView2.Core.dll` from the
`Microsoft.Web.WebView2` NuGet package and copies the per-architecture
`native_uap` build next to the executable. The XAML `WebView2` control used by
[`windows-webview`](windows-webview.md)'s `reactor` feature loads that WinRT
projection assembly at runtime, and — unlike the COM-only `webview2loader.dll`
supplied by the Evergreen runtime — it is not present on the machine by default.
Bundling it unconditionally keeps reactor apps that host a WebView2 working with
no extra build step. The allow-list of WindowsAppSDK runtime files lives in
`assets/runtime.txt`.

### Testing

Run `cargo test -p windows-reactor-setup`; see also the workspace test crates.
