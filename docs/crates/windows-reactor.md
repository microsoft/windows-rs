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

`App::on_exit` runs once on the UI thread immediately before Reactor exits the process after the
final window closes. The callback must finish synchronously. Use it for cleanup or process-lifetime
instrumentation that cannot run after `App::render`, since the normal final-window path terminates
the process.

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

Components may return another component directly without adding a native control. The
`pass_through_component` sample combines this with a memoized wrapper and stateful child:
`cargo run -p reactor_samples --example pass_through_component`.

The `apps/examples` and `minimal/examples` directories include a focused sample for each hook
(`use_state`, `use_ref`, `use_memo`, `use_effect`, `use_reducer`, `use_resource`, `use_callback`,
`use_color_scheme`, and more).

## Building the UI

Build elements with plain builder functions. Each returns a widget that becomes an `Element` with
`.into()`. Containers take a tuple of children.

- Text: `text_block(content)` with `.bold()`, `.semibold()`, `.font_size(..)`, `.wrap()`,
  `.selectable()`, `.max_lines(..)`, `.text_trimming(..)`, and type-ramp helpers (`title`,
  `subtitle`, `body`, `caption`). Sample:
  `cargo run -p reactor_samples --example text_trimming`.
- Buttons: `button(content)` with `.on_click(..)`, `.accent()`, `.subtle()`, `.enabled(..)`,
  `.icon(..)`, `.flyout(..)`, `.menu_flyout(..)`.
- Icons: any control that takes an icon (`button`, `NavViewItem`, command-bar buttons,
  `selector_bar_item`) accepts `impl Into<Icon>`. A bare `Symbol` creates a `SymbolIcon`;
  `Icon::image(source)` creates a full-color `ImageIcon` from raster, SVG, or surface data;
  `Icon::bitmap_icon(uri, show_as_monochrome)` creates a native `BitmapIcon`; `Icon::font(glyph)`
  and `Icon::font_family(glyph, family)` create a `FontIcon`; and `Icon::path(data)` creates a
  `PathIcon` from XAML path mini-language data. Full-color image icons are constrained to 20 DIPs
  so large raster or SVG sources cannot consume the icon host's available space. Samples:
  `cargo run -p reactor_samples --example icon_elements` and
  `cargo run -p reactor_samples --example image_icon_size`.
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

Framework layout modifiers are available on concrete widget builders through `LayoutExt`:
`.margin(..)`, `.width(..)`, `.height(..)`, minimum/maximum dimensions, and horizontal/vertical
alignment. Pointer, keyboard, capture, and drag/drop modifiers are available through `InputExt`.
UI Automation properties use `AccessibilityExt`, tooltips use `TooltipExt`, and opacity and
composition animations use `VisualExt`. Styling uses `PaddingExt`, `BackgroundExt`, and
`TextStyleExt`, implemented only for widgets whose WinUI type supports the property. Grid, Canvas,
and RelativePanel child placement use `GridChildExt`, `CanvasChildExt`, and
`RelativePanelChildExt`. Apply modifiers before converting the builder into `Element`. Spacing
values use `Thickness` (with `Thickness::uniform(..)`).

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
keyboard handlers live on `InputExt`: `.on_tapped(..)`, `.on_pointer_pressed(..)`,
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

`Padding` has no single owning interface. `set_padding` (`backend/winui/mod.rs`) calls the default
interface directly for `Border`, `StackPanel`, `TextBlock`, and `RichTextBlock`; uses `IGrid` for
`Grid` and `SwapChainPanel`; and uses `IControl` for control descendants. `PaddingExt` is generated
only for those categories.

`BackgroundExt` is generated for `Border`, panel descendants, and control descendants.
`TextStyleExt` provides foreground and font modifiers for text blocks and control descendants.
The backend uses the matching default interfaces, `IPanel`, and `IControl`. `BorderBrush` and
`BorderThickness` remain opt-in per-widget builders, currently exposed by `Border` and `TextBox`.

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

`RecordingBackend::fail_next` and `RecordingBackend::fail_on` inject a panic immediately before a
selected backend operation mutates the backend model. Ordinary widget mounts catch failures after
native creation, remove the partially mounted native and logical subtree, run pending component
cleanups, and then resume the panic. Error boundaries can therefore mount a fallback without
retaining controls, handlers, headers, panes, or effects from the failed subtree. This contract does
not yet cover custom elements, templated lists, or failures during rollback.

Component and provider updates retain their logical ownership records while a panic propagates.
This lets an error boundary discard a failed non-structural update, run component cleanups, and
mount its fallback. It also keeps ownership reachable for explicit root teardown when a property or
event update panics before mutating the backend.

If structural replacement fails after removing old native output, an error boundary discards only
the ownership that remains live before mounting its fallback. It does not restore the old subtree.
The strict teardown path still treats a missing control as an invariant violation. Structural
retry without an error boundary and rollback of child collection mutations are not yet defined.

