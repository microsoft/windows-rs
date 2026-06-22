# windows-webview — internals

> Maintenance notes for `windows-webview`. For usage, see the [guide](../guide/windows-webview.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/webview)

> **Status:** experimental and pre-release (`0.0.0`). The crate currently proves
> out the design with synchronous environment/controller creation, navigation,
> and script execution. Events, settings, environment options, and host↔JS
> messaging are not implemented yet.

## How it's built

WebView2 ships only a C/C++ SDK header (`WebView2.h`), not an `.rdl` or
`.winmd`, so the bindings start from the header and run the full pipeline. This
is driven by a dedicated `tool_webview` crate (like `tool_reactor`), not
`tool_bindings`:

| Stage | Tool | Output |
|-------|------|--------|
| `WebView2.h` → `.rdl` | `windows_rdl::clang()` (libclang) | `target/webview/WebView2.rdl` |
| `.rdl` → `.winmd` | `windows_rdl::reader()` | `target/webview/WebView2.winmd` |
| `.winmd` → `bindings.rs` | `windows_bindgen` | `crates/libs/webview/src/bindings.rs` |

The clang step needs `WebView2.h` plus `Windows.Win32.winmd` (for the Win32
types the header references), targets `x86_64-pc-windows-msvc` with
`-fms-extensions`, and emits into the `WebView2` namespace against
`WebView2Loader.dll`. Regenerate with `cargo run -p tool_webview`. Keeping the
header (rather than a checked-in `.rdl`) means SDK updates are a header drop plus
a regenerate. Never hand-edit `src/bindings.rs`.

## Bindings

`src/bindings.rs` is generated in `--flat --minimal` mode from the filter at
`crates/tools/webview/src/webview.txt`. Minimal mode is the key win — full
WebView2 bindings are ~45k lines; the minimal set is ~460. List exactly the
methods you need using **raw metadata names** (`put_IsVisible`, `put_Bounds`,
`get_CoreWebView2`, `Navigate`, `ExecuteScript`), not the vtable-display names
(`SetIsVisible`, `CoreWebView2`). Unlisted methods on a requested interface
become `usize` vtable slots.

The same filter also pulls a few Win32 symbols from the bundled `default` winmd
(`GetMessageW`, `TranslateMessage`, `DispatchMessageW`, `MSG`) — these back the
synchronous pump (see below), so the crate needs no dependency on the `windows`
crate.

The filter also passes `--dead-code`, which emits the generated methods as
`pub(crate)` instead of `pub` so the compiler's dead-code lint flags any binding
the crate stops using. (Free functions go through the `link!` macro and can't be
`pub(crate)`, but interface methods can.) Implemented interfaces (the handlers and
`ICoreWebView2EnvironmentOptions`) are listed only in `--implement`, never in
`--filter`: they are reached as parameters of the `add_*`/factory methods, and
`--minimal` + `--implement` emits each one's `_Impl` trait and vtable **without**
a caller-side method wrapper — so there are no dead `Invoke` getters to lint
around. (This caller-side suppression for implemented C++/COM interfaces was added
to `windows-bindgen`'s `cpp_interface` path to match the existing WinRT
`interface` path; see `crates/libs/bindgen/src/types/cpp_interface.rs`.)

## Implementing handlers without proc-macros

The completion handlers (`EnvironmentCompleted`, `ControllerCompleted`,
`ExecuteScriptCompleted`, `AddScriptCompleted` in `handler.rs`) wrap a Rust
`FnOnce` closure and implement the corresponding `ICoreWebView2…Handler` COM
interface. They use `implement_decl!` rather than `#[implement]`, exactly like
`windows-reactor`, so the crate depends on `windows-core` with **default features
off** — no `syn`/`quote`/`proc-macro2` build cost. This requires the interface's
`_Impl` trait and vtable, which `windows-bindgen` emits via the `--implement`
entries in `webview.txt`.

The event handlers (`NavigationStarting`, `NavigationCompleted`,
`WebMessageReceived`) follow the same approach with an `FnMut` in a `RefCell`.

