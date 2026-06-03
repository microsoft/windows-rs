# Win2D CanvasControl and windows-canvas

This document compares Win2D's `CanvasControl` / `CanvasAnimatedControl` with
the current `windows-canvas` + `windows-reactor` integration and outlines a
path toward feature parity in idiomatic Rust.

---

## Win2D Architecture (C#/C++)

Win2D provides two XAML controls for GPU-accelerated 2D rendering. The
fundamental architectural difference between them is **threading**:

| | CanvasControl | CanvasAnimatedControl |
|---|---|---|
| Thread | UI thread | **Dedicated game loop thread** |
| Render target | `CanvasImageSource` | `SwapChainPanel` |
| Trigger | `Invalidate()` → one draw | Continuous loop |
| Why swap chain? | N/A (uses image source) | Swap chain can present from any thread |

### CanvasControl — UI-Thread, Invalidation-Driven

A `UserControl` hosting a `CanvasImageSource`. It draws **only** when
`Invalidate()` is called, making it ideal for static or rarely-changing
content (charts, diagrams, rendered text).

**Renders on the UI thread** via `CompositionTarget.Rendering`. When
`Invalidate()` is called, it sets a dirty flag and hooks into the
composition rendering callback. On the next vsync, the `Draw` event fires
(still on the UI thread), then the hook is removed until the next
`Invalidate()`.

Because it renders on the UI thread and uses `CanvasImageSource` (not a swap
chain), it cannot block or run independently — it's synchronized with XAML's
own composition cycle.

**Public API:**

| Category | API |
|----------|-----|
| Events | `CreateResources(CanvasCreateResourcesEventArgs)` |
| | `Draw(CanvasDrawEventArgs)` |
| Properties | `ClearColor`, `Size`, `ReadyToDraw`, `DpiScale` |
| | `UseSharedDevice`, `ForceSoftwareRenderer`, `CustomDevice` |
| Methods | `Invalidate()`, `RemoveFromVisualTree()` |

**Device lifecycle:**
- `CreateResources` fires with a `reason` enum:
  - `FirstTime` — initial device creation
  - `NewDevice` — device was lost and recreated
  - `DpiChanged` — display scaling changed
- On device loss, the control automatically recreates the device and re-raises
  `CreateResources` so the app can reload GPU resources (bitmaps, effects, etc.)

### CanvasAnimatedControl — Dedicated Thread, Continuous Loop

A `UserControl` hosting a `CanvasSwapChainPanel`. It runs a **dedicated game
loop thread** that calls `Update` and `Draw` independently of the UI thread.

#### Why a Separate Thread Matters

The entire point of using a `SwapChainPanel` (rather than an image source) is
that it enables **off-thread rendering**:

1. The game loop thread owns the D2D device context and calls `Update`/`Draw`
   at its own cadence — it is not blocked by XAML layout, input processing,
   or other UI-thread work.
2. `Present()` is called from the game loop thread. The DWM composites the
   swap chain's back buffer independently of XAML's composition pass.
3. If the UI thread is busy (e.g., handling a large tree diff in reactor),
   the animation keeps running smoothly.
4. Conversely, if the draw callback is expensive, the UI thread remains
   responsive for input and layout.

**If rendering is synchronized to the UI thread's composition loop, a swap
chain panel provides no advantage over a `CanvasImageSource`/composition
surface.** The swap chain exists specifically to enable independent-thread
presentation.

#### Threading Implementation (GameLoopThread)

Win2D's `GameLoopThread` (`GameLoopThread.cpp`):

1. Spawns a **long-running threadpool thread** via
   `ThreadPool::RunWithPriorityAndOptionsAsync`
2. Creates a `DispatcherQueueController` on that thread (its own message pump)
3. The tick loop runs as a series of dispatcher callbacks:
   - `ScheduleTick()` → `RunAsync(tickHandler)` posts to the game thread's
     dispatcher
   - `Tick()` runs → calls `CanvasAnimatedControl::Tick()` which does
     Update + Draw + Present
   - `TickCompleted()` schedules the next tick (self-perpetuating loop)
4. Shared state (size, DPI, pause, timing config) is exchanged between UI and
   game threads via a mutex (`m_sharedStateMutex`) — held briefly at the
   start of each tick to snapshot values.

