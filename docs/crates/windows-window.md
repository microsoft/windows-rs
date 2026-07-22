# windows-window

> Minimal window creation and message-loop support for hosting graphics and other content against a raw `HWND`.

- 📦 Not published to crates.io
- 🚀 [Getting started](../../crates/libs/window/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/window)

`windows-window` provides just enough Win32 windowing to open a top-level
window and run a message loop, so crates and samples that need a host window —
[`windows-canvas`](windows-canvas.md) swap chains, WebView2 controllers,
Direct2D/Direct3D rendering — don't have to depend on the full
[`windows`](windows.md) crate or hand-roll a `windows-bindgen` build script.

It is intentionally small: a `Window` with an optional message handler and resize
callback, plus `run`, `run_with`, and `quit`. All Win32 details are private; you
work with a safe `Window` and the raw `HWND` it exposes for interop.

## Getting started

```rust,ignore
use windows_window::*;

let window = Window::new("Hello")
    .size(800, 600)
    .on_resize(|width, height| {
        // forward the new client size to your renderer
    })
    .create()?;

// Hand window.hwnd() to windows-canvas, WebView2, Direct2D, etc.
let hwnd = window.hwnd();

run(); // blocking, event-driven message loop
```

## The API

- **`Window::new(title)`** returns a `WindowBuilder`. Configure it with
  `.size(width, height)`, `.style(..)`, `.ex_style(..)`, `.on_message(..)`, and
  `.on_resize(..)`, then `.create()` to open and show the window.
- **`Window::hwnd()`** returns the raw `*mut c_void` handle for interop.
- **`Window::client_size()`** returns the current client area as `(width, height)`.
- **`style(WS_*)` / `ex_style(WS_EX_*)`** override the window styles (raw `u32`).
  They default to `WS_OVERLAPPEDWINDOW` and none — pass Win32 style constants for
  custom chrome (e.g. `WS_EX_NOREDIRECTIONBITMAP` for a DirectComposition target).
- **`on_message(|hwnd, message, wparam, lparam| -> Option<isize>)`** handles raw
  window messages; return `Some(result)` to handle a message or `None` to fall
  through to default processing.
- **`on_resize(|width, height|)`** is a convenience for `WM_SIZE`, giving the new
  client size in pixels.

Handlers are detached while they run, so a handler that triggers reentrant
dispatch (for example calling `SetWindowPos`, which synchronously sends
`WM_WINDOWPOSCHANGING` / `WM_SIZE`) won't re-enter itself — the nested messages
fall through to default processing. This keeps shared state behind an
`Rc<RefCell<..>>` safe from reentrant borrows.

`wndproc` invokes your handlers directly, without a `catch_unwind`, so a panic
that escapes a handler aborts the process at the `extern "system"` boundary
rather than unwinding into the Win32 frames that called it — much like a C++
`noexcept` function terminating. This is intentional; the crate does not try to
recover from handler panics. If a handler needs to survive its own panics, wrap
its body in `std::panic::catch_unwind` (usually via `AssertUnwindSafe`, since
handler closures capture mutable state).

## The message loop

- **`run()`** — a blocking, event-driven loop (`GetMessage`) until the window
  closes. Best for interactive apps such as a WebView2 host.
- **`run_with(render)`** — calls `render` whenever the queue is empty; `render`
  returns `Ok(true)` to keep rendering immediately (continuous animation) or
  `Ok(false)` to wait for the next message (event-driven/occluded). Returns
  `Ok(())` when the window closes, or early if `render` returns an error.
- **`quit()`** — posts a quit message. An unhandled `WM_DESTROY` posts one
  automatically, so closing the window ends the loop without extra code.
- **`pump()`** — dispatches all currently-pending messages without blocking and
  returns `false` if a quit was received. Lets a caller drive the loop while
  waiting on an external condition, such as pumping until an asynchronous
  callback completes.

## Samples

- [`create_window`](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/samples/examples/create_window.rs)
  the minimal case — open a window and pump messages with `run_with`, depending
  only on `windows-window` (no `windows` crate).
- [`window_message`](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/samples/examples/window_message.rs)
  handles raw messages (`WM_PAINT`, mouse, keyboard) via `on_message`, with
  direct access to `wparam`/`lparam`.
- [`canvas/standalone`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas/standalone)
  hosts a `windows-canvas` swap chain in a `windows-window` window driven by
  `run_with`.
- [`windows/direct2d`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows/direct2d)
  drives a Direct2D swap chain with `run_with`, rendering only while visible.
- [`windows/direct3d12`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows/direct3d12)
  binds a Direct3D 12 swap chain to `window.hwnd()` and renders on `WM_PAINT`.
- [`windows/dcomp`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows/dcomp)
  hosts a DirectComposition target (custom `ex_style`) and pumps with `run`,
  relying on reentrancy-safe handlers for its DPI handling.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-window`**.

### How it's built

`src/bindings.rs` is generated by `tool_bindings` from
`crates/tools/bindings/src/window.txt` — minimal, flat bindings for the
`RegisterClassW` / `CreateWindowExW` / message-pump surface plus the handful of
structs and constants they need. Everything in `window.rs` (the `Window`,
`WindowBuilder`, and the message loop) is hand-written over those bindings and
depends only on [`windows-core`](windows-core.md), so the crate stays fast to
build.

### Design

- **One window class** — a single process-wide window class is registered lazily;
  all windows share it and route messages through one `wndproc`.
- **State in `GWLP_USERDATA`** — the boxed message/resize handlers are stored in
  the window's user data, set after `CreateWindowExW` returns, and reclaimed on
  `WM_NCDESTROY`. `Window::drop` calls `DestroyWindow` if the window still exists.
- **Reentrancy-safe** — `wndproc` moves the handlers out of the state before
  invoking them and restores them afterward, so a handler that pumps nested
  messages can't alias (or, behind `RefCell`, panic on re-borrowing) itself.
- **No `catch_unwind`** — handlers are invoked directly, so a panic aborts at the
  `extern "system"` boundary (see [The API](#the-api) above). This is deliberate:
  recovering from handler panics is out of scope for this crate.
- **Not a kitchen sink** — the crate covers window creation, a resize hook, and a
  message loop only. Anything beyond that (menus, input, multiple monitors, etc.)
  belongs in the consuming app or a focused [`windows-bindgen`](windows-bindgen.md)
  binding.

### Testing

`cargo check -p windows-window` and `cargo clippy -p windows-window
--all-targets`. The `canvas/standalone` sample exercises it end to end.

### Future work

- **Adopt the family `use super::*;` import convention.** The other companion crates
  ([`windows-reactor`](windows-reactor.md),
  [`windows-composition`](windows-composition.md),
  [`windows-canvas`](windows-canvas.md)) glob crate-root re-exports with a single
  `use super::*;` per module instead of `use crate::…`. This crate is small and has
  only a stray `use crate::…`; normalizing it (re-exporting any shared internals at
  the crate root as `pub(crate)` so the glob covers them) keeps the family
  consistent.
