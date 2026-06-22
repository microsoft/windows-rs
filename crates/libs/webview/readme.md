## Windows WebView2

Windows WebView2 is a safe Rust wrapper around the [WebView2](https://aka.ms/webview2) COM APIs. It hosts the Microsoft Edge (Chromium) browser inside a window so you can render web content and run JavaScript from Rust.

> **Status:** experimental and pre-release (`0.0.0`). The API is evolving and the crate is not yet published to crates.io. It currently covers environment creation (with options: user data folder, browser arguments, language) and controller creation, navigation and history (navigate, reload, stop, back/forward), document state (source URI, title), settings (script, dev tools, context menus, …), script execution and document-created scripts, events (navigation starting/completed, content loading, document title changed, web message received, window close requested, deferral-based new window and permission requests, and downloads with per-operation progress and control), custom protocols (intercepting requests to serve responses from memory), and host ↔ JavaScript messaging; more of the WebView2 surface will follow.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [WebView2 guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-webview.md)

WebView2 hosting requires a window and a running message loop. Environment and controller creation complete asynchronously on the UI thread, so the wrapper pumps the message loop for you and returns once each step is ready, letting you write the setup as straight-line code.

```rust,no_run
use windows_webview::*;

fn host(window: HWND) -> Result<()> {
    let environment = Environment::new()?;
    let controller = unsafe { environment.create_controller(window)? };
    let webview = controller.webview()?;
    webview.navigate("https://github.com/microsoft/windows-rs")?;
    Ok(())
}
```

Key types: `Environment`, `Controller`, and `WebView`. The browser is hosted for as long as you keep the `Controller` alive. See the [WebView2 guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-webview.md) for how this layer fits alongside reactor and canvas.
