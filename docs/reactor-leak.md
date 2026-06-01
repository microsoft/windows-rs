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

## Remaining: WinUI framework retention

The visible memory growth reported in #4491 is primarily caused by the WinUI
XAML framework retaining internal allocations for destroyed controls. Possible
mitigations (all require further investigation):

1. **Element pooling/recycling** — reuse XAML control objects across page
   navigations instead of creating fresh ones. This is the most promising
   approach but requires significant reconciler changes.

2. **Reduce control churn** — restructure page components to share more
   persistent structure (e.g., keep a shared StackPanel and only swap inner
   content).

3. **WinUI framework-level fixes** — investigate whether `XamlShutdownCompletedOnThread`
   or `DebugSettings.EnableFrameRateCounter` APIs can trigger internal cleanup,
   or whether this is a known WinUI issue with an upstream fix.
