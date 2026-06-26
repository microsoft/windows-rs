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
For a closed, filled polygon, `PathBuilder::new(&device)?.polygon(points)` does
the whole begin/`line_to`/close/build sequence in one call.

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
- **`hit_test`** — geometry hit-testing: a star recolors only when the pointer is
  inside its *actual filled geometry* (`Path::fill_contains_point`), with its
  bounding box (`compute_bounds`) drawn for contrast.
- **`editor`** — an interactive map-style shape editor that composes the reactor
  pointer events with the canvas geometry queries: click to drop a shape, drag a
  shape to move it, right-click to delete, and a toolbar to pick the kind.
  Selection hit-tests the real polygon (`fill_contains_point`) and outlines it
  with `compute_bounds`.

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
  its device and resources on the next frame. The classifier is exported as
  `is_device_lost(HRESULT)` / `check_device_lost(&Result<T>)` for callers driving
  their own draw loop.
- **Hardware-with-WARP fallback** — `GpuDevice::new_or_warp()` tries a hardware
  device and falls back to the WARP software rasterizer when no GPU is available
  (headless sessions, VMs, RDP). `animated_canvas` uses it on both the initial
  mount and device-lost rebuild so the render loop still produces output on
  GPU-less machines instead of silently drawing nothing.

### Input and hit-testing

Drawing into a canvas is only half of an interactive app such as a map or tile
editor. The other half is *input* — and two separate concerns are easily
conflated under "hit-testing":

- **Input routing** — delivering pointer and keyboard events, *with
  coordinates*, to application code.
- **Geometry hit-testing** — asking whether a point lies inside a shape's fill
  or along its stroke (or which glyph/caret a point maps to, for text).

#### Current state

- `animated_canvas` returns a `SwapChainPanel`, which is a full
  [`windows-reactor`](windows-reactor.md) element, so the element pointer
  callbacks (`on_pointer_pressed`, `on_pointer_released`, `on_pointer_exited`,
  `on_tapped`, `on_right_tapped`) already attach to the canvas surface.
- **Pointer position is surfaced on press/release and during movement.** The
  `PointerEventInfo` passed to `on_pointer_pressed`/`on_pointer_released`/
  `on_pointer_moved`/`on_pointer_entered` carries the position (`x`/`y`, in DIPs
  relative to the element) alongside button state. See the `pointer_position`
  and `pointer_tracking` reactor samples.
- **Continuous pointer is wired.** `on_pointer_moved` (drag/hover) and
  `on_pointer_entered` deliver `PointerEventInfo`; `on_pointer_exited` signals
  the pointer leaving the element. The previously-stubbed `PointerMoved`/
  `PointerEntered` vtable slots are now generated.
- There is **no keyboard surface** — `KeyDown`/`KeyUp` exist in the bindings but
  no `on_key_down`/`on_key_up` is exposed on elements. A future keyboard surface
  must reckon with WinUI focus: a bare `Border` cannot take focus, so it never
  receives `KeyDown` (see the roadmap's phase 3 for the focus rule learned from a
  prototype).
- **Geometry hit-testing is wired.** `Path` exposes `fill_contains_point`,
  `stroke_contains_point`, and `compute_bounds` over the underlying
  `ID2D1Geometry`. Boolean ops (`CombineWith`) and the remaining measurement
  methods are still `usize` stubs in `bindings.rs`.

#### Reference: Win2D

`windows-canvas` is an idiomatic Rust port of Win2D, which is the design
reference for the gaps above:

- Win2D's `CanvasControl`/`CanvasAnimatedControl` are XAML `UIElement`s, so input
  is just the standard XAML `PointerPressed`/`PointerMoved`/`PointerReleased` and
  `KeyDown` events, reading the pointer `Position` relative to the control.
  `CanvasAnimatedControl` pairs that input with an `Update`/`Draw` game loop —
  essentially the model an interactive editor wants. `animated_canvas` already
  matches that render model (it draws every frame via `CompositionTarget::Rendering`).
  Pointer input and geometry queries are now in place; the remaining gaps are
  keyboard input (for custom surfaces — see phase 3) and text hit-testing.
- `CanvasGeometry` exposes `FillContainsPoint`, `StrokeContainsPoint`,
  `ComputeBounds`, and `CombineWith`; `CanvasTextLayout` exposes glyph/caret
  hit-testing. These are the geometry/text features still marked as gaps.

#### A good design

The guiding split is **input lives in the reactor; geometry queries live in
canvas**. The reactor owns the XAML element surface and event lifetimes, so it is
the natural home for pointer/keyboard plumbing; canvas stays a pure rendering +
geometry library and gains no input system of its own.