```
UI Thread                           Game Loop Thread
─────────                           ────────────────
                                    ThreadPool::RunAsync → ThreadMain()
                                    CreateDispatcherQueueController()
                                    OnGameLoopStarting()
                                    
  Invalidate() ─── mutex ───────►   Tick():
  Resize()     ─── mutex ───────►     snapshot shared state
  Pause()      ─── mutex ───────►     StepTimer.Tick() → Update callbacks
                                      Draw callback + Present()
                                      ScheduleTick() (loop)
                                    
  RunOnGameLoopThreadAsync() ────►  dispatcher.TryEnqueue(action)
```

#### Public API

| Category | API |
|----------|-----|
| Events | `CreateResources(CanvasCreateResourcesEventArgs)` |
| | `Update(CanvasAnimatedUpdateEventArgs)` |
| | `Draw(CanvasAnimatedDrawEventArgs)` |
| | `GameLoopStarting`, `GameLoopStopped` |
| Properties | `IsFixedTimeStep`, `TargetElapsedTime`, `Paused` |
| | `ClearColor`, `Size`, `ReadyToDraw`, `DpiScale`, `SyncInterval` |
| | `UseSharedDevice`, `ForceSoftwareRenderer`, `CustomDevice` |
| Methods | `Invalidate()`, `ResetElapsedTime()`, `RemoveFromVisualTree()` |
| | `RunOnGameLoopThreadAsync()`, `HasGameLoopThreadAccess` |

**Timing model (`CanvasTimingInformation`):**
- `UpdateCount` — total number of Update calls so far
- `TotalTime` — wall-clock time since the loop started
- `ElapsedTime` — time since last frame (or fixed step size)
- `IsRunningSlowly` — true when fixed-step is falling behind

**StepTimer:**
- Fixed timestep (default): target = 1/60s. If the frame takes longer, multiple
  `Update` calls are issued to catch up (with `IsRunningSlowly` flag).
- Variable timestep: `Update` called once per frame with actual elapsed time.
- `ResetElapsedTime()` resets catch-up state (useful after loading screens).

**Pause semantics:**
- `Paused = true`: game loop thread still ticks but skips Update/Draw. Time
  spent paused is tracked and subtracted when resumed.
- `Invalidate()` while paused: forces exactly one Draw (no Update), useful for
  "draw the current state" without advancing simulation.

### Shared Infrastructure

- **RecreatableDeviceManager** — state machine for device create → commit →
  lost → recover. Tracks async `CreateResources` work via `TrackAsyncAction`.
- **BaseControl** — DPI tracking, XAML Loaded/Unloaded/SizeChanged hooks,
  `RunWithRenderTarget()` that coordinates device + render target creation.
- **ImageControlMixIn** — manages the `CanvasImageSource` render target for
  `CanvasControl`.

---

## Current windows-canvas / windows-reactor

### Low-Level (`windows-canvas` crate)

| Type | Role |
|------|------|
| `GpuDevice` | Owns D3D11 device, D2D factory/device, DXGI factory, DWrite factory |
| `SwapChain` | Wraps `IDXGISwapChain1` + `ID2D1DeviceContext` |
| `DrawingSession` | RAII `BeginDraw`/`EndDraw` with full D2D drawing API |
| `Color`, `StrokeStyle`, `Bitmap`, etc. | Drawing primitives |

**Pipeline:**
```
GpuDevice::new()
  → device.create_swap_chain(w, h) → SwapChain
    → chain.begin_draw() → DrawingSession
      → draw...
      → drop session (EndDraw)
    → chain.present()
```

### Reactor Integration (`animated_canvas`)

```rust
animated_canvas(|ctx| {
    ctx.clear(Color::WHITE);
    ctx.fill_ellipse(...);
})
```

- Hosts a `SwapChainPanel` widget
- Hooks `CompositionTarget::Rendering` for per-frame callbacks
- Creates `DrawContext` each frame: `Deref<Target=DrawingSession>` + `device()` + `width`/`height`
- Handles device loss silently (rebuild on present failure)
- Tracks DPI/scale changes and resizes the swap chain

### Threading Model — Key Architectural Observation

**The current `animated_canvas` renders on the UI thread.** It uses
`CompositionTarget::Rendering` (the XAML composition heartbeat) as its frame
clock. This makes it architecturally equivalent to **Win2D's `CanvasControl`**,
not `CanvasAnimatedControl`.

Implications:
- If the UI thread is busy (layout, input, reactor tree diff), frames are
  skipped — the draw closure simply isn't called that frame.
- If the draw closure is expensive, it blocks the UI thread — input lag,
  layout stutters.
