# windows-composition

> **Status: system composition landed — standalone HWND hosting works.** The
> crate wraps the **system** `Windows.UI.Composition` stack behind safe Rust
> types: `Compositor`, `Visual`/`ContainerVisual`/`SpriteVisual`/`ShapeVisual`,
> shapes and geometries, solid-color and nine-grid brushes, `Vector3` key-frame
> animations, scoped batches, `Color`, plus the hosting types
> `DispatcherQueueController` and `DesktopWindowTarget`. A visual tree can be
> hosted directly in a plain Win32 window — no WinUI 3 / Windows App SDK
> dependency — as shown by the
> [`composition/standalone`](../../crates/samples/composition/standalone) and
> [`composition/minesweeper`](../../crates/samples/composition/minesweeper)
> samples. Surface/effect brushes and the canvas bridge are still to come — see
> the checklist below.

- 📦 Not published
- 🧩 Sibling of [`windows-canvas`](windows-canvas.md) and
  [`windows-reactor`](windows-reactor.md)

## Purpose

`windows-composition` wraps the `Windows.UI.Composition` API behind safe Rust
types — a `Compositor` and its visual tree (`Visual`, `SpriteVisual`,
`ContainerVisual`), brushes (`CompositionColorBrush`, and in future gradients and
surface/effect brushes), and the window-hosting types that put that tree on
screen. It is to `Windows.UI.Composition` what [`windows-canvas`](windows-canvas.md)
is to Direct2D/DXGI: a focused, hand-written safe layer over its own minimal, flat
bindings.

## Two composition stacks — and which crate owns which

There are two distinct composition stacks, and they are **not interoperable** (they
are separate object graphs with distinct COM interface identities, so a visual from
one cannot be attached to a host from the other):

- `Windows.UI.Composition` — the **system** compositor, part of the OS. It can host
  a visual tree in **any window** via
  `ICompositorDesktopInterop::CreateDesktopWindowTarget`, with no Windows App SDK
  dependency. **This is what `windows-composition` wraps.**
- `Microsoft.UI.Composition` — the **lifted** WinUI 3 / Windows App SDK compositor.
  It has **no** standalone HWND target (there is no `ICompositorDesktopInterop` /
  `CreateDesktopWindowTarget` in its metadata *or* its NuGet interop headers); a
  lifted visual tree can only be hosted inside a WinUI element, via
  `Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.SetElementChildVisual`.

Because the lifted stack is only usable *inside* a WinUI element, its safe wrappers
live in [`windows-reactor`](windows-reactor.md) (which owns the WinUI element tree),
next to the `CompositionHost` widget that hosts them. `windows-composition` owns the
**system** stack and the **standalone** hosting story. The two wrapper sets are
nearly identical because the two stacks are **member-identical** — same class names,
members, and signatures — so the same idioms (see below) apply to both.

## Architecture (mirrors windows-canvas / windows-animation)

- **Dependencies:** [`windows-core`](windows-core.md),
  [`windows-numerics`](windows-numerics.md),
  [`windows-collections`](windows-collections.md) (for the `IVector`-backed shape
  collection), and [`windows-window`](windows-window.md) (for the safe
  `HWND`-hosting target) — *not* the `windows` crate.
- **Zero-overhead newtypes.** Every safe type is a newtype over exactly one owned
  COM interface — no boxing, no per-call allocation — exactly as
  [`windows-animation`](windows-animation.md) and
  [`windows-canvas`](windows-canvas.md) do. Methods wrap a single
  `self.0.Method(..)` and return `windows_core::Result`; no `unsafe` at the safe
  call site. Class-to-default-interface relationships are modeled by storing the
  most-derived interface and `cast()`-ing internally. Every wrapper is `Clone`
  (a cheap COM `AddRef`), so handles can be stored in collections and shared.
- **Numerics interop:** offsets and sizes use `windows-numerics`
  (`Vector2`/`Vector3`), matching canvas. Brush color is a flat-bound
  `Windows.UI.Color` wrapped in a small `Color` newtype so callers never touch the
  raw ABI struct.

### Class hierarchies via `Deref`; type families via sealed traits

Two idioms keep the public API free of the private `bindings::` types while still
modeling composition's class hierarchies:

