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
create/destroy. Its [`readme`](../../crates/tests/libs/reactor_perf/readme.md) describes the matched
Microsoft.UI.Reactor comparison.

### Reactor and canvas naming

`windows-reactor` and `windows-canvas` define some of the same short names for different domains.
The rule: canvas keeps the short name (it owns user-facing draw loops) and reactor takes a
domain-prefixed alternative.

### Resource ownership and typed values

`ElementExt::resources` accepts an iterator when every entry has the same Rust value type.
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

## Architecture stabilization plan

Reactor's component identity, dirty propagation, element cardinality, and modifier typing need a
coordinated correction. Recent fixes exposed the same underlying problem in different forms:
logical components borrow native `ControlId` identity even though pass-through components may
share a native root, while unchanged wrappers may hide a dirty logical descendant.

This section tracks the planned work. It is an implementation contract, not a commitment to one
large reconciler rewrite. Each phase must remain independently reviewable and measurable.

### Progress

| Work | Status |
| --- | --- |
| Architecture contract and topology matrix | In progress |
| Dirty descendant behind memoized widget root | Regression, fix, benchmark, and sample added |
| Context subscriber behind memoized widget root | Regression and fix added |
| Logical component IDs and parent paths | State keyed by logical ID; overflow map removed |
| Logical component ownership | Instances, projections, IDs, parent scope, and listeners share one owner |
| Path-scoped dirty traversal | Native parent walks replace the global flag and root-wide scan |
| Reconciler state consolidation review | Complete; mounted ownership model planned |
| Host/window state consolidation | `HostContext` introduced; six `Reconciler` fields removed |
| Native ownership consolidation | `MountedTree` owns kind, parent, children, header, and pane state |
| Recursive teardown | One child-first traversal covers children, rows, headers, and panes |
| Native ownership checks | Debug builds verify unique ownership and matching parent links |
| Private-memory and peak-memory benchmark output | Added to text, JSON, and CSV output |
| Typed element API and fragments | Not started |
| Full mounted ownership evaluation | In progress |

### Invariants

| Area | Required invariant |
| --- | --- |
| Logical identity | Every mounted component, provider, and error boundary has a unique stable ID. |
| Native identity | A `ControlId` identifies one native object and is not logical identity. |
| Parentage | Every logical node has one logical parent, except the root. |
| Native parentage | Every owned native node has one parent across children, slots, or realized rows. |
| Dirty state | A state write marks its owner and the logical ancestor path. |
| Skipping | A node is skipped only when it and all logical descendants are clean. |
| Ownership | Hooks, effects, contexts, rendered output, and cleanup belong to their logical node. |
| Replacement | Replacement unmounts the complete old logical subtree before identity reuse. |
| Cleanup | Every mounted node is cleaned exactly once, with children cleaned before parents. |
| Keys | Keys identify siblings in one child collection and must affect reconciliation. |
| Projection | Transparent wrappers add no hidden native control. |
| Fragments | Multi-child values are accepted only by APIs that support multiple children. |
| Virtualization | Only realized rows own mounted logical nodes and lifecycle state. |

Debug and test builds should check these invariants after reconciliation. They must not remain
assumptions repeated across root, positional, keyed, and templated update paths.

### Phase 1: logical wrapper identity

Introduce a generational logical ID for components, memoized components, providers, and error
boundaries. Allocate the ID before first render so hook setters can retain the exact owner.

The initial logical record should contain only the data needed to remove component identity from
`ControlId`:

```rust,ignore
struct LogicalNode {
    parent: Option<NodeId>,
    native_root: Option<ControlId>,
    kind: LogicalNodeKind,
}
```

Component state continues to hold its `RenderCx`, previous object, previous rendered output, and
context subscriptions, but it is indexed by `NodeId`. The parent relationship crosses intervening
native widgets, so a stateful child remains reachable through a memoized component that renders a
stable widget root.

After the logical path is authoritative:

- `component_instance_overflow` is removed;
- remove component lookup and identity transfer by `ControlId`;
- remove global force-dirty behavior;
- make one update decision responsible for dirty-descendant traversal.

A full arena containing every native widget is not part of this phase. Native controls already have
usable identity, and replacing the entire reconciler at once would create unnecessary migration and
performance risk.

### Phase 2: ownership consolidation

