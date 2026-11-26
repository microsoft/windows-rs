# windows-webview

Show web content in a Windows app with Microsoft Edge WebView2, and exchange messages between Rust
and JavaScript. The WebView2 runtime ships with Windows 11 and is installed on most Windows 10
machines.

- 📦 [crates.io](https://crates.io/crates/windows-webview)
- 📖 [docs.rs](https://docs.rs/windows-webview)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/webview)

## Open a page

```toml
[dependencies]
windows-webview = "0.101"
```

```rust,no_run
use windows_webview::*;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2")
        .size(1000, 700)
        .run(|host| host.webview().navigate("https://github.com/microsoft/windows-rs"))
}
```

`run` initializes a COM STA, creates the window and browser, and runs until the window closes.
It cannot run on a thread already initialized as an MTA.

Event methods return an `EventRegistration` that unsubscribes when dropped; `host.retain` keeps it
for the life of the window. Treat page messages as untrusted input.

## Use another UI framework

With Reactor, enable the `reactor` feature and add `webview(...)` to a view. Its callback receives
the `WebView` once it is ready. Self-contained Reactor apps also need
[windows-reactor-setup](https://docs.rs/windows-reactor-setup).

For a window you create yourself, use `WebViewHost::builder` with its `HWND`. Your code then owns
the message loop and resizing, and closes the host before destroying the window.
The thread must be initialized as a COM STA. Drop hosts, event registrations, and WebView objects
before uninitializing COM.

See the [raw window][raw] sample for a complete HWND host or the [Reactor browser][reactor] for a
browser built with controls.

[raw]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview/raw-window
[reactor]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview/reactor