- **`Deref` for inheritance.** A concrete visual derefs to its base, so base
  operations are available directly: `SpriteVisual` and `ShapeVisual` deref to
  `ContainerVisual`, which derefs to `Visual`. Passing a `&SpriteVisual` where a
  `&Visual` is expected works through deref coercion.
- **Sealed traits for "any brush / shape / animation".** `Brush`, `Shape`, and
  `Animation` are sealed marker traits (only this crate implements them). Each
  exposes an `as_brush()` / `as_shape()` / `as_animation()` that returns the shared
  base wrapper (`CompositionBrush`, `CompositionShape`, `CompositionAnimation`), so
  a method like `SpriteVisual::set_brush(&impl Brush)` accepts a color brush or a
  nine-grid brush without leaking a `bindings::` type into its signature.

### Module layout

| Module | Contents |
| --- | --- |
| `bindings.rs` | Generated flat/minimal `Windows.UI.Composition` + `ICompositorDesktopInterop` (`tool_composition`). |
| `stack.rs` | `DispatcherQueueController` — the per-thread dispatcher-queue bootstrap the compositor requires. |
| `compositor.rs` | `Compositor` — the factory: create visuals, brushes, shapes, geometries, animations, scoped batches, and window targets. |
| `target.rs` | `DesktopWindowTarget` — hosts a root visual inside a window. |
| `visual.rs` | `Visual` (base: offset/size/opacity/scale/anchor/border/relative sizing/`start_animation`), `ContainerVisual` (children), `SpriteVisual` (brush), `VisualCollection`, `BorderMode`. |
| `shape.rs` | `ShapeVisual`, `CompositionShape`/`Shape`, sprite & container shapes, ellipse geometry, shape collection. |
| `brush.rs` | `CompositionBrush`/`Brush`, `CompositionColorBrush`, `CompositionNineGridBrush` (surface/effect brushes to come). |
| `animation.rs` | `CompositionAnimation`/`Animation`, `Vector3KeyFrameAnimation`. |
| `batch.rs` | `CompositionScopedBatch`, `BatchKind`. |
| `color.rs` | `Color` newtype over `Windows.UI.Color`. |

## Hosting a visual tree in a window

System composition requires a `DispatcherQueue` on the thread that creates the
compositor, then a `DesktopWindowTarget` bound to the window. The safe target
takes a [`windows-window`](windows-window.md) `&Window` directly; an
`unsafe` `create_desktop_window_target_for_hwnd` escape hatch remains for callers
that own an `HWND` from another source:

```rust,no_run
use windows_composition::*;
use windows_window::Window;

fn main() -> Result<()> {
    let window = Window::new("Composition").size(800, 600).create()?;

    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

    let target = compositor.create_desktop_window_target(&window, false)?;
    let root = compositor.create_container_visual()?;
    target.set_root(&root)?;

    // ... add sprites to `root.children()` ...

    windows_window::run();
    Ok(())
}
```

See [`composition/standalone`](../../crates/samples/composition/standalone) for a
minimal app and
[`composition/minesweeper`](../../crates/samples/composition/minesweeper) for a
complete one.

## Codegen: a dedicated `tool_composition`

Composition gets its own generator tool (rather than a plain entry in the shared
`tool_bindings`) because it needs a non-default winmd input. `tool_composition`
runs `windows_bindgen` with:

- `--in` = `crates/libs/bindgen/default/Windows.winmd` (the `Windows.UI.Composition`
  and `Windows.System` types) **and**
  `crates/libs/bindgen/default/Windows.Win32.winmd` (for `ICompositorDesktopInterop`
  and the `HWND`/`BOOL` types used to host a tree), and
- `--flat --minimal --dead-code --filter --etc crates/tools/composition/src/composition.txt`.

It emits `crates/libs/composition/src/bindings.rs`. `composition.txt` is the filter
(the composition analogue of `canvas.txt`) — prune or extend the surface by editing
one file, then re-run `cargo run -p tool_composition`.

One interop function is **not** in the repo's Win32 metadata: the CoreMessaging
`CreateDispatcherQueueController` (from `dispatcherqueue.h`), which is the only way
to stand up a dispatcher queue on the *current* thread. `stack.rs` declares it
directly with `windows_core::link!` — a small, stable, self-contained shim.

## Future work