Fail-before append, insert, replace, move, and remove errors may leave the mounted tree and backend
collection in different intermediate orders. An error boundary discards the containing subtree and
mounts its fallback. Without a boundary, explicit root teardown still reaches every live control and
runs component cleanup exactly once. Retrying reconciliation or restoring the prior collection after
such a failure is not supported.

Native destruction is the teardown commit point. Logical cleanup, lifecycle callbacks, and custom
pre-destroy hooks run first, but mounted ownership is retained until `Backend::destroy` succeeds. If
a fail-before destroy panic reaches an error boundary, discard retries the still-owned control
without repeating cleanup. This also covers a failure after earlier descendants were destroyed.
Retrying a failed `unmount_root` remains undefined because root ownership is consumed before
teardown begins.

Two crates measure reconciler performance. `test_reactor_bench` is a headless micro-suite (run with
`cargo run -p test_reactor_bench --release`) that brackets only the reconcile body against
`RecordingBackend` and reports ns/op plus bytes/op and allocs/op - the right instrument for
Rust-side reconciler cost (structural skip, keyed diff, allocation). `test_reactor_perf` is a live
WinUI stress app (a 70x70 stock grid) whose `--churn` flag additionally forces native control
create/destroy. Its [`readme`](../../crates/tests/libs/reactor_perf/readme.md) describes the matched
Microsoft.UI.Reactor comparison.

`reconciler_model.rs` runs deterministic generated update sequences against an independent model.
It checks keyed order, native control counts, typed-reference lifetime, and exact component cleanup
counts after every transition. CI also enforces branch and line coverage floors for the hand-written
reconciler files. The coverage gate excludes generated bindings and live WinUI backend code.

For pull requests, `reactor_bench/compare.ps1` builds the merge base and branch in separate
worktrees on the same runner. It rejects allocation-count increases, byte growth above 10%, and
Rust-side timing regressions above 25% in the stable benchmark set.

### Reactor and canvas naming

`windows-reactor` and `windows-canvas` define some of the same short names for different domains.
The rule: canvas keeps the short name (it owns user-facing draw loops) and reactor takes a
domain-prefixed alternative.

### Resource ownership and typed values

`ResourceExt::resources` accepts an iterator when every entry has the same Rust value type.
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

## Reconciler architecture

Reactor separates logical identity, native identity, mounted output slots, and dirty propagation.
This section records the resulting implementation contract and the measurements used to guard it.

### Progress

| Work | Status |
| --- | --- |
| Architecture contract and topology matrix | Complete |
| Dirty descendant behind memoized widget root | Regression, fix, benchmark, and sample added |
| Context subscriber behind memoized widget root | Regression and fix added |
| Logical component IDs and parent paths | State keyed by logical ID; overflow map removed |
| Logical component ownership | State and lifecycle share one owner |
| Provider logical ownership | Providers have stable IDs, parent links, projection, and teardown |
| Error-boundary ownership | Boundary identity and fallback state are node-owned |
| Custom-element ownership | Handles and teardown are sparse native-node auxiliary state |
| Templated-list ownership | Realized rows and callbacks are sparse templated-node state |
| Native lifecycle ownership | Pre-unmount callbacks are sparse native-node state |
| Path-scoped dirty traversal | Native parent walks replace the global flag and root-wide scan |
| Reconciler state consolidation review | Complete |
| Host/window state consolidation | `HostContext` introduced; six `Reconciler` fields removed |
| Native ownership consolidation | `MountedTree` owns native topology |
| Recursive teardown | One child-first traversal covers children, rows, headers, and panes |
| Native ownership checks | Debug builds verify unique ownership and matching parent links |
| Private-memory and peak-memory benchmark output | Added to text, JSON, and CSV output |
| Typed element API | Universal, visual, attached-layout, and styling capabilities are typed |
| Element cardinality | `Fragment` is child-only; `Element::Group` removed |
| Full mounted ownership evaluation | Complete |

### Invariants

| Area | Required invariant |
| --- | --- |
| Logical identity | Every mounted component, provider, and error boundary has a unique stable ID. |
| Native identity | A `ControlId` identifies one native object and is not logical identity. |
| Parentage | Every logical node has one logical parent, except the root. |
| Native parentage | Every owned native node has one parent across all ownership forms. |
| Dirty state | A state write marks its owner and the logical ancestor path. |
| Skipping | A node is skipped only when it and all logical descendants are clean. |
| Ownership | Hooks, effects, contexts, output, and cleanup belong to their logical node. |
| Replacement | Replacement unmounts the complete old logical subtree before identity reuse. |
| Cleanup | Every mounted node is cleaned exactly once, with children cleaned before parents. |
| Keys | Keys identify siblings in one child collection and must affect reconciliation. |
| Projection | Transparent wrappers add no hidden native control. |
| Fragments | Multi-child values are accepted only by APIs that support multiple children. |
| Virtualization | Only realized rows own mounted logical nodes and lifecycle state. |

