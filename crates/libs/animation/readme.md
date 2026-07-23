## Windows Animation

Windows Animation is a safe Rust wrapper around the Windows Animation Manager (`IUIAnimationManager`). It provides variable interpolation with multiple transition types - smoothly animated values and storyboard sequencing, independent of any UI or rendering framework.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Animation guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-animation.md)

```rust,no_run
use windows_animation::*;

fn main() -> Result<()> {
    let manager = Manager::new()?;
    let opacity = manager.create_variable(0.0)?;

    // Animate from 0.0 to 1.0 over half a second.
    let library = TransitionLibrary::new()?;
    let fade_in = library.linear(0.5, 1.0)?;

    let storyboard = manager.create_storyboard()?;
    storyboard.add_transition(&opacity, &fade_in)?;
    storyboard.schedule(0.0)?;

    // Each frame, advance the clock and read the current value.
    manager.update(0.25)?;
    let value = opacity.value()?;
    assert!((0.0..=1.0).contains(&value));
    Ok(())
}
```

Key types: `Manager`, `TransitionLibrary`, and `Variable`. See the [animation guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-animation.md) for how this layer fits alongside reactor and canvas animation.
