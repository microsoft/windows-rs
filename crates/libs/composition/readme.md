## Windows Composition

Windows Composition is a safe, fast Rust wrapper around the Windows composition engine - the
retained-mode visual layer that draws and animates content off the UI thread. A single wrapper
surface targets one of two composition stacks, selected at compile time by a Cargo feature: the
**system** `Windows.UI.Composition` (`system`, default), which hosts a visual tree directly in any
window with no Windows App SDK dependency; or the **lifted** `Microsoft.UI.Composition` (`lifted`),
hosted inside a WinUI 3 element through the
[`windows-reactor`](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)
bridge. All COM interfaces are private implementation details - you work only with Rust types.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Composition
  guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-composition.md)

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

Core types: `Compositor`, `Visual`, `ContainerVisual`, `SpriteVisual`, `ShapeVisual`, composition
brushes and shapes, and `Vector3` key-frame animations. To show a tree, create a
`DispatcherQueueController` on the thread, create a `Compositor`, then host the root visual in a
window with `Compositor::create_desktop_window_target`, which takes a
[`windows-window`](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-window.md)
`Window`. See the [composition
guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-composition.md) for
how this layer fits alongside reactor and canvas.
