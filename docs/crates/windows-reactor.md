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
(e.g. `Backdrop::Mica`), `icon` (path to an `.ico` file), `fullscreen`, and
`presenter` are common. `render(app)`
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
`animated_canvas(draw)` (enable reactor's `canvas` feature, which pulls in
`windows-canvas`). It returns a `SwapChainPanel` element that redraws every frame
and recovers from device loss automatically — see the `canvas` samples. For content
that is static between updates, `CanvasImageSource` instead draws *on demand* into a
`SurfaceImageSource` shown with the `Image` widget, redrawing only when you call
`draw`; `Image::on_mounted` yields an `ImageHandle` whose
`on_rasterization_scale_changed` reports the host DPI scale so the surface stays
crisp across monitor moves (see the canvas `image_source` sample). Both
`animated_canvas` and `CanvasImageSource` are reactor exports (they own the WinUI
element harness), built on the safe drawing surface `windows-canvas` provides. For
raw Direct3D, the `swap_chain_panel` sample drives a `SwapChainPanel` with
`on_rendering`.

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

`src/bindings.rs` holds the flat WinUI / COM interop bindings the hand-written
backend calls. It is generated by `tool_reactor` (which drives `windows-bindgen`
in `--flat --minimal` mode) from two filter files: the hand-maintained
`crates/tools/reactor/src/base.txt` — the base-interface method list — plus the
tool-produced `generated.txt` (per-widget entries). Regenerate it (and the
selftest copy at `crates/tests/libs/reactor_selftest/src/bindings.rs`) with the
same `cargo run -p tool_reactor`. Raw metadata names apply: `get_Prop`, `put_Prop`,
`add_Event`, `remove_Event`.

To prune or extend bindings: edit `base.txt`, regenerate, and let the compiler
errors reveal the methods you still need (`SetX` → `put_X`, `X()` → `get_X`); add
them as `Ns.IFace::{put_X, get_Y}` under the appropriate base interface. This also
covers the Win32 COM interop interfaces (e.g. `ISwapChainPanelNative`,
`ISurfaceImageSourceNativeWithD2D`) — listed methods get full vtable entries,
unlisted methods become `usize` slots, and the type closure is computed
automatically.

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

`Background` and `Foreground` follow the same pattern and are exposed as the
universal `ElementExt` modifiers `.background(...)` / `.foreground(...)`: `Border`
handles them through its default interface while every other handle falls back to
a single `IControl` cast (`set_background` / `set_foreground` in
`backend/winui/mod.rs`), so they work on any `Control`. `BorderBrush` /
`BorderThickness` use the same `IControl` fallback in the backend
(`set_border_brush` / `set_border_thickness`), but are *not* `ElementExt`
modifiers — they are opt-in per-widget builders. Only `Border` and `TextBox`
currently expose `.border_brush(...)` / `.border_thickness(...)`; the shared
backend dispatch means any other widget could expose them without new backend
work.

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

### Error handling and panics

Reactor sits between the developer's Rust closures and WinUI's COM/`extern
"system"` delegates. Failures can originate on either side, and *where* a failure
happens dictates what can be done with it. This section records the current
behavior, the inconsistencies it creates, and the target design.

#### The five failure boundaries

| Boundary | Where it runs | Current handling | Reaches the developer? |
|---|---|---|---|
| **Synchronous setup** — `bootstrap`, `init_app_platform`, icon validation | main thread, before the message loop | `Result` from `run`/`bootstrap` | **Yes** — genuinely propagates |
| **`OnLaunched` / setup callback** — host creation, `activate` | UI thread, inside `Application::Start` | `run_callback` (`catch_unwind` → `Result`) → `diagnostics::emit` | **No** — the `Result` dies inside `OnLaunched`; `Application::Start` never returns it, the loop keeps pumping |
| **Render pass** — `root.render`, component renders | dispatcher-posted `render_loop` | uncaught, **except** subtrees under `error_boundary` (`catch_unwind` → fallback UI) | Only if wrapped in `error_boundary` |
| **Event handlers / timer ticks / `on_rendering`** | invoked directly from WinUI delegates | **uncaught** | **No** — a panic aborts the process |
| **Backend prop / COM application** | applying props to controls | four different ways (see below) | mostly debug-only or silent |

