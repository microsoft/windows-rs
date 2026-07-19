## Windows Composition

Windows Composition is a safe, fast Rust wrapper around the WinUI 3 composition engine (`Microsoft.UI.Composition`) — the retained-mode visual layer that draws and animates content off the UI thread. All COM interfaces are private implementation details — you work only with Rust types.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Composition guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-composition.md)

```rust,no_run
use windows_composition::*;

fn build(compositor: &Compositor) -> Result<SpriteVisual> {
    let visual = compositor.create_sprite_visual()?;
    visual.set_size(200.0, 120.0)?;

    let brush = compositor.create_color_brush(Color::rgb(0, 120, 215))?;
    visual.set_brush(&brush)?;
    Ok(visual)
}
```

Core types: `Compositor`, `Visual`, `ContainerVisual`, `SpriteVisual`, and the composition brushes. The visuals compose into a tree hosted under a WinUI 3 element (via `windows-reactor`) or a standalone window. See the [composition guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-composition.md) for how this layer fits alongside reactor and canvas.
