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

## Application state and notification icons

Use `App::run_application::<A>(input)` for application state that must survive closing individual
windows. Implement `Application` with `create`, `update`, and `view`, using typed messages and
callbacks as with `Component`. `ApplicationContext` supplies commands and a local message sender;
`ApplicationViewContext` supplies callbacks while describing resources.

`ApplicationView` is a nonvisual declaration. It does not create an invisible component window.

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
Show requests resolve after the current application view commits; an undeclared key is an error.
Requests received during closure wait for the retiring instance before reopening.
Window inputs join the component's ordered message drain after pending native work. Repeated
unprocessed inputs coalesce to the latest value. Application, window, and component drivers defer
nested dispatcher wakes until the active driver returns, including during synchronous native dialogs.

Construct a notification declaration with `NotifyIcon::new(path)`, then attach `tooltip`,
`on_activate`, and `menu`. The icon path names an `.ico` file. Activation callbacks receive a
`ScreenPoint`; attaching a `Menu` handles both mouse and keyboard context-menu positioning without
application-side coordinate conversion. Use captureless `context.callback` mappings when possible
so callback identity remains stable across renders.

A menu uses the latest committed declaration. Changing or removing that menu dismisses its popup;
callbacks from retired registrations and dismissed menu generations are discarded. Repeated
requests for the same open menu do not create additional popups.

The application keeps running while any user window or declared notification icon remains. It
exits after both are gone and pending lifecycle work and messages have settled. Registered but
closed window slots, internal callback windows, and menu hosts do not keep it alive. Temporary
Shell unavailability during Explorer recovery does not remove an icon declaration.

The [notification icon sample](../../crates/samples/reactor/notifyicon) demonstrates independent
window and icon lifetimes. Its application state drives the window's toggle button through typed
inputs and callbacks, without retaining a native window or notification handle.
The [basic sample](../../crates/samples/reactor/notifyicon-basic) contains only an icon and Exit menu.

### Notification implementation and migration

Notification registration is private to Reactor in `src/native/notifyicon.rs`. It retains the
loaded icon, reposts Shell callbacks before invoking application work, uses
`NOTIFYICON_VERSION_4`, and restores registration on `TaskbarCreated`. Creation, update, and failed
Shell recovery use Reactor's application error path and are returned by `App::run_application`.
Native icon and tooltip replacement retain the preceding resource on failure.

Bindings are generated by `tool-bindings` from `notifyicon.txt` and `notifyicon_test.txt`. Backend
unit and live Shell tests reside alongside the private implementation. Public application
lifecycle coverage resides in `test-reactor-integration`'s `application_resources` binary.
The `application_dispatch` fixture exercises nested application/component dispatch and native
closure during a component update. `application_menus` uses UI Automation scoped to its own process
to select actual menu items and check popup replacement, owner switching, removal, and reopening.
It requires an interactive foreground desktop and sends no global mouse or keyboard input.

Applications migrating from `windows-notifyicon` use `windows-reactor` and an application
declaration instead of constructing a registration with `.build()`. Replace imperative icon and
tooltip setters with updated declarations, attach a Reactor menu directly, and omit the icon's
key to remove it. Hidden callback HWNDs and raw message-loop management are implementation details.
This requires the same Windows App Runtime as other Reactor applications.

`App::run`, `App::run_component`, and `App::run_with` retain their existing lifecycle behavior.

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
Notifications contain the restored outer bounds even when maximized, are coalesced, and exclude
minimized state. An unchanged subscription does not replay unchanged placement on each render;
a replacement or new subscription receives the current non-minimized placement. Each window
allows one placement subscriber. Removing its declaration or closing the component/window ends
the subscription.

Applications own persistence: store each observation in the message handler, rather than wait for
a final shutdown notification. This is observed native state, not a continuously controlled
position. Keeping a saved value in `initial_placement` does not cause snapback after a user move.
Reactor converts Win32 workspace coordinates internally; application values use screen pixels,
not DIPs.

Run `cargo run -p reactor-window-placement` to move, resize, or maximize a window and open a copy
at its observed placement. The sample retains placement in memory, without a persistence backend.

For restoration across a full process restart, run
`cargo run -p reactor-window-placement-registry`. Move, resize, or maximize the window, close it,
then run the command again. This sample uses `windows-registry` to save each placement observation
in the `Placement` string value under `HKCU\Software\windows-rs\samples\window-placement`. It loads
that value before opening the window. The five fields are `x y width height maximized`, with
maximized encoded as `0` or `1`; one registry value keeps the fields together.

Persistence is best-effort: missing or invalid placement and registry read errors use the default
window size. Write failures do not interrupt the application. Fallback and save failures are
reported in the console, without adding error state to the component. Delete the `Placement` value
to reset the sample.
Minimized state is not saved.

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

## Configure navigation

`NavigationView` exposes separate pane content and visibility settings:

```rust,ignore
NavigationView::new()
    .pane_header("Workspace")
    .pane_footer("Account")
    .is_back_enabled(false)
    .is_pane_visible(true)
    .content("Page")
```

