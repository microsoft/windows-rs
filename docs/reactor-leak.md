# Reactor Memory Leak Analysis

## Background

PR #4501 fixes a Rust-side leak where header/pane element subtrees were not
unmounted by the reconciler. Issue #4491 reports that memory still grows when
switching pages in the Gallery sample even after that fix.

## Investigation

Diagnostic counters were added to every per-control map in both the reconciler
and the WinUI backend, then the Gallery sample was run while switching pages
repeatedly.

### Findings

| Metric | Observation |
|--------|-------------|
| Reconciler maps (children_mirror, id_kinds, component_instances, etc.) | Stable — oscillate between 28–60 entries depending on current page |
| Backend maps (controls, event_revokers, parent_children, etc.) | Stable — exactly mirror reconciler counts |
| `next_id` (monotonic control counter) | 79 → 1,899 over ~42 page switches |
| Process working set | 116 MB → 291 MB (2.5×) over same session |

All Rust-side data structures are perfectly bounded.

### Root cause

The memory growth was caused by a **bug in the generated composable WinRT
constructors** in `windows-bindgen`.

Every composable class (Button, StackPanel, TextBox, Grid, Border, etc.) uses a
factory `CreateInstance` method with the signature:

```
CreateInstance(baseInterface, **innerInterface, **result)
```

The generated non-aggregating constructor passed `&mut core::ptr::null_mut()`
for `innerInterface`. This creates a **non-null** pointer to a stack slot. The
factory sees a non-null output pointer and writes an AddRef'd inner
(non-delegating) IUnknown into it. The stack slot is then discarded without
calling Release, **leaking one COM reference per construction**.

This leaked inner reference prevents the control from ever reaching ref count
zero, making every XAML control created via the Rust bindings effectively
immortal.

### C++ comparison

An identical C++/WinRT reproduction (create Buttons, add to panel, Clear) showed
zero memory growth because C++/WinRT passes `nullptr` for the inner output,
causing the factory to skip the output entirely.

## Fix

Change `&mut core::ptr::null_mut()` to `core::ptr::null_mut()` in the generated
composable constructor pattern. This passes a true null pointer for the inner
output parameter, matching C++/WinRT semantics.

The fix is in `crates/libs/bindgen/src/types/method.rs` and affects 72
generated constructors in the reactor's `bindings.rs`.

## Additional fixes applied (PR #4501)

### 1. Header/pane subtree leak (reconciler)

`Reconciler::unmount()` now recursively unmounts header and pane element subtrees
that are tracked outside `children_mirror`. Without this, switching away from a
NavigationView with a header element would leak the entire header subtree.

### 2. Backend map cleanup (WinUIBackend::destroy)

`destroy()` was missing cleanup of three per-control maps:

- `menu_click_handlers` — holds `EventHandler` closures with `Rc` captures
- `command_bar_flyout_handlers` — same
- `theme_brush_registry` — theme binding data per control

Since ControlIds are monotonically increasing (never reused), stale entries in
these maps accumulated forever. The fix adds `.remove(&id)` for each.

## Note on upstream WinUI issue

https://github.com/microsoft/microsoft-ui-xaml/issues/5978 documents a separate
WinUI framework memory retention issue. However, our investigation proved that
the memory growth observed in the Gallery sample was NOT caused by this upstream
issue — it was caused by the composable constructor leak described above. With
the fix applied, memory remains flat.
