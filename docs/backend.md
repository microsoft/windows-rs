# WinUI Backend Architecture

## Original Problems

1. **Type erasure** — Widget structs erased into `Vec<Binding>` by `bindings()`,
   then re-dispatched on `(Prop, PropValue, Handle)`.
2. **Unset fragility** — Forgetting a `PropValue::Unset` arm causes controls to get stuck.
3. **Allocation per render** — `bindings()` allocates a Vec every frame per widget.
4. **Scattered logic** — A single control's behavior spread across 300+ non-adjacent arms.
5. **Invalid states representable** — Nothing prevents type-mismatched triples.

## Current Architecture (EventCtx + Typed Handlers)

### Design

Typed per-control handlers own **both props AND events**. `bindings()` is
never called for fully-handled controls — zero Vec allocation per frame.

```
Widget struct (CheckBox)
    │
    ▼  downcast via AsAny
controls::check_box::diff(old, new, handle, &mut EventCtx)
    │                                              │
    ├─ direct field comparison (props)             ├─ ctx.diff_event() queues ops
    │  if new.is_checked != old.is_checked         │
    │    → put_IsChecked(...)                      │
    │                                              │
    ▼                                              ▼
  Ok(())                               apply_event_ops(ctx)
                                           → attach_event / detach_event
```

### TypedResult Tiers

```rust
enum TypedResult {
    NotHandled,    // full legacy bindings() path
    PropsOnly,     // typed props, bindings() for events only
    FullyHandled,  // skip bindings() entirely — zero alloc
}
```

All 45 migrated controls are in `FullyHandled` tier.

### EventCtx (solves borrow conflict)

Handlers can't call `self.attach_event()` while holding `&Handle` (RefCell
conflict). Instead they push ops to an `EventCtx` queue:

```rust
pub struct EventCtx { id: ControlId, ops: Vec<EventOp> }

impl EventCtx {
    pub fn diff_event<T>(&mut self, old: &Option<Callback<T>>, new: &Option<Callback<T>>,
                         event: Event, wrap: fn(Callback<T>) -> EventHandler);
    pub fn mount_event<T>(&mut self, handler: &Option<Callback<T>>,
                          event: Event, wrap: fn(Callback<T>) -> EventHandler);
}
```

After the handler returns and the handle borrow is dropped, the backend
applies queued ops via `self.apply_event_ops(ctx)`.

### Results

| Metric | Master | Branch | Change |
|--------|--------|--------|--------|
| Avg FPS (headless) | ~42 | ~42 | ≈ same |
| Avg Reconcile | 5.3ms | 5.1ms | **−4%** |
| Avg Diff | 4.1ms | 3.9ms | **−5%** |
| mod.rs LOC | 4247 | 3748 | −499 |
| controls/ LOC | 0 | 2795 | +2795 |
| Total backend LOC | 4247 | 6543 | +54% |

### Assessment

**What works:**
- ✅ Zero `bindings()` allocation for 45 controls (problems 1 & 3 solved)
- ✅ Unset bugs structurally impossible in typed handlers (problem 2 solved)
- ✅ Each control self-contained in one file (problem 4 solved)
- ✅ Invalid states unrepresentable — typed fields, not enum triples (problem 5 solved)
- ✅ Events correctly wired via EventCtx (functional tests pass)
- ✅ Modest 5% diff improvement in perf test (test uses simple TextBlocks)

**What doesn't work yet:**
- ❌ Total LOC increased 54% (dual paths still exist)
- ❌ FPS improvement invisible in perf test (TextBlock-heavy, system load variance)
- ❌ Legacy `set_prop` match still 1422 lines (unmigrated controls + modifiers)
- ❌ `attach_event`/`detach_event` match still 1314 lines (needed by EventOps)

---

## What Remains

### Still in set_prop (need `&self` access for handler maps):
- Button / DropDownButton (flyout + menu items + handler wiring)
- CommandBar (primary/secondary commands with click handlers)
- NavigationView (30+ arms, menu items, handler maps)
- TabView / TabViewItem (complex child/tab management)
- ContentDialog (show/hide lifecycle, XamlRoot)
- MenuBar (recursive menu building)

### Cross-cutting modifier props (Foreground, FontSize, Margin, etc.)
These come through the reconciler's `diff_modifiers` path and apply via
`set_prop` regardless of whether the control has a typed handler.

---

## Path to LOC Reduction

The 54% LOC increase exists because we haven't removed anything yet.
The reduction path:

1. **Remove `bindings()` from migrated widgets** — once all controls in
   `FullyHandled`, the `fn bindings()` impls are dead code for the WinUI
   backend. They're still needed for MockBackend in tests, but could be
   feature-gated or auto-derived.
   - Potential savings: ~1500 lines across widget files (not counted in
     backend total, but reduces overall codebase)

2. **Remove dead `set_prop` arms** — arms for controls now handled by typed
   handlers are already removed. Once the remaining complex controls migrate
   (or get their own handler variant that receives `&mut self`), the entire
   `set_prop` match collapses.
   - Potential savings: ~1400 lines from mod.rs

3. **Consolidate attach_event** — the `attach_event` match dispatches
   `(Event, Handle)` → subscribe WinUI event. This could be split into
   per-control files alongside the handlers. Each handler would own its
   event subscription logic entirely.
   - Potential savings: ~1300 lines from mod.rs → distributed into handlers
     (net neutral LOC, but better organization)

4. **diff_prop! macro** for handler compression — most handlers follow the
   same pattern. A `diff_prop!(old, new, field, set_fn, clear_fn)` macro
   would compress handlers from ~50 lines to ~20 lines.
   - Potential savings: ~1000 lines from controls/

**Projected end state:**
| Component | Current | After cleanup |
|-----------|---------|---------------|
| mod.rs | 3748 | ~1500 (dispatch + infra) |
| controls/ | 2795 | ~1800 (after diff_prop!) |
| **Total** | **6543** | **~3300** |

---

## C# Reference (Goal State)

```csharp
public static readonly ControlDescriptor<ButtonElement, Button> Descriptor =
    new ControlDescriptor<ButtonElement, Button>()
        .OneWayConditional(get: e => e.Label, set: (c, v) => c.Content = v, ...)
        .HandCodedEvent<Payload, Handler>(subscribe: (c, h) => c.Click += h, ...);
```

Key differences remaining between our Rust approach and C#:
- C# uses a generic `ControlDescriptor<E, C>` — we use per-control fn files
- C# event subscription is declarative — ours is imperative (EventCtx queuing)
- C# has no intermediate Prop/PropValue/Event enums — we still have them for
  the unmigrated controls and the `attach_event` match

A full descriptor port would eliminate the enums entirely, but requires
rethinking the reconciler's Widget trait and MockBackend assertions.

---

## Architecture Decisions

1. **EventCtx queuing** — avoids RefCell borrow conflict between Handle
   borrow and mutable backend access. Ops are applied after handler returns.

2. **FullyHandled tier** — typed handlers that own both props and events
   skip `bindings()` entirely. No allocation, no linear scan.

3. **Two macro dispatch** — `full { ... }` for zero-alloc controls,
   `props { ... }` for transitional controls (currently empty).

4. **Modifiers stay separate** — width/height/margin/etc. are cross-cutting
   and handled by the reconciler's `diff_modifiers` path, not per-control.

