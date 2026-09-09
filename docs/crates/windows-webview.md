# windows-webview

> A safe Rust wrapper around the [WebView2](https://aka.ms/webview2) COM APIs for hosting
> Microsoft Edge in a window.

- 📦 [crates.io](https://crates.io/crates/windows-webview)
- 📖 [docs.rs](https://docs.rs/windows-webview)
- 🚀 [Getting started](../../crates/libs/webview/readme.md)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/webview)

## When to use it

Use `windows-webview` when a Windows desktop application needs to host web content, exchange
messages with JavaScript, or use browser facilities such as profiles, cookies, downloads, and the
Chrome DevTools Protocol. The crate has no default UI framework. Enable `system` to host WebView2
through `windows-window`, or `reactor` to place the WinUI XAML WebView2 control in a
[`windows-reactor`](windows-reactor.md) view. The features are additive.

The crate wraps a selected WebView2 surface rather than exposing the complete SDK. Use raw WebView2
bindings when an application needs APIs that are not represented here.

## UI operation model

Low-level WebView2 creation and script operations complete through callbacks. Starting an
operation returns `Result<()>`: an error means the operation did not start, while success means its
result may later be passed to the callback on the same UI thread. A callback runs at most once and
may be dropped without running if its native operation or UI apartment ends first.

No operation runs a nested message loop. Win32, WinUI, Reactor, and other UI hosts retain control
of message preprocessing, dispatch, and shutdown. `WebViewWindowBuilder::run` is a separate
convenience for applications that choose `windows-window` as their top-level host; it owns the one
application message loop.

This differs from [`windows-pickers`](windows-pickers.md): a Common Item Dialog is a synchronous
native modal operation whose `Show` method owns its modal loop. Reactor stages such modal work with
`ComponentContext::run_window`; it uses `ElementRef` requests for callback-completed control work.

## The basic idea

A low-level WebView host has three main objects:

| Type | Purpose |
| --- | --- |
| `Environment` | Owns the browser process and user-data context |
| `Controller` | Places a browser inside a native parent window |
| `WebView` | Navigates, runs scripts, exchanges messages, and exposes page events |
| `WebViewHost` | Owns all three objects while another framework owns the parent |
| `WebViewWindow` | Adds an owned `windows-window` parent and lifecycle handling |

`WebViewHostBuilder` creates the first three as one callback-completed operation. Use the individual
types when an application shares an environment or needs a custom creation sequence.

The Microsoft Edge WebView2 runtime must be installed for native window hosting. Low-level hosts
need a live message loop on a COM single-threaded apartment. Standalone applications initialize
one with `windows_core::init_sta()`. `WebViewWindowBuilder::run` handles this for its owned loop.

## Host your first page

The `system` feature provides a complete `windows-window` host:

```rust,no_run
use windows_webview::*;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2")
        .size(1000, 700)
        .run(|host| {
            host.webview()
                .navigate("https://learn.microsoft.com/windows/apps/")?;
            Ok(())
        })
}
```

The host resizes the controller with its parent and closes WebView2 before the HWND is destroyed.
Call `host.retain` or `host.retain_all` for event registrations that should remain active for the
host lifetime. `WebViewWindowBuilder::on_close` reports normal close processing, including
programmatic closure, when another UI framework coordinates several windows. Direct destruction
during drop does not call it.

Use `WebViewHostBuilder` when a framework already owns the parent HWND and UI loop:

```rust,ignore
WebViewHost::builder().create_for_hwnd(parent, move |result| {
    match result {
        Ok(host) => framework.attach_webview(host),
        Err(error) => framework.report_webview_error(error),
    }
})?;
```

The builder chains environment and controller creation without blocking or pumping messages.
`WebViewHost` owns the resulting environment, controller, browser, and retained event
registrations. The framework stores that one value and remains responsible for parent lifetime,
layout, position notifications, and shutdown. The raw `Environment` and `Controller` callback APIs
remain available when a framework needs separate environment reuse or custom controller creation.

## Keep event registrations alive

Every `on_*` method returns an `EventRegistration`. The registration unsubscribes when it is
dropped, so store it for as long as the callback should run:

```rust,ignore
let navigation = webview.on_navigation_completed(|args| {
    println!("navigation succeeded: {}", args.is_success());
})?;

webview.navigate("https://github.com/microsoft/windows-rs")?;

// Keep `navigation` alive while the message loop runs.
```

For several events, a vector is convenient:

```rust,ignore
let registrations = vec![
    webview.on_document_title_changed(|title| {
        println!("the title changed to {title}");
    })?,
    webview.on_process_failed(|args| {
        eprintln!("browser process failed: {:?}", args.kind());
    })?,
];
```

Callbacks run on the UI thread. Keep them short so the window and page remain responsive.

## Navigate and observe page state

The common navigation methods are direct:

```rust,ignore
webview.navigate("https://example.com")?;
webview.reload()?;
webview.go_back()?;
webview.go_forward()?;
```

Use `source` and `document_title` after navigation:

```rust,ignore
let page = webview.clone();
let navigation = webview.on_navigation_completed(move |args| {
    if args.is_success() {
        println!("{} - {}", page.document_title(), page.source());
    }
})?;
```

`on_navigation_starting` can inspect or cancel a request before it begins:

```rust,ignore
let starting = webview.on_navigation_starting(|args| {
    if !args.uri().starts_with("https://") {
        _ = args.set_cancel(true);
    }
})?;
```

Keep a process-failure handler in a long-running host. A failed render process can usually be
followed by `reload`; a failed browser process requires a new WebView.

## Exchange messages with the page

Page JavaScript sends a value to Rust with:

```javascript
window.chrome.webview.postMessage({ action: "save", value: 42 });
```

The host receives it through `on_web_message_received`:

```rust,ignore
let messages = webview.on_web_message_received(|args| {
    println!("{} sent {}", args.source(), args.web_message_as_json());
})?;
```

Inspect `source` before trusting messages from content that can navigate. Use
`try_web_message_as_string` when the protocol accepts only strings.

Rust sends a value in the other direction with:

```rust,ignore
webview.post_web_message_as_json(r#"{"status":"saved"}"#)?;
webview.post_web_message_as_string("refresh")?;
```

The page receives those values with:

```javascript
window.chrome.webview.addEventListener("message", event => {
    console.log("host sent", event.data);
});
```

Use `execute_script` when Rust needs to run a specific expression:

```rust,ignore
webview.execute_script("document.title", |result| {
    println!("title as JSON: {result:?}");
})?;
```

Script results are JSON encoded. The callback runs later on the UI thread.

## Add script before each document loads

`add_script_to_execute_on_document_created` installs script before page JavaScript runs and
returns its identifier through a callback:

```rust,ignore
let page = webview.clone();
webview.add_script_to_execute_on_document_created(
    "document.documentElement.dataset.host = 'windows-rs';",
    move |result| {
        let script = result.unwrap();
        println!("registered {}", script.as_str());
        page.navigate("https://example.com").unwrap();
    },
)?;
```

Save the returned `ScriptId` if the script may need to be removed later:

```rust,ignore
webview.remove_script_to_execute_on_document_created(&saved_script)?;
```

Wait for successful registration before navigating when the script must apply to the first
document.

## Host local files

Map a folder to a virtual HTTPS host rather than navigating to a `file:` URL:

```rust,ignore
let folder = concat!(env!("CARGO_MANIFEST_DIR"), "\\web");
webview.set_virtual_host_name_to_folder_mapping(
    "app.example",
    folder,
    HostResourceAccessKind::Deny,
)?;
webview.navigate("https://app.example/index.html")?;
```

The page can now load scripts, styles, and images relative to that origin. The access kind controls
whether other origins may request the folder's content.

For generated or embedded content, intercept a URL pattern and return a response:

```rust,ignore
let resources = webview.on_web_resource_requested("https://app.example/*", |request| {
    if request.uri().ends_with("/style.css") {
        Some(
            WebResourceResponse::new("body { font-family: Segoe UI, sans-serif; }")
                .content_type("text/css"),
        )
    } else {
        None
    }
})?;
```

Returning `None` lets WebView2 continue the request normally. The callback is synchronous, so
prepare expensive content outside it.

## Put a WebView inside Reactor

Enable the `reactor` feature when the browser should participate in a Reactor layout:

```toml
windows-webview = { version = "0.100.0", default-features = false, features = ["reactor"] }
```

`webview` returns a normal Reactor `View` and sends the initialized browser through a callback:

```rust,ignore
use windows_reactor::*;
use windows_webview::{EventRegistration, WebView, webview};

struct Browser {
    webview: Option<WebView>,
    navigation: Option<EventRegistration>,
}

#[derive(Clone)]
enum Message {
    Initialized(WebView),
}

fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
    let Message::Initialized(webview) = message;
    self.navigation = webview.on_navigation_completed(|_| {}).ok();
    _ = webview.navigate("https://learn.microsoft.com/windows/apps/");
    self.webview = Some(webview);
}

fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
    webview(context.callback(Message::Initialized))
}
```

These methods belong inside the corresponding `Component` implementation. Initialization happens
after the XAML control enters a live visual tree, not during `Component::create`.

The convenience function panics on a native initialization error. Use `webview_result` when the
component should receive and display that error. A self-contained Reactor app must also deploy
`Microsoft.Web.WebView2.Core.dll`; [`windows-reactor-setup`](windows-reactor-setup.md) stages it.
The [`reactor`](../../crates/samples/webview/reactor) sample embeds a WinUI WebView2 control in a
Reactor view and exercises navigation, scripts, messaging, DevTools, and events through the shared
`WebView` API. The [`reactor-window`](../../crates/samples/webview/reactor-window) sample instead
uses a Reactor window as a toolbar for a callback-driven `WebViewWindow`. It sets
`quit_on_close(false)` so both windows share Reactor's UI thread and message loop, and uses
`on_close` to keep the Reactor component synchronized with the external window. This sample also
exercises controller zoom and parent-window lifecycle, which the WinUI WebView2 control does not
expose through supported interop.

## What to read next

Run an example with `cargo run -p webview-<name>`.

| Example | What it shows |
| --- | --- |
| [`minimal`](../../crates/samples/webview/minimal) | Complete `WebViewWindow` hosting |
| [`raw-window`](../../crates/samples/webview/raw-window) | Framework-owned HWND and message loop |
| [`reactor`](../../crates/samples/webview/reactor) | WebView2 embedded in a Reactor view |
| [`reactor-window`](../../crates/samples/webview/reactor-window) | Reactor controls driving a separate `windows-window` host |
| [`events`](../../crates/samples/webview/events) | Navigation, permissions, popups, and process failures |
| [`ipc`](../../crates/samples/webview/ipc) | Messages and script execution |
| [`local-files`](../../crates/samples/webview/local-files) | A folder mapped to an HTTPS origin |
| [`custom-protocol`](../../crates/samples/webview/custom-protocol) | HTML and CSS served from memory |
| [`downloads`](../../crates/samples/webview/downloads) | Download progress and cancellation |
| [`cookies`](../../crates/samples/webview/cookies) | Creating and enumerating cookies |
| [`profile`](../../crates/samples/webview/profile) | Private mode and browsing-data cleanup |
| [`devtools`](../../crates/samples/webview/devtools) | Chrome DevTools Protocol calls and events |
| [`script`](../../crates/samples/webview/script) | Document-created script injection |

Start with `minimal`, `raw-window` when another framework owns the HWND, or `reactor` when Reactor
owns the UI. The remaining samples use the complete system host so they can focus on individual
browser features.

---

## Internal documentation

This section is for contributors to `windows-webview`.

The `system` feature enables the typed `windows-window` controller helpers. The `reactor` feature
depends on `windows-reactor` and bridges its WinUI `WebView2` control to the shared browser API.
Raw HWND controller methods remain available without either host feature.

### Binding generation

WebView2 ships C/C++ headers rather than Windows metadata. `tool-webview` builds the committed
bindings in three stages:

| Stage | Implementation | Output |
| --- | --- | --- |
| Headers -> RDL | `windows_clang::clang()` | `target/webview/WebView2.rdl` |
| RDL -> winmd | `windows_rdl::reader()` | `target/webview/WebView2.winmd` |
| winmd -> Rust | `windows_bindgen` | `crates/libs/webview/src/bindings.rs` |

The tool downloads the pinned `Microsoft.Web.WebView2` NuGet package. It parses `WebView2.h` and
`WebView2Interop.h` as separate inputs because the collector emits declarations owned by each
input, then merges both translation units. It uses `Windows.Win32.winmd` for referenced Win32
types and targets `x86_64-pc-windows-msvc` with Microsoft extensions. Regenerate with
`cargo run -p tool-webview`; never edit `src/bindings.rs`.

Bindings use `--flat --minimal` and the filter in `crates/tools/webview/src/webview.txt`. Filter
method names are raw metadata names such as `put_Bounds` and `get_CoreWebView2`, not projected
names. Implemented interfaces belong in `--implement`, without method filters. The `--dead-code`
option keeps interface methods crate-private. The small Win32 filter supplies COM string allocation
and memory-stream support without a dependency on the full `windows` crate.

### Wrapper implementation

Completion handlers and event adapters in `handler.rs` use `implement_decl!`, avoiding the
`windows-core` proc-macro dependencies. `OptionsObject` uses the same mechanism to implement the
caller-provided environment options interfaces. Its string getters allocate with the COM task
allocator because WebView2 takes ownership.

All WebView2 completion operations use the same callback contract. Their public methods report
immediate initiation failures and retain one-shot handlers for native completion. No operation
owns or nests the host's message loop.

Event adapters convert COM add/remove tokens into `EventRegistration`. Resource interception also
removes its request filter when the registration drops. `protocol.rs` converts response bytes to
an `IStream` with `SHCreateMemStream`, keeping streams out of the public API.

`string.rs` handles borrowed UTF-16 input, borrowed WebView2 callback strings, owned `LPWSTR`
results that must be freed with `CoTaskMemFree`, and task-allocator output strings returned by
implemented interfaces.

### Reactor bridge

The WinUI XAML control exposes a WinRT `CoreWebView2`, while this crate wraps the COM
`ICoreWebView2`. `ICoreWebView2Interop2::GetComICoreWebView2` is the supported bridge; the two
objects do not convert through a plain interface cast.

With the `reactor` feature, `src/reactor.rs` requests initialization after the control is loaded,
retains the loaded-event and asynchronous-operation state through Reactor, crosses the interop
bridge, and reports one result. The canonical WinRT metadata is
`winmd/Microsoft.Web.WebView2.Core.winmd`. Reactor setup supplies the matching projection DLL for
self-contained deployment.

### Maintenance

Generated bindings carry casing lint expectations and allow dead code only when the Reactor bridge
is disabled. Keep those expectations synchronized with generator output.

`test-webview` runs the raw host and browser feature fixtures against a live WebView2 runtime.
`test-reactor-selftest` covers WinUI control initialization, the COM bridge, navigation, and script
execution. Both tests run with real windows and message pumps; `--headless` suppresses interactive
UI rather than replacing the native hosts.
