# windows-webview

Host Microsoft Edge WebView2 content, run scripts, and exchange messages with JavaScript.
The Microsoft Edge WebView2 runtime must be available on the user's machine.

- 📦 [crates.io](https://crates.io/crates/windows-webview)
- 📖 [docs.rs](https://docs.rs/windows-webview)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/webview)

## Host a page

The crate has no default UI framework. Enable `system` for a complete window host:

```toml
[dependencies]
windows-webview = { version = "0.100", features = ["system"] }
```

```rust,no_run
use windows_webview::*;

# #[cfg(feature = "system")]
fn main() -> Result<()> {
    WebViewWindow::new("WebView2")
        .size(1000, 700)
        .run(|host| {
            host.webview().navigate("https://github.com/microsoft/windows-rs")?;
            Ok(())
        })
}
# #[cfg(not(feature = "system"))]
# fn main() {}
```

This host owns the UI loop, initializes COM, resizes the browser, and closes it before destroying
the parent window.

## Keep callbacks active

Event methods return an `EventRegistration` that unsubscribes on drop. Retain it in application
state or use `host.retain` for the host's lifetime.

Creation and script operations complete through callbacks on the UI thread. Their immediate
`Result` reports whether the operation started; the callback reports its result. Keep the message
loop running and callbacks short. A callback may be dropped if the operation or apartment ends.

| Task | API |
| --- | --- |
| Observe navigation | `on_navigation_completed` |
| Handle browser failures | `on_process_failed` |
| Receive page messages | `on_web_message_received` |
| Send page messages | `post_web_message_as_json` or `post_web_message_as_string` |
| Run JavaScript | `execute_script`, with a JSON-encoded result |
| Serve local files | `set_virtual_host_name_to_folder_mapping` |

Page code sends messages with `window.chrome.webview.postMessage(value)`. Validate the source and
payload before allowing a message to trigger native operations.

## Use an existing UI

Use `WebViewHost::builder` when another framework owns the HWND. That framework must provide a
COM STA, message loop, resize handling, and shutdown. Store the resulting host and close it before
destroying its parent.

For Reactor, enable `reactor` and use `webview` or `webview_result`; the latter reports
initialization errors instead of panicking. The host features are additive.
Self-contained apps also need [windows-reactor-setup](https://docs.rs/windows-reactor-setup), in
addition to the WebView2 browser runtime.

See the [raw window][raw] and [Reactor][reactor] samples for complete hosting and messaging.

[raw]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview/raw-window
[reactor]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview/reactor
