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

To drive an editable control from component state, declare its current value and send changes
back as messages.

```rust,ignore
TextBox::new().text(self.name.clone())
    .placeholder_text("Type your name")
    .on_text_changed(context.callback(Message::SetName))
```

Handle `Message::SetName(name)` in `update` by assigning `self.name = name`. Reactor suppresses
feedback from its own writes while still delivering user edits.

Only declared properties are controlled; subscribing to changes does not take ownership. Omitting
a previously declared property clears its native assignment once. An explicit `None` is a declared
value, not an omitted property. Store values in component state when they must survive control
replacement.

The [`controlled`](../../crates/samples/reactor/controlled) sample shows the complete pattern.

## Split up the view

Use a function returning `View` for a stateless piece. Use a child component when a subtree needs
its own state or lifecycle:

```rust,ignore
View::component::<Greeting>(GreetingInput {
    name: self.name.clone(),
})
```

Component input must implement `Clone + PartialEq`.

Children at fixed positions keep identity by position and type. Items that can move need stable
application keys:

```rust,ignore
StackPanel::new().keyed_children(
    self.tasks
        .iter()
        .map(|task| (task.id, task.title.clone())),
)
```

Do not use list indices as keys when items can move. Stable keys keep state and focus attached to
the correct item.

## Do work without blocking the window

Rendering and component updates run on the UI thread. Use `set_timeout` for delayed messages and
`spawn_background` for blocking or CPU-intensive work. Background work returns a message rather
than touching controls or component state directly.

Use an effect when an external subscription or resource should exist only while part of the view
is present. See the [`use-effect`](../../crates/samples/reactor/use-effect) and
[`async-state`](../../crates/samples/reactor/async-state) samples for complete lifecycle patterns.

## Application state and notification icons

Use `App::run_application::<A>(input)` for application state that must survive closing individual
windows. Implement `Application` with `create`, `update`, and `view`, using typed messages and
callbacks as with `Component`. An `ApplicationView` declares windows and notification icons
without requiring a hidden application window.

| Declaration or command | Behavior |
| --- | --- |
| `view.window::<C>("main", input)` | Registers a reopenable window without opening it. |
| `context.show_window("main")` | Opens, restores, or activates that window; repeated requests coalesce. |
| Updated window input | Reaches the existing component without recreating its state. |
| User closes a window | Drops its component state; the declaration remains available to reopen. |
| Omit a window key | Closes its instance and removes its declaration. |
| `view.notify_icon("tray", icon)` | Creates or updates the keyed Shell registration. |
| Omit an icon key | Removes its registration and dismisses its menu. |
| `context.exit()` | Ends the application and releases all owned resources. |

Keys must be unique within each resource kind. A live window key must retain its component type.
Only keys declared in the application's view can be opened.

Construct a notification declaration with `NotifyIcon::new(path)`, then attach `tooltip`,
`on_activate`, and `menu`. The icon path names an `.ico` file. Activation callbacks receive a
`ScreenPoint`; attaching a `Menu` handles both mouse and keyboard context-menu positioning without
application-side coordinate conversion.

Windows and notification icons have independent lifetimes. The application keeps running while
either remains; registered but closed window slots do not keep it alive.

The [notification icon sample](../../crates/samples/reactor/notifyicon) demonstrates independent
window and icon lifetimes with typed inputs and callbacks.
The [basic sample](../../crates/samples/reactor/notifyicon-basic) contains only a notification icon
and an Exit menu.

Notification menus follow the Windows taskbar theme. For a menu independent of a notification icon
or control, `AppContext::show_menu_at` accepts a `Menu` and physical screen coordinates.

## Position and restore windows

Publish startup placement through `context.window_visuals`. The first committed declaration is
applied before the native window becomes visible, for both primary and secondary windows.

| Setting | Meaning |
| --- | --- |
| `initial_position(ScreenPoint)` | Initial outer-window top-left in physical screen pixels |
| `initial_placement(WindowPlacement)` | Restored outer bounds in physical screen pixels, plus maximized state |
| `client_size(width, height)` | Client-area size in DIPs; later explicit changes still resize |
| `constraints(WindowConstraints)` | Client-size limits in DIPs, including when restoring placement |

Full placement takes precedence over initial position and initial client size. Later changes to
either initial setting do not move or resize an open window. Load saved placement before opening
the window; an asynchronous update after startup is too late. Negative coordinates are valid on a
multi-monitor desktop. Windows adjusts fully off-screen bounds to an available monitor.

```rust,ignore
context.window_visuals(WindowVisuals::new().initial_placement(saved_placement));
context.on_window_placement(context.callback(Message::Placement));
```

`on_window_placement` reports the initial placement and native changes while subscribed.
Notifications contain the restored outer bounds even when maximized and exclude minimized state.
Applications own persistence: save observations in the message handler rather than waiting for a
final shutdown notification. Keeping a saved value in `initial_placement` does not move the window
back after a user move.

`on_color_scheme` also reports the current value when subscribed; `on_window_size` reports changes
only. Replacing an observer callback does not replay the current value. Observers end when omitted
from the view or when their component closes.

