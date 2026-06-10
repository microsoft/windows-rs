# Reactor: Rust vs C# Gap Analysis

Comparison of `windows-reactor` (Rust, this repo) against
[`microsoft-ui-reactor`](https://github.com/microsoft/microsoft-ui-reactor)
(C#, 0.1.0-preview.3, June 2026). Both share the same architectural DNA —
hooks-based components, virtual element trees, a reconciler driving WinUI
controls — but differ in scope and maturity.

This document tracks **legitimate gaps**: areas where the C# project has solved
real problems or shipped useful features that the Rust crate would benefit from.
It deliberately excludes C#-specific tooling (Roslyn analyzers, `mur` CLI,
NuGet packaging, VS/VS Code extensions, hot reload) and features that depend on
.NET runtime capabilities (AOT trimming annotations, `MetadataUpdateHandler`).

---

## Reconciler

| Concern | C# | Rust | Gap? |
|---------|-----|------|------|
| Positional child diff | ✅ O(n) | ✅ O(n) | No |
| Keyed diff (prefix/suffix/LIS) | ✅ 4-phase with LIS | ✅ Same algorithm | No |
| Skip-unchanged optimization | ✅ `CanSkipUpdate` + `ShallowEquals` | ✅ `can_skip_update` + `kind_matches` | No |
| State-dirty bypass | ✅ dirty flag on context | ✅ `is_component_state_dirty` | No |
| **Element pooling** | ✅ Recycled by CLR type (max 32) | ❌ None | **Yes** |
| **Render coalescing** | ✅ Batches multiple setState into single render | ✅ Dispatcher batching (tested) | No |
| **Rerender depth guard** | ✅ `MaxRerenderReentrancy = 50` | ❌ No recursion limit | **Yes** |
| Debug counters | ✅ diffed/skipped/created/modified | ✅ diffed/skipped/created (no modified) | Minor |

### Element Pooling

The C# reconciler pools unmounted WinUI controls (up to 32 per CLR type) and
reuses them on subsequent mounts. This avoids expensive COM object creation and
GC pressure on high-frequency list updates. The pool clears tags, event
handlers, accessibility props, flex props, and styles before reuse.

In Rust, every unmount destroys the COM control and every mount creates a new
one. For templated lists with frequent item churn, this is likely a measurable
cost. A pool keyed by `ControlKind` with pre-recycle cleanup would be the
direct analogue.

### Render Coalescing

Both frameworks coalesce multiple `UseState` writes into a single reconcile
pass. The Rust crate's `DispatcherQueue`-based batching is tested in
`setstate_spam_outside_render_coalesces_to_one_render` (100 rapid
`set_state` calls → 1 reconcile). This matches the C# behavior.

### Rerender Depth Guard

The C# reconciler caps re-entrant rerenders at 50 and panics. The Rust crate
has no such guard, so a render function that unconditionally calls `set_state`
would recurse until stack overflow. Adding a simple counter + panic is trivial
but important for debuggability.

---

## Hooks

| Hook | C# | Rust | Gap? |
|------|-----|------|------|
| UseState | ✅ | ✅ `use_state` | No |
| UseReducer | ✅ | ✅ `use_reducer` / `use_reducer_fn` | No |
| UseEffect (with cleanup) | ✅ | ✅ `use_effect` / `use_effect_with_cleanup` | No |
| UseMemo | ✅ | ✅ `use_memo` | No |
| UseCallback | ✅ | ✅ `use_callback` | No |
| UseRef | ✅ | ✅ `use_ref` | No |
| UseContext | ✅ | ✅ `use_context` | No |
| Async/thread-safe state | ✅ `threadSafe: true` | ✅ `use_async_state` | No |
| UseColorScheme | ✅ | ✅ `use_color_scheme` | No |
| **UseObservable** | ✅ Subscribes to `INotifyPropertyChanged` | ❌ | Minor |
| **UseCollection** | ✅ Subscribes to `INotifyCollectionChanged` | ❌ | Minor |
| **UseWindowSize / UseBreakpoint** | ✅ Responsive breakpoints | ⚠️ `use_inner_size` (no breakpoints) | Minor |
| **UseDpi** | ✅ | ✅ `use_dpi` | No |
| **UseClosingGuard** | ✅ Cancellable window close | ❌ | Minor |
| UseResource | ✅ (via composition) | ✅ `use_resource` | No |
| UseMutation | ✅ (via composition) | ✅ `use_mutation` | No |
| UsePersisted | ✅ State persisted across sessions | ❌ | Feature |

### Hook Order Violation Handling

The C# `RenderContext` throws `HookOrderException` when a component changes
hook order between renders, with a clear error message. The Rust `RenderCx`
panics on type mismatch (via `downcast_ref().unwrap()`) but the message is a
generic "hook order invariant violated" without naming the component or listing
expected vs actual hook types. Better diagnostics here would save debugging
time.

---

## Error Handling

| Concern | C# | Rust | Gap? |
|---------|-----|------|------|
| Error boundary element | ✅ | ✅ `error_boundary()` | No |
| **Panic payload extraction** | ✅ Detailed context | ⚠️ Basic `panic_message()` | Minor |
| **Factory-vs-constructor guard** | ✅ Throws with guidance if element wasn't created via factory | ❌ | Minor |
| **Post-shutdown setter protection** | ✅ Throws if background setter fires after dispatcher shutdown | ❌ | **Yes** |

### Post-Shutdown Setter Protection

The C# framework detects when a background `UseState` setter fires after the
dispatcher has been shut down and throws a clear error rather than silently
dropping the update or crashing. The Rust `AsyncSetState::call` dispatches via
`UiMarshaller` with no check for whether the dispatcher is still alive. If the
window has closed and the dispatcher is gone, this could panic inside the
marshaller or silently leak.

---

## Theming

| Concern | C# | Rust | Gap? |
|---------|-----|------|------|
| ThemeRef tokens | ✅ Full set with subtree overrides | ✅ `ThemeRef` enum (30+ tokens) | No |
| Color scheme tracking | ✅ | ✅ `current_color_scheme` / `set_current_color_scheme` | No |
| **Style caching** | ✅ `ConcurrentDictionary` keyed by type + sorted bindings | ❌ | **Yes** |
| **Theme bleed on recycled controls** | ✅ Fixed (clears synthesized Style on pool recycle) | N/A (no pooling) | Future |
| Per-control style overrides | ✅ | ❌ | Feature |

### Style Caching

The C# reconciler caches WinUI `Style` objects keyed by (target type, sorted
binding set). When multiple elements of the same type share the same theme
bindings, they reuse a single `Style` object. The Rust crate creates theme
resources per-element every time. Once element pooling is added, style caching
becomes important to avoid theme-binding bleed (a bug the C# project already
found and fixed).

