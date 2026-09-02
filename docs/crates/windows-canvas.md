# windows-canvas

> Safe 2D drawing over Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC.

- 📦 [crates.io](https://crates.io/crates/windows-canvas)
- 📖 [docs.rs](https://docs.rs/windows-canvas)
- 🚀 [Getting started](../../crates/libs/canvas/readme.md)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/canvas)
- [Reactor guide](windows-reactor.md)
- [Composition guide](windows-composition.md)

## When to use it

Use Canvas for custom 2D graphics, charts, diagrams, image processing, text layout, geometry hit
testing, or animation. It is a drawing API rather than a control toolkit. Combine it with Reactor
when the app also needs WinUI controls.

Use Composition when the scene is primarily a retained tree of visuals that Windows animates and
composes. Use Canvas when the app draws pixels and paths for each requested frame. The two can be
combined by drawing Canvas content into a composition surface.

## Getting started: choose a hosting path

| Path | Choose it when | Main API |
| --- | --- | --- |
| Reactor, continuous | Draw every display frame | `animated_canvas` |
| Reactor, on demand | Draw after layout or invalidation | `canvas` or `canvas_invalidated` |
| Reactor image | Redraw a fixed-size image on request | `CanvasImageSource` |
| Standalone HWND | The app owns a window and frame loop | `GpuDevice`, `SwapChain` |
| Composition | Draw into a retained visual tree | `CanvasCompositionExt` |
| Off-screen | The result is read back or reused as an image | `RenderTarget` or bitmap target |

The README shows the minimum device, swap chain, drawing session, and present sequence. Choose the
host before expanding it; resize, DPI, frame scheduling, and device-loss responsibilities differ
between paths.

## Core model

A `GpuDevice` owns the Direct3D, Direct2D, DXGI, DirectWrite, and WIC resources shared by drawing
objects. It is cloneable; clones share the same underlying device.

A `SwapChain` owns buffers presented to a window or panel. A frame follows this order:

1. Call `begin_draw` to borrow a `DrawingSession`.
2. Clear or cover the target and issue drawing commands.
3. Drop the session to finish the draw.
4. Call `present`.

`DrawingSession` creates device-dependent brushes, bitmaps, text objects, paths, and effects. It
also holds current drawing state such as transforms and targets. Do not retain a session beyond its
frame.

Reactor-hosted callbacks receive `DrawContext`. It dereferences to `DrawingSession` and adds:

- `width` and `height`, in DIPs.
- `device()` for creating or comparing device-owned resources.
- `device_changed()` when size, scale, or device recovery requires cached resources to be rebuilt.

Coordinates in Reactor and standalone drawing are DIPs after the host configures DPI and scale.
Composition drawing surfaces use pixels.

## Common workflows

### Host a canvas in Reactor

Enable the Canvas `reactor` feature and choose a render mode:

- `animated_canvas(draw)` draws on each WinUI rendering event.
- `canvas(draw)` draws on first layout and when size or display scale changes.
- `canvas_invalidated(&invalidator, draw)` adds app-requested repaints.

The host creates and attaches a swap chain, responds to resize and DPI changes, recovers from
device loss, and cleans up on unmount. The draw closure returns `Result<()>`, so resource creation
can use `?`.

For state-driven demand rendering, store an `Invalidator` and shared drawing data in the owning
component. Update both in `Component::update`, then call `invalidate`. The next rendering event
draws one frame and returns to idle.

The convenience functions panic on non-device-loss integration errors. Applications that need to
report or recover from those errors can build with `Canvas::animated`,
`Canvas::animated_with_device`, or `Canvas::invalidated`, add `on_error`, and convert the builder
into `View`.

Cache brushes, layouts, and bitmaps outside the draw closure when their creation cost matters.
Rebuild device-dependent resources whenever `device_changed()` is true. The
[`text_layout`](../../crates/samples/canvas/text_layout) sample demonstrates this pattern.

### Drive a standalone swap chain

Create a `windows-window::Window`, a `GpuDevice`, and a swap chain with
`create_swap_chain_for_window`. The application owns frame scheduling and must:

- Draw and present each required frame.
- Call `resize` after a client-size change.
- Set DPI and composition scale when display scale changes.
- Check `is_device_lost` and recreate the device, swap chain, and dependent resources.

Use `create_swap_chain_for_hwnd` only when another API owns a valid HWND and the caller can uphold
its safety contract.

`GpuDevice::new_or_warp` tries hardware and falls back to WARP. Use `new` when hardware is required
and `new_warp` for deterministic software rendering.

### Draw shapes, paths, and strokes

Create geometry values with `Rect`, `RoundedRect`, and `Ellipse`, then call the corresponding
`fill_*` or `draw_*` methods. Paint may be a solid or gradient brush.

Build freeform geometry with `PathBuilder`:

1. Start a filled figure with `begin` or an open figure with `begin_hollow`.
2. Add `line_to` and `bezier_to` segments.
3. Finish with `close` or `end_open`.
4. Call `build` and draw or fill the resulting `Path`.

Use `polygon` for a closed polygon. `Path::fill_contains_point`,
`stroke_contains_point`, and `compute_bounds` support hit testing and layout.

Configure caps, joins, miter limits, and dashes with `StrokeStyleBuilder`, then use a `*_styled`
drawing method.

### Draw and measure text

Use `TextFormat` with `draw_text` for text that is drawn once and does not need measurement. The
format controls family, size, weight, alignment, wrapping, and paragraph alignment.

Use `TextLayout` when text is repeated, measured, reflowed, or hit-tested. It shapes the text once
and provides metrics, caret bounds, and hit-testing operations. Call `set_max_size` when the layout
box changes and reuse the layout until text, format, size, or device changes.

### Load images and use off-screen targets

Use `load_bitmap(path)` to decode a file through WIC. Use `create_bitmap` for premultiplied BGRA
bytes or `create_bitmap_with_alpha` when alpha mode must be explicit. Draw with `draw_bitmap` or
`draw_image`.

Use `create_bitmap_target` with `with_target` to draw into a reusable bitmap. Use
`GpuDevice::create_render_target` when CPU readback is required. The
[`readback`](../../crates/samples/canvas/readback) sample renders off-screen and retrieves pixels.

Apply a temporary transform with `with_transform`; it restores the previous transform after the
closure. `with_target` provides the corresponding scoped target change. Use these scoped methods
when nested drawing code should not leak state into the rest of the frame.

### Use an on-demand image in Reactor

`CanvasImageSource` draws into a WinUI `Image` without running a swap-chain frame loop:

1. Store a `GpuDevice`, `CanvasImageSource`, and `ElementRef<Image>`.
2. Observe rasterization scale on the image reference.
3. Create the source with a DIP size and current scale.
4. Attach it to the image and call `draw` when content changes.

`draw` returns `Ok(false)` on device loss. Recreate the device and image source, attach the
replacement, and draw again. Recreate the surface when rasterization scale changes.

`attach` panics after an accepted native attachment fails. Use `attach_result` when the app needs
the `IntegrationError`. Both return `false` without accepting a request while the reference is
unbound.

Prefer a Reactor `canvas` for a full-window surface that tracks layout automatically. Use
`CanvasImageSource` for fixed-size images, many small surfaces, or content that redraws only after a
specific data change.

### Draw into a composition surface

Enable the Canvas `composition` feature and use the system Composition stack:

1. Create a Canvas `GpuDevice` and Composition `Compositor`.
2. Call `device.create_graphics_device(&compositor)`.
3. Create a `CompositionDrawingSurface`.
4. Create a composition surface brush and assign it to a visual.
5. Import `CanvasCompositionExt` and call `surface.draw`.

The draw closure receives a borrowed `DrawingSession`. Clear or cover the whole surface because
there is no implicit clear. Coordinates are pixels, and Canvas applies the backing-atlas offset.
`Ok(false)` means device loss; recreate the Canvas device, composition graphics device, surface,
and brush.

This bridge is available only with `windows-composition`'s `system` feature.

## Pitfalls

- Do not keep a `DrawingSession` after its frame or across a resize.
- Drop the session before presenting a swap chain.
- Recreate device-dependent resources after device loss or when `device_changed()` is true.
- Clear or fully cover each target unless preserving prior pixels is intentional.
- Use DIPs for Reactor surfaces and pixels for composition drawing surfaces.
- Do not redraw continuously when content changes only on input; use invalidation.
- Map Reactor pointer coordinates through the same application transform used for drawing before
  geometry hit testing.
- Handle non-device-loss errors instead of treating a blank frame as recovery.

## Samples

Follow the samples by task:

| Goal | Sample |
| --- | --- |
| Learn individual drawing calls | [`samples/examples`][canvas-examples] |
| Host and present to an HWND | [`standalone`][canvas-standalone] |
| Animate in Reactor | [`circles`][canvas-circles], [`clock`][canvas-clock] |
| Repaint only after state changes | `invalidate` under [`samples/examples`][canvas-examples] |
| Cache measured text | [`text_layout`](../../crates/samples/canvas/text_layout) |
| Host a demand-driven chart | [`chart`](../../crates/samples/canvas/chart) |
| Attach an on-demand image | [`image_source`](../../crates/samples/canvas/image_source) |
| Share one device across surfaces | [`shared_device`](../../crates/samples/canvas/shared_device) |
| Hit-test and edit geometry | [`hit_test`](../../crates/samples/canvas/hit_test) |
| Edit interactive geometry | [`editor`](../../crates/samples/canvas/editor) |
| Render and read pixels | [`readback`](../../crates/samples/canvas/readback) |
| Draw into Composition | [`composition/canvas`](../../crates/samples/composition/canvas) |

[canvas-examples]: ../../crates/samples/canvas/samples/examples
[canvas-standalone]: ../../crates/samples/canvas/standalone
[canvas-circles]: ../../crates/samples/canvas/circles
[canvas-clock]: ../../crates/samples/canvas/clock

---

## Internal documentation

The remainder of this page describes how the crate is built and maintained. Applications do not
need it to use Canvas.

### Architecture

The safe wrappers are hand-written and include `GpuDevice`, `SwapChain`, `DrawingSession`,
geometry, brushes, text, bitmaps, effects, and render targets.

`GpuDevice` owns the shared Direct3D, Direct2D, DXGI, DirectWrite, and WIC objects. A swap chain
owns one D2D device context and is rendered on its owning thread. WARP allows tests and callers to
render without a physical GPU.

`device_lost.rs` classifies D2D and DXGI loss codes. Standalone callers perform recovery. The
Reactor host rebuilds its device, swap chain, and cached state after loss.

The Reactor integration lives behind this crate's `reactor` feature. The dependency direction is
`windows-canvas[reactor] -> windows-reactor`. Canvas owns rendering resources and receives only
typed panel metrics, rendering notifications, and attachment completion through
`ElementRef<SwapChainPanel>`.

Continuous mode uses WinUI's `CompositionTarget::Rendering`. Demand mode stays idle until layout,
scale, or `Invalidator` requests a frame. Attachment attempts use generations so stale completion
callbacks cannot ready a replacement surface. `Canvas::on_error` reports initialization,
attachment, resize, drawing, presentation, and failed recovery through `IntegrationError`.

`CanvasImageSource` uses a borrowed drawing session over a WinUI `SurfaceImageSource`.
`CanvasCompositionExt` uses a borrowed drawing session over
`ICompositionDrawingSurfaceInterop`. Both pair every successful `BeginDraw` with `EndDraw`,
including panic cleanup.

### Code generation

`src/bindings.rs` is generated by `tool_bindings` from
`crates/tools/bindings/src/canvas.txt`. It contains minimal flat bindings for Direct2D,
Direct3D 11, DXGI, DirectWrite, and WIC, plus referenced WinRT numerics types.

`src/reactor_bindings.rs` is generated by `tool_reactor` for the WinUI bridge. Generated files are
committed and must not be edited by hand. After changing a binding filter or generator, run the
corresponding tool and verify `cargo check -p windows-canvas --quiet`.

### Testing

Canvas tests use the WARP software rasterizer and do not require a physical GPU. The integration
suite is in `test_canvas`:

```text
cargo test -p test_canvas
```

Reactor integration behavior is also covered by Reactor's recording and live surface tests. The
Composition Canvas sample covers the system composition bridge in a runnable window.