Evaluate the logical graph before expanding it. Move more state into mounted-node ownership only
where the smaller graph cannot provide reliable replacement and teardown.

Candidate ownership currently spread across maps includes:

- header and pane subtrees;
- templated rows;
- custom element handles;
- selection and reorder callbacks;
- pre-unmount callbacks;
- error fallback state.

If these move into a full mounted graph, migrate one category at a time. Positional, keyed, and
templated algorithms should decide correspondence and order, but share the same identity, dirty,
lifecycle, and cleanup rules.

Transparent wrappers may project through arbitrary transparent descendants. Native insertion
algorithms should carry a running projected index rather than repeatedly scanning preceding
logical nodes.

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

The target shape is:

```rust,ignore
struct Reconciler<B> {
    backend: B,
    tree: MountedTree,
    pass: ReconcilePass,
    host: HostContext,
    stats: ReconcileStats,
}
```

`MountedTree` should own node identity, parentage, native projection, children, secondary slots,
and teardown. `ReconcilePass` should own only reusable transient scratch such as forced paths and
keyed-diff buffers. `HostContext` should contain the dispatcher-facing rerender request,
marshaller, host ID, size, DPI, and context environment. `ReconcileStats` should contain counters
only.

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

The existing side state should move as follows:

| Current state | Intended owner |
| --- | --- |
| `children_mirror`, `id_kinds`, header, and pane maps | Mounted native node |
| component instances and native projections | Mounted logical component node |
| error fallback state | Error-boundary node |
| custom handles | Custom node |
| templated state, selection, reorder, and deferred rows | Templated-list node |
| pre-unmount callback | Node lifecycle state |
| forced nodes, forced controls, traversal scratch | `ReconcilePass` |
| marshaller, host ID, size, DPI, rerender request | `HostContext` |

Do not replace the maps with one large per-control struct containing every optional field. Most
controls do not use headers, panes, templating, or custom lifecycle state. Use node-kind enums and
allocate auxiliary state only for node kinds that need it.

The WinUI backend also has a `parent_children` mirror because it converts Reactor's logical child
indices to native visual indices while accounting for phantom controls. Do not merge that backend
detail into `MountedTree` without first changing the backend contract. Two structures with
different purposes are acceptable; two structures claiming to own the same lifecycle are not.

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

Steps 1-3 and 6 are complete. The component portion of step 4 is complete. Child and slot storage
remains sparse inside `MountedTree` rather than adding optional fields to every native node. The
next slice defines logical node kinds for providers and error boundaries instead of adding more
parallel maps. Custom handles remain native-node auxiliary state because they own a native control,
but their teardown should move behind the same mounted-node operation.

### Typed element API

The erased `ElementExt` surface allows modifier calls that compile but cannot affect a native
element. Constructors should retain a concrete wrapper until insertion:

```rust,ignore
component(app, ()) -> ComponentElement<App>
button("Save") -> WidgetElement<Button>
```

Concrete wrappers implement `Into<Element>`. Child builders accept `impl Into<Element>`. Erased
`Element` and logical wrappers do not implement visual modifier traits, so invalid calls fail to
compile:

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

Remove `Group` from mountable `Element`. Use a distinct child collection or fragment type accepted
only by multi-child builders. Components, providers, error boundaries, and single-child controls
continue to accept or return zero or one mountable element.

This makes the current runtime-invalid cases unrepresentable:

- a group as the application root;
- a group returned from a component;
- a group inserted into a single-child control;
- a keyed group whose key is discarded when flattened.

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

### Planned change sequence

1. Add the architecture contract, missing topology regressions, benchmark cases, and minimal
   samples.
2. Introduce logical IDs and parent paths without changing the public API.
3. route state and context dirtiness through logical identity.
4. Remove component collision and global force-dirty mechanisms.
5. Consolidate transparent-wrapper lifecycle and teardown ownership.
6. Introduce concrete public wrappers and compile-time modifier capabilities.
7. Separate multi-child collections from mountable elements.
8. Generate and audit capability implementations.
9. Consolidate remaining mounted ownership only where measurements and invariants require it.
10. Resume dependent features such as typed element references, window lifecycle APIs, and encoded
    image resources.

Do not combine the internal identity migration with the public breaking API change. Each stage
must pass formatting, clippy, headless tests, the relevant WinUI selftests, benchmark comparison,
and its visual sample before the next stage begins.