Debug and test builds should check these invariants after reconciliation. They must not remain
assumptions repeated across root, positional, keyed, and templated update paths.

### Logical wrapper identity

Components, memoized components, providers, and error boundaries receive a logical ID before
their child output mounts, so hook setters retain the exact owner.

The logical record separates optional native projection from the logical node:

```rust,ignore
struct LogicalNode {
    parent: Option<NodeId>,
    native_root: Option<ControlId>,
    kind: LogicalNodeKind,
}
```

Component state holds its `RenderCx`, previous object, previous mounted output, and context
subscriptions, indexed by `NodeId`. The parent relationship crosses intervening native widgets, so
a stateful child remains reachable through a memoized component that renders a stable widget root.

The logical path is authoritative:

- `component_instance_overflow` is removed;
- component lookup and identity transfer no longer depend on `ControlId`;
- global force-dirty behavior is removed;
- one update decision handles dirty-descendant traversal;
- an output slot remains mounted when its logical node temporarily produces no native control.

### Ownership consolidation

Header and pane subtrees, templated rows, custom handles, selection/reorder callbacks, pre-unmount
callbacks, and error fallback state live under mounted ownership. Positional, keyed, and templated
algorithms decide correspondence and order while sharing identity, dirty, lifecycle, and cleanup
rules.

Each native child collection also has an ordered logical output mirror. A slot may contain a native
control, a logical node with no native output, or both. Native insertion and movement derive their
indices from this mirror, so empty components do not shift their native siblings and can later
produce output without remounting their hooks or effects.

### Reconciler simplification

The current `Reconciler` mixes five responsibilities in one struct:

1. mounted tree identity and ownership;
2. transient state for one reconciliation pass;
3. host/window environment;
4. widget-specific auxiliary state;
5. diagnostics.

This is more than a file-layout problem. Independent maps require every mount, replacement, and
unmount path to remember the same set of cleanup operations. Header, pane, templated, custom, and
error-boundary paths have already demonstrated how easily one map can be omitted.

`Reconciler` now has the target ownership shape:

```rust,ignore
struct Reconciler<B> {
    backend: B,
    tree: MountedTree,
    pass: ReconcilePass,
    host: HostContext,
    stats: ReconcileStats,
    root_output: Option<MountedOutput>,
    next_slot_id: u64,
}
```

`MountedTree` owns node identity, parentage, native projection, children, secondary slots, and
teardown. `ReconcilePass` owns reusable transient scratch such as forced paths and keyed-diff
buffers. `HostContext` contains the dispatcher-facing rerender request, marshaller, host ID, size,
DPI, and context environment. `ReconcileStats` contains counters only.

`MountedTree` now records the native parent on the existing per-control node entry. Normal
children, headers, panes, custom controls, and realized templated rows use the same parent
invariant. Dirty components walk these links to the root, stopping when they reach an already
forced ancestor. This makes dirty propagation proportional to path depth and removes the
root-wide ownership scan and its traversal stack. The additional parent field does not add a map
or allocation; the headless allocation counts remain unchanged.

Child and slot mutation now goes through `MountedTree`, so parent links and sparse ownership
storage cannot be updated independently. Unmount collects the complete native ownership list once
and releases it in reverse order. Primary children, realized rows, headers, and panes therefore
finish cleanup before their owner without recursive `unmount` calls or separate subtree
algorithms. Debug builds verify that each owned native control appears once and that both sides of
every parent relationship agree.

The compact teardown list also reduces headless allocation cost. Single component, pass-through,
and deep pass-through mount cycles each use one fewer allocation and 16 fewer bytes. The 64-node
mount/unmount case dropped from 18,076 bytes and 281 allocations to 17,568 bytes and 271
allocations; the 512-node case dropped from 144,340 bytes and 2,085 allocations to 140,248 bytes
and 2,069 allocations.

Component ownership is now grouped in `MountedLogicalTree` under `MountedTree`. It owns component
instances, native projections, logical ID allocation, the active logical parent scope, lifecycle
listener counts, projection transfer, and appeared/disappeared dispatch. `Reconciler` no longer
contains four component identity fields plus two listener counters, and component registration or
removal cannot update a projection without updating listener accounting through the same owner.
This grouping is output-neutral: the headless allocation counts remain unchanged from the compact
teardown baseline.

Providers now allocate compact logical wrapper nodes and enter the logical parent scope while
mounting or updating their child. A component below a provider therefore reaches component
ancestors through the provider rather than stopping at an unrepresented transparent wrapper.
Provider nodes share the projection and teardown APIs with components but use sparse wrapper
storage, so they do not inflate every logical node to the roughly 968-byte component-state size.
The `provider_mount` benchmark reports 527 bytes and 11 allocations for one provider plus one
component, while component-only allocation counts remain unchanged.

