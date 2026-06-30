# windows-webview

> A safe Rust wrapper around the [WebView2](https://aka.ms/webview2) COM APIs for hosting the Microsoft Edge (Chromium) browser in a window.

- 📦 [crates.io](https://crates.io/crates/windows-webview)
- 📖 [docs.rs](https://docs.rs/windows-webview)
- 🚀 [Getting started](../../crates/libs/webview/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/webview)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview)

`windows-webview` hosts WebView2 inside a Win32 window so you can render web
content and run JavaScript from Rust. The three core types are `Environment`
(owns the browser process and user-data folder), `Controller` (hosts the browser
in a parent `HWND`), and `WebView` (the page itself — navigation, scripting,
events). The browser lives for as long as you keep the `Controller` alive.

WebView2 is callback-driven, but environment and controller creation complete on
the calling UI thread, so the wrapper pumps the message loop for you and returns
once each step is ready — letting you write setup as straight-line code.

## Getting started

Requires the Microsoft Edge WebView2 runtime to be installed, a window, and a
running message loop. `Environment::new` initializes the calling thread's COM
apartment as single-threaded (STA) if it is not already, so you don't have to
call `CoInitializeEx` yourself; it returns an error if the thread is already a
multi-threaded apartment. The samples pair this with
[`windows-window`](windows-window.md) for the window and message loop.

```rust,ignore
use windows_webview::*;

fn host(window: HWND) -> Result<()> {
    let environment = Environment::new()?;
    let controller = unsafe { environment.create_controller(window)? };
    let webview = controller.webview()?;
    webview.navigate("https://github.com/microsoft/windows-rs")?;
    Ok(())
}
```

`Environment::with_options(&EnvironmentOptions::new()...)` configures the
user-data folder, additional browser arguments, and language. Resize the browser
with `controller.set_bounds(..)` from your window's `WM_SIZE` handler.

For a controller with a specific profile, private (incognito) mode, or a
background colour from the first frame, use
`create_controller_with_options(window, &ControllerOptions::new().profile_name("work").in_private_mode(true))`
instead of `create_controller`.

## Zoom, background, and DPI

The `Controller` also exposes the browser's display properties:

- `zoom_factor()` / `set_zoom_factor(1.25)` — the page zoom, where `1.0` is 100%.
- `set_default_background_color(Color::TRANSPARENT)` — the colour painted before
  content loads and wherever the page is transparent; `Color::TRANSPARENT` lets
  the host window show through (only fully opaque or fully transparent are
  supported).
- `rasterization_scale()` / `set_rasterization_scale(..)` plus
  `set_should_detect_monitor_scale_changes(false)` — control the DPI scale
  manually instead of letting the browser follow the monitor.

## Focus and keyboard

Focus and accelerator keys are managed by the `Controller`, since they concern
the browser's placement in the host window's focus chain. Call
`controller.move_focus(MoveFocusReason::Programmatic)` from your `WM_SETFOCUS`
handler to give the browser keyboard focus. The focus events keep embedding
correct for keyboard and screen-reader users:

- `on_got_focus` / `on_lost_focus` — the browser gained or lost focus (`FnMut()`).
- `on_move_focus_requested` — focus is leaving the browser (the user tabbed past
  the last element); move focus to the next host control and call
  `MoveFocusRequestedArgs::set_handled(true)`.
- `on_accelerator_key_pressed` — browser-level keys (function keys, or a key with
  Ctrl/Alt) arrive before the page sees them; inspect
  `AcceleratorKeyPressedArgs` (`key_event_kind()`, `virtual_key()`) to implement
  application shortcuts, and `set_handled(true)` to consume the key.

## Navigation and document state

`WebView` exposes the navigation verbs directly: `navigate(uri)`,
`navigate_to_string(html)` (load HTML from a string), `reload`, `stop`,
`go_back`, `go_forward`. The document getters `source()` (current URI) and
`document_title()` return owned strings. `open_dev_tools_window()` opens the
browser DevTools.

For a navigation that needs a custom HTTP method, request headers, or a body —
for example a `POST` or an `Authorization` header — build a `NavigationRequest`
and pass it to `navigate_with_request`:

```rust,ignore
webview.navigate_with_request(
    &NavigationRequest::new("https://api.example/data")
        .method("POST")
        .header("Authorization", "Bearer …")
        .body("{}"),
)?;
```

Call `controller.notify_parent_window_position_changed()` from your window's
`WM_MOVE` handler so the browser repositions any popups and dialogs it owns.
`controller.set_allow_external_drop(false)` stops files dragged from outside the
app being dropped onto the page. When a `WebView` is hidden, hint
`webview.set_memory_usage_target_level(MemoryUsageTargetLevel::Low)` so the
browser can trim memory, and set it back to `Normal` when shown again.

## Events

Every event is an RAII subscription: `on_*` registers a handler and returns an
`EventRevoker` (re-exported from `windows-core`) that unsubscribes on drop. Keep
it alive for as long as you want the handler to fire, or call `forget()` to leave
the handler registered indefinitely.

- `on_navigation_starting` / `on_navigation_completed` / `on_content_loading` —
  navigation lifecycle; `NavigationStartingArgs::set_cancel(true)` vetoes a load.
- `on_document_title_changed` — receives the new title `String`.
- `on_window_close_requested` — the page called `window.close()`.
- `on_new_window_requested` — suppress, redirect, or `defer()` a popup.
- `on_permission_requested` — grant/deny/`defer()` a capability request.
- `on_download_starting` — inspect and control downloads (see below).
- `on_web_resource_requested` — serve responses from memory (see below).
- `on_process_failed` — a browser process crashed, exited, or hung;
  `ProcessFailedArgs::kind()` distinguishes a render-process crash (reload the
  page) from a browser-process exit (recreate the `WebView`). Without a handler a
  crash silently leaves a blank page.
- `on_contains_fullscreen_element_changed` — receives the new `bool` when the
  page enters or leaves HTML fullscreen (for example a video), so the host can
  resize or restore its window to match.

`NewWindowRequestedArgs` and `PermissionRequestedArgs` expose a `defer()` that
returns a `Deferral`, letting a handler resolve the decision asynchronously; the
`Deferral` completes automatically on drop so a forgotten one can't hang the page.

## Settings

`webview.settings()` returns a `Settings` grouping the browser's feature toggles —
`set_script_enabled`, `set_dev_tools_enabled`, `set_default_context_menus_enabled`,
`set_status_bar_enabled`, `set_zoom_control_enabled`, and so on — each a
getter / `set_*` pair.

The later WebView2 settings interfaces are exposed too, each reached by an
internal `cast` so they return `Result`: `user_agent()` / `set_user_agent(..)`,
`set_browser_accelerator_keys_enabled`, `set_general_autofill_enabled`,
`set_password_autosave_enabled`, `set_pinch_zoom_enabled`,
`set_swipe_navigation_enabled`, and `set_non_client_region_support_enabled` (the
`app-region: drag` support a custom title bar needs).

## Host ↔ JavaScript messaging

- **Host → page:** `post_web_message_as_json(json)` /
  `post_web_message_as_string(text)`; the page receives them via
  `window.chrome.webview.addEventListener("message", …)`.
- **Page → host:** `on_web_message_received` delivers a `WebMessageReceivedArgs`
  with `source()`, `web_message_as_json()`, and `try_web_message_as_string()`; the
  page sends with `window.chrome.webview.postMessage(...)`.

`add_script_to_execute_on_document_created(js)` injects a bootstrap script that
runs before page script on every new document — the usual place to set up the
host-communication channel. `execute_script(js, callback)` evaluates script and
delivers the JSON result to the callback.

## Custom protocols

`on_web_resource_requested(uri_filter, handler)` intercepts resource loads whose
URI matches a wildcard filter (such as `https://app.example/*`) and lets the
handler answer them from memory — the basis for shipping a self-contained app
that serves its bundled assets without a local server. The handler is
`FnMut(WebResourceRequest) -> Option<WebResourceResponse>`, run synchronously on
the UI thread; return `Some(response)` to fulfil the request or `None` to let
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
`WebResourceResponse::new(bytes)` defaults to `200 OK`; chain `.status(code, reason)`,
`.header(name, value)`, and `.content_type(value)`. The COM `IStream` body is built
internally and never appears in the API.

## Serving local files

When the assets already live in a folder on disk, mapping a virtual host is
simpler than intercepting each request:
`set_virtual_host_name_to_folder_mapping(host, folder, access)` makes the files
under `folder` load over an ordinary URL such as `https://app.example/index.html`,
with correct MIME types and no per-request handler. `HostResourceAccessKind`
controls cross-origin access (`Deny`, `Allow`, or `DenyCors`), and
`clear_virtual_host_name_to_folder_mapping(host)` removes the mapping.

```rust,ignore
webview.set_virtual_host_name_to_folder_mapping(
    "app.example",
    r"C:\path\to\assets",
    HostResourceAccessKind::Deny,
)?;
webview.navigate("https://app.example/index.html")?;
```

Reach for `on_web_resource_requested` instead when responses are generated in
memory rather than read from disk.

## Cookies

`webview.cookie_manager()` returns a `CookieManager`. Reading cookies is
asynchronous and callback-based, mirroring `execute_script`:

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

Write with `add_or_update_cookie(&Cookie)` — build one with
`Cookie::new(name, value, domain, path)` and set the public fields (`is_secure`,
`is_http_only`, `same_site`, `expires`). Remove cookies with
`delete_cookies(name, uri)`, `delete_cookies_with_domain_and_path(..)`, or
`delete_all_cookies()`. The COM cookie and cookie-list types stay internal.

## Profiles

`webview.profile()` returns the `Profile` the browser belongs to — the on-disk
identity (`name`, `path`, `is_in_private_mode`) shared by every `WebView`
created with it. Set `preferred_color_scheme(PreferredColorScheme::Dark)` to
drive a site's `prefers-color-scheme` theme, change the
`default_download_folder_path`, or clear everything with the callback-based
`clear_browsing_data_all`:

```rust,ignore
let profile = webview.profile()?;
profile.set_preferred_color_scheme(PreferredColorScheme::Dark)?;
profile.clear_browsing_data_all(|result| {
    let _ = result;
})?;
```

Choose the profile, private mode, and background colour up front with
`ControllerOptions` (see [Getting started](#getting-started)); enable browser
extensions and pick a scrollbar style with `EnvironmentOptions`
(`are_browser_extensions_enabled`, `scrollbar_style`).

## Downloads

`on_download_starting` delivers a `DownloadStartingArgs` to inspect or cancel the
download, override its destination, or take its `DownloadOperation`. The
`DownloadOperation` exposes progress getters (`bytes_received`,
`total_bytes_to_receive`, `state`, `interrupt_reason`, …) and control verbs
(`cancel`, `pause`, `resume`), plus its own `on_bytes_received_changed` /
`on_state_changed` subscriptions for live progress.

## Samples

The [`crates/samples/webview/samples`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview/samples)
crate has one example per capability under `examples/`, mirroring
`reactor/samples`. A shared `run` helper in `src/lib.rs` holds the hosting
boilerplate — a [`windows-window`](windows-window.md) window whose `on_resize`
forwards to `Controller::set_bounds`, an `Environment`, and a `Controller` — so
each example stays focused on its feature. The only dependencies are
`windows-webview` and `windows-window`; there is no hand-rolled Win32 message
loop and no `windows` crate. All of them require the Microsoft Edge WebView2
runtime.

Run one with `cargo run -p webview_samples --example <name>`:

| Example | Demonstrates |
| --- | --- |
| `minimal` | The smallest host: create a window, environment, and controller, then navigate. |
| `events` | The navigation lifecycle plus window-close, new-window, permission, and process-failed events. |
| `ipc` | Host ↔ page messaging: an injected bootstrap script, received messages, replies, and `execute_script`. |
| `custom_protocol` | Serving an app entirely from memory with `on_web_resource_requested`. |
| `local_files` | Serving a folder on disk through `set_virtual_host_name_to_folder_mapping`. |
| `downloads` | Watching downloads and reporting per-operation progress and state. |
| `cookies` | Adding a cookie and enumerating cookies with the cookie manager. |
| `profile` | An in-private controller, the dark color scheme, and clearing browsing data. |
| `script` | The document-created script lifecycle: inject a script that runs before each page, read a value it set back with `execute_script`, then remove it. |

## Reactor integration

With the optional `reactor` feature, `windows-webview` can host a browser inside a
[`windows-reactor`](windows-reactor.md) UI tree instead of a bare `windows-window`
HWND. The feature adds one function, `webview`, that returns a reactor `WebView2`
control element and hands you a fully-wired [`WebView`] once the browser is ready:

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

The `WebView` passed to the closure is the *same* type used for HWND hosting, so
navigation, scripting, IPC, cookies, settings, profiles, and every other wrapper
work identically — the reactor path is just a second front-end over the one COM
surface. Enable it with `windows-webview = { version = "...", features = ["reactor"] }`.

Because the reactor hosts the WinUI XAML `WebView2` control, the app must deploy
`Microsoft.Web.WebView2.Core.dll` next to the executable (the COM-only path does not
need it — see the internals below). The self-contained reactor setup does this for
you: `windows_reactor_setup::as_self_contained()` carries that DLL alongside the
other runtime files, so no extra build step is required. The runnable sample is
[`crates/samples/reactor/webview`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/webview).

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-webview`**.

### Capabilities

`windows-webview` exposes a curated, safe, idiomatic API over a *minimal*,
hand-picked slice of the WebView2 COM surface. It hides the COM types behind owned
Rust values and avoids proc macros via `implement_decl`, so the default crate
stays small and quick to build. A Rust `Future`/`async` layer is a deliberate
non-goal: WebView2's single-threaded COM apartment and callback model make the
pump-and-wait approach (below) a more natural fit.

Covered: synchronous creation, navigation and document state, settings, host ↔
page messaging, custom protocols (serve from memory), downloads, profiles,
environment options, controller polish (zoom, default background
colour/transparency, DPI/rasterization scale), focus and keyboard events, and the
navigation, content-loading, document-title, window-close, new-window,
permission, process-failed, and fullscreen events.

Out of scope: host objects (`AddHostObjectToScript` — `postMessage` covers the
same host ↔ page need), the DevTools protocol (`CallDevToolsProtocolMethod`),
`TrySuspend`/`Resume`, `CapturePreview`, and the frame, composition (windowless),
print, and audio interface families. Each can be added later behind the same
minimal-binding pattern when a concrete need arises.

### How it's built

WebView2 ships only C/C++ SDK headers, not an `.rdl` or `.winmd`, so the bindings
start from the headers and run the full pipeline. This is driven by a dedicated
`tool_webview` crate (like `tool_reactor`), not `tool_bindings`:

| Stage | Tool | Output |
|-------|------|--------|
| `WebView2*.h` → `.rdl` | `windows_rdl::clang()` (libclang) | `target/webview/WebView2.rdl` |
| `.rdl` → `.winmd` | `windows_rdl::reader()` | `target/webview/WebView2.winmd` |
| `.winmd` → `bindings.rs` | `windows_bindgen` | `crates/libs/webview/src/bindings.rs` |

The clang step needs the SDK headers plus `Windows.Win32.winmd` (for the Win32
types the headers reference), targets `x86_64-pc-windows-msvc` with
`-fms-extensions`, and emits into the `WebView2` namespace against
`WebView2Loader.dll`. Regenerate with `cargo run -p tool_webview`. Keeping the
headers (rather than a checked-in `.rdl`) means SDK updates are a header drop plus a
regenerate. Never hand-edit `src/bindings.rs`.

Two headers are passed as separate `.input()` calls — `WebView2.h` (the core COM
API) and `WebView2Interop.h` (the COM ↔ WinRT bridge, see below). They must be
listed individually because the clang step only emits each input's *own*
top-level declarations, not the types it `#include`s; `WebView2Interop.h`
`#include`s `WebView2.h`, but pointing clang at the interop header alone yields an
`.rdl` that references `ICoreWebView2` without defining it ("type not found"). The
shared collector merges both translation units into one `WebView2.rdl`.

### The COM ↔ WinRT interop bridge

`WebView2Interop.h` contributes one interface to the bindings:
`ICoreWebView2Interop2`, whose single method
`GetComICoreWebView2() -> ICoreWebView2` is implemented by the **WinRT**
`Microsoft.Web.WebView2.Core.CoreWebView2` runtime class. This is the supported,
documented path from the WinRT projection down to the COM `ICoreWebView2` that this
crate wraps — the two are distinct COM identities and do **not** plain-QI to each
other. It is what lets the WinUI/WinRT XAML `WebView2` control (whose `CoreWebView2`
getter returns the WinRT type) be driven through these COM wrappers via
`WebView::from_core`, so the reactor integration reuses the entire crate instead of
duplicating it against a WinRT projection.

The canonical WinRT metadata, `winmd/Microsoft.Web.WebView2.Core.winmd`, is vendored
in the repo's shared `winmd/` directory (the same place the WinUI metadata lives) so
that both `tool_webview` and `tool_reactor` can resolve the WinRT `CoreWebView2` type
through `windows-metadata` when generating the reactor-facing bindings.

### Reactor hosting (the `reactor` feature)

The optional `reactor` feature lives entirely behind `#[cfg(feature = "reactor")]`,
so the default crate stays lean and free of the WinUI/`windows-reactor` dependency.
It is composed of:

- **A reactor widget.** `windows-reactor` exposes a thin native-handle `WebView2`
  control (`crates/libs/reactor/src/widgets/web_view2.rs`), mirroring its
  `SwapChainPanel`. It wraps the XAML `Microsoft.UI.Xaml.Controls.WebView2` and
  exposes the raw control as an `IInspectable` via `WebView2Handle::as_inspectable`.
- **A second bindgen pass.** `tool_webview` runs a *second* `windows_bindgen` pass
  (filter `crates/tools/webview/src/reactor.txt`) over the shared `winmd/` directory,
  emitting `crates/libs/webview/src/reactor_bindings.rs`. This minimal WinRT surface
  has just `IWebView2` (`EnsureCoreWebView2Async`, `CoreWebView2`,
  `CoreWebView2Initialized`), `IFrameworkElement` (`Loaded`, `IsLoaded`), and the
  opaque `CoreWebView2`/`WebView2` types — referencing `windows_future::IAsyncAction`
  rather than re-emitting the async machinery.
- **The bridge.** `src/reactor.rs`'s `webview()` configures the reactor widget's
  `on_mounted` to cast the control to `IWebView2`, call `EnsureCoreWebView2Async`,
  and on `CoreWebView2Initialized` convert the WinRT `CoreWebView2` to the COM
  `ICoreWebView2` through `ICoreWebView2Interop2::GetComICoreWebView2` (the interop
  bridge above) and wrap it with the feature-gated `WebView::from_core`.

Two runtime subtleties are worth knowing:

- **Deferred to `Loaded`.** The XAML `WebView2` control can only create its
  `CoreWebView2` once it is attached to a live visual tree. Reactor calls
  `on_mounted` *before* the control is parented, so calling
  `EnsureCoreWebView2Async` there fails the async with `HRESULT(0x8007007E)`
  ("module could not be found") and `CoreWebView2Initialized` never fires. The
  bridge therefore defers creation to the control's `Loaded` event (running
  immediately only if `IsLoaded()` is already true). The `Loaded` revoker, the
  `CoreWebView2Initialized` revoker, and the in-flight `IAsyncAction` are all kept
  alive in a per-control `Mounted` struct and dropped in `on_unmounted` — dropping
  the action early can cancel initialization.
- **`Microsoft.Web.WebView2.Core.dll` must be deployed.** The COM-only path is
  served by `webview2loader.dll` from the Evergreen runtime, but the XAML control
  additionally loads the WinRT projection assembly `Microsoft.Web.WebView2.Core.dll`,
  which ships only in the `Microsoft.Web.WebView2` NuGet package (not the
  WindowsAppSDK runtime). `windows_reactor_setup::as_self_contained()` stages that
  package and copies the per-arch `native_uap` DLL next to the executable alongside
  the WindowsAppSDK runtime, so a self-contained reactor app needs no extra build
  step.

### Bindings

`src/bindings.rs` is generated in `--flat --minimal` mode from the filter at
`crates/tools/webview/src/webview.txt`. Minimal mode is the key win — full
WebView2 bindings are ~45k lines; the minimal set is far smaller. List exactly the
methods you need using **raw metadata names** (`put_IsVisible`, `put_Bounds`,
`get_CoreWebView2`, `Navigate`, `ExecuteScript`), not the vtable-display names
(`SetIsVisible`, `CoreWebView2`). Unlisted methods on a requested interface become
`usize` vtable slots.

The same filter also pulls a few Win32 symbols from the bundled `default` winmd
(`GetMessageW`, `TranslateMessage`, `DispatchMessageW`, `MSG`, `CoTaskMemFree`,
`SHCreateMemStream`) — these back the synchronous pump and string/stream helpers,
so the crate needs no dependency on the `windows` crate.

The filter also passes `--dead-code`, which emits the generated methods as
`pub(crate)` instead of `pub`, keeping the raw COM surface crate-private rather
than leaking it into the public API. (Free functions go through the `link!` macro
and can't be `pub(crate)`, but interface methods can.) Implemented interfaces (the
handlers and `ICoreWebView2EnvironmentOptions`) are listed only in `--implement`,
never in `--filter`: they are reached as parameters of the `add_*`/factory methods,
and `--minimal` + `--implement` emits each one's `_Impl` trait and vtable
**without** a caller-side method wrapper — so there are no dead `Invoke` getters to
lint around.

### Implementing handlers without proc-macros

The completion handlers (`EnvironmentCompleted`, `ControllerCompleted`,
`ExecuteScriptCompleted`, `AddScriptCompleted` in `handler.rs`) wrap a Rust
`FnOnce` closure and implement the corresponding `ICoreWebView2…Handler` COM
interface. They use `implement_decl!` rather than `#[implement]`, exactly like
`windows-reactor`, so the crate depends on `windows-core` with **default features
off** — no `syn`/`quote`/`proc-macro2` build cost. This requires the interface's
`_Impl` trait and vtable, which `windows-bindgen` emits via the `--implement`
entries in `webview.txt`.

The repeating event handlers are folded behind an `event_handler!` macro: an
`args` arm wraps the event's args interface, a `sender` arm (for events with no
args interface) wraps the sender. Each holds an `FnMut` in a `RefCell`.

### Caller-implemented interfaces (environment options)

`implement_decl!` is not only for callbacks.
`CreateCoreWebView2EnvironmentWithOptions` takes an
`ICoreWebView2EnvironmentOptions` that **WebView2 has no concrete class for** — the
caller must supply one, and WebView2 reads the configuration through its getters.
`options.rs` therefore implements that interface in Rust: the public
`EnvironmentOptions` is a fluent, consuming-self builder, and a private
`OptionsObject` implements the interface, returning `CoTaskMemAlloc`-allocated
strings (`string::allocate`) that WebView2 frees — the mirror image of
`string::take`. Listing an implemented multi-method interface in `webview.txt`
must be **bare** (`WebView2.ICoreWebView2EnvironmentOptions`); a `::{…}` method
filter on an `--implement` interface is rejected.

### Synchronous creation (pump-and-wait)

`CreateCoreWebView2Environment` and `CreateCoreWebView2Controller` complete
asynchronously by posting to the UI thread's message loop. To present a
straight-line API, the crate drives that loop itself:

- The async primitives (`create_environment`, `create_controller_async` in
  `environment.rs`) are private and take a closure handler.
- `pump::slot::<T>()` allocates an `Rc<Cell<Option<Result<T>>>>` one-shot slot;
  `pump::slot_handler` builds a `FnOnce` that stores the completion result into it.
- `pump::wait` blocks in a `GetMessageW`/`TranslateMessage`/`DispatchMessageW` loop
  until the slot is filled, then returns the result.

Because creation runs on the same UI thread, the slot can be a plain
`Rc<Cell<…>>` with no `mpsc` channel or `WM_APP` thread-kick. The pump should only
run during startup, before the app's own message loop takes over.
`add_script_to_execute_on_document_created` reuses this pump (it is a setup-time
operation); per-operation `execute_script` stays callback-based to avoid
re-entrant pumping during normal run.

### Events (RAII subscriptions)

WebView2 events follow the COM `add_X`/`remove_X` token pattern. Because that
shape is structurally identical to a WinRT event, `windows-bindgen` collapses each
`add_X`/`remove_X` pair into a single `X(handler) -> Result<EventRevoker>` method
(see the event-transform note in `docs/crates/windows-bindgen.md`), exactly as it
does for WinRT. The generated method registers the handler with `add_X`, captures
the returned `i64` token and the `remove_X` slot, and hands back a
`windows_core::EventRevoker` that calls `remove_X(token)` on `Drop` (or is
neutralised with `forget`/`into_token`).

A thin `subscription!` macro in `handler.rs` wires each `on_*` method to the
generated event method: it builds the `handler.rs` closure adapter and forwards to
`self.0.X(&handler)`, returning the `EventRevoker` directly — no per-event token,
`remove`, or wrapper plumbing remains in the crate.

Most args types are thin newtypes over the COM args interface. Two events carry no
args interface (`Invoke` receives a bare `IUnknown`): `DocumentTitleChanged` reads
the title from the sender and hands the closure a `String`; `WindowCloseRequested`
reports nothing, so its closure is `FnMut()`. `NewWindowRequested` and
`PermissionRequested` expose mutable decisions plus a `defer()` returning a
`Deferral` (a `#[must_use]` wrapper over `ICoreWebView2Deferral` that completes on
drop).

`DownloadStarting` is the first **versioned** subscription — it lives on
`ICoreWebView2_4`, so `on_download_starting` `cast`s the inner interface before
subscribing (the pattern the versioned settings will reuse). Its
`DownloadOperation` is a `#[derive(Clone)]` newtype whose own `BytesReceivedChanged`
/ `StateChanged` events carry the changed object as the *sender*, rebuilt by the
adapter and handed to an `FnMut(DownloadOperation)` closure. COM enums
(`COREWEBVIEW2_PERMISSION_KIND`/`_STATE`, `COREWEBVIEW2_DOWNLOAD_STATE`/
`_INTERRUPT_REASON`) arrive as bare `i32` aliases under `--minimal`, so they are
hand-written Rust enums with `from_raw`/`to_raw` mappings; the growable ones are
`#[non_exhaustive]` with an `Unknown` fallback.

### Custom protocols (hiding IStream)

`protocol.rs` intercepts resource loads via `WebResourceRequested`. The event,
its filter (`AddWebResourceRequestedFilter`), and `CreateWebResourceResponse` all
live on **base** interfaces, so the event itself needs no `cast`;
`on_web_resource_requested` only `cast`s to `ICoreWebView2_2` once at registration
to capture the `Environment` for building responses. The filter has the same
lifetime as the handler, so the `protocol::WebResourceRequested` adapter owns it:
it registers the filter (`AddWebResourceRequestedFilter`) when created and removes
it in its own `Drop`. Revoking the returned `EventRevoker` releases the handler,
which in turn removes the filter — so a single revoker tears down **both** the
handler and the filter with no bespoke registration type.

`CreateWebResourceResponse` takes the body as a COM `IStream`, but that type never
reaches the public surface. `WebResourceResponse` is a plain data builder; when the
handler returns one, the adapter calls `SHCreateMemStream` (shlwapi) to copy the
bytes into a system-provided stream and passes it straight to
`CreateWebResourceResponse`. `SHCreateMemStream` was chosen over
`CreateStreamOnHGlobal` (which needs a manual seek-to-start or WebView2 reads an
empty body) and over a hand-rolled `implement_decl!` `IStream` (~13 unsafe vtable
methods, with `Stat` having to report the size) — the system stream is one call,
correct by construction, and keeps `IStream` entirely inside `mod bindings`. It
returns `Option<IStream>` (null on OOM); an empty body passes `None`. The handler
runs synchronously on the UI thread, so the adapter needs no `Send`/dispatch
machinery (async deferral can be added later). `WebResourceRequest` is read-only,
walking `ICoreWebView2HttpRequestHeaders::GetIterator` to collect headers; the
request POST body (another `IStream`) is deferred to a later slice.

### Strings

`string.rs` has helpers for the three string conventions: `encode` (`&str` →
null-terminated UTF-16 `Vec<u16>` for `LPCWSTR` IN params), `decode` (borrowed
`LPCWSTR` → `String`, for callback strings WebView2 owns for the call), and `take`
(caller-owned `LPWSTR` → `String`, then `CoTaskMemFree`). `[out]` string getters
allocate with the COM task allocator and transfer ownership, so each must be freed;
`allocate` is the reverse, for the `[out]` getters the crate implements.

### Lint policy

`mod bindings` carries `#[expect(non_snake_case, non_camel_case_types,
clippy::upper_case_acronyms)]` — the exact set of casing lints the generated code
fires — so a warning surfaces if any of them stops firing after a regenerate. It
also carries `#[allow(dead_code)]`: with `--dead-code` the bindings are
`pub(crate)`, and the `reactor` bridge's `ICoreWebView2Interop2::GetComICoreWebView2`
(reached only through the feature-gated `WebView::from_core`) would otherwise warn
as unused in a default, non-`reactor` build.

### Testing

There is no automated test crate yet — WebView2 needs the Edge runtime, a window,
and a message pump, which is awkward in headless CI. Verify with
`cargo run -p tool_webview`, `cargo check -p windows-webview`,
`cargo clippy -p windows-webview --all-targets`, and `cargo test -p windows-webview`
(doctest).

### Future work — WebView2 parity and family fit

`windows-webview` exposes a curated, minimal slice of the large WebView2 COM
surface (see *Capabilities* above for what is covered and *out of scope*). This
section reframes those gaps as prioritized future work and, like the other crates
in the family, sets out how webview complements
[`windows-reactor`](windows-reactor.md) and [`windows-canvas`](windows-canvas.md).

Ordered roughly by impact; "present" notes what already exists.

#### 1. Windowless (Composition) hosting *(high)*

Present: HWND hosting (`Controller`) and the WinUI XAML control (reactor feature) —
both put the browser in its own window/airspace.

Missing: `ICoreWebView2CompositionController` /
`CreateCoreWebView2CompositionController`, which host the browser in a
**Composition visual** instead of an HWND. This is the single biggest
complementarity gap: an HWND overlay can't be clipped, transformed, rounded,
z-ordered, or animated with the surrounding UI. Windowless hosting would let
webview participate in the same visual tree as canvas and reactor — true
transparency, transforms, and animation — rather than floating above it.

#### 2. Host objects and richer IPC *(medium-high)*

Present: `postMessage` string/JSON both directions, plus document-created script
injection and `execute_script`.

Missing: `AddHostObjectToScript` (expose typed host objects so page script can
call host methods directly, beyond message passing) and
`PostSharedBufferToScript` / shared buffers for **zero-copy** transfer of large
data (e.g. handing a dataset — or canvas-produced bytes — to the page without
serializing through JSON).

#### 3. DevTools Protocol *(medium)*

`CallDevToolsProtocolMethod` and `GetDevToolsProtocolEventReceiver` expose the
Chrome DevTools Protocol — automation, performance tracing, network inspection,
emulation, and scripted control. Valuable for testing and tooling.

#### 4. Print and capture *(medium)*

`PrintToPdf` / `PrintToPdfStream` / `Print` render a page to PDF (pairs with the
canvas printing gap), and `CapturePreview` captures the page as a bitmap — which
could feed a canvas `Bitmap` for thumbnails or compositing.

#### 5. More events and page state *(medium)*

Present: navigation lifecycle, content-loading, document-title, window-close,
new-window, permission, process-failed, fullscreen, download, and web-resource
events.

Missing app-relevant events: **`HistoryChanged`** (to enable/disable back/forward
buttons — useful for a reactor toolbar), **`ContextMenuRequested`** (replace the
browser context menu with a native one), the authentication flows
(`BasicAuthenticationRequested`, `ClientCertificateRequested`,
`ServerCertificateError`), `SourceChanged`, `WebResourceResponseReceived`,
`FaviconChanged`, and audio state (`IsMuted` / `IsDocumentPlayingAudio`).

#### 6. Frames and lifecycle *(low-medium)*

`ICoreWebView2Frame` (iframe navigation and events) and `TrySuspend`/`Resume` (the
crate currently exposes only the `MemoryUsageTargetLevel` hint).

#### How it fits with reactor and canvas

webview is already a first-class reactor content surface (the `reactor` feature),
so the relationship is about division of labor:

- **webview as a gap-filler for reactor.** The C# `microsoft-ui-reactor` ships
  whole Markdown and charting subsystems that `windows-reactor` lacks (see its
  *Future work* §5 and §10). A webview can host web-based markdown renderers and
  chart/visualization libraries **today**, so rich documents, dashboards, charts,
  and maps are reachable without writing native control wrappers — the
  `postMessage`, custom-protocol, and virtual-host plumbing to drive them already
  exists.
- **webview vs. canvas — two content surfaces, not competitors.** In a reactor
  tree both `SwapChainPanel` (canvas: GPU 2D, native) and `WebView2` (the web
  stack) are content hosts. Choose **canvas** for high-performance custom drawing,
  games, and low-latency visualizations; choose **webview** for web content, rich
  text / markdown, HTML/CSS layout, and existing JavaScript ecosystems. They are
  siblings serving different content models.
- **Interop bridges.** The shared-buffer (§2) and capture (§4) gaps are what would
  let the two cooperate — capture a page into a canvas bitmap, or push
  canvas-produced data into a page — and windowless composition (§1) is what makes
  webview a true visual peer of canvas and reactor rather than an HWND overlay.

#### Suggested sequencing

Windowless composition (§1) is the highest-leverage but largest piece. Host
objects / IPC (§2) and the app-relevant events — context menus and authentication
(§5) — are smaller and independently shippable. Print/capture (§4) and the DevTools
protocol (§3) are self-contained add-ons that can land whenever a concrete need
arises, behind the same minimal-binding pattern the crate already uses.