## Caller-implemented interfaces (environment options)

`implement_decl!` is not only for callbacks. `CreateCoreWebView2EnvironmentWithOptions`
takes an `ICoreWebView2EnvironmentOptions` that **WebView2 has no concrete class
for** — the caller must supply one, and WebView2 reads the configuration through
its getters. `options.rs` therefore implements that interface in Rust:

- The public `EnvironmentOptions` is a plain fluent builder (`user_data_folder`,
  `additional_browser_arguments`, `language`, …), in the reactor's consuming-self
  style because it is constructed once and then consumed — unlike the live,
  `set_*`-mutated `Settings`.
- A private `OptionsObject` implements `ICoreWebView2EnvironmentOptions` via
  `implement_decl!`. Its string getters return `CoTaskMemAlloc`-allocated buffers
  (`string::allocate`) that WebView2 frees — the mirror image of `string::take`.
  The interface's setters are required by the vtable but never called by
  WebView2, so they are inert.
- Listing an implemented multi-method interface in `webview.txt` must be **bare**
  (`WebView2.ICoreWebView2EnvironmentOptions`); a `::{…}` method filter on an
  `--implement` interface is rejected because implemented interfaces always emit
  all methods.

`Environment::with_options` runs the same pump-and-wait as `Environment::new`,
just through the with-options factory.

## Synchronous creation (pump-and-wait)

WebView2 is callback-driven: `CreateCoreWebView2Environment` and
`CreateCoreWebView2Controller` complete asynchronously by posting to the UI
thread's message loop. To present a straight-line API (`Environment::new()`,
`Environment::create_controller()`), the crate drives that loop itself:

- The async primitives (`create_environment`, `create_controller_async` in
  `environment.rs`) are private and take a closure handler.
- `pump::slot::<T>()` allocates an `Rc<Cell<Option<Result<T>>>>` one-shot slot;
  `pump::slot_handler` builds a `FnOnce` that stores the completion result into
  it. Both live in `pump.rs` so every sync wrapper shares them.
- `pump::wait` blocks in a `GetMessageW`/`TranslateMessage`/`DispatchMessageW`
  loop until the slot is filled, then returns the result. `-1` maps to
  `Error::from_thread()`, `0` (`WM_QUIT`) to `Error::empty()`.

This is the same idea as wravery's `wait_with_pump`, but with no `mpsc` and no
`WM_APP` thread-kick: because creation runs on the same UI thread, the slot is a
plain `Rc<Cell<…>>`. The pump dispatches all pending messages while waiting, so
it should only run during startup, before the app's own message loop takes over.
`WebView::add_script_to_execute_on_document_created` reuses this pump (it is a
setup-time operation, like creation) and returns the resulting `ScriptId`.
Per-operation scripting (`WebView::execute_script`) stays callback-based to
avoid re-entrant pumping during normal app run.

## Document-created scripts

`add_script_to_execute_on_document_created` registers JavaScript that runs before
any page script on every new document — the standard way to inject a
host-communication bootstrap. Its COM completion handler has the same shape as
`ExecuteScript` (`Invoke(errorCode, result)`), so `handler::AddScriptCompleted`
mirrors `ExecuteScriptCompleted`; the difference is the `result` string is the
script's **id** rather than an eval result. The id is wrapped in a `ScriptId`
newtype (`script.rs`) and round-tripped to
`remove_script_to_execute_on_document_created`, which is synchronous.

## Events (RAII subscriptions)

WebView2 events follow the COM `add_X`/`remove_X` token pattern. The crate wraps
this as an idiomatic subscription returning an RAII guard:

- `handler::NavigationCompleted` adapts an `FnMut(NavigationCompletedArgs)` to the
  `ICoreWebView2NavigationCompletedEventHandler` COM interface. Unlike the
  one-shot completion handlers (`FnOnce` in a `Cell<Option<…>>`), an event may
  fire repeatedly, so the closure lives in a `RefCell<Box<dyn FnMut(…)>>` and is
  invoked through `(*self.0.borrow_mut())(args)`.
