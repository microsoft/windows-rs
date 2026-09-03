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

Use Reactor when an application needs native WinUI 3 controls and benefits from describing the UI
from application state. It is a good fit for forms, navigation, data-driven views, multiple
windows, and apps that combine WinUI controls with Canvas, Composition, or WebView2.

Use [`windows-window`](windows-window.md) instead when you need an HWND and message loop without
WinUI. Use [`windows-composition`](windows-composition.md) directly for a retained visual tree
without XAML controls.

## Getting started: choose a deployment

Start with the README's counter and choose how the Windows App Runtime reaches the target machine.

| Deployment | Choose it when | Setup |
| --- | --- | --- |
| Framework-dependent | The framework package is installed | No build helper |
| Self-contained | The app carries the runtime | `windows-reactor-setup` in `build.rs` |

Reactor detects the staged runtime or resolves the installed framework package at startup. Compare
the
[`framework_dependent`](../../crates/samples/reactor/framework_dependent) and
[`self_contained`](../../crates/samples/reactor/self_contained) samples before packaging an app.

Run a root component with `App::run_component::<C>(input)`. The root view becomes the content of
the first window. Set that window's title and visual options from its component's `view` method.

## Core model

### Component, Input, and Message

A `Component` has three responsibilities:

| Part | Owner | Purpose |
| --- | --- | --- |
| Component fields | The component | Local state and long-lived handles |
| `Input` | The parent | Declarative data passed into the component |
| `Message` | The component | Typed events and completed work that may update state |

The lifecycle is:

1. `create` constructs component state from the initial input.
2. `input_changed` runs when retained input compares unequal to new input.
3. `update` handles one queued message and mutates component state.
4. `view` describes the UI for the current state and input.

After `create`, `input_changed`, or `update`, Reactor calls `view` as needed and publishes the
result. Keep `view` declarative: read state, declare context dependencies and effects, and build a
`View`. Put state changes in `update`.

`Input` must implement `Clone + PartialEq`. Its equality defines whether a child receives
`input_changed`, so include every value that changes the child's behavior. Prefer small values or
shared handles such as `Rc<T>` when copying a large model would be expensive.

Use a function returning `View` for stateless presentation:

```rust,ignore
fn status_card(title: &str, value: String) -> View {
    StackPanel::new().spacing(4.0).children((
        TextBlock::new().text(title),
        TextBlock::new().text(value),
    ))
}
```

Use `View::component::<C>(input)` when the subtree needs local state, lifecycle work, messages, or
an independent recomposition boundary.

### Building views

Controls use typed builders. Start with `Control::new()`, chain property and event methods, and
place content with the control's structural method:

- `content(value)` for one content child.
- `children(values)` for an ordered child collection.
- `keyed_children(values)` when identity must follow keys across reordering.
- Named slot methods for controls with several content positions.
- `items(values)` or `virtual_source(source)` for projected item controls.

Strings and generated controls convert into `View`. Tuples and arrays work well for fixed child
sets. Use iterators for data-driven children. `View::fragment` groups several sibling views without
adding a native control.

Most layout is expressed on the child: alignment, margin, Grid row or column, and attached
properties are builder methods on that child. Container builders configure rows, columns,
orientation, spacing, or padding.

Omitting a property preserves the native default. For optional properties, passing `None` sets an
explicit empty value; it is not always the same as omitting the builder. Generated value and slot
enums are non-exhaustive, so external matches need a wildcard arm.

### Events and messages

Event builders take typed `Callback` values from `ViewContext`:

| Method | Use |
| --- | --- |
| `context.message(message)` | An event with no payload always sends one cloneable message |
| `context.callback(map)` | Convert an event payload into a message |
| `context.forward()` | Event payload and component message have the same type |
| `context.sender()` | Send later from an owned closure or another component |

Callbacks enqueue messages; they do not call `update` inline. Define a message enum around user
intent, such as `Save`, `Select(u64)`, or `NameChanged(String)`, and handle it in one `update`
match. This keeps native event details out of application state transitions.

Controlled controls report feedback through event payloads. Store the new value in component state
and feed it back through the corresponding property builder. Selection callbacks use
`Option<usize>` and nullable date, time, rating, and number values use `Option<T>`, so application
code does not need to interpret native sentinel values.

## Common workflows

### State, context, effects, and background work

Store render-driving state in component fields and mutate it in `update`. Interior mutable values
such as `Cell` or `RefCell` are useful for resources shared with callbacks, but mutating them does
not schedule a render. Send a component message when the view must change.

Use `Context<T>` for data needed by distant descendants:

1. Define a stable `Context::new(default)`.
2. Wrap a subtree with `View::provide(&context, value, child)`.
3. Read it with `ViewContext::use_context`.

The consumer is recomposed when its resolved context value changes. Prefer explicit component
input for ordinary parent-child data; context is best for app-wide values such as a theme.

`ViewContext::use_effect(key, dependency, setup)` runs setup after publication when the dependency
changes. Setup may return a cleanup closure, which runs before replacement or when the component
retires. Keys must be unique within one component publication. Effects are appropriate for typed
observations and external resources, not for deriving view state.

Use `ComponentContext::spawn_background` for blocking or CPU work. It runs on the Windows thread
pool and returns a `ComponentTask`. The closure returns a component message, which is delivered
back to `update`.

- Put expected failures in the message, usually as `Result<T, E>`.
- Check the supplied `CancellationToken` during cooperative work.
- Keep a `ComponentTask` only when explicit cancellation or status is needed; dropping it does not
  cancel the task.
- Scope retirement and Pump shutdown cancel owned work.
- Use `spawn_background_with_rejection` when bounded delivery rejection needs its own message.

