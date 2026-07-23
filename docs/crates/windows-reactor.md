# windows-reactor

> A declarative, React-style UI library for Rust, backed by WinUI 3.

- Not published to crates.io
- [Getting started](../../crates/libs/reactor/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)

`windows-reactor` lets you describe a WinUI 3 user interface as a function of
state. You write a render function that takes a `RenderCx` and returns an
`Element`. The reactor diffs the result against the live visual tree and applies
only the changes. State lives in hooks such as `cx.use_state`. Updating state
schedules a re-render.

## Getting started

A reactor app needs three things: the crate dependency, a render function, and a
`build.rs` that stages the Windows App SDK runtime with
[`windows-reactor-setup`](windows-reactor-setup.md).

`Cargo.toml`:

```toml
[dependencies]
windows-reactor = "..."

[build-dependencies]
windows-reactor-setup = "..."
```

`build.rs`. Pick the helper that matches your deployment model:

```rust,ignore
fn main() {
    // For a self-contained app that carries its own runtime:
    windows_reactor_setup::as_self_contained();
    // Other options: as_framework_dependent(), as_example().
}
```

`src/main.rs`. A render function plus `App`:

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

`bootstrap()` initializes the Windows App SDK runtime. Call it once at startup.
`App::new()` is a builder. Common options are `title`, `inner_size`, `backdrop`
(for example `Backdrop::Mica`), `icon` (path to an `.ico` file), `fullscreen`,
and `presenter`. `render(app)` takes your `Fn(&mut RenderCx) -> Element` and runs
the message loop.

Reactor catches panics at the FFI boundaries it owns (render and event callbacks,
and `ErrorBoundary`) and converts them to errors, so they never unwind across the
WinUI ABI. It does not install a global panic hook. For panics that escape those
boundaries, add `panic = "abort"` to your release profile. The process then
terminates cleanly instead of unwinding into WinUI's C++ frames, which is
undefined behavior:

```toml
[profile.release]
panic = "abort"
```

Set `RUST_BACKTRACE=1` when you want a backtrace.

## State with hooks

Hooks are methods on `RenderCx`. They give a render function persistent state
without globals or `thread_local!`. The most common hooks:

- `use_state(initial)` returns `(value, SetState)`: a value plus a setter.
  `set.call(new_value)` updates the slot and schedules a re-render.
- `use_ref(initial)` returns `HookRef`: mutable storage that does not trigger a
  re-render. Read with `.borrow()`, write with `.borrow_mut()` or `.set(v)`. Use
  it for animation frame counters and cached resources.
- `use_memo(deps, factory)`: recompute a value only when `deps` change.
- `use_effect(deps, f)` and `use_effect_with_cleanup`: run side effects when
  `deps` change.
- `use_reducer` and `use_reducer_fn`: state driven by an update or reducer
  instead of a plain setter.
- `use_resource(fetcher, deps)` returns `Resource<T>`: async data loading with
  loading and error states. A `Resource` converts into an `Element`.