- `WebView::on_navigation_completed` registers the handler with
  `add_NavigationCompleted` (which returns the registration token as an `i64`),
  clones the `ICoreWebView2` source, and returns an `EventRegistration` whose
  removal closure calls `remove_NavigationCompleted(token)`.
- `event::EventRegistration` is `#[must_use]` and holds an
  `Option<Box<dyn FnOnce()>>`. Its `Drop` calls the removal closure, and the
  explicit `remove(self)` takes the closure first so it never runs twice.
- `event::NavigationCompletedArgs` is a thin newtype over the COM args interface,
  exposing `is_success()` and `navigation_id()` (the `WebErrorStatus` enum is
  intentionally left out for now to avoid pulling enum variants into the minimal
  bindings).

The same pattern backs the other events, `WebMessageReceived`
(`handler::WebMessageReceived` + `WebView::on_web_message_received` +
`event::WebMessageReceivedArgs`) and `NavigationStarting`
(`handler::NavigationStarting` + `WebView::on_navigation_starting` +
`event::NavigationStartingArgs`), demonstrating that the subscription shape
generalizes across events with only the args newtype changing.

`NavigationStartingArgs` is also the first **cancelable** event: alongside the
read-only getters (`uri()`, `is_user_initiated()`, `is_redirected()`,
`navigation_id()`) it exposes `is_cancelled()` / `set_cancel(bool)`, which map to
the COM `get_Cancel` / `put_Cancel` (projected as `Cancel()` / `SetCancel`). A
handler can veto a navigation by calling `set_cancel(true)`.

The sample subscribes before navigating and parks the `EventRegistration` in a
`thread_local` so it outlives the call; dropping it on `WM_DESTROY` unsubscribes.

### Events without args interfaces

Two events carry no dedicated args interface — their `Invoke` receives a bare
`IUnknown`. The handler adapters reshape them into natural Rust closures rather
than surfacing the `IUnknown`:

- `DocumentTitleChanged` reads the new title from the `sender`
  (`ICoreWebView2::DocumentTitle`) inside `Invoke` and hands the closure a
  `String`, so consumers write `on_document_title_changed(|title| …)`.
- `WindowCloseRequested` has nothing to report, so its closure is `FnMut()`
  (`on_window_close_requested(|| …)`); the host typically closes its window.

`ContentLoading` is an ordinary args event (`ContentLoadingArgs::is_error_page`,
`navigation_id`).

### Deferral-based events

`NewWindowRequested` and `PermissionRequested` follow the same subscription shape
but their args expose **mutable decisions** and a **deferral**:

- `NewWindowRequestedArgs` — `uri()`, `is_user_initiated()`, `is_handled()` /
  `set_handled(bool)` (block the popup), and `set_new_window(&WebView)` to redirect
  the content into an existing view (`put_NewWindow` takes our `WebView`'s inner
  `ICoreWebView2`).
- `PermissionRequestedArgs` — `uri()`, `kind()`, `is_user_initiated()`, and
  `state()` / `set_state(PermissionState)`.

Both expose `defer() -> Result<Deferral>`. `deferral::Deferral` is a `#[must_use]`
RAII wrapper over `ICoreWebView2Deferral`: `complete()` consumes it and calls
`Complete`, and `Drop` completes it automatically so a forgotten deferral can
never hang the request. This lets a handler return immediately while the decision
(set via `set_state` / `set_handled` before completing) is made asynchronously.

The two COM enums (`COREWEBVIEW2_PERMISSION_KIND` / `_STATE`) arrive as bare `i32`
aliases under `--minimal` (the named constants are stripped), so `PermissionKind`
and `PermissionState` are hand-written Rust enums with `from_raw` / `to_raw`
mappings keyed on the documented integer values.

### Downloads (versioned interface + a sender-carried object)

