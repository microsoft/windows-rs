# windows-composition

> Safe Rust wrappers for the Windows composition engine.

- Not published to crates.io
- [Getting started](../../crates/libs/composition/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/composition)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition)

`windows-composition` wraps `Windows.UI.Composition` and `Microsoft.UI.Composition`. It gives you a
`Compositor`, visuals, brushes, shapes, animations, and hosting helpers. The wrappers use minimal
flat bindings and expose a small safe API.

## Composition stacks

The crate supports two composition stacks. They are separate COM object graphs. A visual from one
stack cannot attach to a host from the other.

- `Windows.UI.Composition` is the system compositor. It can host a visual tree in any window through
  `ICompositorDesktopInterop::CreateDesktopWindowTarget`. It does not need the Windows App SDK.
- `Microsoft.UI.Composition` is the lifted WinUI 3 compositor. It has no standalone HWND target. A
  lifted visual tree is hosted inside a WinUI element through
  `Microsoft.UI.Xaml.Hosting.ElementCompositionPreview`.

The two stacks have the same class names, members, and signatures. The crate uses one wrapper
surface and selects the stack at compile time.

- `system` is the default feature. It targets `Windows.UI.Composition` and enables
  `DispatcherQueueController`, `DesktopWindowTarget`, and the `windows-window` dependency.
- `lifted` targets `Microsoft.UI.Composition`. It selects the lifted bindings and adds no
  dependency. Reactor enables this feature and hosts lifted visuals through `CompositionHost`.

The features are mutually exclusive. Enabling neither feature or both features is a compile error.
Both binding sets are generated from one filter by `tool_composition`. The shared wrapper modules
compile against either stack.

## Hosting a visual tree in a window

System composition needs a `DispatcherQueue` on the compositor thread. It also needs a
`DesktopWindowTarget` bound to a window. The safe target takes a
[`windows-window`](windows-window.md) `&Window`. An unsafe `create_desktop_window_target_for_hwnd`
escape hatch exists for callers that own an `HWND` from another source.

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

See [`composition/standalone`](../../crates/samples/composition/standalone) for a small app. See
[`composition/minesweeper`](../../crates/samples/composition/minesweeper) for a complete app.

## Samples

`crates/samples/composition/` contains standalone system-composition samples. They use
`ICompositorDesktopInterop::CreateDesktopWindowTarget` and do not use WinUI 3 or reactor.

- [`standalone`](../../crates/samples/composition/standalone) creates a `Compositor`, a background,
  and colored `SpriteVisual` items in a `DesktopWindowTarget`.
- [`minesweeper`](../../crates/samples/composition/minesweeper) is a complete app. It uses a
  16-by-16 `SpriteVisual` tile grid, a nine-grid selection brush, `ShapeVisual` drawing, relative
  sizing, pointer hit testing, and `Vector3` key-frame animation.
- [`canvas`](../../crates/samples/composition/canvas) draws Direct2D content into a composition
  surface through `windows-canvas`.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is for contributors and is
not needed to use `windows-composition`.

### Dependencies

`windows-composition` depends on [`windows-core`](windows-core.md),
[`windows-numerics`](windows-numerics.md), and [`windows-collections`](windows-collections.md). It
does not depend on the `windows` crate.

The `system` feature also depends on [`windows-window`](windows-window.md). That feature provides
safe HWND hosting.

The crate reuses collection and numerics crates instead of generating those bindings again. Shape
collections use the `IVector` support from `windows-collections`. Implicit animation collections use
its `IMap` support. Offsets and sizes use `Vector2` and `Vector3` from `windows-numerics`.

Brush colors use a flat-bound `Windows.UI.Color` inside a small `Color` newtype. Callers do not use
the raw ABI struct.

### Wrapper model

Every safe type is a newtype over one owned COM interface. There is no boxing and no per-call
allocation. Methods call one COM method and return `windows_core::Result`. Safe callers do not use
`unsafe`.

Class wrappers store the most-derived interface and cast internally when needed. Each wrapper is
`Clone`. A clone is a cheap COM `AddRef`, so handles can be stored and shared.

### Class hierarchies and type families

Two patterns keep private `bindings::` types out of the public API.

- Concrete visual types implement `Deref` to their base types. `SpriteVisual` and `ShapeVisual`
  deref to `ContainerVisual`, which derefs to `Visual`. A `&SpriteVisual` works where a `&Visual` is
  expected.
- `Brush`, `Shape`, and `Animation` are sealed marker traits. Each trait exposes an `as_brush`,
  `as_shape`, or `as_animation` method. A method such as `SpriteVisual::set_brush(&impl Brush)`
  accepts any crate-defined brush type.

### Module layout

| Module | Contents |
| --- | --- |
| `bindings.rs` | Generated flat minimal system bindings for `Windows.UI.Composition` and `ICompositorDesktopInterop`. |
| `bindings_lifted.rs` | Generated flat minimal lifted bindings for `Microsoft.UI.Composition`. |
| `stack.rs` | System-only `DispatcherQueueController` bootstrap. |
| `compositor.rs` | `Compositor` factory for visuals, brushes, shapes, geometries, animations, batches, and system targets. `from_host` adopts a lifted WinUI compositor. |
| `target.rs` | System-only `DesktopWindowTarget`. |
| `visual.rs` | Visual wrappers, child collections, and visual properties. Lifted builds expose `Visual::as_raw`. |
| `shape.rs` | Shape visuals, shape traits, sprite and container shapes, ellipse geometry, and shape collections. |
| `brush.rs` | Brush traits, color brushes, nine-grid brushes, and surface brushes. |
| `animation.rs` | Animation traits, key-frame animations, easing functions, and implicit animation collections. |
| `batch.rs` | `CompositionScopedBatch` and `BatchKind`. |
| `color.rs` | `Color` newtype over `Windows.UI.Color`. |