- **Reactor input.** `PointerEventInfo` carries the pointer position (`x`/`y`,
  DIPs relative to the element, read from `GetCurrentPoint(element).Position`)
  on press/release **and** during movement via `on_pointer_moved` /
  `on_pointer_entered` (with `on_pointer_exited` for leave). Keyboard is a
  possible future addition (`on_key_down`/`on_key_up` with a virtual-key +
  modifier payload), but it is gated on the focus story and is narrow in value —
  only a hit-test-visible `Panel` or `Control`, not a bare `Border`, can take
  focus, via `IsTabStop` + `Focus(Programmatic)` deferred to `Loaded`. See
  phase 3.
- **One coordinate convention.** Pointer coordinates are DIPs; the app converts
  to canvas space using the same `width`/`height`/scale the draw closure already
  receives. Keep DIPs end to end so screen → canvas mapping is a single, obvious
  transform.
- **Canvas geometry queries.** `Path::fill_contains_point(point)`,
  `stroke_contains_point(point, stroke_width)`, and `compute_bounds()` are thin
  safe wrappers over `ID2D1Geometry`, generated by un-stubbing those methods in
  `crates/tools/bindings/src/canvas.txt`. Boolean `combine(other, op)` and the
  remaining measurement methods are still to do.
- **Grid vs. free-form.** For a grid/tile editor, geometry hit-testing is not
  required at all — a cell is `floor(pos / cell_size)` — so coordinate-carrying
  pointer events alone unblock the common case. Geometry hit-testing is for
  free-form shapes (which arbitrary polygon did the pointer land in).

#### Phases

Each phase is independently shippable. Phases 1, 2, 4, and 6 are **done**; phase
1 alone unblocks the common tile/grid interaction case. The interactive editor
sample (phase 6) now composes the input + geometry work into the real use case,
which is the natural place to motivate keyboard (phase 3) with a concrete need.

1. **Pointer coordinates** *(reactor)* — **done.** `PointerEventInfo` carries
   `x`/`y` (DIPs relative to the element), read from `.Position` in
   `pointer_event_info`; see the `pointer_position` sample. Unblocks tile/grid
   hit-testing.
2. **Continuous pointer** *(reactor)* — **done.** `on_pointer_moved` /
   `on_pointer_entered` deliver `PointerEventInfo` (reusing `pointer_event_info`)
   for drag and hover, with `on_pointer_exited` for leave; the `PointerMoved`/
   `PointerEntered` `IUIElement` vtable slots are now generated. See the
   `pointer_tracking` sample.
3. **Keyboard** *(reactor)* — **not done; deprioritized.** The plumbing is
   straightforward (`KeyDown`/`KeyUp` on `IUIElement`; a `KeyEventInfo` with the
   virtual key from `KeyRoutedEventArgs` + Ctrl/Shift/Alt read via Win32
   `GetKeyState`), but the value is narrow: built-in controls already handle their
   own keys and app shortcuts use keyboard accelerators, so raw key input is
   really only for **custom interactive surfaces** (a canvas editor/game on a
   `SwapChainPanel`). It should be motivated by — and folded into — the
   interactive editor sample (phase 6) rather than shipped standalone.

   The hard part is **focus**, and a prototype surfaced a non-obvious WinUI rule:
   a bare `Border`/decorator **cannot** take keyboard focus, so it never receives
   `KeyDown`. Only a hit-test-visible **`Panel`** (a `StackPanel`/`Canvas`/
   `SwapChainPanel` with a `Background`) or a `Control` is focusable, via
   `IsTabStop = true` + `Focus(FocusState::Programmatic)` **deferred to the
   `FrameworkElement.Loaded` event** (`Focus()` returns `false` before the element
   is in the live visual tree). A `focusable` element modifier would encapsulate
   exactly that.
4. **Geometry queries** *(canvas)* — **done.** `Path::fill_contains_point`,
   `stroke_contains_point`, and `compute_bounds` wrap `ID2D1Geometry` (bindings
   regenerated from `canvas.txt`). See the `canvas_hit_test` sample. Boolean
   `combine` and the remaining measurement methods remain open.
5. **Text hit-testing** *(canvas)* — DirectWrite `HitTestPoint` /
   `HitTestTextPosition`, once a text-layout type exists (this depends on the
   broader text-layout/metrics gap).
