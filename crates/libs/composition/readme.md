## windows-composition

Windows Composition wraps the retained-mode Windows composition engine for standalone windows and
WinUI 3 applications.

The default `system` feature hosts Composition in a `windows-window` window. Use the `reactor`
feature for a Composition host inside Reactor, with default features disabled. The stacks are
mutually exclusive. Both support Canvas drawing surfaces through `windows-canvas[composition]`;
select Canvas's matching `system` or `reactor` feature.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-composition.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-composition]
version = "0.100"
```

```rust,no_run
use windows_composition::*;

fn build(compositor: &Compositor) -> SpriteVisual {
    let visual = compositor.create_sprite_visual();
    visual.set_size(200.0, 120.0);

    let brush = compositor.create_color_brush(CompositionColor::rgb(0, 120, 215));
    visual.set_brush(&brush);
    visual
}
```
