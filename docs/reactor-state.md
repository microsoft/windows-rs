# Reactor State Management

## Overview

`windows-reactor` provides a React-like hooks system for managing state within
components. This document compares the built-in hooks with raw `thread_local!`
and explains when each approach is appropriate.

---

## Available Hooks

| Hook | Returns | Triggers Rerender | Thread-Safe |
|------|---------|-------------------|-------------|
| `cx.use_state(initial)` | `(T, SetState<T>)` | Yes | No (UI thread only) |
| `cx.use_async_state(initial)` | `(T, AsyncSetState<T>)` | Yes | Yes (`Send + Sync`) |
| `cx.use_reducer(initial)` | `(T, Updater<T>)` | Yes | No |
| `cx.use_reducer_fn(reducer, initial)` | `(S, Dispatch<A>)` | Yes | No |
| `cx.use_ref(initial)` | `HookRef<T>` | No | No |
| `cx.use_memo(deps, factory)` | `T` | No (returns cached `T`) | No |
| `cx.use_effect(deps, closure)` | — | No (side effect only) | No |
| `cx.use_callback(deps, closure)` | `Callback<A>` | No (stable identity) | No |

### `use_state` — reactive value

```rust
let (count, set_count) = cx.use_state(0_u32);
// Reading `count` gives the current value.
// Calling `set_count.call(new_value)` schedules a rerender.
```

The value is owned by the component's hook vector; it persists across renders
and is automatically dropped when the component unmounts.

### `use_ref` — mutable cell without rerender

```rust
let frame = cx.use_ref(0_u64);
*frame.borrow_mut() += 1; // silent mutation, no rerender
```

`HookRef<T>` is an `Rc<RefCell<T>>` whose identity is stable across renders.
Use it for data that changes frequently but doesn't need to trigger UI updates
(e.g., frame counters, cached resources, accumulated measurements).

### `use_memo` — derived computation

```rust
let label = cx.use_memo((count,), || format!("{count} circles"));
```

Recomputes only when deps change. Avoids allocation on every render.

---

## `thread_local!` vs Reactor Hooks

### When hooks are better

| Concern | `thread_local!` | Hooks |
|---------|-----------------|-------|
| **Lifecycle** | Global; never dropped until thread exits | Scoped to component; cleaned up on unmount |
| **Reactivity** | Manual — must sync with `use_state` anyway | Automatic rerender on `SetState::call` |
| **Coupling** | Implicit — any code can read/write | Explicit — state is local to the component |
| **Composability** | One global instance per thread | Each component instance owns its own state |
| **Testability** | Leaks between tests unless manually reset | Fresh hooks per `RenderCx::for_test()` |

### When `thread_local!` is still appropriate

- **Framework internals** that need ambient access without a `&self` handle
  (e.g., `HOST_SLOT`, `APP_SLOT` inside reactor itself).
- **Heavy COM/GPU objects** that must be initialized once per thread and shared
  across unrelated subsystems (e.g., a D2D factory or DXGI device shared
  between multiple windows).
- **Interrupt-style callbacks** from OS APIs where you have no way to pass
  context and the closure signature is `fn()` (no captures).

In application-level code, hooks are almost always preferable.

---

## Case Study: `canvas_circles`

The original `canvas_circles` sample used `thread_local!` for two values:

```rust
thread_local! {
    static CIRCLE_COUNT: Cell<u32> = const { Cell::new(5) };
    static FRAME: Cell<u64> = const { Cell::new(0) };
}
```

### Why this is suboptimal

1. **Redundant state** — `CIRCLE_COUNT` duplicates the `use_state` value.
   The app function calls `cx.use_state(5_u32)` and then immediately copies
   the result into the thread_local. Two sources of truth for one value.

2. **Lifecycle mismatch** — The thread_local lives forever; the component may
   unmount (in a multi-page app) but stale values linger.

3. **No composability** — If two `canvas_circles` components existed in the
   same window, they'd collide on the same `CIRCLE_COUNT`.

### The fix

The frame counter becomes `cx.use_ref(0_u64)`. The circle count is shared
with the canvas closure via a second `use_ref` that is updated from `use_state`
on each render. The `animated_canvas` closure is captured once and persists
across renders (it is *not* recreated each frame), so it must read shared
mutable state through a ref — it cannot simply capture a by-value copy.

```rust
fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(5_u32);
    let frame = cx.use_ref(0_u64);
    let circle_count = cx.use_ref(5_u32);
    circle_count.set(count); // sync reactive state → ref each render

    let add = { let s = set_count.clone(); move || s.call(count + 1) };
    let remove = { let s = set_count; move || if count > 0 { s.call(count - 1) } };

    animated_canvas({
        move |ctx| {
            *frame.borrow_mut() += 1;
            let t = *frame.borrow() as f32 * 0.02;
            let count = *circle_count.borrow();
            // ... draw `count` circles ...
        }
    })
}
```

**Result:** Zero `thread_local!`, composable, testable, one source of truth.
The `use_ref` serves the same role as the old thread_local (shared mutable
cell) but is scoped to the component and cleaned up on unmount.

---

## Guidelines

1. **Default to `use_state`** for any value that affects the rendered UI.
2. **Use `use_ref`** for high-frequency mutation that doesn't need to trigger
   UI updates (frame counters, intermediate accumulators, cached GPU handles).
3. **Bridge `use_state` → `use_ref`** when a persistent closure (like
   `animated_canvas`) needs to observe state changes: keep `use_state` for
   reactivity and copy into a `use_ref` each render so the closure reads it.
4. **Use `use_memo`** for expensive derived values to avoid recomputation.
5. **Use `use_effect`** for one-time setup or reactions to state changes.
6. **Avoid `thread_local!`** in application components. Reserve it for
   framework plumbing where no `RenderCx` is available.
7. **Capture refs in closures** passed to canvas/rendering callbacks instead
   of bouncing values through globals.

---

## Comparison with the `direct2d` Sample

The `direct2d.rs` sample (raw D2D via `SwapChainPanel`) still uses
`thread_local!` for the `D2DState` struct. This is a legitimate case: the
`on_mounted` and `on_resize` callbacks are framework-provided closures that
don't carry a `RenderCx`, and the D2D device context is a heavy COM object
that outlives individual renders. In this scenario, `thread_local!` or
`use_ref` with careful lifetime management are both viable — but `use_ref`
is preferred when the component's lifetime matches the resource's lifetime.

The key distinction: **if you have access to `cx` at the point where state
is created and consumed, prefer hooks. If you're in a bare callback with no
`cx`, `thread_local!` may be your only option.**