Error boundaries use the same compact wrapper storage. The boundary node remains stable while
recovery replaces its projected fallback or child subtree, and nested boundaries retain distinct
logical parents. Fallback state is stored on the boundary node, so `Reconciler` no longer has an
`error_boundary_fallbacks` map keyed by a borrowed native identity. Normal
`error_boundary_mount` cycles report 399 bytes and 10 allocations, matching the component-only
mount baseline after warmup.

Custom element handles now live in sparse `MountedTree` auxiliary storage keyed by their native
node. Mount, update, stale-ID cleanup, and `before_destroy` teardown all go through the tree, so
`Reconciler` no longer owns a separate `custom_handles` map. Ordinary native nodes do not gain an
optional boxed field. The headless `custom_mount` case reports 52 bytes and two allocations per
mount/unmount cycle.

Templated lists now use sparse auxiliary state beneath `MountedTree`. The list registry owns each
list's current element, realized rows, and selection/reorder callback trampolines; the templated
owner also holds the shared realization queue and deferred row teardown queue. This removes six
templated fields and maps from `Reconciler`. Adding a selection or reorder handler during an update
now attaches the missing backend trampoline instead of silently ignoring the new handler.

The list state captures its active context values. Deferred row realization restores that snapshot
while mounting the row, and context invalidation traverses realized rows, headers, panes, and
primary children through the same owned-child operation used by teardown.

Realized rows are stored by row index rather than in a dense `Vec<Option<RealizedRow>>`. An
unrealized item therefore does not reserve space the size of an `Element`. Before this change, an
unrealized 64-item list used about 54 KB per mount because every empty slot had the full
`RealizedRow` size. The release `templated_mount` benchmark now reports 276 bytes and eight
allocations, and the 4,096-item case has the same mount cost. Realized-row storage grows with the
visible window rather than the item count.

Pre-unmount callbacks now live in sparse native lifecycle storage under `MountedTree`. Registration,
replacement, removal, and teardown all go through the tree, so a callback cannot outlive its
native node. The last lifecycle side map has left `Reconciler`. The release `lifecycle_mount`
benchmark reports 52 bytes and two allocations, matching a callback-free custom native mount.

The three debug counters now live in `ReconcileStats`, available through `Reconciler::stats()`.
`Reconciler` also retains the root mounted output and the monotonic logical-slot allocator. All
other mounted state is grouped under `tree`, `pass`, `host`, and `stats`.

The existing side state should move as follows:

| Current state | Intended owner |
| --- | --- |
| `children_mirror`, `id_kinds`, header, and pane maps | Mounted native node |
| component instances and native projections | Mounted logical component node |
| error fallback state | Error-boundary node |
| custom handles | Custom node |
| templated state, selection, reorder, and deferred rows | Templated-list node (complete) |
| pre-unmount callback | Node lifecycle state (complete) |
| forced nodes, forced controls, traversal scratch | `ReconcilePass` |
| marshaller, host ID, size, DPI, rerender request | `HostContext` |

Do not replace the maps with one large per-control struct containing every optional field. Most
controls do not use headers, panes, templating, or custom lifecycle state. Use node-kind enums and
allocate auxiliary state only for node kinds that need it.

`MountedTree` owns child projection through sparse owned-only classification. Most children project
into the parent's visual collection; `ContentDialog` is owned-only because WinUI presents it
outside that collection. Insert, move, replace, and remove operations derive visual indices from
the ownership order before calling the backend. The classification stays available when teardown
destroys a child before its parent edge is removed. `WinUIBackend` therefore needs no child-order
mirror and receives only visual children and visual indices.

#### C# Reactor comparison

C# Reactor stores one `ComponentNode` per hidden native `Border`. The state setter captures that
node directly, marks it `SelfTriggered`, and invokes the parent component's rerender callback.
Dirty native ancestors are then found through WinUI's visual-parent relationship.

This gives C# a simpler component identity lookup, but at the cost of one native XAML control per
component. It also makes transparent components affect the native tree. Rust must not copy that
tradeoff: it would increase allocation, measure/layout work, visual depth, and memory.

The useful C# rules are:

- a state setter marks its exact component node directly;
- component render state is owned by that node;
- dirty propagation follows one parent relationship;
- teardown starts from the mounted ownership tree.

The C# `Reconciler` as a whole is not simpler. It spans thousands of lines and contains component,
error-boundary, navigation, pooling, hot-reload, animation, gesture, style, and registration state.
Rust should copy the small node invariants, not its hidden controls or accumulated registries.

#### Consolidation sequence

1. Finish logical dirty-path behavior without a global force mode.
2. Introduce `MountedTree` and `HostContext` wrappers around existing state without behavior
   changes.
