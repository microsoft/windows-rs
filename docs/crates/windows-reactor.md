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

Panics are programming errors, not recoverable application control flow. Reactor reports a panic
at its outer WinUI callback boundary and aborts rather than continuing with partially mutated
state.

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

Effects are post-commit work. Component renders queue changed effects, and the reconciler flushes
them only after the complete native tree and host root state commit. Nested component effects run
child-first, followed by the root render context's effects. A dependency change commits the native
update first, then runs the previous cleanup and the new effect setup.

An effect from a render or native update that fails before commit never runs and therefore has no
cleanup to invoke. A post-commit effect panic is fatal even though the native tree has already
committed.

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

The current panic contract is:

- programmer panics are reported at the outer FFI boundary and abort the process;
- Reactor does not continue with partially mutated WinUI state after a panic;
- recoverable application failures are represented explicitly as component state or `Result`;
- Reactor has no panic-driven subtree error boundary.

Application setup failures already use `Result`. The arena reconciler must also return WinUI and
backend failures as `Result`, invalidate the complete host, and permit one cold-mount attempt. The
old reconciler still uses internal unwinding to preserve teardown reachability after its backend
panics. The architecture checklist deletes that machinery with the old ownership model rather
than adding another recovery path.

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

The recording backend and most reconciler model tests currently live in `test_reactor` and use the
feature-gated white-box API. This is transitional. Keep public behavior tests in `test_reactor`,
move private arena and failure-model tests into `windows-reactor` unit modules, and remove the
`test` feature as the public boundary is tightened.

Fault injection remains useful for backend `Result` handling and whole-host invalidation. Tests
that exist only to prove panic rollback or subtree recovery should be deleted with those features.
Retain tests for successful ownership, exact-once cleanup, native identity, failed root teardown,
and the cold-mount contract introduced by the replacement host.
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

### Course correction

The current reconciler is harder to stabilize than the supported application model requires. It
keeps four overlapping representations synchronized during partial mutation:

1. the previous `Element` tree;
2. logical component and provider records;
3. the `MountedTree` native ownership mirror;
4. WinUI controls and the `WinUIBackend` side tables.

Panic rollback, selective dirty-path rendering, templated realization, and transparent logical
wrappers all cross these representations. A failure can occur after any WinUI mutation, which
turns ordinary reconciliation into a transaction over an API that provides no transaction. The
recent stabilization work made these states explicit and testable, but also demonstrated that
preserving this model requires too much recovery code.

The next phase replaces the model rather than adding more rollback or ownership machinery. Feature
work remains frozen until the replacement core meets the exit criteria below.

### Core decisions

| Area | Decision |
| --- | --- |
| Panics | Programmer panics are fatal. Reactor reports them at its outer FFI boundary and aborts. |
| Recoverable failures | WinUI and backend failures use `Result`, not panic recovery. |
| Failed mutation | Discard the host and attempt one cold mount after a backend failure. |
| Successful updates | Retain native controls so WinUI-owned state survives. |
| Error boundaries | Panic-driven subtree error boundaries are not part of the core. |
| Rendering | Initially rerender the complete logical tree on state or context changes. |
| Memoization | Add selective rendering only after the simple model is correct and measured. |
| Virtualization | Defer templated realization until ordinary ownership is complete. |
| Backend | Keep the backend private; it is an implementation and test boundary, not app API. |
| Testing access | Remove the `test` feature and keep white-box tests inside the crate. |

Application failures that should render alternate UI must be represented explicitly as component
state or `Result` values. Panics remain invariant violations and programming errors. Reactor must
not continue after one while holding partially mutated WinUI state.

A cold mount is a failure-recovery mechanism, not the normal render strategy. Rebuilding every
native control on every render would lose focus, caret position, scroll offsets, WebView state,
animations, and other WinUI-owned state.

### Target mounted model

The replacement core uses one retained arena. Every logical and native node has one `NodeId`, one
parent, and one owner:

```rust,ignore
struct MountedNode<H> {
    parent: Option<NodeId>,
    element: Element,
    kind: MountedNodeKind<H>,
}

enum MountedNodeKind<H> {
    Native(NativeNode<H>),
    Component(ComponentNode),
    Provider(ProviderNode),
}

struct NativeNode<H> {
    handle: H,
    children: Vec<NodeId>,
    subscriptions: Subscriptions,
    slots: NativeSlots,
}
```

