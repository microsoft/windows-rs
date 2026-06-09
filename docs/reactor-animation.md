# Animation in `windows-reactor` and `windows-canvas`

## Overview

Animation in the Windows Rust ecosystem spans two layers:

1. **UI-level animation** (`windows-reactor`) — declarative property transitions on
   elements: opacity fades, scale pops, layout motion, enter/exit transitions. These
   run on the WinUI Composition layer and never require a render loop.

2. **Frame-level animation** (`windows-canvas`) — per-frame rendering via
   `animated_canvas()`. The closure runs every frame (~60 fps) and can animate
   anything by varying draw calls over time.

These are complementary, not competing. A typical app uses reactor animations for UI
chrome (list reorder, button hover, page transitions) and canvas animation for
custom 2D content (games, data visualizations, physics simulations).

---

## Reactor UI Animations

### Implicit Transitions

Implicit transitions animate property changes automatically. When a property like
opacity or scale changes between renders, the compositor interpolates smoothly
instead of snapping.

```rust
use windows_reactor::*;
use std::time::Duration;

text("Hello")
    .opacity(if visible { 1.0 } else { 0.0 })
    .with_opacity_transition(Duration::from_millis(200))
    .with_scale_transition(Duration::from_millis(300))
    .with_translation_transition(Duration::from_millis(300))
```

**Implemented DSL methods:**
- **`with_opacity_transition(duration)`** — animates opacity changes
- **`with_scale_transition(duration)`** — animates scale changes
- **`with_translation_transition(duration)`** — animates translation changes

**Data model supports but no DSL method yet:**
- **Rotation** — `ImplicitTransitions.rotation` exists in the struct but no
  `with_rotation_transition()` is exposed on `ElementExt`

### Layout Animation ✅ Implemented

Animates position/size changes driven by layout recalculation. When an element moves
or resizes due to sibling changes, the motion is smooth rather than instant.

```rust
use windows_reactor::*;

text("Item")
    .with_layout_animation(LayoutAnimationConfig::spring())

// Or with fine-grained control:
text("Item")
    .with_layout_animation(
        LayoutAnimationConfig {
            duration: Duration::from_millis(400),
            use_spring: true,
            damping_ratio: 0.6,
            period: 0.08,
            animate_offset: true,
            animate_size: false,
        }
    )
```

Configuration:
- **`duration`** — total animation time (linear mode)
- **`use_spring`** — spring physics instead of linear interpolation
- **`damping_ratio`** / **`period`** — spring tuning
- **`animate_offset`** — animate position changes (default: true)
- **`animate_size`** — animate size changes (default: false)

Wired end-to-end: DSL → reconciler (`set_layout_animation`) → WinUI backend.

### Property Animation ✅ Implemented

One-shot animations that drive opacity/scale to a target value with easing.

```rust
use windows_reactor::*;

panel.animate(AnimationConfig::fade_in(Duration::from_millis(300)))
panel.animate(AnimationConfig::fade_out(Duration::from_millis(200)))

// Custom:
panel.animate(AnimationConfig {
    opacity: Some(0.5),
    scale: Some(1.2),
    duration: Duration::from_millis(400),
    easing: Easing::EaseInOut,
})
```

Easing options: `Linear`, `EaseOut`, `EaseIn`, `EaseInOut`.

Wired end-to-end: DSL (`.animate()`) → reconciler (`run_property_animation`) → WinUI backend.

### Enter/Exit Transitions ⚠️ Partially Implemented

Animate elements as they mount to or unmount from the tree.

```rust
use windows_reactor::*;

text("Notification")
    .transition(
        Some(AnimationConfig::fade_in(Duration::from_millis(200))),  // enter
        Some(AnimationConfig::fade_out(Duration::from_millis(150))), // exit
    )
```

**Current status:**
- **Enter transition** — ✅ works. On mount, the reconciler calls
  `run_property_animation` with the enter config.
- **Exit transition** — ❌ stored in `AnimationModifiers` but **not consumed** by
  the reconciler on unmount. The element is removed immediately without animation.
  Implementing exit requires deferred removal (keep the element alive until the
  animation completes, then destroy it).

### AnimationModifiers (Internal Structure)

All animation settings are stored in a single `AnimationModifiers` block on each
element. The reconciler applies them independently:

```rust
pub struct AnimationModifiers {
    pub implicit_transitions: Option<ImplicitTransitions>,  // ✅ wired
    pub layout_animation: Option<LayoutAnimationConfig>,    // ✅ wired
    pub property_animation: Option<AnimationConfig>,        // ✅ wired
    pub enter_transition: Option<AnimationConfig>,          // ✅ wired (on mount)
    pub exit_transition: Option<AnimationConfig>,           // ❌ stored but not consumed
}
```

### Backend Implementation

The WinUI backend translates these into Composition layer APIs:
- Implicit transitions → `ImplicitAnimationCollection` on the element's `Visual`
- Layout animation → spring or linear `Vector3KeyFrameAnimation` on `Offset`/`Size`
- Property animation → `ScalarKeyFrameAnimation` on `Opacity`/`Scale`
- Enter transition → same as property animation, triggered on mount

All implemented animations run on the compositor thread — they don't block the UI
thread or the render loop.

---

## Canvas Frame Animation (`animated_canvas`)

### What It Is

`animated_canvas()` is a reactor widget that renders custom 2D content every frame.
It creates a `SwapChainPanel`, initializes a `GpuDevice` and swap chain, and
subscribes to `CompositionTarget::Rendering` to drive the render loop.

```rust
use windows_canvas::*;
use windows_reactor::*;

animated_canvas(|ctx| {
    ctx.clear(Color::CORNFLOWER_BLUE);
    let brush = ctx.create_solid_brush(Color::WHITE).unwrap();
    ctx.fill_ellipse(
        &Ellipse::circle(Vector2::new(ctx.width / 2.0, ctx.height / 2.0), 50.0),
        &brush,
    );
})
```

### How the Loop Works

1. **`on_mounted`** — fires when the `SwapChainPanel` is attached to the visual tree.
   Creates the `GpuDevice` and `SwapChain`.
2. **`on_rendering`** — subscribes to `CompositionTarget::Rendering`, which fires
   once per compositor frame (~60 fps or display refresh rate).
3. **Each frame**: `begin_draw()` → user closure → `present()`.
4. **Device lost**: if `present()` returns `Ok(false)`, the device and swap chain
   are silently rebuilt via `RenderState::rebuild()`.
5. **Resize**: `on_resize` updates dimensions and calls `chain.resize()`.

### DrawContext

The closure receives a `DrawContext` that derefs to `DrawingSession`:

```rust
pub struct DrawContext<'a> {
    pub width: f32,   // render target width in DIPs
    pub height: f32,  // render target height in DIPs
    // + Deref<Target = DrawingSession> for all draw calls
    // + device() for creating paths, brushes, etc.
}
```

### Animation Patterns

Since the draw closure runs every frame, animation is simply a function of time
or state:

```rust
use std::time::Instant;

let start = Instant::now();

animated_canvas(move |ctx| {
    let t = start.elapsed().as_secs_f32();
    let x = (t * 2.0).sin() * 100.0 + ctx.width / 2.0;
    let y = (t * 3.0).cos() * 100.0 + ctx.height / 2.0;

    ctx.clear(Color::BLACK);
    let brush = ctx.create_solid_brush(Color::RED).unwrap();
    ctx.fill_ellipse(&Ellipse::circle(Vector2::new(x, y), 20.0), &brush);
})
```

There is no separate update/draw split or fixed-timestep mechanism — the closure
is both update and draw. For physics simulations, you track your own delta time.

---

## Comparison with Win2D's Animation Model

