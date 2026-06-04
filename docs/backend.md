# WinUI Backend Architecture — Assessment & Next Steps

## Original Problems

1. **Type erasure** — Widget structs erased into `Vec<Binding>` by `bindings()`,
   then re-dispatched on `(Prop, PropValue, Handle)`.
2. **Unset fragility** — Forgetting a `PropValue::Unset` arm causes controls to get stuck.
3. **Allocation per render** — `bindings()` allocates a Vec every frame per widget.
4. **Scattered logic** — A single control's behavior spread across 300+ non-adjacent arms.
5. **Invalid states representable** — Nothing prevents type-mismatched triples.

## What Was Attempted: Typed Control Handlers

### Approach

Replace `set_prop(id, Prop, PropValue)` with per-control handler files
(`controls/text_block.rs`, etc.) that receive typed widget structs and
diff fields directly via `backend.diff_widget(id, old, new)`.

### What Was Built (45 ControlKinds → 43 handler files)

```
crates/libs/reactor/src/winui/backend/
├── mod.rs              (3608 lines, down from 4247)
├── controls/mod.rs     (dispatch registry)
└── controls/*.rs       (43 handlers, 2164 lines total)
```

### Honest Assessment

| Metric | Master | Branch | Verdict |
|--------|--------|--------|---------|
| Avg FPS (headless) | ~31 | ~31 | **No change** |
| Avg Reconcile | 5.3ms | 5.4ms | **No change** |
| mod.rs LOC | 4247 | 3608 | −639 |
| controls/ LOC | 0 | 2164 | +2164 |
| **Total backend LOC** | **4247** | **5772** | **+36% worse** |
| Memory | 183 MB | 183 MB | No change |

### Why It Didn't Work

**The typed handlers don't eliminate allocations.** `bindings()` is STILL called
every frame for every widget because events are embedded in the bindings Vec.
The typed handler skips prop dispatch but the Vec is allocated regardless:

```rust
fn diff_widget(&mut self, id, old, new) {
    let handled = typed_diff!(...);  // runs typed prop handler
    let old_b = old.bindings();  // ← STILL ALLOCATES (needed for events)
    let new_b = new.bindings();  // ← STILL ALLOCATES
    // ... event diffing uses these Vecs
}
```

So the architecture has **two dispatch paths for props** (typed handlers + bindings)
and **one dispatch path for events** (bindings only). The typed handlers are pure
overhead — they duplicate logic without removing the old path.

**Correctness gain is real but modest.** Typed handlers make Unset bugs structurally
impossible for migrated controls. But this was achievable with simpler approaches
(ClearValue consolidation removed the worst cases at −64 lines, not +1500).

**Readability is arguable.** 43 separate files vs one monolithic match. Individual
files are readable, but finding a control now requires checking two places (handler
file + set_prop residual for modifiers/events).

---

## C# Reference Architecture

The C# backend achieves all goals because it was designed from scratch with:

```csharp
public static readonly ControlDescriptor<ButtonElement, Button> Descriptor =
    new ControlDescriptor<ButtonElement, Button>()
        .OneWayConditional(get: e => e.Label, set: (c, v) => c.Content = v, ...)
        .HandCodedEvent<ButtonEventPayload, RoutedEventHandler>(
            subscribe: (c, h) => c.Click += h, ...);
```

Key differences from our Rust attempt:
- **Events are part of the descriptor** — no separate `bindings()` allocation.
- **No intermediate enum types** — no `Prop`, `PropValue`, `Event`, `EventHandler`.
- **Update() is zero-alloc** — descriptors compare old.Field vs new.Field directly.
- **shouldWrite predicate** handles unset — no explicit ClearValue logic per-prop.
- **~30 lines per control** — complete self-contained behavior.

---

## Root Cause Analysis

The Rust backend's complexity comes from **enum-based type erasure**:

```
Widget struct → bindings() → Vec<Binding::Prop(Prop, PropValue)>
                           → Vec<Binding::Event(Event, EventHandler)>
                                         ↓
            match (prop, value, handle) → ~300 arms
```

The typed handler approach tried to bypass the first half (props) while
keeping the second half (events). This half-measure:
- Added code without removing code
- Didn't eliminate allocations
- Created a dual-path system that's harder to reason about

---

## What Would Actually Work

### Option A: Complete the migration + add `fn events()` to Widget

Split `bindings()` into `fn prop_bindings()` (only for unhandled controls) and
`fn event_bindings()` (only events). For typed-handled controls, skip `bindings()`
entirely and use a new `fn events(&self) -> &[(Event, Option<EventHandler>)]`
that returns a **borrowed slice** (zero-alloc).

**Effort:** Medium. Requires changing Widget trait + all 50 widget impls.
**Gain:** Eliminates Vec allocation for typed-handled controls. ~10-15% reconcile speedup.
**LOC:** Still +1500 lines total (handlers remain).

### Option B: Descriptor-based architecture (C# port)

Replace the entire Widget trait with a descriptor system:

```rust
struct PropDescriptor<W, T> {
    get: fn(&W) -> &T,
    set: fn(&T, &Handle) -> Result<()>,
    clear: fn(&Handle) -> Result<()>,
}

struct EventDescriptor<W> {
    get: fn(&W) -> Option<&EventHandler>,
    attach: fn(&mut WinUIBackend, ControlId, EventHandler),
    detach: fn(&mut WinUIBackend, ControlId),
}
```

Each control becomes a static array of descriptors. The reconciler calls
`descriptor.update(old_widget, new_widget, handle)` generically.

**Effort:** Large. Requires rethinking Widget/Reconciler/Backend entirely.
**Gain:** True zero-alloc, eliminates Prop/PropValue enums, ~30 lines per control.
**LOC:** Would reduce total backend from 5772 to ~2000.

### Option C: Keep typed handlers, just stop calling bindings() for handled controls

The simplest incremental fix: when `handled == true`, skip event bindings scan
entirely (events would need to be wired during mount and left alone, or use
a separate lightweight event-comparison path).

Most events are **stable** (set once, never changed). We could track "event
attached" per-control and skip re-diffing.

**Effort:** Small-medium. Add event-stable optimization.
**Gain:** Eliminates most bindings() calls for interactive controls.
**LOC:** −50 (remove event scanning for handled controls).

### Recommendation

**Option C first** (quick win), then **Option B** if we're doing a major version.

The current branch is a halfway point that's worse than either extreme:
- Worse than master (more code, same perf, dual dispatch)
- Worse than a full descriptor rewrite (still has enum overhead)

---

## Remaining in set_prop (unmigrated)

These controls need `&self` access (event_revokers, handler maps):
- Button / DropDownButton (flyout construction + handler wiring)
- CommandBar (primary/secondary commands with click handlers)
- NavigationView (30+ arms, menu items, handler maps)
- TabView / TabViewItem (complex child/tab management)
- ContentDialog (show/hide lifecycle, XamlRoot)
- MenuBar (recursive menu building)

Plus cross-cutting modifier props (Foreground, FontSize, etc.) that
come through the reconciler's `diff_modifiers` path.

---

## Decision Point

Given the assessment above, the options are:

1. **Revert** — typed handlers add complexity without delivering perf gains
2. **Complete Option C** — stop calling `bindings()` for typed controls (small fix, real gain)
3. **Redesign with descriptors** — larger effort, delivers the C# architecture's benefits