The exact types may change during implementation, but the ownership rules may not:

- the arena is the retained model and mounted ownership tree;
- a component owns its render context and one mounted child;
- a provider owns one child and its scoped values;
- a native node owns its backend handle, children, secondary slots, callbacks, and revokers;
- the backend does not maintain a second control-identity graph;
- native handles are passed directly rather than resolved through a public `ControlId` registry;
- headers, panes, and other secondary content are explicit node slots;
- dropping a native node revokes its subscriptions before releasing its handle.

One private backend trait may remain so the arena can run against WinUI and a recording backend.
Its associated handle is owned by the mounted node. The WinUI implementation should not need a
`controls: HashMap<ControlId, Handle>` lookup table.

Stable event trampolines hold replaceable callback cells. Updating an element replaces the current
callback in the cell rather than detaching and reattaching the native event. Event revokers live
with the native node and are released through normal ownership.

### Public boundary

The supported application API should contain only:

- application and window hosting;
- `Component` and `RenderCx`;
- hooks;
- elements, widget builders, and modifiers;
- callbacks and typed element references;
- explicit graphics integration points.

`Backend`, `Reconciler`, `ControlId`, `WinUIBackend`, internal dispatchers, mounted nodes, and
mutable `RenderHost` access are implementation details. Repository samples do not depend on these
low-level interfaces, so the pre-release crate should remove them rather than preserve accidental
compatibility.

The existing typed element, fragment, and typed-reference contracts below remain useful. They
constrain application code without requiring the current reconciler representation.

### Test structure

Follow the useful part of the C# Reactor strategy - separate tests by the boundary they prove - but
do not copy its feature-heavy reconciler architecture:

| Tier | Location | Responsibility |
| --- | --- | --- |
| Private unit | `windows-reactor` | Arena, child diff, hooks, callbacks, and teardown |
| Public headless | `test_reactor` | Supported application API and render behavior only |
| WinUI selftest | `test_reactor_selftest` | Native identity, focus, layout, and shutdown |

White-box tests must not require public production methods. Move them into crate unit modules and
remove the `test` Cargo feature. Keep characterization tests from the stabilization work when they
express behavior retained by the new contract; delete tests whose only purpose is proving removed
panic-recovery machinery.

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
provider, group, or empty element.

`with_key` and `provide` are structural operations rather than native visual modifiers. `KeyExt`
provides reconciliation identity, while `ProvideExt` wraps anything convertible into `Element` in
a context provider. The former `ElementExt` trait and its public modifier accessor have been
removed.

Widget constructors return concrete builders that implement native capabilities and
`Into<Element>`. Logical constructors such as `component` and `memo` return `Element`, so only
structural operations remain available on them. Child builders accept `impl Into<Element>`, and
invalid native modifier calls fail to compile:

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
`RelativePanel`. It does not implement `Into<Element>`, so components, providers, application
roots, and single-child controls continue to accept or return zero or one mountable element:

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

### Architecture migration checklist

Each item should land as a reviewable PR that leaves the existing public widget API working. Do not
build compatibility mirrors between the old and new ownership models. A temporary adapter is
acceptable only when it is read-only and its removal is assigned to the next checklist item.

#### 1. Freeze and define the contract

- [x] Seal undocumented reconciler mutation methods and reduce the normal reconciler surface.
- [x] Document panics as fatal programming errors and backend failures as `Result` values.
- [ ] Define whole-host invalidation and one-attempt cold mounting after a backend failure.
- [ ] Classify each current feature as core, deferred, or removed before moving its code.
- [ ] Keep new feature work frozen until this checklist is complete.

#### 2. Remove recovery complexity

- [x] Remove `ErrorBoundaryElement`, `error_boundary`, fallback state, and subtree panic recovery.
- [x] Remove `App::on_fault`, `Fault`, and the recovery sample, then replace outer render and
  callback log-and-continue handling with report-and-abort handling.

Do not retrofit full `Result` propagation through the reconciler that this plan deletes. Until the
arena path replaces it, the old reconciler may keep its internal unwind guards, poisoning flags,
and teardown-retry state. Continuing after a panic is removed first because it is less safe than
temporary process termination for a backend failure.

#### 3. Tighten the public and test boundaries

