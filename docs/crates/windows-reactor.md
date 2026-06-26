# windows-reactor

> A declarative, React-style UI library for Rust, backed by WinUI 3.

- 📦 Not published to crates.io
- 🚀 [Getting started](../../crates/libs/reactor/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)

`windows-reactor` lets you describe a WinUI 3 user interface as a function of
state. You write a render function that takes a `RenderCx` and returns an
`Element`; the reactor diffs the result against the live visual tree and applies
only the changes. State lives in hooks such as `cx.use_state`, and updating it
schedules a re-render.

## Getting started

A reactor app needs three things: the crate dependency, a render function, and a
`build.rs` that stages the Windows App SDK runtime via
[`windows-reactor-setup`](windows-reactor-setup.md).

`Cargo.toml`:

```toml
[dependencies]
windows-reactor = "..."

[build-dependencies]
windows-reactor-setup = "..."
```

`build.rs` — pick the helper that matches your deployment model:

```rust,ignore
fn main() {
    // For a self-contained app that carries its own runtime:
    windows_reactor_setup::as_self_contained();
    // Other options: as_framework_dependent(), as_example().
}
```

`src/main.rs` — a render function plus `App`:

```rust,ignore
use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);

    vstack((
        text_block(format!("Count: {count}")).font_size(28.0).bold(),
        button("+").on_click(move || set_count.call(count + 1)),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    bootstrap()?;
    App::new().title("Counter").render(app)
}
```

`bootstrap()` initializes the Windows App SDK runtime and must be called once at
startup. `App::new()` is then a builder — `title`, `inner_size`, `backdrop`
(e.g. `Backdrop::Mica`), `fullscreen`, and `presenter` are common. `render(app)`
takes your `Fn(&mut RenderCx) -> Element` and runs the message loop.

Reactor catches panics at the FFI boundaries it owns (render/event callbacks and
`ErrorBoundary`), converting them to errors so they never unwind across the WinUI
ABI. It deliberately does *not* install a global panic hook. For panics that
escape outside those boundaries, add `panic = "abort"` to your release profile so
the process terminates cleanly instead of unwinding into WinUI's C++ frames (which
is undefined behavior):

```toml
[profile.release]
panic = "abort"
```

Set `RUST_BACKTRACE=1` when you want a backtrace — reactor leaves that to you.

## State with hooks

Hooks are methods on `RenderCx`. They give a render function persistent state
without globals or `thread_local!`. The most common:

- **`use_state(initial)` → `(value, SetState)`** — a value plus a setter. Calling
  `set.call(new_value)` updates the slot and schedules a re-render.
- **`use_ref(initial)` → `HookRef`** — mutable storage that does *not* trigger a
  re-render; read with `.borrow()`, write with `.borrow_mut()` or `.set(v)`. Good
  for animation frame counters and cached resources.
- **`use_memo(deps, factory)`** — recompute a value only when `deps` change.
- **`use_effect(deps, f)`** / **`use_effect_with_cleanup`** — run side effects when
  `deps` change.
- **`use_reducer`** / **`use_reducer_fn`** — state driven by an update or
  action/reducer instead of a plain setter.
- **`use_resource(fetcher, deps)` → `Resource<T>`** — async data loading with
  loading/error states; a `Resource` converts straight into an `Element`.
- **`use_context(&context)`** — read a value provided higher in the tree.

```rust,ignore
fn counter(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);
    button(format!("Clicks: {count}"))
        .on_click(move || set_count.call(count + 1))
        .into()
}
```

The `apps/examples` and `minimal/examples` directories include focused samples for
each hook (`use_state`, `use_ref`, `use_memo`, `use_effect`, `use_reducer`,
`use_resource`, `use_callback`, `use_color_scheme`, …).

## Building the UI

Elements are built with plain builder functions, each returning a widget that
becomes an `Element` via `.into()`. Containers take a tuple of children:

- **Text:** `text_block(content)` with `.bold()`, `.semibold()`, `.font_size(..)`,
  `.wrap()`, `.selectable()`, and type-ramp helpers (`title`, `subtitle`, `body`,
  `caption`, …).
- **Buttons:** `button(content)` with `.on_click(..)`, `.accent()`, `.subtle()`,
  `.enabled(..)`, `.icon(..)`, `.flyout(..)`, `.menu_flyout(..)`.
- **Layout:** `vstack((..))` / `hstack((..))` with `.spacing(..)`; `grid((..))`
  with `.rows([..])` / `.columns([..])` (using `GridLength::STAR`,
  `GridLength::Auto`) and per-child `.grid_row(n)` / `.grid_column(n)`.

Roughly 60 WinUI controls are wrapped, including `check_box`, `combo_box`,
`slider`, `list_view`, `tree_view`, `navigation_view`, `tab_view`, `pivot`,
`text_box`, `number_box`, `color_picker`, `calendar_view`, `content_dialog`,
`info_bar`, `teaching_tip`, `command_bar`, and more — see the
[full catalog](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor/src/widgets).

