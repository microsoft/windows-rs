# windows-window

> Minimal window creation and message-loop support for content hosted by a raw `HWND`.

- 📦 [crates.io](https://crates.io/crates/windows-window)
- 📖 [docs.rs](https://docs.rs/windows-window)
- 🚀 [Getting started](../../crates/libs/window/readme.md)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/window)

## When to use it

Use `windows-window` when a Windows desktop application needs a small top-level window and message
loop to host a swap chain, WebView2 controller, Direct2D or Direct3D renderer, or another API that
accepts an `HWND`. It avoids depending on the full [`windows`](windows.md) crate or generating
application-specific bindings for basic windowing.

The crate is not a general UI toolkit. Menus, controls, input policy, multi-window coordination,
and specialized message handling remain the application's responsibility.

## Prerequisites

The crate targets Windows desktop applications. Create and drive a window on the thread that owns
its message queue. Interop code receiving `Window::hwnd()` must not retain the handle beyond the
`Window` lifetime.

The first registered window class sets process DPI awareness to per-monitor v2. Set any different
process DPI policy before creating a `Window`.

The README contains dependency setup and the minimal create-and-run example.

## First workflow: host resize-sensitive content

Most integrations need the following sequence:

1. Put shared renderer or controller state behind `Rc<RefCell<_>>` or another UI-thread owner.
2. Build the window with an `on_resize` closure that updates the hosted content.
3. Call `create`, then use `client_size` for the initial content size.
4. Pass `hwnd` to the hosting API.
5. Choose `run` for an event-driven host or `run_with` for a render loop.
6. Drop hosted resources before the `Window` when their API requires the parent handle to remain
   valid.

`on_resize` receives client-area width and height in physical pixels. The callback also handles the
initial resize messages that arrive after the builder installs its state.

## Window creation and ownership

`Window::new(title)` returns a `WindowBuilder`. `size` sets the initial outer window size.
`style` and `ex_style` replace the defaults with raw `WS_*` and `WS_EX_*` values. The defaults are
`WS_OVERLAPPEDWINDOW` and no extended style.

`on_message` receives `(hwnd, message, wparam, lparam)` and returns `Option<isize>`. Return
`Some(result)` only when the application fully handled the message. Return `None` to use the
crate's built-in handling and `DefWindowProcW`. `on_resize` is the focused alternative for
`WM_SIZE`; if both are installed and `on_message` handles `WM_SIZE`, the resize callback does not
run.

`create` registers the shared window class, creates and shows the window, and returns an error if
creation fails. `Window::client_size` returns `(0, 0)` if `GetClientRect` fails.

Dropping a live `Window` calls `DestroyWindow`. An unhandled `WM_DESTROY` posts `WM_QUIT`, so
closing any window created by this crate ends the thread's message loop. Applications with several
top-level windows must account for that policy.

## Choosing a message loop

| API | Use it when | Behavior |
| --- | --- | --- |
| `run()` | Event-driven updates. | Blocks in `GetMessageW` until quit. |
| `run_with(render)` | Consecutive frames may be needed. | Drains messages, then calls `render`. |
| `pump()` | An external operation owns the wait. | Dispatches pending messages; never blocks. |
| `quit()` | Application state requires loop termination. | Posts `WM_QUIT` to the current thread. |

The `run_with` closure returns `Result<bool>`. Return `Ok(true)` to request another immediate frame,
or `Ok(false)` to block until a message arrives. Propagating an error exits the loop. This lets a
renderer switch between animation and an idle or occluded state without busy-waiting.

`pump` returns `false` after consuming `WM_QUIT`; the caller should then stop its outer loop.
Repeatedly calling `pump` without another wait mechanism spins the CPU.

## Messages, reentrancy, and panics

Message dispatch is reentrant: a handler can call a Win32 API that sends another message before the
first callback returns. The crate temporarily removes both user handlers while either one runs.
Nested messages therefore use default processing instead of re-entering a closure or borrowing its
captured `RefCell` again.

This also means a nested `WM_SIZE` triggered inside a handler does not invoke `on_resize`. Apply any
state update needed by that synchronous operation directly.

Handlers run across an `extern "system"` window-procedure boundary without `catch_unwind`. A panic
that escapes a handler aborts the process rather than unwinding through Win32. Return errors
through captured application state or catch a panic inside the closure if recovery is required.

Do not perform long blocking work in a message handler. It prevents painting, input, timers, and
other components on the same UI thread from progressing.

## Interop and common options

- Use `hwnd()` only with APIs that accept a borrowed parent or target handle.
- Use `client_size()` after creation to size the initial swap chain or child content.
- Forward resize callbacks to `Controller::set_bounds` for
  [`windows-webview`](windows-webview.md), or resize the relevant swap-chain buffers.
- A DirectComposition host may need an extended style such as
  `WS_EX_NOREDIRECTIONBITMAP`; obtain the constant from the consuming bindings and pass its raw
  value to `ex_style`.
- Raw input, paint, keyboard, mouse, DPI, and position behavior can be implemented through
  `on_message`. This crate intentionally does not project message-specific argument types.

## Samples

| Sample | What to study |
| --- | --- |
| [`create_window`](../../crates/samples/windows/samples) | Basic creation and `run_with`. |
| [`window_message`](../../crates/samples/windows/samples) | Paint, mouse, and keyboard messages. |
| [`standalone`](../../crates/samples/canvas/standalone) | Swap-chain hosting and resize flow. |
| [`direct2d`](../../crates/samples/windows/direct2d) | Rendering only while visible. |
| [`direct3d12`](../../crates/samples/windows/direct3d12) | Binding a swap chain to the handle. |
| [`dcomp`](../../crates/samples/windows/dcomp) | Composition, custom style, and DPI. |
| [`webview`](../../crates/samples/webview/samples) | Controller lifetime and resize flow. |

---

## Internal documentation

This section is for contributors to `windows-window`.

`src/bindings.rs` is generated by `tool_bindings` from
`crates/tools/bindings/src/window.txt`. It contains the minimal flat Win32 surface needed for class
registration, creation, DPI setup, destruction, and message dispatch. The hand-written
`window.rs` depends only on [`windows-core`](windows-core.md).

One class is registered lazily for the process. A boxed state containing optional message and
resize handlers is stored in `GWLP_USERDATA` after `CreateWindowExW`. `wndproc` removes the state
on `WM_NCDESTROY`; `Window::drop` checks whether the handle is still live before destroying it.

Before invoking a callback, `wndproc` takes both handlers out of state. After the callback it reads
`GWLP_USERDATA` again because synchronous handling may have destroyed the window and freed the
state. It restores the handlers only when the state still exists. Keep this ordering when changing
dispatch behavior.

The crate does not catch panics in `wndproc`, add message-specific wrappers, or coordinate several
top-level windows. Those boundaries keep the crate small and its ownership rules explicit.
