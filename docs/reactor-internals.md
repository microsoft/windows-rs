# Reactor Internals

## Module-per-Element Property Dispatch

### Background

The WinUI backend (`crates/libs/reactor/src/winui/backend/mod.rs`) originally
contained a single monolithic `set_prop` method with 311 match arms dispatching
property set/unset operations across 54 element handle types. This led to a
class of bugs where `PropValue::Unset` handlers were silently missing — a
catch-all `(_, PropValue::Unset, _) => Ok(())` swallowed the missing cases.

An audit found **64 props** missing their unset handler (37 bool, 9 f64, 18
str), meaning properties like `IsTextSelectionEnabled` on TextBlock would be
set but never cleared when the component re-rendered without that prop.

### Architecture

```
crates/libs/reactor/src/winui/backend/
├── mod.rs               # 3,113 lines — universal props + orchestration
├── elements/
│   ├── mod.rs           # module registry
│   ├── macros.rs        # element_set_prop! macro (99 lines)
│   ├── text_block.rs    # TextBlock props (macro-generated)
│   ├── button.rs        # Button props (hand-written, complex)
│   ├── navigation_view.rs
│   └── ... (57 element modules total)
└── convert.rs           # shared conversion helpers
```

### Dispatch Flow

```rust
fn set_prop(&self, id: ControlId, prop: Prop, value: PropValue) {
    // Step 1: Element-specific dispatch (per-element module)
    let element_result = match handle {
        Handle::TextBlock(tb) => elements::text_block::set_prop(tb, prop, &value),
        Handle::Button(b)     => elements::button::set_prop(b, prop, &value),
        // ... all 55 elements
        _ => None,
    };
    if let Some(r) = element_result {
        return r;  // Element handled it — done.
    }

    // Step 2: Universal props (FontSize, Width, Margin, Opacity, etc.)
    match (prop, &value, handle) {
        (Prop::FontSize, PropValue::F64(v), h) => { ... }
        // ... 49 universal arms
    }
}
```

Element modules return `Option<windows_core::Result<()>>`:
- `Some(Ok(()))` — prop was handled successfully
- `Some(Err(e))` — prop was handled but the WinUI call failed
- `None` — not this element's prop, fall through to universal dispatch

### The `element_set_prop!` Macro

For simple props (bool, f64, str, u16, i32), the macro generates the full
`set_prop` function with exhaustive set+unset handling:

```rust
element_set_prop! {
    pub(in crate::winui::backend) fn set_prop(tb: &Xaml::TextBlock) {
        str  Text(v)                   => tb.put_Text(v),                    default: "";
        bool IsTextSelectionEnabled(v) => tb.put_IsTextSelectionEnabled(v),  default: false;
        bool TextWrappingWrap(v) => {
            let mode = if v { Xaml::TextWrapping::Wrap } else { Xaml::TextWrapping::NoWrap };
            tb.put_TextWrapping(mode)
        }, default: false;
    }
}
```

The macro **structurally guarantees** that every prop has both a set and unset
path — it's impossible to add a prop without specifying `default:`. This
eliminates the entire class of missing-unset bugs at compile time.

### Complex Props

Elements with props that don't fit the simple macro pattern (multi-step logic,
content manipulation, enum mapping) use hand-written `set_prop` functions with
explicit `match (prop, value)` arms. Examples:
- `button.rs` — ButtonIcon content panel manipulation
- `navigation_view.rs` — NavMenuItems tree building, PaneDisplayMode enum mapping
- `grid.rs` — GridRows/GridColumns iteration

### What Remains in `mod.rs`

1. **Universal props** (49 arms) — apply to any handle via trait casts
   (FontSize, Width, Height, Margin, Padding, Opacity, etc.)
2. **Backend-state-dependent props** — need `&self` access to WinUIBackend
   fields like `menu_click_handlers`, `command_bar_flyout_handlers`

### Performance

The two-step dispatch has **zero measurable overhead**:

| Metric | Before | After |
|--------|--------|-------|
| Avg FPS (4,900-cell stress) | 41.6 | 46.9 |
| Avg Reconcile | 3.3 ms | 3.3 ms |
| Avg Diff+Patch | 2.4 ms | 2.4 ms |
| Memory | 187.7 MB | 184.0 MB |

The per-element functions are small enough for the compiler to inline, and
each element only matches its own 2–10 props rather than scanning through
311 arms.

---

## Property Lifecycle Guarantees

### The Bug Class

When an optional prop transitions from `Some(value)` to `None` between
renders, the reconciler emits `PropValue::Unset`. The backend must reset the
XAML property to its default. Before the refactor, 64 props silently ignored
`Unset` via a catch-all, leaving stale values on controls.

### Prevention

The `element_set_prop!` macro prevents this structurally:
- Every `bool Foo(v) => setter, default: val;` generates both the set
  (`PropValue::Bool(v) => setter`) and unset (`PropValue::Unset => setter
  with v=default`) arms
- Adding a new prop without `default:` is a compile error

For hand-written modules, code review must verify that every `(Prop::X,
PropValue::SomeType(...))` arm has a corresponding `(Prop::X,
PropValue::Unset)` arm.

### Test Coverage

| Layer | What's Tested | File |
|-------|---------------|------|
| Reconciler (headless) | Unset emitted on prop removal | `tests/prop_binding.rs` |
| Reconciler (headless) | Unset regression suite | `tests/prop_unset_regression.rs` |
| WinUI live (selftest) | Mount all 53 elements | `selftest/fixtures/controls*.rs` |
| WinUI live (selftest) | Prop updates (8 controls) | `selftest/fixtures/prop_updates.rs` |
| WinUI live (selftest) | Interactions (8 controls) | `selftest/fixtures/interactions.rs` |
| Stress perf | 4,900 TextBlock updates/frame | `test_reactor_perf` |

---

## Future Work

- **`attach_event` migration** — same pattern for the 853-line event handler
  dispatch. Same potential for missing detachment handlers.
- **Universal prop grouping** — the 49 remaining arms could be organized by
  category (layout, typography, brush) if further decomposition is needed.
- **Backend-state extraction** — MenuBar/CommandBar/ContentDialog state could
  be moved to per-element state structs, enabling full migration.
