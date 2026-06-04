# WinUI Backend Architecture

## Current State

The WinUI backend (`crates/libs/reactor/src/winui/backend/mod.rs`) handles prop
dispatch through a single 4000-line match statement:

```rust
match (prop, &value, handle) {
    (Prop::Text, PropValue::Str(s), Handle::TextBlock(tb)) => tb.put_Text(s),
    (Prop::IsEnabled, PropValue::Bool(v), Handle::Button(b)) => b.cast::<IControl>()?.put_IsEnabled(*v),
    // ... ~300 arms
}
```

### Problems

1. **Type erasure** — Each widget struct (e.g. `Button { label, is_enabled }`) is
   erased into `Vec<Binding::Prop(Prop, PropValue)>` by `bindings()`, then the
   backend re-dispatches on the triple `(Prop, PropValue, Handle)`.

2. **Unset fragility** — Every prop that can be "set" must also handle
   `PropValue::Unset`. Forgetting an Unset arm causes controls to get stuck.
   The previous catch-all `(_, PropValue::Unset, _) => Ok(())` hid this.

3. **Allocation per render** — `bindings()` allocates a Vec on every render frame
   for every widget, even unchanged ones.

4. **Scattered logic** — A single control's behavior is spread across many
   non-adjacent match arms (Button has ~12 arms scattered over 2000 lines).

5. **Invalid states representable** — Nothing prevents `(Prop::Text, PropValue::Bool, Handle::Slider)`
   from compiling. The match just falls through to a diagnostic.

### Recent Improvements (ClearValue)

We bound `IDependencyObject::ClearValue(DependencyProperty)` and base-class
property statics. This gives us:

- **Correct unset behavior** — ClearValue restores the XAML-declared default,
  eliminating hardcoded magic values (e.g. `put_FontSize(14.0)`).
- **Consolidated cross-control props** — IsEnabled went from 20 per-control
  arms to 2 generic arms via interface cast.
- **-64 lines**, **-18 match arms**, all tests pass, no perf regression.

But this is incremental polish on a fundamentally complex architecture.

---

## C# Descriptor Architecture (Reference)

The C# reactor backend uses typed descriptors that eliminate the match entirely:

```csharp
// ButtonDescriptor.cs — the ENTIRE backend for Button
public static readonly ControlDescriptor<ButtonElement, Button> Descriptor =
    new ControlDescriptor<ButtonElement, Button>()
        .OneWayConditional(
            get:         e => e.Label,
            set:         (c, v) => c.Content = v,
            shouldWrite: e => e.ContentElement is null)
        .OneWayConditional(
            get:         e => e.IsEnabled,
            set:         (c, v) => c.IsEnabled = v,
            shouldWrite: e => !e.IsDisabledFocusable)
        .HandCodedEvent<ButtonEventPayload, RoutedEventHandler>(
            subscribe: (c, h) => c.Click += h,
            ...);
```

Key design principles:
- **Typed element → typed control** — `ButtonElement` maps to `WinUI.Button`.
  No enums, no type erasure.
- **OneWayConditional** — `shouldWrite` predicate handles "unset" naturally.
  When the predicate flips true→false, the framework calls ClearValue.
- **Direct field comparison** — `Update()` compares `get(oldEl)` vs `get(newEl)`.
  No Vec allocation, no linear scan.
- **Self-contained per control** — each descriptor file is 20-60 lines with
  complete coverage of that control's props.

---

## Proposed Redesign: Typed Control Handlers

### Core Idea

Replace the type-erased `set_prop(id, Prop, PropValue)` with typed per-control
handlers that receive the full widget struct and diff fields directly.

### Architecture

```
┌─────────────┐     ┌───────────────┐     ┌──────────────────┐
│  Widget      │     │  Reconciler    │     │  Backend         │
│  (Button)    │────▶│  update()      │────▶│  diff_widget()   │
│  typed struct│     │  has old & new │     │  dispatch by kind│
└─────────────┘     └───────────────┘     └──────────────────┘
                                                    │
                                           ┌────────┴────────┐
                                           ▼                 ▼
                                    button::diff()    text_block::diff()
                                    typed handler     typed handler
```

### Backend Trait Change

```rust
pub trait Backend {
    fn create(&mut self, kind: ControlKind) -> ControlId;

    // NEW: typed widget diff — replaces set_prop for widget props
    fn mount_widget(&mut self, id: ControlId, widget: &dyn Widget);
    fn diff_widget(&mut self, id: ControlId, old: &dyn Widget, new: &dyn Widget);

    // KEEP: for modifiers and attached props (cross-cutting concerns)
    fn set_prop(&mut self, id: ControlId, prop: Prop, value: PropValue);

    // ... rest unchanged
}
```

