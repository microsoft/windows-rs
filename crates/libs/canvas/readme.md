# windows-canvas

Safe 2D drawing for shapes, paths, text, images, and effects, built on Direct2D.
Canvas draws a frame from commands; use [windows-composition](https://docs.rs/windows-composition)
when a retained visual tree is a better fit.

- 📦 [crates.io](https://crates.io/crates/windows-canvas)
- 📖 [docs.rs](https://docs.rs/windows-canvas)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/canvas)

## Draw a frame

```toml
[dependencies]
windows-canvas = "0.100"
```

```rust,no_run
use windows_canvas::*;

fn draw_frame(chain: &mut SwapChain) -> Result<bool> {
    let session = chain.begin_draw()?;
    session.clear(ColorF::BLACK);
    let brush = session.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    session.fill_rect(&Rect::from_xywh(20.0, 20.0, 200.0, 120.0), &brush);
    drop(session);
    chain.present()
}
```

Finish and drop the drawing session before presenting. Clear or fully cover each frame unless
retaining old pixels is intentional.

## Choose a host

| Feature | Use |
| --- | --- |
| `system` | Host a swap chain in a `windows-window` window. |
| `reactor` | Return a Canvas view from a Reactor component. |
| `composition` | Draw into Composition surfaces with a matching host feature. |

The crate has no default host. You can combine `system` and `reactor` without `composition`.
With `composition`, select exactly one of those host features.

Start with the [standalone sample][standalone] for window creation and a frame loop. A standalone
host owns resizing and recovery when `present` returns `Ok(false)`.

The Reactor host manages sizing, display scale, and recovery. Use `canvas` for creation/resize
redraws, `animated_canvas` for every frame, or `canvas_invalidated` with an `Invalidator` for
input-driven drawings. Rebuild cached device-dependent resources when `ctx.device_changed()` is
true.

## Add drawing operations

Use `PathBuilder` for geometry, `TextLayout` for reusable or measured text, and `load_bitmap` for
images. `with_transform` scopes a transform without leaving it active for later commands.
Drawing dimensions are in device-independent pixels.

See the [transform][transform], [text layout][text], and [circles][circles] samples.

[standalone]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas/standalone
[transform]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas/transform
[text]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas/text_layout
[circles]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas/circles
