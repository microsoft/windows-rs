# windows-canvas

> A safe, fast 2D graphics library backed by Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC.

- 📦 [crates.io](https://crates.io/crates/windows-canvas)
- 📖 [docs.rs](https://docs.rs/windows-canvas)
- 🚀 [Getting started](../../crates/libs/canvas/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/canvas)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)

`windows-canvas` wraps the DirectX graphics stack behind safe Rust types. A `GpuDevice` owns the
Direct3D and Direct2D devices. From it, you create a `SwapChain` to present frames. Each frame,
`begin_draw` returns a `DrawingSession`. Use it to clear, draw shapes and text, and fill regions.

Use it inside a [`windows-reactor`](windows-reactor.md) window, or use it with your own `HWND`.

## Getting started inside a reactor window

Enable the reactor `canvas` feature. Then call `animated_canvas(draw)`. It returns a
`SwapChainPanel` element. The element creates the device and swap chain. It handles resize, DPI
changes, and device loss.

The closure receives a `DrawContext` and returns `Result<()>`, so resource creation inside it can
use `?`. It derefs to the frame `DrawingSession`, so all drawing methods are available on `ctx`.

```toml
[dependencies]
windows-reactor = { version = "0.100", features = ["canvas"] }
windows-canvas = "0.100"
```

See the reactor canvas samples for a complete animated drawing loop.

`ctx.width` and `ctx.height` give the surface size in DIPs. Use `ctx.device()` and
`ctx.device_changed()` for cached resources. Recreate bitmaps and brushes when the device changes.

For content that changes with its size rather than every frame - text, a chart, a diagram - use
`canvas(draw)` instead. It manages the device, swap chain, resize, DPI, and device loss exactly like
`animated_canvas`, but calls `draw` only on the first layout and when the surface resizes or the
display scale changes. When the window is idle, no GPU work happens.

Use `canvas` for content that changes only when its size or scale changes.

Because `draw` runs only on resize, size-dependent resources such as a `TextLayout` fitted to the
client area can be shaped once and cached in a `use_ref`, then rebuilt only when `device_changed`
reports a resize or device loss. See the `text_layout` sample.

When content changes with app state rather than size, drive repaints with
`canvas_invalidated(&inv, draw)`. Keep drawing state in a `use_ref`, mutate it in an event handler,
then call `inv.invalidate()` to schedule one repaint. Mutating a `use_ref` does not reconcile the
tree. Get a stable invalidator from `cx.use_invalidator()`:

Use `canvas_invalidated` when event handlers mutate drawing state between renders.

The `invalidate` sample draws this way, and the `editor` and `hit_test` samples use the same pattern
to repaint only in response to pointer input.

## Getting started standalone

With a [`windows-window`](windows-window.md) `Window`, create a `GpuDevice`. Then create a swap
chain for the window. You drive the frame loop.

For a raw handle from another source, `create_swap_chain_for_hwnd` is the `unsafe` escape hatch.

The standalone canvas samples show swap-chain creation, drawing, presentation, and resize handling.

On resize, call `chain.resize(width, height)`. Use `chain.set_dpi(..)` and
`chain.set_composition_scale(..)` for sharp output. `chain.is_device_lost()` reports device loss.

## Getting started with an on-demand surface

`animated_canvas` presents a new frame each vsync. Use `CanvasImageSource` for static content. It
redraws only when you call `draw`.

Enable the reactor `canvas` feature. Create a `CanvasImageSource` from a device that you own.
Display it with a reactor `Image` widget.

`CanvasImageSource` draws on demand and exposes an image source for reactor's `Image` widget.

`new(device, width, height, scale)` takes a size in DIPs. It uses the host DPI scale to allocate
physical pixels. Drawing inside `draw` uses DIPs and origin `(0, 0)`.

`draw` returns `Ok(false)` on device loss. Create a new `GpuDevice`, call `set_device`, and draw
again.

Get the `scale` from the reactor `Image`. `Image::on_mounted` returns an `ImageHandle`.
`ImageHandle::on_rasterization_scale_changed` reports the host DPI scale. Rebuild the surface when
the scale changes. See the `image_source` sample. For a full-window surface that resizes with the
window, prefer `canvas`, which handles the device, swap chain, resize, and DPI for you.

## Getting started with a composition surface

Enable the `composition` feature to draw into a [`windows-composition`](windows-composition.md)
`CompositionDrawingSurface`. The app owns the composition graph. The bridge only lends Direct2D
drawing.

```toml
[dependencies]
windows-canvas = { version = "0.100", features = ["composition"] }
windows-composition = { version = "0.100", features = ["system"] }
```

The composition bridge creates a graphics device and drawing surface from an existing compositor.

`draw` runs inside the surface native `BeginDraw` and `EndDraw` bracket. It returns `Ok(false)` on
device loss. Recreate the device, graphics device, and surface. Then draw again.

There is no implicit clear. Clear or draw over the full surface. Coordinates are pixels with origin
`(0, 0)`. The backing-atlas offset is applied for you. This path is system-only. See the
[`composition/canvas`](../../crates/samples/composition/canvas) sample.

## Drawing basics

These methods are on `DrawingSession` and `DrawContext`.

- **Clear and fill:** `clear(ColorF)`, `fill_rect(&Rect, &paint)`,
  `fill_rounded_rect(&RoundedRect, &paint)`, `fill_ellipse(&Ellipse, &paint)`,
  `fill_path(&Path, &paint)`.
- **Stroke:** `draw_rect`, `draw_ellipse`, `draw_line`, `draw_rounded_rect`, and `draw_path`. Each
  has a `*_styled` variant that takes a `StrokeStyle`.
- **Brushes:** `create_solid_brush(ColorF)`, `create_linear_gradient(start, end, &[GradientStop])`,
  and `create_radial_gradient(center, rx, ry, &[GradientStop])`. Use `brush.set_color(..)` to update
  a solid brush.

Colors are `ColorF`. Use `ColorF::rgb(r, g, b)`, `ColorF::new(r, g, b, a)`, `ColorF::from_rgb8(..)`,
or `ColorF::from_rgba8(..)`. Constants include `WHITE`, `BLACK`, `RED`, `CORNFLOWER_BLUE`,
`DARK_SLATE_BLUE`, and `TRANSPARENT`.

## Geometry and paths

Use `Rect::new(left, top, right, bottom)` or `Rect::from_xywh(..)`. Use
`Ellipse::new(center, rx, ry)` or `Ellipse::circle(center, r)`. Use `RoundedRect::new(rect, rx, ry)`
or `RoundedRect::uniform(rect, r)`. Centers and points use `Vector2`, re-exported from
`windows-numerics`.

Build freeform paths with `PathBuilder::new(&device)`. Start with `.begin(start)` for a filled
figure, or `.begin_hollow(start)` for an open figure. Add segments with `line_to` and `bezier_to`.
Finish with `close` or `end_open`. Then call `build()` to get a `Path`.

For a closed polygon, use `PathBuilder::new(&device)?.polygon(points)`.

Use `Path::fill_contains_point`, `Path::stroke_contains_point`, and `Path::compute_bounds` for
geometry queries.

Configure strokes with `StrokeStyleBuilder`. It sets `start_cap`, `end_cap`, `caps`, `line_join`,
`miter_limit`, `dash_style`, and `dash_offset`. See `CapStyle`, `LineJoin`, and `DashStyle`.

## Text

Create a `TextFormat`, then call `draw_text(text, &format, &Rect, &paint)`:

`TextFormat` controls font, alignment, wrapping, and paragraph alignment.

`TextFormat::new_bold(..)` and `with_weight(family, size, FontWeight::BOLD)` set weight.
`TextAlignment` and `ParagraphAlignment` control placement. `with_word_wrapping(..)` sets
wrapping.

For repeated text, or when you need to measure or hit-test, build a `TextLayout`. It shapes the
text once, then answers geometry queries and draws without re-shaping (unlike `draw_text`, which
re-shapes every call):

`TextLayout` caches shaped text and provides metrics, hit testing, caret bounds, and drawing.

Use `set_max_size(..)` to reflow the layout when its box changes. `TextMetrics::bounds()` returns
the inked text rectangle within the layout box.

## Transforms, bitmaps, and effects

- **Transforms:** Use `set_transform(&Matrix3x2)` and `transform()`. Use
  `with_transform(&matrix, |s| { .. })` for scoped transforms. Matrix types come from
  `windows-numerics`.
- **Bitmaps:** Use `load_bitmap(path)` to decode an image file. Use
  `create_bitmap(pixels, width, height)` to upload premultiplied BGRA pixels. Use
  `create_bitmap_with_alpha` to select the `AlphaMode`. Then use
  `draw_bitmap(&bitmap, &Rect, opacity)` or `draw_image(&bitmap)`.
- **Off-screen targets:** Use `create_bitmap_target()` with `with_target(&bitmap, |s| { .. })`. Use
  `GpuDevice::create_render_target` for a target with CPU readback. Use `create_shadow(&bitmap)` and
  `draw_effect(&effect)` for drop shadows and effects.

## Samples

The
[`crates/samples/canvas`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)
tree contains these samples:

- **`standalone`**: creates a device and swap chain for an HWND.
- **`shared_device`**: shares one `GpuDevice` across many surfaces.
- **`samples`**: runs focused drawing examples in a reactor window.
- **`circles`**: animates circles and reuses brushes.
- **`clock`**: draws an animated analog clock with transforms and shadows.
- **`image_source`**: redraws a `CanvasImageSource` only when data changes.
- **`chart`**: hosts an on-demand swap chain on a `SwapChainPanel`.
- **`readback`**: renders off-screen and reads pixels back to the CPU.
- **`hit_test`**: tests whether the pointer is inside a filled `Path`, repainting on demand.
- **`editor`**: combines reactor pointer events with canvas geometry queries, repainting on demand.
- **`text_layout`**: caches a `TextLayout` in a `use_ref`, re-shaping it only when the window
  resizes.

The `samples` crate also has focused single-file examples under
[`samples/examples`](../../crates/samples/canvas/samples/examples), including `invalidate`, which
links clicked points with a line and repaints only when `Invalidator::invalidate` is called.

## Future work

`windows-canvas` is a Rust-idiomatic wrapper over Direct2D, DirectWrite, DXGI, and WIC. It aims to
be a natural Rust equivalent of [Win2D](https://github.com/microsoft/Win2D) rather than a port of
its API. This section records how the two compare and what is worth adding next.

### Where it already stands

On efficiency it starts ahead of Win2D. There is no WinRT component or language projection in the
way: the safe types call Direct2D/DXGI/DirectWrite/WIC COM directly, and classes `Deref` to their
default interface for zero-cost casts. Sessions bracket `BeginDraw`/`EndDraw` with `Drop`, factories
are cached, and `GpuDevice` is `Clone` to share one device across surfaces. Device-lost recovery,
WARP fallback, DPI, and composition scale are all handled.

On simplicity the surface is small and Rust-shaped: a typestate `PathBuilder`, builder patterns
(`StrokeStyleBuilder`, `TextFormat::with_*`, `TextLayout`), a sealed `Paint` trait, and scoped
`with_transform`/`with_target` closures. The `circles` and `clock` samples are far shorter than the
raw Direct2D they replace.

It covers the immediate-mode drawing subset of Win2D. The larger gaps are listed below, roughly in
priority order.

| Priority | Win2D feature | Status in windows-canvas | Impact |
| --- | --- | --- | --- |
| High | Layers and clipping (`PushLayer` / `PushAxisAlignedClip`) | Not exposed | No clip regions or group opacity |
| High | Rich geometry: combine (union/intersect/xor/exclude), arcs, quadratic bezier, widen, outline, transformed geometry, length/area | `PathBuilder` does line/bezier/close, polygon, hit-test, bounds | No boolean geometry or arcs |
| Medium | `CanvasImageBrush` (bitmap/image brush), brush opacity/transform/extend-mode | Solid, linear, radial only; gradient gamma/extend fixed | No tiled or pattern fills |
| Medium | `DrawImage`/`DrawBitmap` with source rect, interpolation, composite mode | `draw_bitmap` (dest + opacity), `draw_image` (fixed mode) | Limited compositing control |
| Medium | ~60 built-in effects | `create_shadow` plus generic `draw_effect` | Effects graph mostly absent |
| Low | `CanvasSpriteBatch` | Not exposed | Perf for many sprites |
| Low | Bitmap save/encode, `SetPixelBytes`, virtual bitmap, antialias/blend mode, DIP-vs-pixel units | Load plus `RenderTarget` readback only | Round-trip and export gaps |

### Suggested order

1. Clip and layer support, following the existing `with_*` scoped-closure pattern (for example
   `with_clip` and `with_layer(opacity, ..)`).
2. Geometry operations: arcs, quadratic bezier, boolean combine, and `Geometry` types for rect and
   ellipse hit-testing.
3. Image brush and a richer `draw_image`.
4. A small curated set of effects.

Effects breadth and sprite batch are where Win2D is largest, but most apps do not need the full
surface, so they stay low priority.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is not needed to use
`windows-canvas`.

### How it's built

`src/bindings.rs` is generated by `tool_bindings` from `crates/tools/bindings/src/canvas.txt`. It
contains minimal, flat bindings for Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC. It also uses
the reference `Windows.winmd` for WinRT numerics types.

The safe wrappers are hand-written. They include `GpuDevice`, `SwapChain`, `DrawingSession`,
geometry, text, bitmaps, brushes, effects, and render targets.

The reactor integration lives in [`windows-reactor`](windows-reactor.md). It is behind the reactor
`canvas` feature. The optional `composition` feature connects this crate to
[`windows-composition`](windows-composition.md).

### Design

- **No WinRT layer:** The safe types wrap Direct2D, Direct3D, DXGI, DirectWrite, and WIC directly.
- **Single-threaded rendering:** A `SwapChain` owns one D2D device context. Rendering happens on the
  thread that owns the swap chain.
- **Continuous rendering:** `animated_canvas` drives frames on the UI thread with
  `CompositionTarget::Rendering`.
- **Demand-driven rendering:** `canvas` repaints only on resize and DPI change; `canvas_invalidated`
  adds an `Invalidator` for state-driven repaints. Both stay idle otherwise.
- **On-demand image source:** `CanvasImageSource` draws into a WinUI `SurfaceImageSource` only when
  requested. It uses a borrowed `DrawingSession`.
- **Composition bridge:** `CanvasCompositionExt::draw` draws Direct2D content into a
  `CompositionDrawingSurface`. It also uses a borrowed `DrawingSession`.
- **Device-lost recovery:** `device_lost.rs` classifies DXGI and D2D loss codes. `EndDraw` and
  `Present` set a flag. The next frame recreates the device and resources.
- **WARP fallback:** `GpuDevice::new_or_warp()` tries hardware first. It falls back to the WARP
  software rasterizer when no GPU is available.
- **Shareable device:** `GpuDevice` is `Clone`. A clone shares the same Direct3D, Direct2D, DXGI,
  and DirectWrite objects.

### Reactor integration

The reactor harness lives in [`windows-reactor`](windows-reactor.md). It exports `animated_canvas`,
`canvas`, `canvas_invalidated`, `Invalidator`, `CanvasImageSource`, `CanvasSwapChain`, and
`DrawContext` under the reactor `canvas` feature.

The dependency direction is `windows-reactor[canvas]` to `windows-canvas`. Reactor owns the WinUI
element harness. That includes `SwapChainPanel`, `SurfaceImageSource`, the render loop, resize, DPI,
and unmount cleanup.

This crate exposes the drawing surface that the bridge needs. It includes `GpuDevice`, `SwapChain`,
borrowed `DrawingSession` constructors, and the `ID2D1DeviceContext` interop type. This crate has no
`windows-reactor` dependency.

Input belongs to reactor. Geometry queries belong to canvas. Pointer events use DIPs. Apps map those
DIPs into canvas space with their own transform.

### Testing

Tests render with the WARP software rasterizer. They need no GPU. The integration suite lives in the
`test_canvas` crate. Run `cargo test -p test_canvas`.