# windows-animation

Interpolate values with the Windows Animation Manager, then apply them in your own renderer.
The crate does not draw, request frames, or run a timer.

- 📦 [crates.io](https://crates.io/crates/windows-animation)
- 📖 [docs.rs](https://docs.rs/windows-animation)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/animation)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/animation)

## Animate a value

```toml
[dependencies]
windows-animation = "0.100"
windows-core = "0.100"
```

```rust,no_run
use windows_animation::*;

fn main() -> Result<()> {
    windows_core::init_mta()?;
    let manager = Manager::new()?;
    let opacity = manager.create_variable(0.0)?;
    let library = TransitionLibrary::new()?;
    let fade = library.linear(0.5, 1.0)?;

    let storyboard = manager.create_storyboard()?;
    storyboard.add_transition(&opacity, &fade)?;
    storyboard.schedule(0.0)?;

    manager.update(0.25)?;
    println!("{}", opacity.value()?);
    Ok(())
}
```

Use the existing apartment when a UI framework initializes COM for you.

## Connect a frame loop

Schedule and update with absolute seconds from the same monotonic clock, not frame-to-frame
deltas. On each frame, call `Manager::update` before reading variables and drawing.
The application decides when another frame is needed.

| Transition | Use |
| --- | --- |
| `linear` | Constant-rate movement |
| `accelerate_decelerate` | Ease-in and ease-out |
| `instantaneous` | A scheduled jump |

Acceleration and deceleration ratios must sum to <= 1.0. Use storyboard keyframes to coordinate
several variables or start a transition after another finishes. For one variable, use
`Manager::schedule_transition` directly.

The [Canvas clock](https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas/clock)
shows a live frame loop. For retained visuals animated by Windows, use
[windows-composition](https://docs.rs/windows-composition) instead.
