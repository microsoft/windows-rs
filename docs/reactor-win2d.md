# windows-reactor vs Win2D: Analysis and Learnings

## Overview

This document compares [Win2D](https://github.com/microsoft/win2d) (a C#/C++ WinRT library for
immediate-mode 2D graphics) with the `SwapChainPanel` support just added to `windows-reactor`
([PR #4499](https://github.com/microsoft/windows-rs/pull/4499)). The goal is to determine what
windows-reactor can learn from Win2D's design, and whether we should provide Win2D interop, a
Rust equivalent, or rely on the existing raw D2D/D3D access.

---

## What Win2D Is

Win2D is a managed wrapper over Direct2D/Direct3D that provides:

| Layer | What it does |
|-------|-------------|
| **XAML controls** | `CanvasControl` (retained-mode redraw-on-invalidate), `CanvasAnimatedControl` (game-loop tick), `CanvasSwapChainPanel`, `CanvasVirtualControl` (virtualized large surfaces) |
| **Device management** | `CanvasDevice` — auto-creates D3D11 device, handles device-lost, reconnects |
| **Drawing session** | `CanvasDrawingSession` — friendly API over `ID2D1DeviceContext` with ~200 overloads for draw/fill/text/image |
| **Resources** | `CanvasBitmap`, `CanvasRenderTarget`, `CanvasSwapChain`, `CanvasCommandList` |
| **Geometry** | `CanvasGeometry`, `CanvasPathBuilder`, `CanvasCachedGeometry` |
| **Text** | `CanvasTextFormat`, `CanvasTextLayout`, inline objects, font management |
| **Effects pipeline** | 60+ GPU image effects (blur, blend, color matrix, lighting, …) |
| **SVG** | `CanvasSvgDocument` rendering |
| **Composition interop** | `CanvasComposition` bridges WinUI Composition + Win2D |

Win2D's source is ~400K lines of C++ implementing WinRT components. It is essentially a
C#-friendly projection of Direct2D concepts.

---

## What windows-reactor Provides Today

After PR #4499, reactor has:

1. **`SwapChainPanelWidget`** — a WinUI `SwapChainPanel` that:
   - Fires `on_ready(SwapChainPanelHandle)` with the native panel IInspectable.
   - `SwapChainPanelHandle::set_swap_chain(&impl Interface)` attaches any DXGI swap chain.
   - Fires `on_resize(w, h)` on layout size changes.
2. **`on_rendering` / `Rendering`** — a per-frame tick callback (CompositionTarget.Rendering).
3. Full access to raw `windows` crate APIs (D3D11, D2D1, DXGI, DWrite, etc.).

The current usage pattern (see `examples/direct2d.rs`):
```rust
swap_chain_panel()
    .on_ready(|panel| { /* create D3D device, swap chain, D2D context */ })
    .on_resize(|w, h| { /* resize buffers */ })
```
with a `thread_local!` holding a `D2DState` struct that the `on_rendering` callback drives.

---

## Comparison

| Concern | Win2D (C#) | windows-reactor (Rust) |
|---------|-----------|----------------------|
| **Device creation** | Automatic (`CanvasDevice.GetSharedDevice()`) | Manual ~20 lines of unsafe D3D/DXGI |
| **Device-lost recovery** | Built-in (CreateResources re-fires) | Not handled — app must implement |
| **Draw API** | `session.DrawEllipse(…)` — safe, typed | Raw `ID2D1DeviceContext` calls, `unsafe` |
| **DPI handling** | Automatic DPI scaling in controls | Manual (reactor exposes DPI but no D2D integration) |
| **Effects** | Strongly-typed effect graph | Raw `ID2D1Effect` COM calls |
| **Text** | `CanvasTextLayout` with rich formatting | Raw DWrite |
| **Game loop** | `CanvasAnimatedControl` (fixed/variable timestep) | `on_rendering` (variable only) |
| **Composition interop** | `CanvasComposition` helpers | Manual via `windows` crate |
| **Learning curve** | Low for C# devs (one NuGet) | Higher (raw graphics init boilerplate) |
| **Flexibility** | Constrained to Win2D types | Full D3D/D2D/compute freedom |
| **Binary size / deps** | ~5 MB native DLL | Zero extra deps (just windows crate) |
| **Performance** | Excellent (thin C++ over D2D) | Excellent (direct D2D calls) |

---

## Key Learnings from Win2D

### 1. Device Lifecycle Management is the #1 Pain Point

Win2D's biggest value is not the drawing API (which is just renaming D2D calls) — it's:
- **Auto device creation** — no DXGI factory chain boilerplate.
- **Device-lost handling** — D3D devices can be lost (driver updates, GPU hangs, display changes). Win2D fires `CreateResources` to let apps rebuild. Most raw D2D apps ignore this and crash.
- **Shared device** — multiple controls reuse one device.

**Takeaway**: Reactor should consider a lightweight `GpuDevice` helper or a `use_gpu_device()` hook that creates/shares a D3D11 device and handles device-lost by re-triggering the `on_ready` callback.

### 2. Drawing Session Abstraction — Not Needed in Rust

Win2D's `CanvasDrawingSession` exists because C# cannot call COM methods ergonomically.
In Rust, `windows` crate projections of `ID2D1DeviceContext` are already ergonomic:
```rust
context.DrawEllipse(&ellipse, &brush, 2.0, None);
```
Adding a Rust wrapper would be redundant overhead with no ergonomic win.

### 3. Controls with Built-in Render Loop Are Convenient

Win2D's `CanvasAnimatedControl` bundles:
- Swap chain creation
- Resize handling
- Fixed-timestep game loop
- Input forwarding
- Clear color

Our `SwapChainPanel` + `on_rendering` already covers this, but the boilerplate in `direct2d.rs`
(~70 lines just for init) suggests a higher-level builder could help.

### 4. Effects Pipeline Is Win2D's Unique Strength

The 60+ typed effects with automatic GPU graph compilation are where Win2D shines for C# devs.
However, Rust can call `ID2D1Effect` directly and the D2D effect system is well-documented.
This is not worth wrapping.

### 5. The CanvasControl (Retained) Pattern

`CanvasControl` only redraws when `Invalidate()` is called — it caches the last frame.
This is useful for non-animated content (charts, diagrams). Reactor's `SwapChainPanel` always
requires an explicit render loop. A retained-mode helper could be valuable.

---

## Recommendations

### Do NOT Do

1. **Port Win2D to Rust** — It would be a massive effort (400K+ LOC) with diminishing returns
   since Rust already has excellent Direct2D access. The value proposition of Win2D is
   "make D2D accessible to C#" — Rust doesn't need that.

2. **Add Win2D WinRT interop** — While technically possible (Win2D is a WinRT component), it
   would add a 5MB native dependency, lose Rust's ownership model benefits, and create a
   confusing layering (reactor → Win2D → D2D instead of reactor → D2D directly).

3. **Wrap every D2D concept** — Don't replicate the drawing session, geometry builders, etc.
   The `windows` crate already provides typed, documented access.

### DO Consider

1. **`GpuDevice` helper** (high value, low effort)
   ```rust
   // Proposed API sketch:
   let device = cx.use_gpu_device(); // or a free function
   // Returns a shared D3D11Device + D2D1Device, handles device-lost
   ```
   Encapsulates:
   - D3D11 device creation with BGRA support
   - DXGI factory acquisition
   - D2D1 factory + device creation
   - Device-lost detection + recreation callback

2. **`SwapChainPanel` builder with auto-setup** (medium value)
   ```rust
   swap_chain_panel()
       .auto_device()         // create device + swap chain automatically
       .on_draw(|session, size, dt| {
           // session: &ID2D1DeviceContext, already targeted
           session.DrawEllipse(…);
       })
   ```
   This would eliminate the ~70 lines of boilerplate in `direct2d.rs`.

3. **Device-lost recovery** (high value, medium effort)
   When the D3D device is lost, re-invoke `on_ready` so the app can rebuild resources.
   Win2D's `CreateResources` pattern is the model here.

4. **Fixed-timestep game loop option** (low-medium value)
   ```rust
   on_rendering_fixed(Duration::from_secs_f64(1.0 / 60.0), |dt| { … })
   ```
   Useful for game-like content where variable timestep causes physics jitter.

5. **Retained-mode canvas widget** (medium value, future)
   A `canvas_control()` that caches the rendered frame and only redraws on `.invalidate()`.
   Useful for charts, diagrams, and static graphical content that doesn't need per-frame updates.

---

## Next Steps

| # | Action | Priority | Effort |
|---|--------|----------|--------|
| 1 | Add a `GpuDevice` helper (shared D3D11+D2D1 device, device-lost) | High | Medium |
| 2 | Add `swap_chain_panel().auto_device().on_draw(…)` convenience | High | Low-Med |
| 3 | Document the manual pattern with a cookbook / guide | Medium | Low |
| 4 | Investigate retained-mode `canvas_control()` widget | Medium | Medium |
| 5 | Add fixed-timestep `on_rendering` variant | Low | Low |
| 6 | Add a "device lost" example showing recovery | Medium | Low |

---

---

## Proposal: `windows-canvas` Crate

### Motivation

While D2D/D3D APIs *are* accessible from Rust, the developer experience has friction:

1. **Build time** — The `windows` crate feature system (`Win32_Graphics_Direct2D`,
   `Win32_Graphics_Direct3D11`, `Win32_Graphics_Dxgi`, etc.) pulls in large amounts of
   generated code. Even a simple D2D sample adds 30+ seconds to incremental builds.
2. **Boilerplate** — Creating a D3D11 device → DXGI factory → swap chain → D2D factory →
   D2D device → device context → bitmap target is ~70 lines of unsafe initialization code
   that every app repeats identically.
3. **Unsafe everywhere** — Nearly every D2D/D3D call requires `unsafe` in the `windows` crate,
   even for operations that cannot actually fail unsafely (e.g. `SetTarget`, `BeginDraw`).
4. **No device-lost handling** — Apps crash when the GPU resets unless they manually implement
   recovery.
5. **Separation of concerns** — Developers building standard XAML apps with buttons/lists
   shouldn't pay build-time or dependency cost for graphics APIs they don't use.

### Design Principles

| Principle | Rationale |
|-----------|-----------|
| **Separate crate** | `windows-reactor` stays focused on UI; `windows-canvas` is opt-in |
| **Self-contained bindings** | Like reactor's `bindings.rs`, generate only the D2D/D3D/DXGI/DWrite subset — fast builds |
| **Safe by default** | Wrap the unsafe COM layer in safe Rust types where soundness can be guaranteed |
| **Escape hatch** | Always expose the raw COM interface for advanced usage |
| **Win2D-inspired, not Win2D-cloned** | Learn from its API shape but design for Rust idioms |
| **Composable with reactor** | Works standalone OR integrates with `SwapChainPanel` |

### Proposed Architecture

```
windows-canvas/
├── src/
│   ├── lib.rs
│   ├── bindings.rs          # Generated: minimal D3D11 + DXGI + D2D1 + DWrite subset
│   ├── device.rs            # GpuDevice: shared D3D11+D2D1, device-lost recovery
│   ├── swap_chain.rs        # SwapChain: creation, resize, present
│   ├── session.rs           # DrawingSession: safe wrapper over ID2D1DeviceContext
│   ├── brush.rs             # SolidBrush, LinearGradient, RadialGradient
│   ├── geometry.rs          # PathBuilder, Geometry (safe path construction)
│   ├── text.rs              # TextFormat, TextLayout (safe DWrite wrappers)
│   ├── image.rs             # Bitmap, RenderTarget
│   ├── effect.rs            # Typed effect wrappers (optional feature-gated)
│   └── reactor.rs           # Integration: CanvasPanel widget (feature = "reactor")
├── build.rs                 # Generates bindings.rs from winmd
└── Cargo.toml
```

### Key Types

```rust
/// Shared GPU device with automatic device-lost recovery.
pub struct GpuDevice { /* D3D11Device + D2D1Device + DXGIFactory */ }

impl GpuDevice {
    /// Create with default settings (hardware, BGRA support, feature level 11.0).
    pub fn new() -> Result<Self>;

    /// Subscribe to device-lost. Callback fires on the thread that detects the loss.
    pub fn on_device_lost(&self, f: impl Fn() + Send + 'static) -> EventRevoker;

    /// Create a composition swap chain sized to the given dimensions.
    pub fn create_swap_chain(&self, width: u32, height: u32) -> Result<SwapChain>;

    /// Access the underlying COM interfaces for advanced usage.
    pub fn d3d_device(&self) -> &ID3D11Device;
    pub fn d2d_device(&self) -> &ID2D1Device;
    pub fn dxgi_factory(&self) -> &IDXGIFactory2;
}
```

```rust
/// Manages a DXGI swap chain with safe resize and present.
pub struct SwapChain { /* IDXGISwapChain1 + cached size */ }

impl SwapChain {
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()>;
    pub fn present(&self) -> Result<()>;

    /// Begin a drawing session targeting this swap chain's back buffer.
    pub fn draw(&mut self) -> Result<DrawingSession<'_>>;
}
```

```rust
/// Safe wrapper over ID2D1DeviceContext. Calls BeginDraw on creation,
/// EndDraw on drop. Drawing methods are safe (no UB possible).
pub struct DrawingSession<'a> { /* &'a mut target context */ }

impl DrawingSession<'_> {
    pub fn clear(&self, color: Color);
    pub fn draw_line(&self, p0: Point, p1: Point, brush: &Brush, width: f32);
    pub fn draw_rect(&self, rect: Rect, brush: &Brush, width: f32);
    pub fn fill_rect(&self, rect: Rect, brush: &Brush);
    pub fn draw_ellipse(&self, center: Point, rx: f32, ry: f32, brush: &Brush, width: f32);
    pub fn fill_ellipse(&self, center: Point, rx: f32, ry: f32, brush: &Brush);
    pub fn draw_text(&self, text: &str, rect: Rect, brush: &Brush, format: &TextFormat);
    pub fn draw_geometry(&self, geometry: &Geometry, brush: &Brush, width: f32);
    pub fn fill_geometry(&self, geometry: &Geometry, brush: &Brush);
    pub fn draw_image(&self, image: &Bitmap, dest: Rect);
    pub fn push_layer(&self, bounds: Rect, opacity: f32) -> Layer<'_>;
    pub fn transform(&self) -> Matrix3x2;
    pub fn set_transform(&self, transform: Matrix3x2);

    /// Escape hatch: access the raw D2D1 device context.
    pub fn raw(&self) -> &ID2D1DeviceContext;
}
```

### Reactor Integration (feature-gated)

```rust
// In windows-canvas with feature = "reactor"
use windows_reactor::*;

/// A SwapChainPanel that auto-manages device + swap chain + resize.
/// Much simpler than the manual pattern.
pub fn canvas_panel() -> CanvasPanelWidget { ... }

impl CanvasPanelWidget {
    /// Called each frame with a ready-to-use drawing session.
    pub fn on_draw(self, f: impl Fn(&mut DrawingSession, Size, f64) + 'static) -> Self;

    /// Called once when the device is created (or recreated after device-lost).
    pub fn on_create_resources(self, f: impl Fn(&GpuDevice) + 'static) -> Self;
}
```

Usage becomes:
```rust
fn app(cx: &mut RenderCx) -> Element {
    canvas_panel()
        .on_create_resources(|device| {
            // Load bitmaps, create brushes, etc.
        })
        .on_draw(|session, size, dt| {
            session.clear(Color::rgb(13, 13, 26));
            session.fill_ellipse(
                Point::new(size.width / 2.0, size.height / 2.0),
                50.0, 50.0,
                &SolidBrush::new(Color::rgb(77, 153, 230)),
            );
        })
        .into()
}
```

Compare with today's manual pattern (~70 lines of unsafe init + thread_local + manual resize).

### Build Time Strategy

Like `windows-reactor`, `windows-canvas` would generate its own `bindings.rs` containing
**only** the interfaces it needs:

| From `windows` crate | Subset for `windows-canvas` |
|----------------------|----------------------------|
| `Win32_Graphics_Direct3D11` (huge) | `ID3D11Device`, `D3D11CreateDevice`, `ID3D11Texture2D` |
| `Win32_Graphics_Dxgi` (huge) | `IDXGIFactory2`, `IDXGISwapChain1`, `IDXGIDevice`, `IDXGISurface` |
| `Win32_Graphics_Direct2D` (huge) | `ID2D1Factory1`, `ID2D1Device`, `ID2D1DeviceContext`, `ID2D1Bitmap1`, brushes, geometry |
| `Win32_Graphics_DirectWrite` (huge) | `IDWriteFactory`, `IDWriteTextFormat`, `IDWriteTextLayout` |

This targeted generation means:
- **No dependency on the `windows` crate** for consumers
- **Fast incremental builds** (the bindings file is ~5-10K lines vs 200K+ for the full features)
- **Zero feature-flag complexity** for users

### What NOT to Include (keep scope tight for v0.1)

- ❌ 3D rendering helpers (that's a different domain)
- ❌ Full effects pipeline (expose raw `ID2D1Effect` escape hatch instead)
- ❌ SVG document rendering (niche, users can call D2D SVG directly)
- ❌ Ink/handwriting support
- ❌ Printing
- ❌ Video/media rendering

These can be added as features later if demand exists.

### Comparison: Today vs `windows-canvas`

**Today** (direct2d.rs example — 277 lines):
```
├── Manually create D3D11 device (unsafe, 15 lines)
├── Manually create D2D factory + device (unsafe, 5 lines)
├── Manually create DXGI swap chain (unsafe, 20 lines)
├── Manually wire swap chain to panel (5 lines)
├── Manually create bitmap target (unsafe, 15 lines)
├── Manually handle resize (unsafe, 20 lines)
├── thread_local! state management (10 lines)
├── Manual BeginDraw/EndDraw (unsafe)
└── No device-lost handling
```

**With `windows-canvas`** (~30 lines):
```
├── canvas_panel().on_create_resources(...).on_draw(...)
├── Device creation: automatic
├── Swap chain: automatic
├── Resize: automatic
├── Device-lost: automatic (re-fires on_create_resources)
├── Drawing: safe DrawingSession API
└── Escape hatch: session.raw() for anything custom
```

---

## Revised Next Steps

| # | Action | Priority | Effort | Crate |
|---|--------|----------|--------|-------|
| 1 | Create `windows-canvas` crate with `GpuDevice` + `SwapChain` + `DrawingSession` | **High** | Medium | `windows-canvas` |
| 2 | Generate focused `bindings.rs` (D3D11/DXGI/D2D1/DWrite subset) | **High** | Medium | `windows-canvas` |
| 3 | Add `canvas_panel()` reactor integration (feature-gated) | **High** | Low | `windows-canvas` |
| 4 | Device-lost detection + recovery + `on_create_resources` pattern | **High** | Medium | `windows-canvas` |
| 5 | Safe geometry builder (`PathBuilder` → `Geometry`) | Medium | Low | `windows-canvas` |
| 6 | Safe text layout (`TextFormat`, `TextLayout`) | Medium | Low | `windows-canvas` |
| 7 | Retained-mode `canvas_control()` (invalidate-based redraw) | Medium | Medium | `windows-canvas` |
| 8 | Fixed-timestep rendering option | Low | Low | `windows-canvas` |
| 9 | Port `direct2d.rs` example to use `windows-canvas` | Medium | Low | samples |
| 10 | Document cookbook / migration guide | Medium | Low | docs |

---

## Summary

Win2D solves a real problem for C# developers: Direct2D is hard to use from managed code.
Rust has the same problem — while the `windows` crate *can* call D2D, it's verbose, unsafe,
and adds significant build time.

A dedicated `windows-canvas` crate addresses this by:
- **Isolating build cost** — reactor users who don't need graphics don't pay for it
- **Generating minimal bindings** — fast compilation, no feature-flag maze
- **Providing safe, ergonomic drawing APIs** — inspired by Win2D's `DrawingSession` pattern
- **Handling device lifecycle** — the #1 lesson from Win2D (device creation, sharing, recovery)
- **Integrating cleanly with reactor** — via a feature-gated `canvas_panel()` widget

This is not a Win2D port. It's a Rust-native drawing library that learns from Win2D's design
while preserving full access to the underlying Direct2D/Direct3D APIs for advanced use cases.
