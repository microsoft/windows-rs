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

## Summary

Win2D solves a real problem for C# developers: Direct2D is hard to use from managed code.
In Rust, the `windows` crate already provides ergonomic, typed, zero-cost access to D2D/D3D —
so the drawing API layer of Win2D is not needed.

What IS worth learning from Win2D:
- **Device lifecycle management** (creation + shared device + device-lost recovery)
- **The convenience of bundled controls** (auto swap chain, auto resize, auto clear)
- **The retained-mode control pattern** (redraw-on-invalidate for non-animated content)

The right approach for windows-reactor is to provide thin helpers that eliminate boilerplate
while preserving full access to the underlying D2D/D3D APIs — not to build a Rust Win2D clone.