Do not access Reactor UI state from the worker. Move `Send` data into it and return the result as a
message.

### Windows

Declare window properties from `view`:

- `window_title` sets the owning window title.
- `window_visuals` configures client size, constraints, backdrop, and related visual options.
- `on_window_size` and `on_color_scheme` route window changes to typed messages.

Declare each of these at most once per component publication. A descendant may make the
declaration for its owning window, but centralizing it in the window's root component is easier to
follow.

Open another independent window with `ComponentContext::open_window(root)` during `create`,
`input_changed`, or `update`. Use `context.window().request_close()` in those same lifecycle
methods to close the owning window after publication. Each opened window owns an independent Pump.
The application exits when the last window closes.

Use a `TitleBar` view for app-drawn title-bar content and events. It is separate from the native
window title set by `window_title`.

### Lists and stable identity

For a small changing list, produce `KeyedView` values and pass them to `keyed_children` or
`View::keyed_fragment`. Keys preserve component state when rows move. A key must identify the
logical item, not its current index.

For a large list, use `ItemsRepeater::virtual_source(VirtualSource::new(...))`. The source supplies:

1. A revision for the key sequence.
2. The item count.
3. A key function.
4. A view factory called when WinUI realizes an index.

Increment the revision whenever length, key values, or key order changes. Keep it stable for
payload-only updates. Reactor validates keys on initial mount and revision changes, but constructs
views only for realized rows.

Use normal item controls when their native selection or grouping behavior is the goal. Use
`ItemsRepeater` when realization cost and row ownership matter.

### Images

Use `Image::source`, `source_file`, or their icon equivalents for URI and file sources. Use
`source_data(EncodedImage)` for PNG or other bitmap bytes supported by WinUI:

- `EncodedImage::from_static` retains a static byte slice without copying.
- `EncodedImage::new` owns shared runtime data.
- SVG uses the URI or file path rather than `EncodedImage`.

Decoding is asynchronous. `Image::on_opened` and `on_failed` report URI loads and encoded-image
completion. Replacing the source, removing the node, or resetting the runtime cancels pending work.

Use a typed `ElementRef<Image>` only when another crate owns the native image source. For example,
`windows-canvas` uses it to attach a `CanvasImageSource` and observe rasterization scale.

### Typed ElementRef integrations

Create an `ElementRef<T>` as a component field and attach it with the control's
`element_ref(&reference)` builder. The type prevents attaching a reference to the wrong control.
It is unbound before publication and after removal.

Supported capabilities include:

| Reference | Capability |
| --- | --- |
| Focus-capable controls | `request_focus` and `request_focus_result` |
| `ElementRef<SwapChainPanel>` | Swap-chain attachment and surface observation |
| `ElementRef<Image>` | Native image-source attachment and rasterization-scale observation |
| `ElementRef<Grid>` | Lifted Composition host observation and child-visual attachment |
| `ElementRef<WebView2>` | Request the application-facing CoreWebView2 object |

A one-shot method returning `false` means the reference is currently unbound. An accepted request
finishes through its completion callback with a value or `IntegrationError`. An observation
returns `ElementObservation`; retain it for as long as events are needed and drop it to stop them.

Reactor keeps its XAML objects private. Integrations exchange only typed capabilities and
application-owned native payloads. Prefer the higher-level Canvas and Composition adapters over
managing these requests directly.

### Debugging

Set `WINDOWS_REACTOR_TRACE=1` in a debug build to print one reconciliation summary before each
nonempty component update is applied. The trace names composed component types and reports native
property, topology, subscription, creation, and destruction command counts. Because it is written
before WinUI calls, the last line can identify the update involved in a native failure. Release
builds omit this output.

When a view does not update, check these in order:

1. The event callback sends the intended message.
2. `update` changes component state used by `view`.
3. Child `Input::eq` observes every value relevant to the child.
4. List keys are stable and a virtual source revision changes when its key sequence changes.
5. An `ElementRef` request is made while bound and its observation handle remains alive.

## Pitfalls

- Do not mutate native XAML objects outside Reactor's command path.
- Do not perform blocking work in `view`, callbacks, or `update`; use background work.
- Do not mutate component state from `view`.
- Do not use list indices as keys when items can move.
- Do not recreate a `Context` on every render; consumers identify a context by its instance.
- Do not drop an `ElementObservation` while its callback is still required.
- Treat unexpected native command failures as fatal. Reactor cannot safely continue with native
  and retained trees out of sync.

## Sample progression

Read and run the samples in this order:

1. [`counter`](../../crates/samples/reactor/counter) - the basic component and message loop.
2. [`form`](../../crates/samples/reactor/form) - several controls and typed form state.
3. [`controlled`](../../crates/samples/reactor/controlled) - native feedback into controlled
   values.
4. [`samples/examples`](../../crates/samples/reactor/samples/examples) - focused examples. Start
   with `component_input`, `composition`, `async_state`, `use_effect`, `element_ref`, `window`,
   `secondary_window`, `keyed_list_reorder`, and `image`.
5. [`navigation`](../../crates/samples/reactor/navigation) - a multi-view application structure.
6. [`virtual`](../../crates/samples/reactor/virtual) - keyed virtual rows, editing, background
   loading, and typed focus.
7. [`gallery`](../../crates/samples/reactor/gallery) - broad control and layout coverage.
8. [`composition`](../../crates/samples/reactor/composition),
   [`webview`](../../crates/samples/reactor/webview), and the
   [Canvas samples](../../crates/samples/canvas) - native integrations.
9. [`apps`](../../crates/samples/reactor/apps) - larger application examples.

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