- [ ] Stop glob-reexporting backend, reconciler, and engine implementation details from `lib.rs`.
- [ ] Make `Backend`, `Reconciler`, `ControlId`, and `WinUIBackend` private, including mutable
  reconciler access.
- [ ] Remove the `test` Cargo feature and `ReconcilerTestExt`.
- [ ] Move white-box reconciler, backend-fault, and model tests into `windows-reactor` unit modules.
- [ ] Keep `test_reactor` tests limited to supported public behavior.
- [ ] Record the resulting public API and add a check that prevents accidental expansion.

#### 4. Remove selective-render bookkeeping

- [ ] Make every state, context, size, and DPI change request a root render.
- [ ] Render every mounted component during that pass unless ordinary element equality skips native
  work.
- [ ] Remove context subscriber sets, forced logical nodes, forced controls, and dirty-ancestor
  reconstruction.
- [ ] Remove component memoization paths that require selective descendant traversal.
- [ ] Measure the simple root-render model before reintroducing any optimization.

#### 5. Introduce one mounted arena

- [ ] Add a private generational `NodeId` arena containing native, component, and provider nodes.
- [ ] Give every node one parent and make child ownership structural.
- [ ] Store the previous element and component render state on the arena node that owns them.
- [ ] Store native handles directly on native nodes and remove backend control-ID lookup.
- [ ] Store normal children, headers, panes, and other secondary content as explicit node slots.
- [ ] Store event callback cells and RAII revokers on native nodes.
- [ ] Make fallible backend operations return `Result` through the arena reconciler.
- [ ] Stop mutation on the first backend error and mark the complete host invalid.
- [ ] Make one child-first arena traversal the only teardown path.
- [ ] Delete `MountedTree`, `MountedLogicalTree`, projection maps, and replaced side tables as their
  responsibilities move.

#### 6. Restore the minimum complete reconciler

- [ ] Mount and update ordinary native widgets through the arena.
- [ ] Support components, hooks, providers, post-commit effects, and exact-once cleanup.
- [ ] Support positional children before keyed reconciliation.
- [ ] Add keyed reconciliation only after positional ownership passes the model tests.
- [ ] Preserve typed element references across update and clear them before native release.
- [ ] Preserve focus, text input state, scroll position, and native identity on successful updates.
- [ ] Drop an invalid host and permit one cold-mount attempt with a fresh arena and backend state.
- [ ] Reject stale asynchronous updates after host replacement or unmount.

#### 7. Re-evaluate deferred features

- [ ] Decide whether templated realization belongs in the core or a later optional layer.
- [ ] If retained, implement realized rows as arena-owned subtrees using the same teardown rules.
- [ ] Reconsider component memoization only with a measured workload that root rendering fails.
- [ ] Reconsider recoverable UI boundaries only with an explicit `Result`-based design, never panic
  rollback.
- [ ] Keep custom backend and arbitrary native-extension APIs out of the public core.

#### 8. Cut over and delete the old model

- [ ] Switch the production host to the arena reconciler.
- [ ] Remove old adapters immediately after cutover.
- [ ] Delete old reconciliation poisoning, teardown-retry, unwind-rollback, and recovery-test
  machinery.
- [ ] Delete obsolete fault injectors, maps, flags, and documentation.
- [ ] Recount production source, containers, unwind boundaries, and public API against this
  baseline.
- [ ] Remove the feature freeze only after the exit criteria pass.

### PR discipline

Each architecture PR must:

1. preserve or add a deterministic test for the contract being moved;
2. move one ownership or failure boundary;
3. delete the superseded state or path in the same PR;
4. run the smallest headless suite plus relevant WinUI selftests;
5. compare allocations when a mounted-node representation or hot path changes;
6. update this checklist without adding a historical diary.

### Exit criteria

The feature freeze ends when:

- the normal public API contains no backend or reconciler mutation surface;
- the `test` feature and public white-box hooks are gone;
- one arena is the only retained logical and native ownership model;
- the backend owns no parallel control-identity graph;
- successful updates preserve required WinUI-owned state;
- a backend failure either cold-mounts a fresh host once or terminates without partial reuse;
- panic recovery and reconciliation rollback machinery are gone;
- cleanup and event revocation run exactly once through structural ownership;
- headless model tests, public API tests, WinUI selftests, coverage floors, and allocation gates
  pass;
- production source, public surface, and state-container counts are materially below the current
  baseline.