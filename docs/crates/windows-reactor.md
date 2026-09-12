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
- [Tray icon integration](windows-trayicon.md)

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

## Application lifetime

`App::run`, `App::run_windows`, and `App::run_component` exit after the last Reactor window closes.
This is convenient when windows define the complete application lifetime.

Use `App::run_with` when tray icons, services, or other process resources are peers of the Reactor
windows:

```rust,ignore
App::run_with(|app| {
    let exit = app.proxy();
    let tray = TrayIcon::new("app.ico")
        .on_event(move |event| {
            if matches!(event, TrayIconEvent::ContextMenu { .. }) {
                _ = exit.exit();
            }
        })
        .build()?;

    Ok(tray)
})
```

The startup closure runs on the UI thread. It may call `AppContext::open_window` immediately or
later from work posted through `AppProxy::dispatch`. Its return value remains owned by the
application until explicit exit, so the example keeps the tray icon alive even while no Reactor
window exists.

`AppContext` is UI-thread-bound and may be cloned into callbacks on that thread. `AppProxy` is
`Send + Sync` and may be cloned into worker threads. Closing the last Reactor window in this mode
does not exit the process; call `AppContext::exit` on the UI thread or `AppProxy::exit` from
another thread.

The [`reactor-trayicon`](../../crates/samples/reactor/trayicon) sample demonstrates this lifetime
model. Use `WindowRef::request_activate` when an external resource needs to restore and foreground
an existing Reactor window.

## Window title bars

`window_frame` gives a Reactor window an integrated WinUI title bar. It creates the required
two-row layout, uses the same text for the native and visible window titles, and places the
component's content below the title bar:

```rust,ignore
context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Acrylic));
context.window_frame("Canvas keyboard input", content)
```

Call `window_frame` from the component that supplies the window's native root so the frame fills
the window. The title-bar container is not a tab stop.

`window_title` and `window_visuals` remain available for windows that use the system title bar.
They declare native window state without adding title-bar content to the view. Material backdrops
do not extend through the system title bar, so use `window_frame` when Mica or Acrylic should cover
the complete window.

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
different control types. For these ordinary content and container controls, put `content` or
`children` last in a builder chain because it finishes the control and returns a `View`.

Controls with several named content areas expose one builder method for each area. The method
signature distinguishes a single view from a keyed collection:

```rust,ignore
NavigationView::new()
    .menu_items([
        ("home", NavigationViewItem::new().content("Home")),
        ("files", NavigationViewItem::new().content("Files")),
    ])
    .footer_menu_items([(
        "settings",
        NavigationViewItem::new().content("Settings"),
    )])
    .content("Page content")
    .into()
```

These named methods remain part of the control builder, so properties and other named areas can be
chained after them. The compiler rejects passing one view to a collection area or a collection to
a single-view area. Assigning the same named area more than once replaces its earlier value.

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

## Handle keyboard input on custom surfaces

`Border` can act as a focusable interaction surface around Canvas, Composition, or another custom
view. Enable tab focus and native post-event pointer focus, then use routed callbacks for input
events whose WinUI `Handled` value must be set before the native callback returns:

```rust,ignore
Border::new()
    .is_tab_stop(true)
    .focus_on_pointer_release(true)
    .on_preview_key_down(context.routed_callback(|info: KeyEventInfo| {
        match info.key {
            VirtualKey::LEFT => RoutedMessage::handled(Message::MoveLeft),
            VirtualKey::RIGHT => RoutedMessage::handled(Message::MoveRight),
            _ => RoutedMessage::bubble_without_message(),
        }
    }))
    .on_character_received(context.routed_callback(|info: CharacterEventInfo| {
        RoutedMessage::handled(Message::Character(info.character))
    }))
    .on_got_focus(context.callback(Message::Focused))
    .on_lost_focus(context.callback(Message::Blurred))
    .content(custom_surface)
```

The routed callback inspects an owned payload and decides whether the native event is handled.
Its optional component message still enters the normal queue, so `Component::update` and
reconciliation never run inside the WinUI callback. Return `bubble_without_message` for keys such
as Tab that should retain their normal XAML behavior. Input bubbles when the component queue is
already full. Once handled, its deferred message retains its native FIFO position and waits for
component queue capacity.

