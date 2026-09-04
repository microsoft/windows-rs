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

The `composition` feature works with either Composition stack. When Reactor selects lifted
Composition, that stack takes precedence over the default system stack.

## The basic idea

Canvas is an immediate-mode drawing API. The application receives a drawing session and describes
the current frame with commands such as `clear`, `fill_rect`, `draw_text`, and `draw_bitmap`.

```text
begin a frame -> issue drawing commands -> finish the frame -> present it
```

There is no retained scene tree. If a shape should still be visible in the next frame, draw it
again. Keep application state - points, colors, text, images - in normal Rust data and turn that
state into drawing commands.

The simplest host is Reactor. It creates the graphics device and swap chain, tracks size and display
scale, and handles device recovery. Enable the feature in `Cargo.toml`:

```toml
windows-canvas = { version = "0.100.0", features = ["reactor"] }
windows-reactor = "0.100.0"
```

## Your first canvas

`canvas` draws once when the surface is ready and again when its size or display scale changes:

```rust,ignore
use windows_canvas::*;
use windows_reactor::App;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::DARK_SLATE_BLUE);

    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    let center = Vector2::new(ctx.width / 2.0, ctx.height / 2.0);
    let radius = ctx.width.min(ctx.height) * 0.3;
    ctx.fill_ellipse(&Ellipse::circle(center, radius), &brush);

    Ok(())
}

fn main() -> Result<()> {
    App::run(canvas(draw))
}
```

The callback receives a `DrawContext`. Its `width` and `height` are the available size in
device-independent pixels. It dereferences to `DrawingSession`, so drawing methods and resource
creation are available directly on `ctx`.

Clear the surface, or fully cover it, unless keeping pixels from the previous frame is intentional.

## Draw shapes with brushes

A brush supplies paint. Create it once within a frame and reuse it for as many operations as
needed:

```rust,ignore
let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

ctx.fill_rect(&Rect::from_xywh(20.0, 20.0, 160.0, 100.0), &brush);
ctx.draw_ellipse(
    &Ellipse::circle(Vector2::new(260.0, 100.0), 60.0),
    &brush,
    4.0,
);

brush.set_color(ColorF::WHITE);
ctx.draw_line(
    Vector2::new(20.0, 160.0),
    Vector2::new(320.0, 160.0),
    &brush,
    2.0,
);
```

`Rect`, `RoundedRect`, and `Ellipse` are lightweight geometry values. Use `fill_*` for a solid
interior and `draw_*` for an outline. Gradient brushes use the same drawing methods.

## Apply a temporary transform

`with_transform` applies translation, rotation, or scale only while its closure runs, then restores
the previous transform:

```rust,ignore
let transform =
    Matrix3x2::rotation_around(15.0, Vector2::new(ctx.width / 2.0, ctx.height / 2.0));

ctx.with_transform(&transform, || {
    ctx.fill_rect(
        &Rect::from_xywh(ctx.width / 2.0 - 80.0, ctx.height / 2.0 - 40.0, 160.0, 80.0),
        &brush,
    );
});
```

Scoped transforms make helper functions easier to combine because one drawing operation cannot
leave a transform behind for the next one.

## Build a custom path

For a polygon, `PathBuilder::polygon` is the shortest route:

```rust,ignore
let points = [
    Vector2::new(160.0, 40.0),
    Vector2::new(280.0, 240.0),
    Vector2::new(40.0, 120.0),
    Vector2::new(280.0, 120.0),
    Vector2::new(40.0, 240.0),
];

let star = PathBuilder::new(ctx.device())?.polygon(points)?;
let brush = ctx.create_solid_brush(ColorF::new(1.0, 0.8, 0.0, 1.0))?;
ctx.fill_path(&star, &brush);
```

Use `begin`, `line_to`, and `bezier_to` when the path needs individual segments:

```rust,ignore
let curve = PathBuilder::new(ctx.device())?
    .begin_hollow(Vector2::new(40.0, 160.0))
    .bezier_to(
        Vector2::new(120.0, 20.0),
        Vector2::new(240.0, 300.0),
        Vector2::new(320.0, 160.0),
    )
    .end_open()
    .build()?;

ctx.draw_path(&curve, &brush, 3.0);
```

Paths also support bounds and fill or stroke hit testing, which is useful for diagrams and editors.

## Draw text

For ordinary text, create a `TextFormat` and draw into a rectangle:

```rust,ignore
let format = TextFormat::new("Segoe UI", 32.0)?
    .with_alignment(TextAlignment::Center)
    .with_paragraph_alignment(ParagraphAlignment::Center);
let brush = ctx.create_solid_brush(ColorF::WHITE)?;

ctx.draw_text(
    "Hello, Canvas!",
    &format,
    &Rect::new(0.0, 0.0, ctx.width, ctx.height),
    &brush,
);
```

