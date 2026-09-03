# windows-composition

> Safe wrappers for the Windows retained-mode composition engine.

- 📦 [crates.io](https://crates.io/crates/windows-composition)
- 📖 [docs.rs](https://docs.rs/windows-composition)
- 🚀 [Getting started](../../crates/libs/composition/readme.md)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/composition)
- [Reactor hosting samples](../../crates/samples/reactor/composition)
- [Canvas guide](windows-canvas.md)
- [Window guide](windows-window.md)

## When to use it

Use Composition for a retained tree of lightweight visuals, brushes, shapes, and animations. It is
a good fit for animated backgrounds, overlays, game boards, transitions, and scenes where Windows
should retain and compose objects between updates.

Composition does not provide controls, text input, accessibility behavior, or layout panels. Use
[`windows-reactor`](windows-reactor.md) for WinUI controls. Use
[`windows-canvas`](windows-canvas.md) when custom drawing is easier as immediate-mode 2D commands.
Composition can host Canvas-drawn surfaces.

## Getting started: choose a hosting path

The crate supports two separate composition stacks. Select exactly one Cargo feature.

| Feature | Stack | Host | Choose it when |
| --- | --- | --- | --- |
| `system` (default) | `Windows.UI.Composition` | HWND target | The app owns a window |
| `reactor` | `Microsoft.UI.Composition` | WinUI `Grid` | The visual is in a Reactor view |

The stacks are different COM object graphs. A visual created by one cannot be attached to the
other. Cargo features are mutually exclusive, so disable default features when selecting
`reactor`.

Use system composition for standalone visual applications and for the Canvas composition-surface
bridge. Use lifted composition for visuals placed among WinUI controls.

## Core model

A `Compositor` is the factory and owner context for a composition graph. Create visuals, brushes,
shapes, easing functions, and animations from the same compositor as their destination tree.

Visuals form an ordered retained tree:

- `ContainerVisual` holds child visuals.
- `SpriteVisual` fills its bounds with a brush and can also hold children.
- `ShapeVisual` renders a collection of composition shapes.
- `Visual` supplies size, offset, opacity, scale, visibility, relative layout, and animation.

`SpriteVisual` dereferences to `ContainerVisual`, which dereferences to `Visual`. This lets a sprite
use child and base visual methods without casts. Shape visuals dereference to `Visual`.

Brushes define visual content. The crate provides color, nine-grid, and system-only surface
brushes. Shapes pair composition geometry with brushes and live in a `ShapeVisual`.

All wrappers are cloneable COM handles. Keep the compositor, host target, root visual, and any
objects that the application will update. Windows retains objects attached to the graph, but clear
ownership in application state makes resize, input, and recovery code easier to manage.

## Common workflows

### Host system composition in a window

System composition requires a dispatcher queue on the compositor thread:

1. Create `DispatcherQueueController::create_on_current_thread`.
2. Create a `Compositor`.
3. Create a `windows_window::Window`.
4. Call `create_desktop_window_target(&window, is_topmost)`.
5. Create a root `ContainerVisual` and call `target.set_root(&root)`.
6. Add child visuals and enter the window message loop.

Declare the queue before composition objects so it outlives them. Keep the
`DesktopWindowTarget` alive while its visual tree should remain connected.

Use `create_desktop_window_target_for_hwnd` only for a live HWND owned by the calling thread. The
safe `windows-window` overload is preferred.

Composition does not automatically resize the root or children. Handle the window resize callback
and update sizes, or set relative size and offset adjustments when the visual should track its
parent.

### Host lifted composition in Reactor

The Reactor bridge uses an `ElementRef<Grid>` and a retained observation:

1. Store the typed reference and composition scene state in a component.
2. Attach the reference to a `Grid`.
3. In `use_effect`, call `observe_composition_host` and retain the returned observation in cleanup.
4. On `CompositionHostEvent::Ready`, create a `Compositor` with `Compositor::from_host`.
5. Build a root visual and attach it with `request_set_child_visual`.
6. On `Ready` and `Metrics`, update the visual tree for width, height, and scale.

The host `Grid` and its element visual remain owned by Reactor. Only attach application-created
visuals. A request returning `false` means the reference is not currently bound. An accepted
request completes with `Result<(), CompositionHostError>`.

Detach with `request_set_child_visual(None, ...)` when application state requires it. Dropping the
observation stops host events and prevents rebinding after the component retires.

### Build and update a visual tree

Create a root container and use its `children()` collection:

- `insert_at_bottom` draws a child behind existing children.
- `insert_at_top` draws it above existing children.
- `remove` detaches one child.
- `remove_all` clears the collection.

Set a visual's `size` and `offset` in DIPs. For parent-relative layout,
`set_relative_size_adjustment` scales size by the parent and
`set_relative_offset_adjustment` adds a parent-relative offset.

A `SpriteVisual` paints its bounds with any crate-defined `Brush`. Create a color brush for flat
content. Use `CompositionNineGridBrush` when scalable edge and center regions are needed. For
Direct2D content, create a system composition surface brush as described below.

### Draw shapes

For a retained vector shape:

1. Create a `ShapeVisual` and set its size and offset.
2. Create an ellipse geometry and set its radius.
3. Create a sprite shape from the geometry.
4. Set the sprite shape's fill brush and offset.
5. Append it to `shape_visual.shapes()`.

Use `CompositionContainerShape` to group shapes. Shape collections accept any crate-defined type
implementing the sealed `Shape` trait.

### Animate properties

Create an animation type matching the property:

- `ScalarKeyFrameAnimation` for properties such as `"Opacity"`.
- `Vector3KeyFrameAnimation` for properties such as `"Scale"` or `"Offset"`.

Insert key frames with progress in `0.0..=1.0`, set duration, and call
`visual.start_animation(property, &animation)`. Use a linear or cubic-bezier easing function when
inserting eased key frames.

Use `CompositionAnimationGroup` when a lifted Reactor host API needs several animations as one
payload. Add each animation and convert the group with `as_host`. To animate a composition visual
directly, call `start_animation` once for each scalar or vector animation.

For automatic transitions after a property changes:

1. Create an implicit animation collection.
2. Set each animation's target property.
3. Insert each animation under the same property name.
4. Attach the collection with `visual.set_implicit_animations(Some(&collection))`.
5. Change the visual property normally.

Create a `CompositionScopedBatch`, start related work, and call `end` to seal the tracked work.
`BatchKind` selects animation, effect, infinite-animation, or all-animation work.

Property names are composition strings such as `"Opacity"` and `"Scale"`. A typo is not checked by
Rust, so keep names next to the animation construction and use the documented composition property
name.

### Draw Canvas content into a visual

This path requires system composition and the Canvas `composition` feature:

1. Create a Canvas `GpuDevice`.
2. Create a `CompositionGraphicsDevice` from the compositor.
3. Create and size a `CompositionDrawingSurface`.
4. Create a surface brush and set it on a `SpriteVisual`.
5. Import `CanvasCompositionExt` and draw into the surface.

Resize both the sprite and surface when their destination changes. Canvas drawing coordinates are
pixels on this path. Clear or cover the full surface on each draw.

If `surface.draw` returns `Ok(false)`, recreate the Canvas device, composition graphics device,
surface, and brush. The lifted Reactor stack does not expose this Direct2D surface bridge.

## Pitfalls

- Do not mix system and lifted visuals, compositors, or hosts.
- Enable exactly one of the `system` and `reactor` features.
- Keep the system dispatcher queue and desktop target alive.
- Create graph objects from the compositor that owns their destination.
- Update visual sizes on host resize or use relative adjustments.
- Remember that child insertion order determines z-order.
- Use DIPs for visual geometry and pixels for Canvas composition surfaces.
- Check the result of Reactor child-visual attachment and retain the host observation.
- Treat animation property names as exact, case-sensitive API strings.

## Samples

Choose a sample for the hosting path and task:

| Goal | Sample |
| --- | --- |
| Create an HWND target and sprites | [`standalone`][composition-standalone] |
| Build an interactive visual app | [`minesweeper`][composition-minesweeper] |
| Draw Canvas content into a visual | [`canvas`](../../crates/samples/composition/canvas) |
| Learn the Reactor host lifecycle | [`host`][composition-host] |
| Respond to lifted host DPI and size | [`dpi`][composition-dpi] |
| Attach and detach a lifted tree | [`toggle`][composition-toggle] |
| Animate lifted visuals | [`animation`][composition-animation] |
| Build many lifted shapes | [`circles`][composition-circles] |

[composition-standalone]: ../../crates/samples/composition/standalone
[composition-minesweeper]: ../../crates/samples/composition/minesweeper
[composition-host]: ../../crates/samples/reactor/composition/examples/host.rs
[composition-dpi]: ../../crates/samples/reactor/composition/examples/dpi.rs
[composition-toggle]: ../../crates/samples/reactor/composition/examples/toggle.rs
[composition-animation]: ../../crates/samples/reactor/composition/examples/animation.rs
[composition-circles]: ../../crates/samples/reactor/composition/examples/circles.rs

Start with `standalone` for system composition or `host` for Reactor. `minesweeper` demonstrates
relative sizing, pointer hit testing, sprite and shape visuals, nine-grid brushes, and key-frame
animation in one application.

---

## Internal documentation

The remainder of this page describes how the crate is built and maintained. Applications do not
need it to use Composition.

### Dependencies and feature model

The crate depends on `windows-core`, `windows-numerics`, `windows-collections`, and `windows-time`,
not the umbrella `windows` crate. Visual offsets and sizes reuse `Vector2` and `Vector3`. Shape
collections use `IVector`, implicit animation collections use `IMap`, and animation timing uses
`TimeSpan`.

The `system` feature also depends on `windows-window` and enables dispatcher queue, HWND target,
graphics-device, and drawing-surface support. The `reactor` feature selects lifted bindings and
exposes host conversions. Compile errors reject zero or two selected stacks.

Cargo unifies features across a build graph. Unified CI jobs that build Reactor exclude
system-stack consumers, then check those consumers in a separate step. Add new consumers to the
matching CI groups. `tool_yml` generates the stack-specific MSRV and feature checks.

### Wrapper model

Each safe type is a newtype over one owned COM interface. Concrete visual types dereference through
their base hierarchy. `Brush`, `Shape`, and `Animation` are sealed traits that keep private
generated binding types out of public signatures.

Constructors and immediate recoverable operations return `windows_core::Result`. Retained-object
property and factory conveniences fail fast on COM errors. `Color` wraps the flat
`Windows.UI.Color` ABI value.

| Module | Contents |
| --- | --- |
| `compositor.rs` | Factories for graph objects and stack-specific hosts |
| `visual.rs` | Visual hierarchy, properties, and child collections |
| `brush.rs` | Brush hierarchy and implementations |
| `shape.rs` | Shape visuals, geometry, shapes, and collections |
| `animation.rs` | Key frames, easing, groups, and implicit animations |
| `batch.rs` | Scoped batches and batch kinds |
| `surface.rs` | System graphics devices and drawing surfaces |
| `stack.rs`, `target.rs` | System dispatcher queue and HWND hosting |
| `color.rs` | Public color value |

### Code generation

Composition uses `tool_composition` because it needs system and non-default lifted metadata. The
tool runs `windows_bindgen` twice with `--flat --minimal --dead-code`.

- System bindings read the repository's Windows and Win32 metadata and include
  `Windows.UI.Composition`, dispatcher queue, desktop interop, and surface interop APIs.
- Lifted bindings read the pinned Microsoft UI metadata used by Reactor and include
  `Microsoft.UI.Composition` with shared foundation and color types.

`crates/tools/composition/src/composition.txt` is the shared filter. System-only regions are omitted
from lifted generation, and composition namespace entries are rewritten for the lifted stack.
`lib.rs` selects `bindings.rs` or `bindings_lifted.rs`.

Generated binding files are committed and must not be edited by hand. After changing the filter,
metadata inputs, or tool, run:

```text
cargo run -p tool_composition --quiet
cargo check -p windows-composition --quiet
```

Run `cargo run -p tool_yml` after changing feature-matrix generation.

### Reactor bridge

Reactor reports `CompositionHostEvent` through a typed `ElementRef<Grid>`. A ready event contains
an application-safe compositor capability and layout metrics. `Compositor::from_host` adopts that
capability. `Visual::as_raw` exports only an application-owned lifted visual for attachment.

Both crates generate lifted bindings from the same pinned Microsoft UI metadata. Host attachment
and observation commands exist in Reactor's native and recording runtimes. Window and node
identities reject late events and completions.

Reactor also consumes lifted key-frame, group, easing, and implicit-animation wrappers for element
lifecycle transitions.

### Canvas bridge

The system stack exposes `ICompositorInterop` and
`ICompositionDrawingSurfaceInterop`. A Canvas device becomes a
`CompositionGraphicsDevice`, which creates a premultiplied BGRA drawing surface. A composition
surface brush paints that surface onto a visual.

`windows-canvas` implements the drawing half behind its `composition` feature. It borrows the
Direct2D context returned by `begin_draw`, applies the backing-atlas offset, and pairs the operation
with `end_draw`. Surface interop remains system-only.

### Testing

`test_composition` includes pure value tests and live headless tests. The live tests create a
dispatcher queue and real compositor, then exercise visual, brush, shape, collection, and animation
wrappers without a window. Getter checks also verify that wrapper methods route to the intended
versioned COM interface.

Run:

```text
cargo test -p test_composition
```

Window hosting is covered by the standalone sample. Reactor host behavior is covered by Reactor's
recording tests and live self-test fixtures.
