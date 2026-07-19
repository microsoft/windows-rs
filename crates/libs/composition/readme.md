## Windows Composition

Windows Composition is a safe, fast Rust wrapper around the Windows composition engine (`Windows.UI.Composition`) — the retained-mode visual layer that draws and animates content off the UI thread. Unlike the lifted `Microsoft.UI.Composition` (which requires a WinUI 3 host), this system stack can host a visual tree directly in any window, with no Windows App SDK dependency. All COM interfaces are private implementation details — you work only with Rust types.

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

Core types: `Compositor`, `Visual`, `ContainerVisual`, `SpriteVisual`, `ShapeVisual`, composition brushes and shapes, and `Vector3` key-frame animations. To show a tree, create a `DispatcherQueueController` on the thread, create a `Compositor`, then host the root visual in a window with `Compositor::create_desktop_window_target`, which takes a [`windows-window`](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-window.md) `Window`. See the [composition guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-composition.md) for how this layer fits alongside reactor and canvas.