Use `TextLayout` when text is drawn repeatedly or needs measurement, reflow, caret bounds, or hit
testing. Cache the layout and rebuild it when its text, format, size, or device changes. The
[`text_layout`](../../crates/samples/canvas/text_layout) sample shows that pattern.

## Load and draw an image

`load_bitmap` decodes an image file:

```rust,ignore
let bitmap = ctx.load_bitmap("assets/photo.png")?;
let destination = Rect::from_xywh(20.0, 20.0, bitmap.width(), bitmap.height());
ctx.draw_bitmap(&bitmap, &destination, 1.0);
```

For generated image data, `create_bitmap` accepts tightly packed premultiplied BGRA pixels:

```rust,ignore
let bitmap = ctx.create_bitmap(&pixels, width, height)?;
ctx.draw_bitmap(
    &bitmap,
    &Rect::from_xywh(0.0, 0.0, width as f32, height as f32),
    1.0,
);
```

Bitmaps, brushes, paths, and text layouts depend on the graphics device. Creating them inside the
draw callback is the easiest starting point. Cache expensive resources only when profiling shows
the need, and rebuild cached resources when `ctx.device_changed()` is true.

## Redraw when state changes

Use `canvas` when content changes only because the surface was created or resized. Use
`animated_canvas` for motion that needs every display frame:

```rust,ignore
animated_canvas(|ctx| {
    draw_frame(ctx, elapsed_time())
})
```

For input-driven content, use `canvas_invalidated`. Store an `Invalidator` with the component,
change the drawing data in `update`, and request one new frame:

```rust,ignore
struct Sketch {
    points: Rc<RefCell<Vec<Vector2>>>,
    invalidator: Invalidator,
}

fn update(&mut self, point: Vector2, _context: &ComponentContext<Self>) {
    self.points.borrow_mut().push(point);
    self.invalidator.invalidate();
}

fn draw_points(ctx: &DrawContext, points: &[Vector2]) -> Result<()> {
    ctx.clear(ColorF::BLACK);
    let brush = ctx.create_solid_brush(ColorF::WHITE)?;
    for &point in points {
        ctx.fill_ellipse(&Ellipse::circle(point, 3.0), &brush);
    }
    Ok(())
}

fn view(&self, _input: &(), _context: &mut ViewContext<Self>) -> View {
    let points = Rc::clone(&self.points);
    canvas_invalidated(&self.invalidator, move |ctx| {
        draw_points(ctx, &points.borrow())
    })
}
```

Invalidation keeps the app idle between changes. Do not use continuous animation for a chart,
drawing, or diagram that changes only after input.

## Drive a standalone swap chain

Without Reactor, the application creates a `GpuDevice` and `SwapChain` and owns the frame loop. The
essential sequence is:

```rust,no_run
use windows_canvas::*;

fn draw_frame(chain: &mut SwapChain) -> Result<bool> {
    let session = chain.begin_draw()?;
    session.clear(ColorF::DARK_SLATE_BLUE);

    let brush = session.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    session.fill_rect(&Rect::from_xywh(20.0, 20.0, 200.0, 120.0), &brush);

    drop(session);
    chain.present()
}
```

The drawing session borrows the swap chain, so finish and drop it before `present`. A standalone
host must also resize the swap chain, update display scale, schedule frames, and recreate the
device, swap chain, and dependent resources when `present` returns `Ok(false)`. The
[`standalone`](../../crates/samples/canvas/standalone) sample connects those pieces to
`windows-window`.

## What to read next

| Sample | What it shows |
| --- | --- |
| [`examples`](../../crates/samples/canvas/samples/examples) | One drawing operation at a time |
| [`invalidate`][canvas-invalidate] | Input-driven repainting |
| [`clock`](../../crates/samples/canvas/clock) | Continuous animation |
| [`chart`](../../crates/samples/canvas/chart) | A demand-driven chart |
| [`hit_test`](../../crates/samples/canvas/hit_test) | Geometry hit testing |
| [`image_source`](../../crates/samples/canvas/image_source) | An on-demand WinUI image |
| [`readback`](../../crates/samples/canvas/readback) | Off-screen rendering and pixel readback |
| [`composition`](../../crates/samples/composition/canvas) | Drawing into a Composition visual |

[canvas-invalidate]: ../../crates/samples/canvas/samples/examples/invalidate.rs

Start with the small examples. `CanvasImageSource`, off-screen targets, shared devices, and
Composition surfaces solve specific hosting problems and are easier to learn after the normal draw
callback feels familiar.

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
Composition Canvas sample covers the system bridge, and the Stacker Reactor sample covers the
lifted bridge.
