# windows-animation

> A safe wrapper around the Windows Animation Manager (`IUIAnimationManager`).

- 📦 [crates.io](https://crates.io/crates/windows-animation)
- 📖 [docs.rs](https://docs.rs/windows-animation)
- 🚀 [Getting started](../../crates/libs/animation/readme.md)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/animation)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/animation)

## When to use it

Use `windows-animation` when a Windows application needs time-based interpolation for values that
it will draw or apply itself. It works well with immediate-mode rendering: advance the manager on
each frame, read each `Variable`, and use the values in a canvas, Direct2D, Direct3D, or
DirectComposition scene.

The crate does not create a timer, request frames, draw content, or update a UI property. For
retained WinUI visuals in [`windows-reactor`](windows-reactor.md), Reactor's Composition
transitions usually fit better because the compositor owns their frame updates.

## Prerequisites

Initialize COM on the thread that creates the animation objects. For a console or rendering thread,
`windows_core::init_mta()` is suitable, as shown by the samples. A UI thread may use its existing
apartment. Use one monotonic time origin for scheduling and updates.

The README contains dependency setup and a minimal storyboard. The workflow below focuses on
integrating animation into a real frame loop.

## First workflow: drive a rendered property

1. Create one `Manager` and one `TransitionLibrary`.
2. Create a `Variable` for each scalar property, using the current rendered value as its initial
   value.
3. Build a transition and schedule it at an absolute time from the application's animation clock.
4. On every requested frame, sample that clock and call `Manager::update(time)` once.
5. Read all variables, draw or apply the values, and request another frame while animation remains
   active.

For example, the per-frame portion of a renderer can be kept separate from transition setup:

```rust,no_run
use windows_animation::{Manager, Result, Variable};

fn draw_frame(manager: &Manager, x: &Variable, now: f64) -> Result<f64> {
    manager.update(now)?;
    let x = x.value()?;
    // Use `x` in the renderer for this frame.
    Ok(x)
}
```

`now` is an absolute timestamp in the same time domain used when scheduling, not elapsed time since
the previous frame. Supplying deltas makes the animation clock stall or move incorrectly.

## Core API and lifecycle

`Manager` coordinates variables, transitions, and storyboards. Reuse it for values updated from
the same frame clock. `Variable` holds the current interpolated `f64`. `TransitionLibrary` creates
transition descriptions. `Transition` and `Storyboard` are retained COM-backed values and can be
cloned.

There are two scheduling paths:

- `Manager::schedule_transition` applies one transition to one variable at an absolute start time.
- `Manager::create_storyboard` creates a group that can coordinate several variables or sequence
  transitions before scheduling the group.

Keep the `Manager` and every `Variable` needed by rendering alive for the animation lifetime.
Propagate the `windows_core::Result` from creation, scheduling, updates, and value reads; COM
initialization and invalid transition parameters are reported through those results.

## Transition choices

| API | Use it for |
| --- | --- |
| `linear(duration, final_value)` | Constant-rate movement to a value. |
| `accelerate_decelerate(...)` | Ease-in and ease-out movement. |
| `instantaneous(final_value)` | A scheduled jump without interpolation. |

Acceleration and deceleration are ratios of total duration and must sum to <= 1.0. Durations and
final values use `f64`.

For direct scheduling:

```rust,no_run
use windows_animation::*;

fn retarget(manager: &Manager, library: &TransitionLibrary, x: &Variable, now: f64)
    -> Result<()>
{
    let transition = library.accelerate_decelerate(0.25, 320.0, 0.3, 0.3)?;
    manager.schedule_transition(x, &transition, now)
}
```

## Sequencing with storyboards

`Storyboard::add_transition` adds a transition and returns a `Keyframe` marking its end.
`add_transition_at_keyframe` starts another transition at that point. A keyframe can coordinate a
later transition on the same variable or another variable.

Build the entire storyboard before calling `schedule(start_time)`. Its time is absolute in the
manager's clock domain. The `storyboard` sample sequences a rise and fall on one variable; the
same pattern can start a fade after a movement finishes.

## Frame-loop and error pitfalls

- Update a manager once per frame before reading its variables. Reading first returns the value
  from the previous update.
- Use a monotonic high-resolution clock. Wall-clock changes must not move animation time backward.
- Schedule and update with the same time origin. `0.0` is valid for a deterministic offline
  timeline, while a live app commonly uses seconds since renderer startup.
- The crate does not report whether more frames are required. The application owns invalidation
  and its stop condition.
- Do not block the UI thread between updates in an interactive application; missed updates appear
  as skipped visual frames even though interpolation remains time-correct.
- Treat a failed creation, transition, schedule, update, or value read as an application error.
  Do not continue rendering with a stale value unless that is an explicit fallback.

## DirectComposition

`Variable::copy_curve` copies the variable's curve to an object implementing
`IDCompositionAnimation`. DirectComposition can then evaluate it on its own composition thread,
without the application reading `Variable::value` each frame.

This API targets Win32 DirectComposition, not the Windows.UI.Composition engine used by WinUI and
Reactor. The two animation object models are not interchangeable.

## Samples

Run the headless examples with:

```text
cargo run -p animation_samples --example variable
cargo run -p animation_samples --example storyboard
```

`variable` schedules one accelerate/decelerate transition and prints deterministic samples from
0.0 through 1.0 seconds. `storyboard` chains a rise and fall through a keyframe. Both initialize
COM explicitly and show the absolute-time update pattern.

The [`canvas/clock`](../../crates/samples/canvas/clock) sample applies the same API in a live
per-frame rendering loop.

---

## Internal documentation

This section is for contributors to `windows-animation`.

`src/bindings.rs` is generated by `tool_bindings` from
`crates/tools/bindings/src/animation.txt`. The hand-written `Manager`, `TransitionLibrary`,
`Storyboard`, `Transition`, `Keyframe`, and `Variable` wrappers expose the selected Windows
Animation Manager 2 interfaces.

`Manager::new` and `TransitionLibrary::new` create their COM classes with `CoCreateInstance`.
Manager methods delegate scheduling and updates to `IUIAnimationManager2`. Storyboards add an
explicit keyframe after each transition so the returned opaque `Keyframe` can anchor another
transition. `Variable::copy_curve` casts the supplied interface to `IDCompositionAnimation` before
copying the curve.

Keep the generated filter and the hand-written surface aligned. The headless examples are also the
clearest end-to-end checks of COM creation, direct scheduling, keyframe sequencing, and updates.