6. **Interactive editor sample** *(done)* — the `editor` sample is a map-style
   shape editor composing pointer (1, 2) + geometry (4) end to end: click to drop
   a shape, left-drag to move it, right-click to delete, and a toolbar picks the
   kind. Selection hit-tests the real polygon via `fill_contains_point` and
   outlines it with `compute_bounds`. The whole model lives in one `use_ref` so
   the high-frequency pointer handlers mutate in place without reconcile churn
   during a drag. It is the natural place to motivate keyboard (3) with a real
   need (e.g. arrow keys / Delete on the editor surface).

### Testing

Tests render with the WARP software rasterizer, so they need no GPU. The
integration suite lives in the `test_canvas` crate: `cargo test -p test_canvas`.

### Future work — Win2D parity

`windows-canvas` is an idiomatic Rust port of Win2D, but it deliberately covers
only the most common drawing path so far. This section catalogs the gaps against
Win2D's full surface ([Win2D](https://github.com/microsoft/Win2D), `winrt/lib/*.abi.idl`) so contributors can see
where the crate is headed. The goal is **not** a 1:1 port — Win2D is ~400K lines
of C++ and wraps ~100 effects — but to reach the capabilities real apps need, in
a smaller, safe, idiomatic API.

The list is ordered roughly by user impact. "Present" notes what already exists so
the delta is clear.

#### 1. Geometry operations *(high)*

Present: `PathBuilder` (lines + cubic Bézier, filled/hollow figures) producing a
`Path` for drawing.

Missing vs Win2D's `CanvasGeometry`/`CanvasPathBuilder`:

- **Hit-testing** — `fill_contains_point` / `stroke_contains_point` are present;
  see *Input and hit-testing* above.
- **Bounds & measurement** — `compute_bounds` is present; `ComputeStrokeBounds`,
  `ComputeArea`, `ComputePathLength`, `ComputePointOnPath` remain.
- **Boolean / shape ops** — `CombineWith` (union/intersect/xor/exclude),
  `Outline`, `Simplify`, `Stroke` (geometry realization), `Tessellate`.
- **Richer path building** — arcs, quadratic Béziers, `SetFilledRegionDetermination`,
  `SetSegmentOptions`, `AddGeometry`.
- **Geometry factories** — build geometries directly from rect / rounded-rect /
  ellipse / circle / polygon / geometry group / text / glyph run / ink.
- **`CanvasCachedGeometry`** — pre-realized fill/stroke for fast repeated draws.

Most are thin wraps over `ID2D1Geometry1` / `ID2D1Factory1`, gated on un-stubbing
those methods in `canvas.txt` (currently `usize` stubs in `bindings.rs`).

#### 2. Text layout, metrics, and fonts *(high)*

Present: `TextFormat` (family, size, weight, horizontal/vertical alignment) and
single-shot `draw_text`.

Missing vs Win2D's `CanvasTextLayout`/`CanvasFontSet`/`CanvasFontFace`:

- **`TextLayout` type** — measured, reusable layout with `LayoutBounds`,
  `DrawBounds`, `LineMetrics`, `ClusterMetrics`, `LineCount`.
- **Text hit-testing** — `HitTest`, `GetCaretPosition`, `GetCharacterRegions`
  (caret placement, selection, click-to-character).
- **Per-range formatting** — color/brush/font/size/weight/underline/strikethrough/
  typography over character ranges.
- **Format options** — word wrapping, trimming + trimming sign, line spacing,
  optical alignment, vertical text.
- **Font enumeration** — system font families, font matching, glyph metrics,
  supported typographic features (`CanvasFontSet`, `CanvasFontFace`).
- **Custom text rendering & glyph runs** — `CanvasTextRenderer`, `DrawGlyphRun`,
  `CanvasTypography`, `CanvasTextAnalyzer`.

This is consistently Win2D's most-used feature beyond basic drawing.

#### 3. Drawing session state, layers, and image draw *(medium-high)*

Present: clear; draw/fill of rect, rounded-rect, ellipse, line, path; `draw_text`;
`draw_bitmap`/`draw_image`; transform get/set/scoped; `with_target`.

Missing vs `CanvasDrawingSession`:

- **Layers & clipping** — `CreateLayer`/`ActiveLayer` (opacity masks, geometric
  clips), push/pop clip rectangles.
- **Render state** — `Blend`, `Antialiasing`, `TextAntialiasing`, `Units`
  (pixels vs DIPs), per-session `Dpi`.
- **Full `DrawImage` overloads** — source rect, offset/dest rect, opacity,
  interpolation mode, composite mode, perspective.
