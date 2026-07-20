## Windows WebView2

The [windows-webview](https://crates.io/crates/windows-webview) crate is a safe Rust wrapper around the [WebView2](https://aka.ms/webview2) COM APIs. It hosts the Microsoft Edge (Chromium) browser inside a window so you can render web content and run JavaScript from Rust.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [WebView2 guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-webview.md)

WebView2 hosting requires a window and a running message loop. Environment and controller creation complete asynchronously on the UI thread, so the wrapper pumps the message loop for you and returns once each step is ready, letting you write the setup as straight-line code.

```rust,no_run
use windows_webview::*;
use windows_window::Window;

fn host(window: &Window) -> Result<()> {
    let environment = Environment::new()?;
    let controller = environment.create_controller(window)?;
    let webview = controller.webview()?;
    webview.navigate("https://github.com/microsoft/windows-rs")?;
    Ok(())
}
```

Key types: `Environment`, `Controller`, and `WebView`. The browser is hosted for as long as you keep the `Controller` alive. See the [WebView2 guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-webview.md) for how this layer fits alongside reactor and canvas.
