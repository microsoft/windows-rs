# windows-reactor module reference

This document maps every module in the `windows-reactor` crate and explains how
they relate to each other.

## Public API surface

Everything flows through `lib.rs`, which re-exports from private modules into a
single flat namespace. Users write `use windows_reactor::*` and never need to
name an interior module.

```text
lib.rs
├── pub use app::*          → App
├── pub use backend::*      → Backend, WinUIBackend, WinUIDispatcher, Dispatcher, …
├── pub use bootstrap::*    → bootstrap(), BootstrapHandle
├── pub use element::*      → Element, GroupElement, ElementExt, …
├── pub use engine::*       → Component, RenderCx, RenderHost, …
├── pub use hooks::*        → DispatcherTimer, Rendering, on_rendering
├── pub use host::*         → ReactorHost, Backdrop, PresenterKind, …
├── pub use interaction::*  → Callback, Context, Resource, …
├── pub use reconciler::*   → Reconciler, RealizationQueue, …
├── pub use style::*        → Thickness, Modifiers, Theme, Animation, …
├── pub use widget::*       → Widget trait, PropBindings, RichText, …
├── pub use widgets::*      → 57 widget constructors
├── pub use windows_core    → Error, Interface, Result
└── pub use windows_time    → DateTime, TimeSpan
```

## Module tree

```text
src/
├── lib.rs
├── app.rs                App builder: title, size, backdrop, run/render
├── app_shim.rs           WinUI Application override plumbing (internal)
├── bootstrap.rs          bootstrap() → BootstrapHandle
├── diagnostics.rs        Panic hook, crash logs, ExpectPanicGuard
│
│   ── core modules ──
├── engine.rs             Component, ComponentElement, RenderCx (all hooks),
│                         RenderHost, RenderStats
├── element.rs            Element enum, GroupElement, ElementExt, IntoElements,
│                         ErrorBoundary, CustomElement
├── style.rs              Thickness, WindowSize, Modifiers (margin, padding,
│                         brushes, animation, grid, accessibility, tooltips),
│                         Color, Brush, GridLength, Theme, ColorScheme
├── widget.rs             Widget trait, Children, Binding, PropBindings,
│                         Prop, PropValue, Event, EventHandler,
│                         RichText types, TemplatedList types
├── interaction.rs        Callback, IntoCallback, impl_rc_fn_wrapper!,
│                         KeyboardAccelerator, Context, ContextStack,
│                         Resource, MutationState, MutationTrigger
├── reconciler.rs         Reconciler<B: Backend>
│   └── reconciler/       5 private submodules: child, diff_helpers,
│                         templated, widget_dispatch, wrappers
├── widgets/              57 per-control files (one per WinUI control)
│   └── mod.rs            Re-exports + widget_header! macro
│
│   ── backend ──
├── backend/
│   ├── mod.rs            Backend trait, ControlId, ControlKind, Prop/Event
│   │                     enums, Dispatcher/SendDispatcher traits,
│   │                     UiMarshaller, test dispatchers (RunOnDemand,
│   │                     Channel, Recording), WinUIDispatcher
│   └── winui/
│       ├── mod.rs        WinUIBackend: impl Backend (Handle enum, XAML ops)
│       ├── convert.rs    Rust ↔ XAML type conversions
│       ├── diag.rs       Missing-feature warnings
│       ├── generated_set_prop.rs      (tool_reactor)
│       └── generated_attach_event.rs  (tool_reactor)
│
│   ── WinUI integration ──
├── hooks.rs              DispatcherTimer, Rendering, on_rendering()
├── host.rs               ReactorHost, Backdrop, PresenterKind, theme, titlebar
│
│   ── generated ──
├── bindings.rs           Raw WinUI COM/WinRT bindings (tool_bindings, ~900 KB)
└── generated.rs          Widget → PropBindings mapping (tool_reactor)
```

## Key modules

### `engine.rs` — component model and hooks

The heart of the reactor. Defines `Component` (the trait every user component
implements), `RenderCx` (the context object passed to `render`, carrying all
hooks), and `RenderHost` (the framework-facing driver that owns the component
tree and coordinates renders).