Layout and appearance modifiers are available on any `Element` (the `ElementExt`
trait): `.margin(..)`, `.padding(..)`, `.width(..)` / `.height(..)`,
`.horizontal_alignment(..)` / `.vertical_alignment(..)` (with
`HorizontalAlignment` / `VerticalAlignment`), `.background(..)`,
`.foreground(..)`, `.opacity(..)`, and transition helpers such as
`.with_opacity_transition(..)`. Spacing values use `Thickness` (with
`Thickness::uniform(..)`).

## Handling events

Event handlers take closures. `button(..).on_click(move || …)` is the most
common; pointer and keyboard handlers live on `ElementExt`: `.on_tapped(..)`,
`.on_pointer_pressed(..)`, `.on_pointer_released(..)`, `.on_pointer_moved(..)`,
`.on_pointer_entered(..)`, `.on_pointer_exited(..)`, `.keyboard_accelerator(..)`.
A `SetState` or `Dispatch` can be passed directly
wherever a handler is expected (via `IntoCallback`).

### Handler identity

When a value-carrying event just forwards its argument to a setter, pass the
setter directly instead of wrapping it in a closure:

```rust,ignore
// Prefer this — shorter, and the handler keeps a stable identity:
text_box(text).on_text_changed(set_text)

// Avoid this — a fresh closure is allocated every render:
text_box(text).on_text_changed(move |value| set_text.call(value))
```

This is not just cosmetic. Setters from `use_state`/`use_reducer` are memoized
per hook slot, so passing one straight through hands the reconciler the *same*
handler identity each render. The diff can then skip the whole control. An inline
closure (`move |v| set.call(v)`) allocates a fresh identity every render, so the
control is always re-diffed and its WinUI event re-bound.

When a handler needs to compute a value or run extra logic — so a setter can't be
passed directly — wrap it in `cx.use_callback(deps, …)` to memoize it and recover
the same stable identity for hot paths. For the common case of a unit event
(`on_click`) that stores a fixed or pre-computed value, `SetState::setter(value)`
is shorthand for `move || set.call(value)`:
`button("Reset").on_click(set_count.setter(0))`.

## Graphics integration

For custom 2D drawing, host a [`windows-canvas`](windows-canvas.md) surface with
`animated_canvas(draw)` (enable the `reactor` feature on `windows-canvas`). It
returns a `SwapChainPanel` element that redraws every frame and recovers from
device loss automatically — see the `canvas` samples. For raw Direct3D, the
`swap_chain_panel` sample drives a `SwapChainPanel` with `on_rendering`.

## Web content integration

To host a browser, use [`windows-webview`](windows-webview.md)'s `webview(on_ready)`
(enable its `reactor` feature). It returns a `WebView2` element backed by the WinUI
XAML `WebView2` control and hands you a ready-to-drive `WebView` once the browser
initializes — see the `reactor/webview` sample. The `as_self_contained()` setup
carries the required `Microsoft.Web.WebView2.Core.dll` automatically. (How the
widget bridges the WinRT control to the COM `ICoreWebView2` is covered in
[`windows-webview`](windows-webview.md).)

## Samples