3. Move native kind, children, header, and pane ownership into mounted native nodes.
4. Move component, provider, error-boundary, and custom lifecycle into logical node kinds.
5. Move templated rows and their callbacks into a templated node kind.
6. Make recursive node teardown the only unmount path.
7. Delete the replaced side maps after each migration rather than keeping compatibility mirrors.

Every consolidation step must state which fields and special-case branches it deletes. A new
abstraction that only wraps the old maps without enabling their removal is not sufficient.

Steps 1-7 are complete. Child, slot, custom, templated, and lifecycle storage remains sparse inside
`MountedTree` rather than adding optional fields to every native or logical node. The resource,
item-key, and exit-transition entries in the earlier audit were stale: PR #4782 already added
resource replacement, `ItemKey` clearing, and WinUI implicit hide transitions, with regressions and
visual samples. Structural typing and element cardinality are also complete.

### Typed element API

Native modifiers use sealed capability traits so calls that cannot affect a native element do not
compile. Resource dictionaries use `ResourceExt`, framework layout uses `LayoutExt`, and pointer,
keyboard, capture, and drag/drop modifiers use `InputExt`. UI Automation properties use
`AccessibilityExt`, and tooltips use `TooltipExt`. These traits, `VisualExt`, and the attached
layout traits `GridChildExt`, `CanvasChildExt`, and `RelativePanelChildExt` are implemented by
concrete native widget builders but not by erased `Element` or logical wrappers:

```rust,compile_fail
# use windows_reactor::*;
let element: Element = button("Delete").into();
element.resources([("ButtonBackground", "Red")]);
```

The same boundary applies to layout:

```rust,compile_fail
# use windows_reactor::*;
let element: Element = button("Save").into();
element.width(100.0);
```

Input modifiers have the same boundary:

```rust,compile_fail
# use windows_reactor::*;
let element: Element = button("Save").into();
element.on_tapped(|| {});
```

Accessibility and tooltip modifiers also require a concrete widget:

```rust,compile_fail
# use windows_reactor::*;
let element: Element = button("Save").into();
element.automation_name("Save document");
```

Attached layout also requires a concrete native child:

```rust,compile_fail
# use windows_reactor::*;
let element: Element = text_block("Cell").into();
element.grid_row(1);
```

Opacity and animations have the same native-element boundary:

```rust,compile_fail
# use windows_reactor::*;
let element: Element = text_block("Faded").into();
element.opacity(0.5);
```

Styling is also limited to native types that expose each property:

```rust,compile_fail
# use windows_reactor::*;
Image::new("asset.png").padding(8.0);
```

```rust,compile_fail
# use windows_reactor::*;
border(text_block("Panel")).font_size(16.0);
```

Apply capabilities before erasure:

```rust
# use windows_reactor::*;
let element: Element = button("Delete")
    .resources([("ButtonBackground", "Red")])
    .width(100.0)
    .on_tapped(|| {})
    .automation_name("Delete item")
    .tooltip("Delete the selected item")
    .padding(8.0)
    .background(ThemeRef::Accent)
    .foreground(ThemeRef::AccentText)
    .opacity(0.8)
    .grid_row(1)
    .into();
# let _ = element;
```

The widget enum declaration now emits `KeyExt`, one sealed native-modifier accessor,
`ResourceExt`, `LayoutExt`, `InputExt`, `AccessibilityExt`, `TooltipExt`, `VisualExt`,
`PaddingExt`, `BackgroundExt`, `TextStyleExt`, and the three attached-layout trait implementations
from one widget list. This removed duplicated widget matches that omitted newer controls.
`SwapChainPanel`, `CompositionHost`, and `WebView2` were missing from the erased `with_key` match;
key dispatch now comes from the authoritative declaration.

The migrations updated tests and samples to retain concrete widgets while adding modifiers, then
erase them only at insertion. The attached-layout migration found calls made after provider
wrapping and helper functions that returned `Element` too early. Applying placement before
`provide`, and returning concrete widget builders from helpers, makes those mistakes visible in the
types. `TemplatedListBuilder` implements the attached-layout, visual, and styling traits directly
because it represents a native `Control` outside the widget enum.

The attached-layout traits prove that the target is a concrete native element. They cannot prove
that the element will later be inserted under the matching Grid, Canvas, or RelativePanel parent.

Styling uses reviewed categories on each authoritative widget declaration:

| Category | Capabilities | Widgets |
| --- | --- | --- |
| `Control` | Padding, background, foreground, and fonts | WinUI `Control` descendants |
| `PaddedPanel` | Padding and background | `Grid`, `StackPanel`, `SwapChainPanel` |
| `Panel` | Background | `Canvas`, `RelativePanel` |
| `Text` | Padding, foreground, and fonts | `TextBlock`, `RichTextBlock` |
| `Border` | Padding and background | `Border` |
| `Visual` | None | Shapes, images, composition hosts, `WebView2` |

