# Canvas Internals

This document covers the architecture, design decisions, and Win2D comparison
for contributors working on `windows-canvas`. For the user-facing API guide,
see [canvas.md](canvas.md).

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
├── color.rs         # ColorF (f32 RGBA) with named constants
├── text.rs          # TextFormat (DWrite), alignment enums
├── geometry.rs      # PathBuilder (typestate), Path
└── reactor.rs       # feature="reactor": animated_canvas() widget + DrawContext
```

### Bindings

Generated via `tool_bindings` using `crates/tools/bindings/src/canvas.txt`.
To add new D2D/DWrite/DXGI methods, add them to the filter file and re-run:

```sh
cargo run -p tool_bindings
cargo check -p windows-canvas --quiet
```

The bindings use `--minimal --flat` mode — ~2K lines vs 200K+ from the full
`windows` crate, giving sub-second incremental builds.

---

## Design Decisions (Informed by Win2D)

Win2D is a ~400K LOC C++ library. These decisions were made with full knowledge
of Win2D's approach:

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
   Users interact only with Rust types. This mirrors how `windows-reactor`
   hides its internal bindings.

### Divergences from Win2D

- **No WinRT layer** — direct Rust types, not COM projections
- **No device context pool** — single-threaded rendering (swap chain owns one context)

---

## Win2D Comparison and Feature Parity

The goal is to reach parity with Win2D's two XAML controls in idiomatic Rust.

### Win2D Controls

| | CanvasControl | CanvasAnimatedControl |
|---|---|---|
| Thread | UI thread | **Dedicated game loop thread** |
| Render target | `CanvasImageSource` | `SwapChainPanel` |
| Trigger | `Invalidate()` → one draw | Continuous loop |
| Key events | `CreateResources`, `Draw` | `CreateResources`, `Update`, `Draw` |
| Timing | N/A | `CanvasTimingInformation` (total, elapsed, running slowly) |
| Fixed timestep | N/A | `IsFixedTimeStep` / `TargetElapsedTime` |
| Pause | N/A | `Paused` — skips Update/Draw, tracks paused time |
| Device-lost | Auto-recreate + re-raise `CreateResources` | Same |

**CanvasControl** draws only when `Invalidate()` is called — ideal for static
content. It renders on the UI thread via `CompositionTarget.Rendering`.

**CanvasAnimatedControl** runs Update/Draw on a **dedicated game loop thread**
via `DispatcherQueue`. The swap chain exists specifically to enable off-thread
`Present()`. Key threading detail: Win2D's `GameLoopThread` spawns a
threadpool thread, creates its own `DispatcherQueueController`, and ticks via
self-perpetuating dispatcher callbacks. Shared state (size, DPI, pause) is
exchanged via a mutex held briefly at tick start.

### Current State vs Win2D

**`animated_canvas` renders on the UI thread** via `CompositionTarget::Rendering`.
This is architecturally equivalent to Win2D's `CanvasControl` (not
`CanvasAnimatedControl`). The swap chain's primary benefit — off-thread
presentation — is not currently utilized.

| Feature | Win2D | `windows-canvas` | Status |
|---------|-------|-------------------|--------|
| GPU device creation + recovery | ✅ | ✅ | Done |
| Swap chain resize + DPI | ✅ | ✅ | Done |
| Per-frame draw closure | ✅ | ✅ | Done |
| Stroke styles | ✅ | ✅ | Done |
| Bitmaps (WIC) | ✅ | ✅ | Done |
| Text rendering (DWrite) | ✅ | ✅ | Done |
| Geometry paths | ✅ | ✅ | Done |
| Gradients | ✅ | ✅ | Done |
| Transforms | ✅ | ✅ | Done |
| Invalidation-only mode (`canvas()`) | `CanvasControl` | ❌ | Gap |
| Timing info in DrawContext | `CanvasTimingInformation` | ❌ | Gap |
| Pause / resume | `Paused` property | ❌ | Gap |
| Fixed timestep | `IsFixedTimeStep` | ❌ | Gap |
| **Dedicated render thread** | **GameLoopThread** | **❌** | **Key gap** |
| Opacity layers | ✅ | ❌ | Gap |
| CanvasTextLayout (metrics) | ✅ | ❌ | Gap |

### Planned Entry Points

#### `canvas()` — Invalidation-Driven (Win2D `CanvasControl`)

For static/rarely-changing content (charts, diagrams):

```rust
let handle = use_canvas_handle();