The [`crates/samples/reactor`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
tree is the best reference:

- **`samples`** — the smallest app plus an `examples/` folder with ~90 focused
  per-control and per-hook examples (`counter`, `calculator`, `navigation_view`,
  `list_view`, `content_dialog`, `color_picker`, and many more).
- **`apps`** — complete applications: `notepad`, `solitaire`, `minesweeper`,
  `tictactoe`, `dotsweeper`.
- **`gallery`** — a WinUI-gallery-style shell with navigation across many controls.
- **`direct2d`** / **`swap_chain_panel`** — hosting Direct2D / Direct3D content.
- **`webview`** — hosting a WebView2 browser via `windows-webview`'s `reactor` feature.
- **`framework_dependent`** / **`self_contained`** — the two deployment models,
  differing only in `build.rs`.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-reactor`**.

### How it's built

The hooks runtime, element tree, reconciler, and WinUI backend are hand-written.
The per-widget dispatch is generated by `tool_reactor` from
`crates/tools/reactor/src/winui.toml` plus the WinUI `.winmd` metadata:

| Generated file | Contents |
|----------------|----------|
| `src/generated.rs` | per-widget `bindings()` helpers |
| `src/backend/winui/generated_set_prop.rs` | property setter dispatch |
| `src/backend/winui/generated_attach_event.rs` | event handler dispatch |
| `crates/tools/reactor/src/generated.txt` | binding filter entries |

The tool is metadata-driven: setter pattern, value type, and event-invoke
pattern are all inferred from `.winmd`. TOML keys are WinUI metadata names, and
only non-standard mappings need overrides. Regenerate with
`cargo run -p tool_reactor`, then verify with `cargo check -p windows-reactor`.

Generated dispatch falls through to hand-written code in the backend for cases
too complex to express declaratively (Button icon+text layout, NavigationView
menu items, ContentDialog modal popup, and similar). Never edit the generated
files or `generated.txt` by hand.

### Bindings

`src/bindings.rs` is generated by `windows-bindgen` (`cargo run -p tool_bindings`)
from `crates/tools/bindings/src/reactor.txt` using `--minimal` mode — list
exactly the methods you need. Raw metadata names apply: `get_Prop`, `put_Prop`,
`add_Event`, `remove_Event`.

To prune or extend bindings: edit `reactor.txt`, regenerate, and let the compiler
errors reveal the methods you still need (`SetX` → `put_X`, `X()` → `get_X`); add
them as `Ns.IFace::{put_X, get_Y}`. This also covers the Win32 COM interfaces
(DXGI, D2D, DWrite) — listed methods get full vtable entries, unlisted methods
become `usize` slots, and the type closure is computed automatically.

### COM pitfalls

These bite anyone editing the backend by hand. The generated code already
follows them.

- **Classes Deref to their default interface.** Don't `cast` to it — call the
  method directly (`button.SetFlyout(&flyout)`, not
  `button.cast::<IButton>()?.SetFlyout(...)`). This applies to **event-handler
  `sender`/`args`** too: the delegate hands you the concrete arg class and the
  `sender` is the control, both of which already `Deref` to their default
  interface — so `args.SelectedItem()` and a control captured at attach
  (`let control = h.clone();`) read at **zero** per-event QI, versus
  `args.cast::<I…Args>()` / `sender.cast::<TextBox>()` on every fire. Only cast to
  **non-default parent** interfaces (e.g. `Button` → `IContentControl`/`IControl`).
  Watch the *static* type, not the name: `DropDownButton.cast::<IButton>()` looks
  redundant but `IButton` is a parent there (the default is `IDropDownButton`), so
  it is a genuine cast.
- **`Param<T>` eliminates parent-class casts.** A method taking
  `impl Param<Brush>` accepts a `SolidColorBrush` directly — no `cast::<Brush>()`.
- **Use `From`/`into()`, not `cast`, for `IInspectable`,** and plain `None` for
  optional inspectable parameters.
- **`put_IsChecked` (CheckBox) takes `Option<bool>`** — it is a tri-state
  nullable boolean.
- **TextBox/PasswordBox need get-before-set** to avoid resetting the caret.
- **ProgressBar uses `IRangeBase`** for Value/Min/Max; ProgressRing has direct
  setters.
- **ContentDialog needs a `XamlRoot`** from a live element, so it requires
  backend access.
- **Font properties are shared** across `IControl`/`ITextBlock`/`IRichTextBlock`.

### Known quirks

`Padding` has no single owning interface: `Control`, `Border`, `StackPanel`,
`TextBlock`, and `RichTextBlock` each declare their own. `set_padding`
(`backend/winui/mod.rs`) therefore dispatches on the `Handle` variant — calling
the setter directly on `Border`, `StackPanel`, `TextBlock`, and `RichTextBlock`
through their default interface, and falling back to a single `IControl` cast for
everything else — so `.padding(...)` works on controls, borders, stack panels,
and text blocks. Containers that genuinely lack a `Padding` property (e.g. bare
`Panel`/`Grid`) still fall through to `diag::unhandled_modifier`, which warns
under debug builds; use `.margin(...)` there instead.

### Threading

Reactor runs on a WinUI STA thread and keeps per-thread state in `thread_local!`
slots. Two categories exist, and the distinction matters when refactoring:

- **STA-affine COM handles and caches** (the host, application, root window and
  framework element, and the shared `DataTemplate`) must stay thread-local — they
  hold COM objects that are only valid on the UI thread.
- **One-shot latches and per-thread scalars** (pending theme/title-bar requests,
  current color scheme) are thread-local only because the public API exposes them
  as free functions (`set_requested_theme`, etc.). They could move onto the host
  struct if those functions took a host reference.

### Performance notes

The reconciler skips unchanged controls (kind-matching plus shallow compare), so
at steady state it creates no new WinUI controls — the diff/patch cost is
dominated by COM property-set calls. Two deliberate non-features follow from this:

- **No element pooling.** With zero controls created at steady state there is
  nothing to recycle; COM creation is a fixed-cost FFI call with no GC pressure.
- **No rerender depth guard.** The render loop is non-recursive — `set_state`
  during a render sets a dirty flag and enqueues the follow-up render through the
  dispatcher rather than re-entering, so unbounded recursion is impossible.

State writes are coalesced through the dispatcher (many `set_state` calls in one
turn produce a single render).

#### Comparing against the C# reactor

The `test_reactor_perf` app (`crates/tests/libs/reactor_perf`) is a deliberate
port of the C# **`microsoft-ui-reactor`** `stress_perf` harness: same stocks-grid
workload (~4,800 cells here, 80×60; ~4,900 in C#, 70×70), the same
`--headless --percent N --duration S` CLI, and the same report — both write
`<AppName>.report.txt` with `Total Renders` and `Renders/sec`, so the numbers are
directly comparable on the same machine and power state.

The C# harness ships **two** reactor variants; the Rust crate has only one because
its idioms (cached cells, memoized setters) are always on:

- `StressPerf.Reactor` — naive baseline, "what an unaware user writes."
- `StressPerf.ReactorOptimized` — spec-034 perf idioms (direct record-initializer
  construction + `UseMemoCells`). **This is the fair comparison for Rust**, since
  `windows-reactor` already caches cells and skips unchanged subtrees by default.

Run both (from each repo root) and diff the `Renders/sec` line:

```powershell
# Rust (this repo)
cargo run -q --release -p test_reactor_perf --bin test_reactor_perf -- `
  --headless --percent 50 --duration 10

# C# (D:\git\microsoft-ui-reactor) — match your platform (x64 / ARM64)
dotnet run --project tests/stress_perf/StressPerf.ReactorOptimized -c Release `
  -p:Platform=x64 -- --headless --percent 50 --duration 10
```

`--percent` is the fraction of cells mutated per tick; hold it (and `--duration`)
equal across both runs. `Renders/sec` is a render-count throughput proxy — good for
cross-framework comparison but not the same as the user-perceived ETW Present rate;
see the C# `stress_perf/METHODOLOGY.md` for that distinction and the admin-mode
present-tracer harness.

A point-in-time snapshot (same x64 Release box, `--percent 50 --duration 10`,
median of two runs) — refresh when the workload or either framework changes:

| Metric        | Rust `windows-reactor` | C# `ReactorOptimized` |
| ------------- | ---------------------- | --------------------- |
| Renders/sec   | ~8.7                   | ~4.1                  |
| Avg Reconcile | ~7.9 ms                | ~46 ms                |
| Avg Diff      | ~7.1 ms                | ~39 ms                |
| Avg Memory    | ~190 MB                | ~285 MB               |

Both hold `renders/tick ≈ 1.0`, so the throughput gap tracks reconcile cost: the
Rust diff fits inside the frame tick while the C# reconcile gates its frame rate.
Both stacks are framework-dependent and bootstrap into the **same** installed
WinAppSDK 2.0 runtime (`Microsoft.WindowsAppRuntime.2`), so the XAML/WinUI layer is
identical — only the language runtime above it (native Rust vs .NET CoreCLR) differs.

#### Event-handler rebind (why there is no trampoline)

Inline handlers (`button("x").on_click(|| …)`) allocate a fresh `Callback` every
render, so their identity always differs and the reconciler rebinds the WinUI
event each time a control is diffed: revoke the old delegate, QI-cast to the
event interface (e.g. `IButtonBase`), and add a new one. A *trampoline* — bind the
WinUI delegate once and have it read the current handler from an
`Rc<RefCell<…>>` slot, so a change becomes a slot write — was prototyped and
measured against this cost.

An isolated microbenchmark (1000 buttons, handler-only churn, no layout) put the
WinUI rebind at ~2.7 µs and the slot-write at ~1.7 µs: ~0.95 µs saved per rebind.
That only matters under pathological churn (thousands of inline handlers
re-binding per frame); a normal app rebinds a handful per frame, where the
absolute saving is negligible. The trampoline was **rejected** — it adds
per-control slot state and codegen across every event arm without a practical
win. The real lever for hot handlers is a *stable handler identity*, which lets
`can_skip_update` skip the control's whole diff, not just the rebind. Two sources
of stable identity exist: `use_callback` (memoized closure) and — because state
setters are themselves memoized per hook slot — a `use_state`/`use_reducer` setter
passed straight to an `on_*` handler.

#### Interface-cast dispatch (avoid QI probing)

Each control is held as a `Handle` enum whose variant *is* the concrete WinUI
class (`Handle::TextBlock(bindings::TextBlock)`, …). `cast_inner::<T>()` matches
the variant and calls `windows_core::Interface::cast`, which is a COM
`QueryInterface` — and on XAML's aggregated objects a QI is comparatively
expensive, especially a *failing* one.

Shared modifiers that apply across many control families (`padding`, `foreground`,
`font_size`/`font_weight`/`font_family`) used to **probe** interfaces with an
`if let Ok(_) = cast_inner::<IControl>() … else if … ITextBlock … else …` chain.
For a `TextBlock` that meant 1–2 *failed* QIs before the successful one. A perf
run (80×60 grid of text cells whose foreground flips each frame) showed ~44 % of
all `cast_inner` calls failing — ~9,400 wasted QIs/second, every one an
`IControl` probe against a `TextBlock` (which derives from `FrameworkElement`,
not `Control`).

Because the variant already names the concrete type, these setters now `match`
the handle instead of probing. A class derefs to its **default** interface
(`bindings::TextBlock: Deref<Target = ITextBlock>`), so the common text cases call
the setter directly at **zero** QI; everything else falls through to a single
`IControl` cast. After the change the same perf run drops to **<100 failed QIs in
5 seconds** (effectively zero), with no behavior change (all self-test fixtures
pass). `set_background` keeps a short probe: its targets span `IPanel` (five panel
variants), `IControl`, and `IBorder`, and it is not on a measured hot path.

### Investigation backlog

Health/efficiency leads surfaced while profiling event-handler churn. Items 1, 2,
and 6 have since landed; the rest are not committed work and each needs
measurement on representative trees before investing.

1. **`use_callback` now skips unchanged controls *(fixed)*.** Previously every
   `on_*` builder re-wrapped its closure in a fresh `Callback::new`, so even a
   stable `use_callback` result got a new identity each render and
   `can_skip_update` never skipped the control — `use_callback` was inert for
   widget events. The setters now accept `impl IntoCallback<T>` (or
   `impl IntoUnitCallback` for parameterless handlers like `Button::on_click`) and
   call `.into_callback()`, so an existing `Callback` flows through with its
   identity preserved while bare closures still work unchanged. Pass a
   `use_callback` result straight to `on_click` to get the win. Measured on a
   1000-button handler-churn bench (reconcile ms per frame): inline `on_click`
   2.50 ms / 1001 diffed; `use_callback` wrapped in a closure 2.49 ms / 1001
   diffed (still re-wrapped, so no effect); `use_callback` passed directly
   0.10 ms / 0 diffed (the unchanged subtree is pruned at the root in one
   compare — ~24×).
2. **State/reducer setters are memoized so passing them directly skips too
   *(fixed)*.** `make_state_setter`/`make_updater` used to allocate a fresh `Rc`
   every render, so even though `IntoCallback for SetState` reuses that `Rc`
   (`from_rc`, not a fresh `Callback::new`), the identity still changed each frame
   and `on_text_changed(set_name)` never skipped. Each slot now caches its handle
   (`HookSlot::State { handle, .. }`) and a single generic `memo_handle` helper
   builds it once and clones it each render, so the common "handler just calls a
   setter" pattern skips *without* `use_callback`. The same helper backs both
   `use_state` (`SetState`) and `use_reducer` (`Updater`). `use_reducer_fn`'s
   `Dispatch` is deliberately *not* memoized: it captures the user `reducer`
   (memoizing would pin the first render's closure), and it is almost always
   wrapped at the call site (`on_click(move || dispatch.call(Action::X))`), so a
   stable identity would not help skipping anyway. Reconcile bench (`setter` mode,
   `on_text_changed(set_text)`): 0.10 ms / 0 diffed — matching the `use_callback`
   path. The `alloc_bench` tool (counting global allocator + `RenderCx`, no WinUI)
   measures steady-state heap allocations per render: `use_state` 0, `use_reducer`
   0 (was 1 before memoization), `use_callback`/`use_ref` 0, `use_reducer_fn` 2
   (the un-memoized reducer + dispatch `Rc`s). `use_callback` is now only needed
   for handlers that close over render-derived data (where deps must gate the
   rebuild).
3. **`diff_props` is O(n²) per control *(low–medium)*.** `find_prop`/`find_event`
   (`widget.rs`) are linear scans called inside the per-binding loop. Fine for the
   handful of bindings most controls carry; profile the binding-count distribution
   on real trees before considering sorted bindings / a small-map.
4. **Silently-dropped props *(partly fixed)*.** Unsupported `(prop, control)`
   combos hit `diag::unhandled_modifier` (`backend/winui/diag.rs`), which both
   floods debug output per control creation and silently no-ops a prop the author
   thinks applies. The `Padding`-on-`TextBlock`/`StackPanel` case was a real gap —
   both types own a `Padding` property WinUI supports, but the bindings filter and
   `set_padding` only covered `Control`/`Border`; `set_padding` now also casts to
   `ITextBlock`/`IRichTextBlock`/`IStackPanel`. Remaining work: audit the rest of
   the dropped set and either support each combo or surface it once rather than
   per-control.
5. **Per-prop WinUI set cost *(measured — no steady-state lever)*.** `set_prop`
   was instrumented to time each call keyed by `Prop` and run against the
   `test_reactor_perf` grid (the headless self-test does not commit to the WinUI
   backend, so it never calls `set_prop` — the perf app is the only set-prop
   workload). Findings: steady state is dominated entirely by `Text` and
   `Foreground` *volume* (~2.4 µs/call each, one set per genuinely-changed cell —
   no redundant sets), and those are irreducible thin COM property sets, so there
   is **no batching/deferral win on the hot path**. The expensive-per-call props
   are all one-shot at creation: `Button.Content` ~300 µs/call (confirms the
   measure/arrange hypothesis — ~125× a `Text` set, but only paid when a button's
   label changes), and `GridRows`/`GridColumns` ~800 µs/call (each rebuilds N
   row/column definitions in a loop). None sit on a steady-state path, so the lead
   is closed: the reconciler already issues the minimum number of sets and the
   costly props are rare creation-time operations.
6. **Redundant default-interface QIs eliminated *(fixed)*.** A sweep of the WinUI
   backend removed `cast`s that re-`QueryInterface` an object to an interface it
   already `Deref`s to (see *COM pitfalls*). Three buckets: (a) **event-handler
   `sender` casts** — value-change handlers (`TextChanged`, `PasswordChanged`,
   `Toggled`, `SelectionChanged`, `ValueChanged`, RichEditBox, Pivot/ComboBox,
   etc.) cast `sender` to the control on every fire; they now capture the typed
   handle at attach and read through `Deref` at 0 QI. The generator
   (`gen_attach.rs::gen_sender_getter`) emits this capture pattern, so the five
   generated handlers are fixed at the source. (b) **event-handler `args` casts** —
   `DragEventArgs`/`NavigationViewSelectionChangedEventArgs`/
   `KeyboardAcceleratorInvokedEventArgs` each `Deref` to the interface the code was
   casting to; the drag path (`build_drag_context`/`accept_or_reject`, hot on every
   `DragOver`, plus `DragEnter`/`Drop`) dropped its `IDragEventArgs`/
   `IDataPackageView`/`IDragOperationDeferral` casts and two private helpers were
   retyped to the concrete classes. (c) **hand-written `set_prop`/build casts** —
   `RowDefinition`/`ColumnDefinition`/`BitmapImage` cast to their own default
   interface. Capturing a control in its own handler creates a delegate→control
   cycle; this is severed by the existing revoker teardown (validated by the
   `event_detachment` and `repro_leak_header_pane` self-test fixtures). The
   remaining casts are all genuine: parent interfaces (`IPanel`, `IControl`,
   `IFrameworkElement`, `DropDownButton`→`IButton`), versioned interfaces
   (`ICompositor2`, `INavigationView2`), `IInspectable`→class downcasts, and
   collection interfaces — none are removable. Keep new hand-written handlers to
   the capture-at-attach pattern so this does not regress. To hunt for redundant
   casts dynamically, enable the opt-in `windows_cast_diagnostics` cfg on
   `windows-core` (debug-only, off by default, zero release impact): it warns to
   stderr — with the exact `#[track_caller]` call site — whenever `cast`'s
   `QueryInterface` returns the same interface pointer it started from (i.e. the
   source already exposes that interface). Set it via `RUSTFLAGS` (or uncomment the
   line in `.cargo/config.toml`), e.g.
   `$env:RUSTFLAGS = "--cfg windows_cast_diagnostics"; cargo run -p test_reactor_selftest -- --headless`
   and grep the output. A hit is usually a class cast to its own default interface
   (replace with `Deref`); a cast to `IUnknown`/`IInspectable` is reported too
   (prefer `.into()`), though in practice this codebase has ~none since it already
   uses `.into()`. The report counts every invocation, so sort/unique by call site.
   Acting on it once took the self-test from ~1000 hits across ~36 sites down to 4:
   the leads it surfaced beyond the backend — `app_shim.rs` (`IXamlMetadataProvider`)
   and the self-test harness/`exec.rs` (`IDispatcherQueue` and ~30 builder casts) —
   are all cleaned. The 4 residual hits are genuine false positives the heuristic
   cannot distinguish: `factory_cache.rs`'s runtime `IAgileObject` agility probe and
   `generic_factory.rs`'s type-erased `IInspectable`→class activation, where the
   same-pointer result is incidental and the cast is not statically removable.
7. **Casts hidden behind `Param<T>` / `required_hierarchy!` *(examined, nothing to
   reduce)*.** Besides explicit `.cast()`s, conversions also happen implicitly when
   a value is passed as `impl Param<T>`. `windows-bindgen` statically classifies
   every conversion in `bindings.rs` into one of two macros (`crates/libs/core/src/imp/mod.rs`):
   `interface_hierarchy!` (`CanInto::QUERY = false`) for the free relationships —
   every type → `IUnknown`/`IInspectable`, and a class → its **default** interface —
   which `Param` resolves with a `transmute_copy` (zero cost), versus
   `required_hierarchy!` (`QUERY = true`) for a class → a **non-default** required
   interface and an interface → a required (sibling) interface, which `Param`
   resolves with a real `QueryInterface` (`crates/libs/core/src/param.rs`). The
   `required_hierarchy!` entries are therefore the *implicit* form of the genuine
   "cast to a non-default parent" calls in item 6 (e.g. `SolidColorBrush`→`Brush`,
   `Button`→`IControl`) — one QI per call, not redundant, and **not statically
   removable** (transmuting to a vtable the object does not expose would be
   unsound). bindgen has already done the removable work by routing every
   transmute-able conversion through `interface_hierarchy!`/`Deref` instead (the
   class default interface is explicitly filtered out of `required_hierarchy!`).
   The only lever left is the same hot-path rule as item 6: if a `Param`-driven
   non-default conversion lands on a steady-state path, capture the converted handle
   once rather than re-converting each call. The `windows_cast_diagnostics` probe
   instruments this `Param`/`self.cast()` path too, and (as expected) never reports
   these as same-pointer hits — confirming they are genuine QIs, not redundant ones.

### Reactor / canvas naming

`windows-reactor` and `windows-canvas` define some of the same short names for
different domains. The rule: canvas keeps the short name (it owns user-facing draw
loops) and reactor takes a domain-prefixed alternative. `Color` → `ColorF` in
canvas is done; `Brush`, `Ellipse`, and `FontWeight` still overlap.

### Testing

Unit tests live in `test_reactor` (headless). Integration tests live in
`test_reactor_selftest`, which launches a real WinUI window — pass `--headless`
for CI.

Most pointer handler behavior (attach/detach, memoization, slot changes) is
covered headlessly in `test_reactor` via the `RecordingBackend`. The one path
that needs a live `PointerRoutedEventArgs` — the backend's `set_pointer_handlers`
wiring and `pointer_event_info` extraction — is exercised end-to-end by the
`Pointer_Injection_Gesture` selftest fixture, which drives real OS mouse input
through the WinRT `InputInjector` (move → press/release left and right → exit)
and asserts the reactor's `on_pointer_*` callbacks fire with the right position
and button flags. Because OS input injection requires the harness window to be
foreground at the injected screen point, the fixture records a TAP `# SKIP`
(never a failure) when the host can't deliver input (locked session, foreground
lock, no interactive desktop), so it can't flake CI.

