## Windows WebView2

Windows WebView2 is a safe Rust wrapper around the [WebView2](https://aka.ms/webview2) COM APIs. It hosts the Microsoft Edge (Chromium) browser inside a window so you can render web content and run JavaScript from Rust.

> **Status:** experimental and pre-release (`0.0.0`). The API is evolving and the crate is not yet published to crates.io. It currently covers environment and controller creation, navigation, and script execution; more of the WebView2 surface will follow.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [WebView2 guide](https://github.com/microsoft/windows-rs/blob/master/docs/guide/windows-webview.md)

WebView2 is callback driven: creation and scripting complete asynchronously on the UI thread, so the wrapper takes Rust closures that run when each operation finishes. Hosting requires a window and a running message loop.

```rust,no_run
use windows_webview::*;

fn host(window: HWND) -> Result<()> {
    create_environment(move |environment| {
        let environment = environment.unwrap();

        unsafe {
            environment
                .create_controller(window, move |controller| {
                    let controller = controller.unwrap();
                    let webview = controller.webview().unwrap();
                    webview
                        .navigate("https://github.com/microsoft/windows-rs")
                        .unwrap();
                })
                .unwrap();
        }
    })
}
```

Key types: `Environment`, `Controller`, and `WebView`. The browser is hosted for as long as you keep the `Controller` alive. See the [WebView2 guide](https://github.com/microsoft/windows-rs/blob/master/docs/guide/windows-webview.md) for how this layer fits alongside reactor and canvas.
