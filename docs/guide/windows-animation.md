# windows-animation

> A safe wrapper around the Windows Animation Manager (`IUIAnimationManager`).

- 📦 Not published (experimental, `0.0.0`)
- 🛠 [Internals](../internals/windows-animation.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/animation)

## Overview

`windows-animation` wraps the Win32 Windows Animation Manager COM APIs in safe
Rust types. A `Manager` owns animation `Variable`s, each a smoothly animated
`f64`. You describe motion with a `Transition` from the `TransitionLibrary`,
group transitions into a `Storyboard`, and `schedule` it. Driving the manager's
clock with `update` advances every variable.

> **Status**: experimental and unpublished.

## Example

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