`KeyEventInfo` includes the mapped and original virtual key, physical-key status, and modifier
state. `CharacterEventInfo::character` is one UTF-16 code unit so surrogate pairs are preserved
without lossy conversion. `focus_on_pointer_release(true)` queues a native
`FocusState::Pointer` request from the routed pointer event. The request runs after the native
callback returns. It works when a child such as Canvas is the direct hit-test source.
`ElementRef<Border>` supports other programmatic focus cases.

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

## Schedule UI work

Use `set_timeout` to send a component message after a delay:

```rust,ignore
self.timer = Some(
    context
        .set_timeout(Duration::from_millis(500), Message::Tick)
        .unwrap(),
);
```

The timer runs on the UI dispatcher, so its message does not need to implement `Send` and waiting
does not occupy a thread-pool worker. Store the returned `ComponentTimer`; dropping or cancelling
it prevents delivery.

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

## Run modal work with the owning window

Use `run_window` for a native operation that must run on the owning UI thread with the window's
HWND. For example, a Win32 message box can return its selection as a component message:

```rust,ignore
Message::Confirm => {
    let accepted = context.run_window(|window| {
        let answer = unsafe {
            MessageBoxW(
                window.as_raw(),
                w!("Continue with this operation?"),
                w!("Confirm"),
                (MB_YESNO | MB_ICONQUESTION) as u32,
            )
        };
        Message::Answered(answer)
    });
    if !accepted {
        self.status = "Another window operation is pending".to_string();
    }
}
Message::Answered(IDYES) => {
    self.status = "You chose Yes".to_string();
}
```

Reactor runs the closure in the next host dispatch after the current publication commits and
queues its returned message. It discards the work if publication fails, the requesting component
retires, or the window starts closing. A `true` return means that the work was staged, not that it
is guaranteed to run. The `WindowHandle` is scoped to the closure. Each Reactor window accepts one
pending operation at a time, so a second request returns `false` until the first operation runs or
is discarded.

Component dispatch for the owning window is suspended until the closure returns. A native modal
loop can continue drawing and processing its own input, and other Reactor windows remain
independent. Keep slow non-UI work in `spawn_background`; `run_window` is for native calls such as
modal dialogs that must remain on the UI thread.

The [`message-box`](../../crates/samples/reactor/message-box) sample contains the complete
component. `windows-pickers` builds on the same mechanism and provides `request` methods that map
picker results into component messages.

## Reach for the other APIs when you need them

The component/message/view loop covers most application code. These APIs solve more specific
problems:

| API | Use it for |
| --- | --- |
| `use_effect` | Starting and cleaning up an external subscription |
| `use_effect_guard` | Owning a subscription or revoker that cleans itself up when dropped |
| `Context<T>` | Sharing app-wide data such as a theme with distant descendants |
| `ElementRef<T>` | Focus or another operation that cannot be expressed as state |
| `open_window` | Opening an independent secondary window |
| `run_window` | Running a native modal operation with the owning HWND |
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

Use the [`gallery`](../../crates/samples/reactor/gallery) to explore individual WinUI controls. The
standalone samples under [`crates/samples/reactor`](../../crates/samples/reactor) show Reactor
concepts and behaviors:

| Example | What it shows |
| --- | --- |
| [`function-component`](../../crates/samples/reactor/function-component) | Child components and input |
| [`component-input`](../../crates/samples/reactor/component-input) | Controlled component input |
| [`keyed-list-reorder`](../../crates/samples/reactor/keyed-list-reorder) | Stable identity in changing lists |
| [`async-state`](../../crates/samples/reactor/async-state) | Background work |
| [`message-box`](../../crates/samples/reactor/message-box) | Modal native window work |
| [`use-effect`](../../crates/samples/reactor/use-effect) and [`context`](../../crates/samples/reactor/context) | Lifecycle work and shared data |
| [`pointer-tracking`](../../crates/samples/reactor/pointer-tracking) | Pointer capture and movement |
| [`exit-transition`](../../crates/samples/reactor/exit-transition) | Transition-driven removal |
| [`secondary-window`](../../crates/samples/reactor/secondary-window) | Multiple Reactor windows |
| [`calculator`](../../crates/samples/reactor/calculator) | A larger component with derived state |