The structural reason the last two boundaries abort: the generated delegate
`Invoke` thunks in `bindings.rs` are `extern "system"` with **no** `catch_unwind`.
A panic in an `on_click`, a `DispatcherTimer` tick, or `on_rendering` unwinds into
that boundary and aborts (after the default panic hook prints). The only two
places reactor catches panics today are `run_callback` (setup) and `error_boundary`
(render subtrees).

#### The inconsistencies

- **`Result` that cannot propagate.** `App::run` / `activate` return `Result`, but
  every failure inside `OnLaunched` is caught, logged, and then the app limps on
  (often windowless). The signature promises propagation the runtime cannot honor.
  This is distinct from the COM-plumbing `Result`s in `backend/winui/mod.rs`,
  `app_shim.rs`, and `convert.rs`, which are *mandatory* and correct.
- **Same failure, different fate.** A panic during render under an `error_boundary`
  degrades to fallback UI; the *identical* panic in that component's `on_click`
  aborts the process. Developers cannot predict which, and nothing documents it.
- **Best-effort backend drops handled four ways:** `let _ = fe.SetStyle(...)`
  (silent), `diag::com_error` (debug log), `diag::unhandled_prop` /
  `unhandled_modifier` (debug log), and inline `if cfg!(debug_assertions) {
  eprintln! }` (debug log). Three express the same intent with different code.
- **Three logging conventions:** `diagnostics::emit` (unconditional stderr, FFI/app
  level), `diag::*` (debug-only, backend), and raw `eprintln!` (debug-only,
  scattered). The first two overlap in purpose.
- **No documented contract** for what panics, what returns `Result`, and what is
  silently dropped.

`panic!` usage itself is already consistent — the sites are rules-of-hooks and
invariant violations (hook order, type mismatch, `EventHandler` variant mismatch)
in `engine.rs` and `backend/mod.rs`. That is the correct use of `panic!` and stays.

#### Target design — differentiate by failure *class*

The unifying principle: **`Result` only where it can propagate (synchronous,
pre-loop); one fault hook for everything inside WinUI callbacks; `panic!` only for
bugs; one debug-log helper for best-effort drops.**

1. **Programmer errors / invariants → `panic!` (now *caught*, not fail-fast).** The
   panic sites are unchanged (rules-of-hooks, type mismatches), but the *outcome*
   changed: a panic at a reactor-owned boundary unwinds into the fault boundary and
   is reported to `on_fault` (default: log-and-continue) rather than aborting the
   process. `panic!` is therefore **decoupled from fail-fast** — it now means "isolate
   and report this callback," not "kill the process." This relies on `panic = "unwind"`
   (the Cargo default); under a `panic = "abort"` profile the entire model is bypassed
   (`catch_unwind` never runs) and every panic aborts — the traditional whole-binary
   fail-fast posture, chosen by the app, not the library. When a specific fault is
   genuinely unrecoverable, escalate with the *uncatchable* primitives —
   `std::process::abort()` / `exit()`, either directly in the callback or as a branch
   inside `on_fault` — because `catch_unwind` cannot intercept those.
2. **Synchronous, pre-loop configuration errors → `Result` from `run`/`bootstrap`
   *(implemented)*.** Validate up front so the `Result` is meaningful (as
   `App::icon` path validation already does — it runs on the calling thread before
   `Application::Start`).
3. **Failures inside UI-thread callbacks (render, event handlers, timers,
   `on_rendering`) → one reactor-owned fault boundary, not `Result` *(implemented)*.**
   Reactor catches panics at the entry points it owns — `Callback::invoke` (every
   event handler), the `DispatcherTimer` tick, `on_rendering`, and the render pass
   (`render_once`) — turning a panic into a *controlled, logged fault* instead of an
   abort, and delivering it to a developer-supplied `App::on_fault(|fault| ...)`
   hook (default: log-and-continue). The catch is **context-aware**: a callback that
   panics *during* a render pass is left to propagate so
   `error_boundary` can recover the subtree first; only panics outside render
   (or escaping every boundary) are reported to `on_fault`. Implemented as the
   `fault` module (`fault.rs`): a thread-local `IN_RENDER` guard makes
   `fault::catch` transparent during render and active outside it, while
   `fault::render_scope` wraps the render pass. This makes "panic for bugs" *safe*
   (panics stop aborting) and makes the event-handler / render split predictable.
