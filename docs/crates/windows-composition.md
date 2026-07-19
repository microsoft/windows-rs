# windows-composition

> **Status: scaffolding landed — early, minimal surface.** The crate exists with
> a generated binding layer and a first slice of safe wrappers
> (`Compositor` with standalone `new()` + `from_raw` adoption,
> `DispatcherQueueController`, `Visual` with `as_interface` interop,
> `ContainerVisual`, `SpriteVisual`, `VisualCollection`, `CompositionColorBrush`,
> `Color`). The reactor seam has landed — the `CompositionHost` widget in
> [`windows-reactor`](windows-reactor.md) hosts a wrapper-built visual tree (see
> the [`reactor/composition_host`](../../crates/samples/reactor/composition_host)
> sample). Interop, the canvas bridge, and standalone samples are still to come —
> see the checklist below.

- 📦 Not published
- 🧩 Proposed sibling of [`windows-canvas`](windows-canvas.md) and
  [`windows-reactor`](windows-reactor.md)

## Purpose

`windows-composition` would wrap the `Microsoft.UI.Composition` API behind safe
Rust types — a `Compositor` and its visual tree (`Visual`, `SpriteVisual`,
`ContainerVisual`), brushes (`CompositionColorBrush`, gradients, surface/effect
brushes), and the keyframe/expression animation types the compositor animates
off-thread. It is to `Microsoft.UI.Composition` what [`windows-canvas`](windows-canvas.md)
is to Direct2D/DXGI: a focused, hand-written safe layer over its own minimal,
flat bindings.

## Why a separate crate (namespace matters)

There are two distinct composition stacks, and they are **not interoperable**:

- `Windows.UI.Composition` — the *system* compositor, projected by the
  [`windows`](windows.md) crate (`Windows::UI::Composition`).
- `Microsoft.UI.Composition` — the *WinUI 3 / Windows App SDK* compositor, whose
  metadata lives in `Microsoft.UI.winmd` (staged from NuGet, not in the `windows`
  crate).

Reactor's elements are WinUI 3 elements, so their visuals come from
`Microsoft.UI.Xaml.Hosting.ElementCompositionPreview` →
`Microsoft.UI.Composition.Visual`. You **cannot** attach a system
`Windows.UI.Composition` visual to a WinUI 3 element. Any crate that builds
visuals reactor can host must therefore project `Microsoft.UI.Composition`
specifically — which is exactly why the `windows` crate's existing
`Windows.UI.Composition` projection does not solve this, and a dedicated crate is
needed.

This is understood to be a **hard requirement, not a preference**: WinUI 3 is
built on the *lifted* composition stack (`Microsoft.UI.Composition`), and the two
stacks are separate object graphs with distinct COM interface identities, so
`ElementCompositionPreview` accepts only a same-stack visual built from the
element's own `Microsoft.UI.Composition.Compositor`. There is no "use the system
one and cast" shortcut. **TODO: verify this hard requirement before scaffolding
the crate** — confirm empirically (e.g. attempt to attach a `Windows.UI.Composition`
visual to a WinUI 3 element and observe the failure) and/or against current
WinUI 3 / Windows App SDK documentation, since the whole rationale for a separate
crate rests on it.

## Forward-looking flexibility: one safe layer, swappable stack

Although the crate targets `Microsoft.UI.Composition`, the two composition stacks
are **member-identical** — `Microsoft.UI.Composition` is a lifted fork of
`Windows.UI.Composition` with the same class names, members, and signatures. The
bindings are generated in `--flat` mode, which **strips namespaces**, so
`Compositor` from `Microsoft.UI.Composition` and `Compositor` from
`Windows.UI.Composition` produce *identically named* flat types. The hand-written
safe layer (`src/*.rs`) therefore compiles unchanged against either stack.

The design isolates every stack-specific fact so a swap is a localized change and
the safe layer stays 100% shared:

| Stack-specific fact | Where it lives | Shared safe layer? |
| --- | --- | --- |
| Which winmd + namespace filter feeds codegen | `tool_composition` inputs + `composition.txt` | — |
| Interop interface IIDs (`ICompositorInterop` vs `ICompositorDesktopInterop`, …) | `tool_composition/src/extras.rdl` | — |
| Compositor activation + `DispatcherQueue` type | one `stack.rs` module | isolated |
| `Compositor`, `Visual`, brushes, animations, surfaces | `src/*.rs` over flat `bindings::*` | **yes — unchanged** |

