## windows-canvas

Windows Canvas wraps Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC for 2D drawing.

The core crate is host-neutral. Enable `system` for the typed `windows-window` swap-chain helper,
`reactor` for Canvas controls in Reactor, and `composition` to draw into Composition surfaces. The
`system` and `reactor` host adapters can coexist unless `composition` selects their incompatible
Composition backends.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-canvas.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-canvas]
version = "0.100"
```

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
