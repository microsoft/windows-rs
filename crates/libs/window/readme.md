## windows-window

The [windows-window](https://crates.io/crates/windows-window) crate provides basic window creation
and message-loop support for Canvas, WebView2, and custom rendering.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-window.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-window]
version = "0.100"
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

    // `window.hwnd()` can be handed to windows-canvas, WebView2, Direct2D, etc.
    println!("created window {:?}", window.hwnd());

    run();
    Ok(())
}
```
