## Windows Window

Windows Window is a small, fast-to-build crate that provides the basic
window creation and message-loop facilities needed to host content such as
[`windows-canvas`](https://crates.io/crates/windows-canvas) swap chains,
WebView2 controllers, or Direct2D/Direct3D rendering - without pulling in the
full `windows` crate or hand-rolling `windows-bindgen` build scripts.

It is minimal: a `Window` with a message handler and an optional
resize callback, plus a message loop. All Win32 details are private
implementation; you work with a safe Rust type and the raw `HWND` it exposes.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)

```rust,no_run
use windows_window::*;

fn main() -> Result<()> {
    let window = Window::new("Hello")
        .size(800, 600)
        .on_resize(|width, height| {
            println!("resized to {width} x {height}");
        })
        .create()?;

    // `window.hwnd()` can be handed to windows-canvas, WebView2, Direct2D, etc.
    println!("created window {:?}", window.hwnd());

    run();
    Ok(())
}
```

`Window::hwnd()` returns the raw `*mut c_void` handle for interop. Use `run()`
for an event-driven message loop, `run_with(render)` to drive a continuous
render loop (for animation), and `quit()` to post a quit message. An unhandled
`WM_DESTROY` automatically posts a quit message.
