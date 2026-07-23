# windows-canvas

> A safe, fast 2D graphics library backed by Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC.

- Not published to crates.io
- [Getting started](../../crates/libs/canvas/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/canvas)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)

`windows-canvas` wraps the DirectX graphics stack behind safe Rust types. A
`GpuDevice` owns the Direct3D and Direct2D devices. From it, you create a
`SwapChain` to present frames. Each frame, `begin_draw` returns a
`DrawingSession`. Use it to clear, draw shapes and text, and fill regions.

Use it inside a [`windows-reactor`](windows-reactor.md) window, or use it with
your own `HWND`.

## Getting started inside a reactor window

Enable the reactor `canvas` feature. Then call `animated_canvas(draw)`. It
returns a `SwapChainPanel` element. The element creates the device and swap
chain. It handles resize, DPI changes, and device loss.

The closure receives a `DrawContext`. It derefs to the frame `DrawingSession`.
Thus, all drawing methods are available on `ctx`.

```toml
[dependencies]
windows-reactor = { version = "...", features = ["canvas"] }
windows-canvas = "..."
```

```rust,ignore
use windows_canvas::*;   // drawing types: ColorF, Ellipse, Vector2, DrawingSession
use windows_reactor::*;  // the harness: animated_canvas, DrawContext

let panel = animated_canvas(|ctx| {
    ctx.clear(ColorF::CORNFLOWER_BLUE);

    let center = Vector2::new(ctx.width / 2.0, ctx.height / 2.0);
    let Ok(brush) = ctx.create_solid_brush(ColorF::WHITE) else { return };
    ctx.fill_ellipse(&Ellipse::circle(center, 80.0), &brush);
});
```

`ctx.width` and `ctx.height` give the surface size in DIPs. Use `ctx.device()`
and `ctx.device_changed()` for cached resources. Recreate bitmaps and brushes
when the device changes.

## Getting started standalone

With a [`windows-window`](windows-window.md) `Window`, create a `GpuDevice`.
Then create a swap chain for the window. You drive the frame loop.

For a raw handle from another source, `create_swap_chain_for_hwnd` is the
`unsafe` escape hatch.

```rust,ignore
use windows_canvas::*;
use windows_window::Window;

let device = GpuDevice::new()?;                 // or GpuDevice::new_warp()? (software)
let chain = device.create_swap_chain_for_window(&window, width, height)?;

// each frame / on paint:
let session = chain.begin_draw()?;
session.clear(ColorF::DARK_SLATE_BLUE);

let brush = session.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
session.fill_ellipse(&Ellipse::circle(Vector2::new(200.0, 150.0), 100.0), &brush);

drop(session);                                  // Drop ends the draw
chain.present()?;
```

On resize, call `chain.resize(width, height)`. Use `chain.set_dpi(..)` and
`chain.set_composition_scale(..)` for sharp output. `chain.is_device_lost()`
reports device loss.

## Getting started with an on-demand surface

`animated_canvas` presents a new frame each vsync. Use `CanvasImageSource` for
static content. It redraws only when you call `draw`.

Enable the reactor `canvas` feature. Create a `CanvasImageSource` from a device
that you own. Display it with a reactor `Image` widget.

```rust,ignore
use windows_canvas::*;   // ColorF, Ellipse, Vector2, DrawingSession
use windows_reactor::*;  // CanvasImageSource, Image

// Create once, on the UI thread, from a device you own.
let surface = CanvasImageSource::new(&device, 320.0, 320.0, scale)?;

// Redraw only when the data changes.
surface.draw(ColorF::CORNFLOWER_BLUE, |session| {
    let Ok(brush) = session.create_solid_brush(ColorF::WHITE) else { return };
    session.fill_ellipse(&Ellipse::circle(Vector2::new(160.0, 160.0), 96.0), &brush);
})?;

// Display it with the reactor `Image` widget.
let image = Image::new(surface.image_source());
```

`new(device, width, height, scale)` takes a size in DIPs. It uses the host DPI
scale to allocate physical pixels. Drawing inside `draw` uses DIPs and origin
`(0, 0)`.

`draw` returns `Ok(false)` on device loss. Create a new `GpuDevice`, call
`set_device`, and draw again.

Get the `scale` from the reactor `Image`. `Image::on_mounted` returns an
`ImageHandle`. `ImageHandle::on_rasterization_scale_changed` reports the host
DPI scale. Rebuild the surface when the scale changes. See the `image_source`
sample.

## Getting started with a composition surface

