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

All Rust-side data structures are perfectly bounded. No Rust-side leak exists
beyond the backend map issue fixed below.

### Root cause of observed memory growth

Each page switch destroys the old page's XAML controls (via COM `Release()`) and
creates fresh ones. Over 42 switches, ~1,820 XAML controls are created and
destroyed. The WinUI/XAML framework's internal allocator retains the backing
memory for these objects and does not return it to the OS. This is framework-
level memory fragmentation/retention, not a reference leak.

Explicitly calling `Children.Clear()` and `put_Content(None)` on elements before
destroying them was tested and did not reduce memory growth.

## Known upstream issue

This is a **known WinUI 3 bug** tracked at:
https://github.com/microsoft/microsoft-ui-xaml/issues/5978

Key details from that issue (open since September 2021, still active in 2025):
- Affects all WinUI 3 apps using NavigationView page navigation
- Memory grows on every `Navigate` call regardless of cache mode settings
- Confirmed to affect Microsoft's own apps (WinUI Gallery, PowerToys, Clock,
  Windows 11 Settings)
- `GC.Collect()` does not recover the memory (native heap, not managed)
- `Bindings.StopTracking()`, `IDisposable`, `NavigationCacheMode.Disabled`
  all fail to mitigate
- Community reports growth of 2-3 MB per navigation, matching our observations
- The WinUI team acknowledged the issue and shipped partial fixes in SDK 1.4
  but users confirm it persists through the latest SDK versions

## Minimal reproduction

A standalone reproduction is provided at:
`crates/samples/reactor/winui_leak_repro/`

This sample uses **no reactor reconciler** — it directly creates and destroys
XAML controls via raw WinUI APIs on a timer, proving the leak is in the WinUI
framework itself. Run with:

```
cargo run -p winui-leak-repro
```

Results on a typical machine:
- 100 simulated page navigations (21 controls each)
- Memory grows from ~34 MB to ~63 MB (steady ~0.3 MB per navigation)
- Growth is unbounded — never plateaus or shrinks
- The Gallery grows faster (~3-4 MB per navigation) due to NavigationView
  transitions and more complex control types

## Fixes applied (PR #4501)

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

## Workarounds and mitigation strategies

Since the WinUI framework retains memory for destroyed controls, the most
effective mitigation on our side is to **reduce the number of XAML controls
created and destroyed during navigation**:

### 1. Element pooling / recycling (recommended)

Instead of creating fresh XAML elements on every page switch, maintain a pool of
reusable controls. When a page is unmounted, return its controls to the pool
instead of destroying them. When mounting a new page, draw from the pool first.

This requires reconciler changes:
- `Backend::create()` checks the pool before allocating
- `Backend::destroy()` returns to pool instead of dropping
- Pool entries are keyed by `ControlKind`
- Properties must be reset to defaults when recycling

### 2. Structural sharing across pages

Restructure page components to share persistent containers. For example, keep a
single StackPanel as the page root and only swap inner content rather than
destroying and recreating the entire page tree.

### 3. Lazy content loading

Defer creation of off-screen content. Only create controls when they become
visible, reducing the total number of controls created during a session.

### 4. Report to WinUI team

The minimal reproduction (`winui-leak-repro`) demonstrates the issue without any
framework dependency beyond raw WinUI APIs. This can be shared with the WinUI
team as evidence that the issue persists and is not application-specific.
