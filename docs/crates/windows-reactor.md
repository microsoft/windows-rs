# windows-reactor

> A declarative WinUI 3 library built around components, typed messages, and native controls.

- 📦 [crates.io](https://crates.io/crates/windows-reactor)
- 📖 [docs.rs](https://docs.rs/windows-reactor)
- 🚀 [Getting started](../../crates/libs/reactor/readme.md)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)
- [Self-contained deployment](windows-reactor-setup.md)
- [Canvas integration](windows-canvas.md)
- [Composition integration](windows-composition.md)

## When to use it

Use Reactor when an application needs native WinUI 3 controls and you would rather describe what
the UI should look like than manually keep a control tree in sync with application state. It is a
good fit for forms, navigation, data-driven views, and apps that combine WinUI controls with
Canvas, Composition, or WebView2.

Use [`windows-window`](windows-window.md) instead when you need an HWND and message loop without
WinUI. Use [`windows-composition`](windows-composition.md) directly for a retained visual tree
without XAML controls.

## The basic idea

A Reactor app is built from components. A component owns some state and has a `view` method that
describes the controls currently on screen. Controls send typed messages back to the component,
the component updates its state, and Reactor refreshes the parts of the native WinUI tree that
changed.

```text
event -> message -> update state -> build a new view -> update WinUI
```

This is the main pattern to learn. Component fields are the normal place for state, and imperative
control references are not needed for everyday UI code.

For a static window, getting text on screen can be this small:

```rust,no_run
use windows_reactor::*;

fn main() {
    App::run("Hello, Windows!".into()).unwrap();
}
```

Strings convert to a `View` containing a `TextBlock`. Create a `TextBlock` yourself only when its
properties need to be changed.

## Your first component

After adding the dependency shown in the
[crate README](../../crates/libs/reactor/readme.md), the following is a complete counter app:

```rust,no_run
use windows_reactor::*;

#[derive(Clone)]
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
        context.window_title("Counter");

        StackPanel::new().spacing(8.0).children((
            format!("Count: {}", self.count),
            Button::new()
                .on_click(context.message(Message::Increment))
                .content("Increment"),
            Button::new()
                .on_click(context.message(Message::Reset))
                .content("Reset"),
        ))
    }
}

fn main() {
    App::run_component::<Counter>(()).unwrap();
}
```

The four component methods have straightforward jobs:

| Method | Purpose |
| --- | --- |
| `create` | Make the initial component state |
| `update` | Handle a message and change that state |
| `view` | Describe the UI for the current state |
| `input_changed` | Optionally react when a parent changes this component's input |

`Input` is `()` because the root component has no parent-owned data. `Message` is an enum because
the component can receive two kinds of event. The button callbacks enqueue those messages; they do
not mutate the component directly.

`App::run_component` creates the WinUI application and first window, mounts `Counter`, and runs the
UI loop.

## Build views from controls

WinUI controls use typed builders. Start with `Control::new()`, set properties, connect events, and
add content or children:

```rust,ignore
Border::new()
    .padding(16.0)
    .content(
        StackPanel::new().spacing(12.0).children((
            TextBlock::new()
                .text("Account")
                .font_size(24.0),
            TextBox::new().placeholder_text("Name"),
            Button::new().content("Save"),
        )),
    )
```

Use `content` for a control with one child, such as a button or border. Use `children` for a
container with an ordered set of children. Tuples are convenient because the children may have
different control types. Put `content` or `children` last in a builder chain because it finishes
the control and returns a `View`.

Values that implement `Into<View>` can be used directly with these methods. In particular, use a
`&str` or `String` for ordinary text and reach for `TextBlock` only to set font, layout,
accessibility, or other control properties:

```rust,ignore
StackPanel::new().children((
    "A short label",
    format!("Welcome, {}!", self.name),
    TextBlock::new().text("Styled text").font_size(24.0),
))
```

Layout is just more builder methods. Containers describe their rows, columns, direction, or
spacing, while children say where they belong:

```rust,ignore
Grid::new()
    .rows([GridLength::Auto, GridLength::Star(1.0)])
    .columns([GridLength::Auto, GridLength::Star(1.0)])
    .children((
        TextBlock::new().text("Name").grid_row(0).grid_column(0),
        TextBox::new().grid_row(0).grid_column(1),
        TextBlock::new()
            .text("Details go here")
            .grid_row(1)
            .grid_column_span(2),
    ))
```

The builders expose native WinUI concepts with Rust types, so invalid property values and callback
payloads are usually caught by the compiler.

## Keep state in the component

For an editable control, pass the current value as a property and route changes back through a
message. This is often called a controlled control:

```rust,ignore
struct Profile {
    name: String,
}

impl Component for Profile {
    type Input = ();
    type Message = String;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            name: String::new(),
        }
    }

    fn update(&mut self, name: String, _context: &ComponentContext<Self>) {
        self.name = name;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(8.0).children((
            TextBox::new()
                .text(self.name.clone())
                .placeholder_text("Type your name")
                .on_text_changed(context.forward()),
            format!("Hello, {}!", self.name),
        ))
    }
}
```

Use `context.message(value)` when an event should always send the same message. Use
`context.callback(function)` when the event carries a value that should become part of the
message enum. When the event payload already is the component's message type, as in the example
above, `context.forward()` is the shortest form.

Keep event-driven state changes in `update`, and keep `view` focused on turning the current state
into controls. That makes the direction of data flow easy to follow.

## Split the UI into understandable pieces

For small, stateless pieces, use an ordinary function that returns `View`:

```rust,ignore
fn section_heading(text: &str) -> View {
    TextBlock::new()
        .text(text)
        .font_size(20.0)
        .font_weight(FontWeight::BOLD)
        .into()
}
```

When a piece needs its own state, messages, or lifecycle, make it a component. Parents pass data
through the component's `Input`:

```rust,ignore
#[derive(Clone, PartialEq)]
struct GreetingInput {
    name: String,
}

struct Greeting;

impl Component for Greeting {
    type Input = GreetingInput;
    type Message = ();

    fn create(_input: &GreetingInput, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(
        &self,
        input: &GreetingInput,
        _context: &mut ViewContext<Self>,
    ) -> View {
        format!("Hello, {}!", input.name).into()
    }
}
```

The parent places it in its view like any other child:

```rust,ignore
View::component::<Greeting>(GreetingInput {
    name: self.name.clone(),
})
```

`Input` must implement `Clone + PartialEq`. Reactor compares it with the previous input and updates
the child when it changes. Start with explicit inputs; shared context is more useful when data must
reach many distant descendants.

## Use normal Rust for conditionals

There is no special syntax for conditional UI. Build a `View` with an ordinary `if` or `match`:

```rust,ignore
let status: View = if self.loading {
    ProgressRing::new().is_active(true).into()
} else {
    "Ready".into()
};

StackPanel::new().children((
    status,
    Button::new().content("Refresh"),
))
```

Use `View::empty()` when one branch should render nothing. `View::fragment` groups several sibling
views without adding a native container.

## Render changing lists with stable keys

Use `children` for a fixed group of controls. For a changing collection, map each item to a
`KeyedView` and use `keyed_children`:

```rust,ignore
let rows = self
    .tasks
    .iter()
    .map(|task| (task.id, task.title.clone()));

StackPanel::new().spacing(4.0).keyed_children(rows)
```

Each `(key, view)` tuple converts to a `KeyedView`, and the title string converts to its text view.
The key should identify the logical item, such as a record ID. Do not use the current list index
when items can move. Stable keys let Reactor keep the right child component state attached as items
are inserted, removed, or reordered.

For very large collections, `ItemsRepeater` and `VirtualSource` add virtualization. Start with
`keyed_children`; move to virtualization only when the list is large enough to need it.

## Move slow work off the UI thread

Rendering, component updates, and native controls live on the UI thread. Use
`spawn_background` for blocking or CPU-intensive work, then return a message with the result:

```rust,ignore
enum Message {
    Load,
    Loaded(String),
}

fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
    match message {
        Message::Load => {
            self.loading = true;

            _ = context.spawn_background(|_| {
                let value = load_data();
                Message::Loaded(value)
            });
        }
        Message::Loaded(value) => {
            self.value = value;
            self.loading = false;
        }
    }
}
```

The background closure must only capture `Send` data and must not touch controls or component
state. Put expected failures in the returned message, usually as `Result<T, E>`, and display the
result after `update` stores it.

## Reach for the other APIs when you need them

The component/message/view loop covers most application code. These APIs solve more specific
problems:

| API | Use it for |
| --- | --- |
| `use_effect` | Starting and cleaning up an external subscription |
| `Context<T>` | Sharing app-wide data such as a theme with distant descendants |
| `ElementRef<T>` | Focus or another operation that cannot be expressed as state |
| `open_window` | Opening an independent secondary window |
| `ItemsRepeater` | Virtualizing a large collection |

Prefer component input over context for normal parent-to-child data, and prefer properties and
messages over `ElementRef`. The declarative path is usually shorter and easier to maintain.

## Deployment

Reactor detects a staged Windows App Runtime or resolves an installed framework package at
startup. For self-contained deployment, use `windows-reactor-setup` from `build.rs`. Compare the
[`framework_dependent`](../../crates/samples/reactor/framework_dependent) and
[`self_contained`](../../crates/samples/reactor/self_contained) samples when choosing how to
package the app.

## What to read next

The focused examples in
[`samples/examples`](../../crates/samples/reactor/samples/examples) are the easiest way to learn
one concept at a time:

| Example | What it shows |
| --- | --- |
| `counter` | Components, state, messages, and events |
| `function_component` | Child components and input |
| `text_box` | Controlled input |
| `stack` and `grid` | Layout |
| `auto_suggest_box` | Ordinary Rust conditionals and collections |
| `keyed_list_reorder` | Stable identity in changing lists |
| `async_state` | Background work |
| `use_effect` and `context` | Lifecycle work and shared data |

The [`gallery`](../../crates/samples/reactor/gallery) is a control catalog. The
[`navigation`](../../crates/samples/reactor/navigation) and
[`apps`](../../crates/samples/reactor/apps) samples show how these same ideas fit together in a
larger application. See the [`composition`](../../crates/samples/reactor/composition),
[`webview`](../../crates/samples/reactor/webview), and
[Canvas](../../crates/samples/canvas) samples only when the app needs those integrations.

---

## Internal documentation

The remainder of this page describes how the crate is built and maintained. Applications do not
need it to use Reactor.

### Architecture

| Layer | Location |
| --- | --- |
| Public frontend | `src/core/public.rs`, `src/element.rs`, `src/generated.rs` |
| Component lifecycle and effects | `src/core/component.rs` |
| Reconciler | `src/core/pump` |
| Scheduling | `src/core/engine.rs`, `src/core/scheduler.rs` |
| Native runtime | `src/native/winui` |
| Recording runtime | `src/test/recording.rs` |
| Typed integrations | `src/reference.rs` |

Components produce the public `View` representation. The Pump plans tree and lifecycle changes,
then publishes commands to a runtime. The WinUI runtime applies those commands to native objects.
`RecordingRuntime` consumes the same command stream for deterministic tests.

In a debug build, set `WINDOWS_REACTOR_TRACE=1` to print a reconciliation summary before each
nonempty component update is applied. The trace reports native property, topology, subscription,
creation, and destruction command counts.

One structural tree handles controls, components, fragments, slots, dialogs, overlays, and
windows. Keyed views retain ownership across moves. Generational window, component, and node
identities reject work that targets retired objects.

Generated controls store shared payloads behind `Rc`; cloning a view shares the payload.
Runtime-generated identity maps use `rustc_hash`. Collections keyed by application data keep
randomized hashing.

Unexpected native command failures are fatal because a partially applied batch would leave the
native and retained trees inconsistent. Native subtree destruction detaches the subtree's external
edge without clearing internal native collections that are destroyed in the same batch. This lets
WinUI finish deferred visual-state work safely.

`ToolTip` is an internal attachment type. The public surface is `TooltipExt` and `Tooltip`.

### Native integration boundary

Reactor-owned XAML objects never cross the public API. Typed `ElementRef` commands and observations
are represented in both the WinUI and recording runtimes. Observations follow structural
replacement and reject late callbacks by window and node identity. Accepted one-shot requests
complete exactly once; `IntegrationError::Native` retains the HRESULT and `Unavailable` reports a
retired or unavailable target.

Canvas owns its devices, swap chains, image sources, resize handling, and recovery. Composition
owns application visual trees and animations. WebView users receive the CoreWebView2 object rather
than Reactor's XAML control.

### Code generation

`crates/tools/reactor` refreshes pinned WinUI, Windows App SDK, and WebView2 metadata, resolves
`crates/tools/reactor/src/winui.toml`, and generates:

| Output | Contents |
| --- | --- |
| `crates/libs/reactor/src/generated.rs` | Public control builders and retained data |
| `crates/libs/reactor/src/native/winui/generated.rs` | Native command application |
| `crates/libs/reactor/src/native/winui/bindings.rs` | Minimal WinUI bindings |
| `crates/libs/canvas/src/reactor_bindings.rs` | Minimal Canvas bridge bindings |
| `crates/tests/libs/reactor_surface/src/generated_surface.rs` | Live projected API cases |

`bindings.txt` is the hand-maintained runtime filter. `control_bindings.txt` is generated from
`winui.toml`. Content properties come from WinUI's `ContentPropertyAttribute`, including inherited
properties. Feedback values come from event payloads instead of generated property readers.

Generated Rust files are committed and must not be edited by hand. After changing the schema,
metadata inputs, filters, or generator, run:

```text
cargo run -p tool_reactor --quiet
cargo run -p tool_reactor --quiet
cargo check -p windows-reactor --quiet
```

The second generator run must leave the tree unchanged.

Bindings used only by the `test` feature are allowed to be dead in a normal build. Enabling the
feature removes that allowance so the live surface build checks all generated test callables.

### Testing

| Layer | Command or location |
| --- | --- |
| Internal deterministic tests | `cargo test -p windows-reactor` |
| External API tests | `cargo test -p test_reactor` |
| Generator tests | `cargo test -p tool_reactor` |
| Live handwritten fixtures | `cargo run -p test_reactor_selftest -- --headless` |
| Generated WinUI surface | `cargo run -p test_reactor_surface -- --headless` |
| Planner benchmarks | `cargo run -p test_reactor_bench --release` |
| Live grid benchmark | `cargo run -p test_reactor_bench --bin reactor-live-grid --release` |

The generated surface test covers projected controls, properties, events, content, collections,
slots, attachments, virtual items, and TreeView nodes. Handwritten self-tests own imperative
references, retirement, and other OS interactions.

`crates/libs/reactor/public-api.txt` is the checked public API snapshot. Regenerate it with the
repository's pinned `cargo-public-api` process after an intentional API change.
