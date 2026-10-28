# windows-window

Create a desktop window and run its message loop. Use it to host graphics, WebView2, or another
API that accepts an `HWND`; it is not a control toolkit.

- 📦 [crates.io](https://crates.io/crates/windows-window)
- 📖 [docs.rs](https://docs.rs/windows-window)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows/window-message)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/window)

## Get started

```toml
[dependencies]
windows-window = "0.100"
```

```rust,no_run
use windows_window::*;

fn main() -> Result<()> {
    let window = Window::new("Hello")
        .client_size(800, 600)
        .on_resize(|width, height| {
            println!("resized to {width} x {height}");
        })
        .create()?;

    println!("created window {:?}", window.hwnd());
    run();
    Ok(())
}
```

## Attach content and choose a loop

Create and drive the window on its owning thread. Use `hwnd()` as a borrowed handle and
`client_size()` for the initial content size. Register `on_resize` to update hosted bounds and
`on_close` to release content that requires a live parent.

Builder `size` sets outer dimensions; builder `client_size` sets content dimensions.
Resize callbacks report physical pixels. Set any custom DPI policy before creating the first window.

| Loop | Use |
| --- | --- |
| `run()` | Event-driven content; waits for messages. |
| `run_with(render)` | Return `Ok(true)` for another frame or `Ok(false)` to wait. |
| `pump()` | An external loop owns the wait; returns false after quit. |

Closing a window quits by default. Use `quit_on_close(false)` when application and window lifetimes
differ, then call `quit()` explicitly. Dropping a window destroys it without posting quit.

`on_message` returns `Some(result)` only for fully handled messages; `None` allows normal
processing and focused callbacks. Handling `WM_CLOSE` yourself bypasses `on_close`.
Nested messages use default processing rather than re-entering your handler. Keep callbacks short
and do not let panics escape them.

See [standalone Canvas][canvas] and [windows-webview](https://docs.rs/windows-webview) for hosts.

[canvas]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas/standalone