Use `WindowVisuals::theme` to choose System, Light, or Dark. With no backdrop, transparent content
shows the window's themed background. Use `WindowVisuals::backdrop` to select Mica, MicaAlt, or
Acrylic; an opaque application background covers the material.

| Sample | Shows |
| --- | --- |
| [`window-placement`](../../crates/samples/reactor/window-placement) | Open a window at an observed placement |
| [`window-placement-registry`](../../crates/samples/reactor/window-placement-registry) | Save placement across process restarts |
| [`window-observers`](../../crates/samples/reactor/window-observers) | Subscribe to size, placement, and color-scheme changes |
| [`window-theme`](../../crates/samples/reactor/window-theme) | Themes, backdrops, and application backgrounds |

## Display images

`Image` and `ImageIcon` support these source builders:

| Builder | Source |
| --- | --- |
| `source(uri)` | A URI; returns a `Result` |
| `source_file(path)` | A file path; returns a `Result` |
| `source_data(bytes)` | Encoded bitmap bytes |
| `source_svg(bytes)` | SVG bytes |

Use `EncodedImage::from_static` for embedded assets or `EncodedImage::new` for owned bytes.
URI and file sources select SVG by the `.svg` extension. In-memory sources choose their decoder
through the builder:

```rust,no_run
# use windows_reactor::*;
let svg = EncodedImage::from_static(
    br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24">
    <circle cx="12" cy="12" r="10" fill="red"/></svg>"#,
);
StackPanel::new().children((
    Image::new().source_svg(svg.clone()),
    AppBarButton::new().icon(Icon::image_svg(svg)),
));
```

Keep owned bytes in component state and clone the value when describing the view. Equal sources
share native images within a runtime. `Image::on_opened` and `on_failed` report loading outcomes
for each assignment, including when a shared source has already loaded.

The [`image-sources`](../../crates/samples/reactor/image-sources) sample compares the source
builders across images and icon slots. [`image-reuse`](../../crates/samples/reactor/image-reuse)
shows page replacement with shared sources.

## Animate a visual

`Border` pairs `opacity`, `scale`, and `translation(x, y)` with matching
`opacity_transition`, `scale_transition`, and `translation_transition` durations. Translation uses
DIPs and changes visual position without changing layout space.

New or replaced visuals start at their declared values without animating from native defaults.
Later transition settings apply before target changes in the same render, regardless of builder order.
Removing a target restores its default using the current transition. Removing a transition affects
future target changes; an animation already running can finish. Native transitions respect the
Windows animation setting. See the
[translation sample](../../crates/samples/reactor/translation-transition) for sliding and reversal.

## Add icon content

Use `Icon` to supply a control's icon content:

```rust,no_run
# use windows_reactor::*;
StackPanel::new().children((
    TitleBar::new()
        .title("Search")
        .icon(Symbol::Home)
        .left_header(TextBlock::new().text("Files")),
    AutoSuggestBox::new().query_icon(Icon::font("\u{E721}")),
));
```

Use `Icon::symbol`, `Icon::font`, `Icon::bitmap`, `Icon::image_*`, or `Icon::path` for control icon
slots. Use `SymbolIcon`, `FontIcon`, `BitmapIcon`, `ImageIcon`, and `PathIcon` when the icon is a
standalone visual that needs layout or other visual properties.

## Choose a deployment model

A framework-dependent application uses an installed Windows App SDK framework package and needs
only `windows-reactor`.

For a self-contained application, add:

```toml
[build-dependencies]
windows-reactor-setup = "0.100"
```

Then create `build.rs`:

```rust,ignore
fn main() {
    windows_reactor_setup::as_self_contained();
}
```

The [setup guide](windows-reactor-setup.md) covers prerequisites, WebView2, and packaging.

## Explore larger samples

| Sample | What it demonstrates |
| --- | --- |
| [`calculator`](../../crates/samples/reactor/calculator) | State, messages, and Grid layout |
| [`solitaire`](../../crates/samples/reactor/solitaire) | A larger application with keyed state |
| [`stacker`](../../crates/samples/reactor/stacker) | Reactor, Canvas, and Composition together |
| [`gallery`](../../crates/samples/reactor/gallery) | Available controls and navigation |

Browse the [sample directory](../../crates/samples/reactor) for smaller examples of controlled
input, context, effects, multiple windows, virtualization, WebView2, and deployment.
The [navigation sample](../../crates/samples/reactor/navigation-view-properties) demonstrates pane
content and visibility settings.

## Contributing

Controls are generated from `crates/tools/reactor/src/schema.toml` and the committed WinUI metadata.
Edit the schema or generator, not the generated files. `tool-reactor-metadata` refreshes the
metadata; `tool-reactor` generates declarations, native adapters, and bindings.
The native binding filter is `crates/tools/reactor/src/bindings.txt`.

Use metadata names unless a shared Reactor name matches the public value, role, and behavior.
Container names distinguish one owned visual (`content`), ordered visuals (`children`), and item
declarations (`items`).

After a generator or schema change, regenerate and inspect the output:

```text
cargo run -p tool-reactor --quiet
cargo check -p windows-reactor --quiet
cargo test -p windows-reactor --all-features --lib
```

Changes to the public API also require updating `public-api.txt`.
