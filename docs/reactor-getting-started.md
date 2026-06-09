# Windows Reactor — Getting Started

Windows Reactor is a declarative UI library for Rust, backed by WinUI 3. It uses a React-like component model with hooks for state management and a builder-pattern DSL for composing UI elements.

## Minimal App

```rust
use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    text_block("Hello, world!").font_size(24.0).bold().into()
}

fn main() -> Result<()> {
    App::new().title("My App").render(app)
}
```

Every app follows this pattern: define a render function `fn(&mut RenderCx) -> Element` and pass it to `App::new().render(app)`. Widget builders convert to `Element` via `.into()`.

## State with `use_state`

```rust
fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);
    let bump = move || set_count.call(count + 1);

    vstack((
        text_block(format!("count = {count}")).font_size(18.0).bold(),
        button("Click").on_click(bump),
    ))
    .into()
}
```

`use_state` returns the current value and a `SetState` handle. Calling `set_count.call(new_value)` triggers a rerender.

## Layout

Use `vstack(children)` and `hstack(children)` for vertical/horizontal stacking, and `grid(children)` for two-dimensional layout:

```rust
fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        hstack((text_block("A"), text_block("B"))).spacing(8.0),
        grid((
            text_block("0,0").grid_row(0).grid_column(0),
            text_block("0,1").grid_row(0).grid_column(1),
        ))
        .columns([GridLength::Star(1.0), GridLength::Star(1.0)]),
    ))
    .spacing(12.0)
    .into()
}
```

## Components

Extract reusable UI into function components:

```rust
#[derive(Clone, PartialEq)]
struct GreetingProps {
    name: String,
}

fn greeting(props: &GreetingProps, _cx: &mut RenderCx) -> Element {
    text_block(format!("Hello, {}!", props.name)).bold().into()
}

fn app(_cx: &mut RenderCx) -> Element {
    component(greeting, GreetingProps { name: "world".into() })
}
```

Components receive typed props and their own `RenderCx` with independent hook state.

## Hooks

| Hook | Purpose |
|------|---------|
| `cx.use_state(initial)` | Reactive state; returns `(T, SetState<T>)` — triggers rerender on change |
| `cx.use_async_state(initial)` | Like `use_state` but returns `AsyncSetState<T>` that is `Send + Sync` |
| `cx.use_reducer(initial)` | Returns `(T, Updater<T>)` for functional updates via `updater.call(|prev| next)` |
| `cx.use_reducer_fn(reducer, initial)` | Redux-style state with `Dispatch<A>` for dispatching typed actions |
| `cx.use_ref(initial)` | Mutable `HookRef<T>` that never triggers a rerender |
| `cx.use_effect(deps, closure)` | Side effect that runs when deps change |
| `cx.use_effect_with_cleanup(deps, f)` | Effect returning an optional cleanup closure |
| `cx.use_memo(deps, closure)` | Memoized computation, re-runs only when deps change |
| `cx.use_callback(deps, closure)` | Stable `Callback` identity across renders |
| `cx.use_resource(fetcher, deps)` | Background data fetch; returns `Resource<T>` |
| `cx.use_mutation::<T>()` | Async write operations; returns `(MutationState<T>, MutationTrigger<T>)` |
| `cx.use_context(&CTX)` | Read a value provided by an ancestor via `.provide()` |
| `cx.use_color_scheme()` | Subscribe to system Light/Dark theme changes |
| `cx.use_inner_size()` | Track window inner dimensions (re-renders on resize) |
| `cx.use_dpi()` | Track per-monitor DPI (re-renders on DPI change) |
| `cx.use_ui_marshaller()` | Get a `UiMarshaller` for custom cross-thread dispatch |

## Common Controls

Factory functions create control builders. Chain methods to configure:

```rust
// Buttons
button("Click me").on_click(handler).accent()
button("With icon").icon(SymbolGlyph::Save).on_click(handler)

// Text input
text_box(value).header("Name").placeholder("Type here…").on_changed(set_value)
text_box(value).multiline().on_changed(set_value)

// Selection
check_box(checked).label("Accept").on_changed(set_checked)
ComboBox::new(["A", "B", "C"]).on_selection_changed(set_index)
Slider::new(value).range(0.0, 100.0).on_changed(set_value)
ToggleSwitch::new(on).header("WiFi").on_changed(set_on)

// Progress
ProgressBar::new(65.0).range(0.0, 100.0)
ProgressRing::indeterminate()

// Lists
list_view(items, |item, _idx| text_block(item.clone()))
    .on_selection_changed(set_selected)
```

## Navigation

Use `NavigationView` with enum-based routing:

```rust
#[derive(Clone, PartialEq)]
enum Page { Home, Settings }

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state(Page::Home);

    let body: Element = match &page {
        Page::Home => text_block("Home").into(),
        Page::Settings => text_block("Settings").into(),
    };

    NavigationView::new(
        [
            NavViewItem::new("Home").tag("home").icon(SymbolGlyph::Home),
            NavViewItem::new("Settings").tag("settings").icon(SymbolGlyph::Setting),
        ],
        body,
    )
    .selected_tag(match page { Page::Home => "home", Page::Settings => "settings" })
    .on_selection_changed(move |tag: String| {
        set_page.call(match tag.as_str() {
            "settings" => Page::Settings,
            _ => Page::Home,
        })
    })
    .into()
}
```

## Async Data

`use_resource` runs a fetcher on a background thread, passing `deps` as the argument. It refetches when deps change and discards stale results automatically:

```rust
fn fetch_items(page: i32) -> std::result::Result<Vec<String>, String> {
    // runs off the UI thread; deps value is passed as the argument
    Ok(vec![format!("Item on page {page}")])
}

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state(0_i32);
    let items = cx.use_resource(fetch_items, page);

    let content: Element = match &items {
        Resource::Loading => ProgressRing::indeterminate().into(),
        Resource::Ready(data) => vstack(
            data.iter().map(|s| text_block(s).into()).collect::<Vec<Element>>()
        ).into(),
        Resource::Error(e) => text_block(format!("Error: {e}")).into(),
        _ => text_block("...").into(),
    };

    vstack((content, button("Next").on_click(move || set_page.call(page + 1)))).into()
}
```

The fetcher signature is `Fn(D) -> Result<T, String>` where `D` is the deps type.

## Context (Dependency Injection)

Share values across the tree without prop-drilling:

```rust
use std::sync::LazyLock;

static THEME: LazyLock<Context<String>> = LazyLock::new(|| Context::new("light".into()));

fn child(_: &(), cx: &mut RenderCx) -> Element {
    let theme = cx.use_context(&THEME);
    text_block(format!("Theme: {theme}")).into()
}

fn app(cx: &mut RenderCx) -> Element {
    let (theme, set_theme) = cx.use_state("dark".to_string());

    vstack((
        button("Toggle").on_click(move || set_theme.call("light".into())),
        component(child, ()),
    ))
    .provide(&THEME, theme)
    .into()
}
```

## Window Configuration

```rust
fn main() -> Result<()> {
    App::new()
        .title("My App")
        .backdrop(Backdrop::Mica)
        .inner_size(800.0, 600.0)
        .inner_constraints(InnerConstraints {
            min_width: Some(400.0),
            min_height: Some(300.0),
            ..Default::default()
        })
        .render(app)
}
```

## Animations

Implicit transitions tween property changes smoothly:

```rust
border(text_block("Hello"))
    .opacity(if visible { 1.0 } else { 0.0 })
    .with_opacity_transition(Duration::from_millis(500))
```

Explicit animations via `.animate()`:

```rust
border(text_block("Hello"))
    .with_scale_transition(Duration::from_millis(500))
    .animate(AnimationConfig {
        scale: Some(if big { 1.5 } else { 1.0 }),
        duration: Duration::from_millis(500),
        easing: Easing::EaseOut,
        ..Default::default()
    })
```

## Error Boundaries

Catch panics in subtrees and render fallback UI:

```rust
error_boundary(
    component(risky_child, props),
    |msg| text_block(format!("Error: {msg}")).into(),
)
```

## Custom Rendering (Direct3D / SwapChainPanel)

Use `swap_chain_panel()` to host a Direct3D/Direct2D surface inside a reactor UI. The `on_mounted` callback provides a `SwapChainPanelHandle` for attaching your DXGI swap chain, and `on_resize` fires when the panel's layout size changes:

```rust
fn app(cx: &mut RenderCx) -> Element {
    swap_chain_panel()
        .on_mounted(|panel| {
            // Create D3D device + swap chain, then:
            // panel.set_swap_chain(&swap_chain).unwrap();
        })
        .on_resize(|width, height| {
            // Resize swap chain buffers to match the panel's actual size.
        })
        .into()
}
```

Use `on_rendering(|| { ... })` to drive per-frame presentation. See the full examples:

```sh
cargo run -p minimal --example swap_chain_panel   # D3D11 animated clear
cargo run -p minimal --example direct2d           # D2D circles + WinUI buttons
```

## Running Samples

```sh
# Minimal examples (crates/samples/reactor/minimal/)
cargo run -p minimal --example button
cargo run -p minimal --example counter
cargo run -p minimal --example navigation

# App examples (crates/samples/reactor/apps/)
cargo run -p examples --example notepad
cargo run -p examples --example tictactoe
cargo run -p examples --example calculator

# Gallery (comprehensive control showcase)
cargo run -p gallery
```
