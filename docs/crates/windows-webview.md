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
Chrome DevTools Protocol. The default API hosts WebView2 in an `HWND`. Enable the `reactor` feature
to place the WinUI XAML WebView2 control in a
[`windows-reactor`](windows-reactor.md) view.

The crate wraps a selected WebView2 surface rather than exposing the complete SDK. Use raw WebView2
bindings when an application needs APIs that are not represented here.

## The basic idea

A WebView host has three main objects:

| Type | Purpose |
| --- | --- |
| `Environment` | Owns the browser process and user-data context |
| `Controller` | Places a browser inside a native parent window |
| `WebView` | Navigates, runs scripts, exchanges messages, and exposes page events |

Create them in that order, navigate the `WebView`, and keep the `Controller` alive while the page is
hosted.

The Microsoft Edge WebView2 runtime must be installed for native window hosting. The host also
needs a live message loop on a single-threaded apartment. `Environment::new` initializes the
calling thread as an STA when needed, as does `Environment::with_options`.

## Host your first page

[`windows-window`](windows-window.md) provides a small parent window and message loop:

```rust,ignore
use windows_webview::*;
use windows_window::{Window, run};

fn main() -> Result<()> {
    let window = Window::new("WebView2")
        .size(1000, 700)
        .create()?;

    let environment = Environment::new()?;
    let controller = environment.create_controller(&window)?;
    let webview = controller.webview()?;

    let (width, height) = window.client_size();
    controller.set_bounds(0, 0, width, height)?;
    webview.navigate("https://learn.microsoft.com/windows/apps/")?;

    run();
    Ok(())
}
```

The controller bounds use parent-client pixels. Update them whenever the parent window changes
size. The [`minimal`](../../crates/samples/webview/samples/examples/minimal.rs) example uses the
shared sample host, which includes resize and shutdown handling.

Environment and controller creation start asynchronous WebView2 operations, but these constructors
wait for completion while pumping the UI thread. Create them during setup before entering the
application's own message loop.

Keep the parent window alive longer than its controller. When the application controls shutdown
order, call `controller.close()` before destroying the parent window.

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

`add_script_to_execute_on_document_created` installs script before page JavaScript runs:

```rust,ignore
let script = webview.add_script_to_execute_on_document_created(
    "document.documentElement.dataset.host = 'windows-rs';",
)?;
```

Keep the returned `ScriptId` if the script may need to be removed:

```rust,ignore
webview.remove_script_to_execute_on_document_created(&script)?;
```

Like environment creation, adding this script waits for an asynchronous WebView2 operation while
pumping the UI thread. Install it during setup.

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
windows-webview = { version = "0.100.0", features = ["reactor"] }
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
The [`reactor/webview`](../../crates/samples/reactor/webview) sample contains the complete
component and deployment layout.

## What to read next

Run an example with `cargo run -p webview_samples --example <name>`.

| Example | What it shows |
| --- | --- |
| `minimal` | Window hosting, resize, and navigation |
| `events` | Navigation, permissions, popups, and process failures |
| `ipc` | Messages and script execution |
| `local_files` | A folder mapped to an HTTPS origin |
| `custom_protocol` | HTML and CSS served from memory |
| `downloads` | Download progress and cancellation |
| `cookies` | Creating and enumerating cookies |
| `profile` | Private mode and browsing-data cleanup |
| `devtools` | Chrome DevTools Protocol calls and events |
| [`reactor/webview`](../../crates/samples/reactor/webview) | Hosting WebView2 in Reactor |

Start with `minimal`, then `ipc` or `local_files`. Profiles, downloads, cookies, and DevTools are
independent workflows that can wait until the basic host lifecycle is familiar.

---

## Internal documentation

This section is for contributors to `windows-webview`.

### Binding generation

WebView2 ships C/C++ headers rather than Windows metadata. `tool_webview` builds the committed
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
`cargo run -p tool_webview`; never edit `src/bindings.rs`.

Bindings use `--flat --minimal` and the filter in `crates/tools/webview/src/webview.txt`. Filter
method names are raw metadata names such as `put_Bounds` and `get_CoreWebView2`, not projected
names. Implemented interfaces belong in `--implement`, without method filters. The `--dead-code`
option keeps interface methods crate-private. The small Win32 filter supplies message pumping,
COM string allocation, and memory-stream support without a dependency on the full `windows` crate.

### Wrapper implementation

Completion handlers and event adapters in `handler.rs` use `implement_decl!`, avoiding the
`windows-core` proc-macro dependencies. `OptionsObject` uses the same mechanism to implement the
caller-provided environment options interfaces. Its string getters allocate with the COM task
allocator because WebView2 takes ownership.

`pump.rs` stores a one-shot `Result<T>` in an `Rc<Cell<_>>` and dispatches messages until the
completion callback fills it. This is valid because creation and completion stay on one STA
thread. Runtime operations remain callback-based to avoid nested message pumping.

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

There is no headless integration suite because WebView2 requires a runtime, window, and message
pump. The example applications are the end-to-end coverage for hosting and feature workflows.
