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

## The basic idea

Composition is a retained graphics API. You create a tree of visual objects, change their
properties when application state changes, and let Windows draw and animate the result.

```text
create visuals -> attach them to a tree -> update properties -> Windows composes the scene
```

This differs from Canvas, where the application issues drawing commands for each frame. A
Composition visual stays in the tree until it is removed. Changing its offset, size, brush, or
animation is enough to change what appears on screen.

A `Compositor` creates every object in a scene. The most common visual types are:

| Type | Purpose |
| --- | --- |
| `ContainerVisual` | Groups child visuals without drawing content |
| `SpriteVisual` | Draws a rectangular brush and can contain children |
| `ShapeVisual` | Draws retained vector shapes |

Start with sprite visuals. They are enough for backgrounds, cards, overlays, game pieces, and many
animations.

## Choose how to host the scene

The crate supports two Composition stacks:

| Feature | Use it when |
| --- | --- |
| `system` (default) | The app owns a `windows-window` window |
| `reactor` | The scene belongs inside a Reactor view |

The two stacks use different Windows APIs and their objects cannot be mixed. Cargo features are
additive, so the `reactor` feature takes precedence if dependency feature unification also enables
the default `system` feature. Applications can still disable the default feature to state their
intent directly:

```toml
windows-composition = { version = "0.100.0", default-features = false, features = ["reactor"] }
```

Both stacks can create a graphics device and drawing surface for `windows-canvas` through Canvas's
`composition` feature.

The examples below begin with the default system stack because it shows the visual model with the
least framework code.

## Your first visual tree

This complete program creates a window and fills it with a dark sprite:

```rust,no_run
use windows_composition::*;
use windows_window::*;

fn main() -> Result<()> {
    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

    let window = Window::new("Composition")
        .size(800, 600)
        .create()?;
    let target = compositor.create_desktop_window_target(&window, false)?;

    let root = compositor.create_sprite_visual();
    root.set_relative_size_adjustment(Vector2::new(1.0, 1.0));
    root.set_brush(&compositor.create_color_brush(CompositionColor::rgb(30, 30, 46)));
    target.set_root(&root);

    run();
    Ok(())
}
```

The dispatcher queue and target must stay alive while the scene is displayed. Declaring them in
`main` keeps their lifetimes clear.

The `Compositor` creates both the visual and its brush. Composition objects in one tree should come
from the same compositor.

## Build a scene from nested visuals

Visuals form an ordered tree. Insert a child at the top to draw it in front of existing children,
or at the bottom to draw it behind them:

```rust,ignore
let card = compositor.create_sprite_visual();
card.set_size(240.0, 160.0);
card.set_offset(40.0, 40.0, 0.0);
card.set_brush(&compositor.create_color_brush(CompositionColor::rgb(0, 120, 215)));

let badge = compositor.create_sprite_visual();
badge.set_size(32.0, 32.0);
badge.set_offset(192.0, 16.0, 0.0);
badge.set_brush(&compositor.create_color_brush(CompositionColor::rgb(255, 255, 255)));

card.children().insert_at_top(&badge);
root.children().insert_at_top(&card);
```

The badge's offset is relative to the card. Moving the card moves both visuals. Keep handles to
visuals that the application will change later; attached visuals remain in the scene even if a
local clone is dropped.

Use the child collection to change the tree:

```rust,ignore
root.children().remove(&card);
root.children().remove_all();
```

## Size and position visuals

`set_size` uses width and height. `set_offset` uses x, y, and z:

```rust,ignore
visual.set_size(200.0, 120.0);
visual.set_offset(24.0, 48.0, 0.0);
```

Composition does not lay out a scene like WinUI. Either update exact sizes when the host changes or
make a child track its parent:

```rust,ignore
background.set_relative_size_adjustment(Vector2::new(1.0, 1.0));
```

A relative size of `(1.0, 1.0)` fills the parent. Relative adjustments can be combined with an
exact size or offset, which is useful for margins:

```rust,ignore
panel.set_relative_size_adjustment(Vector2::new(1.0, 1.0));
panel.set_size(-32.0, -32.0);
panel.set_offset(16.0, 16.0, 0.0);
```

Coordinates are device-independent pixels. The host is still responsible for reporting its own
size changes when exact dimensions are used.

## Animate a property

Animations are retained objects too. Create an animation, add key frames, and start it on a visual:

```rust,ignore
use std::time::Duration;

let pulse = compositor.create_vector3_key_frame_animation();
pulse.insert_key_frame(0.0, Vector3::new(1.0, 1.0, 1.0));
pulse.insert_key_frame(0.5, Vector3::new(1.2, 1.2, 1.0));
pulse.insert_key_frame(1.0, Vector3::new(1.0, 1.0, 1.0));
pulse.set_duration(Duration::from_millis(800));
pulse.set_iterate_forever();

card.start_animation("Scale", &pulse);
```

Windows advances the animation without an application frame loop. Use
`ScalarKeyFrameAnimation` for one-number properties such as `"Opacity"` and
`Vector3KeyFrameAnimation` for properties such as `"Scale"` and `"Offset"`.

Property names are exact, case-sensitive Composition strings. Rust cannot catch a misspelling in
`"Scale"`.

## Draw retained shapes

Use a `ShapeVisual` when the scene needs an ellipse or another vector shape rather than a
rectangular sprite:

```rust,ignore
let visual = compositor.create_shape_visual();
visual.set_size(200.0, 200.0);

let geometry = compositor.create_ellipse_geometry();
geometry.set_radius(Vector2::new(80.0, 80.0));

let circle = compositor.create_sprite_shape(&geometry);
circle.set_offset(Vector2::new(100.0, 100.0));
circle.set_fill_brush(&compositor.create_color_brush(CompositionColor::rgb(0, 120, 215)));
visual.shapes().append(&circle);

root.children().insert_at_top(&visual);
```

Shapes remain editable after they are attached. Change the geometry, brush, or visual properties
instead of rebuilding the scene for every update.

## Put Composition inside Reactor

With the `reactor` feature, a Reactor `Grid` provides the host. The flow is:

1. Attach an `ElementRef<Grid>` to the grid.
2. Observe the composition host from `use_effect`.
3. On `CompositionHostEvent::Ready`, adopt the supplied compositor with
   `Compositor::from_host`.
4. Build a visual tree and attach it with `request_set_child_visual`.
5. Update its size on `Ready` and `Metrics` events.

The important bridge calls look like this:

```rust,ignore
let compositor = Compositor::from_host(compositor)?;
let root = compositor.create_sprite_visual();
root.set_relative_size_adjustment(Vector2::new(1.0, 1.0));
root.set_brush(&compositor.create_color_brush(CompositionColor::rgb(30, 30, 46)));

let _ = host.request_set_child_visual(Some(root.host_visual()), |result| {
    if let Err(error) = result {
        eprintln!("failed to attach composition visual: {error:?}");
    }
});
```

The observation must live until the effect is cleaned up. The
[`host`](../../crates/samples/reactor/composition/examples/host.rs) sample contains the complete
component lifecycle; start there rather than copying only the bridge calls.

## What to read next

| Sample | What it shows |
| --- | --- |
| [`standalone`](../../crates/samples/composition/standalone) | Window hosting and sprite visuals |
| [`host`](../../crates/samples/reactor/composition/examples/host.rs) | Reactor host lifecycle |
| [`animation`][composition-animation] | Key-frame animation |
| [`circles`](../../crates/samples/reactor/composition/examples/circles.rs) | A larger shape scene |
| [`dpi`][composition-dpi] | Reactor size and scale events |
| [`minesweeper`][composition-minesweeper] | Input, layout, shapes, and animation |
| [`canvas`](../../crates/samples/composition/canvas) | Canvas drawing in a composition surface |

[composition-animation]: ../../crates/samples/reactor/composition/examples/animation.rs
[composition-dpi]: ../../crates/samples/reactor/composition/examples/dpi.rs
[composition-minesweeper]: ../../crates/samples/composition/minesweeper

Start with `standalone` for the system stack or `host` for Reactor. The other APIs - implicit
animations, scoped batches, nine-grid brushes, and Canvas surfaces - are useful once the basic
visual tree feels familiar.

`CompositionScopedBatch::on_completed` returns an event revoker that must remain alive until the
tracked work finishes. In a Reactor component, have the callback send a message and update the
retained scene from `Component::update`.

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
property and factory conveniences fail fast on COM errors. `CompositionColor` wraps the flat
`Windows.UI.Color` ABI value.

| Module | Contents |
| --- | --- |
| `compositor.rs` | Factories for graph objects and stack-specific hosts |
| `visual.rs` | Visual hierarchy, properties, and child collections |
| `brush.rs` | Brush hierarchy and implementations |
| `shape.rs` | Shape visuals, geometry, shapes, and collections |
| `animation.rs` | Key frames, easing, groups, and implicit animations |
| `batch.rs` | Scoped batches and batch kinds |
| `surface.rs` | System and lifted graphics devices and drawing surfaces |
| `stack.rs`, `target.rs` | System dispatcher queue and HWND hosting |
| `color.rs` | Public color value |

### Code generation

Composition uses `tool_composition` because it needs system and non-default lifted metadata. The
tool first uses `windows-rdl` to generate the lifted native interop metadata from `interop.rdl`,
then runs `windows_bindgen` twice with `--flat --minimal --dead-code`.

- System bindings read the repository's Windows and Win32 metadata and include
  `Windows.UI.Composition`, dispatcher queue, desktop interop, and surface interop APIs.
- Lifted bindings read the pinned Microsoft UI and Graphics metadata used by Reactor, the
  repository's Windows and Win32 metadata, and the generated interop metadata. They include
  `Microsoft.UI.Composition` with shared foundation, color, DirectX, and native interop types.

`crates/tools/composition/src/composition.txt` is the shared filter. System-only regions are omitted
from lifted generation. Composition, DirectX, and native interop entries are rewritten for the
lifted stack. `lib.rs` selects `bindings.rs` or `bindings_lifted.rs`.

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
capability. `Visual::host_visual` exports only an application-owned lifted visual for attachment.

Both crates generate lifted bindings from the same pinned Microsoft UI metadata. Host attachment
and observation commands exist in Reactor's native and recording runtimes. Window and node
identities reject late events and completions.

Reactor also consumes lifted key-frame, group, easing, and implicit-animation wrappers for element
lifecycle transitions.

### Canvas bridge

Each stack exposes matching compositor, graphics-device, and drawing-surface interop interfaces. A
Canvas device becomes a `CompositionGraphicsDevice`, which creates a premultiplied BGRA drawing
surface. A composition surface brush paints that surface onto a visual.

`windows-canvas` implements the drawing half behind its `composition` feature. It borrows the
Direct2D context returned by `begin_draw`, applies the backing-atlas offset, and pairs the operation
with `end_draw`.

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