The [`counter`](../../crates/samples/reactor/counter) demonstrates components, state, messages, and
events. The [`gallery`](../../crates/samples/reactor/gallery) is a control catalog. The
[`navigation`](../../crates/samples/reactor/navigation) and
[`dotsweeper`](../../crates/samples/reactor/dotsweeper) samples show how these same ideas fit
together in a larger application. See the [`composition`](../../crates/samples/composition),
[`webview`](../../crates/samples/webview/reactor), and
[Canvas](../../crates/samples/canvas) samples only when the app needs those integrations.
The [`canvas-keyboard`](../../crates/samples/canvas/keyboard) sample is a focusable custom-rendered
text surface with routed edit keys, UTF-16 character input, visible focus state, and a standard
WinUI button that demonstrates focus transfer. It shows when custom input is useful without
replacing `TextBox` for ordinary text editing.

---

## Internal documentation

The remainder of this page describes how the crate is built and maintained. Applications do not
need it to use Reactor.

### Architecture

| Layer | Location |
| --- | --- |
| Public frontend | `src/core/public.rs`, `src/element.rs`, `src/generated.rs` |
| Component lifecycle and effects | `src/core/component/mod.rs` |
| Reconciler | `src/core/pump` |
| Scheduling | `src/core/engine/mod.rs`, `src/core/scheduler.rs` |
| Native runtime | `src/native/winui` |
| Recording runtime | `src/test/recording/mod.rs` |
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

Swap-chain panel metrics include a binding generation so an integration can distinguish resize
from structural panel replacement. `request_surface_frame` schedules one normal-priority
UI-thread callback independently of `CompositionTarget::Rendering`; Canvas uses it to keep drawing
during interactive resize without invoking application drawing code from a layout callback.

Canvas owns its devices, swap chains, image sources, resize handling, and recovery. Composition
owns application visual trees and animations. WebView users receive the CoreWebView2 object rather
than Reactor's XAML control.

Routed keyboard callbacks are stored separately from ordinary event callbacks. WinUI key and
character arguments are copied into owned payloads, the callback decides `Handled` synchronously,
and any resulting component message is placed in the existing native-event FIFO. This preserves
native event order without running `Component::update` or reconciliation across a WinRT callback.
Callback replacements publish transactionally and do not replace the native subscription.

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
cargo run -p tool-reactor --quiet
cargo run -p tool-reactor --quiet
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
| Generator tests | `cargo test -p tool-reactor` |
| Live handwritten fixtures | `cargo run -p test-reactor-selftest -- --headless` |
| Generated WinUI surface | `cargo run -p test-reactor-surface -- --headless` |
| Planner benchmarks | `cargo run -p test-reactor-bench --release` |
| Live grid benchmark | `cargo run -p test-reactor-bench --bin reactor-live-grid --release` |
| Live input benchmark | `cargo run -p test-reactor-bench --bin reactor-live-input --release` |
| Live Notepad benchmark | `cargo run -p test-reactor-bench --bin reactor-live-notepad --release` |

The generated surface test covers projected controls, properties, events, content, collections,
slots, attachments, virtual items, and TreeView nodes. Handwritten self-tests own imperative
references, retirement, and other OS interactions.

The live Notepad benchmark uses the same controlled `TextBox` shape as the `reactor-notepad`
sample. It injects Unicode keyboard input and measures raw `WM_CHAR`, WinUI `TextChanged`,
`Text()` retrieval, Reactor event and component queues, reconciliation, Rust allocations, and
whether controlled feedback causes a native write-back. Use `--text-size` to test document-size
scaling. Run the release build with its window in the foreground and leave the machine idle while
collecting results. The test-only probes add instrumentation overhead, and the injected
`KEYEVENTF_UNICODE` path does not measure physical-key translation or IME composition.

Pass `--filter <name>` to run matching handwritten fixtures. For example:

```text
cargo run -p test-reactor-selftest -- --headless --filter Window_NestedOperationRearming
```

`crates/libs/reactor/public-api.txt` is the checked public API snapshot. Regenerate it with the
repository's pinned `cargo-public-api` process after an intentional API change.
