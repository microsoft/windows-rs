## windows-webview

Windows WebView wraps the [WebView2](https://aka.ms/webview2) COM APIs to host Microsoft Edge
(Chromium) in a window.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-webview.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows-webview = "0.100"
windows-window = "0.100"
```

WebView2 requires a window and a running message loop. Environment and controller creation complete
asynchronously on the UI thread. These constructors pump the message loop until each step finishes.

```rust,no_run
use windows_webview::*;
use windows_window::{Window, run};

fn main() -> Result<()> {
    let window = Window::new("WebView2").size(1000, 700).create()?;
    let environment = Environment::new()?;
    let controller = environment.create_controller(&window)?;
    let webview = controller.webview()?;

    let (width, height) = window.client_size();
    controller.set_bounds(0, 0, width, height)?;
    webview.navigate("https://github.com/microsoft/windows-rs")?;

    run();
    Ok(())
}
```

Keep the `Controller` alive while the browser is hosted.