The dispatcher-thread hooks in `hooks.rs` — `DispatcherTimer` (repeating and
one-shot) and the `CompositionTarget::Rendering` subscription (`on_rendering`) —
are likewise only meaningful on a live WinUI thread, so they are covered by the
`Timer_*` and `Rendering_Subscription_*` selftest fixtures. These wait on a
wall-clock-bounded `Harness::pump_until` (not the iteration-bounded `render_until`)
so a real `DispatcherQueueTimer` interval can elapse, and assert both that the
callback fires and that dropping the RAII handle stops it. The rendering fixture
soft-SKIPs when the agent delivers no composition frames.

The `RecordingBackend` harness (and its `Op` log) lives in the `test_reactor`
crate, not in `windows-reactor`, so it adds no weight to normal builds. The few
engine/reconciler/widget inspectors that need access to private fields stay in
`windows-reactor` behind the `test` feature, which the test crates enable and
normal/published builds leave off. This is the rule for *all* tests: no
`#[cfg(test)]` modules inside the published library crates — put the test in the
matching `test_*` crate. If it needs an internal item, expose that item behind the
existing `test` feature rather than adding bespoke scaffolding (the engine/
reconciler/widget inspectors work this way). Don't invent a feature — or a public
helper — just to test a trivial pure function; leave it private and untested.