### Reconciler Change (one line)

```rust
// Before:
fn update_widget(&mut self, old: &dyn Widget, new: &dyn Widget, id: ControlId) {
    self.diff_props(id, &old.bindings(), &new.bindings());  // allocates 2 Vecs, linear scan
    self.diff_modifiers(id, old.modifiers(), new.modifiers());
    ...
}

// After:
fn update_widget(&mut self, old: &dyn Widget, new: &dyn Widget, id: ControlId) {
    self.backend.diff_widget(id, old, new);  // typed dispatch, zero allocation
    self.diff_modifiers(id, old.modifiers(), new.modifiers());
    ...
}
```

### Widget Trait Addition

```rust
pub(crate) trait Widget: AsAny {
    fn kind(&self) -> ControlKind;
    fn key(&self) -> Option<&str>;
    fn modifiers(&self) -> &Modifiers;
    fn bindings(&self) -> PropBindings;  // keep for MockBackend/tests
    // ...
}

// Enable downcasting
trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}
impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any { self }
}
```

### Per-Control Handler (example: Button)

```rust
// winui/backend/controls/button.rs
use crate::core::widgets::Button;

pub fn mount(btn: &Button, handle: &Handle) -> Result<()> {
    let ctrl = handle.cast_inner::<Xaml::IContentControl>()?;
    ctrl.put_Content(&IReference::from(btn.label.as_str()))?;

    if !btn.is_enabled {
        handle.cast_inner::<Xaml::IControl>()?.put_IsEnabled(false)?;
    }
    if btn.style != ButtonStyle::Default {
        apply_button_style(handle, &btn.style)?;
    }
    // ... ~15 lines for all Button props
    Ok(())
}

pub fn diff(old: &Button, new: &Button, handle: &Handle) -> Result<()> {
    if new.label != old.label {
        let ctrl = handle.cast_inner::<Xaml::IContentControl>()?;
        ctrl.put_Content(&IReference::from(new.label.as_str()))?;
    }
    if new.is_enabled != old.is_enabled {
        if new.is_enabled {
            handle.clear_value(Xaml::Control::get_IsEnabledProperty()?)?;
        } else {
            handle.cast_inner::<Xaml::IControl>()?.put_IsEnabled(false)?;
        }
    }
    if new.style != old.style {
        if new.style == ButtonStyle::Default {
            clear_button_style(handle)?;
        } else {
            apply_button_style(handle, &new.style)?;
        }
    }
    // ... ~30 lines total
    Ok(())
}
```

### WinUI Backend Dispatch

```rust
fn diff_widget(&mut self, id: ControlId, old: &dyn Widget, new: &dyn Widget) {
    let handle = &self.handles[&id];
    let result = match new.kind() {
        ControlKind::Button => {
            let old = old.as_any().downcast_ref::<Button>().unwrap();
            let new = new.as_any().downcast_ref::<Button>().unwrap();
            controls::button::diff(old, new, handle)
        }
        ControlKind::TextBlock => {
            let old = old.as_any().downcast_ref::<TextBlock>().unwrap();
            let new = new.as_any().downcast_ref::<TextBlock>().unwrap();
            controls::text_block::diff(old, new, handle)
        }
        // ... one arm per control (~50 total, each 4 lines)
    };
    if let Err(e) = result {
        diag::backend_error(id, e);
    }
}
```

### Benefits

| Metric | Before (match) | After (typed handlers) |
|--------|---------------|----------------------|
| Backend LOC | ~4200 (one file) | ~2000 (50 files × ~40 lines) |
| Match arms | ~300 | 50 (kind dispatch only) |
| Allocs per render | 2 Vecs per widget | 0 |
| Unset correctness | Manual per-arm | Structural (field comparison) |
| Invalid states | Representable | Compile error |
| Adding new prop | Find correct position in 4K match | Add field + 2 lines in handler |

### Unset Handling

With typed handlers, "unset" becomes natural:

```rust
// Old field had a value, new doesn't → clear
if old.font_size.is_some() && new.font_size.is_none() {
    handle.clear_value(Control::get_FontSizeProperty()?)?;
}
// New field has a value → set it
if let Some(size) = new.font_size {
    if new.font_size != old.font_size {
        ctrl.put_FontSize(size)?;
    }
}
```

This pattern can be captured in a helper:

```rust
fn diff_opt<T: PartialEq>(
    old: &Option<T>,
    new: &Option<T>,
    set: impl FnOnce(&T) -> Result<()>,
    clear: impl FnOnce() -> Result<()>,
) -> Result<()> {
    match (old, new) {
        (_, Some(v)) if old.as_ref() != Some(v) => set(v),
        (Some(_), None) => clear(),
        _ => Ok(()),
    }
}

// Usage:
diff_opt(&old.font_size, &new.font_size,
    |v| ctrl.put_FontSize(*v),
    || handle.clear_value(Control::get_FontSizeProperty()?),
)?;
```

### Migration Path

1. Add `AsAny` to Widget trait (backward compatible)
2. Add `mount_widget` / `diff_widget` to Backend trait with default impls
   that delegate to `bindings()` + `set_prop` (backward compatible)
3. Implement typed handlers for 5 controls (TextBlock, Button, StackPanel,
   Border, CheckBox) as proof of concept
4. Verify perf improvement (expect measurable gain from zero-alloc diff)
5. Migrate remaining controls one by one
6. Once all migrated, remove `bindings()` from reconciler hot path
   (keep for MockBackend assertions in tests)

### Common Props via Trait

Cross-cutting props (IsEnabled, Font*, Layout, Margin, etc.) get a shared
trait impl to avoid repeating in every handler:

```rust
trait CommonProps {
    fn diff_common(&self, old: &Modifiers, new: &Modifiers, handle: &Handle) -> Result<()>;
}

// Or simply a free function:
fn diff_common_props(old: &Modifiers, new: &Modifiers, handle: &Handle) -> Result<()> {
    diff_opt(&old.width, &new.width,
        |v| handle.as_framework_element().cast::<IFrameworkElement>()?.put_Width(*v),
        || handle.clear_value(FrameworkElement::get_WidthProperty()?),
    )?;
    // ... all Modifier-level props
    Ok(())
}
```

This mirrors how modifiers already work — they're cross-cutting and handled
separately from per-control props.

---

## What We Keep from ClearValue Work

The ClearValue bindings and property statics remain valuable in the new design:
- Typed handlers still need ClearValue for "field went from Some to None"
- Property statics are the correct way to identify what to clear
- The consolidated interface-cast pattern (IControl, IDependencyObject) carries over

---

## Implementation Progress

### Phase 1: Typed handler proof of concept ✅

**Status:** Implemented and validated.

- `controls/text_block.rs` — 56 lines (mount + diff)
- `controls/stack_panel.rs` — 34 lines (mount + diff)
- `controls/border.rs` — 64 lines (mount + diff)
- Dispatch macros `typed_mount!` / `typed_diff!` — adding a control is one line each
- Reconciler now calls `backend.mount_widget()` / `backend.diff_widget()`
- Widget trait now requires `AsAny` for downcasting
- Backend trait has `mount_widget`/`diff_widget` with default fallback impls

**Results:**
- All 55 test suites pass (0 failures)
- Perf: **44.1 FPS / 5.0ms avg reconcile** (vs master 41.2 FPS / 5.5ms)
  - **+7% FPS, -9% reconcile time** from eliminating Vec allocations
- Clippy clean, fmt clean
- Zero regressions

**Key files changed:**
- `core/widget.rs` — added `AsAny` trait, Widget now requires it
- `core/backend.rs` — added `mount_widget`/`diff_widget` with default impls
- `core/reconciler/widget_dispatch.rs` — calls new backend methods
- `winui/backend/mod.rs` — overrides with typed dispatch for TextBlock
- `winui/backend/controls/mod.rs` — new module
- `winui/backend/controls/text_block.rs` — typed mount/diff handler

### Next: Migrate more controls

Priority order (by usage frequency in apps):
1. ~~TextBlock~~ ✅
2. Button (complex: events, flyouts, icons, styling)
3. StackPanel (simple: orientation only)
4. Border (background, corner radius, padding)
5. CheckBox, TextBox, Grid, ...

Each migration removes match arms from mod.rs and adds a self-contained handler file.

## Architecture Decisions

1. **Fallback-first approach** — controls that haven't been migrated use the
   old bindings-based path transparently. No big-bang rewrite needed.

2. **AsAny for downcasting** — simpler than enum dispatch or double dispatch.
   The kind() match ensures we downcast to the correct type.

3. **Events stay in set_prop for now** — typed handlers handle props only.
   Events are wired separately and can be migrated later.

4. **Modifiers stay separate** — width/height/margin/etc. are cross-cutting
   and handled by the reconciler's `diff_modifiers` path, not per-control.