4. **Best-effort backend prop application → one helper, one policy *(implemented)*.**
   Collapsed the `let _ =`, `diag::com_error`, and ad-hoc `eprintln!` variants into a
   single debug-warn / release-noop helper (`diag::warn` core plus `diag::dropped`,
   which reports the dropped `Result`'s call site via `#[track_caller]`). Within the
   backend apply path (`backend/winui/mod.rs`) the silent and raw-`eprintln` forms are
   gone; the only bare `let _ =` sites left there drop a non-`Result` (an unused
   parameter and a fire-and-forget event token). (`host.rs` retains a few `let _ =`
   drops on genuinely fire-and-forget window plumbing — `WindowHandle`, `Activate`,
   the cursor `PostMessageW` — which are not developer-requested configuration.)
5. **Make `activate()` honest *(implemented)*.** `activate` keeps `Result` only for
   its genuinely synchronous failures — the dispatcher lookup
   (`DispatcherQueue::GetForCurrentThread`) and enqueue (`TryEnqueueWithPriority`),
   both of which run on the calling thread and propagate to the caller. The deferred
   work that runs later inside the enqueued UI-thread callback (presenter / icon /
   backdrop) can no longer return a `Result` to anyone, so it routes to the fault
   path instead: the whole callback is wrapped in `fault::catch("activate", …)` so a
   panic is a controlled fault rather than a process abort, and each best-effort
   configuration failure is delivered to `on_fault` via `fault::report` (replacing the
   swallowed inner `Result` and the ad-hoc `eprintln!`). This required a
   `fault::report(context, message)` companion to `fault::catch` for reporting an
   explicit failure (not a panic) through the same handler.
6. **Document the contract** (this section) and surface it in the readme.

Answering the three framing questions directly: *panic more?* yes for bugs, but
only once the callback catch boundaries exist (otherwise more panics = more
aborts). *`println` more?* consolidate to one helper, but logging alone is invisible
in release — a fault hook, not just logging, is what makes failures actionable.
*stop using `Result`?* trim the semi-functional app/host-boundary `Result`; keep the
COM-plumbing and synchronous-setup `Result`s, which work.

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

### Known gaps and fixes

Specific bugs and gaps with a known reproduction, tracked separately from the C#
parity catalog below. Recorded so we can track progress and, where possible, close
the "had to drop to the raw `windows` crate" cases. Items marked *(fixed)* have
landed; *(gap)* items are still outstanding.

1. **Nested component doesn't re-render from its own `use_state` *(fixed)*.** A
   component that mutates only its own `use_state`, buried under structurally
   *unchanged non-component* parents (e.g. `scroll_viewer` → `grid`), never
   re-rendered: its `state_dirty` flag was set and `request_rerender` fired, but the
   pass pruned the subtree before descending to it. Root cause: `peek_state_dirty`
   (`engine.rs`) was consulted only via `is_component_state_dirty` at the node
   *currently being visited* (`reconciler.rs` `update`, `reconciler/child.rs`), so a
   dirty component below a pruned parent was skipped. Context changes avoided this
   because `force_context_subscribers` sets the global `force_component_rerender`
   flag that punches through pruning (`reconciler.rs`); plain `use_state` writes had
   no equivalent path — that asymmetry was the whole bug (the root component itself
   was never affected: the host re-renders it every pass via its own `RenderCx`, and
   the symptom was also masked whenever some prop on the path changed anyway, e.g. a
   sibling chart's per-tick `revision`, forcing descent for an unrelated reason).
   Fixed: `Reconciler::reconcile` now calls `force_state_dirty_components`, which
   scans `component_instances` for any entry reporting `peek_state_dirty()` and seeds
   `forced_components` / sets `force_component_rerender` the same way the context path
   does, plus a `debug_assert!` that no seeded instance is still dirty after the pass
   (a dropped re-render failed silently before). Regression:
   `test_reactor::nested_state_rerender`. Related trap still worth documenting:
   reactor's `TextBox` is *controlled* (pushes `Prop::Value` every render), so
   round-tripping a controlled field through global state on every keystroke races
   persistence and can truncate long values — add uncontrolled-input guidance or a
   first-class uncontrolled text mode.

2. **Window icon *(fixed)*.** WinUI 3 doesn't adopt the exe's embedded icon for
   the title-bar/taskbar icon, and reactor previously exposed no way to set it —
   `AppWindow.SetIcon` was stubbed out of the bindings and `IAppWindow`/
   `IWindowNative::WindowHandle` are `pub(crate)`, forcing apps to add
   `Win32_System_LibraryLoader` + `Win32_System_Threading` +
   `Win32_UI_WindowsAndMessaging` to the raw `windows` dependency and re-find their
   own window (`EnumThreadWindows` by title — `FindWindowW` doesn't find WinUI
   windows) to `SendMessageW(WM_SETICON)`. Now: `SetIcon` is un-stubbed in the
   reactor bindings filter (`crates/tools/reactor/src/base.txt`, the `IAppWindow`
   method list), and `App::icon(path)` sets the window icon from a path to an `.ico`
   file. `ReactorHost::set_icon` applies it on the UI thread in `activate()` via
   `AppWindow.SetIcon` (`host.rs`), alongside the presenter/backdrop application.
   The application is best-effort (a failure is logged, not fatal), matching how
   presenter/backdrop degrade — surfacing it as a `Result` from `activate()` is not
   viable because host setup runs inside WinUI's `OnLaunched`, whose error does not
   propagate out of `Application::Start` (the loop keeps pumping). `AppWindow.SetIcon`
   also silently tolerates a missing file, so `App::run` instead validates the path
   up front on the calling thread — before `Application::Start` — and returns a real
   `Result` error for a missing icon, which is the only architecturally sound place
   to give the caller one. Sample: `cargo run -p reactor_samples --example icon`.

3. **Swapping a canvas out of the tree leaks its render loop *(fixed)*.**
   Conditionally swapping one keyless subtree for a shorter, differently shaped one
   (a `vstack` with a nested `component(...)` → a compact `hstack`) left orphaned
   rendering surfaces on screen; adding distinct `.with_key(...)` to each layout
   appeared to fix it. Investigation showed the reconciler is **not** at fault —
   keyless positional reconcile correctly unmounts and destroys nested
   `SwapChainPanel`s when a container shrinks (`reconciler/child.rs`; verified
   against the exact shape). The real leak was in
   [`windows-canvas`](windows-canvas.md)'s `animated_canvas`: its per-frame
   `RenderState` holds the `CompositionTarget::Rendering` subscription and the
   swap chain in a **reference cycle** (the rendering callback captures an `Rc`
   back to the cell that owns it), and it registered no `on_unmounted` handler —
   so refcounting alone could never drop it. When the panel left the tree the
   render loop kept firing and presenting to the detached swap chain. Keying only
   masked it by reshuffling which panels survived. Fixed by adding an
   `on_unmounted` teardown that clears the state cell, dropping the `RenderState`
   in place (revoking the subscription and releasing the swap chain). Regression:
   `test_reactor::animated_canvas_installs_unmount_teardown`.

4. **On-demand D2D drawing surface pulls in the raw `windows` crate *(fixed)*.**
   Drawing on-demand content (a surface that repaints on a data change rather than
   every frame) into reactor's `SurfaceImageSource` used to require hand-rolled
   D3D11/D2D/DXGI/DirectWrite plus a hand-managed shared `ID2D1Device`. This is now
   covered by `CanvasImageSource` (a reactor export under reactor's `canvas`
   feature, built on [`windows-canvas`](windows-canvas.md)): create it from a canvas
   `GpuDevice`, draw with the safe canvas `DrawingSession` API, and hand
   `image_source()` to `Image::new` — no raw `windows` crate. It complements
   `animated_canvas` (continuous, swap-chain) with an on-demand path that only
   redraws when you call `draw`. Internally it drives reactor's `SurfaceImageSource`
   widget through a *borrowed* `DrawingSession` (the surface owns the
   `BeginDraw`/`EndDraw` bracket, and canvas exposes
   `DrawingSession::from_borrowed_context_with_dpi` so the private `SetDpi` binding
   stays inside `windows-canvas`). To keep the surface crisp,
   `Image::on_mounted` yields an `ImageHandle` whose
   `on_rasterization_scale_changed` reports the host element's DPI scale (read from
   `XamlRoot.RasterizationScale` after `FrameworkElement.Loaded`, and again on
   `XamlRoot.Changed`); pass that scale to `CanvasImageSource::new`. See the canvas
   `image_source` sample.

5. **Composition-interop seam for hosting composition content *(fixed)*.**
   Hosting custom composition content requires reaching the
   `Microsoft.UI.Composition` layer (WinUI 3's own *lifted* compositor, *not* the
   system `Windows.UI.Composition` projected by the `windows` crate — the two
   stacks are non-interoperable) *under an arbitrary reactor element*: attach a
   custom `Visual` (`ElementCompositionPreview.SetElementChildVisual`), obtain the
   `Compositor` that owns the element's visual so it can build *same-compositor*
   child visuals, and read the element's rasterization (DPI) scale so the
   composition surface is crisp. Reactor already drives this plumbing internally
   for its opacity/scale transitions — `ElementCompositionPreview::GetElementVisual`
   plus `Visual.Compositor()` — but that path is `pub(crate)` and single-purpose.

   This now ships as the [`CompositionHost`](../../crates/libs/reactor/src/widgets/composition_host.rs)
   widget (`composition_host()`), the composition-visual counterpart of the
   `SwapChainPanel` widget (which covers the DXGI-swap-chain case). Its
   `on_mounted` hands back a `CompositionHostHandle` — an opaque wrapper over the
   native element's `IInspectable`. To keep reactor free of a composition-wrapper
   dependency, the handle exposes only a **raw `IInspectable` seam**:
   `compositor_raw()` (`GetElementVisual` → `ICompositionObject::Compositor`),
   `set_child_visual_raw(&IInspectable)`
   (`SetElementChildVisual`), and `on_rasterization_scale_changed()`
   (via `IUIElement::XamlRoot` → `IXamlRoot::RasterizationScale`, default `1.0`).
   The `ElementCompositionPreview` plumbing stays in reactor (it owns the
   `Microsoft.UI.Xaml` bindings); the host is backed by a plain stretching `Grid`,
   so no new `ControlKind` or native control type is needed.

   The **safe, typed** composition API lives in the sibling
   [`windows-composition`](windows-composition.md) crate, whose single wrapper
   surface compiles against *either* the system stack (`system` feature, default)
   *or* the lifted stack (`lifted` feature). Enable reactor's `composition` feature
   (which turns on `windows-composition/lifted`) to get typed inherent methods on
   the handle — `host.compositor() -> Compositor` and
   `host.set_child_visual(&Visual)` over the raw seam — the same
   `Compositor`/`Visual`/`SpriteVisual`/… types used for standalone system hosting,
   so there is only **one** composition wrapper to learn and maintain (reactor no
   longer carries its own copy, and no extension trait import is needed). See the
   [`reactor/composition`](../../crates/samples/reactor/composition) samples
   (`circles`, `host`, `animation`, `dpi`, `toggle`) for the end-to-end flow.

   > **Dependency direction (flipped).** `windows-reactor` *optionally* depends on
   > `windows-composition` and `windows-canvas` behind its `composition` and `canvas`
   > features, so its handle returns typed `windows_composition::Compositor` /
   > `windows_composition::Visual` directly and the canvas/composition bridges live
   > here in reactor rather than in those crates. This replaced an earlier design
   > where the dependency ran the other way (`windows-composition[reactor]` →
   > `windows-reactor`) and callers imported a `CompositionHostExt` trait. The flip
   > lets `windows-composition`/`windows-canvas` drop their `reactor` feature and
   > raw-seam accessors, and makes reactor the single owner of the WinUI element
   > harness — at the cost of reactor optionally pulling in those two crates. The
   > mutually-exclusive composition-stack CI constraint is unaffected: a `system`
   > consumer and a `lifted` consumer still can never share one unified build (see
   > [`windows-composition.md`](windows-composition.md)).

6. **Templated list stays blank when it grows from empty *(fixed)*.** Previously,
   eager realization queued `Realize` requests for rows `0..count` only at *mount*,
   so a list that mounted empty (count 0) and later grew to N items stayed blank
   until a re-mount. This is now moot for `ListView`/`GridView`: they are driven by
   WinUI container recycling (see #7), so rows realize when their containers scroll
   into view regardless of when the data arrives. `update_templated_list`
   (`reconciler/templated.rs`) resizes the observable `ItemsSource` in place on a
   count change, so WinUI realizes/recycles the delta. `FlipView` (not a
   `ListViewBase`, so no recycling events) realizes every row from the reconciler —
   at mount and for any rows added on growth — so a 0→N `FlipView` also fills in
   without a re-mount.

7. **Templated lists aren't actually virtualized *(fixed)*.** `ListView`/`GridView`
   now use true scroll-driven WinUI virtualization instead of materializing every
   row. The backend (`backend/winui/mod.rs`) sets the control's `ItemsSource` to an
   observable vector of boxed `i32` indices `0..count` and subscribes
   `ContainerContentChanging`. WinUI lazily prepares only the containers on (and
   near) screen; each realize event records the container's content host and drives
   the reconciler's existing `realize(row)` callback, each recycle event clears it
   and drives `recycle(row)`. Because those events fire outside a render pass, the
   realize/recycle closures enqueue work and call `request_rerender`, so the queue
   drains on the next UI-thread frame (`request_render` coalesces a scroll burst into
   ~one reconcile per frame). The reconciler mounts the row subtree and hands the
   backend a `ControlId`, which the backend places into the container's content host.

   **Why the content host is a `ContentControl`, not the item container.** A
   `ListViewItem`/`GridViewItem` renders its content through a
   `ListViewItemPresenter`, which draws *string* content into its own internal
   `TextBlock` and cannot host an arbitrary `UIElement`. So each container is given a
   shared `ItemTemplate` — a `DataTemplate` whose root is a plain `ContentControl`
   (parsed once via `XamlReader`, reused across every list; the parse is off the
   per-item hot path). The realize event resolves that container's
   `ContentTemplateRoot` (the `ContentControl`) and the backend sets *its* `Content`
   to the reactor-built element, which a real `ContentControl` hosts correctly. The
   per-row cost is just a property set.

   **Reorder mirrors back into state.** Drag-reorder under virtualization reorders
   *containers*, not the live elements, so the app's data must be updated to match.
   `on_reorder` (backed by `DragItemsCompleted`) reads the reordered boxed indices
   back as a `Vec<usize>` permutation and invokes the app callback; the app reorders
   its own state (see the `list_view`/`grid_view`/`virtual_list` samples). The
   backend then resets the source to identity so the next drag reads cleanly. The
   former `eager_templated_realization` flag is removed — `FlipView` realizes all
   rows unconditionally, `ListView`/`GridView` never do.

   Verified end-to-end by the `Reconciler_Mount_VirtualList` self-test fixture: a
   300-row `ListView` realizes its first row and keeps the realized set bounded well
   below the total, proving containers are recycled rather than fully materialized.

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

- **`VirtualList`** — a higher-level convenience wrapper for large collections.
  Raw `list_view`/`grid_view` now do true UI virtualization (only on-screen
  containers are realized; see "Known gaps and fixes" #7), so this would add
  *data* virtualization / incremental loading on top (windowing a data source of
  millions of rows), not UI virtualization.
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

> **Consolidate the internal composition slice.** Reactor's transition/animation
> engine (`backend/winui/mod.rs`) drives *lifted* `Microsoft.UI.Composition` types
> directly — `ElementCompositionPreview.GetElementVisual`, implicit-animation
> collections, easing functions, and expression key frames — so `tool_reactor`'s
> [`base.txt`](../../crates/tools/reactor/src/base.txt) carries its own minimal
> slice of `Microsoft::UI::Composition::*` bindings (Compositor, Visual,
> KeyFrameAnimation, …). This is *not* the public composition surface — that lives
> in [`windows-composition`](windows-composition.md) and is hosted through this
> crate's `CompositionHost` typed methods — and it uses several primitives
> `windows-composition` doesn't wrap at all (implicit animations, easing
> functions, expression key frames), so the two only partially overlap. Now that the
> dependency direction is flipped (reactor optionally depends on
> `windows-composition`), reactor *could* reuse `windows-composition`'s wrappers for
> the overlapping types and keep only the animation-engine-specific extras in its own
> slice. The catch: the internal engine is compiled unconditionally, whereas the
> `composition` dependency is *optional*, so consolidating would mean either making
> `windows-composition` a mandatory reactor dependency or widening its public surface
> to cover the engine-only primitives. Worth doing only if that cost is judged
> acceptable against the duplicated `base.txt` entries it removes.

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
