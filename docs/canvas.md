# `windows-canvas` — Idiomatic 2D Graphics for Rust

`windows-canvas` is a safe, fast 2D rendering crate built on Direct2D, Direct3D 11,
DXGI, DirectWrite, and WIC. All COM interfaces are private implementation details —
users work only with Rust types.

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

## With `windows-reactor`

With `feature = "reactor"` enabled, canvas provides `animated_canvas()` — a widget
that handles device creation, swap chain management, resize, and the rendering loop.
Device-lost recovery is automatic.

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

## Standalone (without reactor)

Canvas works without reactor using a plain Win32 HWND:

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

## Samples

| Sample | Command | Description |
|--------|---------|-------------|
| Minimal examples | `cargo run -p canvas_minimal --example hello` | Per-concept demos (hello, text, path, brush, color, lines, shapes, transform, gradient, bitmap) |
| Circles | `cargo run -p canvas_circles` | Animated circles with reactor integration |
| Clock | `cargo run -p canvas_clock` | Analog clock |
| Standalone | `cargo run -p canvas_standalone` | Win32 HWND without reactor |

See [canvas-internals.md](canvas-internals.md) for architecture, design decisions,
and contributor information.