Enable the `composition` feature to draw into a
[`windows-composition`](windows-composition.md) `CompositionDrawingSurface`. The
app owns the composition graph. The bridge only lends Direct2D drawing.

```toml
[dependencies]
windows-canvas = { version = "...", features = ["composition"] }
windows-composition = { version = "...", features = ["system"] }
```

```rust,ignore
use windows_canvas::{CanvasCompositionExt, ColorF, Ellipse, GpuDevice, Vector2};

let graphics = device.create_graphics_device(&compositor)?;   // adopt the D2D device
let surface = graphics.create_drawing_surface(256.0, 256.0)?; // premultiplied BGRA
sprite.set_brush(&compositor.create_surface_brush(&surface)); // paint a visual

surface.draw(|session| {
    session.clear(ColorF::CORNFLOWER_BLUE);
    let Ok(brush) = session.create_solid_brush(ColorF::WHITE) else { return };
    session.fill_ellipse(&Ellipse::circle(Vector2::new(128.0, 128.0), 96.0), &brush);
})?;
```

`draw` runs inside the surface native `BeginDraw` and `EndDraw` bracket. It
returns `Ok(false)` on device loss. Recreate the device, graphics device, and
surface. Then draw again.

There is no implicit clear. Clear or draw over the full surface. Coordinates are
pixels with origin `(0, 0)`. The backing-atlas offset is applied for you. This
path is system-only. See the
[`composition/canvas`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition/canvas)
sample.

## Drawing basics

These methods are on `DrawingSession` and `DrawContext`.

- **Clear and fill:** `clear(ColorF)`, `fill_rect(&Rect, &paint)`,
  `fill_rounded_rect(&RoundedRect, &paint)`, `fill_ellipse(&Ellipse, &paint)`,
  `fill_path(&Path, &paint)`.
- **Stroke:** `draw_rect`, `draw_ellipse`, `draw_line`, `draw_rounded_rect`, and
  `draw_path`. Each has a `*_styled` variant that takes a `StrokeStyle`.
- **Brushes:** `create_solid_brush(ColorF)`,
  `create_linear_gradient(start, end, &[GradientStop])`, and
  `create_radial_gradient(center, rx, ry, &[GradientStop])`. Use
  `brush.set_color(..)` to update a solid brush.

Colors are `ColorF`. Use `ColorF::rgb(r, g, b)`, `ColorF::new(r, g, b, a)`,
`ColorF::from_rgb8(..)`, or `ColorF::from_rgba8(..)`. Constants include `WHITE`,
`BLACK`, `RED`, `CORNFLOWER_BLUE`, `DARK_SLATE_BLUE`, and `TRANSPARENT`.

## Geometry and paths

Use `Rect::new(left, top, right, bottom)` or `Rect::from_xywh(..)`. Use
`Ellipse::new(center, rx, ry)` or `Ellipse::circle(center, r)`. Use
`RoundedRect::new(rect, rx, ry)` or `RoundedRect::uniform(rect, r)`. Centers and
points use `Vector2`, re-exported from `windows-numerics`.

Build freeform paths with `PathBuilder::new(&device)`. Start with
`.begin(start)` for a filled figure, or `.begin_hollow(start)` for an open
figure. Add segments with `line_to` and `bezier_to`. Finish with `close` or
`end_open`. Then call `build()` to get a `Path`.

For a closed polygon, use `PathBuilder::new(&device)?.polygon(points)`.

Use `Path::fill_contains_point`, `Path::stroke_contains_point`, and
`Path::compute_bounds` for geometry queries.

Configure strokes with `StrokeStyleBuilder`. It sets `start_cap`, `end_cap`,
`caps`, `line_join`, `miter_limit`, `dash_style`, and `dash_offset`. See
`CapStyle`, `LineJoin`, and `DashStyle`.

## Text

Create a `TextFormat`, then call `draw_text(text, &format, &Rect, &paint)`:

```rust,ignore
let format = TextFormat::new("Segoe UI", 24.0)
    .with_alignment(TextAlignment::Center)
    .with_paragraph_alignment(ParagraphAlignment::Center);
session.draw_text("Hello", &format, &Rect::from_xywh(0.0, 0.0, w, h), &brush);
```

`TextFormat::new_bold(..)` and `with_weight(family, size, FontWeight::BOLD)` set
weight. `TextAlignment` and `ParagraphAlignment` control placement.

## Transforms, bitmaps, and effects