Recommendation: ship a **single committed Microsoft.UI binding set** now (what
reactor needs). The standalone samples (see *Samples*) exercise the **system**
`Windows.UI.Composition` stack, which validates the swap path for real rather than
on paper — same `src/*.rs`, different `tool_composition` input + `stack.rs`. If a
second stack needs to ship, add a `system` Cargo feature selecting a second
committed `bindings_system.rs` + system `stack.rs`; no wrapper changes.

## Architecture (mirrors windows-canvas / windows-animation)

- **Dependencies:** [`windows-core`](windows-core.md) and
  [`windows-numerics`](windows-numerics.md) only — *not* the `windows` crate.
- **Zero-overhead newtypes.** Every safe type is a newtype over exactly one owned
  COM interface — no boxing, no per-call allocation — exactly as
  [`windows-animation`](windows-animation.md)
  (`pub struct Manager(IUIAnimationManager2)`) and
  [`windows-canvas`](windows-canvas.md) (`GpuDevice`, `DrawingSession`) do. Methods
  wrap a single `unsafe { self.0.Method(..) }` and return `windows_core::Result`;
  no `unsafe` at the call site. Class-to-default-interface relationships are modeled
  by storing the most-derived interface and `cast()`-ing internally (the same way
  reactor's backend already does: `visual.cast::<ICompositionObject>()`).
- **Numerics interop:** offsets, sizes, and matrices use `windows-numerics`
  (`Vector2`/`Vector3`/`Matrix4x4`/`Quaternion`), matching canvas. Brush color is a
  flat-bound `Windows.UI.Color`; wrap it in a small `Color` newtype (or reuse
  canvas's `ColorF` conversion) so callers never touch the raw ABI struct.

### Module layout

| Module | Contents |
| --- | --- |
| `bindings.rs` | Generated flat/minimal `Microsoft.UI.Composition` + interop (`tool_composition`). |
| `stack.rs` | The only stack-specific wrapper code: `Compositor` activation + `DispatcherQueue` bootstrap. |
| `compositor.rs` | `Compositor` — the factory: create visuals, brushes, animations, graphics device. |
| `visual.rs` | `Visual` (base), `ContainerVisual` (children), `SpriteVisual` (brush + size). |
| `brush.rs` | `CompositionBrush` base; `CompositionColorBrush`, linear/radial gradient, `CompositionSurfaceBrush`. |
| `surface.rs` | `CompositionGraphicsDevice`, `CompositionDrawingSurface` — the D2D/DXGI interop entry points. |
| `animation.rs` | `ScalarKeyFrameAnimation`, `Vector3KeyFrameAnimation`, `ExpressionAnimation`, `CompositionEasingFunction`. |
| `reactor.rs` *(feature)* | `composition_host(build)` convenience returning a reactor element (adopts the element's compositor via the reactor seam). |

## Codegen: a dedicated `tool_composition`

Unlike canvas/animation (whose bindings are a plain `bindgen(["--etc", "…txt"])`
call inside the shared `tool_bindings`), composition needs an imperative staging
step, so it gets its own tool — mirroring how `tool_reactor` stages `extras.winmd`
before generating (`crates/tools/reactor/src/main.rs`
`generate_reactor_bindings`). Two things force this:

1. **Non-default winmd input.** `Microsoft.UI.Composition` lives in
   `Microsoft.UI.winmd`, which is *not* one of bindgen's default inputs — it must
   be passed via `--in`. It is already staged and committed at
   `crates/tools/reactor/winmd/Microsoft.UI.winmd`.
2. **Interop interfaces aren't in metadata.** The COM interop interfaces that
   bridge composition to Direct2D/DXGI (`ICompositorInterop`,
   `ICompositionGraphicsDeviceInterop`, `ICompositionDrawingSurfaceInterop`; plus
   `ICompositorDesktopInterop` for the standalone system-stack samples) are declared
   in C++ headers, not the winmd. They must be hand-authored in an `extras.rdl` and
   merged to a winmd — exactly the pattern `tool_reactor` uses for
   `ISwapChainPanelNative` / `ISurfaceImageSourceNativeWithD2D`
   (`crates/tools/reactor/src/extras.rdl`).

`tool_composition` therefore: (1) merges
`crates/tools/composition/src/extras.rdl` into an `extras.winmd` via
`windows_rdl::Reader`; (2) runs `windows_bindgen` with `--in` = the staged
`Microsoft.UI.winmd` + `Windows.winmd` + the interop `extras.winmd`, plus
`--flat --minimal --dead-code --filter --etc crates/tools/composition/src/composition.txt`;
(3) emits `crates/libs/composition/src/bindings.rs` and the test bindings
`crates/tests/libs/composition/src/bindings.rs`. `composition.txt` is the filter
(the composition analogue of `canvas.txt`) — prune/extend the surface by editing
one file, just like canvas.

## The canvas ↔ composition bridge (the payoff)

The most valuable seam is drawing Direct2D content into a composition surface —
Win2D's `CanvasComposition`. It reuses machinery that **already exists** in
`windows-canvas`:

1. `Compositor::create_graphics_device(device)` →
   `ICompositorInterop::CreateGraphicsDevice(ID2D1Device)`. The device comes from
   canvas via `GpuDevice::d2d_device()`; composition accepts it as a plain
   `windows_core` COM pointer, so **composition never depends on canvas**.
2. `CompositionGraphicsDevice::create_drawing_surface(size)` →
   `CompositionDrawingSurface`.
3. To repaint: `ICompositionDrawingSurfaceInterop::BeginDraw` returns an
   `ID2D1DeviceContext` + an offset `POINT` — **exactly** the two inputs
   `windows-canvas`'s `DrawingSession::from_borrowed_context(context, offset)`
   already consumes for `CanvasImageSource` (`crates/libs/canvas/src/reactor.rs:290`).
   Draw with the safe canvas `DrawingSession` API, then `EndDraw`.

To avoid a dependency cycle, the bridge lives in **`windows-canvas` behind a new
`composition` feature** that depends on `windows-composition` and reuses canvas's
own panic-safe borrowed-`DrawingSession` path (the `EndDrawGuard` bracket
`CanvasImageSource::draw` uses). Dependency direction stays one-way:
`windows-canvas` → `windows-composition`; composition core stays canvas-free. The
result: `SpriteVisual` + `CompositionSurfaceBrush(surface)` displays Direct2D-drawn
content in the visual layer — the composition-visual complement to canvas's
existing DXGI swap-chain path.

## Layering with reactor and canvas

The three crates compose without overlapping:

- **reactor** owns the XAML element tree and exposes the *seam* to reach the
  compositor under any element — three `Backend` methods:
  `element_compositor(host)`, `element_rasterization_scale(host)`, and
  `set_element_child_visual(host, visual)` (see gap #5 in
  [`windows-reactor.md`](windows-reactor.md)). Reactor does **not** build visuals.
- **windows-composition** builds the visuals: a caller gets the element's
  `Compositor` from the reactor seam, builds a same-compositor `Visual` tree with
  safe `windows-composition` types, and hands the root back via
  `set_element_child_visual`.
- **windows-canvas** already covers the *DXGI swap-chain* path
  (`SwapChainPanel` / `ISwapChainPanelNative::SetSwapChain`) and, via its new
  `composition` feature, the *composition-surface draw* bridge above. Composition
  covers the complementary *composition-visual* path (custom-drawn / off-thread
  animated content hosted under a plain element).
- **windows-animation** stays independent (Win32 UIAnimation Manager); composition
  brings its own compositor-driven keyframe/expression animations.

This is the same relationship canvas already has with reactor: an optional
`reactor` feature can offer a hosted convenience (e.g. a `composition_host(build)`
helper returning a reactor element) while the core crate stays usable standalone.

## Samples

A new `crates/samples/composition/` directory collects both minimal and richer
samples (as `crates/samples/canvas/` does). Because they are **standalone** (no
WinUI 3 / reactor), they most naturally target the **system**
`Windows.UI.Composition` stack via `ICompositorDesktopInterop::CreateDesktopWindowTarget`
— which requires **no Windows App SDK bootstrap** (see below) and doubles as
real-world validation of the swap-stack design.

- **minimal** — a `Compositor`, a colored `SpriteVisual` in a
  `DesktopWindowTarget` on a plain `windows-window` HWND. The "hello triangle" of
  composition.
- **Port of `crates/samples/windows/dcomp`.** That sample today uses **Win32
  DirectComposition** (`IDCompositionDevice`, `IDCompositionRotateTransform3D`) plus
  UIAnimation + Direct2D + WIC to animate a card-matching game. Reimplementing it on
  the WinRT composition stack — visuals/transforms via `windows-composition`,
  per-card drawing via the canvas bridge, motion via composition keyframe animations
  — is *not* a mechanical port but exercises visuals, surface brushes, animations,
  and the canvas bridge end-to-end. (Keep the original DirectComposition sample; add
  the WinRT one alongside.)
- **Port of [minesweeper-rs](https://github.com/robmikh/minesweeper-rs).** robmikh's
  standalone `Windows.UI.Composition` minesweeper is the canonical stress test: a
  large `SpriteVisual` tile grid, sprite-sheet `CompositionSurfaceBrush`, nine-grid
  scaling, hit-testing by visual layout, and reveal animations. It validates the
  crate's design and usefulness on a real app and, being system-stack + standalone,
  proves the swap path. (Distinct from the existing *reactor* `minesweeper.rs`
  example, which is a XAML-control game.)

Naming note: a reactor `composition.rs` example already exists but is unrelated
(`Element::Group` / `CustomElement`); the new samples live under
`crates/samples/composition/` to avoid confusion.

## Bootstrapping and deployment — do we need a `windows-reactor-setup`?

The answer depends entirely on **which stack** a given consumer uses, and this is
one more reason the stack split matters:

- **System stack (`Windows.UI.Composition`)** — part of the OS. **No bootstrap,
  no `windows-reactor-setup`, no manifest.** Works in any HWND with just a
  `DispatcherQueueController.CreateOnCurrentThread` + `CreateDesktopWindowTarget`.
  This is why the standalone samples above have zero deployment friction, and why
  robmikh's minesweeper runs as a plain `cargo run`.
- **Lifted stack (`Microsoft.UI.Composition`)** — a **Windows App SDK** component,
  *not* in the OS. It needs the WinAppSDK runtime present plus the bootstrap
  (`MddBootstrapInitialize`, authored in `crates/tools/reactor/src/extras.rdl`) or a
  self-contained deployment. `windows-reactor-setup` is the `build.rs` helper that
  supplies this — `as_framework_dependent` (copy the bootstrap DLL + `resources.pri`)
  or `as_self_contained` (stage the WinAppSDK runtime DLLs + embed a manifest).

Consequences:

- **Reactor-hosted composition needs nothing new.** Reactor apps already call
  `windows_reactor_setup::as_self_contained()` in their `build.rs`; the lifted
  compositor and its bootstrap come for free. `windows-composition` inherits it.
- **Standalone *lifted* composition** (Microsoft.UI in your own HWND, no reactor)
  *would* need the same WinAppSDK bootstrap — but **reuse `windows-reactor-setup`**
  rather than writing a new crate. Despite the name it is generic WinAppSDK staging;
  a standalone lifted sample just adds it as a `build-dependency`. (A future rename
  to something stack-neutral like `windows-app-sdk-setup` is a nice-to-have, not a
  blocker.)
- **Standalone *system* composition** needs **no setup crate at all.**

So: no, we do not need a *new* bootstrap crate. We need to (a) keep the standalone
samples on the system stack so they need no bootstrap, and (b) let any standalone
lifted consumer reuse `windows-reactor-setup`.

## Testing: `test_composition`

Tests live in a dedicated `crates/tests/libs/composition` crate (`test_composition`),
matching the repo convention that library crates keep tests out of their own `src/`
(as `test_canvas` / `test_reactor` do). Composition needs a device for surface/brush
tests, so use a **WARP** device (as `test_canvas` does) so they run headless in CI.
Visual-tree construction, brush/animation property round-trips, and the
graphics-device/drawing-surface interop are unit-testable without a window;
hosting-under-an-element is covered by a `reactor_selftest` fixture once the reactor
seam lands.

## What could graduate from reactor

Reactor already drives `Microsoft.UI.Composition` internally for its implicit
transitions and property animations (`backend/winui/mod.rs` —
`ElementCompositionPreview::GetElementVisual`, `Visual.Compositor()`, keyframe
setup). That plumbing is currently `pub(crate)` and single-purpose. Once
`windows-composition` exists, those internals could move behind its safe types (or
a feature) so reactor consumes the wrapper instead of maintaining a private copy.

## Consistency checklist (make it a first-class crate)

- `crates/libs/composition/` — `Cargo.toml` (deps `windows-core` + `windows-numerics`;
  an optional `windows-reactor` behind a `reactor` feature is added later with the
  seam module), `readme.md`, both license files, `[lints] workspace = true`.
  Auto-joins the workspace via `crates/libs/*`.
- `crates/tools/composition/` (`tool_composition`) with `src/composition.txt` (and
  a later `src/extras.rdl` for interop); auto-joins via `crates/tools/*`; added to
  the `gen` workflow matrix (regenerate → fail on diff), like `tool_reactor`.
- `crates/tests/libs/composition/` (`test_composition`); auto-joins via `crates/tests/*/*`.
- `crates/samples/composition/` — minimal + dcomp + minesweeper ports.
- `windows-canvas` gains an optional `composition` feature + bridge module.
- Docs: this page + a crate `readme.md`; add rows to the `docs/readme.md` crate
  index and its "UI & graphics" grouping.

## Open questions

- **Winmd source of truth.** Reference the existing
  `crates/tools/reactor/winmd/Microsoft.UI.winmd` (no duplication) vs. stage a private
  copy under `crates/tools/composition/winmd`. Recommendation: reference the existing
  staged copy; both crates are pinned to the same Windows App SDK version.
- **Standalone Compositor construction.** The `Compositor` needs a
  `DispatcherQueue` on the thread (`Microsoft.UI.Dispatching` lifted /
  `Windows.System` system). Bind `DispatcherQueueController.CreateOnCurrentThread` in
  `stack.rs`; reactor already has a queue for the hosted path.
- **Bindings visibility.** Reactor's generated composition methods are `pub(crate)`;
  `windows-composition` needs a *public* flat/minimal projection — confirm
  `--flat --minimal` emits the intended visibility for a standalone crate (it does
  for canvas/animation).
- **Scope.** Start with the minimum reactor needs (compositor + container/sprite
  visual + color & surface brush + graphics-device/drawing-surface interop + basic
  scalar/vector keyframe animation), then grow toward Win2D's `CanvasComposition`
  surface area.

## Status / next steps

1. Land this plan (done — this page).
2. Confirm the hard requirement (system vs lifted visual attach) empirically.
3. Scaffold `crates/libs/composition`, `crates/tools/composition`
   (`composition.txt`), and `crates/tests/libs/composition`; wire the
   `Microsoft.UI.winmd` input. **(done)**
4. Generate bindings; hand-write the first `Compositor`/`Visual`/`SpriteVisual`/
   `ContainerVisual`/`CompositionColorBrush`/`Color` wrappers; add a value-level
   test. **(done — a minimal slice; opacity/brush/effect/surface/animation
   wrappers and WARP-backed runtime tests still to come.)**
5. Add standalone `Compositor::new()` (dispatcher queue + `stack.rs`) so a live
   compositor can be built for headless/runtime tests. **(done — `Compositor::new()`
   activates the lifted compositor; `DispatcherQueueController::create_on_current_thread()`
   supplies the required per-thread queue. Consumers must bootstrap the Windows App
   SDK first via `windows-reactor-setup`; WARP-backed runtime tests need that runtime
   present, so they land with the first sample.)**
6. Author interop (`extras.rdl`: `ICompositorInterop`,
   `ICompositionGraphicsDeviceInterop`, `ICompositionDrawingSurfaceInterop`,
   `ICompositorDesktopInterop`); add the canvas `composition` feature and the
   draw-into-surface bridge.
7. Add `crates/samples/composition/` (minimal, dcomp, minesweeper) on the system
   stack — validating the crate and the swap path with zero bootstrap.
8. Add the reactor seam (gap #5) and a hosted `reactor`-feature convenience.
   **(done — the reactor-side seam landed as the `CompositionHost` widget in
   [`windows-reactor`](windows-reactor.md): `composition_host()` delivers a
   `CompositionHostHandle` with `compositor()` / `set_child_visual()` /
   `clear_child_visual()` / `on_rasterization_scale_changed()`, binding
   `ElementCompositionPreview::SetElementChildVisual`. The handle traffics in raw
   `IInspectable`, so it pairs with `Compositor::from_raw` / `Visual::as_interface`
   without reactor depending on this crate. Exercised end-to-end by the
   [`reactor/composition_host`](../../crates/samples/reactor/composition_host)
   sample. An optional composition-side `reactor`-feature convenience wrapper is
   still to come — the raw-`IInspectable` seam already makes it unnecessary for
   callers.)**
9. Graduate reactor's private composition plumbing onto the wrapper.