`download.rs` adds the first **versioned** subscription and the first wrapped COM
object that is not an args bag. `DownloadStarting` lives on `ICoreWebView2_4`, not
the base `ICoreWebView2`, so `WebView::on_download_starting` `cast`s its inner
interface (`let source: ICoreWebView2_4 = self.0.cast()?;`) before subscribing —
the same pattern the versioned settings will use. The cast pulls in the
`ICoreWebView2_4` GUID, so the filter requests only its two `add_/remove_` methods
(every other slot stays a `usize` placeholder under `--minimal --flat`).

`DownloadStartingArgs` is a normal args type (cancel, result-file-path override,
handled, `defer()`), but `download_operation()` hands back a `DownloadOperation`
newtype over `ICoreWebView2DownloadOperation`. It is `#[derive(Clone)]` (it wraps a
refcounted interface) and exposes the progress getters (`bytes_received`,
`total_bytes_to_receive`, `state`, `interrupt_reason`, …) plus control verbs
(`cancel`, `pause`, `resume`). Its own `BytesReceivedChanged` / `StateChanged`
events carry **no args** and an `IUnknown` placeholder — the changed object is the
*sender*. The handler adapters (`BytesReceivedChanged`, `DownloadStateChanged`)
rebuild a `DownloadOperation` from `sender` and pass it to an
`FnMut(DownloadOperation)` closure, so callers read the new bytes/state straight
off the operation they were handed. `COREWEBVIEW2_DOWNLOAD_STATE` and
`_INTERRUPT_REASON` are again bare `i32` aliases, mapped to the hand-written
`DownloadState` / `DownloadInterruptReason` enums.

## Navigation and document state

`WebView` exposes the navigation verbs directly off `ICoreWebView2`:
`navigate`, `navigate_to_string` (load HTML from a string), `reload`, `stop`,
`go_back`, `go_forward`. The two document getters, `source()` (current URI) and
`document_title()`, return owned `[out]` strings and so go through
`string::take` (decode + `CoTaskMemFree`).

## Settings

`WebView::settings()` returns a `settings::Settings` newtype over
`ICoreWebView2Settings`, grouping the browser's feature toggles behind one
cohesive type (the reactor lesson: hide a sprawling COM surface behind a single,
clearly-named Rust object). The base interface is entirely boolean properties, so
`settings.rs` defines a small `settings_bool!` macro that expands each COM
`get_X` / `put_X` pair into an idiomatic getter / `set_*` pair — e.g.
`AreDevToolsEnabled` / `put_AreDevToolsEnabled` becomes `are_dev_tools_enabled()`
/ `set_dev_tools_enabled(bool)`. Getters swallow errors to `bool` (matching the
event-args convention); setters return `Result<()>` (matching `Controller`). The
macro keeps the nine properties declarative and uniform without per-property
boilerplate.

Versioned settings (`ICoreWebView2Settings2`–`9`: user agent, autofill, pinch
zoom, swipe navigation, …) require `cast`-ing to later interfaces and are left
for a future slice.

## Host ↔ JavaScript messaging

Two directions, both on `ICoreWebView2`:

- **Host → page**: `WebView::post_web_message_as_json` /
  `post_web_message_as_string` encode a `&str` to UTF-16 and call
  `PostWebMessageAsJson` / `PostWebMessageAsString`. The page receives them via
  `window.chrome.webview.addEventListener("message", …)`.
- **Page → host**: `WebView::on_web_message_received` is the RAII subscription
  above; the page sends with `window.chrome.webview.postMessage(...)`.
  `WebMessageReceivedArgs` exposes `source()`, `web_message_as_json()`, and
  `try_web_message_as_string()` (the last returns `Result` because it genuinely
  fails when the page posted a non-string value).

The sample wires the full round-trip: it subscribes to incoming messages, and on
`NavigationCompleted` runs `execute_script` to have the page call
`postMessage`, which then arrives back on the host handler.

## Custom protocols (serving responses from memory)

`protocol.rs` lets an app intercept resource loads and answer them from Rust,
which is how a self-contained app serves its bundled HTML/JS/CSS without a local
server. The three moving parts all live on **base** interfaces, so no `cast` is
needed for the event itself: `AddWebResourceRequestedFilter` and
`add_WebResourceRequested` are on `ICoreWebView2`, and
`CreateWebResourceResponse` is on `ICoreWebView2Environment`.