All hooks are methods on `RenderCx`:

| Hook | Signature | Purpose |
|------|-----------|---------|
| `use_state` | `(T) → (T, SetState<T>)` | Local component state with setter. |
| `use_reducer` | `(T) → (T, Updater<T>)` | Functional state updates via `f(prev) → next`. |
| `use_reducer_fn` | `(R, S) → (S, Dispatch<A>)` | Reducer with explicit action type. |
| `use_ref` | `(T) → HookRef<T>` | Persistent mutable ref that survives re-renders. |
| `use_memo` | `(deps, factory) → T` | Cached computation, recomputed when deps change. |
| `use_callback` | `(deps, f) → Callback<A>` | Stable callback identity, rebuilt when deps change. |
| `use_effect` | `(deps, f)` | Side-effect after render, re-runs when deps change. |
| `use_effect_with_cleanup` | `(deps, f) → cleanup` | Effect with teardown on re-run or unmount. |
| `use_context` | `(Context<T>) → T` | Read a value from an ancestor `Context` provider. |
| `use_async_state` | `(T) → (T, AsyncSetState<T>)` | Thread-safe state slot for async writes. |
| `use_resource` | `(fetcher, deps) → Resource<T>` | Async data fetch, refetches when deps change. |
| `use_mutation` | `() → (MutationState<T>, MutationTrigger<T>)` | Async write with lifecycle tracking. |
| `use_color_scheme` | `() → ColorScheme` | Current light/dark theme. |
| `use_inner_size` | `() → WindowSize` | Current window inner dimensions. |
| `use_dpi` | `() → u32` | Current display DPI. |
| `use_ui_marshaller` | `() → UiMarshaller` | Thread-safe handle to enqueue work on the UI thread. |

### `backend/` — backend contract and WinUI implementation

`backend/mod.rs` defines the `Backend` trait — the abstract interface the
reconciler uses for all XAML operations — plus supporting types (`ControlId`,
`ControlKind`, `Prop`, `Event`, dispatcher traits, test doubles). The
`WinUIDispatcher` is also here since it implements `Dispatcher`.

`backend/winui/` contains the sole `Backend` implementation: `WinUIBackend`.
The private `Handle` enum maps `ControlKind` variants to live XAML control
instances. Helper submodules handle type conversions (`convert.rs`), diagnostic
warnings (`diag.rs`), and generated property/event dispatch tables.

### `reconciler.rs` — tree diffing

`Reconciler<B: Backend>` diffs old vs. new element trees and issues backend
mutations. Five private submodules handle distinct phases: child matching,
diff helpers, templated-list reconciliation, widget dispatch, and wrapper logic.

## Generated code

| File | Generator | Purpose | Size |
|------|-----------|---------|------|
| `bindings.rs` | `tool_bindings` | Raw WinUI XAML COM/WinRT bindings | ~900 KB |
| `generated.rs` | `tool_reactor` | Widget struct → `PropBindings` mapping | ~28 KB |
| `backend/winui/generated_set_prop.rs` | `tool_reactor` | Prop dispatch to XAML setters | ~21 KB |
| `backend/winui/generated_attach_event.rs` | `tool_reactor` | Event dispatch to XAML event handlers | ~11 KB |

Regenerate with:
```sh
cargo run -p tool_reactor --quiet    # regenerates generated.rs + backend generated files
cargo run -p tool_bindings --quiet   # regenerates bindings.rs
```

## Dependency graph

```text
samples / tests
      │
      ▼
  ┌─────────────────────────────────────────┐
  │              windows-reactor             │
  │                                          │
  │  app / bootstrap / diagnostics           │
  │              │                           │
  │              ▼                           │
  │  engine.rs  ← element.rs                │
  │  style.rs   ← widget.rs                │
  │  interaction.rs                          │
  │  reconciler → backend/                  │
  │               ├── Backend trait          │
  │               └── winui/ (WinUIBackend) │
  │  widgets/ (57 controls)                  │
  │  hooks.rs, host.rs                       │
  └──────────────┬───────────────────────────┘
                 │
     ┌───────────┴───────────┐
     ▼                       ▼
 windows-core           windows-time
```

