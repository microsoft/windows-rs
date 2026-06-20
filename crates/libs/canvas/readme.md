## Windows Canvas

Windows Canvas is a safe, fast 2D graphics library for Rust developers, backed by Direct2D (with Direct3D 11, DXGI, DirectWrite, and WIC). All COM interfaces are private implementation details — you work only with Rust types.

> **Status:** experimental and pre-release (`0.0.0`). The API is evolving and the crate is not yet published to crates.io.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Canvas guide](https://github.com/microsoft/windows-rs/blob/master/docs/guide/windows-canvas.md)

```rust,ignore
use windows_canvas::*;

let device = GpuDevice::new()?;
let mut chain = device.create_swap_chain(800, 600)?;

let session = chain.begin_draw()?;
session.clear(Color::CORNFLOWER_BLUE);
session.fill_ellipse(&Ellipse::circle(center, 50.0), &brush);
drop(session);
chain.present()?;
```

Core types: `GpuDevice`, `SwapChain`, `DrawingSession`, `PathBuilder`, `TextFormat`, `Bitmap`, and the `Paint` trait. With the `reactor` feature enabled, `animated_canvas()` drives a per-frame render loop on the UI thread with automatic device-lost recovery. See the [canvas guide](https://github.com/microsoft/windows-rs/blob/master/docs/guide/windows-canvas.md) for the full API and samples.