The categories follow both WinUI inheritance and the interfaces used by the backend. They generate
selective trait implementations without a second widget list. The `SwapChainPanel` category also
exposed a dispatch gap: it inherits `Grid` padding, but the backend handled only the exact `Grid`
variant. Padding now dispatches through its `IGrid` interface.

Unsupported calls now fail to compile instead of reaching `diag::unhandled_modifier`. Styling
methods also leave erased `Element` unavailable, so a modifier cannot silently target a component,
provider, error boundary, group, or empty element.

`with_key` and `provide` are structural operations rather than native visual modifiers. `KeyExt`
provides reconciliation identity, while `ProvideExt` wraps anything convertible into `Element` in
a context provider. The former `ElementExt` trait and its public modifier accessor have been
removed.

Widget constructors return concrete builders that implement native capabilities and
`Into<Element>`. Logical constructors such as `component`, `memo`, and `error_boundary` return
`Element`, so only structural operations remain available on them. Child builders accept
`impl Into<Element>`, and invalid native modifier calls fail to compile:

```rust,compile_fail
# use windows_reactor::*;
# fn app(_props: &(), _cx: &mut RenderCx) -> Element { Element::Empty }
component(app, ()).width(100.0);
```

Styling a component boundary requires an explicit native wrapper:

```rust,ignore
border(component(app, ())).width(100.0)
```

Use sealed capability traits for supported property families, including framework-element layout,
UI-element input, control properties, text styling, panels, shapes, and attached layout
properties. Generate implementations from explicit capability classifications in the curated
Reactor configuration plus WinUI inheritance. Raw metadata alone does not encode every useful or
meaningful property relationship, so generation must support reviewed semantic overrides and emit
a coverage report.

Attached properties can reject invalid element targets, but cannot always prove that the element
will later be inserted under the matching parent. Do not claim a stronger compile-time guarantee
than the builder types can provide.

### Element cardinality

`Fragment` is a child-only collection accepted by `vstack`, `hstack`, `grid`, `Canvas`, and
`RelativePanel`. It does not implement `Into<Element>`, so components, providers, error boundaries,
application roots, and single-child controls continue to accept or return zero or one mountable
element:

```rust
# use windows_reactor::*;
let row = fragment((text_block("Name"), text_block("Value")));
let panel = vstack((text_block("Header"), row));
# let _ = panel;
```

```rust,compile_fail
# use windows_reactor::*;
let _ = border(fragment((text_block("a"), text_block("b"))));
```

`IntoChildren` flattens nested fragments and removes `Element::Empty` while constructing the
parent's child vector. The reconciler therefore receives a flat slice and does not allocate a
temporary fragment view. It retains an empty-element filter only for callers that populate public
widget child vectors directly.

Removing the mount-time child-reference vector saves one allocation per multi-child mount and eight
bytes per child in the headless benchmark: 64-node mount/unmount fell from 17,568 to 17,056 bytes
and 271 to 270 allocations; 512 nodes fell from 140,248 to 136,152 bytes and 2,069 to 2,068
allocations.

Removing `Element::Group` makes the former runtime-invalid cases unrepresentable:

- a fragment as the application root;
- a fragment returned from a component;
- a fragment inserted into a single-child control;
- a keyed fragment whose key would be discarded when flattened.

### Typed element references

`ElementRef<T>` provides identity-stable access to a mounted native element without retaining it
after unmount. `RenderCx::use_element_ref` creates the reference once for a hook slot, and
`ElementRefExt::element_ref` attaches it to a compatible concrete widget:

```rust,ignore
let input = cx.use_element_ref::<TextBoxHandle>();
let focus_target = input.clone();

vstack((
    text_box("").element_ref(&input),
    button("Focus").on_click(move || {
        let _ = focus_target.focus();
    }),
))
```

The reconciler populates the shared reference after native creation, moves it when the attached
reference changes during an in-place update, and clears it before native destruction. The public
reference stores a typed handle only while mounted; it does not expose a permanent raw
`IInspectable`.

The attachment trait has an associated handle type and is sealed by the existing native capability
boundary. A `TextBox` accepts `ElementRef<TextBoxHandle>`, while `Image`, `SwapChainPanel`, and
`CompositionHost` accept their existing handle types. Attaching an incompatible reference does not
compile.

`ElementRef<TextBoxHandle>::focus` calls WinUI `UIElement.Focus` with
`FocusState::Programmatic`. It returns `Ok(false)` when the element is not mounted or WinUI rejects
the request, and preserves COM errors as `Err`. This follows the useful lifecycle behavior of the
C# Reactor reference API without copying its untyped reference layer, runtime type checks, or
event-dispatch machinery.

