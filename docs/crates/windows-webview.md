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

## Prerequisites

- The Microsoft Edge WebView2 runtime must be installed for `HWND` hosting.
- The host must provide a live parent window and continue dispatching messages on its UI thread.
  [`windows-window`](windows-window.md) is a small host suitable for this purpose.
- Create the environment on a COM single-threaded apartment (STA). `Environment::new` and
  `Environment::with_options` initialize the calling thread as an STA when needed. They return an
  error if that thread was already initialized as a multi-threaded apartment.
- Keep the parent window alive longer than its `Controller`.

The README contains dependency setup and the smallest hosting example. The workflow below starts
where that example leaves off.

## First workflow: host a resizable page

1. Create the parent window on the UI thread.
2. Create one `Environment`, then a `Controller` for the window.
3. Set the controller bounds to the current client area and repeat that operation from the window's
   resize callback.
4. Obtain the `WebView`, register the events needed by the application, and retain every returned
   `EventRegistration`.
5. Navigate, then enter the host message loop.
6. Close the controller before destroying its parent when shutdown order is under application
   control.

The important resize and subscription work looks like this once the controller exists:

```rust,no_run
use windows_webview::*;

fn configure(controller: &Controller, webview: &WebView, width: i32, height: i32)
    -> Result<EventRegistration>
{
    controller.set_bounds(0, 0, width, height)?;
    let navigation = webview.on_navigation_completed(|args| {
        println!("navigation succeeded: {}", args.is_success());
    })?;
    webview.navigate("https://learn.microsoft.com/windows/apps/")?;
    Ok(navigation)
}
```

Dropping `navigation` immediately would unsubscribe the handler. The WebView samples keep
registrations in a `Vec<EventRegistration>` for the lifetime of the message loop.

## Object and callback lifecycle

`Environment` owns the user-data location and browser process context. Reuse it to create
controllers that should share that context. `Controller` owns the browser hosted in a parent
window and controls bounds, visibility, focus, and display properties. `WebView` represents the
page and provides navigation, scripting, messaging, profile, and event APIs.

Environment and controller creation are asynchronous WebView2 operations. The crate presents them
as synchronous calls by pumping the calling thread's message queue until each callback completes.
Call them during setup, before entering the application's main message loop. The same rule applies
to `add_script_to_execute_on_document_created`. Normal operations such as `execute_script`,
cookie enumeration, profile cleanup, and DevTools calls remain callback-based and complete on the
UI thread.

Call `Controller::close` for explicit shutdown. In all cases, keep the `Controller` alive while
using its `WebView`. A raw parent handle passed to an unsafe `create_*_for_hwnd` method must remain
valid for the controller's lifetime.

## Navigation and page state

`WebView` supports `navigate`, `navigate_to_string`, `reload`, `stop`, `go_back`, and
`go_forward`. Use `source` and `document_title` to read the current top-level document.
`NavigationRequest` adds a custom method, headers, or request body:

```rust,no_run
use windows_webview::{NavigationRequest, Result, WebView};

fn submit(webview: &WebView) -> Result<()> {
    let request = NavigationRequest::new("https://example.test/session")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(br#"{"active":true}"#.to_vec());
    webview.navigate_with_request(&request)
}
```

`on_navigation_starting` can inspect and cancel a navigation. `on_content_loading` and
`on_navigation_completed` report later stages. Use the navigation ID to correlate callbacks.
Handle `on_process_failed`: a render-process exit can be followed by `reload`, while a browser
process exit requires a new `WebView`.

## Events, decisions, and cleanup

Every `on_*` method returns a `#[must_use]` `EventRegistration`. Dropping it, or calling
`remove`, unregisters the callback. Event callbacks run on the UI thread and are not `Send`;
keep their work short and move longer work out of the callback without blocking message dispatch.

The event set covers navigation, content loading, title and fullscreen changes, window-close and
new-window requests, permissions, downloads, process failures, web messages, resource requests,
focus, and accelerator keys.

`NewWindowRequestedArgs` and `PermissionRequestedArgs` can be completed after the event returns by
taking a `Deferral`. The deferral completes on drop. Keep it alive until the decision has been
applied; dropping it early tells WebView2 that handling is complete.

Download progress subscriptions have the same RAII rule. Keep the registrations returned by
`DownloadOperation::on_bytes_received_changed` and `on_state_changed`, not just the outer
download-starting registration.

## Host integration

### Size, visibility, DPI, and position

- `Controller::set_bounds` uses parent-client pixel coordinates.
- Call `notify_parent_window_position_changed` from the parent's `WM_MOVE` handling so browser
  popups and dialogs follow the host.
- `set_visible` hides or shows the controller. Set the `WebView` memory target to
  `MemoryUsageTargetLevel::Low` while hidden and restore `Normal` when shown.
- `zoom_factor` controls page zoom. `rasterization_scale` controls rendering scale.
- Monitor DPI changes are detected by default. Disable
  `set_should_detect_monitor_scale_changes` before setting a scale that the application owns.