---

## Performance Optimizations

The C# project has an extensive `tests/stress_perf/` suite with a 4,800-cell
stock grid benchmark. The Rust `test_reactor_perf` crate mirrors this workload
(now matched at 80×60=4,800 cells, identical RNG, same report format). Both
produce phase-breakdown timing (tree build / diff-patch / effects), element
stats (diffed/skipped/created), renders/sec, and summary CSV for
cross-comparison.

### Benchmark Data (same machine, x64, Release, AC power)

All numbers measured on the same machine, same architecture (x64), same OS,
same WinUI 3 runtime. Both use `--headless --duration 10` on an 80×60=4,800
cell stock grid with identical deterministic RNG.

#### C# WinUI Direct (imperative property set — no framework overhead)

| Mutation | Avg FPS | Total Renders | Avg Update | Memory |
|----------|---------|---------------|------------|--------|
| **10%** | 29.3 | 151 | 2.7 ms | 241 MB |
| **50%** | 7.6 | 38 | 13.7 ms | 246 MB |
| **100%** | 5.8 | 29 | 21.1 ms | 249 MB |

#### C# Reactor (declarative reconciliation)

| Mutation | Avg FPS | Total Renders | Reconcile | Tree | Diff | Memory |
|----------|---------|---------------|-----------|------|------|--------|
| **10%** | 19.1 | 86 | 57.7 ms | 26.0 ms | 31.7 ms | 296 MB |
| **50%** | 7.6 | 33 | 84.1 ms | 26.2 ms | 57.9 ms | 295 MB |
| **100%** | 6.6 | 29 | 84.9 ms | 28.2 ms | 56.7 ms | 297 MB |

#### Rust Reactor (declarative reconciliation)

| Mutation | Avg FPS | Total Renders | Reconcile | Tree | Diff | Diffed | Skipped | Created | Memory |
|----------|---------|---------------|-----------|------|------|--------|---------|---------|--------|
| **10%** | 56.0 | 240 | 3.5 ms | 0.9 ms | 2.5 ms | 463 | 4347 | 0 | 177 MB |
| **50%** | 14.7 | 84 | 8.0 ms | 0.8 ms | 7.3 ms | 1895 | 2915 | 0 | 184 MB |
| **100%** | 9.4 | 58 | 11.4 ms | 0.7 ms | 10.7 ms | 3041 | 1769 | 0 | 183 MB |