Headless tests cover stable hook identity, mount, in-place reference replacement, removal, and
pre-destroy clearing. The `ElementRef_TextBoxFocusAndClear` WinUI fixture verifies real focus and
clearing after host replacement. The `element_ref` sample provides a direct visual focus check.

The host-replacement fixture exposed an existing teardown leak: production and test post-render
callbacks strongly captured their own `RenderHost`, forming an `Rc` cycle. Post-render and native
theme/size callbacks now hold `WeakRenderHost`, and dropping the final strong host unmounts the
native root and runs root hook cleanups. This is required for references, effects, controls, and
backend state to be released when a window host is replaced or destroyed.

### Reconciliation proof

One authoritative node-update path must own:

1. kind and key compatibility;
2. replacement;
3. state and context dirtiness;
4. memo eligibility;
5. lifecycle dispatch;
6. cleanup responsibility.

The topology test matrix must combine:

| Dimension | Cases |
| --- | --- |
| Root | Widget, pass-through component, empty, changed widget kind |
| Wrappers | Component, memo, provider, error boundary |
| Dirtiness | Parent only, child only, both, context-driven |
| Children | Positional, keyed movement, insertion, removal |
| Lifecycle | Mount, rerender, replacement, unmount, error recovery |
| Effects | Dependency change, cleanup order, unmount cleanup |
| Virtualization | Realize, recycle, key change, disappear, reappear |
| Secondary slots | Header, pane, and custom child ownership |

Debug and test checks should prove that every logical ID is reachable once, parent links agree,
setters cannot target reused identities, native child order matches logical projection, and every
effect and callback is cleaned exactly once.

Compile-fail tests should cover modifiers on logical wrappers, modifiers on unsupported widget
types, and fragments in single-child positions. Start with rustdoc `compile_fail` contracts where
the exact diagnostic is not important. Add a dedicated compile-test dependency only if richer
coverage justifies it.

### Performance and memory gates

Architecture work must preserve Reactor's advantage over Microsoft.UI.Reactor and should reduce
Rust-side time and memory where possible.

Before each behavior-changing phase:

1. Run `cargo run -p test_reactor_bench --release` and save the full table.
2. Run `test_reactor_perf` at 0%, 10%, 50%, and 100% updates with JSON output.
3. Run the matched `StressPerf.ReactorOptimized` C# cases when the comparison environment changes.
4. Record toolchain, Windows App SDK versions, hardware, and source revisions with the results.

The headless suite must include component-heavy steady-state and update cases, pass-through
nesting, dirty descendants behind memoized widget roots, keyed changes, mount/unmount, and
virtualized row recycling. It reports `ns/op`, `bytes/op`, and `allocs/op`.

The live suite remains responsible for native XAML creation, destruction, composition throughput,
working set, and end-to-end rendering. Add process private bytes and peak working set to its JSON
output so arena or ownership changes cannot hide retained-memory regressions behind zero Rust
garbage collections.

Performance requirements:

- no hidden native controls for component identity;
- no mounted-node allocation for unrealized virtualized rows;
- dirty updates visit the logical dirty path rather than the whole native tree;
- steady-state hooks retain their current zero-allocation behavior;
- investigate any repeatable regression above 5%;
- do not accept a regression above 10% without a measured and documented reason;
- compare memory per mounted node and memory after repeated mount/unmount cycles.

Use matched release builds, fixed update streams, warmup passes, and multiple repetitions.
Wall-clock results are evidence only when the allocation counts and reconciler operation counts
agree with the expected algorithmic change.

### Samples

Every user-visible fix or feature addition should include the smallest runnable sample that makes
the behavior visually clear. Reconciler changes need a sample when a user can trigger the topology
interactively, even when the same case has a headless regression test.

Samples should:

- isolate one behavior;
- display the expected state in the window;
- provide one direct interaction that would expose the old bug;
- avoid unrelated styling or application structure;
- name the invariant being demonstrated in the introductory text.

Headless tests remain the correctness gate. Samples are the manual WinUI and visual validation
surface.

### Implemented change sequence

1. Added the architecture contract, missing topology regressions, benchmark cases, and minimal
   samples.
2. Introduced logical IDs and parent paths without changing the public API.
3. Routed state and context dirtiness through logical identity.
4. Removed component collision and global force-dirty mechanisms.
5. Consolidated transparent-wrapper lifecycle and teardown ownership.
6. Introduced concrete public wrappers and compile-time modifier capabilities.
7. Separated multi-child collections from mountable elements.
8. Generated and audited capability implementations.
9. Consolidated mounted ownership where measurements and invariants required it.

Each stage passed formatting, clippy, headless tests, the relevant WinUI selftests, benchmark
comparison, and its visual sample before the next stage began.

## Core stabilization program

