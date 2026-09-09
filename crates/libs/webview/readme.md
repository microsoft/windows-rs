## windows-webview

Windows WebView wraps the [WebView2](https://aka.ms/webview2) COM APIs to host Microsoft Edge
(Chromium) in a window.

The crate has no default UI framework. Enable `system` for a complete `windows-window` host or
`reactor` for a WebView2 control in a Reactor view. Enable both when an application uses both host
types.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-webview.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows-webview = { version = "0.100", features = ["system"] }
```

`WebViewWindowBuilder::run` owns the COM apartment, parent window, WebView2 environment and
controller, resize handling, shutdown order, and the application's `windows-window` message loop:

```rust,ignore
use windows_webview::*;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2")
        .size(1000, 700)
        .run(|host| {
            host.webview()
                .navigate("https://github.com/microsoft/windows-rs")?;
            Ok(())
        })
}
```

Use `WebViewHost::builder` when another framework owns the parent HWND and message loop. It chains
environment and controller creation into one callback and returns one host object for the
framework to retain. The [`raw-window`](../../crates/samples/webview/raw-window) sample shows the
raw HWND contract, while [`reactor/webview`](../../crates/samples/reactor/webview) shows the WinUI
integration path.
