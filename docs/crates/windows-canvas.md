# windows-canvas

> A safe, fast 2D graphics library backed by Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC.

- 📦 Not published to crates.io
- 🚀 [Getting started](../../crates/libs/canvas/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/canvas)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)

`windows-canvas` wraps the DirectX graphics stack behind safe Rust types. A
`GpuDevice` owns the underlying Direct3D/Direct2D devices; from it you create a
`SwapChain` to present frames. Each frame, `begin_draw` yields a `DrawingSession`
with methods to clear, draw shapes and text, and fill regions with brushes and
gradients. Device loss is recovered automatically.

There are two ways to use it: hosted inside a [`windows-reactor`](windows-reactor.md)
window (simplest — recommended), or standalone against your own `HWND`.

## Getting started — inside a reactor window

Enable the `reactor` feature and call `animated_canvas(draw)`. It returns a
`SwapChainPanel` element that creates the device and swap chain, handles resize
and DPI/scale changes, calls `begin_draw`/`present` each frame, and recovers from
device loss for you. Your closure receives a `DrawContext` that derefs to the
frame's `DrawingSession`, so every drawing method is available directly.

```toml
[dependencies]
windows-canvas = { version = "...", features = ["reactor"] }
windows-reactor = "..."
```

```rust,ignore
use windows_canvas::*;

let panel = animated_canvas(|ctx| {
    ctx.clear(ColorF::CORNFLOWER_BLUE);

    let center = Vector2::new(ctx.width / 2.0, ctx.height / 2.0);
    let brush = ctx.create_solid_brush(ColorF::WHITE);
    ctx.fill_ellipse(&Ellipse::circle(center, 80.0), &brush);
});
```

`ctx.width` / `ctx.height` give the surface size in DIPs. `ctx.device()` and
`ctx.device_changed()` let you (re)create cached resources such as bitmaps when
the device is recreated after loss.

## Getting started — standalone

With your own window handle, create a `GpuDevice`, then a swap chain for the
`HWND`, and drive the frame loop yourself:

```rust,ignore
use windows_canvas::*;

let device = GpuDevice::new()?;                 // or GpuDevice::new_warp()? (software)
let chain = unsafe { device.create_swap_chain_for_hwnd(hwnd, width, height)? };

// each frame / on paint:
let session = chain.begin_draw()?;
session.clear(ColorF::DARK_SLATE_BLUE);

let brush = session.create_solid_brush(ColorF::CORNFLOWER_BLUE);
session.fill_ellipse(&Ellipse::circle(Vector2::new(200.0, 150.0), 100.0), &brush);

drop(session);                                  // Drop ends the draw
chain.present()?;
```

On resize call `chain.resize(width, height)`; `chain.set_dpi(..)` and
`chain.set_composition_scale(..)` keep rendering crisp. `chain.is_device_lost()`
reports loss if you want to react explicitly.

## Drawing basics

Everything below is a method on the `DrawingSession` (and therefore on
`DrawContext` too).

- **Clear / fill:** `clear(ColorF)`, `fill_rect(&Rect, &paint)`,
  `fill_rounded_rect(&RoundedRect, &paint)`, `fill_ellipse(&Ellipse, &paint)`,
  `fill_path(&Path, &paint)`.
- **Stroke:** `draw_rect`, `draw_ellipse`, `draw_line`, `draw_rounded_rect`,
  `draw_path`, each with a `*_styled` variant taking a `StrokeStyle`.
- **Brushes:** `create_solid_brush(ColorF)`,
  `create_linear_gradient(start, end, &[GradientStop])`,
  `create_radial_gradient(center, rx, ry, &[GradientStop])`. A `Brush` color can
  be updated and reused with `brush.set_color(..)`.

Colors are `ColorF`: `ColorF::rgb(r, g, b)`, `ColorF::new(r, g, b, a)`,
`ColorF::from_rgb8(..)` / `from_rgba8(..)`, plus constants like `WHITE`, `BLACK`,
`RED`, `CORNFLOWER_BLUE`, `DARK_SLATE_BLUE`, `TRANSPARENT`.

## Geometry and paths