## Usage by samples and tests

Most consumers use `use windows_reactor::*` — the flat re-export design means
a single glob import gives access to every widget constructor, hook, layout
type, and trait.

| Crate | Style | Notes |
|-------|-------|-------|
| `gallery` | `use windows_reactor::*` | Full control gallery; exercises nearly every widget. |
| `examples` | `use windows_reactor::*` | Multi-example crate (notepad, solitaire, etc.). |
| `reactor_minimal` | Selective imports | Bare-minimum window. |
| `reactor_direct2d` | Selective imports | D3D/D2D interop via `SwapChainPanel`. |
| `test_reactor` | `use windows_reactor::*` | Unit tests for core types and reconciler logic. |
| `test_reactor_selftest` | Selective per-fixture | Integration tests with a live WinUI window. |
| `test_reactor_perf` | `use windows_reactor::*` | Stock-grid performance benchmark. |
| Canvas samples | `animated_canvas`, hooks | D2D rendering within reactor layout. |

## Widget catalogue

The `widgets/` directory contains 57 widget modules, one per WinUI control.
Each exports a struct and a snake_case factory function:

| Factory function | WinUI control | Category |
|-----------------|---------------|----------|
| `auto_suggest_box` | AutoSuggestBox | Input |
| `border` | Border | Layout |
| `breadcrumb_bar` | BreadcrumbBar | Navigation |
| `button` | Button | Input |
| `calendar_date_picker` | CalendarDatePicker | Date/time |
| `calendar_view` | CalendarView | Date/time |
| `canvas` | Canvas | Layout |
| `check_box` | CheckBox | Input |
| `color_picker` | ColorPicker | Input |
| `combo_box` | ComboBox | Input |
| `command_bar` | CommandBar | Navigation |
| `content_dialog` | ContentDialog | Overlay |
| `date_picker` | DatePicker | Date/time |
| `drop_down_button` | DropDownButton | Input |
| `expander` | Expander | Layout |
| `flyout` | Flyout | Overlay |
| `grid` | Grid | Layout |
| `hyperlink_button` | HyperlinkButton | Input |
| `icon` | FontIcon / SymbolIcon | Display |
| `image` | Image | Display |
| `info_badge` | InfoBadge | Display |
| `info_bar` | InfoBar | Display |
| `list_box` | ListBox | Collection |
| `menu_bar` | MenuBar | Navigation |
| `navigation_view` | NavigationView | Navigation |
| `number_box` | NumberBox | Input |
| `password_box` | PasswordBox | Input |
| `person_picture` | PersonPicture | Display |
| `pivot` | Pivot | Navigation |
| `progress_bar` | ProgressBar | Display |
| `progress_ring` | ProgressRing | Display |
| `radio_button` | RadioButton | Input |
| `radio_buttons` | RadioButtons | Input |
| `rating_control` | RatingControl | Input |
| `relative_panel` | RelativePanel | Layout |
| `repeat_button` | RepeatButton | Input |
| `rich_edit_box` | RichEditBox | Input |
| `scroll_view` | ScrollView | Layout |
| `scroll_viewer` | ScrollViewer | Layout |
| `selector_bar` | SelectorBar | Navigation |
| `shape` | Shape | Display |
| `slider` | Slider | Input |
| `split_button` | SplitButton | Input |
| `split_view` | SplitView | Layout |
| `stack_panel` / `vstack` / `hstack` | StackPanel | Layout |
| `surface_image_source` | SurfaceImageSource | Interop |
| `swap_chain_panel` | SwapChainPanel | Interop |
| `tab_view` | TabView | Navigation |
| `teaching_tip` | TeachingTip | Overlay |
| `text_block` | TextBlock | Display |
| `text_box` | TextBox | Input |
| `time_picker` | TimePicker | Date/time |
| `title_bar` | TitleBar | Chrome |
| `toggle_button` | ToggleButton | Input |
| `toggle_switch` | ToggleSwitch | Input |
| `tree_view` | TreeView | Collection |
| `viewbox` | Viewbox | Layout |
