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

## Implementing handlers without proc-macros

The three completion handlers (`EnvironmentCompleted`, `ControllerCompleted`,
`ExecuteScriptCompleted` in `handler.rs`) wrap a Rust `FnOnce` closure and
implement the corresponding `ICoreWebView2…Handler` COM interface. They use
`implement_decl!` rather than `#[implement]`, exactly like `windows-reactor`, so
the crate depends on `windows-core` with **default features off** — no
`syn`/`quote`/`proc-macro2` build cost. This requires the interface's `_Impl`
trait and vtable, which `windows-bindgen` emits via the `--implement` entries in
`webview.txt`.

## Synchronous creation (pump-and-wait)

WebView2 is callback-driven: `CreateCoreWebView2Environment` and
`CreateCoreWebView2Controller` complete asynchronously by posting to the UI
thread's message loop. To present a straight-line API (`Environment::new()`,
`Environment::create_controller()`), the crate drives that loop itself:

- The async primitives (`create_environment`, `create_controller_async` in
  `environment.rs`) are private and take a closure handler.
- `pump::slot::<T>()` allocates an `Rc<Cell<Option<Result<T>>>>` one-shot slot;
  `slot_handler` builds a `FnOnce` that stores the completion result into it.
- `pump::wait` blocks in a `GetMessageW`/`TranslateMessage`/`DispatchMessageW`
  loop until the slot is filled, then returns the result. `-1` maps to
  `Error::from_thread()`, `0` (`WM_QUIT`) to `Error::empty()`.

This is the same idea as wravery's `wait_with_pump`, but with no `mpsc` and no
`WM_APP` thread-kick: because creation runs on the same UI thread, the slot is a
plain `Rc<Cell<…>>`. The pump dispatches all pending messages while waiting, so
it should only run during startup, before the app's own message loop takes over.
Per-operation scripting (`WebView::execute_script`) stays callback-based to
avoid re-entrant pumping during normal app run.

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

The sample subscribes before navigating and parks the `EventRegistration` in a
`thread_local` so it outlives the call; dropping it on `WM_DESTROY` unsubscribes.

## Strings

`string.rs` has `encode` (`&str` → null-terminated UTF-16 `Vec<u16>` for
`LPCWSTR` IN params) and `decode` (caller-owned `LPCWSTR` → `String`). It does
**not** yet handle `CoTaskMemAlloc`/`CoTaskMemFree` OUT strings — adding string
getters (title, source, `WebMessageAsJson`) will need an RAII free wrapper.

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
