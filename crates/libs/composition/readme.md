# windows-composition

Retained visuals, shapes, brushes, and animations. Create a scene, update its properties, and let
Windows compose it without an application frame loop.

- 📦 [crates.io](https://crates.io/crates/windows-composition)
- 📖 [docs.rs](https://docs.rs/windows-composition)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/composition)

## Create a visual

```toml
[dependencies]
windows-composition = "0.101"
```

```rust,no_run
use windows_composition::*;

fn build(compositor: &Compositor) -> SpriteVisual {
    let visual = compositor.create_sprite_visual();
    visual.set_size(200.0, 120.0);
    visual.set_brush(&compositor.create_color_brush(CompositionColor::rgb(0, 120, 215)));
    visual
}
```

## Host the scene

| Feature | Host |
| --- | --- |
| `system` (default) | A `windows-window` window with a dispatcher queue and desktop target |
| `reactor` | A Reactor `Grid` observed through an effect |

The backends are mutually exclusive. Disable default features when selecting `reactor`, and do
not mix objects from the two backends.

Keep the system dispatcher queue and target alive while displaying a scene. In Reactor, retain the
host observation for the effect's lifetime. The [host sample][host] shows the complete lifecycle.
Objects in one tree must come from the same compositor.

Position and size visuals explicitly; Composition does not provide control layout.

Use [windows-reactor](https://docs.rs/windows-reactor) for controls and
[windows-canvas](https://docs.rs/windows-canvas) for immediate-mode drawing. Canvas surfaces require
its `composition` feature and the matching `system` or `reactor` feature.

See the [animation][animation] and [Minesweeper][minesweeper] samples.

[host]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition/host
[animation]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition/animation
[minesweeper]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition/minesweeper