Feature work is frozen while this checklist is active. Bug fixes and changes needed to prove or
simplify the core remain in scope. Each item should land as a small PR that leaves the crate
working, updates this checklist, and states which invariant it proves or which complexity it
removes.

A major rewrite is not the starting point. The current logical IDs, `MountedTree`, sparse auxiliary
state, recursive teardown, model tests, and performance gates provide a usable migration base.
Larger internal changes should follow characterization tests and move one ownership boundary at a
time.

### Stabilization checklist

- [x] Separate logical identity from native `ControlId`.
- [x] Consolidate mounted ownership and child-first teardown under `MountedTree`.
- [x] Add deterministic model tests, lifecycle stress tests, coverage floors, and performance
  comparisons.
- [x] Make the recording backend check live IDs and unique ownership across children, headers,
  panes, and realized rows after every generated model transition.
- [x] Clear root ownership when `Reconciler::unmount` removes the root, preventing host teardown
  from destroying the same native control twice.
- [x] Unify logical and native consistency validation, add root and logical-cycle checks, and run
  the validator after stable reconcile, public unmount, root unmount, templated realization, and
  every generated model transition.
- [x] Move logical IDs, projections, wrapper records, lifecycle accounting, and their consistency
  proof into `reconciler/logical_tree.rs` without changing behavior or weakening coverage floors.
- [x] Make `MountedTree` own native consistency validation and logical-child permutation so
  reconciliation code no longer mutates native ownership maps directly.
- [x] Move native topology, secondary slots, sparse lifecycle state, and consistency validation
  into `reconciler/mounted_tree.rs` without changing behavior, and validate keyed permutation
  preconditions at that boundary. Move the reconciler module root to `reconciler/mod.rs` to match
  other multi-file modules.
- [x] Group host state under `HostContext` and move component, provider, and error-boundary
  reconciliation into `reconciler/wrappers.rs` without changing behavior.
- [x] Store sparse visual versus owned-only child projection in `MountedTree`, test mixed indexing
  across insert, move, replace, and remove, and delete WinUI's child-order mirror.
- [x] Add fail-before backend injection for ordinary widget create, property, event, child, header,
  and pane operations; roll failed mounts back before propagating the panic.
- [x] Retain component and provider ownership across failed non-structural updates so error
  boundaries can discard the subtree, run cleanups, and mount a fallback.
- [x] Let error boundaries discard failed structural replacements without destroying removed
  controls twice, including replacements nested under native and component output.
- [x] Characterize fail-before append, insert, replace, move, and remove updates: error boundaries
  discard the subtree, while uncaught failures remain reachable for explicit teardown.
- [x] Retain mounted ownership until native destroy succeeds so error boundaries can retry
  fail-before destroy without repeating component or lifecycle cleanup.
- [ ] Extend backend fault injection to custom and templated mounts, root destroy retry, collection
  retry or rollback, and rollback itself; define the valid state after each failure.
- [ ] Define and test render, commit, effect, cleanup, error-boundary, and reentrant-event ordering.
- [ ] Enforce the UI-thread boundary in release builds and reject stale asynchronous updates after
  unmount or host replacement.
- [ ] Move templated realization and recycling through the same ownership checks as ordinary,
  header, and pane content.
- [ ] Mechanically verify every callable WinUI vtable entry and prevent placeholder slots from
  becoming callable.
- [ ] Replace final-window `process::exit` teardown with an orderly, testable host shutdown.
- [ ] Classify every open Reactor issue as a core defect, required platform contract, deferred
  feature, or unsupported behavior.
- [ ] Complete a final architecture review and remove the feature freeze only after the exit
  criteria below pass.

### PR discipline

Each stabilization PR must:

1. Add or strengthen a deterministic test before changing behavior.
2. Change one ownership, lifecycle, scheduler, or backend boundary.
3. Delete superseded state or special-case branches in the same PR.
4. Run the smallest headless suite that proves the invariant, plus relevant WinUI selftests.
5. Compare reconciler performance when a hot path or mounted-node representation changes.
6. Update this checklist and record any newly discovered follow-up work.

Do not introduce compatibility mirrors that leave two structures responsible for the same
identity, parent relationship, subscription, or cleanup. A temporary adapter is acceptable only
when one side is read-only and its removal is assigned to the next PR.

### Exit criteria

The feature freeze ends when:

- debug and test builds validate logical and native ownership after every reconciliation boundary;
- injected backend failures and panics cannot leave partially owned native or logical nodes;
- cleanup and event revocation run exactly once under update, replacement, rollback, and shutdown;
- ordinary and templated children use the same identity and teardown rules;
- off-thread and stale asynchronous updates have deterministic behavior;
- every regression fixed by Reactor PRs 4782, 4795, and 4807 has a narrow permanent test;
- the headless model, WinUI selftests, coverage floors, and performance gates pass; and
- the remaining architecture is small enough that each mounted-state field has one documented
  owner.
