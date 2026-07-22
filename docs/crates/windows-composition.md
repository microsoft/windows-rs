# windows-composition

> A safe wrapper around the Windows composition engine (`Windows.UI.Composition` and lifted `Microsoft.UI.Composition`) for building and animating retained-mode visual trees.

- 📦 Not published to crates.io
- 🚀 [Getting started](../../crates/libs/composition/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/composition)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition)

## Purpose

`windows-composition` wraps the `Windows.UI.Composition` API behind safe Rust
types — a `Compositor` and its visual tree (`Visual`, `SpriteVisual`,
`ContainerVisual`), brushes (`CompositionColorBrush`, and in future gradients and
surface/effect brushes), and the window-hosting types that put that tree on
screen. It is to `Windows.UI.Composition` what [`windows-canvas`](windows-canvas.md)
is to Direct2D/DXGI: a focused, hand-written safe layer over its own minimal, flat
bindings.

## Two composition stacks — one crate, forked at compile time

There are two distinct composition stacks, and they are **not interoperable** (they
are separate object graphs with distinct COM interface identities, so a visual from
one cannot be attached to a host from the other):

- `Windows.UI.Composition` — the **system** compositor, part of the OS. It can host
  a visual tree in **any window** via
  `ICompositorDesktopInterop::CreateDesktopWindowTarget`, with no Windows App SDK
  dependency.
- `Microsoft.UI.Composition` — the **lifted** WinUI 3 / Windows App SDK compositor.
  It has **no** standalone HWND target (there is no `ICompositorDesktopInterop` /
  `CreateDesktopWindowTarget` in its metadata *or* its NuGet interop headers); a
  lifted visual tree can only be hosted inside a WinUI element, via
  `Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.SetElementChildVisual`.

The two stacks are **member-identical** — same class names, members, and signatures
— so `windows-composition` presents **a single safe wrapper surface** and selects
the stack **at compile time** with a Cargo feature:

- **`system`** (default) targets `Windows.UI.Composition` and owns the standalone
  HWND-hosting story (`DispatcherQueueController`, `DesktopWindowTarget`, the
  `windows-window` dependency).
