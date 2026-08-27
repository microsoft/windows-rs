## Windows Canvas

Windows Canvas wraps Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC for 2D drawing.

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

Core types include `GpuDevice`, `SwapChain`, `DrawingSession`, `PathBuilder`, `TextFormat`, and
`Bitmap`. To host a canvas in [`windows-reactor`][reactor-guide], enable this crate's `reactor`
feature. `animated_canvas()` draws each frame on the UI thread and recovers from device loss. See
the [canvas guide][canvas-guide] for demand-driven surfaces, recoverable errors, and
`CanvasImageSource` integration.

[canvas-guide]: https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-canvas.md
[reactor-guide]: https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md