### Cross-Framework Comparison (apples-to-apples, same machine)

| Metric | C# Reactor | Rust Reactor | Ratio |
|--------|-----------|-------------|-------|
| Avg FPS @ 10% | 19.1 | 56.0 | **Rust 2.9×** |
| Avg FPS @ 100% | 6.6 | 9.4 | **Rust 1.4×** |
| Total Renders @ 10% (10s) | 86 | 240 | **Rust 2.8×** |
| Avg Reconcile @ 10% | 57.7 ms | 3.5 ms | **Rust 16× faster** |
| Tree build @ 10% | 26.0 ms | 0.9 ms | **Rust 29× faster** |
| Diff-patch @ 10% | 31.7 ms | 2.5 ms | **Rust 13× faster** |
| Memory | 296 MB | 177 MB | **Rust 40% less** |
| Elements Created (steady) | not reported | **0** | No churn at steady state |

Rust also beats C# WinUI Direct at 100% mutation (9.4 FPS vs 5.8 FPS) — the
skip-unchanged optimization avoids touching unchanged controls, while Direct's
synchronous COM property-set loop scales linearly with cell count.

### Key Insight: Elements Created = 0

The element stats reveal that **no new WinUI controls are created at steady
state** — the reconciler's skip-unchanged optimization plus kind-matching means
every render pass diffs and patches existing controls without creating or
destroying any. This means:

- **Element pooling would have zero effect** on this workload. Pooling only
  helps when controls are frequently created and destroyed (e.g., keyed list
  reordering with type changes). The steady-state stock grid never creates.
- **The diff-patch phase is the bottleneck.** At 100% mutation, diff takes
  11.9 ms of the 12.6 ms total reconcile. Each diffed element costs ~3.9 μs
  (11.9 ms / 3041 elements), which is primarily COM property-set overhead.

### C# Perf Investigation Findings (spec 034)

The detailed investigation in `docs/perf-investigations/reactor-vs-direct-10pct.md`
found:

- **GC pressure is the bottleneck, not reconcile time.** Reactor allocates
  ~22 MB/tick (vs 35 KB for Direct), causing ~5 gen0 + ~1.7 gen1 GCs per tick
  and ~6 gen2 (full-STW) collections per second.
- **User-side memoization** (caching previous `Element[]`, reusing refs for
  unchanged cells) produced the largest win: **+49% renders, 6× lower
  allocations, beats Direct by 31%.**
- **Direct record initializer** (bypassing fluent chain's 5 `with`-clones per
  cell) cut tree build time from 24 ms to 12 ms and allocations from 17 to
  8.6 MB/tick.
- **Bucketed ElementModifiers** (splitting Layout/Visual into sub-records):
  smaller win than expected — only +6% renders / −11% bytes — because the
  benchmark's hot pair (Padding + Foreground) straddles the bucket boundary.

### Implications for Rust

These findings significantly change the priority analysis:

| Optimization | C# | Rust | Real impact | Priority |
|-------------|-----|------|-------------|----------|
| Skip-unchanged | ✅ | ✅ | Core — both have it | Done |
| **Cell memoization** | ✅ `UseMemoCells` | ✅ Already built-in | Largest measured win (+49%). Rust perf test already caches cells and only rebuilds dirty indices — this is the *default* Rust approach. | **Already done** |
| **Element pooling** | ✅ 32/type | ❌ | Unclear benefit in Rust. C# pools to reduce GC pressure from COM object creation. Rust has no GC; COM creation cost is a fixed FFI call. Worth measuring but may not matter. | **Measure first** |
| **Render coalescing** | ✅ | ✅ | Rust already has dispatcher-based batching (tested: 100 setState → 1 render). | **Already done** |
| **Modifier bucketing** | ✅ −11% bytes | ❌ | C# saw only +6% renders. Rust `Modifiers` is stack-allocated, no GC pressure. The clone cost is trivially small vs C#'s record-with-copy. | **Skip** |
| **Rerender depth guard** | ✅ cap=50 | ❌ | Not a perf optimization — it's a correctness guard against infinite recursion. | **Add (trivial)** |

### What Rust Has That C# Doesn't

The Rust perf test already implements what C# calls "ReactorOptimized":
- Cells are cached in a `Vec<Element>` and only dirty indices are rebuilt
- No GC pressure (Rust has no garbage collector)
- Element structs are stack-allocated value types, not heap-allocated records
- The fluent builder chain produces a single owned struct, not 5 `with`-clones

The C# project's biggest performance lesson — **allocation pressure / GC is the
real bottleneck** — does not apply to Rust. The Rust reconciler's costs are
dominated by COM interop (property-set calls to WinUI controls) and the element
tree diff itself, both of which are fixed per-element costs.

