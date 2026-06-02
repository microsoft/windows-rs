# `windows-canvas` — Idiomatic 2D Graphics for Rust

## Introducing `windows-canvas`

`windows-canvas` is a safe, fast 2D rendering crate built on Direct2D, Direct3D 11,
DXGI, DirectWrite, and WIC. It generates ~2K lines of bindings via `--minimal --flat`
(vs 200K+ from the full `windows` crate) for sub-second incremental builds.

All COM interfaces are private implementation details. Users work only with Rust types:

```rust
let device = GpuDevice::new()?;
let mut chain = device.create_swap_chain(800, 600)?;

let session = chain.begin_draw()?;
session.clear(Color::CORNFLOWER_BLUE);
session.fill_ellipse(&Ellipse::circle(center, 50.0), &brush);
session.draw_text("Hello", &format, &rect, &brush);
drop(session);
chain.present()?;
```

Key abstractions:
- **GpuDevice** — creates D3D11/D2D/DXGI/DWrite factories
- **SwapChain** — manages DXGI buffers, resize, present, device-lost detection
- **DrawingSession** — RAII BeginDraw/EndDraw with safe draw calls
- **PathBuilder** — typestate geometry builder (compile-time figure lifecycle)
- **TextFormat** — device-independent DWrite text formatting
- **Bitmap** — WIC-based image loading (requires COM initialization)
- **Paint** — sealed trait unifying Brush, LinearGradient, RadialGradient

---

## `windows-canvas` and `windows-reactor`

The two crates are companions: reactor owns UI, canvas owns 2D rendering.
They integrate seamlessly but each works standalone.

With `feature = "reactor"` enabled, canvas provides `animated_canvas()` — a widget
that handles device creation, swap chain management, resize, and the rendering loop.
Device-lost recovery is automatic (the device and swap chain are silently rebuilt).

```rust
use windows_canvas::*;
use windows_reactor::{App, Backdrop, Element, RenderCx};

fn main() -> windows_core::Result<()> {
    App::new()
        .title("My App")
        .backdrop(Backdrop::Mica)
        .render(|_cx: &mut RenderCx| -> Element {
            animated_canvas(|ctx| {
                ctx.clear(Color::CORNFLOWER_BLUE);
                let brush = ctx.create_solid_brush(Color::WHITE).unwrap();
                ctx.fill_ellipse(
                    &Ellipse::circle(Vector2::new(ctx.width / 2.0, ctx.height / 2.0), 50.0),
                    &brush,
                );
            })
            .into()
        })
}
```

The `DrawContext` passed to the closure derefs to `DrawingSession` and adds
`width`, `height`, and `device()` for creating paths and resources.

---

## Getting started with `windows-canvas` standalone

Canvas works without reactor using a plain Win32 HWND. Generate minimal window
bindings with a build script and use `create_swap_chain_for_hwnd`:

```rust
// build.rs
fn main() {
    windows_bindgen::bindgen(["--etc", "bindings.txt"]);
}

// main.rs
let device = GpuDevice::new()?;
let mut chain = unsafe { device.create_swap_chain_for_hwnd(hwnd, 800, 600)? };

loop {
    // pump messages...
    let session = chain.begin_draw()?;
    session.clear(Color::DARK_SLATE_BLUE);
    // draw...
    drop(session);
    chain.present()?;
}
```

See `crates/samples/canvas/standalone/` for a complete working example.

---

## Architecture

```
crates/libs/canvas/src/
├── lib.rs           # Re-exports only — bindings are private
├── bindings.rs      # Generated (--minimal --flat) D3D11/DXGI/D2D1/DWrite/WIC
├── types.rs         # Rect, Ellipse, RoundedRect, Brush, LinearGradient, RadialGradient
├── bitmap.rs        # Bitmap: WIC-based image loading + draw
├── device.rs        # GpuDevice: D3D11 + D2D1 + DXGI + DWrite factory creation
├── device_lost.rs   # HRESULT detection for device-lost errors
├── swap_chain.rs    # SwapChain: DXGI swap chain + D2D1 device context
├── session.rs       # DrawingSession: RAII BeginDraw/EndDraw, safe draw calls
├── color.rs         # Color (f32 RGBA) with named constants
├── text.rs          # TextFormat (DWrite), alignment enums
├── geometry.rs      # PathBuilder (typestate), Path
└── reactor.rs       # feature="reactor": animated_canvas() widget + DrawContext
```

---

## API Reference

```rust
// Brushes (reusable across frames)
let brush = chain.create_solid_brush(Color::RED)?;
brush.set_color(Color::BLUE);

// Gradients (per-session — sealed Paint trait unifies all three)
let linear = session.create_linear_gradient(
    start, end, &[GradientStop::new(0.0, Color::RED), GradientStop::new(1.0, Color::BLUE)]
)?;
let radial = session.create_radial_gradient(
    center, rx, ry, &[GradientStop::new(0.0, Color::WHITE), GradientStop::new(1.0, Color::BLACK)]
)?;
session.fill_rect(&rect, &linear);

// Shapes
session.fill_rounded_rect(&RoundedRect::uniform(rect, 12.0), &brush);
session.draw_line(p0, p1, &brush, 2.0);
session.with_transform(&matrix, || { /* scoped transform */ });

// Bitmaps (requires COM init)
let bitmap = chain.load_bitmap("image.png")?;
session.draw_bitmap(&bitmap, &dest_rect, 1.0);

// Text (device-independent)
let format = TextFormat::new("Segoe UI", 24.0)?
    .with_alignment(TextAlignment::Center);
session.draw_text("Hello", &format, &rect, &brush);

// Geometry (typestate)
let path = PathBuilder::new(&device)?
    .begin(point).line_to(p2).bezier_to(c1, c2, end).close()
    .build()?;
session.fill_path(&path, &brush);
```

