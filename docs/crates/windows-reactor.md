# windows-reactor

> A declarative WinUI 3 library built around components, typed messages, and native controls.

- 📦 [crates.io](https://crates.io/crates/windows-reactor)
- 📖 [docs.rs](https://docs.rs/windows-reactor)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)

Windows Reactor lets Rust state drive native WinUI 3 controls. Start with a counter, then add
editable state, child components, and background work. Later sections cover common application
tasks and link to complete samples.

Reactor is currently a preview, so releases may still refine public APIs.

## Create a project

Create a binary crate and add Reactor:

```text
cargo new counter
cd counter
cargo add windows-reactor@0.100
```

Use the default features. An installed Windows App SDK runtime is required for the default
deployment model; see the [setup guide](windows-reactor-setup.md) for prerequisites.

For a GUI executable that should not open a console window, add this to the top of `main.rs`:

```rust,ignore
#![windows_subsystem = "windows"]
```

Leave it out while diagnosing startup problems so console output remains visible.

## Add a component

Replace `main.rs` with:

```rust,no_run
use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Increment,
    Reset,
}

struct Counter {
    count: i32,
}

impl Component for Counter {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Increment => self.count += 1,
            Message::Reset => self.count = 0,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let content = StackPanel::new().spacing(8.0).children((
            format!("Count: {}", self.count),
            Button::new()
                .on_click(context.message(Message::Increment))
                .content("Increment"),
            Button::new()
                .on_click(context.message(Message::Reset))
                .content("Reset"),
        ));

        context.window_frame("Counter", content)
    }
}

fn main() {
    App::run_component::<Counter>(()).unwrap();
}
```

Run it:

```text
cargo run
```

The component loop is:

```text
native event -> typed message -> update state -> describe the next view
```

| Method | Purpose |
| --- | --- |
| `create` | Build the initial state |
| `update` | Handle a message and change the state |
| `view` | Describe the controls for the current state |

`view` uses typed builders. `StackPanel::children` accepts a tuple of different control types, and
strings convert directly to text views. The button callbacks enqueue messages for `update`; they
do not mutate state inside the native event callback.

`context.window_frame` publishes the window title and returns the standard title-bar layout. For a
static view with no state or events, call `App::run(view)` instead.

## Add editable state

Declare a control's current value from component state and send changes back as messages:

```rust,ignore
TextBox::new()
    .text(self.name.clone())
    .on_text_changed(context.callback(Message::SetName))
```

Handle `Message::SetName(name)` in `update` by assigning `self.name = name`. Only declared
properties are controlled. If `update` rejects a change, the next render restores the declared
value. Collections work the same way: a user reorder reports the item keys in their new order,
and the state must be sorted to match. The [`controlled`](../../crates/samples/reactor/controlled)
sample shows the pattern for text and numeric input.

## Split up the view

Use a function returning `View` for a stateless piece. Use a child component when a subtree needs
its own state:

```rust,ignore
View::component::<Greeting>(GreetingInput {
    name: self.name.clone(),
})
```

Component input must implement `Clone + PartialEq`. Give items that can move stable keys, not list
indices, so state and focus stay with the right item:

```rust,ignore
StackPanel::new().keyed_children(self.tasks.iter().map(|task| (task.id, task.title.clone())))
```

## Do work without blocking the window

Rendering and updates run on the UI thread. Use `set_timeout` for delayed messages and
`spawn_background` for blocking work; background work returns a message rather than touching
state directly. See the [`use-effect`](../../crates/samples/reactor/use-effect) and
[`async-state`](../../crates/samples/reactor/async-state) samples.

## Windows and notification icons

Use `App::run_application::<A>(input)` when state must outlive individual windows. The
application's view declares keyed windows and notification icons, and `context.show_window`
opens a declared window. The application keeps running while any window or icon remains.

Use `context.window_visuals` for size, placement, theme, and backdrop, and observers such as
`on_window_placement` to save placement.

| Sample | Shows |
| --- | --- |
| [`notifyicon-basic`](../../crates/samples/reactor/notifyicon-basic) | A notification icon with an Exit menu |
| [`notifyicon`](../../crates/samples/reactor/notifyicon) | Windows and icons with independent lifetimes |
| [`window-placement-registry`](../../crates/samples/reactor/window-placement-registry) | Save placement across restarts |
| [`window-observers`](../../crates/samples/reactor/window-observers) | Size, placement, and color-scheme changes |
| [`window-theme`](../../crates/samples/reactor/window-theme) | Themes and backdrops |

## Images, icons, and animation

| Sample | Shows |
| --- | --- |
| [`image-sources`](../../crates/samples/reactor/image-sources) | URI, file, bitmap, and SVG sources for images and icons |
| [`translation-transition`](../../crates/samples/reactor/translation-transition) | `Border` translation transition and reversal |

## Choose a deployment model

A framework-dependent application uses an installed Windows App SDK and needs only
`windows-reactor`. For a self-contained application, add `windows-reactor-setup` as a build
dependency and call it from `build.rs`:

```rust,ignore
fn main() {
    windows_reactor_setup::as_self_contained();
}
```

The [setup guide](windows-reactor-setup.md) covers prerequisites, WebView2, and packaging.

## Explore larger samples

| Sample | Shows |
| --- | --- |
| [`calculator`](../../crates/samples/reactor/calculator) | State, messages, and Grid layout |
| [`solitaire`](../../crates/samples/reactor/solitaire) | A larger application with keyed state |
| [`stacker`](../../crates/samples/reactor/stacker) | Reactor, Canvas, and Composition together |
| [`gallery`](../../crates/samples/reactor/gallery) | Available controls and navigation |

Browse the [sample directory](../../crates/samples/reactor) for more.

## Contributing

Controls are generated from `crates/tools/reactor/src/schema.toml` and the committed WinUI metadata.
Edit the schema or generator, not the generated files:

```text
cargo run -p tool-reactor --quiet
cargo test -p windows-reactor --all-features --lib
```

Changes to the public API also require updating `public-api.txt`.

Reactor fails fast: invalid declarations, internal bugs, and native failures after startup panic.
`Result` is returned only where the caller can act: startup, errors from app closures (through
`App::run`), and imperative completions such as focus requests.
