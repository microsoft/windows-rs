## Windows WebView2

Windows WebView wraps the [WebView2](https://aka.ms/webview2) COM APIs to host Microsoft Edge
(Chromium) in a window.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [WebView2
  guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-webview.md)

WebView2 requires a window and a running message loop. Environment and controller creation complete
asynchronously on the UI thread. These constructors pump the message loop until each step finishes.

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

Keep the `Controller` alive while the browser is hosted. See the [WebView2
guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-webview.md) for the
API and hosting options.
