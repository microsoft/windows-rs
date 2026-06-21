# windows-canvas

> A safe, fast 2D graphics library backed by Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC.

- 📦 Not published (experimental, `0.0.0`)
- 🛠 [Internals](../internals/windows-canvas.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/canvas)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)

## Overview

`windows-canvas` wraps the DirectX graphics stack behind safe Rust types. A
`GpuDevice` owns the underlying Direct3D/Direct2D devices; from it you create a
`SwapChain` to present frames. Each frame, `begin_draw` yields a
`DrawingSession` with methods to clear, draw shapes and text, and fill regions
using brushes and gradients. Device-lost is recovered automatically.

> **Status**: experimental and unpublished. The API evolves alongside the
> samples in the repository.

## Example

Draw a filled rectangle into an off-screen swap chain using the WARP software
renderer (so it needs no physical GPU):

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

With the `reactor` feature, `animated_canvas` hosts a continuously redrawing
surface inside a [`windows-reactor`](windows-reactor.md) UI.