- `set_default_background_color` controls the area before the page paints. WebView2 supports fully
  opaque or fully transparent alpha; use `Color::TRANSPARENT` to show the host behind the page.

### Focus and keyboard

Call `move_focus(MoveFocusReason::Programmatic)` when the parent receives `WM_SETFOCUS`.
`on_move_focus_requested` lets the host continue Tab navigation into another native control;
move focus and mark the request handled. `on_accelerator_key_pressed` can consume application
shortcuts before the page handles them. The related browser accelerator setting determines
whether WebView2's built-in shortcuts remain enabled.

### Controller creation options

`ControllerOptions` selects a profile name, private mode, and initial background color. Pass it to
`Environment::create_controller_with_options`. These choices apply at controller creation and
cannot be retrofitted through `WebView`.

## Environment and browser settings

`EnvironmentOptions` configures the browser executable folder, user-data folder, browser arguments,
language, minimum compatible browser version, operating-system account sign-on, browser
extensions, and scrollbar style. Pass it to `Environment::with_options`.

Choose a writable, application-owned user-data folder when the default location is unsuitable.
Controllers created with the same environment share its browser process and data context, while
named profiles isolate cookies, cache, and storage inside that context.

`WebView::settings` returns toggles for script, web messages, dialogs, status bar, DevTools,
context menus, host objects, zoom controls, error pages, accelerator keys, autofill, password
saving, pinch zoom, swipe navigation, and non-client regions. It also supports a user-agent
override. Settings take effect on the next navigation.

## Host and JavaScript communication

The page calls `window.chrome.webview.postMessage(...)`; the host receives it through
`on_web_message_received`. Inspect `source` before trusting messages from navigable content.
Use `web_message_as_json` for any JavaScript value or `try_web_message_as_string` when the protocol
requires a string.

The host sends values with `post_web_message_as_json` or `post_web_message_as_string`.
`execute_script` returns a JSON-encoded result through its callback. A script registered with
`add_script_to_execute_on_document_created` runs before page script in each new document; retain
its `ScriptId` if it may need to be removed.

## Local content and request interception

For files on disk, use `set_virtual_host_name_to_folder_mapping` and navigate to an HTTPS origin
such as `https://app.example/index.html`. `HostResourceAccessKind` controls cross-origin access.
Clear the mapping when access should end.

For generated or embedded bytes, use `on_web_resource_requested`. Its wildcard filter limits which
requests reach the handler. Return `Some(WebResourceResponse)` to provide status, headers, content
type, and body, or `None` to continue normal browser handling. The handler runs synchronously on
the UI thread, so prepare expensive content ahead of time.

## Profiles, cookies, downloads, and DevTools

- `CookieManager` creates, updates, enumerates, and deletes cookies. Enumeration is callback-based.
- `Profile` exposes its name, path, private status, preferred color scheme, download folder, and
  callback-based browsing-data cleanup.
- `on_download_starting` can change the result path, cancel the operation, or retain a
  `DownloadOperation` for pause, resume, cancellation, progress, state, and interruption details.
- `call_dev_tools_protocol_method` sends a method and JSON parameters without opening a remote
  debugging port. Most events registered through `on_dev_tools_protocol_event` require enabling
  their CDP domain first.

## Reactor integration

Enable the `reactor` feature when the browser belongs in a Reactor visual tree. `webview` returns a
`View` and supplies a ready `WebView` to its callback. It panics on a native initialization error;
use `webview_result` when the component should handle `IntegrationError` itself. The returned
`WebView` supports the same navigation, messaging, settings, and event APIs as the `HWND` path.

The XAML control initializes only after it enters a live visual tree. Do not expect its callback
during component construction. A self-contained Reactor application must also deploy
`Microsoft.Web.WebView2.Core.dll`; [`windows-reactor-setup`](windows-reactor-setup.md) stages it.
The [`reactor/webview`](../../crates/samples/reactor/webview) sample shows the component and
deployment layout.

## Samples

Run WebView examples with `cargo run -p webview_samples --example <name>`.

| Example | Workflow |
| --- | --- |
| `minimal` | Create an `HWND` host, size its controller, and navigate. |
| `events` | Observe navigation, popup, permission, close, and process-failure events. |
| `ipc` | Inject script, exchange messages, and execute JavaScript. |
| `custom_protocol` | Serve HTML and CSS from memory. |
| `local_files` | Map a folder to an HTTPS virtual host. |
| `downloads` | Track download progress and state. |
| `cookies` | Add and enumerate cookies. |
| `profile` | Use private mode, color scheme, and browsing-data cleanup. |
| `script` | Add, execute, and remove document-created script. |
| `devtools` | Call a CDP method and subscribe to a CDP event. |

The shared helper in `crates/samples/webview/samples/src/lib.rs` demonstrates correct resize,
registration, controller, and message-loop lifetimes.

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
