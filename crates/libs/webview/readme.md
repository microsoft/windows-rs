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
windows-webview = "0.100"
```

```rust,no_run
use windows_webview::*;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2")
        .size(1000, 700)
        .run(|host| host.webview().navigate("https://github.com/microsoft/windows-rs"))
}
```

`run` creates the window and browser, then runs until the window closes.

## Showcase

Clone the repository and run this sample from its root.

### Browser

A small browser built with Reactor and WebView2.

```sh
cargo run -p webview-reactor
```

![Browser built with Reactor and WebView2](https://raw.githubusercontent.com/microsoft/windows-rs/master/docs/webview-reactor.png)

## Talk to the page

```rust,no_run
use windows_webview::*;

const PAGE: &str = r#"
<button onclick="chrome.webview.postMessage('ping')">Send</button>
<script>
  chrome.webview.addEventListener('message', e => document.body.append(e.data));
</script>"#;

fn main() -> Result<()> {
    WebViewWindow::new("Messages").run(|host| {
        let webview = host.webview().clone();
        host.retain(host.webview().on_web_message_received(move |args| {
            let reply = format!(" host received {}", args.web_message_as_json());
            webview.post_web_message_as_string(&reply).unwrap();
        })?);
        host.webview().navigate_to_string(PAGE)
    })
}
```

Event methods return an `EventRegistration` that unsubscribes when dropped; `host.retain` keeps it
for the life of the window. Treat page messages as untrusted input.

| To | Use |
| --- | --- |
| React to navigation | `on_navigation_completed` |
| Run JavaScript | `execute_script`, which passes the JSON result to a callback |
| Serve local files | `set_virtual_host_name_to_folder_mapping` |
| Recover from a browser crash | `on_process_failed` |

## Use another UI framework

With Reactor, enable the `reactor` feature and add `webview(...)` to a view. Its callback receives
the `WebView` once it is ready. Self-contained Reactor apps also need
[windows-reactor-setup](https://docs.rs/windows-reactor-setup).

For a window you create yourself, use `WebViewHost::builder` with its `HWND`. Your code then owns
the message loop and resizing, and closes the host before destroying the window.

See the [raw window][raw] sample for a complete HWND host.

[raw]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview/raw-window