- **The swap chain panel provides no threading benefit in this mode.** A
  composition surface (like Win2D's `CanvasImageSource`) would achieve the
  same result with less overhead.

The swap chain is still useful for:
- Multi-buffer flip presentation (lower latency than BitBlt)
- Future off-thread rendering (the swap chain *can* present from any thread)
- AlphaMode control for transparent overlays

But the **primary motivation** for a swap chain — independent-thread
rendering — is not currently utilized.

### What Exists Today

| Feature | Status |
|---------|--------|
| GPU device creation + recovery | ✅ |
| Swap chain resize + DPI | ✅ |
| Per-frame draw closure | ✅ |
| Stroke styles | ✅ |
| Bitmaps | ✅ |
| Text rendering | ✅ |
| Geometry paths | ✅ |
| Gradients | ✅ |
| Transforms | ✅ |

### What's Missing

| Feature | Win2D | windows-canvas | Notes |
|---------|-------|----------------|-------|
| Invalidation-only mode | `CanvasControl` | ❌ | Lower cost for static content |
| Timing info | `CanvasTimingInformation` | ❌ | Trivial to add |
| Pause / resume | `Paused` property | ❌ | |
| CreateResources event | ✅ (with reason) | ❌ (implicit) | Rust RAII may suffice |
| Fixed timestep | `IsFixedTimeStep` | ❌ | |
| Target frame rate | `TargetElapsedTime` | ❌ (always vsync) | |
| **Game loop thread** | **dedicated thread** | **❌ (UI thread)** | **Key gap** |
| Auto-clear color | `ClearColor` property | ❌ (manual) | |
| Invalidate-when-paused | ✅ | ❌ | |

---

## Proposed Design

### Philosophy

Win2D's API is shaped by C# conventions (events, properties, WinRT
projections). The Rust equivalent should be shaped by Rust conventions:
closures, builders, RAII, and the reactor component model.

Key design principles:
- **No explicit CreateResources event.** Rust's RAII + `thread_local!` /
  `OnceCell` pattern means resources are created lazily in the draw closure
  and automatically cleaned up. Device loss triggers a silent rebuild.
- **UI-thread animated canvas = CanvasControl equivalent.** The current
  `animated_canvas` is correctly modeled as a UI-thread continuous render —
  it should NOT be confused with Win2D's CanvasAnimatedControl which runs
  on a separate thread.
- **Threaded canvas is a distinct, opt-in feature.** True off-thread
  rendering requires fundamentally different state management (the closure
  cannot freely access reactor state). It should be a separate API.

### Three Entry Points

#### `canvas()` — Static/Invalidation-Driven (Win2D `CanvasControl` equivalent)

For content that doesn't change every frame (charts, rendered documents, etc.):

```rust
/// A canvas that draws once, then only redraws when invalidated.
fn canvas(draw: impl Fn(&DrawContext<'_>) + 'static) -> CanvasWidget
```

Usage in a reactor component:

```rust
let handle = use_canvas_handle();

canvas(|ctx| {
    ctx.clear(Color::WHITE);
    draw_chart(ctx, &data);
})
.handle(handle.clone())

// In an event handler:
handle.invalidate();
```

**Behavior:**
- Draws once when the panel is ready
- Redraws only when `invalidate()` is called or the surface resizes
- Device loss triggers automatic redraw
- Much lower GPU/CPU cost for static content

#### `animated_canvas()` — UI-Thread Continuous (current behavior, enhanced)

For content that changes every frame but doesn't need thread independence
(clocks, UI transitions, data visualizations):

```rust
/// A canvas that redraws every frame on the UI thread.
fn animated_canvas(draw: impl Fn(&DrawContext<'_>) + 'static) -> AnimatedCanvasWidget
```

This is the existing API with timing information added. The closure runs on
the UI thread, meaning it can freely access reactor component state captured
in the closure. This is intentionally simpler than Win2D's threaded model.

**When to use:** UI-driven animations where the draw callback is cheap (< 2ms)
and doesn't need to run independently of layout/input.

With `DrawContext` enhanced to include timing:

```rust
pub struct DrawContext<'a> {
    session: DrawingSession<'a>,
    device: &'a GpuDevice,
    pub width: f32,
    pub height: f32,
    pub timing: FrameTiming,
}

#[derive(Clone, Copy, Debug)]
pub struct FrameTiming {
    /// Seconds since the first frame.
    pub total_time: f64,
    /// Seconds since the last frame.
    pub elapsed_time: f64,
    /// Number of frames rendered.
    pub frame_count: u64,
}
```

Builder methods:

```rust
animated_canvas(|ctx| { ... })
    .paused(is_paused)       // stop/start the render loop
    .clear_color(Color::WHITE) // auto-clear before each frame
```

#### `threaded_canvas()` — Dedicated Render Thread (Win2D `CanvasAnimatedControl` equivalent)

For demanding workloads that need to decouple rendering from the UI thread
(games, simulations, complex visualizations):

```rust
/// A canvas that renders on a dedicated background thread.
fn threaded_canvas(
    update: impl Fn(&mut State, &FrameTiming) + Send + 'static,
    draw: impl Fn(&DrawContext<'_>, &State) + Send + 'static,
) -> ThreadedCanvasWidget
```

**Key architectural differences from `animated_canvas`:**
- Runs on a **dedicated thread** with its own message pump
- The draw closure **cannot** access reactor state directly (it's `Send`)
- State is explicitly passed via the `State` generic
- UI-to-render communication uses a channel or shared state with mutex

**When to use:** When draw takes > 2ms, when you need guaranteed frame rate
independent of UI complexity, or when you have physics/simulation that must
tick at a fixed rate regardless of UI load.

Proposed threading model (following Win2D's `GameLoopThread` approach):

```
UI Thread                              Render Thread
─────────                              ─────────────
                                       std::thread::spawn → thread main
                                       create DispatcherQueue (own pump)

  .property(value) ─── channel ──────► update shared state
  reactor re-render ─── channel ──────► property change notification
                                       
                                       loop:
                                         step_timer.tick()
                                         update(&mut state, &timing)
                                         begin_draw → draw(ctx, &state)
                                         present()
                                         wait_for_vsync / target_fps
```

**State sharing options:**

```rust
// Option A: Channel-based (preferred for Rust — no locks in hot path)
threaded_canvas::<GameState>()
    .init(|| GameState::new())
    .update(|state, timing| { state.physics_step(timing.elapsed_time); })
    .draw(|ctx, state| { render_game(ctx, state); })
    .on_input(tx.clone())  // sender for input events

// Option B: Arc<Mutex<T>> shared state (simpler, tiny critical sections)
let shared = Arc::new(Mutex::new(AppState::default()));
threaded_canvas()
    .state(shared.clone())
    .draw(|ctx, state| { ... })
```

### Implementation Phases

#### Phase 1: Add FrameTiming to `animated_canvas` (non-breaking) ✅ Ready

Add `FrameTiming` to `DrawContext`. The existing `animated_canvas` API is
unchanged — timing is simply available as a new field. No new types or
widgets.

```rust
pub struct DrawContext<'a> {
    // ... existing fields ...
    pub timing: FrameTiming,
}
```

Internally, track a `std::time::Instant` at first frame and compute
`total_time` / `elapsed_time` / `frame_count` each frame.

#### Phase 2: Add `canvas()` with invalidation ✅ Ready

Introduce the invalidation-driven canvas. Implementation:
- Same `SwapChainPanel` host as `animated_canvas`
- Subscribe to `CompositionTarget::Rendering` only when dirty
- Expose `CanvasHandle` with `invalidate()` method
- Auto-invalidate on resize and device loss

Alternatively, consider whether `canvas()` should use a composition surface
instead of a swap chain (lower overhead, no vsync needed for static content).

#### Phase 3: Pause/Resume for animated_canvas ✅ Ready

Add `.paused(bool)` builder:
- When paused, unsubscribe from `CompositionTarget::Rendering`
- `invalidate()` schedules exactly one frame (draw without advancing timing)
- Resuming re-subscribes and continues timing from where it left off

#### Phase 4: Auto-clear, target FPS

- `.clear_color(color)` — auto-clear before calling the draw closure
- `.target_fps(30)` — skip frames to hit a target rate (useful for
  battery-conscious apps)

#### Phase 5: Threaded canvas (major feature)

Dedicated render thread with independent tick loop. Key implementation work:

1. **Thread creation:** `std::thread::spawn` a render thread
2. **DispatcherQueue:** Create via `CreateDispatcherQueueController` on the
   render thread (needed for WinRT async operations on that thread)
3. **Tick loop:** Self-scheduling via dispatcher callbacks (like Win2D) or a
   simple `loop` + `thread::sleep` with high-resolution timer
4. **StepTimer:** Port Win2D's fixed/variable timestep logic
5. **State synchronization:** Channel or `Arc<Mutex<T>>` for UI→render data
6. **Lifecycle:** Stop thread on widget removal, handle device loss by
   signaling thread to recreate resources
7. **SwapChain ownership:** The render thread owns the `DrawingSession`;
   `Present()` is called from the render thread

**Open questions:**
- Should `threaded_canvas` use a separate D2D device context (thread-safe
  by design) or share the device with UI-thread canvases?
- How to handle reactor state that the draw closure needs — should we
  require explicit `Send + Sync` state, or provide a mechanism to snapshot
  reactor state on the UI thread and send it to the render thread?
- Should the render thread have its own `GpuDevice` or share one with the
  rest of the application?

---

## Comparison: Win2D C# vs Proposed Rust

### Animated (UI-thread)

```csharp
// C# Win2D — CanvasAnimatedControl (runs on game loop thread)
<canvas:CanvasAnimatedControl Draw="OnDraw" />

void OnDraw(CanvasAnimatedControl sender, CanvasAnimatedDrawEventArgs args) {
    var ds = args.DrawingSession;
    var t = args.Timing.TotalTime.TotalSeconds;
    ds.Clear(Colors.White);
    ds.DrawCircle(x, y, r, Colors.Blue);
}
```

```rust
// Rust windows-canvas — animated_canvas (runs on UI thread)
animated_canvas(|ctx| {
    let t = ctx.timing.total_time;
    ctx.clear(Color::WHITE);
    ctx.fill_ellipse(
        &Ellipse { center: (x, y).into(), radius_x: r, radius_y: r },
        &brush,
    );
})
```

Note: The Rust `animated_canvas` runs on the UI thread — it is simpler (can
access component state directly) but does NOT provide the threading isolation
that Win2D's `CanvasAnimatedControl` does. For that, use `threaded_canvas`.

### Threaded (dedicated render thread)

```csharp
// C# Win2D — runs Update + Draw on dedicated thread
void OnUpdate(CanvasAnimatedControl s, CanvasAnimatedUpdateEventArgs e) {
    gameState.Step(e.Timing.ElapsedTime);
}
void OnDraw(CanvasAnimatedControl s, CanvasAnimatedDrawEventArgs e) {
    RenderGame(e.DrawingSession, gameState);
}
```

```rust
// Rust — threaded_canvas (proposed, not yet implemented)
threaded_canvas::<GameState>()
    .init(|| GameState::new())
    .update(|state, timing| { state.step(timing.elapsed_time); })
    .draw(|ctx, state| { render_game(ctx, state); })
```

The Rust version is more concise because:
- No event handler boilerplate (closures capture state directly)
- No `sender` argument (reactor manages the control)
- Drawing session is accessed via `Deref` (no `.DrawingSession` getter)
- Timing is a simple struct field (no `.Timing.TotalTime.TotalSeconds` chain)
- State ownership is explicit (generic `State` parameter)

---

## Threading Model Summary

| | `canvas()` | `animated_canvas()` | `threaded_canvas()` |
|---|---|---|---|
| **Thread** | UI | UI | Dedicated render thread |
| **Win2D equivalent** | `CanvasControl` | (no direct equivalent) | `CanvasAnimatedControl` |
| **Frame trigger** | `invalidate()` | `CompositionTarget::Rendering` | Self-ticking loop |
| **Can access reactor state** | ✅ (in closure) | ✅ (in closure) | ❌ (must use channels/shared state) |
| **Blocks UI** | Only on draw | On every frame | Never |
| **Best for** | Static content | Lightweight UI animations | Games, simulations, heavy rendering |
| **Swap chain benefit** | Minimal | Flip presentation only | Full (off-thread present) |

---

## Crate Structure

```
windows-canvas/src/
├── lib.rs          — public exports
├── device.rs       — GpuDevice (unchanged)
├── swap_chain.rs   — SwapChain (unchanged)
├── session.rs      — DrawingSession (unchanged)
├── types.rs        — Color, StrokeStyle, enums (unchanged)
├── bitmap.rs       — Bitmap (unchanged)
├── reactor.rs      — animated_canvas, canvas, DrawContext (enhanced)
└── timing.rs       — FrameTiming (new)
```

The `windows-canvas` crate remains a thin safe wrapper over D2D/DXGI. The
reactor integration lives entirely in `reactor.rs` behind a `reactor` feature
gate. This mirrors Win2D's separation of `CanvasDevice` / `CanvasDrawingSession`
(low-level primitives) from `CanvasControl` / `CanvasAnimatedControl` (XAML
integration).

When `threaded_canvas` is implemented, it will likely live in a separate
module (`reactor_threaded.rs`) due to its fundamentally different state
management and threading requirements.