- **`CanvasSpriteBatch`** — high-throughput batched sprite drawing (directly
  relevant to tile/game scenarios).
- **`DrawCachedGeometry`**, ink, gradient mesh, SVG drawing.

#### 4. Effects *(medium)*

Present: an opaque `Effect` wrapper, drop shadow (`create_shadow`), and
`draw_effect`.

Missing: a real effect graph. Win2D exposes ~54 generated effects
(`GaussianBlur`, `ColorMatrix`, `Blend`, `Composite`, `Crop`, `Transform2D/3D`,
`HueRotation`, `Saturation`, `Tint`, `Vignette`, lighting/transfer/morphology,
…), typed parameters, chained inputs, `CacheOutput`/`BufferPrecision`, and custom
`PixelShaderEffect`. A pragmatic port wraps ~10 common effects plus a
`Effect::custom(clsid)` escape hatch rather than all ~100 D2D effects.

#### 5. Bitmaps, render targets, and I/O *(medium)*

Present: load a bitmap from a file path (WIC); `create_bitmap_target` for
off-screen draws; `width`/`height`.

Missing vs `CanvasBitmap`/`CanvasRenderTarget`/`CanvasImage`:

- **Saving** — `SaveToFile`/`SaveToStream` as PNG/JPEG/BMP/TIFF/GIF/JpegXR.
- **Pixel access** — get/set pixel bytes/colors, copy regions.
- **Construction** — from bytes, from colors, from a D3D11 surface / software
  bitmap; explicit pixel format and `CanvasAlphaMode`.
- **`CanvasRenderTarget`** — first-class off-screen target with size/DPI/format.
- **`CanvasCommandList`** — record drawing commands and replay / use as an effect
  input.
- **`CanvasVirtualBitmap`** and histogram / `GetBounds` helpers.

#### 6. Brushes *(medium-low)*

Present: solid color, linear gradient, radial gradient (stops fixed at creation),
`set_color` on solid brushes.

Missing: `CanvasImageBrush` (sample/tile an image or command list), brush
`Opacity` and `Transform`, gradient `EdgeBehavior` / interpolation space / alpha
mode / pre-interpolation, and HDR color variants.

#### 7. Hosting surfaces *(medium-low)*

Present: `animated_canvas` — a per-frame UI-thread loop on a `SwapChainPanel`
(architecturally Win2D's `CanvasAnimatedControl` render model), plus the
standalone `HWND` swap-chain path.

Missing vs Win2D's XAML controls:

- **Invalidation-only mode** (`CanvasControl`) — redraw on demand instead of every
  frame; ideal for static or rarely-changing content. This was tracked as a
  prospective `canvas()` builder.
- **Dedicated game-loop thread** (`CanvasAnimatedControl`'s independent loop with
  `Update`/`Draw`, fixed time step, input source) — a prospective
  `threaded_canvas()`.
- **Virtualized / tiled surfaces** (`CanvasVirtualControl`,
  `CanvasVirtualImageSource`) for very large content.
- **XAML image-source targets** (`CanvasImageSource`) and **composition interop**
  (`CanvasComposition`) for drawing into the visual layer.

#### 8. Lower-priority parity

- **Swap chain controls** — rotation, source size, transform matrix, buffer
  count/format/alpha, `WaitForVerticalBlank`.
- **Device management** — shared-device cache (`GetSharedDevice`), a `DeviceLost`
  event (today loss is polled), interop from an existing D3D11 device, debug
  level, capability queries.
- **Printing** — `CanvasPrintDocument`.
- **SVG** — `CanvasSvgDocument` / `CanvasSvgElement`.
- **Color management / HDR** — ICC profiles, `EffectTransferTable3D`, HDR color
  paths.

#### Suggested sequencing

Geometry ops (1) and text layout (2) unblock the most-requested scenarios
(hit-testing, measured/interactive text) and pair naturally with the reactor input
work in *Input and hit-testing*. Layers/clipping and sprite batch (3) follow for
richer and higher-performance drawing. Effects (4) and bitmap I/O (5) are
self-contained and can land independently. Brushes (6) are incremental. Alternate
hosting surfaces (7) are larger architectural pieces best taken once the drawing
API is fuller.

Note that *animation* is not in this list: Win2D has no animation engine of its
own (its `CanvasAnimatedControl` only provides the loop, which `animated_canvas`
already matches). Smooth motion in the frame loop is best driven by
[`windows-animation`](windows-animation.md), sampled each frame, rather than
reinvented here — a small helper wiring its animated variables into
`animated_canvas` is the intended integration.