- **`lifted`** targets the lifted `Microsoft.UI.Composition` stack. It is a pure
  binding-set selector — it pulls in no extra dependencies. Lifted composition is
  *only* ever hosted through a WinUI element, so lifted consumers reach this crate
  **through** [`windows-reactor`](windows-reactor.md): reactor's optional
  `composition` feature turns this feature on and layers the typed host bridge on top
  (see [The reactor host bridge](#the-reactor-host-bridge-lifted-feature) below).

The two features are **mutually exclusive** (enabling neither or both is a
`compile_error!`). Both bindings sets are generated from **one** filter by
`tool_composition` (see [windows-clang](windows-clang.md)-style codegen notes
below), so the shared wrapper modules (`visual.rs`, `shape.rs`, `brush.rs`,
`animation.rs`, `compositor.rs`, `color.rs`) compile unchanged against either. This
keeps the crate small and fast to compile while eliminating the previous duplication
of the lifted wrappers inside `windows-reactor`.

### Feature unification and CI (maintenance note)

Because the `system` and `lifted` features are mutually exclusive, this crate
cannot be built for **both** stacks in a single Cargo invocation. Cargo unifies
features across a build graph, so a `--workspace` / `--all` build that contains
both a `system` consumer (`test_composition`, the composition samples) and a
`lifted` consumer (`windows-reactor`, which **requires** this crate's `lifted`
stack, and everything that depends on reactor) enables both features at once and
trips the `compile_error!`. Two consequences for maintainers:

- **Unified CI jobs are lifted-primary and exclude the system-stack consumers.**
  Because reactor — and its whole test/sample subtree — force the `lifted` stack,
  the unified `clippy.yml` (`cargo clippy --workspace`) and `test.yml`
  (`cargo test --all`) passes exclude the smaller `system` side instead:
  `windows-composition` itself (a workspace root that would otherwise default to
  `system`), `test_composition`, and the `composition_standalone` /
  `composition_minesweeper` / `composition_canvas` samples. Each CI file then runs a
  **second** step that lints/tests exactly those `system`-side crates together. Any
  *new* crate that selects the `system` stack must be added to both the exclusion
  list and the second step. Per-crate tooling is unaffected — `tool_clippy_all` runs
  `cargo clippy -p <name>` one crate at a time, and `-p <crate>` builds only pull the
  features that crate selects.
- **`tool_yml` special-cases this crate.** The generated `msrv.yml` and
  `no-default-features.yml` matrices otherwise run `cargo check -p <name>
  --all-features` and `--no-default-features`, both invalid here (both stacks /
  neither stack). `crates/tools/yml/src/{msrv,no_default_features}.rs` special-case
  `windows-composition` to check each stack explicitly instead; re-run
  `cargo run -p tool_yml` after touching those generators.

## Architecture (mirrors windows-canvas / windows-animation)

- **Dependencies:** [`windows-core`](windows-core.md),
  [`windows-numerics`](windows-numerics.md),
  [`windows-collections`](windows-collections.md) (for the `IVector`-backed shape
  collection and the `IMap`-backed implicit-animation collection — these Foundation
  collection interfaces are *referenced* from `windows-collections`, never
  re-generated locally, exactly as `Vector2`/`Vector3` come from `windows-numerics`),
  and — *optional, feature-gated* — [`windows-window`](windows-window.md)
  (the safe `HWND`-hosting target, `system` feature). Never the `windows` crate, and —
  since the dependency flip — never `windows-reactor`: the lifted host bridge now lives
  in reactor, which depends on *this* crate (not the other way around).
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
| `bindings.rs` | Generated flat/minimal **system** `Windows.UI.Composition` + `ICompositorDesktopInterop` (`tool_composition`, `system` feature). |
| `bindings_lifted.rs` | Generated flat/minimal **lifted** `Microsoft.UI.Composition` (`tool_composition`, `lifted` feature). Same member surface as `bindings.rs`. |
| `stack.rs` *(system)* | `DispatcherQueueController` — the per-thread dispatcher-queue bootstrap the compositor requires. |
| `compositor.rs` | `Compositor` — the factory: create visuals, brushes, shapes, geometries, animations, scoped batches, and (system) window targets. `from_host` (lifted) adopts a WinUI element's lifted compositor. |
| `target.rs` *(system)* | `DesktopWindowTarget` — hosts a root visual inside a window. |
| `visual.rs` | `Visual` (base: offset/size/opacity/scale/anchor/border/relative sizing/`start_animation`), `ContainerVisual` (children), `SpriteVisual` (brush), `VisualCollection`, `BorderMode`. `Visual::as_raw` (lifted) surfaces the interop `IInspectable`. |
| `shape.rs` | `ShapeVisual`, `CompositionShape`/`Shape`, sprite & container shapes, ellipse geometry, shape collection. |
| `brush.rs` | `CompositionBrush`/`Brush`, `CompositionColorBrush`, `CompositionNineGridBrush` (surface/effect brushes to come). |
| `animation.rs` | `CompositionAnimation`/`Animation`, `Vector3KeyFrameAnimation`, `ScalarKeyFrameAnimation`, `CompositionEasingFunction` (linear / cubic-bezier), and `ImplicitAnimationCollection` — the key-frame, easing, and implicit-transition surface reactor's animation engine drives. Key-frame collections cast to [`windows-collections`](windows-collections.md) `IMap` rather than re-binding it. |
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
    let root = compositor.create_container_visual();
    target.set_root(&root);

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
`tool_bindings`) because it needs non-default winmd inputs and emits **two**
bindings modules from **one** filter. `tool_composition` runs `windows_bindgen`
twice with `--flat --minimal --dead-code`:

- **System** (`bindings.rs`) — `--in` = `crates/libs/bindgen/default/Windows.winmd`
  (the `Windows.UI.Composition` and `Windows.System` types) **and**
  `crates/libs/bindgen/default/Windows.Win32.winmd` (for `ICompositorDesktopInterop`
  and the `HWND`/`BOOL` hosting types), filtered by
  `crates/tools/composition/src/composition.txt`.
- **Lifted** (`bindings_lifted.rs`) — `--in` =
  `crates/tools/reactor/winmd/Microsoft.UI.winmd` (for `Microsoft.UI.Composition`)
  **and** `Windows.winmd` (for the shared `Windows.Foundation` / `Windows.UI.Color`
  types), filtered by a filter **derived from the same `composition.txt`**.

`composition.txt` is the single source of truth. `// region: system-only` /
`// endregion` markers delimit the entries that exist only on the system stack
(`Compositor::CreateInstance`, `Windows.System.DispatcherQueueController`, and the
`DesktopWindowTarget` / `ICompositorDesktopInterop` block). To produce the lifted
filter the tool drops those regions and rewrites `Windows.UI.Composition.` →
`Microsoft.UI.Composition.` (the shared `Windows.Foundation` / `Windows.UI.Color`
entries are untouched), writing a scratch filter under `target/`. Prune or extend
either surface by editing the one `composition.txt`, then re-run
`cargo run -p tool_composition`.

`lib.rs` selects the module at compile time: `#[cfg(feature = "system")]` →
`bindings.rs`, `#[cfg(feature = "lifted")]` → `bindings_lifted.rs`, with
`compile_error!` guards for neither/both.

One interop function is **not** in the repo's Win32 metadata: the CoreMessaging
`CreateDispatcherQueueController` (from `dispatcherqueue.h`), which is the only way
to stand up a dispatcher queue on the *current* thread. `stack.rs` declares it
directly with `windows_core::link!` — a small, stable, self-contained shim.

## The reactor host bridge (`lifted` feature)

Lifted composition is hosted inside a WinUI element, and that element tree belongs
to [`windows-reactor`](windows-reactor.md). Since the dependency flip, **reactor
depends on this crate** — as a *required* dependency pinned to this crate's `lifted`
feature — so the typed host bridge lives in reactor rather than here. Reactor's
[`CompositionHost`](windows-reactor.md) widget exposes inherent typed methods on its
`CompositionHostHandle`:

```rust,ignore
let compositor = host.compositor()?;         // Compositor::from_host(element's compositor)
let root = compositor.create_container_visual();
host.set_child_visual(&root)?;               // set the element's child visual
```

This crate's contribution to the bridge is the `lifted` binding set plus the two
seam helpers reactor consumes: `Compositor::from_host(&IInspectable)` (adopts the
element's compositor) and `Visual::{from_host, as_raw}` (adopts / surfaces a visual's
interop `IInspectable`). Because both crates' lifted bindings derive from the same
`Microsoft.UI.winmd`, the `IInspectable` handed across the seam has matching IIDs,
so the `.cast()` inside `from_host` / `as_raw` is zero-overhead and ABI-safe. The
dependency is one-way (`windows-reactor` → `windows-composition[lifted]`), mirroring
how reactor's optional `canvas` feature depends on `windows-canvas`. Reactor's
transition/animation engine also drives this crate's key-frame, easing, and
implicit-animation wrappers directly — see
[windows-reactor.md](windows-reactor.md)'s animation section.

## The canvas bridge (`system` feature)

Direct2D content can be drawn into a composition surface and painted onto a visual,
mirroring Win2D's `CanvasComposition`. This crate owns the composition half:

- `Compositor::create_graphics_device(&impl Interface)` wraps
  `ICompositorInterop::CreateGraphicsDevice`, adopting the app's Direct2D (or DXGI)
  rendering device into a `CompositionGraphicsDevice`.
- `CompositionGraphicsDevice::create_drawing_surface(width, height)` allocates a
  premultiplied-BGRA `CompositionDrawingSurface`.
- `Compositor::create_surface_brush(&surface)` produces a `CompositionSurfaceBrush`
  (a [`Brush`]) to paint any visual with the surface.
- `CompositionDrawingSurface::{begin_draw::<T>, end_draw, resize}` is the interop
  seam over `ICompositionDrawingSurfaceInterop`. `begin_draw` returns the drawing
  target `T` (an `ID2D1DeviceContext`) plus the backing-atlas `(x, y)` pixel offset —
  exactly the shape [`windows-canvas`](windows-canvas.md)'s
  `DrawingSession::from_borrowed_context(context, offset)` consumes. It mirrors
  reactor's `SurfaceImageSource::begin_draw::<T>`, so the same generic-`T` pattern
  keeps this crate free of any Direct2D dependency.

The Direct2D drawing itself lives in `windows-canvas` behind its one-way
`composition` feature (`windows-canvas` → `windows-composition`): import
`windows_canvas::CanvasCompositionExt` for `device.create_graphics_device(&compositor)`
and `surface.draw(|session| …) -> Result<bool>` (device-lost → `Ok(false)`, no implicit
clear — the Win2D policy). See the
[`composition/canvas`](../../crates/samples/composition/canvas) sample.

The whole bridge is **system-only**: lifted `Microsoft.UI.Composition` has no
Direct2D-surface interop metadata (`ICompositorInterop` /
`ICompositionDrawingSurfaceInterop` exist only in the system winmd), so these entries
sit in the filter's `// region: system-only` block and the wrappers are gated
`#[cfg(feature = "system")]`.

## Future work

- **Effect & gradient brushes** — `CompositionSurfaceBrush` landed for the canvas
  bridge above; gradient and effect brushes remain.
- **More animations** — the key-frame surface now covers `Vector3` and scalar
  values, linear / cubic-bezier easing, implicit-animation collections, and
  expression key frames (added so reactor's animation engine could drop its private
  binding slice and drive these wrappers directly). Remaining: `Vector2` / color
  key-frame value types and standalone expression animations.
- **Scoped-batch completion** — `CompositionScopedBatch` currently exposes only
  `end()`; surfacing its `Completed` event (and a way to await it) would let callers
  sequence work when a batch of animations finishes.
- **Revisit the explicit `system` feature.** `system` is the default, so naming a
  feature for "the normal build" can look redundant. It stays because Cargo optional
  dependencies can only be turned *on* by a positive feature, never *off*:
  `windows-window` (needed only for the system stack's HWND hosting) is
  `optional = true` and enabled by `system = ["dep:windows-window"]`, so
  `lifted`-stack consumers don't pull it into their tree. Dropping `system` in favour
  of gating code with `#[cfg(not(feature = "lifted"))]` would work for the *code* but
  not the *dependency* — `windows-window` would have to become unconditional (pulled
  even by lifted consumers) — and it would trade the symmetric
  `cfg(feature = "system")` / `cfg(feature = "lifted")` gates (and the
  `compile_error!` on neither/both) for double negatives. Worth simplifying only if the
  optional `windows-window` dependency is removed or made unconditional.

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
- [x] Dual-stack compile-time fork: single wrapper surface over `system`
      (`Windows.UI.Composition`) and `lifted` (`Microsoft.UI.Composition`)
      bindings, generated from one filter by `tool_composition`.
- [x] Reactor host bridge (`lifted` feature): reactor's inherent typed
      `compositor()` / `set_child_visual()` over its raw `CompositionHostHandle`
      seam; `reactor/composition` samples.
- [ ] Surface & effect brushes.
- [ ] Canvas ↔ composition bridge (in `windows-canvas`, `composition` feature).
- [ ] Expression animations and more key-frame value types.
- [ ] `dcomp` port.
