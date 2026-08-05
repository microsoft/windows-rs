# windows-reactor

> A declarative, React-style UI library for Rust, backed by WinUI 3.

- 📦 [crates.io](https://crates.io/crates/windows-reactor)
- 📖 [docs.rs](https://docs.rs/windows-reactor)
- 🚀 [Getting started](../../crates/libs/reactor/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)

`windows-reactor` lets you describe a WinUI 3 user interface as a function of state. You write a
render function that takes a `RenderCx` and returns an `Element`. The reactor diffs the result
against the live visual tree and applies only the changes. State lives in hooks such as
`cx.use_state`. Updating state schedules a re-render.

## Getting started

A reactor app needs three things: the crate dependency, a render function, and a `build.rs` that
stages the Windows App SDK runtime with [`windows-reactor-setup`](windows-reactor-setup.md).

`Cargo.toml`:

```toml
[dependencies]
windows-reactor = "0.100"

[build-dependencies]
windows-reactor-setup = "0.100"
```

`build.rs`. Pick the helper that matches your deployment model:

The build script selects `as_self_contained`, `as_framework_dependent`, or `as_example`.

`src/main.rs`. A render function plus `App`:

The crate readme contains a checked counter example. The sample applications cover packaging,
layout, events, and state.

For a framework-dependent app, `bootstrap()` initializes the Windows App SDK runtime and must be
called once at startup. A self-contained app does not call it. `App::new()` is a builder. Common
options are `title`, `inner_size`, `backdrop` (for example `Backdrop::Mica`), `icon` (path to an
`.ico` file), `fullscreen`, and `presenter`. `render(app)` takes your
`Fn(&mut RenderCx) -> Element` and runs the message loop.

Reactor catches panics at the FFI boundaries it owns (render and event callbacks, and
`ErrorBoundary`) and converts them to errors, so they never unwind across the WinUI ABI. It does not
install a global panic hook. For panics that escape those boundaries, add `panic = "abort"` to your
release profile. The process then terminates cleanly instead of unwinding into WinUI's C++ frames,
which is undefined behavior:

```toml
[profile.release]
panic = "abort"
```

Set `RUST_BACKTRACE=1` when you want a backtrace.

## State with hooks

Hooks are methods on `RenderCx`. They give a render function persistent state without globals or
`thread_local!`. The most common hooks:

- `use_state(initial)` returns `(value, SetState)`: a value plus a setter. `set.call(new_value)`
  updates the slot and schedules a re-render.
- `use_ref(initial)` returns `HookRef`: mutable storage that does not trigger a re-render. Read with
  `.borrow()`, write with `.borrow_mut()` or `.set(v)`. Use it for animation frame counters and
  cached resources.
- `use_memo(deps, factory)`: recompute a value only when `deps` change.
- `use_effect(deps, f)` and `use_effect_with_cleanup`: run side effects when `deps` change.
- `use_reducer` and `use_reducer_fn`: state driven by an update or reducer instead of a plain
  setter.
- `use_resource(fetcher, deps)` returns `Resource<T>`: async data loading with loading and error
  states. A `Resource` converts into an `Element`.
- `use_context(&context)`: read a value provided higher in the tree.
- `use_open_window()`: returns an opener for secondary top-level windows (see [Multiple
  windows](#multiple-windows)).

Render functions receive `&mut RenderCx` and return `Element`.

The `apps/examples` and `minimal/examples` directories include a focused sample for each hook
(`use_state`, `use_ref`, `use_memo`, `use_effect`, `use_reducer`, `use_resource`, `use_callback`,
`use_color_scheme`, and more).

## Building the UI

Build elements with plain builder functions. Each returns a widget that becomes an `Element` with
`.into()`. Containers take a tuple of children.

- Text: `text_block(content)` with `.bold()`, `.semibold()`, `.font_size(..)`, `.wrap()`,
  `.selectable()`, and type-ramp helpers (`title`, `subtitle`, `body`, `caption`).
- Buttons: `button(content)` with `.on_click(..)`, `.accent()`, `.subtle()`, `.enabled(..)`,
  `.icon(..)`, `.flyout(..)`, `.menu_flyout(..)`.
- Icons: any control that takes an icon (`button`, `NavViewItem`, command-bar buttons,
  `selector_bar_item`) accepts `impl Into<Icon>`. A bare `Symbol` creates a `SymbolIcon`;
  `Icon::image(source)` creates a full-color `ImageIcon` from raster, SVG, or surface data;
  `Icon::bitmap_icon(uri, show_as_monochrome)` creates a native `BitmapIcon`; `Icon::font(glyph)`
  and `Icon::font_family(glyph, family)` create a `FontIcon`; and `Icon::path(data)` creates a
  `PathIcon` from XAML path mini-language data. Sample:
  `cargo run -p reactor_samples --example icon_elements`.
- Images: `Image::new(source)` accepts a URI or `ImageSource`. URI paths ending in `.svg`
  (case-insensitive, before any query or fragment) use the platform SVG decoder; other URIs use
  the bitmap decoder. Sample: `cargo run -p reactor_samples --example image`.
- Layout: `vstack((..))` and `hstack((..))` with `.spacing(..)`; `grid((..))` with `.rows([..])` and
  `.columns([..])` (using `GridLength::STAR` and `GridLength::Auto`) and per-child `.grid_row(n)`
  and `.grid_column(n)`.

About 60 WinUI controls are wrapped, including `check_box`, `combo_box`, `slider`, `list_view`,
`tree_view`, `navigation_view`, `tab_view`, `pivot`, `text_box`, `number_box`, `color_picker`,
`calendar_view`, `content_dialog`, `info_bar`, `teaching_tip`, and `command_bar`. See the [full
catalog](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor/src/widgets).

`TabItem::with_key` supplies the stable identity returned by `TabView::on_close_requested`. Adding,
changing, or removing that key updates the existing native `TabViewItem`; removing it also clears
the WinUI `Tag` instead of retaining stale callback identity. See the `tab_view_item_key` sample.

Layout and appearance modifiers are available on any `Element` through the `ElementExt` trait:
`.margin(..)`, `.padding(..)`, `.width(..)`, `.height(..)`, `.horizontal_alignment(..)`,
`.vertical_alignment(..)` (with `HorizontalAlignment` and `VerticalAlignment`), `.background(..)`,
`.foreground(..)`, `.opacity(..)`, and transition helpers such as `.with_opacity_transition(..)`.
Spacing values use `Thickness` (with `Thickness::uniform(..)`).

`transition(enter, exit)` runs lifecycle animations when an element enters or leaves the WinUI
visual tree. The logical Reactor element is removed synchronously; WinUI Composition retains its
departing visual until the implicit hide animation finishes. This keeps keyed and positional child
indices exact during reconciliation instead of reinserting a temporary "ghost" element. Opacity
and scale are supported, including both in one animation group:

```rust
# use std::time::Duration;
# use windows_reactor::*;
button("Animated")
    .transition(
        Some(AnimationConfig::fade_in(Duration::from_millis(200))),
        Some(AnimationConfig::fade_out(Duration::from_millis(300))),
    );
```

An explicit `.animate(..)` on the same element takes precedence over its enter transition because
both target the initial property animation. The exit transition remains registered. See the
`exit_transition` sample.

## Handling events

Event handlers take closures. `button(..).on_click(move || ...)` is the most common. Pointer and
keyboard handlers live on `ElementExt`: `.on_tapped(..)`, `.on_pointer_pressed(..)`,
`.on_pointer_released(..)`, `.on_pointer_moved(..)`, `.on_pointer_entered(..)`,
`.on_pointer_exited(..)`, `.keyboard_accelerator(..)`. You can pass a `SetState` or `Dispatch`
directly wherever a handler is expected (through `IntoCallback`).

`PointerEventInfo::x` and `y` are relative to the element receiving the event. `window_x` and
`window_y` are relative to the overall window, so drag deltas remain stable when the element moves
while handling the gesture. A moving drag handle can also lose hit testing when the cursor outruns
layout. Add `.capture_pointer_on_press()` to keep receiving events until release, begin the drag
only when `PointerEventInfo::capture_succeeded` is true, and clear drag state from
`.on_pointer_capture_lost(..)` and `.on_pointer_canceled(..)`. See the `pointer_resize` sample.

`NavigationView::on_pane_open_changed` reports the settled `IsPaneOpen` property, including
light-dismiss and adaptive changes made by WinUI. `on_display_mode_changed` reports the actual
`NavigationViewDisplayMode` (`Minimal`, `Compact`, or `Expanded`), while `pane_display_mode`
continues to configure the layout policy (`Auto`, `Left`, `Top`, and so on):

```rust
# use windows_reactor::*;
# let items = [NavViewItem::new("Home")];
# let content = text_block("Content");
# let pane_open = true;
NavigationView::new(items, content)
    .pane_open(pane_open)
    .on_pane_open_changed(|_| {})
    .pane_display_mode(NavigationViewPaneDisplayMode::Auto)
    .on_display_mode_changed(|_| {});
```

The callbacks observe dependency properties rather than guessing state from pane transition
events. Transition events can be canceled and do not cover every property change. Observers are
attached only for callbacks the element requests, so an unused callback adds no native observer or
reconciliation work. See the `responsive_navigation` sample.

### Handler identity

When a value-carrying event just forwards its argument to a setter, pass the setter directly instead
of wrapping it in a closure:

Pass a state setter directly when its argument type matches the event.

This is not only cosmetic. Setters from `use_state` and `use_reducer` are memoized per hook slot.
Passing one straight through hands the reconciler the same handler identity each render, so the diff
can skip the whole control. An inline closure (`move |v| set.call(v)`) allocates a fresh identity
every render, so the control is always re-diffed and its WinUI event re-bound.

When a handler must compute a value or run extra logic, wrap it in `cx.use_callback(deps, ...)` to
memoize it and recover a stable identity for hot paths. For a unit event (`on_click`) that stores a
fixed or pre-computed value, `SetState::setter(value)` is shorthand for `move || set.call(value)`:
`button("Reset").on_click(set_count.setter(0))`.

## Multiple windows

`App::run` opens the primary window. An app can open more top-level windows at runtime with the
`ReactorWindow` builder. Each window hosts its own reactor tree with its own hooks, state, and
render function, while sharing the one UI thread and message loop. WinUI is single-threaded
apartment, so every window runs on the same thread.

`ReactorWindow` opens another top-level window from an event handler.

`ReactorWindow` mirrors the `App` window options (`title`, `inner_size`, `inner_constraints`,
`presenter`, `fullscreen`, `backdrop`, `icon`). `.render(f)` takes a `Fn(&mut RenderCx) -> Element`.
`.open(factory)` takes any `Component`. Both run synchronously on the current UI thread (unlike
`App::run` there is no `Send` bound) and return `Result<WindowHandle>`.

- `WindowHandle` is a control handle for an open window. The registry owns the window's host, so the
  handle is just an identifier. Call `.close()` to close the window. Dropping the handle does
  nothing and never affects the window's lifetime. The handle is `!Send` and `!Sync`: you can only
  control a WinUI window from the UI thread that opened it.
- Closing the last window exits. Reactor tracks every open window. When the last one closes (primary
  or secondary), the process exits. Closing any earlier window just drops that window.
- `cx.use_open_window()` returns a small `Copy` opener you can capture into handlers as an
  alternative to naming `ReactorWindow` directly. `opener.render(f)` and `opener.open(factory)` open
  a default-configured window.

Per-window themes are not available yet. The requested color scheme is app-global.

## Graphics integration

For custom 2D drawing, host a [`windows-canvas`](windows-canvas.md) surface with
`animated_canvas(draw)`. Enable reactor's `canvas` feature, which pulls in `windows-canvas`. It
returns a `SwapChainPanel` element that redraws every frame and recovers from device loss
automatically. See the `canvas` samples. To render on a device the app already created and shares
across many surfaces, use `animated_canvas_with_device(device, draw)` with a cloneable
`windows_canvas::GpuDevice` (a clone shares the same underlying devices).

For content that changes with its size rather than every frame, `canvas(draw)` is the demand-driven
counterpart. It manages the device, swap chain, resize, DPI, and device loss just like
`animated_canvas`, but calls `draw` only on the first layout and on resize or scale change, staying
idle otherwise. Use it for text, charts, or diagrams. See the canvas `text_layout` sample.

When the content changes with app state rather than size, `canvas_invalidated(&inv, draw)` adds
explicit repaint control. Get a stable `Invalidator` from `cx.use_invalidator()`, keep the drawing
state in a `use_ref`, mutate it in an event handler, then call `inv.invalidate()` to schedule one
repaint. Mutating a `use_ref` does not reconcile the tree, so nothing runs between changes - the
cheapest way to drive interactive or data-driven drawing. See the canvas `invalidate`, `editor`, and
`hit_test` samples.

For a surface hosted in an `Image` widget rather than a `SwapChainPanel`, `CanvasImageSource` draws
on demand into a `SurfaceImageSource` and redraws only when you call `draw`. `Image::on_mounted`
yields an `ImageHandle` whose `on_rasterization_scale_changed` reports the host DPI scale, so the
surface stays crisp across monitor moves. See the canvas `image_source` sample.

For an on-demand surface that still presents through a swap chain (lower latency than a
`SurfaceImageSource`, but only when the data changes), `CanvasSwapChain` hosts a composition swap
chain on a `SwapChainPanel`. Create it in the panel's `on_mounted`, store it in `use_ref`, and
`draw` from a `use_effect` on data change. It stays idle when nothing changes. See the canvas
`chart` sample.

`animated_canvas`, `canvas`, `CanvasImageSource`, and `CanvasSwapChain` are reactor exports. They
own the WinUI element harness and build on the safe drawing surface that `windows-canvas` provides.
For raw
Direct3D, the `swap_chain_panel` sample drives a `SwapChainPanel` with `on_rendering`.

## Web content integration

To host a browser, use [`windows-webview`](windows-webview.md)'s `webview(on_ready)` and enable its
`reactor` feature. It returns a `WebView2` element backed by the WinUI XAML `WebView2` control, and
hands you a ready `WebView` once the browser initializes. See the `reactor/webview` sample. The
`as_self_contained()` setup carries the required `Microsoft.Web.WebView2.Core.dll` automatically.
For how the widget bridges the WinRT control to the COM `ICoreWebView2`, see
[`windows-webview`](windows-webview.md).

## Samples

The [`crates/samples/reactor`](../../crates/samples/reactor) tree is the best reference:

- `samples`: the smallest app plus an `examples/` folder with about 90 focused per-control and
  per-hook examples (`counter`, `calculator`, `navigation_view`, `list_view`, `content_dialog`,
  `keyed_list_reorder`, `lightweight_resources`, `pointer_resize`, `color_picker`,
  `secondary_window`, and more).
- `apps`: complete applications (`notepad`, `solitaire`, `minesweeper`, `tictactoe`, `dotsweeper`).
- `gallery`: a WinUI-gallery-style shell with navigation across many controls.
- `direct2d` and `swap_chain_panel`: hosting Direct2D and Direct3D content.
- `webview`: hosting a WebView2 browser through `windows-webview`'s `reactor` feature.
- `framework_dependent` and `self_contained`: the two deployment models, which differ only in
  `build.rs`.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is for contributors and is
not needed to use `windows-reactor`.

### How it's built

The hooks runtime, element tree, reconciler, and WinUI backend are hand-written. `tool_reactor`
generates the per-widget dispatch from `crates/tools/reactor/src/winui.toml` plus the WinUI `.winmd`
metadata:

| Generated file | Contents |
|----------------|----------|
| `src/generated.rs` | per-widget `bindings()` helpers |
| `src/backend/winui/generated_set_prop.rs` | property setter dispatch |
| `src/backend/winui/generated_attach_event.rs` | event handler dispatch |
| `crates/tools/reactor/src/generated.txt` | binding filter entries |

The tool is metadata-driven: it infers the setter pattern, value type, and event-invoke pattern from
`.winmd`. TOML keys are WinUI metadata names, and only non-standard mappings need overrides.
Regenerate with `cargo run -p tool_reactor`, then verify with `cargo check -p windows-reactor`.

The WinUI and Windows App SDK `.winmd` files under `crates/tools/reactor/winmd/` are committed (they
are also read by `tool_webview` and `tool_composition`) but treated as generated. `tool_reactor`
refreshes them on every run from the pinned `WINDOWS_APP_SDK_VERSION`: it downloads the
`Microsoft.WindowsAppSDK` metapackage, resolves the component versions from its nuspec, and copies
each component's winmd (plus the WebView2 `Core.winmd` at `tool_webview`'s pin) into place. Bumping
that one constant updates them all, and `gen.yml`'s zero-diff check catches any drift. See
[Dependencies](../dependencies.md).

Generated dispatch falls through to hand-written backend code for cases too complex to express
declaratively (Button icon and text layout, NavigationView menu items, ContentDialog modal popup).
Never edit the generated files or `generated.txt` by hand.

### Bindings

`src/bindings.rs` holds the flat WinUI and COM interop bindings the hand-written backend calls.
`tool_reactor` generates it (driving `windows-bindgen` in `--flat --minimal` mode) from two filter
files: the hand-maintained `crates/tools/reactor/src/base.txt` (the base-interface method list) and
the tool-produced `generated.txt` (per-widget entries). Regenerate it, and the selftest copy at
`crates/tests/libs/reactor_selftest/src/bindings.rs`, with the same `cargo run -p tool_reactor`. Raw
metadata names apply: `get_Prop`, `put_Prop`, `add_Event`, `remove_Event`.

To prune or extend bindings: edit `base.txt`, regenerate, and let the compiler errors show the
methods you still need (`SetX` maps to `put_X`, `X()` maps to `get_X`). Add them as
`Ns.IFace::{put_X, get_Y}` under the right base interface. This also covers the Win32 COM interop
interfaces (for example `ISwapChainPanelNative`, `ISurfaceImageSourceNativeWithD2D`). Listed methods
get full vtable entries, unlisted methods become `usize` slots, and the type closure is computed
automatically.

### COM pitfalls

These bite anyone editing the backend by hand. The generated code already follows them.

- Classes deref to their default interface. Do not `cast` to it. Call the method directly
  (`button.SetFlyout(&flyout)`, not `button.cast::<IButton>()?.SetFlyout(...)`). This applies to
  event-handler `sender` and `args` too: the delegate hands you the concrete arg class and the
  `sender` is the control, and both already deref to their default interface. So
  `args.SelectedItem()` and a control captured at attach (`let control = h.clone();`) read at zero
  per-event QI, versus `args.cast::<I...Args>()` or `sender.cast::<TextBox>()` on every fire. Only
  cast to non-default parent interfaces (for example `Button` to `IContentControl` or `IControl`).
  Watch the static type, not the name: `DropDownButton.cast::<IButton>()` looks redundant, but
  `IButton` is a parent there (the default is `IDropDownButton`), so it is a genuine cast.
- `Param<T>` removes parent-class casts. A method taking `impl Param<Brush>` accepts a
  `SolidColorBrush` directly, with no `cast::<Brush>()`.
- Use `From` or `into()`, not `cast`, for `IInspectable`, and plain `None` for optional inspectable
  parameters.
- `put_IsChecked` (CheckBox) takes `Option<bool>`. It is a tri-state nullable boolean.
- TextBox and PasswordBox need get-before-set to avoid resetting the caret.
- ProgressBar uses `IRangeBase` for Value, Min, and Max. ProgressRing has direct setters.
- ContentDialog needs a `XamlRoot` from a live element, so it requires backend access.
- Font properties are shared across `IControl`, `ITextBlock`, and `IRichTextBlock`.

### Padding, background, and foreground dispatch

`Padding` has no single owning interface. `Control`, `Border`, `StackPanel`, `TextBlock`, and
`RichTextBlock` each declare their own. `set_padding` (`backend/winui/mod.rs`) dispatches on the
`Handle` variant: it calls the setter directly on `Border`, `StackPanel`, `TextBlock`, and
`RichTextBlock` through their default interface, and falls back to a single `IControl` cast for
everything else. Containers that lack a `Padding` property (for example a bare `Panel` or `Grid`)
fall through to `diag::unhandled_modifier`, which warns in debug builds. Use `.margin(...)` there
instead.

`Background` and `Foreground` follow the same pattern and are exposed as the `ElementExt` modifiers
`.background(...)` and `.foreground(...)`. `Border` handles them through its default interface;
every other handle falls back to a single `IControl` cast (`set_background` and `set_foreground`).
`BorderBrush` and `BorderThickness` use the same `IControl` fallback (`set_border_brush` and
`set_border_thickness`) but are not `ElementExt` modifiers. They are opt-in per-widget builders,
currently exposed by `Border` and `TextBox`.

### Threading

Reactor runs on a WinUI STA thread and keeps per-thread state in `thread_local!` slots. Two
categories exist:

- STA-affine COM handles and caches (the host, application, root window and framework element, and
  the shared `DataTemplate`) must stay thread-local. They hold COM objects that are only valid on
  the UI thread.
- One-shot latches and per-thread scalars (pending theme and title-bar requests, current color
  scheme) are thread-local only because the public API exposes them as free functions
  (`set_requested_theme` and similar).

### Error model

Reactor sits between the developer's Rust closures and WinUI's COM and `extern "system"` delegates.
It handles failures by where they happen:

- Synchronous, pre-loop setup (`bootstrap`, icon path validation) returns a `Result` from `run` or
  `bootstrap`. This is the only place a `Result` reaches the caller, so validate configuration up
  front.
- Failures inside UI-thread callbacks (render, event handlers, timers, `on_rendering`) go to one
  reactor-owned fault boundary, not a `Result`. Reactor catches panics at the entry points it owns
  (`Callback::invoke`, the `DispatcherTimer` tick, `on_rendering`, and the render pass) and delivers
  them to a developer-supplied `App::on_fault(|fault| ...)` hook (default: log-and-continue). The
  catch is context-aware: a callback that panics during a render pass is left to propagate so
  `error_boundary` can recover the subtree first. Only panics outside render, or escaping every
  boundary, reach `on_fault`. This lives in `fault.rs`.
- Best-effort backend property application uses one helper: `diag::warn` (with `diag::dropped`,
  which reports the dropped `Result`'s call site through `#[track_caller]`). It warns in debug and
  is a no-op in release.
- `panic!` is only for programmer errors and invariant violations (rules of hooks, type mismatch,
  `EventHandler` variant mismatch) in `engine.rs` and `backend/mod.rs`.

The fault boundary relies on `panic = "unwind"` (the Cargo default). Under a `panic = "abort"`
profile the whole model is bypassed and every panic aborts.

### Performance notes

The reconciler skips unchanged controls (kind match plus shallow compare), so at steady state it
creates no new WinUI controls. The diff and patch cost is then dominated by COM property-set calls.
Two design choices follow:

- No element pooling. With zero controls created at steady state there is nothing to recycle.
- No rerender depth guard. The render loop is non-recursive: `set_state` during a render sets a
  dirty flag and enqueues the follow-up render through the dispatcher rather than re-entering, so
  unbounded recursion is impossible.

State writes are coalesced through the dispatcher, so many `set_state` calls in one turn produce a
single render. Steady-state heap allocations per render are zero for `use_state`, `use_reducer`,
`use_callback`, and `use_ref`.

Each control is held as a `Handle` enum whose variant is the concrete WinUI class
(`Handle::TextBlock(bindings::TextBlock)`). Shared modifiers (`padding`, `foreground`, `font_*`)
match on the `Handle` variant instead of probing interfaces with `cast`, because a class derefs to
its default interface at zero QI and a failing `QueryInterface` on XAML's aggregated objects is
expensive. The same rule applies to event handlers: capture the typed handle at attach and read
through `Deref` rather than casting `sender` or `args` on every fire.

### Testing

Unit tests live in `test_reactor` (headless). Integration tests live in `test_reactor_selftest`,
which launches a real WinUI window. Pass `--headless` for CI.

Most pointer handler behavior (attach, detach, memoization, slot changes) is covered headlessly in
`test_reactor` through the `RecordingBackend`. Paths that need a live WinUI thread are covered by
selftest fixtures instead:

- `Pointer_Injection_Gesture` drives real OS mouse input through the WinRT `InputInjector` and
  asserts the `on_pointer_*` callbacks fire with the right position and button flags.
- `Timer_*` and `Rendering_Subscription_*` cover `DispatcherTimer` and the
  `CompositionTarget::Rendering` subscription (`on_rendering`).

Fixtures that need real OS input or composition frames record a TAP `# SKIP` (never a failure) when
the host cannot deliver them, so they do not flake CI.

The `RecordingBackend` harness lives in the `test_reactor` crate, not in `windows-reactor`, so it
adds no weight to normal builds. Do not put `#[cfg(test)]` modules inside the published library
crates. Put the test in the matching `test_*` crate. If it needs an internal item, expose that item
behind the existing `test` feature that the test crates enable and published builds leave off.

Two crates measure reconciler performance. `test_reactor_bench` is a headless micro-suite (run with
`cargo run -p test_reactor_bench --release`) that brackets only the reconcile body against
`RecordingBackend` and reports ns/op plus bytes/op and allocs/op - the right instrument for
Rust-side reconciler cost (structural skip, keyed diff, allocation). `test_reactor_perf` is a live
WinUI stress app (a 70x70 stock grid) whose `--churn` flag additionally forces native control
create/destroy. The "Future considerations" section below uses both to evaluate three C# perf
techniques.

### Reactor and canvas naming

`windows-reactor` and `windows-canvas` define some of the same short names for different domains.
The rule: canvas keeps the short name (it owns user-facing draw loops) and reactor takes a
domain-prefixed alternative.

## Future considerations: learnings from Microsoft.UI.Reactor (C#)

This section records a comparison against the C# sibling framework Microsoft.UI.Reactor, captured
for later evaluation. It is not a committed plan. The C# project is roughly five times larger by
line count, but most of that lives in optional feature areas (charting, docking, markdown, a Yoga
flex-layout port, a data grid, a CLI, Roslyn analyzers, devtools). The core reconciler, hooks
runtime, and DSL are close in design to this crate's, and in a few places this crate is already
ahead.

### What the Rust core already does (do not reinvent)

| Capability | Rust location | C# equivalent |
| --- | --- | --- |
| Keyed diff with LIS move-minimization | `reconciler/child.rs` (`reconcile_keyed_middle`, `compute_lis`) | `ChildReconciler.RunKeyedMiddleCore` |
| Common prefix/suffix strip before the middle diff | `reconciler/child.rs` (`reconcile_keyed_live`) | `ChildReconciler.ReconcileKeyed` |
| Structural skip-unchanged with theme and dirty gates | `element.rs` (`can_skip_update`), `reconciler.rs` (`update`) | `Element.CanSkipUpdate`, `ChildDiffHints` |
| Render coalescing across many state writes | `engine.rs` (`request_render`) | delegated to the caller's `requestRerender` in Core |
| List virtualization with row recycling | `reconciler/templated.rs` (realize/recycle) | virtualized list controls |
| Accessibility modifiers, theme tokens, implicit transitions, error boundaries, FFI panic capture | `element.rs`, `style.rs`, `fault.rs` | equivalents present |
| Off-thread state writes | `engine.rs` (`use_async_state`, `AsyncSetState`, `UiMarshaller`); sync `SetState`/`Updater`/`Dispatch` are `!Send` | `RenderContext.MarshalIfOffUIThread` |
| Effect/memo dependency comparison | `engine.rs` (generic `Deps`/`PartialEq` over tuples and arrays) | `RenderContext.UseEffect<T1>` `params object[]` special-case |

The render-coalescing case is worth calling out: the C# Core has no dedicated batch queue and
invokes `requestRerender` synchronously, leaving batching to the host. This crate's
dispatcher-scheduled coalescing in `request_render` is the cleaner design and should stay.

Two core behaviors were re-checked against the C# runtime and are non-gaps where Rust's type system
does the work C# does at runtime. **Off-thread state writes**: C#'s `MarshalIfOffUIThread` inspects
the calling thread on every `UseState`/`UseReducer` write and either marshals to the dispatcher or
throws. Rust splits this statically - the synchronous `SetState`/`Updater`/`Dispatch` capture
`Rc<RefCell<..>>` and are therefore `!Send`, so moving one to another thread is a compile error, and
the legitimate off-thread path is the explicit `use_async_state` -> `AsyncSetState` (backed by
`Arc<Mutex<..>>`, `Send`) that auto-marshals through the host's `UiMarshaller`. The common path pays
no per-write thread check. **Single-array dependencies**: C#'s `UseEffect<T1>`/`UseMemo`/`UseCallback`
special-case a lone `object[]` dependency to compare it element-wise, working around `params object[]`
overload ambiguity. Rust's `Deps`/`PartialEq` over tuples and arrays already compares element-wise
with no such ambiguity, so no special case is needed.

### Candidate performance optimizations (evaluated - see measured findings below)

The C# framework carries three allocation- and object-reuse techniques this crate does not:
per-type native control pooling (`ElementPool`), a per-render changed-index hint table
(`ChildDiffHints`), and `ArrayPool`-rented scratch buffers in the keyed diff. They are listed here
as candidates, not recommendations. Much of their value in C# comes from fighting managed GC
pressure and expensive managed allocation - constraints this crate does not share. Rust builds the
virtual `Element` tree, the `FxHashMap`/`Vec` scratch used by the keyed diff, and per-render
closures with a fast allocator and no GC, so a technique that pays for itself in C# can be net
negative here once its bookkeeping and branching are added.

Each was treated as a hypothesis to be proven, not adopted. The two instruments used are
`test_reactor_bench` (a headless reconciler micro-suite ported from C#'s `PerfBench.ControlModel` -
it brackets only the reconcile body against `RecordingBackend` and reports ns/op plus bytes/op and
allocs/op from a counting global allocator) and a `--churn` scenario added to `test_reactor_perf`
(which drives real WinUI controls, so it is the only instrument that measures native control
create/destroy cost). Neither backend inflates the other's blind spot: the headless bench never
touches WinUI, and the churn scenario isolates the native path the headless bench cannot see.

| Candidate | Mechanism | Why it may not help in Rust | What would have to be proven |
| --- | --- | --- | --- |
| Native control pooling | Keep a per-type stack of released WinUI controls and rent on mount instead of destroying and recreating. | The pooled object is the native XAML/COM control, whose creation cost is language-independent - but keyed diffing already reuses and moves controls, so churn only occurs on kind-mismatch remounts (`update`'s `!kind_matches` arm) and unkeyed list shrink/grow. Steady-state and keyed-reorder workloads never destroy same-typed controls, so there is nothing to pool. Reset discipline (clearing every mutated property on return) is also a known source of stale-state bugs. | A churn-heavy scenario (repeated add/remove of same-typed controls, or frequent kind-mismatch swaps) where native control creation dominates the frame, and pooling removes it, with no stale-state regressions in the selftest suite. |
| Changed-index diff hints | Carry a hint on a container recording which child indices changed, so a render skips straight to changed children instead of running `can_skip_update` on every sibling. | `can_skip_update` is already a discriminant check plus a shallow field compare with early-out, not a deep walk. The hint only pays off when the same child `Vec` survives across renders by reference, but render functions here rebuild their child vectors every render by design, so there is usually no stable identity to key a hint on. The idiomatic escape hatch already exists: memoize a subtree with a `Component` and `should_update`, or `use_memo`. | A large stable-list scenario (for example the 70x70 grid) where per-tick `can_skip_update` calls measurably dominate, and a hint or memoized subtree removes that cost below what `should_update` memoization already achieves. |
| Scratch-buffer reuse in keyed diff | Reuse the `FxHashMap`, `Vec<i32>`, `Vec<bool>`, and LIS `FxHashSet` allocated by `reconcile_keyed_middle` across reconciles. | The positional fast path (`reconcile_positional_live`) allocates none of these, and the keyed middle only allocates when keys are present and the prefix/suffix strip does not cover the whole list. Rust allocation of these small structures is cheap, so reuse mostly trades a fast allocation for permanent retained memory and added lifetime complexity. | A large keyed-reorder scenario where these allocations show up in a profile as a real share of reconcile time, and a reused scratch arena cuts it without inflating idle working set. |

A caveat for all three: the current `test_reactor_perf` steady-state scenario is a positional 70x70
stock grid that updates a `--percent` share of cells per tick. It exercises
`reconcile_positional_live` and `can_skip_update` but never the keyed middle diff or control churn.
The evaluation below therefore adds the missing coverage - the headless keyed benches and the
`--churn` scenario - and reports what the numbers say.

### Measured findings

Numbers below are from `test_reactor_bench` (release, headless) and `test_reactor_perf --headless
--percent 0` with and without `--churn`, on one development machine. Absolute values are
machine-specific; the ratios and scaling are the point. Reproduce with `cargo run -p
test_reactor_bench --release` and `cargo run -p test_reactor_perf --release -- --headless --percent
0 --churn --churn-count 400`.

Headless micro-suite (ns/op is best-of-reps; bytes/op and allocs/op from the counting allocator):

| bench | N | ns/op | bytes/op | allocs/op | skip | diff | crt |
| --- | --- | --- | --- | --- | --- | --- | --- |
| mount_unmount | 64 | 20,598 | 18,076 | 281 | 0 | 0 | 65 |
| mount_unmount | 512 | 156,634 | 144,340 | 2,085 | 0 | 0 | 513 |
| update_1_changed | 64 | 2,265 | 658 | 7 | 63 | 2 | 0 |
| update_1_changed | 512 | 14,213 | 658 | 7 | 511 | 2 | 0 |
| update_1_changed | 4096 | 125,631 | 658 | 7 | 4095 | 2 | 0 |
| update_all_changed | 512 | 146,972 | 202,678 | 2,562 | 0 | 513 | 0 |
| update_no_change | 512 | 7,060 | 0 | 0 | 1 | 0 | 0 |
| keyed_reverse | 64 | 6,640 | 7,620 | 11 | 64 | 1 | 0 |
| keyed_reverse | 512 | 94,238 | 58,244 | 11 | 512 | 1 | 0 |
| keyed_rotate1 | 512 | 100,835 | 82,828 | 33 | 512 | 1 | 0 |

WinUI churn scenario (native control create/destroy, averaged over both add and remove ticks):

| scenario | churn/tick | avg created/render | Avg Diff | Avg Reconcile | Avg FPS |
| --- | --- | --- | --- | --- | --- |
| no churn | 0 | 0 | 1.0 ms | 2.0 ms | 58.2 |
| churn 400 | 400 | 199 | 9.3 ms | 10.2 ms | 57.0 |
| churn 800 | 800 | 400 | 16.7 ms | 17.6 ms | 43.7 |

**Diff hints - do not add.** `update_1_changed` isolates the per-sibling skip-walk: one changed
leaf among N children forces N-1 `can_skip_update` calls. The per-skip cost is only about 27-31 ns
((125,631 - 14,213) / (4095 - 511)), and the walk allocates nothing (bytes/op is a constant 658 -
just the one changed leaf, independent of N). More important, a changed-index hint cannot remove
the dominant half of that cost: `update_no_change` shows the root `can_skip_update` deep-equality
compare alone is about 7 us for 512 nodes, and a single-leaf change pays that 7 us to fail the root
compare *plus* about 7 us for the child walk (14.2 us total). A per-child hint would only skip the
second half, and only if the render layer produced the hint cheaply - but render functions rebuild
their child vectors every render by design, so there is no stable identity to hang a hint on. The
existing escape hatch (`Component` + `should_update`, or `use_memo`) already collapses an unchanged
subtree to the single O(1) root compare. As churn rises the skip share vanishes entirely
(`update_all_changed`: 0 skips). Ceiling is roughly 2x on the sparsest possible update, with no
allocation win and real added complexity. Not worth it.

**Scratch-buffer reuse in the keyed diff - do not add.** The keyed benches take the full key-map +
LIS path every op, yet allocate only 11-33 times per reconcile (constant in count - `keyed_reverse`
is 11 allocs whether N is 64 or 512; only the sizes grow). At 94-101 us/op the allocation is a small
fraction of reconcile time, and the positional fast path allocates none of it. With no GC, a
retained scratch arena would trade a handful of cheap allocations for permanent working-set and
added lifetime complexity, saving microseconds at most. Not worth it.

**Native control pooling - defer, revisit only for churn-heavy non-virtualized UIs.** This is the
only candidate with a real cost signal. `mount_unmount` shows the Rust-side create+destroy is about
0.3 us per control (156,634 ns / 513), but the WinUI churn scenario shows the *native* control
create/destroy is about 37 us each ((16.7 - 1.0) ms / ~400 net controls per cycle) - roughly 100x
the Rust-side cost and language-independent, exactly the cost a pool removes. The catch is workload:
the steady-state grid and every keyed-reorder bench create zero controls (`crt` is 0 in all of them;
keyed diffing reuses and moves controls instead of recreating). Native churn happens only on
kind-mismatch remounts (`update`'s `!kind_matches` arm) and unkeyed list shrink/grow, and the main
real-world source of that - long scrolling lists - is already covered by the row recycling in
`reconciler/templated.rs`. A general `ElementPool` also needs per-type reset discipline (clearing
every mutated property on return), a known stale-state bug source. Verdict: no general pool now. If
a real application shows sustained same-typed churn that virtualization does not cover, scope a pool
narrowly to the kind-mismatch remount path and prove it against a `--churn` run with no selftest
regressions.

Net: all three C# techniques are GC-era answers. Two (diff hints, scratch reuse) buy nothing
measurable here and add complexity; the third (pooling) targets a real but narrow cost that the
crate's existing virtualization already covers for the common case. The default stands: do nothing
absent a specific application profile that shows otherwise.

### Cross-language comparison against Microsoft.UI.Reactor (C#)

The evaluation above measures Rust in isolation. To size the language difference directly,
`test_reactor_perf` was aligned to the C# stress harness and run head to head against it. Three
changes made the two apples-to-apples: the Rust grid was set to 70x70 (4900 cells) to match C#'s
`StockDataSource` (was 80x60); both drive the seed-42 `NetRandom`/`Random(42)` dirty-cell stream, so
the same cells change each tick; and `test_reactor_perf` gained a `--json` mode plus a counting
global allocator that emits the same headline metrics and allocation accounting as the C#
`PerfTracker` (`avgReconcileMs`, `avgDiffMs`, `rendersPerSec`, `avgFps`, `allocBytesPerRender`, GC
gen0/1/2).

The right C# counterpart is `StressPerf.ReactorOptimized`, not `StressPerf.Reactor`. Both drive real
WinUI, but the naive `StressPerf.Reactor` rebuilds all 4900 Elements every tick, while
`ReactorOptimized` uses `UseMemoCellsByIndex` to rebuild only changed cells - which is what the Rust
harness's dirty-cell path already does. Comparing against the Optimized variant is the fair test;
the naive numbers are included only as the upper bound C# pays without that hint.

Sweep of `--percent` 0/10/50/100, headless, 10 s per point, same machine, Release/optimized both
sides (Rust `--headless --percent N --duration 10 --json`; C#
`StressPerf.ReactorOptimized --headless --percent N --duration 10 --json`). These results were
measured on August 5, 2026 with .NET SDK 10.0.302 and C# Reactor 0.1.0-preview.13 (`c9191b97`).
Rust used Windows App SDK 2.3.1; the C# repository used its validated Runtime 2.1.3 and WinUI 2.1.0
pins. See the [`test_reactor_perf` readme](../../crates/tests/libs/reactor_perf/readme.md) for the
commands and environment caveat.

| percent | Avg Reconcile (ms) Rust / C# | Avg Diff (ms) Rust / C# | renders/s Rust / C# | Avg FPS Rust / C# | Alloc/render Rust / C# | GC gen0/1/2 Rust / C# |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 2.29 / 11.84 | 1.00 / 11.05 | 24.40 / 14.17 | 57.63 / 55.49 | 3.75M / 0.84M | 0/0/0 / 14/13/12 |
| 10 | 3.53 / 19.69 | 2.50 / 17.61 | 23.29 / 13.22 | 51.74 / 28.34 | 4.13M / 2.16M | 0/0/0 / 34/30/20 |
| 50 | 7.87 / 39.08 | 7.01 / 32.91 | 8.59 / 4.12 | 13.42 / 9.60 | 5.32M / 5.23M | 0/0/0 / 18/18/7 |
| 100 | 10.90 / 52.08 | 10.04 / 42.52 | 6.01 / 3.23 | 8.17 / 7.56 | 6.28M / 7.94M | 0/0/0 / 22/21/7 |

What the numbers say:

- **Reconcile is 4.8-5.6x faster in Rust** across the sweep (2.29 vs 11.84 ms at p=0, up to 10.90 vs
  52.08 ms at p=100), against the optimized C# variant. Throughput is 1.7-2.1x higher.
- **Zero GC versus frequent gen2 collections.** Rust runs the entire sweep with no garbage
  collector; C# takes 14-34 gen0 and 7-20 gen2 collections per 10 s window. Gen2 collections are the
  usual source of frame hitches, and they appear even in the optimized variant.
- **Allocation is a wash, and at low churn Rust allocates more** (3.75M vs 0.84M bytes/render at
  p=0), crossing over near p=50 and coming out ahead at p=100 (6.28M vs 7.94M). This is a harness
  choice, not an inherent cost: the Rust scenario clones the full 4900-element cells vec every render
  (`s.cells.borrow()[..vis].to_vec()`), so its allocation is dominated by a constant ~3.5M baseline
  regardless of churn, whereas C#'s `UseMemoCellsByIndex` reuses unchanged Element records. Matching
  that reuse in the harness would cut the Rust baseline, but note it changes nothing about the speed
  result - Rust wins on reconcile time even while allocating more, because the allocator is cheap and
  there is no collector to pay back later. This is the concrete reason the GC-era caching techniques
  above do not port: the pressure they relieve is not there.

Caveat on the headless micro-suite. `test_reactor_bench` is *not* cross-comparable to C#'s
`PerfBench.ControlModel`. The Rust bench measures only the reconcile body against `RecordingBackend`
(no native controls), while the C# `ControlModel` Reactor variant calls `Reconciler.Mount` and
creates real WinUI controls, so it measures a native-inclusive layer. They sit at different levels
and their ns/op figures should not be placed side by side. The stress-app sweep above is the valid
full-stack cross-language comparison; the micro-suite stays a Rust-only instrument.

### Hook gaps (incremental, low risk)

This crate provides `use_state`, `use_reducer`, `use_reducer_fn`, `use_ref`, `use_memo`,
`use_callback`, `use_effect`, `use_effect_with_cleanup`, `use_context`, `use_resource`,
`use_mutation`, `use_color_scheme`, `use_open_window`, `use_async_state`, `use_inner_size`, and
`use_dpi`. The C# set adds these worth considering:

| Hook | Purpose | Notes |
| --- | --- | --- |
| `use_reduced_motion`, `use_high_contrast` | Accessibility signals from the system | Cheap; reduced-motion should gate the existing implicit-transition system. |
| `use_infinite_resource` | Paged async fetch | This crate has `use_resource` and `use_mutation` but no pagination. |
| `use_focus`, `use_element_ref`, `use_focus_trap` | Imperative focus escape hatches | Useful for dialogs and keyboard navigation. |
| `use_breakpoint` | Responsive layout from window size | Thin wrapper over `use_inner_size`. |
| `use_persisted` | State persisted across runs | Design question: where persistence lives. |
| `use_observable`, `use_collection` | Bridges to externally mutated model state | Less idiomatic in Rust; lowest priority. |

### DSL ergonomics

This crate has `group` (a fragment flattened into the parent child list), `Element::Empty`, and
`Option<Element>` for conditional children. The C# DSL adds `When`/`If`/`ForEach` combinators and a
keyed `Memo(key, ...)` wrapper. A small `when(cond, || el)` helper and a keyed memo wrapper for
virtualized rows would read better at near-zero cost. These are ergonomics, not performance, and do
not need the benchmark gate above.

### Element lifecycle transitions

Lifecycle transitions use WinUI Composition implicit show/hide animations. This is smaller than
the C# Reactor implementation, which retains and reinserts removed controls until their exit
animations complete. Rust can destroy the logical subtree immediately while WinUI keeps only the
departing composition visual alive. The reconciler therefore needs no asynchronous removal state,
and keyed and positional child indices remain correct while an exit animation runs.

### Larger feature areas (product decisions, likely out of scope)

The C# framework is much bigger mainly because of subsystems that are strategic product bets rather
than gaps in this crate: a flex/Yoga layout engine, a commanding abstraction (a command record
bundling label, icon, shortcut, and action - this crate has only the `command_bar` widget),
type-safe navigation and routing with a back stack (this crate has the `navigation_view` control but
no router), keyframe animation, markdown, charting, a data grid and property grid, docking, ICU
localization, Roslyn-style analyzers that enforce rules of hooks (this crate relies on runtime
hook-order checks; a clippy lint could cover part of this), hot reload and live preview, and a
scaffolding CLI. Each is a large investment, and most cut against this crate's minimal, WinUI-native
design. They belong in a separate decision, not in the reconciler or hooks work above.

## Open issue working plan

This plan tracks the open `windows-reactor` issues reviewed in August 2026. The order favors
correctness and common WinUI authoring needs. C# Reactor is a reference for behavior and test cases,
not a surface-area target. Each change must fit the Rust design, include focused headless coverage
where possible, and add the smallest runnable sample that proves the user-facing behavior.

Before expanding an issue beyond its reported case:

1. Reproduce the failure with a test or sample.
2. Identify the invariant that is missing from the current design.
3. Prefer a local correction over a new subsystem.
4. Compare the proposed behavior with C# Reactor, React, and WinUI where relevant.
5. Measure hot-path changes and reject added machinery that does not buy correctness or speed.
6. Reevaluate the remaining plan after the change lands.

| Priority | Issue | Assessment | Direction |
| --- | --- | --- | --- |
| Done | [#4778](https://github.com/microsoft/windows-rs/issues/4778) keyed templated lists | Correctness bug: realized rows followed slots instead of keys. | Equal-count keyed reorders now preserve realized controls and row-local state. Missing and duplicate keys retain positional behavior. |
| Done | [#4776](https://github.com/microsoft/windows-rs/issues/4776) resources | Valid request, plus stale keys were never removed. | Typed string, solid-color brush, number, thickness, and corner-radius values now replace Reactor-owned keys. Theme references remain deferred because resolving them to concrete values would break WinUI theme-resource behavior. |
| Done | [#4772](https://github.com/microsoft/windows-rs/issues/4772) pointer coordinates | Element-relative coordinates cannot anchor a moving drag target, and a moving handle can lose routed events. | `PointerEventInfo` now copies both element-local and window-relative positions. Opt-in pointer capture keeps fast drags routed and exposes capture-loss/cancellation without raw WinRT arguments. |
| Done | [#4771](https://github.com/microsoft/windows-rs/issues/4771) navigation pane events | Valid state gap, but transition events are an unreliable controlled-state foundation. | Settled `IsPaneOpen` and actual `DisplayMode` callbacks now cover light dismiss, adaptive layout, and programmatic changes. Transition events remain deferred until a separate use case requires them. |
| Done | [#4720](https://github.com/microsoft/windows-rs/issues/4720) icon subclasses | Image and font icons worked, but native `BitmapIcon` and `PathIcon` support was missing. | `bitmap_icon(uri, mode)` now exposes native monochrome/full-color `BitmapIcon` behavior, while `path(data)` adds vector paths. Generic images remain a separate `ImageSource` path. |
| Done | Exit transition correctness | `transition(enter, exit)` stored the exit configuration but never consumed it. | WinUI implicit show/hide animations now run opacity and scale lifecycle transitions without retaining logical ghost children or adding asynchronous reconciler state. |
| Done | `TabItem` key clearing | Removing a key updated the Rust model but left the old native `Tag`, so close callbacks reported stale identity. | Key removal now emits the existing `Unset` property path and clears `FrameworkElement.Tag` on the same native item without remounting it. |
| Close | [#4753](https://github.com/microsoft/windows-rs/issues/4753) SVG support | Fixed by PR #4764 and covered by `ImageSource` extension dispatch. | Close after confirming the existing SVG sample. File/memory loading and colorization are separate requests. |
| Deferred | [#4692](https://github.com/microsoft/windows-rs/issues/4692) bootstrap mismatch | Documentation is fixed; the loader failure remains a sharp edge. | Leave `windows-reactor-setup` unchanged for now. If resumed, evaluate dynamic loading that reports misuse without silently bootstrapping a self-contained deployment. |

### Planned order

1. Fix keyed templated-list identity and add reorder tests plus a visible shuffle sample.
2. Correct resource ownership/removal, then add typed resource values and a lightweight-styling
   sample.
3. Add root-relative pointer coordinates and a resize-drag sample.
4. Add `NavigationView` pane-open and display-mode callbacks with a responsive navigation sample.
5. Finish the remaining icon forms and separate unrelated image-loading requests.
6. Fix exit transitions without adding asynchronous reconciler state or temporary ghost children.
7. Clear stale native `TabItem` identity when an optional item key is removed.
8. Leave the bootstrap sharp edge deferred until setup work resumes.

After each item, rerun the Rust/C# stress comparison if the reconciler or allocation behavior
changes. Rust should retain its smaller runtime model and reconciliation advantage; copying C#
pooling, collection, or descriptor machinery requires evidence that the existing Rust path cannot
meet the same invariant more directly.

### Resource ownership and typed values

`ElementExt::resources` still accepts an iterator when every entry has the same Rust value type.
`resource_overrides` uses a consuming builder when one resource dictionary contains different
WinUI value types:

```rust
# use windows_reactor::*;
button("Delete").resource_overrides(|resources| {
    resources
        .set("ButtonBackground", Color::rgb(178, 34, 34))
        .set("ButtonBorderThemeThickness", Thickness::uniform(0.0))
        .set("ControlCornerRadius", CornerRadius::uniform(8.0))
});
```

Each native element tracks only the keys that Reactor inserted. Updating the builder removes
missing Reactor-owned keys before inserting current values, including when the new builder is
empty. Native or application code can keep unrelated entries in the same resource dictionary.

`Color` values intentionally create `SolidColorBrush` instances because lightweight control
resources such as `ButtonBackground` expect brushes. Use strings only for resources that actually
expect strings. `ThemeRef` is not accepted here: looking up a theme key and storing its current
value would lose WinUI's element-aware `{ThemeResource}` resolution. Theme-aware control
properties continue to use the existing theme-binding APIs.