- `use_context(&context)`: read a value provided higher in the tree.
- `use_open_window()`: returns an opener for secondary top-level windows (see
  [Multiple windows](#multiple-windows)).

```rust,ignore
fn counter(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);
    button(format!("Clicks: {count}"))
        .on_click(move || set_count.call(count + 1))
        .into()
}
```

The `apps/examples` and `minimal/examples` directories include a focused sample
for each hook (`use_state`, `use_ref`, `use_memo`, `use_effect`, `use_reducer`,
`use_resource`, `use_callback`, `use_color_scheme`, and more).

## Building the UI

Build elements with plain builder functions. Each returns a widget that becomes
an `Element` with `.into()`. Containers take a tuple of children.

- Text: `text_block(content)` with `.bold()`, `.semibold()`, `.font_size(..)`,
  `.wrap()`, `.selectable()`, and type-ramp helpers (`title`, `subtitle`, `body`,
  `caption`).
- Buttons: `button(content)` with `.on_click(..)`, `.accent()`, `.subtle()`,
  `.enabled(..)`, `.icon(..)`, `.flyout(..)`, `.menu_flyout(..)`.
- Icons: any control that takes an icon (`button`, `NavViewItem`, command-bar
  buttons, `selector_bar_item`) accepts `impl Into<Icon>`. `Icon` has three
  kinds: `Symbol(Symbol)` for a built-in system glyph (a bare `Symbol` converts
  automatically, so `.icon(Symbol::Home)` keeps working), `Icon::bitmap(uri)` for
  an image (`BitmapIcon`), and `Icon::font(glyph)` or
  `Icon::font_family(glyph, family)` for a font glyph (`FontIcon`). Sample:
  `cargo run -p reactor_samples --example icon_elements`.
- Layout: `vstack((..))` and `hstack((..))` with `.spacing(..)`; `grid((..))`
  with `.rows([..])` and `.columns([..])` (using `GridLength::STAR` and
  `GridLength::Auto`) and per-child `.grid_row(n)` and `.grid_column(n)`.

About 60 WinUI controls are wrapped, including `check_box`, `combo_box`,
`slider`, `list_view`, `tree_view`, `navigation_view`, `tab_view`, `pivot`,
`text_box`, `number_box`, `color_picker`, `calendar_view`, `content_dialog`,
`info_bar`, `teaching_tip`, and `command_bar`. See the
[full catalog](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor/src/widgets).

Layout and appearance modifiers are available on any `Element` through the
`ElementExt` trait: `.margin(..)`, `.padding(..)`, `.width(..)`, `.height(..)`,
`.horizontal_alignment(..)`, `.vertical_alignment(..)` (with `HorizontalAlignment`
and `VerticalAlignment`), `.background(..)`, `.foreground(..)`, `.opacity(..)`, and
transition helpers such as `.with_opacity_transition(..)`. Spacing values use
`Thickness` (with `Thickness::uniform(..)`).

## Handling events

Event handlers take closures. `button(..).on_click(move || ...)` is the most
common. Pointer and keyboard handlers live on `ElementExt`: `.on_tapped(..)`,
`.on_pointer_pressed(..)`, `.on_pointer_released(..)`, `.on_pointer_moved(..)`,
`.on_pointer_entered(..)`, `.on_pointer_exited(..)`, `.keyboard_accelerator(..)`.
You can pass a `SetState` or `Dispatch` directly wherever a handler is expected
(through `IntoCallback`).

### Handler identity

When a value-carrying event just forwards its argument to a setter, pass the
setter directly instead of wrapping it in a closure:

```rust,ignore
// Prefer this. It is shorter, and the handler keeps a stable identity:
text_box(text).on_text_changed(set_text)

// Avoid this. A fresh closure is allocated every render:
text_box(text).on_text_changed(move |value| set_text.call(value))
```

This is not only cosmetic. Setters from `use_state` and `use_reducer` are
memoized per hook slot. Passing one straight through hands the reconciler the
same handler identity each render, so the diff can skip the whole control. An
inline closure (`move |v| set.call(v)`) allocates a fresh identity every render,
so the control is always re-diffed and its WinUI event re-bound.

When a handler must compute a value or run extra logic, wrap it in
`cx.use_callback(deps, ...)` to memoize it and recover a stable identity for hot
paths. For a unit event (`on_click`) that stores a fixed or pre-computed value,
`SetState::setter(value)` is shorthand for `move || set.call(value)`:
`button("Reset").on_click(set_count.setter(0))`.

## Multiple windows

`App::run` opens the primary window. An app can open more top-level windows at
runtime with the `ReactorWindow` builder. Each window hosts its own reactor tree
with its own hooks, state, and render function, while sharing the one UI thread
and message loop. WinUI is single-threaded apartment, so every window runs on the
same thread.

```rust,ignore
fn app(cx: &mut RenderCx) -> Element {
    button("Open counter window")
        .on_click(|| {
            // Opens immediately on the UI thread and returns a handle.
            let _ = ReactorWindow::new()
                .title("Counter")
                .inner_size(320.0, 220.0)
                .render(counter); // counter: Fn(&mut RenderCx) -> Element
        })
        .into()
}
```

`ReactorWindow` mirrors the `App` window options (`title`, `inner_size`,
`inner_constraints`, `presenter`, `fullscreen`, `backdrop`, `icon`). `.render(f)`
takes a `Fn(&mut RenderCx) -> Element`. `.open(factory)` takes any `Component`.
Both run synchronously on the current UI thread (unlike `App::run` there is no
`Send` bound) and return `Result<WindowHandle>`.

- `WindowHandle` is a control handle for an open window. The registry owns the
  window's host, so the handle is just an identifier. Call `.close()` to close
  the window. Dropping the handle does nothing and never affects the window's
  lifetime. The handle is `!Send` and `!Sync`: you can only control a WinUI
  window from the UI thread that opened it.
- Closing the last window exits. Reactor tracks every open window. When the last
  one closes (primary or secondary), the process exits. Closing any earlier
  window just drops that window.
- `cx.use_open_window()` returns a small `Copy` opener you can capture into
  handlers as an alternative to naming `ReactorWindow` directly. `opener.render(f)`
  and `opener.open(factory)` open a default-configured window.

Per-window themes are not available yet. The requested color scheme is app-global.

## Graphics integration

For custom 2D drawing, host a [`windows-canvas`](windows-canvas.md) surface with
`animated_canvas(draw)`. Enable reactor's `canvas` feature, which pulls in
`windows-canvas`. It returns a `SwapChainPanel` element that redraws every frame
and recovers from device loss automatically. See the `canvas` samples. To render
on a device the app already created and shares across many surfaces, use
`animated_canvas_with_device(device, draw)` with a cloneable
`windows_canvas::GpuDevice` (a clone shares the same underlying devices).

For content that is static between updates, `CanvasImageSource` draws on demand
into a `SurfaceImageSource` shown with the `Image` widget, and redraws only when
you call `draw`. `Image::on_mounted` yields an `ImageHandle` whose
`on_rasterization_scale_changed` reports the host DPI scale, so the surface stays
crisp across monitor moves. See the canvas `image_source` sample.

For an on-demand surface that still presents through a swap chain (lower latency
than a `SurfaceImageSource`, but only when the data changes), `CanvasSwapChain`
hosts a composition swap chain on a `SwapChainPanel`. Create it in the panel's
`on_mounted`, store it in `use_ref`, and `draw` from a `use_effect` on data
change. It stays idle when nothing changes. See the canvas `chart` sample.

`animated_canvas`, `CanvasImageSource`, and `CanvasSwapChain` are reactor exports.
They own the WinUI element harness and build on the safe drawing surface that
`windows-canvas` provides. For raw Direct3D, the `swap_chain_panel` sample drives
a `SwapChainPanel` with `on_rendering`.

## Web content integration

To host a browser, use [`windows-webview`](windows-webview.md)'s
`webview(on_ready)` and enable its `reactor` feature. It returns a `WebView2`
element backed by the WinUI XAML `WebView2` control, and hands you a ready
`WebView` once the browser initializes. See the `reactor/webview` sample. The
`as_self_contained()` setup carries the required
`Microsoft.Web.WebView2.Core.dll` automatically. For how the widget bridges the
WinRT control to the COM `ICoreWebView2`, see
[`windows-webview`](windows-webview.md).

## Samples

The [`crates/samples/reactor`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
tree is the best reference:

- `samples`: the smallest app plus an `examples/` folder with about 90 focused
  per-control and per-hook examples (`counter`, `calculator`, `navigation_view`,
  `list_view`, `content_dialog`, `color_picker`, `secondary_window`, and more).
- `apps`: complete applications (`notepad`, `solitaire`, `minesweeper`,
  `tictactoe`, `dotsweeper`).
- `gallery`: a WinUI-gallery-style shell with navigation across many controls.
- `direct2d` and `swap_chain_panel`: hosting Direct2D and Direct3D content.
- `webview`: hosting a WebView2 browser through `windows-webview`'s `reactor`
  feature.
- `framework_dependent` and `self_contained`: the two deployment models, which
  differ only in `build.rs`.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is for
contributors and is not needed to use `windows-reactor`.

### How it's built

The hooks runtime, element tree, reconciler, and WinUI backend are hand-written.
`tool_reactor` generates the per-widget dispatch from
`crates/tools/reactor/src/winui.toml` plus the WinUI `.winmd` metadata:

| Generated file | Contents |
|----------------|----------|
| `src/generated.rs` | per-widget `bindings()` helpers |
| `src/backend/winui/generated_set_prop.rs` | property setter dispatch |
| `src/backend/winui/generated_attach_event.rs` | event handler dispatch |
| `crates/tools/reactor/src/generated.txt` | binding filter entries |

The tool is metadata-driven: it infers the setter pattern, value type, and
event-invoke pattern from `.winmd`. TOML keys are WinUI metadata names, and only
non-standard mappings need overrides. Regenerate with `cargo run -p tool_reactor`,
then verify with `cargo check -p windows-reactor`.

The WinUI and Windows App SDK `.winmd` files under `crates/tools/reactor/winmd/`
are committed (they are also read by `tool_webview` and `tool_composition`) but
treated as generated. `tool_reactor` refreshes them on every run from the pinned
`WINDOWS_APP_SDK_VERSION`: it downloads the `Microsoft.WindowsAppSDK` metapackage,
resolves the component versions from its nuspec, and copies each component's winmd
(plus the WebView2 `Core.winmd` at `tool_webview`'s pin) into place. Bumping that
one constant updates them all, and `gen.yml`'s zero-diff check catches any drift.
See [Dependencies](../dependencies.md).

Generated dispatch falls through to hand-written backend code for cases too
complex to express declaratively (Button icon and text layout, NavigationView
menu items, ContentDialog modal popup). Never edit the generated files or
`generated.txt` by hand.

### Bindings

`src/bindings.rs` holds the flat WinUI and COM interop bindings the hand-written
backend calls. `tool_reactor` generates it (driving `windows-bindgen` in
`--flat --minimal` mode) from two filter files: the hand-maintained
`crates/tools/reactor/src/base.txt` (the base-interface method list) and the
tool-produced `generated.txt` (per-widget entries). Regenerate it, and the
selftest copy at `crates/tests/libs/reactor_selftest/src/bindings.rs`, with the
same `cargo run -p tool_reactor`. Raw metadata names apply: `get_Prop`, `put_Prop`,
`add_Event`, `remove_Event`.

To prune or extend bindings: edit `base.txt`, regenerate, and let the compiler
errors show the methods you still need (`SetX` maps to `put_X`, `X()` maps to
`get_X`). Add them as `Ns.IFace::{put_X, get_Y}` under the right base interface.
This also covers the Win32 COM interop interfaces (for example
`ISwapChainPanelNative`, `ISurfaceImageSourceNativeWithD2D`). Listed methods get
full vtable entries, unlisted methods become `usize` slots, and the type closure
is computed automatically.

### COM pitfalls

These bite anyone editing the backend by hand. The generated code already follows
them.

- Classes deref to their default interface. Do not `cast` to it. Call the method
  directly (`button.SetFlyout(&flyout)`, not
  `button.cast::<IButton>()?.SetFlyout(...)`). This applies to event-handler
  `sender` and `args` too: the delegate hands you the concrete arg class and the
  `sender` is the control, and both already deref to their default interface. So
  `args.SelectedItem()` and a control captured at attach
  (`let control = h.clone();`) read at zero per-event QI, versus
  `args.cast::<I...Args>()` or `sender.cast::<TextBox>()` on every fire. Only cast
  to non-default parent interfaces (for example `Button` to `IContentControl` or
  `IControl`). Watch the static type, not the name:
  `DropDownButton.cast::<IButton>()` looks redundant, but `IButton` is a parent
  there (the default is `IDropDownButton`), so it is a genuine cast.
- `Param<T>` removes parent-class casts. A method taking `impl Param<Brush>`
  accepts a `SolidColorBrush` directly, with no `cast::<Brush>()`.
- Use `From` or `into()`, not `cast`, for `IInspectable`, and plain `None` for
  optional inspectable parameters.
- `put_IsChecked` (CheckBox) takes `Option<bool>`. It is a tri-state nullable
  boolean.
- TextBox and PasswordBox need get-before-set to avoid resetting the caret.
- ProgressBar uses `IRangeBase` for Value, Min, and Max. ProgressRing has direct
  setters.
- ContentDialog needs a `XamlRoot` from a live element, so it requires backend
  access.
- Font properties are shared across `IControl`, `ITextBlock`, and
  `IRichTextBlock`.

### Padding, background, and foreground dispatch

`Padding` has no single owning interface. `Control`, `Border`, `StackPanel`,
`TextBlock`, and `RichTextBlock` each declare their own. `set_padding`
(`backend/winui/mod.rs`) dispatches on the `Handle` variant: it calls the setter
directly on `Border`, `StackPanel`, `TextBlock`, and `RichTextBlock` through their
default interface, and falls back to a single `IControl` cast for everything else.
Containers that lack a `Padding` property (for example a bare `Panel` or `Grid`)
fall through to `diag::unhandled_modifier`, which warns in debug builds. Use
`.margin(...)` there instead.

`Background` and `Foreground` follow the same pattern and are exposed as the
`ElementExt` modifiers `.background(...)` and `.foreground(...)`. `Border` handles
them through its default interface; every other handle falls back to a single
`IControl` cast (`set_background` and `set_foreground`). `BorderBrush` and
`BorderThickness` use the same `IControl` fallback (`set_border_brush` and
`set_border_thickness`) but are not `ElementExt` modifiers. They are opt-in
per-widget builders, currently exposed by `Border` and `TextBox`.

### Threading

Reactor runs on a WinUI STA thread and keeps per-thread state in `thread_local!`
slots. Two categories exist:

- STA-affine COM handles and caches (the host, application, root window and
  framework element, and the shared `DataTemplate`) must stay thread-local. They
  hold COM objects that are only valid on the UI thread.
- One-shot latches and per-thread scalars (pending theme and title-bar requests,
  current color scheme) are thread-local only because the public API exposes them
  as free functions (`set_requested_theme` and similar).

### Error model

Reactor sits between the developer's Rust closures and WinUI's COM and
`extern "system"` delegates. It handles failures by where they happen:

- Synchronous, pre-loop setup (`bootstrap`, icon path validation) returns a
  `Result` from `run` or `bootstrap`. This is the only place a `Result` reaches
  the caller, so validate configuration up front.
- Failures inside UI-thread callbacks (render, event handlers, timers,
  `on_rendering`) go to one reactor-owned fault boundary, not a `Result`. Reactor
  catches panics at the entry points it owns (`Callback::invoke`, the
  `DispatcherTimer` tick, `on_rendering`, and the render pass) and delivers them
  to a developer-supplied `App::on_fault(|fault| ...)` hook (default:
  log-and-continue). The catch is context-aware: a callback that panics during a
  render pass is left to propagate so `error_boundary` can recover the subtree
  first. Only panics outside render, or escaping every boundary, reach `on_fault`.
  This lives in `fault.rs`.
- Best-effort backend property application uses one helper: `diag::warn` (with
  `diag::dropped`, which reports the dropped `Result`'s call site through
  `#[track_caller]`). It warns in debug and is a no-op in release.
- `panic!` is only for programmer errors and invariant violations (rules of
  hooks, type mismatch, `EventHandler` variant mismatch) in `engine.rs` and
  `backend/mod.rs`.

The fault boundary relies on `panic = "unwind"` (the Cargo default). Under a
`panic = "abort"` profile the whole model is bypassed and every panic aborts.

### Performance notes

The reconciler skips unchanged controls (kind match plus shallow compare), so at
steady state it creates no new WinUI controls. The diff and patch cost is then
dominated by COM property-set calls. Two design choices follow:

- No element pooling. With zero controls created at steady state there is nothing
  to recycle.
- No rerender depth guard. The render loop is non-recursive: `set_state` during a
  render sets a dirty flag and enqueues the follow-up render through the
  dispatcher rather than re-entering, so unbounded recursion is impossible.

State writes are coalesced through the dispatcher, so many `set_state` calls in
one turn produce a single render. Steady-state heap allocations per render are
zero for `use_state`, `use_reducer`, `use_callback`, and `use_ref`.

Each control is held as a `Handle` enum whose variant is the concrete WinUI class
(`Handle::TextBlock(bindings::TextBlock)`). Shared modifiers (`padding`,
`foreground`, `font_*`) match on the `Handle` variant instead of probing
interfaces with `cast`, because a class derefs to its default interface at zero QI
and a failing `QueryInterface` on XAML's aggregated objects is expensive. The same
rule applies to event handlers: capture the typed handle at attach and read
through `Deref` rather than casting `sender` or `args` on every fire.

### Testing

Unit tests live in `test_reactor` (headless). Integration tests live in
`test_reactor_selftest`, which launches a real WinUI window. Pass `--headless`
for CI.

Most pointer handler behavior (attach, detach, memoization, slot changes) is
covered headlessly in `test_reactor` through the `RecordingBackend`. Paths that
need a live WinUI thread are covered by selftest fixtures instead:

- `Pointer_Injection_Gesture` drives real OS mouse input through the WinRT
  `InputInjector` and asserts the `on_pointer_*` callbacks fire with the right
  position and button flags.
- `Timer_*` and `Rendering_Subscription_*` cover `DispatcherTimer` and the
  `CompositionTarget::Rendering` subscription (`on_rendering`).

Fixtures that need real OS input or composition frames record a TAP `# SKIP`
(never a failure) when the host cannot deliver them, so they do not flake CI.

The `RecordingBackend` harness lives in the `test_reactor` crate, not in
`windows-reactor`, so it adds no weight to normal builds. Do not put
`#[cfg(test)]` modules inside the published library crates. Put the test in the
matching `test_*` crate. If it needs an internal item, expose that item behind the
existing `test` feature that the test crates enable and published builds leave off.

### Reactor and canvas naming

`windows-reactor` and `windows-canvas` define some of the same short names for
different domains. The rule: canvas keeps the short name (it owns user-facing draw
loops) and reactor takes a domain-prefixed alternative.