### Code generation

Composition uses `tool_composition` instead of `tool_bindings`. The tool needs non-default winmd
inputs and emits two binding modules from one filter.

`tool_composition` runs `windows_bindgen` twice with `--flat --minimal --dead-code`.

- System bindings read the default `Windows.winmd` and `Windows.Win32.winmd`. They include
  `Windows.UI.Composition`, `Windows.System`, `ICompositorDesktopInterop`, `HWND`, and `BOOL`.
- Lifted bindings read `Microsoft.UI.winmd` from the reactor tool inputs and the default
  `Windows.winmd`. They include `Microsoft.UI.Composition` plus shared `Windows.Foundation` and
  `Windows.UI.Color` types.

`composition.txt` is the source filter. `// region: system-only` and `// endregion` mark entries
that exist only in the system stack. The tool removes those regions for lifted bindings. It also
rewrites `Windows.UI.Composition.` to `Microsoft.UI.Composition.` Shared `Windows.Foundation` and
`Windows.UI.Color` entries stay unchanged.

Edit `composition.txt` to change either stack. Then run `cargo run -p tool_composition` and verify
the affected crate.

`lib.rs` selects `bindings.rs` for `system` and `bindings_lifted.rs` for `lifted`. Compile errors
reject invalid feature combinations.

`stack.rs` declares `CreateDispatcherQueueController` with `windows_core::link!`. That function
comes from `dispatcherqueue.h` and is not in the repo's Win32 metadata. It creates the dispatcher
queue on the current thread.

### Feature unification and CI

The `system` and `lifted` features cannot build together. Cargo unifies features across a build
graph. A workspace build that includes both stack consumers would enable both features and hit the
compile error.

Unified CI jobs are lifted-primary. Reactor and its tests use the lifted stack, so the unified
clippy and test jobs exclude system-stack consumers. A second step checks those system crates
together. Add a new system-stack crate to both the exclusion list and the second step.

`tool_yml` also handles this crate. Generated MSRV and no-default-features matrices cannot use
`--all-features` or `--no-default-features` for this crate. They check each stack directly. Re-run
`cargo run -p tool_yml` after changing those generators.

### Reactor host bridge

Lifted composition is hosted inside a WinUI element. Reactor owns that element tree. Reactor depends
on `windows-composition` with the `lifted` feature.

Reactor's `CompositionHost` widget exposes typed methods on `CompositionHostHandle`:

```rust,ignore
let compositor = host.compositor()?;         // Compositor::from_host(element's compositor)
let root = compositor.create_container_visual();
host.set_child_visual(&root)?;               // set the element's child visual
```

This crate provides the lifted binding set and seam helpers for that bridge.
`Compositor::from_host(&IInspectable)` adopts the element's compositor.
`Visual::{from_host, as_raw}` adopts or exposes a visual's interop `IInspectable`.

Both crates use the same `Microsoft.UI.winmd` input for lifted bindings. The `IInspectable` values
have matching IIDs, so the casts in the seam helpers are ABI-safe. Reactor's animation engine also
uses this crate's key-frame, easing, and implicit-animation wrappers.

### Canvas bridge

Direct2D content can be drawn into a composition surface and painted onto a visual. This mirrors
Win2D's `CanvasComposition`. This crate owns the composition half.

- `Compositor::create_graphics_device(&impl Interface)` wraps
  `ICompositorInterop::CreateGraphicsDevice`. It adopts the app's Direct2D or DXGI rendering device
  into a `CompositionGraphicsDevice`.
- `CompositionGraphicsDevice::create_drawing_surface(width, height)` allocates a premultiplied-BGRA
  `CompositionDrawingSurface`.
- `Compositor::create_surface_brush(&surface)` produces a `CompositionSurfaceBrush` that implements
  `Brush`.
- `CompositionDrawingSurface::{begin_draw::<T>, end_draw, resize}` wraps
  `ICompositionDrawingSurfaceInterop`. `begin_draw` returns the drawing target `T` and the
  backing-atlas `(x, y)` pixel offset. `windows-canvas` passes them to
  `DrawingSession::from_borrowed_context(context, offset)`.

The Direct2D drawing lives in `windows-canvas` behind its `composition` feature. Import
`windows_canvas::CanvasCompositionExt` to use `device.create_graphics_device(&compositor)` and
`surface.draw(|session| ...)`. The draw method returns `Result<bool>`. `Ok(false)` means device
loss.

The bridge is system-only. Lifted `Microsoft.UI.Composition` has no Direct2D surface interop
metadata. The related filter entries are inside the `system-only` region, and the wrappers use
`#[cfg(feature = "system")]`.

### Testing

`test_composition` has pure-value tests and headless live tests.

- Pure-value tests cover `Color` constructors and component round trips.
- Headless live tests create a `DispatcherQueueController`, build a real `Compositor`, and exercise
  visual, brush, shape, and animation wrappers. The tests do not need a window or message pump, so
  they run in CI.

Getters read from the same COM objects that setters write. This also verifies that wrapper methods
route to the correct versioned interface. Window hosting is covered by the runnable `standalone`
sample.