Win2D (C#) provides two distinct controls:

| | **CanvasControl** | **CanvasAnimatedControl** |
|---|---|---|
| Loop | None (event-driven `Invalidate()`) | Game loop with `Update` + `Draw` |
| Timing | N/A | Fixed or variable timestep |
| Threading | UI thread | Dedicated game loop thread |
| Swap chain | No (uses `CanvasImageSource`) | Yes (swap chain panel) |
| Use case | Static/rarely-changing content | Real-time animation/games |

### How `windows-canvas` Maps to Win2D

| Win2D | windows-canvas |
|---|---|
| `CanvasAnimatedControl` | `animated_canvas()` |
| `CanvasControl` | Deferred (roadmap item #9: `canvas()`) |
| `CreateResources` event | Resources created in `on_mounted`; device-lost rebuilds automatically |
| `Update` event | Part of the draw closure (no separation) |
| `Draw` event | The draw closure itself |
| `CanvasAnimatedDrawEventArgs.Timing` | Not provided (use `Instant::now()` / `elapsed()`) |
| `IsFixedTimeStep` / `TargetElapsedTime` | Not supported (always variable, vsync-locked) |
| `Paused` | Drop the widget or guard with a condition |
| Device-lost recovery | Automatic via `RenderState::rebuild()` |

### Key Differences

1. **No update/draw separation** — Win2D separates game logic (`Update`) from
   rendering (`Draw`) and can run multiple updates per frame when running slow.
   `animated_canvas` merges them into a single closure. This is simpler but means
   physics simulations must handle variable dt themselves.

2. **No fixed timestep** — Win2D's `IsFixedTimeStep` ensures deterministic
   simulation. Canvas always runs at the compositor's refresh rate.

3. **No timing args** — Win2D passes `ElapsedTime`, `TotalTime`, and
   `IsRunningSlowly` to every frame. Canvas provides only width/height; apps
   must track time externally.

4. **No dedicated thread** — Win2D runs the game loop on a background thread.
   `animated_canvas` runs on the UI thread via `CompositionTarget::Rendering`.

5. **No `CanvasControl` equivalent yet** — the deferred `canvas()` widget (roadmap
   item #9) would provide invalidate-on-demand rendering without a continuous loop.

---

## The Direct2D Sample and UIAnimation

The `crates/samples/windows/direct2d/src/main.rs` sample demonstrates a third
animation approach: the **Windows Animation Manager** (`IUIAnimationManager`).

### How It Works

1. Creates an `IUIAnimationManager` and an `IUIAnimationVariable` (initial value 0.0)
2. Schedules an accelerate/decelerate transition (0→1 over time)
3. Each frame: calls `manager.Update(time)` with performance-counter time
4. Reads `variable.GetValue()` to get the current interpolated value (0.0→1.0)
5. Uses that value to scale clock hand angles for a smooth startup animation

```rust
// Simplified from the sample:
let manager: IUIAnimationManager = CoCreateInstance(&UIAnimationManager, ...)?;
let variable = manager.CreateAnimationVariable(0.0)?;
manager.ScheduleTransition(&variable, &transition, now)?;

// In the render loop:
manager.Update(get_time(frequency)?)?;
let swing = variable.GetValue()?; // 0.0 → 1.0 over time
```

### Relationship to Canvas

The UIAnimation Manager is a **Win32 COM API** — it's lower-level than WinUI
Composition animations and independent of both reactor and canvas. It provides:

- Arbitrary variable interpolation (not just visual properties)
- Multiple transition types (linear, parabolic, cubic, accelerate/decelerate, etc.)
- Storyboard sequencing (parallel/sequential transitions on multiple variables)
- Priority comparison (conflict resolution between competing animations)

### Should Canvas Support UIAnimation Natively?

**Current recommendation: No.** Here's why:

1. **Scope mismatch** — UIAnimation is a general interpolation engine. Canvas is a
   rendering surface. Bundling interpolation into the canvas crate conflates concerns.

2. **Easy to use externally** — apps can create animation variables via the `windows`
   crate and read values in the draw closure. No special integration needed.

3. **Reactor fills the gap** — for property animations on UI elements, reactor's
   `AnimationConfig` and `ImplicitTransitions` cover common cases via Composition.

4. **Simplicity wins** — `animated_canvas` deliberately keeps the API minimal:
   "here's a closure that runs every frame." Time-based animation is just math.

However, **a lightweight timing helper** could be valuable as a canvas utility:

```rust
// Hypothetical future addition:
pub struct Tween {
    start: Instant,
    duration: Duration,
    easing: EasingFn,
}

impl Tween {
    pub fn value(&self) -> f32 { /* 0.0..1.0 */ }
    pub fn is_done(&self) -> bool { ... }
}
```

This would avoid every app reinventing easing math without pulling in a full
animation manager.

---

## How Reactor and Canvas Animation Complement Each Other

```
┌──────────────────────────────────────────────────────┐
│                  Your Application                    │
├───────────────────────┬──────────────────────────────┤
│   UI Layer            │   Custom 2D Layer            │
│   (windows-reactor)   │   (windows-canvas)           │
├───────────────────────┼──────────────────────────────┤
│ • Implicit transitions│ • Per-frame rendering        │
│ • Layout animation    │ • Time-based math            │
│ • Enter/exit fades    │ • Physics simulations        │
│ • Property animation  │ • Data visualization         │
│                       │ • Game rendering             │
├───────────────────────┼──────────────────────────────┤
│ Composition Layer     │ Direct2D / DXGI              │
│ (Visual tree)         │ (Swap chain)                 │
└───────────────────────┴──────────────────────────────┘
```

**Use reactor animations when:**
- Animating standard UI properties (opacity, position, scale, rotation)
- Elements are entering/leaving the tree
- Layout is changing and you want smooth reflow
- You want fire-and-forget — no render loop management

**Use canvas animation when:**
- Drawing custom shapes, charts, or scenes
- Simulation-driven updates (physics, particles)
- You need full control over what's rendered each frame
- Performance-critical rendering (60fps Direct2D)

**They combine naturally:**

```rust
fn view(cx: &mut RenderCx) -> Element {
    column((
        // Reactor-animated header
        text("Score: 42")
            .with_opacity_transition(Duration::from_millis(200)),

        // Canvas-animated game area
        animated_canvas(move |ctx| {
            // Custom 2D rendering at 60fps
            draw_game_world(ctx);
        })
        .with_layout_animation(LayoutAnimationConfig::spring()),
    ))
}
```

---

## Outstanding / Not Yet Supported

### In Reactor

| Feature | Status | Notes |
|---------|--------|-------|
| Implicit transitions (opacity, scale, translation) | ✅ Implemented | DSL + reconciler + WinUI backend |
| Implicit rotation transition | ⚠️ Data model only | `ImplicitTransitions.rotation` exists; no DSL method |
| Layout animation (spring/linear) | ✅ Implemented | DSL + reconciler + WinUI backend |
| Property animation (one-shot) | ✅ Implemented | `.animate()` DSL + reconciler + WinUI backend |
| Enter transition | ✅ Implemented | Via `run_property_animation` on mount |
| Exit transition | ❌ Not wired | Stored in struct; reconciler doesn't animate before removal |
| Keyframe animations | ❌ Not yet | C# Reactor has `KeyframeAnimations` |
| Scroll-linked animation | ❌ Not yet | C# Reactor has `ScrollAnimationBuilder` |
| Stagger (cascading delays) | ❌ Not yet | C# Reactor has `StaggerConfig` |
| Interaction states (hover/press) | ❌ Not yet | C# Reactor has `InteractionStates` |
| Theme transitions (page navigation) | ❌ Not yet | WinUI has `NavigationTransitionInfo` |
| Connected animations | ❌ Not yet | WinUI has `ConnectedAnimationService` |

### In Canvas

| Feature | Status | Notes |
|---------|--------|-------|
| Frame loop via `animated_canvas()` | ✅ Implemented | Vsync-locked via `CompositionTarget::Rendering` |
| Device-lost recovery | ✅ Implemented | Automatic `RenderState::rebuild()` |
| DPI-aware rendering | ✅ Implemented | `SetMatrixTransform` + `SetDpi` for crisp output at all scales |
| Stroke styles (caps, joins, dashes) | ✅ Implemented | `StrokeStyleBuilder` + `draw_*_styled()` methods |
| `canvas()` (invalidate-on-demand) | ❌ Deferred | Roadmap item #9; needs `ISurfaceImageSource` |
| Timing/elapsed args in DrawContext | ❌ Not yet | Apps must use `Instant::now()` externally |
| Fixed timestep option | ❌ Not yet | Win2D has `IsFixedTimeStep` |
| Update/draw separation | ❌ Not yet | Single closure model |
| Background render thread | ❌ Not yet | Runs on UI thread via Rendering event |
| Drop shadow effect | ❌ Not yet | D2D has shadow effects; not exposed in canvas |
| Built-in easing/tween helpers | ❌ Not yet | Possible lightweight addition (see below) |

---

## Summary

Animation in the Rust Windows ecosystem is split by intent:

- **Reactor** handles UI motion declaratively — you set the destination, the
  compositor figures out the journey. Zero render loops, zero manual timing.

- **Canvas** handles custom rendering imperatively — you own every pixel every
  frame. Animation is just math applied to draw calls.

- **UIAnimation Manager** (Win32) provides industrial-strength interpolation for
  apps that need storyboards, priority queues, or complex multi-variable
  transitions. It works alongside both reactor and canvas but isn't bundled in.

The design intentionally avoids a monolithic animation framework. Each layer does
one thing well, and they compose without conflict.