canvas(|ctx| {
    ctx.clear(Color::WHITE);
    draw_chart(ctx, &data);
}).handle(handle.clone())

// On data change:
handle.invalidate();
```

Draws once, then only on `invalidate()` or resize. Much lower GPU/CPU cost.

#### `animated_canvas()` — UI-Thread Continuous (current, to be enhanced)

Current behavior. Planned enhancements:

- `FrameTiming` in `DrawContext` (total_time, elapsed_time, frame_count)
- `.paused(bool)` builder
- `.clear_color(color)` auto-clear

#### `threaded_canvas()` — Dedicated Thread (Win2D `CanvasAnimatedControl`)

For games, simulations, heavy rendering:

```rust
threaded_canvas::<GameState>()
    .init(|| GameState::new())
    .update(|state, timing| { state.step(timing.elapsed_time); })
    .draw(|ctx, state| { render_game(ctx, state); })
```

Key differences from `animated_canvas`:
- Dedicated render thread with own `DispatcherQueue`
- Draw closure is `Send` — cannot access reactor state directly
- Explicit state parameter
- UI-to-render communication via channels or `Arc<Mutex<T>>`

### Implementation Phases

| Phase | Feature | Status |
|-------|---------|--------|
| 1 | Add `FrameTiming` to `animated_canvas` DrawContext | Not started |
| 2 | Add `canvas()` with invalidation | Not started |
| 3 | Pause/resume for `animated_canvas` | Not started |
| 4 | Auto-clear, target FPS | Not started |
| 5 | `threaded_canvas` (dedicated render thread) | Not started |

### Threading Model Summary

| | `canvas()` | `animated_canvas()` | `threaded_canvas()` |
|---|---|---|---|
| **Thread** | UI | UI | Dedicated |
| **Win2D equivalent** | `CanvasControl` | (no direct equivalent) | `CanvasAnimatedControl` |
| **Frame trigger** | `invalidate()` | `CompositionTarget::Rendering` | Self-ticking loop |
| **Can access reactor state** | ✅ | ✅ | ❌ (channels/shared) |
| **Blocks UI** | Only on draw | Every frame | Never |
| **Best for** | Static content | Lightweight animations | Games, simulations |

---

## Open Issues

- **`on_resize` / `on_mounted` ordering** — `on_resize` stores the size in a
  `Cell`; `on_mounted` reads it to create the initial swap chain. If `on_mounted`
  fires first when size is still (0,0), `.max(1)` ensures a 1×1 chain, then
  `on_resize` resizes correctly. Not a bug — documented here for awareness.

- **`Brush` is created from both `SwapChain` and `DrawingSession`** — the split
  is intentional (cached across sessions vs per-frame), but gradients can only
  be created from `DrawingSession`.

- **`canvas()` deferred** — Reactor has no composition surface primitives.
  Adding `ISurfaceImageSource` is significant. `animated_canvas()` covers
  real-time use cases.

---

## Samples

| Path | Description |
|------|-------------|
| `crates/samples/canvas/minimal/examples/` | 10 per-concept demos (hello, text, path, brush, color, lines, shapes, transform, gradient, bitmap) using shared `canvas_minimal::run` harness |
| `crates/samples/canvas/standalone/` | Win32 HWND without reactor, uses `create_swap_chain_for_hwnd` |
| `crates/samples/canvas/circles/` | Animated circles with reactor integration |
| `crates/samples/canvas/clock/` | Analog clock |

---

## Testing

19 headless WARP-based tests covering the full API surface. Run with:

```sh
cargo test -p windows-canvas
```

Tests use `GpuDevice::new_warp()` for software rendering without a GPU.
