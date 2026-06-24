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

`.padding()` is silently ignored on `vstack`/`hstack`: `Padding` belongs to
`Control`, but `StackPanel` derives from `Panel`, so the call compiles with no
effect. Use `.margin(...)` instead. With the `diagnostics` feature,
`diag::unhandled_modifier` warns when a modifier is applied to an element that
can't honor it.

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
turn produce a single render). The `reactor_perf` test app emits CSV compatible
with the C# `microsoft-ui-reactor` `stress_perf` benchmarks; compare against its
`ReactorOptimized` variant (which caches cells like Rust does), not the base
`Reactor`.

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
framework (`D:\git\microsoft-ui-reactor`), already referenced above for the
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