- **Surface & effect brushes** — `CompositionSurfaceBrush`, gradients.
- **The canvas ↔ composition bridge.** Direct2D content drawn into a composition
  surface (Win2D's `CanvasComposition`). The system interop for this
  (`ICompositorInterop` / `ICompositionGraphicsDeviceInterop` /
  `ICompositionDrawingSurfaceInterop`, which return an `ID2D1DeviceContext` + offset
  `POINT`) maps directly onto canvas's existing
  `DrawingSession::from_borrowed_context(context, offset)`. To avoid a dependency
  cycle the bridge would live in `windows-canvas` behind a `composition` feature
  (`windows-canvas` → `windows-composition`, one-way).
- **More animations** — expression animations and additional key-frame value types
  (scalar/`Vector2`/color), building on the `Vector3` key-frame animation and scoped
  batch that already landed.

## Samples

`crates/samples/composition/` collects standalone composition samples (as
`crates/samples/canvas/` does). Being standalone (no WinUI 3 / reactor), they target
the **system** `Windows.UI.Composition` stack via
`ICompositorDesktopInterop::CreateDesktopWindowTarget`.

- **[`standalone`](../../crates/samples/composition/standalone)** *(landed)* — a
  `Compositor`, a background plus a row of colored `SpriteVisual`s in a
  `DesktopWindowTarget` on a plain `windows-window` HWND. The "hello triangle" of
  composition.
- **[`minesweeper`](../../crates/samples/composition/minesweeper)** *(landed)* — a
  port of robmikh's [minesweeper-rs](https://github.com/robmikh/minesweeper-rs): a
  16×16 `SpriteVisual` tile grid, a hollow nine-grid selection brush, dot-pattern
  neighbor counts drawn with `ShapeVisual`/`CompositionSpriteShape`, board scaling
  via relative sizing + `Scale`, pointer hit-testing by visual layout, and a spiral
  `Vector3` key-frame reveal animation grouped in a scoped batch. Input comes from
  `windows-window`'s message hook; mines are placed with a tiny built-in PRNG so the
  sample keeps its dependency list to just `windows-composition` and
  `windows-window`. Validates the crate's design on a real app.
- **Port of `crates/samples/windows/dcomp`** *(planned)* — that sample today uses
  Win32 DirectComposition + UIAnimation + Direct2D + WIC. Reimplementing it on the
  WinRT composition stack exercises visuals, surface brushes, and animations
  end-to-end. (Keep the original alongside.)

## Testing

`test_composition` (`crates/tests/libs/composition`) has two layers:

- **Pure-value tests** for `Color` (opaque `rgb`, component round-trip).
- **Headless live tests** (`live.rs`) that create a `DispatcherQueueController` on
  the test thread, build a real `Compositor`, and drive the visual, brush, shape,
  and animation wrappers — composition objects are constructed synchronously, so no
  window or message pump is needed and the suite runs in CI. Getters read back
  through the same COM objects the setters wrote, which also verifies each wrapper
  method routes to the correct versioned interface (e.g. `IVisual2`,
  `ICompositionObject`, `ICompositor5`). Hosting a tree in a window is left to the
  runnable `standalone` sample.

## Checklist

- [x] Scaffold the crate, `tool_composition`, and `test_composition`.
- [x] Safe wrappers: `Compositor`, `Visual`, `ContainerVisual`, `SpriteVisual`,
      `ShapeVisual`, `VisualCollection`, `Color`.
- [x] Brushes: `CompositionColorBrush`, `CompositionNineGridBrush` (sealed `Brush`).
- [x] Shapes: sprite & container shapes, ellipse geometry, shape collection
      (sealed `Shape`).
- [x] Animations: `Vector3KeyFrameAnimation` + scoped batches (sealed `Animation`).
- [x] Standalone HWND hosting (`DispatcherQueueController`, `DesktopWindowTarget`),
      with a safe `&Window` target over [`windows-window`](windows-window.md).
- [x] `composition/standalone` and `composition/minesweeper` samples.
- [x] Headless live `test_composition` coverage (visuals, brushes, shapes, animations).
- [ ] Surface & effect brushes.
- [ ] Canvas ↔ composition bridge (in `windows-canvas`, `composition` feature).
- [ ] Expression animations and more key-frame value types.
- [ ] `dcomp` port.
