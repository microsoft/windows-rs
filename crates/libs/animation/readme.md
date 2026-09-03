## windows-animation

Windows Animation wraps the Windows Animation Manager (`IUIAnimationManager`) for variable
interpolation, transitions, and storyboards. It does not depend on a UI or rendering framework.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-animation.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-animation]
version = "0.100"
```

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
