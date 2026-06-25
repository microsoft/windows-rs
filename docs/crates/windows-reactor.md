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
`.on_pointer_pressed(..)`, `.on_pointer_released(..)`,
`.keyboard_accelerator(..)`. A `SetState` or `Dispatch` can be passed directly
wherever a handler is expected (via `IntoCallback`).

## Graphics integration

For custom 2D drawing, host a [`windows-canvas`](windows-canvas.md) surface with
`animated_canvas(draw)` (enable the `reactor` feature on `windows-canvas`). It
returns a `SwapChainPanel` element that redraws every frame and recovers from
device loss automatically — see the `canvas` samples. For raw Direct3D, the
`swap_chain_panel` sample drives a `SwapChainPanel` with `on_rendering`.

## Web content integration

To host a browser, use [`windows-webview`](windows-webview.md)'s `webview(on_ready)`
(enable the `reactor` feature on `windows-webview`). It returns a `WebView2` element
backed by the WinUI XAML `WebView2` control and hands you a ready-to-drive `WebView`
once the browser initializes — see the `reactor/webview` sample. Reactor exposes the
control as a thin native-handle widget (mirroring `SwapChainPanel`): the widget owns
the XAML control, and `windows-webview` attaches to it through its `IInspectable`
handle, defers `EnsureCoreWebView2Async` to the control's `Loaded` event, and bridges
the WinRT `CoreWebView2` to the COM `ICoreWebView2` the crate wraps. The
`as_self_contained()` setup carries the required `Microsoft.Web.WebView2.Core.dll`
automatically.

## Samples

The [`crates/samples/reactor`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
tree is the best reference:

- **`minimal`** — the smallest app plus an `examples/` folder with ~90 focused
  per-control and per-hook examples (`counter`, `calculator`, `navigation_view`,
  `list_view`, `content_dialog`, `color_picker`, and many more).
- **`apps`** — complete applications: `notepad`, `solitaire`, `minesweeper`,
  `tictactoe`, `dotsweeper`, `diagnostics_demo`.
- **`gallery`** — a WinUI-gallery-style shell with navigation across many controls.
- **`direct2d`** / **`swap_chain_panel`** — hosting Direct2D / Direct3D content.
- **`webview`** — hosting a WebView2 browser via `windows-webview`'s `reactor` feature.
- **`framework-dependent`** / **`self-contained`** — the two deployment models,
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
  `button.cast::<IButton>()?.SetFlyout(...)`). Only cast to **non-default parent**
  interfaces (e.g. `Button` → `IContentControl`/`IControl`).
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
  hold COM objects that are only valid on the UI thread. `EXPECT_PANIC` is also
  thread-local by design, since the panic hook reads it.
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
This is the *optimized* C# variant (cached cells), the fair comparison for Rust.
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

Health/efficiency leads surfaced while profiling event-handler churn. Items 1–2
have since landed; the rest are not committed work and each needs measurement on
representative trees before investing.

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
5. **Per-prop WinUI set cost *(medium)*.** Setting some properties forces a
   synchronous measure/arrange subtree rebuild (`Button.Content` is the worst
   offender observed). "Diff cost dominated by COM property-set calls" can be made
   concrete by timing per-`Prop` set cost and flagging the expensive ones for
   batching/deferral.

### Reactor / canvas naming

`windows-reactor` and `windows-canvas` define some of the same short names for
different domains. The rule: canvas keeps the short name (it owns user-facing draw
loops) and reactor takes a domain-prefixed alternative. `Color` → `ColorF` in
canvas is done; `Brush`, `Ellipse`, and `FontWeight` still overlap.

### Testing

Unit tests live in `test_reactor` (headless). Integration tests live in
`test_reactor_selftest`, which launches a real WinUI window — pass `--headless`
for CI.

The `RecordingBackend` harness (and its `Op` log) lives in the `test_reactor`
crate, not in `windows-reactor`, so it adds no weight to normal builds. The few
engine/reconciler/widget inspectors that need access to private fields stay in
`windows-reactor` behind the `test` feature, which the test crates enable and
normal/published builds leave off.

### Future work — C# reactor parity

`windows-reactor` is the Rust counterpart of the C# **`microsoft-ui-reactor`**
framework ([microsoft-ui-reactor](https://github.com/microsoft/microsoft-ui-reactor)), already referenced above for the
`stress_perf` performance benchmarks. The C# project is considerably more mature —
~50 hooks, ~80 control factories, and whole subsystems for validation, charting,
docking, and localization. This section catalogs the gaps so contributors can see
the intended direction. As with canvas, the goal is idiomatic Rust coverage of
what real apps need, not a mechanical 1:1 port.

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

Each is independently shippable; the window/system and focus groups likely have the
broadest appeal.

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
independent additions — the window/system and focus groups first. Multi-window and
pickers (§2) and commanding (§3) round out the desktop-app shell. The validation /
observable stack (§4) and composite controls (§5) are larger efforts best taken
once the hook and commanding foundations exist. Animations (§6), navigation (§7),
theming (§8), and the remaining subsystems can land independently as demand
arises.