`WebView::on_web_resource_requested(uri_filter, handler)` registers a filter (a
wildcard URI such as `https://app.example/*`, always with
`COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL` so sub-resources are covered too) and the
event. The handler is `FnMut(WebResourceRequest) -> Option<WebResourceResponse>`,
run **synchronously on the UI thread**: it returns `Some(response)` to fulfil the
request or `None` to let WebView2 handle it normally. There is no deferral or
threading in this slice — serving in-memory bytes is synchronous, which keeps the
adapter free of `Send`/dispatch machinery (async deferral can be added later).

The returned `EventRegistration` removes *both* the event handler and the filter
on drop, so dropping it fully unsubscribes (and re-subscribing won't stack
duplicate filters).

**Hiding `IStream`.** `CreateWebResourceResponse` takes the body as a COM
`IStream`, but that type never reaches the public surface. `WebResourceResponse`
is a plain builder (`new(bytes)`, `status`, `header`, `content_type`); when the
handler returns one, the adapter calls `SHCreateMemStream` (shlwapi) to copy the
bytes into a system-provided stream and passes it straight to
`CreateWebResourceResponse`. `SHCreateMemStream` was chosen over
`CreateStreamOnHGlobal` (which needs a manual seek-to-start or WebView2 reads an
empty body) and over a hand-rolled `implement_decl!` `IStream` (~13 unsafe vtable
methods, with `Stat` having to report the size correctly) — the system stream is
one call, correct by construction, and keeps `IStream` entirely inside
`mod bindings`. `SHCreateMemStream` returns `Option<IStream>` (null on OOM); an
empty body passes `None`, matching WebView2's "no content" contract.

`WebResourceRequest` is a read-only view: `uri()`, `method()`, and `headers()`.
Header enumeration walks `ICoreWebView2HttpRequestHeaders::GetIterator` with the
`HasCurrentHeader` / `GetCurrentHeader` / `MoveNext` cursor, taking each
caller-owned `LPWSTR` pair through `string::take`. The request POST body (another
`IStream`) is deferred to a later slice.

## Strings

`string.rs` has three helpers:

- `encode` (`&str` → null-terminated UTF-16 `Vec<u16>` for `LPCWSTR` IN params),
- `decode` (borrowed `LPCWSTR` → `String`, used for callback strings that
  WebView2 owns for the duration of the call, e.g. the `ExecuteScript` result),
- `take` (caller-owned `LPWSTR` → `String`, then `CoTaskMemFree`). WebView2
  `[out]` string getters (`get_Source`, `get_WebMessageAsJson`,
  `TryGetWebMessageAsString`) allocate with the COM task allocator and transfer
  ownership to the caller, so each must be freed. `CoTaskMemFree` is pulled into
  the minimal bindings via the filter.

## Lint policy

`mod bindings` carries `#[expect(non_snake_case, non_camel_case_types,
clippy::upper_case_acronyms)]` — the exact set of lints the generated code fires.
`#[expect]` (over `#[allow]`) means a warning surfaces if any listed lint stops
firing after a regenerate, so dead-code is never silently hidden.

## Sample

`crates/samples/webview/minimal` mirrors `canvas_standalone`: its `build.rs`
generates just the Win32 window/message-loop bindings it needs via
`windows-bindgen`, so it depends only on `windows-webview` + `windows-core`, not
the `windows` crate. With synchronous creation the sample reads top-to-bottom —
create window, `Environment::new()`, `create_controller(window)`, `navigate`,
run the message loop.

## Testing

There is no automated test crate yet — WebView2 needs the Edge runtime, a
window, and a message pump, which is awkward in headless CI. Verify with
`cargo run -p tool_webview`, `cargo check -p windows-webview`,
`cargo clippy -p windows-webview --all-targets`, and
`cargo test -p windows-webview` (doctest).
