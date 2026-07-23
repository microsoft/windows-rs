## Windows Canvas

Windows Canvas is a safe, fast 2D graphics library for Rust developers, backed by Direct2D (with Direct3D 11, DXGI, DirectWrite, and WIC). All COM interfaces are private implementation details - you work only with Rust types.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Canvas guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-canvas.md)

```rust,no_run
use windows_canvas::*;

fn main() -> Result<()> {
    let device = GpuDevice::new_warp()?;
    let mut swap_chain = device.create_swap_chain(640, 480)?;

    let session = swap_chain.begin_draw()?;
    session.clear(ColorF::CORNFLOWER_BLUE);

    let brush = session.create_solid_brush(ColorF::rgb(1.0, 0.5, 0.0))?;
    session.fill_rect(&Rect::new(20.0, 20.0, 200.0, 120.0), &brush);
    drop(session);

    swap_chain.present()?;
    Ok(())
}
```

Core types: `GpuDevice`, `SwapChain`, `DrawingSession`, `PathBuilder`, `TextFormat`, `Bitmap`, and the `Paint` trait. To host a canvas inside a [`windows-reactor`](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md) window, enable reactor's `canvas` feature: `animated_canvas()` drives a per-frame render loop on the UI thread with automatic device-lost recovery, and `CanvasImageSource` provides an on-demand surface that repaints only when its data changes. See the [canvas guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-canvas.md) for the full API and samples.