---

## Design Decisions

### Informed by Win2D (~400K LOC C++)

1. **DrawingSession is mandatory** — RAII BeginDraw/EndDraw ensures lifecycle
   correctness and device-lost detection at EndDraw time.

2. **Device-lost recovery is automatic** — `animated_canvas` silently rebuilds
   device + swap chain on GPU reset. Draw closures recreate per-frame resources
   naturally. Matches Win2D's `RunWithDevice` pattern.

3. **PathBuilder as typestate** — Win2D's BeginFigure→Add→EndFigure→Close
   maps to compile-time state enforcement in Rust.

4. **TextFormat is device-independent** — DWrite factory is a shared singleton.
   TextFormats can be created and cached without a GPU device (same as Win2D).

5. **No geometry pooling** — Win2D doesn't pool path geometries either.
   Create on demand; they're cheap.

6. **Bindings are private** — all COM interfaces are implementation details.
   Users interact only with Rust types (`Brush`, `Rect`, `Ellipse`, etc.).
   This mirrors how `windows-reactor` hides its internal bindings.

### Divergences from Win2D

- **No WinRT layer** — direct Rust types, not COM projections
- **No device context pool** — single-threaded rendering (swap chain owns one context)

---

## What Works Well

- **Build time**: ~2.5K lines of bindings, <1s incremental
- **Ergonomics**: 126-line animated circles demo vs 247-line raw D2D equivalent
- **Type safety**: PathBuilder prevents misuse at compile time
- **Zero unsafe in user code**: all drawing calls are safe
- **Device-lost**: transparent recovery in animated_canvas
- **Consistency**: follows reactor patterns (factory functions, RAII, Deref delegation)
- **Transform stack**: set/get + scoped `with_transform` (matches Win2D TemporaryTransform)
- **Paint polymorphism**: sealed trait with zero-cost COM Deref (no QueryInterface)
- **Stroke styles**: `StrokeStyleBuilder` with caps, joins, dashes + `draw_*_styled()` methods
- **DPI-aware**: crisp rendering at all display scales via composition scale tracking
- **Bitmap loading**: WIC-based, supports PNG/JPEG/BMP/etc.
- **Test coverage**: 19 headless WARP-based tests covering full API surface
- **Examples**: 10 per-concept windowed demos

## What Needs Improvement

1. **Opacity layers** — needed for complex compositing.

2. **CanvasTextLayout** — metrics, hit-testing for text-heavy apps.

3. **Drop shadow effect** — D2D has shadow effects; not exposed in canvas.

---

## Open Issues

### Design Notes

- **`on_resize` / `on_ready` ordering** — `on_resize` stores the size in a
  `Cell`; `on_ready` reads it to create the initial swap chain. If `on_ready`
  fires first when size is still (0,0), `.max(1)` ensures a 1×1 chain, then
  `on_resize` resizes correctly. Not a bug — documented here for awareness.

- **`Brush` is created from both `SwapChain` and `DrawingSession`** — the split
  is intentional (cached across sessions vs per-frame), but gradients can only
  be created from `DrawingSession`. Consider adding gradient creation to
  `SwapChain` for consistency, or documenting why only solid brushes are cached.

---

## Roadmap

| # | Task | Priority | Status |
|---|------|----------|--------|
| 1 | Standalone HWND sample + `create_swap_chain_for_hwnd` | High | ✅ Done |
| 2 | Remove `DrawContext::session()` (Deref is sufficient) | High | ✅ Done |
| 3 | Fix Paint trait doc to list all implementors | High | ✅ Done |
| 4 | Fix bitmap example doc (explain per-frame load) | Medium | ✅ Done |
| 5 | Add missing test cases (error paths, hollow paths, multi-figure) | Medium | ✅ Done |
| 6 | Stroke style (dashes, line caps, line joins) | Low | |
| 7 | CanvasTextLayout (metrics, hit-testing) | Low | |
| 8 | Opacity layers | Low | |
| 9 | `canvas()` composition widget (invalidate-on-demand) | Deferred | |

---

## Notes

- **Bindings**: generated via `tool_bindings` using `crates/tools/bindings/src/canvas.txt`.
  Add new D2D/DWrite/DXGI methods there and re-run `cargo run -p tool_bindings`.

- **TextFormat caching**: DWrite returns the shared factory singleton, so creating
  TextFormats is cheap. But real apps should cache them outside the draw loop.

- **Minimal examples**: each example in `crates/samples/canvas/minimal/examples/` uses
  a shared harness (`canvas_minimal::run`) that opens a full-client-area window with
  `animated_canvas`. The example only provides the draw callback. Currently 10 examples:
  hello, text, path, brush, color, lines, shapes, transform, gradient, bitmap.

- **Standalone sample** (`crates/samples/canvas/standalone/`): uses a build.rs that
  generates minimal Win32 window bindings via `windows-bindgen --minimal --flat`.
  The D2D rendering comes from `windows-canvas` via `create_swap_chain_for_hwnd`.

- **`canvas()` deferred**: Reactor has no composition surface primitives. Adding
  `ISurfaceImageSource` is significant. `animated_canvas()` covers real-time use cases.
