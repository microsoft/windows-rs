## windows-composition

Windows Composition wraps the retained-mode Windows composition engine for standalone windows and
WinUI 3 applications.

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

    let brush = compositor.create_color_brush(Color::rgb(0, 120, 215));
    visual.set_brush(&brush);
    visual
}
```
