## Windows Animation

Windows Animation is a safe Rust wrapper around the Windows Animation Manager (`IUIAnimationManager`). It provides variable interpolation with multiple transition types — smoothly animated values and storyboard sequencing, independent of any UI or rendering framework.

> **Status:** experimental and pre-release (`0.0.0`). The API is evolving and the crate is not yet published to crates.io.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Animation guide](https://github.com/microsoft/windows-rs/blob/master/docs/guide/windows-animation.md)

```rust,ignore
use windows_animation::*;

let manager = Manager::new()?;
let library = TransitionLibrary::new()?;
let variable = manager.create_variable(0.0)?;

// Schedule a transition, then sample the variable over time.
let transition = library.accelerate_decelerate(0.5, 1.0, 0.2, 0.8)?;
manager.schedule_transition(&variable, &transition)?;

manager.update(elapsed_seconds)?;
let value = variable.value()?; // smoothly interpolates 0.0 -> 1.0
```

Key types: `Manager`, `TransitionLibrary`, and `Variable`. See the [animation guide](https://github.com/microsoft/windows-rs/blob/master/docs/guide/windows-animation.md) for how this layer fits alongside reactor and canvas animation.
