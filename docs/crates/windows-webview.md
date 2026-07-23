# windows-webview

> A safe Rust wrapper around the [WebView2](https://aka.ms/webview2) COM APIs for hosting the Microsoft Edge browser in a window.

- [crates.io](https://crates.io/crates/windows-webview)
- [docs.rs](https://docs.rs/windows-webview)
- [Getting started](../../crates/libs/webview/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/webview)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview)

`windows-webview` hosts WebView2 inside a Win32 window. Use it to render web
content and run JavaScript from Rust. The three core types are `Environment`,
`Controller`, and `WebView`. `Environment` owns the browser process and
user-data folder. `Controller` hosts the browser in a parent `HWND`. `WebView`
represents the page, including navigation, scripting, and events. The browser
lives while you keep the `Controller` alive.

WebView2 is callback-driven. Environment and controller creation complete on the
calling UI thread. The wrapper pumps the message loop for you, then returns when
each step is ready. Setup code can stay linear.

## Getting started

A WebView2 app needs the Microsoft Edge WebView2 runtime, a window, and a
message loop. `Environment::new` initializes the calling thread's COM apartment
as single-threaded (STA) if needed. You do not need to call `CoInitializeEx`.
It returns an error if the thread is already in a multi-threaded apartment. The
samples use [`windows-window`](windows-window.md) for the window and message
loop.

```rust,ignore
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

`Environment::with_options(&EnvironmentOptions::new()...)` configures the
user-data folder, browser arguments, and language. Resize the browser from your
window's `WM_SIZE` handler with `controller.set_bounds(..)`.

Use `create_controller_with_options` when you need a profile name, private mode,
or a background color before the first frame:
`create_controller_with_options(window, &ControllerOptions::new().profile_name("work").in_private_mode(true))`.

## Zoom, background, and DPI

`Controller` exposes the browser display properties:

- `zoom_factor()` / `set_zoom_factor(1.25)`: page zoom. `1.0` is 100%.
- `set_default_background_color(Color::TRANSPARENT)`: the color painted before
  content loads and where the page is transparent. `Color::TRANSPARENT` lets the
  host window show through. Only fully opaque or fully transparent colors are
  supported.
- `rasterization_scale()` / `set_rasterization_scale(..)` plus
  `set_should_detect_monitor_scale_changes(false)`: control the DPI scale
  yourself instead of following the monitor.

## Focus and keyboard

`Controller` manages focus and accelerator keys. These apply to the browser's
place in the host window's focus chain. Call
`controller.move_focus(MoveFocusReason::Programmatic)` from your `WM_SETFOCUS`
handler to give the browser keyboard focus. Focus events keep embedding correct
for keyboard and screen-reader users:

- `on_got_focus` / `on_lost_focus`: the browser gained or lost focus (`FnMut()`).
- `on_move_focus_requested`: focus is leaving the browser. Move focus to the
  next host control and call `MoveFocusRequestedArgs::set_handled(true)`.
- `on_accelerator_key_pressed`: browser-level keys arrive before the page sees
  them. Inspect `AcceleratorKeyPressedArgs` (`key_event_kind()`, `virtual_key()`).
  Call `set_handled(true)` to consume the key.

## Navigation and document state

`WebView` exposes the navigation verbs: `navigate(uri)`,
`navigate_to_string(html)`, `reload`, `stop`, `go_back`, and `go_forward`. The
getters `source()` and `document_title()` return owned strings.
`open_dev_tools_window()` opens the browser DevTools.

For a navigation with a custom HTTP method, request headers, or a body, build a
`NavigationRequest` and pass it to `navigate_with_request`:

```rust,ignore
webview.navigate_with_request(
    &NavigationRequest::new("https://api.example/data")
        .method("POST")
        .header("Authorization", "******")
        .body("{}"),
)?;
```

Call `controller.notify_parent_window_position_changed()` from your window's
`WM_MOVE` handler. This lets the browser reposition popups and dialogs that it
owns. `controller.set_allow_external_drop(false)` stops files from outside the
app being dropped onto the page. When a `WebView` is hidden, call
`webview.set_memory_usage_target_level(MemoryUsageTargetLevel::Low)` so the
browser can trim memory. Set it back to `Normal` when shown again.

## Events

Every event is an RAII subscription. An `on_*` method registers a handler and
returns an `EventRegistration`. It unsubscribes on drop or through `remove()`.
Keep it alive for as long as you need the handler.

- `on_navigation_starting` / `on_navigation_completed` / `on_content_loading`:
  navigation lifecycle. `NavigationStartingArgs::set_cancel(true)` vetoes a
  load.
- `on_document_title_changed`: receives the new title `String`.
- `on_window_close_requested`: the page called `window.close()`.
- `on_new_window_requested`: suppress, redirect, or `defer()` a popup.
- `on_permission_requested`: grant, deny, or `defer()` a capability request.
- `on_download_starting`: inspect and control downloads.
- `on_web_resource_requested`: serve responses from memory.
- `on_process_failed`: a browser process crashed, exited, or hung.
  `ProcessFailedArgs::kind()` separates a render-process crash from a
  browser-process exit. Reload the page for a render-process crash. Recreate the
  `WebView` for a browser-process exit. Without a handler, a crash leaves a
  blank page.
- `on_contains_fullscreen_element_changed`: receives a `bool` when the page
  enters or leaves HTML fullscreen. The host can resize or restore its window.

`NewWindowRequestedArgs` and `PermissionRequestedArgs` expose `defer()`, which
returns a `Deferral`. Use it when a handler must resolve the decision later. The
`Deferral` completes on drop, so a forgotten deferral cannot hang the page.

## Settings

`webview.settings()` returns a `Settings` value for browser feature toggles. It
has getter and `set_*` pairs such as `set_script_enabled`,
`set_dev_tools_enabled`, `set_default_context_menus_enabled`,
`set_status_bar_enabled`, and `set_zoom_control_enabled`.

Later WebView2 settings interfaces are exposed too. Each uses an internal
`cast`, so these methods return `Result`: `user_agent()` /
`set_user_agent(..)`, `set_browser_accelerator_keys_enabled`,
`set_general_autofill_enabled`, `set_password_autosave_enabled`,
`set_pinch_zoom_enabled`, `set_swipe_navigation_enabled`, and
`set_non_client_region_support_enabled`. The last one enables `app-region: drag`
for custom title bars.

## Host and JavaScript messaging

- Host to page: `post_web_message_as_json(json)` /
  `post_web_message_as_string(text)`. The page receives them with
  `window.chrome.webview.addEventListener("message", ...)`.
- Page to host: `on_web_message_received` delivers a `WebMessageReceivedArgs`
  with `source()`, `web_message_as_json()`, and `try_web_message_as_string()`.
  The page sends messages with `window.chrome.webview.postMessage(...)`.

`add_script_to_execute_on_document_created(js)` injects a bootstrap script. It
runs before page script on every new document. Use it to set up the host channel.
`execute_script(js, callback)` evaluates script and passes the JSON result to the
callback.

## DevTools protocol

For automation beyond page script, use the in-process
[Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/)
channel. It does not require a remote debugging port.

- Call a method: `call_dev_tools_protocol_method(method, params_json, callback)`
  sends a CDP command, such as `"Browser.getVersion"`. It takes parameters as a
  JSON object string. It delivers the result object as JSON to the callback.
- Subscribe to an event: `on_dev_tools_protocol_event(event_name, handler)`
  returns an `EventRegistration`. It delivers each event's parameters as JSON
  with `DevToolsProtocolEventReceivedArgs`. Most CDP events fire only after their
  domain is enabled, so call the matching `*.enable` method first. Each event
  name has its own receiver object. The registration keeps it alive.

## Custom protocols

`on_web_resource_requested(uri_filter, handler)` intercepts resource loads whose
URI matches a wildcard filter, such as `https://app.example/*`. The handler can
answer them from memory. This is useful for a self-contained app that serves its
bundled assets without a local server. The handler is
`FnMut(WebResourceRequest) -> Option<WebResourceResponse>` and runs on the UI
thread. Return `Some(response)` to handle the request. Return `None` to let
WebView2 handle it normally.

```rust,ignore
let registration = webview.on_web_resource_requested("https://app.example/*", |request| {
    println!("serving from memory: {}", request.uri());
    let html = "<!DOCTYPE html><html><body><h1>Served from Rust</h1></body></html>";
    Some(WebResourceResponse::new(html).content_type("text/html"))
})?;

webview.navigate("https://app.example/")?;
```

`WebResourceRequest` exposes `uri()`, `method()`, and `headers()`.
`WebResourceResponse::new(bytes)` defaults to `200 OK`. Chain
`.status(code, reason)`, `.header(name, value)`, and `.content_type(value)`. The
COM `IStream` body stays inside the crate.

## Serving local files

When assets live in a folder on disk, map a virtual host instead of intercepting
each request. `set_virtual_host_name_to_folder_mapping(host, folder, access)`
makes the files under `folder` load through an ordinary URL, such as
`https://app.example/index.html`. WebView2 supplies MIME types and no per-request
handler is needed. `HostResourceAccessKind` controls cross-origin access:
`Deny`, `Allow`, or `DenyCors`. Use
`clear_virtual_host_name_to_folder_mapping(host)` to remove the mapping.

```rust,ignore
webview.set_virtual_host_name_to_folder_mapping(
    "app.example",
    r"C:\path\to\assets",
    HostResourceAccessKind::Deny,
)?;
webview.navigate("https://app.example/index.html")?;
```

Use `on_web_resource_requested` when responses are generated in memory.

## Cookies

`webview.cookie_manager()` returns a `CookieManager`. Reading cookies is
asynchronous and callback-based, like `execute_script`:

```rust,ignore
let manager = webview.cookie_manager()?;
manager.get_cookies("https://example.com", |result| {
    if let Ok(cookies) = result {
        for cookie in cookies {
            println!("{} = {}", cookie.name, cookie.value);
        }
    }
})?;
```

Write with `add_or_update_cookie(&Cookie)`. Build one with
`Cookie::new(name, value, domain, path)` and set the public fields: `is_secure`,
`is_http_only`, `same_site`, and `expires`. Remove cookies with
`delete_cookies(name, uri)`, `delete_cookies_with_domain_and_path(..)`, or
`delete_all_cookies()`. The COM cookie and cookie-list types stay internal.

## Profiles

`webview.profile()` returns the `Profile` that owns the browser. It is the
on-disk identity shared by every `WebView` created with it. It exposes `name`,
`path`, and `is_in_private_mode`. Set
`preferred_color_scheme(PreferredColorScheme::Dark)` to drive a site's
`prefers-color-scheme` theme. Change `default_download_folder_path`, or clear
browsing data with the callback-based `clear_browsing_data_all`:

```rust,ignore
let profile = webview.profile()?;
profile.set_preferred_color_scheme(PreferredColorScheme::Dark)?;
profile.clear_browsing_data_all(|result| {
    let _ = result;
})?;
```

Choose the profile, private mode, and background color up front with
`ControllerOptions`. See [Getting started](#getting-started). Enable browser
extensions and pick a scrollbar style with `EnvironmentOptions`.

## Downloads

`on_download_starting` delivers a `DownloadStartingArgs`. Use it to inspect or
cancel the download, override its destination, or take its `DownloadOperation`.
`DownloadOperation` exposes progress getters such as `bytes_received`,
`total_bytes_to_receive`, `state`, and `interrupt_reason`. It also exposes
`cancel`, `pause`, and `resume`, plus `on_bytes_received_changed` and
`on_state_changed` subscriptions for progress updates.

## Samples

The [`crates/samples/webview/samples`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview/samples)
crate has one example per capability under `examples/`. A shared `run` helper in
`src/lib.rs` holds the hosting boilerplate: a [`windows-window`](windows-window.md)
window whose `on_resize` forwards to `Controller::set_bounds`, an `Environment`,
and a `Controller`. Each example can focus on its feature. The only dependencies
are `windows-webview` and `windows-window`. There is no hand-written Win32
message loop and no `windows` crate. All examples require the Microsoft Edge
WebView2 runtime.

Run one with `cargo run -p webview_samples --example <name>`:

| Example | Demonstrates |
| --- | --- |
| `minimal` | The smallest host: create a window, environment, and controller, then navigate. |
| `events` | The navigation lifecycle plus window-close, new-window, permission, and process-failed events. |
| `ipc` | Host and page messaging: an injected bootstrap script, received messages, replies, and `execute_script`. |
| `custom_protocol` | Serving an app from memory with `on_web_resource_requested`. |
| `local_files` | Serving a folder on disk through `set_virtual_host_name_to_folder_mapping`. |
| `downloads` | Watching downloads and reporting per-operation progress and state. |
| `cookies` | Adding a cookie and enumerating cookies with the cookie manager. |
| `profile` | An in-private controller, the dark color scheme, and clearing browsing data. |
| `script` | The document-created script lifecycle: inject a script that runs before each page, read a value it set back with `execute_script`, then remove it. |
| `devtools` | Driving the page over the Chrome DevTools Protocol: call `Browser.getVersion` and subscribe to `Runtime.consoleAPICalled`. |

## Reactor integration

With the optional `reactor` feature, `windows-webview` can host a browser inside
a [`windows-reactor`](windows-reactor.md) UI tree. The feature adds `webview`,
which returns a reactor `WebView2` control element. It hands you a ready
[`WebView`] when the browser initializes:

```rust,ignore
use windows_reactor::*;
use windows_webview::webview;

fn app(_cx: &mut RenderCx) -> Element {
    webview(|web| {
        web.navigate("https://learn.microsoft.com/windows/apps/").unwrap();
    })
    .into()
}

fn main() -> Result<()> {
    App::new().title("WebView2").render(app)
}
```

The `WebView` passed to the closure is the same type used for HWND hosting.
Navigation, scripting, IPC, cookies, settings, profiles, and the other wrappers
work the same way. Enable it with
`windows-webview = { version = "...", features = ["reactor"] }`.

The reactor path hosts the WinUI XAML `WebView2` control. The app must deploy
`Microsoft.Web.WebView2.Core.dll` next to the executable. The COM-only path does
not need it. The self-contained reactor setup does this for you:
`windows_reactor_setup::as_self_contained()` carries that DLL with the other
runtime files. The runnable sample is
[`crates/samples/reactor/webview`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/webview).

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is for
contributors and is not needed to use `windows-webview`.

### Capabilities

`windows-webview` exposes a safe API over a hand-picked slice of the WebView2
COM surface. It hides COM types behind owned Rust values. It uses
`implement_decl!` instead of proc macros, so the default crate stays small and
quick to build. WebView2 uses a single-threaded COM apartment and callbacks. For
creation, this crate uses the pump-and-wait model described below.

Covered areas include synchronous creation, navigation, document state, settings,
host and page messaging, DevTools protocol methods and events, custom protocols,
downloads, profiles, environment options, zoom, default background color, DPI,
focus, keyboard events, and the exposed WebView2 event set.

### How it's built

WebView2 ships C/C++ SDK headers. It does not ship an `.rdl` or `.winmd` file.
The bindings start from those headers and run the full pipeline. A dedicated
`tool_webview` crate drives the pipeline:

| Stage | Tool | Output |
|-------|------|--------|
| `WebView2*.h` -> `.rdl` | `windows_clang::clang()` (libclang) | `target/webview/WebView2.rdl` |
| `.rdl` -> `.winmd` | `windows_rdl::reader()` | `target/webview/WebView2.winmd` |
| `.winmd` -> `bindings.rs` | `windows_bindgen` | `crates/libs/webview/src/bindings.rs` |

The clang step needs the SDK headers plus `Windows.Win32.winmd`, which supplies
the Win32 types that the headers reference. It targets `x86_64-pc-windows-msvc`
with `-fms-extensions` and emits the `WebView2` namespace against
`WebView2Loader.dll`. Regenerate with `cargo run -p tool_webview`.
`tool_webview` downloads the `Microsoft.Web.WebView2` NuGet package pinned by
`WEBVIEW2_VERSION` and parses headers from it. Never edit `src/bindings.rs` by
hand.

Two headers are passed as separate `.input()` calls: `WebView2.h` for the core
COM API and `WebView2Interop.h` for the COM to WinRT bridge. They must be listed
separately because the clang step emits only each input's own top-level
declarations. `WebView2Interop.h` includes `WebView2.h`, but the interop header
alone produces an `.rdl` that references `ICoreWebView2` without defining it. The
shared collector merges both translation units into one `WebView2.rdl`.

### The COM to WinRT interop bridge

`WebView2Interop.h` contributes one interface to the bindings:
`ICoreWebView2Interop2`. Its single method is
`GetComICoreWebView2() -> ICoreWebView2`. The WinRT
`Microsoft.Web.WebView2.Core.CoreWebView2` runtime class implements it. This is
the supported path from the WinRT projection to the COM `ICoreWebView2` that this
crate wraps. The two are distinct COM identities and do not plain-QI to each
other.

This bridge lets the WinUI XAML `WebView2` control use the same COM wrappers.
The control's `CoreWebView2` getter returns the WinRT type. `WebView::from_core`
converts it through `ICoreWebView2Interop2` and reuses the crate instead of
adding a separate WinRT wrapper.

The canonical WinRT metadata, `winmd/Microsoft.Web.WebView2.Core.winmd`, lives
in the shared `winmd/` directory with the WinUI metadata. `tool_webview` and
`tool_reactor` use it to resolve the WinRT `CoreWebView2` type when generating
reactor-facing bindings.

### Reactor hosting (the `reactor` feature)

The optional `reactor` feature is behind `#[cfg(feature = "reactor")]`. The
default crate does not depend on WinUI or `windows-reactor`. It has three parts:

- A reactor widget. `windows-reactor` exposes a native-handle `WebView2` control
  in `crates/libs/reactor/src/widgets/web_view2.rs`. It mirrors `SwapChainPanel`.
  It wraps `Microsoft.UI.Xaml.Controls.WebView2` and exposes the raw control as an
  `IInspectable` through `WebView2Handle::as_inspectable`.
- A second bindgen pass. `tool_webview` runs another `windows_bindgen` pass over
  the shared `winmd/` directory with `crates/tools/webview/src/reactor.txt`. It
  emits `crates/libs/webview/src/reactor_bindings.rs`. This minimal WinRT surface
  has `IWebView2` (`EnsureCoreWebView2Async`, `CoreWebView2`,
  `CoreWebView2Initialized`), `IFrameworkElement` (`Loaded`, `IsLoaded`), and the
  opaque `CoreWebView2` and `WebView2` types. It references
  `windows_future::IAsyncAction` instead of re-emitting the async machinery.
- The bridge. `src/reactor.rs` configures the reactor widget's `on_mounted`
  callback. It casts the control to `IWebView2`, calls `EnsureCoreWebView2Async`,
  and handles `CoreWebView2Initialized`. It converts the WinRT `CoreWebView2` to
  COM through `ICoreWebView2Interop2::GetComICoreWebView2` and wraps it with
  `WebView::from_core`.

Two runtime details matter:

- Deferred to `Loaded`. The XAML `WebView2` control can create its `CoreWebView2`
  only after it is attached to a live visual tree. Reactor calls `on_mounted`
  before the control is parented. The bridge defers creation to the control's
  `Loaded` event. It runs immediately if `IsLoaded()` is already true. The
  `Loaded` revoker, the `CoreWebView2Initialized` revoker, and the in-flight
  `IAsyncAction` are kept alive in a per-control `Mounted` struct. They are
  dropped in `on_unmounted`. Dropping the action early can cancel initialization.
- `Microsoft.Web.WebView2.Core.dll` must be deployed. The COM-only path uses
  `webview2loader.dll` from the Evergreen runtime. The XAML control also loads
  the WinRT projection assembly `Microsoft.Web.WebView2.Core.dll`, which ships in
  the `Microsoft.Web.WebView2` NuGet package. `windows_reactor_setup::as_self_contained()`
  stages that package and copies the per-arch `native_uap` DLL next to the
  executable with the Windows App SDK runtime.

### Bindings

`src/bindings.rs` is generated in `--flat --minimal` mode from the filter at
`crates/tools/webview/src/webview.txt`. Minimal mode keeps the generated surface
small. List exactly the methods you need by raw metadata name, such as
`put_IsVisible`, `put_Bounds`, `get_CoreWebView2`, `Navigate`, and
`ExecuteScript`. Do not use the vtable-display names, such as `SetIsVisible` and
`CoreWebView2`. Unlisted methods on a requested interface become `usize` vtable
slots.

The same filter also pulls a few Win32 symbols from the bundled `default` winmd:
`GetMessageW`, `TranslateMessage`, `DispatchMessageW`, `MSG`, `CoTaskMemFree`,
and `SHCreateMemStream`. These back the synchronous pump and string or stream
helpers. The crate does not need a dependency on the `windows` crate.

The filter passes `--dead-code`, which emits generated methods as `pub(crate)`
instead of `pub`. This keeps the raw COM surface crate-private. Free functions go
through the `link!` macro and cannot be `pub(crate)`, but interface methods can.
Implemented interfaces are listed only in `--implement`, never in `--filter`.
They are reached as parameters of the `add_*` and factory methods. `--minimal`
plus `--implement` emits each `_Impl` trait and vtable without a caller-side
method wrapper, so there are no dead `Invoke` getters to lint around.

### Implementing handlers without proc macros

The completion handlers in `handler.rs` wrap a Rust `FnOnce` closure and
implement the matching `ICoreWebView2...Handler` COM interface. They are
`EnvironmentCompleted`, `ControllerCompleted`, `ExecuteScriptCompleted`, and
`AddScriptCompleted`. They use `implement_decl!` instead of `#[implement]`, like
`windows-reactor`. The crate depends on `windows-core` with default features off,
so it avoids the `syn`, `quote`, and `proc-macro2` build cost. This requires the
interface `_Impl` trait and vtable, which `windows-bindgen` emits from the
`--implement` entries in `webview.txt`.

The repeating event handlers use the `event_handler!` macro. An `args` arm wraps
the event args interface. A `sender` arm wraps the sender for events with no args
interface. Each stores an `FnMut` in a `RefCell`.

### Caller-implemented interfaces (environment options)

`implement_decl!` is also used for interfaces that the caller implements.
`CreateCoreWebView2EnvironmentWithOptions` takes an
`ICoreWebView2EnvironmentOptions` that WebView2 does not provide as a concrete
class. The caller must supply one. WebView2 reads the configuration through its
getters.

`options.rs` implements that interface in Rust. The public `EnvironmentOptions`
is a fluent, consuming-self builder. A private `OptionsObject` implements the
interface and returns `CoTaskMemAlloc`-allocated strings through
`string::allocate`. WebView2 frees them. Listing an implemented multi-method
interface in `webview.txt` must be bare:
`WebView2.ICoreWebView2EnvironmentOptions`. A method filter on an `--implement`
interface is rejected.

### Synchronous creation (pump and wait)

`CreateCoreWebView2Environment` and `CreateCoreWebView2Controller` complete
asynchronously by posting to the UI thread's message loop. To present a linear
API, the crate drives that loop itself:

- The async primitives (`create_environment`, `create_controller_async` in
  `environment.rs`) are private and take a closure handler.
- `pump::slot::<T>()` allocates an `Rc<Cell<Option<Result<T>>>>` one-shot slot.
- `pump::slot_handler` builds a `FnOnce` that stores the completion result into
  the slot.
- `pump::wait` blocks in a `GetMessageW`, `TranslateMessage`, and
  `DispatchMessageW` loop until the slot is filled, then returns the result.

Creation runs on the same UI thread, so the slot can be a plain `Rc<Cell<_>>`.
It does not need an `mpsc` channel or `WM_APP` thread kick. The pump should run
only during startup, before the app's own message loop takes over.
`add_script_to_execute_on_document_created` reuses this pump because it is a
setup-time operation. Per-operation `execute_script` stays callback-based to
avoid re-entrant pumping during normal run.

### Events (RAII subscriptions)

WebView2 events use the COM `add_X` and `remove_X` token pattern. The crate wraps
that pattern in an RAII subscription. A `subscription!` macro generates each
`on_*` method. It registers the `handler.rs` adapter with `add_X`, clones the
source interface, and returns an `EventRegistration`. The removal closure calls
`remove_X(token)`. `EventRegistration` is `#[must_use]`. It holds an
`Option<Box<dyn FnOnce()>>` and runs the removal on `Drop` or explicit
`remove(self)`. It takes the closure first, so removal never runs twice.

Most args types are thin newtypes over the COM args interface. Two events carry
no args interface because `Invoke` receives a bare `IUnknown`.
`DocumentTitleChanged` reads the title from the sender and passes a `String` to
the closure. `WindowCloseRequested` reports nothing, so its closure is `FnMut()`.
`NewWindowRequested` and `PermissionRequested` expose mutable decisions plus a
`defer()` that returns a `Deferral`. `Deferral` is a `#[must_use]` wrapper over
`ICoreWebView2Deferral` and completes on drop.

`DownloadStarting` lives on `ICoreWebView2_4`, so `on_download_starting` casts the
inner interface before subscribing. Its `DownloadOperation` is a
`#[derive(Clone)]` newtype. Its `BytesReceivedChanged` and `StateChanged` events
carry the changed object as the sender. The adapter rebuilds it and passes it to
an `FnMut(DownloadOperation)` closure. COM enums arrive as `i32` aliases under
`--minimal`, so the crate uses hand-written Rust enums with `from_raw` and
`to_raw` mappings. Growable enums are `#[non_exhaustive]` and have an `Unknown`
fallback.

### Custom protocols (hiding IStream)

`protocol.rs` intercepts resource loads with `WebResourceRequested`. The event,
its filter (`AddWebResourceRequestedFilter`), and `CreateWebResourceResponse` all
live on base interfaces, so the event needs no `cast`. `on_web_resource_requested`
casts to `ICoreWebView2_2` once at registration to capture the `Environment` for
building responses. The returned `EventRegistration` removes both the handler and
the filter on drop.

`CreateWebResourceResponse` takes the body as a COM `IStream`, but that type does
not reach the public API. `WebResourceResponse` is a plain data builder. When the
handler returns one, the adapter calls `SHCreateMemStream` to copy bytes into a
system-provided stream, then passes it to `CreateWebResourceResponse`. An empty
body passes `None`. The handler runs on the UI thread, so the adapter needs no
`Send` or dispatch support. `WebResourceRequest` is read-only and collects
headers with `ICoreWebView2HttpRequestHeaders::GetIterator`.

### Strings

`string.rs` has helpers for the three string conventions:

- `encode`: convert `&str` to a null-terminated UTF-16 `Vec<u16>` for `LPCWSTR`
  input parameters.
- `decode`: convert borrowed `LPCWSTR` values to `String` for callback strings
  that WebView2 owns for the call.
- `take`: convert caller-owned `LPWSTR` values to `String`, then free them with
  `CoTaskMemFree`.

`[out]` string getters allocate with the COM task allocator and transfer
ownership, so each must be freed. `allocate` is the reverse helper for `[out]`
getters that the crate implements.

### Lint policy

`mod bindings` carries `#[expect(non_snake_case, non_camel_case_types,
clippy::upper_case_acronyms)]`. This is the exact set of casing lints the
generated code fires. A warning surfaces if any expectation stops firing after a
regenerate. It also carries `#[allow(dead_code)]`. With `--dead-code`, bindings
are `pub(crate)`. The `reactor` bridge's `ICoreWebView2Interop2::GetComICoreWebView2`
is reached only through the feature-gated `WebView::from_core`, so it would warn
as unused in a default build without that allowance.

### Testing

There is no automated test crate. WebView2 needs the Edge runtime, a window, and
a message pump, which do not fit headless CI well. Verify with
`cargo run -p tool_webview`, `cargo check -p windows-webview`,
`cargo clippy -p windows-webview --all-targets`, and
`cargo test -p windows-webview` for doctests.