---

## Control Coverage

Both frameworks wrap ~50-60 WinUI controls. The Rust crate covers 58 widget
types (listed in `core/widgets/`). Key coverage differences:

| Control | C# | Rust | Notes |
|---------|-----|------|-------|
| Most standard controls | ✅ | ✅ | Good parity |
| **ContentDialog** | ✅ Full lifecycle | ✅ | Parity |
| **WebView2** | ✅ With events | ❌ | Missing |
| **MediaPlayerElement** | ✅ With events | ❌ | Missing |
| **Frame** (navigation) | ✅ | ❌ | Missing (tied to navigation system) |
| ScrollView (modern) | ✅ | ❌ (only ScrollViewer) | Missing |

---

## Accessibility

| Concern | C# | Rust | Gap? |
|---------|-----|------|------|
| AutomationProperties (name, id, help text) | ✅ | ✅ `AccessibilityModifiers` | No |
| LiveSetting / HeadingLevel | ✅ | ✅ | No |
| **WCAG validation** | ✅ Compile-time + runtime linting | ❌ | Feature |
| **Full AutomationProperties surface** | ✅ ItemType, ItemStatus, LabeledBy, etc. | ⚠️ Partial (5 properties) | **Yes** |

The Rust `AccessibilityModifiers` exposes 5 properties (`automation_name`,
`automation_id`, `help_text`, `live_setting`, `heading_level`). The C# framework
exposes the full WinUI `AutomationProperties` surface. Properties like
`LabeledBy`, `ItemType`, `ItemStatus`, `IsRequiredForForm`, `Landmarks`, and
`FullDescription` are missing from Rust.

---

## Multi-Window and Shell Integration

| Feature | C# | Rust | Gap? |
|---------|-----|------|------|
| Single-window app | ✅ | ✅ | No |
| **Multi-window** | ✅ `ReactorWindow` / `OpenWindow` | ❌ | Feature |
| **Tray icons** | ✅ `ReactorTrayIcon` | ❌ | Feature |
| **Taskbar progress/overlay** | ✅ | ❌ | Feature |
| **Jump lists** | ✅ | ❌ | Feature |
| Window close guard | ✅ `UseClosingGuard` | ❌ | Feature |
| Window persistence | ✅ `IWindowPersistenceStore` | ❌ | Feature |

These are large features that may not be immediate priorities for the Rust crate,
but multi-window support in particular is a fundamental capability gap for any
desktop framework.

---

## Features Not Applicable to Rust Port

These C# features are excluded from the gap analysis because they either depend
on .NET runtime capabilities or are tooling concerns orthogonal to the framework
core:

- Hot reload (MetadataUpdateHandler, hook-order recovery, state migration)
- Roslyn analyzers and codefixes
- `mur` CLI and project templates
- NuGet packaging and versioning
- VS/VS Code extensions
- WinForms interop
- Docking (very large feature, C#-specific use cases)
- Charting/D3 (application-level, not framework)
- Flex layout/Yoga (could be a separate Rust crate)
- DevTools MCP server
- Localization source generator

---

## Priority Recommendations

### High Priority (correctness / robustness)

1. **Rerender depth guard** — trivial to add, prevents infinite recursion
2. **Post-shutdown setter protection** — `AsyncSetState` should check dispatcher
   liveness before dispatching
3. **Hook-order error messages** — name the component and expected/actual types

### Medium Priority (worth measuring)

4. **Element pooling** — the C# project pools up to 32 controls per type, but
   their motivation was GC pressure, which doesn't apply to Rust. Pooling may
   still help by avoiding COM `CreateInstance` calls. **Measure COM creation cost
   with the perf harness before investing** — if `Elements Created` is near zero
   at steady-state (because skip-unchanged avoids new mounts), pooling has no
   effect.
5. **Style caching** — deduplicate WinUI Style objects for theme bindings

### Lower Priority (feature gaps)

6. **Full accessibility surface** — add missing `AutomationProperties`
7. **Modern ScrollView** — wrap `Microsoft.UI.Xaml.Controls.ScrollView`
8. **Multi-window** — foundation for desktop app scenarios

### Not Recommended (based on C# data)

- **Modifier struct bucketing** — C# saw only +6% renders / −11% bytes, and the
  Rust struct is stack-allocated (no GC pressure). Not worth the complexity.

---

*Last updated: June 2026. Based on C# preview.3 (commit 234364d) and local
Rust crate (commit 1eccc01). All benchmarks measured on the same x64 machine.*