`Rect::new(left, top, right, bottom)` / `Rect::from_xywh(..)`,
`Ellipse::new(center, rx, ry)` / `Ellipse::circle(center, r)`,
`RoundedRect::new(rect, rx, ry)` / `RoundedRect::uniform(rect, r)`. Centers and
points use `Vector2` (re-exported from `windows-numerics`).

Freeform paths are built with `PathBuilder::new(&device)`, then `.begin(start)`
(filled) or `.begin_hollow(start)` (open), adding segments with `line_to`,
`bezier_to`, and finishing with `close` or `end_open`; `build()` yields a `Path`.

Strokes are configured with `StrokeStyleBuilder` — `start_cap`, `end_cap`,
`caps`, `line_join`, `miter_limit`, `dash_style`, `dash_offset` (see `CapStyle`,
`LineJoin`, `DashStyle`).

## Text

Create a `TextFormat`, then `draw_text(text, &format, &Rect, &paint)`:

```rust,ignore
let format = TextFormat::new("Segoe UI", 24.0)
    .with_alignment(TextAlignment::Center)
    .with_paragraph_alignment(ParagraphAlignment::Center);
session.draw_text("Hello", &format, &Rect::from_xywh(0.0, 0.0, w, h), &brush);
```

`TextFormat::new_bold(..)` and `with_weight(family, size, FontWeight::BOLD)` set
weight; `TextAlignment` and `ParagraphAlignment` control horizontal/vertical
placement.

## Transforms, bitmaps, and effects

- **Transforms:** `set_transform(&Matrix3x2)` / `get_transform()`, or the scoped
  `with_transform(&matrix, |s| { .. })`. Use `Matrix3x2::translation(..)` and
  `Matrix3x2::rotation(..)` (from `windows-numerics`).
- **Bitmaps:** `load_bitmap(path)` then `draw_bitmap(&bitmap, &Rect, opacity)` or
  `draw_image(&bitmap)`.
- **Off-screen targets:** `create_bitmap_target()` plus
  `with_target(&bitmap, |s| { .. })` render into a bitmap; `create_shadow(&bitmap)`
  and `draw_effect(&effect)` add drop shadows and other effects.

## Samples

The [`crates/samples/canvas`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)
tree:

- **`standalone`** — the HWND path end to end: device, swap chain, draw loop,
  filled ellipse, centered text.
- **`minimal`** — `canvas_minimal::run()` wraps `animated_canvas` in a reactor
  window, with an `examples/` folder of focused snippets: `hello`, `color`,
  `brush`, `gradient`, `lines`, `stroke`, `shapes`, `path`, `text`, `bitmap`,
  `transform`.
- **`circles`** — animated circles with brush reuse and a text label.
- **`clock`** — an animated analog clock combining transforms, stroke styles,
  shadows, bitmap targets, and text.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-canvas`**.

### How it's built

`src/bindings.rs` is generated by `tool_bindings` from
`crates/tools/bindings/src/canvas.txt` — minimal, flat bindings for Direct2D,
Direct3D 11, DXGI, DirectWrite, and WIC. The safe wrappers (`GpuDevice`,
`SwapChain`, `DrawingSession`, geometry, and text types) are hand-written. The
optional `reactor` feature integrates with `windows-reactor` through
`animated_canvas`.

### Design

- **No WinRT layer** — the safe types wrap Direct2D/Direct3D/DXGI directly, not
  Win2D's WinRT projections.
- **Single-threaded rendering** — a `SwapChain` owns one D2D device context; there
  is no context pool. Rendering happens on whichever thread owns the swap chain.
- **One built-in render loop** — `animated_canvas` (reactor feature) drives frames
  on the UI thread via `CompositionTarget::Rendering`. There is no dedicated
  render-thread variant.
- **Automatic device-lost recovery** — `device_lost.rs` classifies the DXGI/D2D
  device-lost codes; `EndDraw`/`Present` set a flag, and the swap chain recreates
  its device and resources on the next frame.

### Testing

Tests render with the WARP software rasterizer, so they need no GPU:
`cargo test -p windows-canvas`.