- **Transforms:** Use `set_transform(&Matrix3x2)` and `transform()`. Use
  `with_transform(&matrix, |s| { .. })` for scoped transforms. Matrix types come
  from `windows-numerics`.
- **Bitmaps:** Use `load_bitmap(path)` to decode an image file. Use
  `create_bitmap(pixels, width, height)` to upload premultiplied BGRA pixels.
  Use `create_bitmap_with_alpha` to select the `AlphaMode`. Then use
  `draw_bitmap(&bitmap, &Rect, opacity)` or `draw_image(&bitmap)`.
- **Off-screen targets:** Use `create_bitmap_target()` with
  `with_target(&bitmap, |s| { .. })`. Use `GpuDevice::create_render_target` for
  a target with CPU readback. Use `create_shadow(&bitmap)` and
  `draw_effect(&effect)` for drop shadows and effects.

## Samples

The [`crates/samples/canvas`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)
tree contains these samples:

- **`standalone`**: creates a device and swap chain for an HWND.
- **`shared_device`**: shares one `GpuDevice` across many surfaces.
- **`samples`**: runs focused drawing examples in a reactor window.
- **`circles`**: animates circles and reuses brushes.
- **`clock`**: draws an animated analog clock with transforms and shadows.
- **`image_source`**: redraws a `CanvasImageSource` only when data changes.
- **`chart`**: hosts an on-demand swap chain on a `SwapChainPanel`.
- **`readback`**: renders off-screen and reads pixels back to the CPU.
- **`hit_test`**: tests whether the pointer is inside a filled `Path`.
- **`editor`**: combines reactor pointer events with canvas geometry queries.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is not
needed to use `windows-canvas`.

### How it's built

`src/bindings.rs` is generated by `tool_bindings` from
`crates/tools/bindings/src/canvas.txt`. It contains minimal, flat bindings for
Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC. It also uses the reference
`Windows.winmd` for WinRT numerics types.

The safe wrappers are hand-written. They include `GpuDevice`, `SwapChain`,
`DrawingSession`, geometry, text, bitmaps, brushes, effects, and render targets.

The reactor integration lives in [`windows-reactor`](windows-reactor.md). It is
behind the reactor `canvas` feature. The optional `composition` feature connects
this crate to [`windows-composition`](windows-composition.md).

### Design

- **No WinRT layer:** The safe types wrap Direct2D, Direct3D, DXGI, DirectWrite,
  and WIC directly.
- **Single-threaded rendering:** A `SwapChain` owns one D2D device context.
  Rendering happens on the thread that owns the swap chain.
- **Continuous rendering:** `animated_canvas` drives frames on the UI thread with
  `CompositionTarget::Rendering`.
- **On-demand image source:** `CanvasImageSource` draws into a WinUI
  `SurfaceImageSource` only when requested. It uses a borrowed `DrawingSession`.
- **Composition bridge:** `CanvasCompositionExt::draw` draws Direct2D content
  into a `CompositionDrawingSurface`. It also uses a borrowed `DrawingSession`.
- **Device-lost recovery:** `device_lost.rs` classifies DXGI and D2D loss codes.
  `EndDraw` and `Present` set a flag. The next frame recreates the device and
  resources.
- **WARP fallback:** `GpuDevice::new_or_warp()` tries hardware first. It falls
  back to the WARP software rasterizer when no GPU is available.
- **Shareable device:** `GpuDevice` is `Clone`. A clone shares the same Direct3D,
  Direct2D, DXGI, and DirectWrite objects.

### Reactor integration

The reactor harness lives in [`windows-reactor`](windows-reactor.md). It exports
`animated_canvas`, `CanvasImageSource`, `CanvasSwapChain`, and `DrawContext`
under the reactor `canvas` feature.

The dependency direction is `windows-reactor[canvas]` to `windows-canvas`.
Reactor owns the WinUI element harness. That includes `SwapChainPanel`,
`SurfaceImageSource`, the render loop, resize, DPI, and unmount cleanup.

This crate exposes the drawing surface that the bridge needs. It includes
`GpuDevice`, `SwapChain`, borrowed `DrawingSession` constructors, and the
`ID2D1DeviceContext` interop type. This crate has no `windows-reactor`
dependency.

Input belongs to reactor. Geometry queries belong to canvas. Pointer events use
DIPs. Apps map those DIPs into canvas space with their own transform.

### Testing

Tests render with the WARP software rasterizer. They need no GPU. The integration
suite lives in the `test_canvas` crate. Run `cargo test -p test_canvas`.