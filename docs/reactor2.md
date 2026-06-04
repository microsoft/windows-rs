# Reactor Backend Redesign — Typed Control Handlers

## Status: 20 controls migrated ✅

## Problem

Every property on every frame went through three indirection layers:

1. `bindings()` allocates `Vec<Binding>` of `Prop`/`PropValue` pairs
2. Reconciler scans old/new Vecs to find changes
3. `set_prop()` matches `(Prop, PropValue, Handle)` triples

Goal: `w.font_size = Some(14.0)` → `h.put_FontSize(14.0)`. One hop.

## Architecture

```
Reconciler → mount_props(id, widget.as_any()) → returns true if handled
  ├─ true  → if has_events(): apply_events(bindings)
  └─ false → apply_props(bindings)  [legacy path]

Reconciler → diff_props(id, old.as_any(), new.as_any()) → returns true if handled
  ├─ true  → if has_events(): diff_events(old_bindings, new_bindings)
  └─ false → diff_props(old_bindings, new_bindings)  [legacy path]
```

The `widget_map!` macro generates `try_mount`/`try_diff` functions that
downcast `&dyn Any` to concrete widget types. Each widget provides
explicit mount/diff blocks with direct COM calls. A `changed!` helper
macro guards diff blocks with `if old.field != new.field`.

### Event split

- **Prop-only controls**: `has_events() = false`, `bindings()` returns `Vec::new()`.
  Zero allocation on every frame.
- **Event controls**: `has_events() = true`, `bindings()` returns only Event entries.
  Props handled via typed path, events via lightweight `apply_events`/`diff_events`.

## Controls Migrated (20)

**Props-only (14):** TextBlock, StackPanel, ProgressBar, ProgressRing,
Border, Image, PersonPicture, InfoBadge, Viewbox, ScrollViewer, Grid,
ScrollView, Canvas†, RelativePanel†

**Props+Events (6):** TextBox, CheckBox, ToggleSwitch, Slider, NumberBox,
RadioButton, HyperlinkButton, PasswordBox

†Canvas/RelativePanel have empty bindings — no typed entry needed.

## LOC Impact

| File | Before | After | Delta |
|------|--------|-------|-------|
| mod.rs | 4304 | 3902 | -402 |
| widget_maps.rs | — | 707 | +707 |
| Widget files | — | — | -531 |
| Plumbing (trait/reconciler) | — | — | +69 |
| **Net** | | | **-157** |

## What Stays in set_prop

**Cross-cutting** (~40 Prop variants): layout, font, brush, attached props,
shape properties. These go through `modifiers()`/`attached()` and already
bypass `bindings()`.

**Complex controls** (~8 remaining): Button (flyouts, menu items),
NavigationView (complex menus/search), TabView, ContentDialog, ComboBox,
Expander, InfoBar, RichEditBox. These have complex state management or
need backend `self` access.

## Remaining Work

1. Migrate remaining complex controls where feasible
2. Continue removing dead set_prop arms
3. Eventually set_prop shrinks to only cross-cutting concerns

## Validation

```
cargo fmt -p windows-reactor
cargo clippy -p windows-reactor --all-targets
cargo run -p test_reactor_selftest    # "Total failures: 0"
cargo run -p test_reactor_perf -- --headless --duration 5
```