`pane_header` owns one visual, like `pane_footer`; strings become text views. Updating it follows
the usual positional reconciliation rules, and omitting it on a later render removes the header.
It is separate from `header`, which belongs above the main content.
`is_back_enabled` controls the back button's enabled state, not its visibility or navigation
behavior. `is_pane_visible` controls pane visibility, not whether a visible pane is open.
Omitting either Boolean property clears its local value and restores WinUI's default or styled
value. Neither property changes application selection or maintains navigation history.

Run the [interactive sample](../../crates/samples/reactor/navigation-view-properties):

```text
cargo run -p reactor-navigation-view-properties
```

Cycle the pane header between text, a bordered replacement, and no header. Toggle the back arrow
between disabled and enabled, then hide and restore the pane. The controls stay in the main content
so the pane can always be restored. The back arrow has no navigation action in this sample.

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

The default API exposes application components, declarations, references, and window integration.
Host/runtime protocols such as `ComponentHost`, `Runtime`, `Adapter`, retained graphs, mutations,
service injection, schema metadata, and generic property/event records are available only with
the `test` feature. Applications should use `App::run_component`, component messages, and `provide`
rather than drive a retained host directly.
Typed control callbacks and reference integration methods do not expose the runtime records.
`ElementRef` exposes typed asynchronous operations rather than native object identity. The `test`
feature retains `ElementRef::get()` and `ObjectId` for graph assertions and benchmarks.
The reference compatibility traits are sealed because only generated controls and `ElementRef`
implementations participate in that contract. The public API snapshot includes doc-hidden items so
internal marker traits and methods cannot change without review.
`AnyElement` remains a doc-hidden default marker for erased references, including component roots.
Swap-chain metrics carry an opaque binding token so companion crates can reject stale completions
without depending on Reactor's internal binding counter.

Callback and view conversion traits are sealed. They accept the closure, callback, tuple, array,
and vector forms supplied by Reactor without creating downstream implementation protocols.
This is a closed conversion policy, not a requirement for runtime correctness. Custom collection
wrappers must produce `Vec<View>` or another supported input instead of implementing `IntoViews`.
Task and timer handles expose cancellation and delivery rejection in normal builds; full queue-state
inspection is diagnostic and remains available with the `test` feature.
`is_rejected()` polls for terminal queue rejection, not completion: `false` also includes pending
and cancelled work. It does not notify the application or retry a rejected message.

`src/lib.rs` explicitly exports handwritten application types. Generated controls and their value
enums are exported from `declaration::generated_declarations`; the metadata in `generated` and
validation errors in `ir` stay internal by default. These protocols still run in production.
Restricting their exports does not make their implementations test-only. Keep application exports
separate from internal wildcard imports so adding a public helper to an implementation module
does not automatically add an application API.

| Location | Purpose | Compilation |
| --- | --- | --- |
| `src/tests` | Unit tests and their private fixtures | `cfg(test)` |
| `src/test_support` | Recording adapters, diagnostics, and live-test helpers | `cfg(test)` or the `test` feature, as gated by the owner |
| Other `src` modules | Production implementation | Normal library builds |

`test_support` mirrors the owning modules: `component`, `reconcile`, `reference`, and `native`.
Headless host operations and services also live in this directory. The recording adapter remains
a root module. Support files are private child modules loaded with
`#[path]`, so diagnostic implementations can inspect their owner's state without widening
production visibility. Existing diagnostic exports remain available through the `test` feature.
Live application helpers in `test_support/native/app.rs` require that feature.

Keep only representation and trait hooks beside the production code: conditional fields,
counter updates, and conditional erased-component inspection methods. Diagnostic queries and
test-only implementations belong in `test_support`. Unit-only mutation capture and reference
scan instrumentation remain gated by `cfg(test)`, not the `test` feature, so external benchmarks
do not acquire their allocation or layout costs.

Handwritten native WinUI code compiles without a module-wide dead-code allowance. Diagnostic
window helpers and event simulation live in `test_support`; generated bindings retain their
separate allowance for unused metadata projections.

Full property-contract enumeration and queued key/character payloads are diagnostic-only.
Production validates individual properties and invokes routed input callbacks synchronously.

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

Run the unit tests with `cargo test -p windows-reactor --all-features --lib`. Privileged WinUI
coverage lives in `test-reactor-selftest`; performance runners keep the diagnostic `test` feature.
Application lifecycle, menu, Canvas, WebView, and window-state fixtures live in
[`test-reactor-integration`](../../crates/tests/libs/reactor_integration/readme.md), which does not
enable that feature. Build it in a separate Cargo invocation from privileged packages to avoid
feature unification. Unit test sources are included in the published package.

The Workbench sample uses `App::run_component` and owns its theme through `provide`. Its normal
build does not enable the `test` feature; only its headless regression tests do.

The coverage gate in `crates/tests/libs/reactor_selftest/coverage.ps1` checks production and
headless support files separately, not the unit-test bodies. Use modules rather than `include!`
for handwritten support so coverage is attributed to the support files.
