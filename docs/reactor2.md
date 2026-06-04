# Reactor Backend Redesign

## Problem

Every property on every frame goes through three indirection layers:

1. `bindings()` allocates `Vec<Binding>` of `Prop`/`PropValue` pairs (1,336 lines)
2. Reconciler scans old/new Vecs to find changes
3. `set_prop()` matches `(Prop, PropValue, Handle)` triples (1,751 lines)

Goal: `w.font_size = Some(14.0)` → `h.put_FontSize(14.0)`. One hop.

## Approach: `widget_map!` macro

A `macro_rules!` macro in `widget_maps.rs` (backend-side) that generates typed
mount/diff dispatch. Each field is declared once with a **kind** that generates
both mount and diff code automatically.

### Field kinds

In all setter expressions, `v` = the field value, `h` = primary interface,
`handle` = raw Handle (for extra casts), `w` = new widget struct.

**`val`** — always set, `v` = `w.field`:
```
val content => h.put_Text(v.as_str());
// mount: { let v = &w.content; h.put_Text(v.as_str())?; }
// diff:  if old.content != w.content { let v = &w.content; h.put_Text(v.as_str())?; }
```

**`opt`** — `Option<T>`, guarded on mount, default on unset:
```
opt font_size [14.0] => h.put_FontSize(v);
// mount: if let Some(v) = w.font_size { h.put_FontSize(v)?; }
// diff:  if old.font_size != w.font_size { let v = w.font_size.unwrap_or(14.0); h.put_FontSize(v)?; }
```

**`flag`** — bool, skip false on mount:
```
flag wrap_text => h.put_TextWrapping(if v { Wrap } else { NoWrap });
// mount: if w.wrap_text { let v = w.wrap_text; h.put_TextWrapping(Wrap)?; }
// diff:  if old.wrap_text != w.wrap_text { let v = w.wrap_text; ... }
```

**`custom`** — freeform mount/diff for edge cases:
```
custom value {
    mount { h.put_Text(w.value.as_str())?; }
    diff {
        if h.get_Text().ok().as_deref() != Some(w.value.as_str()) {
            h.put_Text(w.value.as_str())?;
        }
    }
}
```

### Proof-of-concept: 3 controls (hard → easy)

**TextBox** (hardest — get-before-set, events, optionals):
```rust
widget_map! {
    widgets::TextBox as ITextBox [events] {
        custom value {
            mount { h.put_Text(w.value.as_str())?; }
            diff {
                if h.get_Text().ok().as_deref() != Some(w.value.as_str()) {
                    h.put_Text(w.value.as_str())?;
                }
            }
        }
        opt  placeholder  [""]   => h.put_PlaceholderText(v);
        custom header {
            mount { if let Some(ref s) = w.header { h.put_Header(&string_as_textblock(s)?)?; } }
            diff {
                match &w.header {
                    Some(s) => h.put_Header(&string_as_textblock(s)?)?,
                    None => h.put_Header(None)?,
                }
            }
        }
        flag accepts_return      => h.put_AcceptsReturn(v);
        flag text_wrapping_wrap  => h.put_TextWrapping(
            if v { Xaml::TextWrapping::Wrap } else { Xaml::TextWrapping::NoWrap }
        );
        flag is_enabled [true]   => handle.cast_inner::<Xaml::IControl>()?.put_IsEnabled(v);
    }
}
```

**CheckBox** (multi-interface, tri-state, events):
```rust
widget_map! {
    widgets::CheckBox [events] {
        val  is_checked => handle.cast_inner::<Xaml::IToggleButton>()?.put_IsChecked(Some(v));
        custom label {
            mount {
                if let Some(ref s) = w.label {
                    handle.cast_inner::<Xaml::IContentControl>()?
                        .put_Content(&string_as_textblock(s)?)?;
                }
            }
            diff {
                match &w.label {
                    Some(s) => handle.cast_inner::<Xaml::IContentControl>()?
                        .put_Content(&string_as_textblock(s)?)?,
                    None => handle.cast_inner::<Xaml::IContentControl>()?
                        .put_Content(None)?,
                }
            }
        }
        flag is_enabled [true] => handle.cast_inner::<Xaml::IControl>()?.put_IsEnabled(v);
    }
}
```

**TextBlock** (simple, prop-only, validates zero-alloc path):
```rust
widget_map! {
    widgets::TextBlock as ITextBlock {
        val  content                   => h.put_Text(v.as_str());
        opt  font_size          [14.0] => h.put_FontSize(v);
        opt  font_weight         [400] => h.put_FontWeight(WinFontWeight { Weight: v });
        flag wrap_text                 => h.put_TextWrapping(
            if v { Xaml::TextWrapping::Wrap } else { Xaml::TextWrapping::NoWrap }
        );
        flag is_text_selection_enabled => h.put_IsTextSelectionEnabled(v);
    }
}
```

### What the macro generates

Two dispatch functions in `widget_maps.rs`:

```rust
pub(super) fn try_mount(widget: &dyn Any, handle: &Handle) -> Option<R> {
    // For each widget_map! entry:
    if let Some(w) = widget.downcast_ref::<widgets::TextBlock>() {
        return Some((|| -> R {
            let h = handle.cast_inner::<Xaml::ITextBlock>()?;
            // generated mount code per field kind
            Ok(())
        })());
    }
    None
}

pub(super) fn try_diff(old_any: &dyn Any, new_any: &dyn Any, handle: &Handle) -> Option<R> {
    if let Some(w) = new_any.downcast_ref::<widgets::TextBlock>() {
        let old = old_any.downcast_ref::<widgets::TextBlock>().unwrap();
        return Some((|| -> R {
            let h = handle.cast_inner::<Xaml::ITextBlock>()?;
            // generated diff code with if old.field != w.field guards
            Ok(())
        })());
    }
    None
}
```

### Events

Event controls add `[events]` marker. The reconciler:
- Typed path handles props only
- `has_events() -> true` widgets still call `bindings()` but only events processed
- Controls without events: zero `bindings()` allocation

### What stays in set_prop

Cross-cutting (~40 Prop variants): layout, font, brush, attached, shape.
These go through `modifiers()`/`attached()`, already bypass `bindings()`.

Legacy (~10 controls): Button, NavigationView, TabView, ContentDialog, etc.
Need backend state (`self.controls`, handler maps).

## Proof-of-Concept Plan

**Goal: working macro + 3 controls in one session. No batch migration.**

| Step | What | Time |
|------|------|------|
| 0 | Plumbing: `as_any`, `has_events`, `mount_props`/`diff_props`, event helpers, dispatch | 15 min |
| 1 | Write `widget_map!` macro definition (~60-80 lines) | 30 min |
| 2 | **TextBox** — hardest control, validates edge cases | 30 min |
| 3 | **CheckBox** — multi-interface + events | 15 min |
| 4 | **TextBlock** — simple, validates zero-alloc | 10 min |
| 5 | Validate: selftest, perf, LOC delta | 10 min |

**Decision point after Step 5:**
If 3 controls work cleanly → proceed with bulk migration.
If macro can't handle edge cases → stop and reassess.

### Success criteria
- Net LOC negative after just 3 controls
- Zero selftest regressions
- Macro definition ≤ 80 lines
- Each widget entry ≤ 15 lines average

### Validation commands
```
cargo fmt -p windows-reactor
cargo clippy -p windows-reactor --all-targets
cargo run -p test_reactor_selftest    # "Total failures: 0"
cargo run -p test_reactor_perf -- --headless --duration 5
```
