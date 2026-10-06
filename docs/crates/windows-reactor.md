# windows-reactor

> A declarative WinUI 3 library built around components, typed messages, and native controls.

- 📦 [crates.io](https://crates.io/crates/windows-reactor)
- 📖 [docs.rs](https://docs.rs/windows-reactor)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)

Windows Reactor lets Rust state drive native WinUI 3 controls. This guide builds a counter
and then points to focused samples for the rest of the API.

Reactor is currently a preview, so releases may still refine public APIs.

## Create a project

Create a binary crate and add Reactor:

```text
cargo new counter
cd counter
cargo add windows-reactor@0.100
```

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

## Add icon content

`Icon` describes icon content for control slots. Reactor realizes the same value as WinUI's visual
`IconElement` or nonvisual `IconSource`, depending on the destination property:

```rust,ignore
StackPanel::new().children((
    TitleBar::new()
        .title("Search")
        .icon(Symbol::Home)
        .left_header(TextBlock::new().text("Files")),
    AutoSuggestBox::new().query_icon(Icon::font("\u{E721}")),
))
```

Use `Icon::symbol`, `Icon::font`, `Icon::bitmap`, `Icon::image_*`, or `Icon::path` for control icon
slots. Use `SymbolIcon`, `FontIcon`, `BitmapIcon`, `ImageIcon`, and `PathIcon` when the icon is a
standalone visual that needs layout or other visual properties.

## Add editable state

Editable controls are controlled: pass the current value from component state and send changes
back as messages.

```rust,ignore
TextBox::new(self.name.clone())
    .placeholder_text("Type your name")
    .on_text_changed(context.callback(Message::SetName))
```

Handle `Message::SetName(name)` in `update` by assigning `self.name = name`. Reactor suppresses
feedback from its own writes while still delivering user edits.

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

## Maintain generated controls

`crates/tools/reactor/src/schema.toml` declares controls, properties, relations, and events.
Native classes, property types, event payloads, and collection types come from the WinUI metadata.
The schema specifies only Reactor behavior and public API exceptions. Controls default to the
`Visual` category, relations default to one positional owned visual, and adapters define
Reactor-specific value conversions. Unknown schema fields are rejected, and inferred metadata is
validated before generation. Control capabilities such as focus, references, and attachments are
declared on the control.

### Choose public names

Metadata names are the default. Normalize a name only when existing Reactor APIs expose the same
public value, role, and behavior under different native spellings. Keep distinct names when the
callback payload, lifecycle boundary, ownership, or collection shape differs.

Prefer the shortest established Reactor term for a shared concept. Do not mechanically expand
names to mirror property spelling, and do not retain the native spelling as an alias. Apply and
review one concept family at a time. For example, date and time pickers use `on_date_changed` and
`on_time_changed` because `SelectedDateChanged`, `DateChanged`, and `SelectedTimeChanged` all
report the current date or time value.

For container relations, `content` means one owned visual that fills the control. Native `Child`
and `Content` members therefore share the public `content` name. Keep `children` for ordered visual
collections and `items` for item or container declarations.

After changing the schema or generator, run:

```text
cargo run -p tool-reactor --quiet
```

The command updates the Reactor declarations, native adapter, native bindings, and live coverage.
`crates/tools/reactor/src/bindings.txt` is the handwritten native binding filter.

### Module imports

Handwritten modules use `use super::*;`, with shared imports supplied by `src/lib.rs` and
`src/native/mod.rs`. Keep native binding names separate where they overlap Reactor declarations;
the WinUI adapter uses the `native::` prefix for those bindings. `Weak` denotes an `Rc` weak
reference, while `SyncWeak` denotes an `Arc` weak reference.

### Test boundaries

| Location | Purpose | Compilation |
| --- | --- | --- |
| `src/tests` | Unit tests and their private fixtures | `cfg(test)` |
| `src/test_support` | Recording adapters, diagnostics, and live-test helpers | `cfg(test)` or the `test` feature, as gated by the owner |
| Other `src` modules | Production implementation | Normal library builds |

`test_support` mirrors the owning modules: `component`, `reconcile`, `reference`, and `native`.
The recording adapter remains a root module. Support files are private child modules loaded with
`#[path]`, so diagnostic implementations can inspect their owner's state without widening
production visibility. Existing diagnostic exports remain available through the `test` feature.
Live application helpers in `test_support/native/app.rs` require that feature.

Keep only representation and trait hooks beside the production code: conditional fields,
counter updates, and the conditional `tracked_tasks` trait method. Diagnostic queries and
test-only implementations belong in `test_support`. Unit-only mutation capture and reference
scan instrumentation remain gated by `cfg(test)`, not the `test` feature, so external benchmarks
do not acquire their allocation or layout costs.

Both directories are included in the published package. In particular, `test_support` cannot be
excluded while a published feature depends on it. The Cargo `test` feature is an ordinary
opt-in feature, not the compiler's unit-test configuration.

### Private unit tests

Unit test sources live under `crates/libs/reactor/src/tests`. The directory contains library test
modules, not separate Cargo integration-test targets.

| Files | Scope |
| --- | --- |
| `mod.rs` and its child modules | Shared fixtures and headless subsystem tests |
| `component/mod.rs` and its child modules | Component-host tests and shared private fixtures |
| `native/app.rs`, `native/transient_menu.rs`, `native/winui.rs` | Native implementation tests |

Keep tests that need private implementation details beneath the owning module, using `#[path]`
to place their files in this directory. Test modules inherit imports with `use super::*;`; do not
widen production visibility to make a test helper accessible.

The directory layout groups tests by ownership, but `#[path]` determines their module scope.
For example, `tests/native/app.rs` is `native::app::tests`, not `tests::native::app`. Component
tests are grouped by context, lifecycle, messaging, reconciliation, virtualization, and window
services. Keep a fixture with its consumers; only fixtures used by multiple groups belong in
`component/mod.rs`.

Filter tests by function name, for example
`cargo test -p windows-reactor --all-features context_change_renders_only_subscribers`, rather
than a full module path. This keeps commands independent of subsystem file organization.

Run the unit tests with `cargo test -p windows-reactor --all-features --lib`. Live WinUI and
cross-crate integration fixtures remain in `test-reactor-selftest`; performance runners remain in
their existing packages. The `test` feature still supplies their diagnostic APIs. Unit test
sources are included in the published package.

The coverage gate in `crates/tests/libs/reactor_selftest/coverage.ps1` checks production and
headless support files separately, not the unit-test bodies. Use modules rather than `include!`
for handwritten support so coverage is attributed to the support files.