### Future work — C# reactor parity

`windows-reactor` is the Rust counterpart of the C# **`microsoft-ui-reactor`**
framework ([microsoft-ui-reactor](https://github.com/microsoft/microsoft-ui-reactor)), already referenced above for the
`stress_perf` performance benchmarks. The C# project covers considerably more
surface area — ~50 hooks, ~80 control factories, and whole subsystems for
validation, charting, docking, and localization. This section catalogs those gaps
so contributors can see the intended direction. As with canvas, the goal is
idiomatic Rust coverage of what real apps need, not a mechanical 1:1 port.

Ordered roughly by user impact; "present" notes what already exists.

#### 1. Hooks breadth *(high)*

Present: `use_state`, `use_ref`, `use_memo`, `use_effect`
(+`use_effect_with_cleanup`), `use_reducer`/`use_reducer_fn`, `use_resource`,
`use_context`, `use_callback`, `use_color_scheme` — the core ~11.

The C# framework exposes ~50. Notable missing groups:

- **Async / data** — `use_mutation`, `use_infinite_resource`, `use_data_source`.
- **Observable binding** — `use_observable`, `use_observable_tree`,
  `use_observable_property`, `use_collection` (INPC / observable-collection
  integration).
- **Window / system** — `use_window_size`, `use_breakpoint`, `use_dpi`,
  `use_window`/`use_window_state`/`use_is_active`, `use_window_position`,
  `use_displays`, `use_closing_guard`, `use_file_picker`/`use_folder_picker`,
  `use_window_drag_move`, `use_tray_icon`, `use_open_window`.
- **Theme / a11y / environment** — `use_is_dark_theme`, `use_high_contrast`,
  `use_reduced_motion`, `use_announce`, `use_intl`, `use_persisted`.
- **Focus** — `use_focus`, `use_focus_trap`, `use_element_focus`,
  `use_element_ref`.
- **Commanding** — `use_command` (see §3).
- **Tooling** — `use_devtools`, the `use_memo_cells` family.

Each is independently shippable.

#### 2. Multi-window, system tray, and pickers *(high)*

Present: a single `App` window builder (`title`, `inner_size`, `backdrop`,
`presenter`, `fullscreen`).

Missing: secondary windows (`ReactorWindow` / `use_open_window`), tray icons,
window drag-move, multi-display awareness, window position/state persistence,
close-guard confirmation, and file/folder pickers. These turn the crate from
"single-window app shell" into a general desktop-app framework.

#### 3. Commanding *(medium-high)*

Present: direct `on_click` and `keyboard_accelerator` per control.

Missing: a `Command` abstraction (`Command`, `StandardCommand`,
`CommandBindings`, `use_command`) — a reusable, enable/disable-aware action bound
once and shared across buttons, menu items, and accelerators. The C# control
factories all accept a command (`button(command)`, `menu_item(command)`, …); the
Rust equivalents accept only closures.

#### 4. Data binding, observables, and validation *(medium-high)*

Present: state hooks and one-way rendering from state.

Missing the entire form/validation and observable stack: `FormField`,
`ValidationRule`, `ValidationContext`, `ValidationVisualizer`, and the
observable-collection hooks from §1. This is what makes data-entry UIs ergonomic.

#### 5. Higher-level controls *(medium)*

Present: ~60 wrapped WinUI controls (text, buttons, inputs, layout, navigation,
lists, dialogs).

Missing the C# composite controls built *on top* of WinUI:

- **`VirtualList`** — large-collection virtualization beyond raw `ListView`.
- **`DataGrid`** and **`PropertyGrid`** — tabular / reflection-driven editing
  (`TypeRegistry`, `TypeMetadata`).
- **`MaskedTextBox` / `AutoSuggest`** input controls and input formatters.
- **Flexbox layout** — the C# project embeds a Yoga flexbox engine
  (`Flex`/`FlexRow`/`FlexColumn`, `UniformGrid`, `WrapGrid`); Rust currently relies
  on WinUI `StackPanel`/`Grid` only.

#### 6. Animations *(medium)*

Present: implicit transitions (`with_opacity_transition`, `with_scale_transition`).

Missing the richer animation system: keyframe animations, layout animations,
interaction-state animations, stagger configuration, and connected / scroll-linked
animations (`AnimationConfig`, `KeyframeAnimations`, `InteractionStates`,
`StaggerConfig`). These belong on Windows.UI.Composition (the backend behind
reactor's existing transitions), which animates retained visuals off-thread;
sampling-based [`windows-animation`](windows-animation.md) is intentionally *not*
the fit here — it targets immediate-mode canvas drawing (see that crate's *Future
work*).

#### 7. Navigation framework *(medium)*

Present: the `navigation_view` control wrapper.

Missing: a navigation/routing layer — `NavigationHandle`, `use_navigation`,
`use_navigation_lifecycle`, `use_system_back_button` — that manages a page stack
and lifecycle rather than just rendering the chrome.

#### 8. Theming and resources *(medium-low)*

Present: `background`/`foreground` brush bindings and `use_color_scheme`.

Missing: a general theme/resource system — theme-keyed bindings (`ThemeRef`,
`ThemeBindings`), per-control resource overrides, and a resource builder
(`ResourceBuilder`, `ResourceOverrides`).

#### 9. Gestures and drag-and-drop *(medium-low)*

Present: pointer and tap handlers (and the coordinate/keyboard work tracked in
[`windows-canvas`](windows-canvas.md)'s *Input and hit-testing*).

Missing: a gesture-recognition and drag-and-drop layer (the C# `Reconciler.Gestures`
/ `DragDrop` / `DragAttached` machinery).

#### 10. Lower-priority subsystems

- **Localization** — `LocaleProvider`, intl accessors, `.resw` providers,
  pseudo-localization.
- **Accessibility tooling** — `use_announce`, an accessibility scanner, semantic
  panels.
- **Feature modules** — Markdown rendering, charting, and docking are whole
  optional subsystems in the C# project with no Rust equivalent yet.

#### Suggested sequencing

The hook breadth (§1) is the highest-leverage area and decomposes into many small,
independent additions — start with the window/system and focus groups. Multi-window
and pickers (§2) and commanding (§3) complete the desktop-app shell; the
validation/observable stack (§4) and composite controls (§5) are larger efforts
best taken once those foundations exist. Animations (§6), navigation (§7), theming
(§8), and the remaining subsystems can land independently as demand arises.
