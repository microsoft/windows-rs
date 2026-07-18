# windows-composition

> **Status: planned — this crate does not exist yet.** This page is the design
> plan for a proposed `windows-composition` crate. Nothing here is shipped.

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

## Architecture (mirrors windows-canvas)

- **Dependencies:** [`windows-core`](windows-core.md) and
  [`windows-numerics`](windows-numerics.md) only — *not* the `windows` crate.
- **Bindings:** minimal, flat bindings for `Microsoft.UI.Composition` generated
  by `tool_bindings` from a new `crates/tools/bindings/src/composition.txt` filter
  into `crates/libs/composition/src/bindings.rs`. The metadata source is
  `Microsoft.UI.winmd`, staged the same way `tool_reactor` already stages it
  (`crates/tools/reactor/src/stage.rs`).
- **Safe wrappers:** hand-written types over the generated interfaces
  (`Compositor`, `Visual`/`SpriteVisual`, brushes, keyframe animations), following
  the canvas convention (owned COM objects behind safe methods, no `unsafe` at the
  call site).
- **Numerics interop:** offsets, sizes, colors, and matrices use
  `windows-numerics` types (`Vector2`/`Vector3`/`Matrix4x4`), matching canvas.

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
  (`SwapChainPanel` / `ISwapChainPanelNative::SetSwapChain`). Composition covers
  the complementary *composition-visual* path (custom-drawn / off-thread animated
  content hosted under a plain element). A canvas surface can be presented into a
  composition `SpriteVisual` (Win2D `CanvasComposition` style) once both exist.

This is the same relationship canvas already has with reactor: an optional
`reactor` feature can offer a hosted convenience (e.g. a `composition_host(build)`
helper returning a reactor element) while the core crate stays usable standalone.

## What could graduate from reactor

Reactor already drives `Microsoft.UI.Composition` internally for its implicit
transitions and property animations (`backend/winui/mod.rs` —
`ElementCompositionPreview::GetElementVisual`, `Visual.Compositor()`, keyframe
setup). That plumbing is currently `pub(crate)` and single-purpose. Once
`windows-composition` exists, those internals could move behind its safe types (or
a feature) so reactor consumes the wrapper instead of maintaining a private copy.

## Open questions

- **Bindings visibility.** Reactor's generated composition methods are
  `pub(crate)`. `windows-composition` needs a *public* flat/minimal projection;
  confirm `tool_bindings` emits the intended visibility for a standalone crate (it
  does for canvas).
- **Surface interop with canvas.** The exact seam for presenting a Direct2D/DXGI
  surface into a composition brush (`ICompositorInterop::CreateCompositionSurfaceForSwapChain`
  / `CreateGraphicsDevice`) needs the WinUI-flavored interop interfaces bound.
- **Scope.** Start with the minimum reactor needs (compositor + sprite visual +
  color/surface brush + basic keyframe animation), then grow toward Win2D's
  `CanvasComposition` surface area.

## Status / next steps

1. Land this plan (done — this page).
2. Scaffold `crates/libs/composition` + `crates/tools/bindings/src/composition.txt`;
   wire `Microsoft.UI.winmd` staging.
3. Generate bindings, hand-write the `Compositor`/`Visual`/brush wrappers, add
   tests (WARP where a device is needed).
4. Add the reactor seam (gap #5) and a hosted `reactor`-feature convenience.
5. Add canvas ↔ composition surface interop.
