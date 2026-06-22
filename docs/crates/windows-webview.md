# windows-webview

> A safe Rust wrapper around the [WebView2](https://aka.ms/webview2) COM APIs for hosting the Microsoft Edge (Chromium) browser in a window.

- 📦 Not published to crates.io
- 🚀 [Getting started](../../crates/libs/webview/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/webview)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview)

> **Status:** experimental and pre-release (`0.0.0`). The API is evolving and the
> crate is not yet published to crates.io.

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
running message loop.

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
`document_title()` return owned strings.

## Events

Every event is an RAII subscription: `on_*` registers a handler and returns an
`EventRegistration` that unsubscribes on drop (or via `remove()`). Keep it alive
for as long as you want the handler to fire.

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

## Downloads

`on_download_starting` delivers a `DownloadStartingArgs` to inspect or cancel the
download, override its destination, or take its `DownloadOperation`. The
`DownloadOperation` exposes progress getters (`bytes_received`,
`total_bytes_to_receive`, `state`, `interrupt_reason`, …) and control verbs
(`cancel`, `pause`, `resume`), plus its own `on_bytes_received_changed` /
`on_state_changed` subscriptions for live progress.

## Samples

[`crates/samples/webview/minimal`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/webview)
hosts WebView2 in a plain Win32 message loop using only `windows-webview` and
`windows-core` (no `windows` crate). It wires navigation, document-title, content,
permission, new-window, download, and custom-protocol events, serves an embedded
page from memory, and round-trips a host ↔ page message.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-webview`**.

### Coverage and roadmap

Priorities are grounded in what the top `webview2-com` dependents (`wry` →
`tauri`, ~60M downloads) actually use, and in the concrete reliability and
accessibility gaps they hit. A deliberate non-goal is a Rust `Future`/`async`
layer: the entire production ecosystem runs on the same callback + pump-and-wait
model this crate already uses, with no community demand for futures and a
single-threaded COM apartment that makes them awkward.

Covered: synchronous creation, navigation and document state, settings, host ↔
page messaging, custom protocols (serve from memory), downloads, environment
options, and the navigation/title/window/permission/new-window events.

Tracked gaps, roughly by impact:

| Gap | Status | Notes |
|-----|--------|-------|
| `ProcessFailed` (renderer-crash detection) | ✅ Done | `on_process_failed` delivers a `ProcessFailedArgs` with `kind()`. |
| Focus & keyboard events (`GotFocus`/`LostFocus`/`MoveFocusRequested`/`AcceleratorKeyPressed`) | ✅ Done | `Controller::on_got_focus`/`on_lost_focus`/`on_move_focus_requested`/`on_accelerator_key_pressed` plus `move_focus`. |
| `SetVirtualHostNameToFolderMapping` | ✅ Done | `WebView::set_virtual_host_name_to_folder_mapping` / `clear_virtual_host_name_to_folder_mapping`. |
| Controller polish (zoom factor, default background colour/transparency, DPI/rasterization scale) | ✅ Done | `Controller::zoom_factor`/`set_zoom_factor`, `default_background_color`/`set_default_background_color`, `rasterization_scale`/`set_rasterization_scale`, `should_detect_monitor_scale_changes`. |

#### wry / Tauri parity

The remaining gaps below are exactly what the top dependents (`wry` → `tauri`)
call that this crate does not yet expose. Clearing them makes the crate a viable
WebView2 backend for that ecosystem.

| Gap | Used by | Status | Notes |
|-----|---------|--------|-------|
| Versioned `Settings2..9` (user agent, swipe nav, pinch zoom, autofill, …) | wry | ✅ Done | `Settings::user_agent`, `are_browser_accelerator_keys_enabled`, `is_general_autofill_enabled`, `is_password_autosave_enabled`, `is_pinch_zoom_enabled`, `is_swipe_navigation_enabled`, `is_non_client_region_support_enabled`. |
| Cookie manager (`GetCookies`/`AddOrUpdateCookie`/`DeleteCookie`) | wry | ⬜ Planned | `GetCookies` completes asynchronously (pump-and-wait). |
| Controller creation options (`ICoreWebView2ControllerOptions`/`Options3`) | tauri | ⬜ Planned | Incognito/private mode, profile name, default background at create. Needs a `create_controller_with_options`. |
| Profile (`ICoreWebView2Profile`/`Profile2`) | tauri | ⬜ Planned | Theme, `ClearBrowsingDataAll`, add browser extension. |
| `NavigateWithWebResourceRequest` (`ICoreWebView2_10`) | wry | ⬜ Planned | Navigate with custom request headers. |
| `OpenDevToolsWindow` | wry | ⬜ Planned | Devtools feature. |
| `NotifyParentWindowPositionChanged` (Controller) | wry | ⬜ Planned | Called on `WM_MOVE`. |
| `SetAllowExternalDrop` (`Controller4`) | wry | ⬜ Planned | Drag-and-drop control. |
| `ContainsFullScreenElementChanged` event | tauri-runtime-wry | ⬜ Planned | HTML fullscreen handling. |
| Environment options breadth (`AreBrowserExtensionsEnabled`, scrollbar style) | wry | ⬜ Planned | Extends the existing `EnvironmentOptions` builder. |
| `SetMemoryUsageTargetLevel` (`ICoreWebView2_19`) | wry | ⬜ Planned | Background-tab memory trimming. |

Out of scope for parity (wry feature-gates or works around them):
`AddWebResourceRequestedFilterWithRequestSourceKinds` (`_22`),
`AddHostObjectToScript`, `CallDevToolsProtocolMethod`, `TrySuspend`/`Resume`,
`CapturePreview`.

### How it's built

WebView2 ships only a C/C++ SDK header (`WebView2.h`), not an `.rdl` or `.winmd`,
so the bindings start from the header and run the full pipeline. This is driven by
a dedicated `tool_webview` crate (like `tool_reactor`), not `tool_bindings`:

| Stage | Tool | Output |
|-------|------|--------|
| `WebView2.h` → `.rdl` | `windows_rdl::clang()` (libclang) | `target/webview/WebView2.rdl` |
| `.rdl` → `.winmd` | `windows_rdl::reader()` | `target/webview/WebView2.winmd` |
| `.winmd` → `bindings.rs` | `windows_bindgen` | `crates/libs/webview/src/bindings.rs` |

The clang step needs `WebView2.h` plus `Windows.Win32.winmd` (for the Win32 types
the header references), targets `x86_64-pc-windows-msvc` with `-fms-extensions`,
and emits into the `WebView2` namespace against `WebView2Loader.dll`. Regenerate
with `cargo run -p tool_webview`. Keeping the header (rather than a checked-in
`.rdl`) means SDK updates are a header drop plus a regenerate. Never hand-edit
`src/bindings.rs`.

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
`pub(crate)` instead of `pub` so the compiler's dead-code lint flags any binding
the crate stops using. (Free functions go through the `link!` macro and can't be
`pub(crate)`, but interface methods can.) Implemented interfaces (the handlers and
`ICoreWebView2EnvironmentOptions`) are listed only in `--implement`, never in
`--filter`: they are reached as parameters of the `add_*`/factory methods, and
`--minimal` + `--implement` emits each one's `_Impl` trait and vtable **without** a
caller-side method wrapper — so there are no dead `Invoke` getters to lint around.

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

This is the same idea as wravery's `wait_with_pump`, but with no `mpsc` and no
`WM_APP` thread-kick: because creation runs on the same UI thread, the slot is a
plain `Rc<Cell<…>>`. The pump should only run during startup, before the app's own
message loop takes over. `add_script_to_execute_on_document_created` reuses this
pump (it is a setup-time operation); per-operation `execute_script` stays
callback-based to avoid re-entrant pumping during normal run.

### Events (RAII subscriptions)

WebView2 events follow the COM `add_X`/`remove_X` token pattern, wrapped as a
subscription returning an RAII guard. A `subscription!` macro generates each `on_*`
method: it registers the `handler.rs` adapter with `add_X` (which returns an `i64`
token), clones the source interface, and returns an `EventRegistration` whose
removal closure calls `remove_X(token)`. `EventRegistration` is `#[must_use]`,
holds an `Option<Box<dyn FnOnce()>>`, and runs the removal on `Drop` or explicit
`remove(self)` (taking the closure first so it never runs twice).

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
to capture the `Environment` for building responses. The returned
`EventRegistration` removes **both** the handler and the filter on drop.

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
clippy::upper_case_acronyms)]` — the exact set of lints the generated code fires.
`#[expect]` (over `#[allow]`) means a warning surfaces if any listed lint stops
firing after a regenerate, so dead code is never silently hidden.

### Testing

There is no automated test crate yet — WebView2 needs the Edge runtime, a window,
and a message pump, which is awkward in headless CI. Verify with
`cargo run -p tool_webview`, `cargo check -p windows-webview`,
`cargo clippy -p windows-webview --all-targets`, and `cargo test -p windows-webview`
(doctest).
