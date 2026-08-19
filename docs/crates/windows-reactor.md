# windows-reactor

`windows-reactor` is a declarative WinUI 3 library for Windows applications.

- [crates.io](https://crates.io/crates/windows-reactor)
- [docs.rs](https://docs.rs/windows-reactor)
- [Crate readme](../../crates/libs/reactor/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)

Applications render an `Element` tree from state. Reactor retains the mounted tree, compares each
render with the committed state, and sends validated native updates to WinUI. Components and hooks
remain Rust values; WinUI controls, event registrations, and native resources remain owned by the
mounted tree.

## Getting started

Add the runtime and setup crates:

```toml
[dependencies]
windows-reactor = "0.100"

[build-dependencies]
windows-reactor-setup = "0.100"
```

A framework-dependent application stages the bootstrap files in `build.rs`:

```rust,no_run
fn main() {
    windows_reactor_setup::as_framework_dependent();
}
```

Call `bootstrap()` before starting Reactor:

```rust,no_run
use windows_reactor::{
    Application, Element, RenderCx, Window, button, component, run_reactor_winui_app, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let count = cx.use_state(|| 0_i32);

    let windows = if open.value() {
        let increment = count.clone();
        let close = open.clone();
        vec![
            Window::new(
                "Counter",
                vstack(
                    12.0,
                    [
                        windows_reactor::text_block(format!("Count: {}", count.value())),
                        button("Increment", move || {
                            increment.update(|value| *value += 1);
                        }),
                    ],
                ),
                move || close.set(false),
            )
            .build(),
        ]
    } else {
        Vec::new()
    };

    Application::new(windows).build()
}

fn main() -> windows_core::Result<()> {
    windows_reactor::bootstrap()?;
    run_reactor_winui_app(component(app))
}
```

Use `windows_reactor_setup::as_self_contained()` when the application carries its own Windows App
Runtime. Self-contained applications do not call `bootstrap()`. Multi-example crates can use
`as_example()`.

## Application and component model

`Application` owns application resources and a collection of top-level `Window` elements. A
window owns its content and any declared owned windows. Removing the final window ends the
application run.

Use ordinary Rust functions to compose subtrees that need no independent lifecycle. Use
`component` when a subtree owns hooks, effects, context reads, or local invalidation.
`component_with_props` stores owned props and borrows them during rendering.

`memo_component` and `memo_component_with_props` skip a render while their dependencies or props
and inherited context remain equal. A local state change still renders the component, and the
latest closure or props are retained for that render.

`fragment` groups children without creating a native control. Context providers and components
are also logical nodes: they participate in ownership and reconciliation but do not create a
layout peer.

## Builders and framework capabilities

Control constructors return typed builders. Shared framework modifiers are available only on
compatible builders:

- layout: size, minimum and maximum size, margin, alignment, and visibility;
- input: pointer events, capture, keyboard accelerators, taps, and typed drag/drop;
- accessibility: automation name, id, help text, and heading level;
- visual state: opacity and typed opacity or scale transitions;
- styling: padding, background, foreground, fonts, character spacing, and control chrome;
- attached layout: Grid, Canvas, and RelativePanel child placement; and
- resources and typed theme brushes.

Apply modifiers before `.build()` or conversion to `Element`. An erased element or logical
component does not expose native-only modifiers, so unsupported combinations fail to compile
instead of becoming ignored runtime properties.

Lowercase helpers cover common terminal controls and layout forms:

```rust,ignore
vstack(
    12.0,
    [
        text_block("Document"),
        hstack(8.0, [button("Save", save), button("Close", close)]),
    ],
)
```

Use the named builders when a control needs optional properties, structural content, controlled
state, or more than one event.

## Hooks, state, context, and resources

Hooks are ordered slots owned by a component generation. A component must call the same hook kinds
in the same order on every render.

| Hook | Purpose |
| --- | --- |
| `use_state` | Cloned state with `value`, `set`, and `update`. |
| `use_async_state` | State completion from worker threads. |
| `use_ref` | Mutable component storage that does not render. |
| `use_memo` | Recompute a cloned value when dependencies change. |
| `use_callback` | Keep callback identity while dependencies are equal. |
| `use_reducer` | Apply typed actions through a reducer. |
| `use_effect` | Run post-commit work after dependency changes. |
| `use_timeout` | Run a one-shot dispatcher timer. |
| `use_interval` | Run a repeating dispatcher timer. |
| `use_resource` | Run cancellable Windows thread-pool work. |
| `use_mutation` | Track a triggered worker operation and its result. |

`State::set`, `State::update`, and `HookRef::set` act only while their component generation is
live. A stale mutation is ignored, and a stale `update` does not call its closure. `State::value`
is the render-time read and treats a stale handle as an invariant violation. Code that knowingly
retains a handle beyond component lifetime can use `try_value`.

Effects run after the complete native commit. Dependency changes run the prior cleanup before the
new effect. Unmount runs cleanup exactly once. Timers, worker completions, and resources carry
component, hook-slot, and revision identity so work queued for a removed owner is rejected.

`Context<T>` creates a runtime context identity. `ContextKey<T>` provides a const-constructible key
for module-level contexts:

```rust,ignore
static THEME: ContextKey<Theme> = ContextKey::new(default_theme);

provide_context_key(
    &THEME,
    Theme::Dark,
    component(|cx| page(cx.use_context_key(&THEME))),
)
```

Each Reactor owns its context defaults and provider values. Context changes participate in
component memoization, so an equal props value does not hide a changed inherited context.

`use_resource` passes a `CancellationToken` and dependencies to a worker closure. The result is
`Resource::Loading`, `Resource::Ready`, or `Resource::Failed`. Dependency changes cancel the prior
token and reject its late completion.

## Reconciliation, keys, and collections

The application layer is the only reconciler. It mounts, updates, replaces, reorders, and retires
nodes in one generational arena. The WinUI runtime applies typed commands and queues typed events;
it does not own another logical tree.

Positional children retain identity while their kind and position match. `.key(u64)` gives a child
application identity that survives insertion, removal, and reorder. Component identity also
includes its render closure type, and props components include the props type.

Every node has one structural parent. Secondary native relationships use explicit roles rather
than inferred child positions. These roles include content, pane, header, tooltip, TeachingTip
target, flyout content, dialog content, command sections, owned windows, and native-host content.

Collection controls keep application identity separate from native positions:

- `ListBox` and virtual collections use `CollectionSelection` for zero, one, or many keys;
- `ComboBox`, `RadioButtons`, `SelectorBar`, and `NavigationView` use `Option<u64>`;
- `Pivot`, `FlipView`, and `TabView` use controlled selected indices;
- keyed descriptors retain native item identity across label and order changes;
- virtual rows are ordinary arena-owned subtrees keyed by application item identity; and
- realization leases reject callbacks from recycled rows.

`VirtualList` and `VirtualGrid` use WinUI realization while Reactor owns each realized row subtree.
Selection callbacks report keys, not positions. Reorder callbacks report the complete final key
order for the application to store.

## Controlled state and events

Interactive values pair the declared value with a callback. Reactor stores the expected native
feedback beside the mounted native handle.

| Family | Examples |
| --- | --- |
| Text | `TextBox`, `PasswordBox`, `RichEditBox`, `AutoSuggestBox` |
| Toggle | `CheckBox`, `RadioButton`, `ToggleButton`, `ToggleSwitch` |
| Scalar | `Slider`, `NumberBox`, `RatingControl`, `ColorPicker` |
| Date and time | `DatePicker`, `CalendarDatePicker`, `CalendarView`, `TimePicker` |
| Selection | List, virtual, navigation, selector, pivot, flip, and tab controls |
| Structure | `SplitView`, `Expander`, `TreeView`, and window close state |

Native feedback updates the mounted comparison value before the current callback runs. If the next
render accepts that value, Reactor does not write it back. If application state retains the prior
value, reconciliation restores the declaration. Reactor-originated writes do not echo through the
application callback.

Display-only forms use `display(...)` or a display-state modifier where the control supports one.
They prevent native input from drifting away from the declared value. Notifications that do not
own a declarative value retain control-specific events, such as clicks, dialog results, query
submission, scrolling, invocation, and close requests.

Native callbacks queue events and request the application pump. Rendering and reconciliation do
not run inside the native callback or inside a native mutation batch. Event targets include node
generation and any required key, revision, or realization lease.

## Windows, dialogs, and structural content

Windows are ordinary keyed application children. A `Window` supports:

- title, icon, theme, backdrop, presenter, client size, and size constraints;
- overlapped presenter policy and a custom typed `TitleBar`;
- size and resolved color-scheme callbacks;
- owned windows;
- application resources and element resource overrides; and
- a generation-bound `WindowRef` for activation.

An OS close request invokes the declared callback. The window closes when application state removes
it. Removing an owner closes its owned windows first.

`ContentDialog`, `TeachingTip`, tooltips, flyouts, menus, and command bars use explicit ownership
and structural slots. Their content remains normal Reactor subtrees with normal component, hook,
key, and cleanup behavior. Delayed opens and native async operations are accepted only while the
owning node generation and operation revision remain current.

Typed element references publish a target after native commit and clear it before native
destruction. They expose operations such as TextBox focus without exposing raw WinUI controls.

## Composition, Canvas, and WebView

`CompositionHost` is available in the default build. It owns a WinUI host, a root Composition
visual, typed application state, layout callbacks, and a generation-bound
`CompositionHostRef<T>`.

Enable Canvas integration with:

```toml
windows-reactor = { version = "0.100", features = ["canvas"] }
```

The `canvas` feature adds:

- `animated_canvas` for continuous drawing;
- `swap_chain_canvas` for first-layout and size or scale drawing;
- `swap_chain_canvas_invalidated` for explicit demand-driven repaint;
- `canvas_image` and `canvas_image_invalidated` for `SurfaceImageSource` content; and
- `SwapChainHost` and `SwapChainHostRef<T>` for application-owned swap-chain state.

Canvas surfaces own their GPU device, native surface, subscriptions, draw callback, and device-loss
recovery. Draw coordinates and layout sizes are DIPs; pixel sizes and rasterization scales are
reported separately.

Enable WebView integration with:

```toml
windows-reactor = { version = "0.100", features = ["webview"] }
```

`WebViewHost` owns the XAML WebView2 control, async creation owner, safe
`windows_webview::WebView`, event registrations, source state, and pending commands.
`WebViewRef` provides generation-bound navigation, reload, stop, back, and forward commands.
Creation and navigation completion are queued through the Reactor event path.

The WebView2 process-failure, download, protocol, script, cookie, profile, and deferral APIs remain
available through [`windows-webview`](windows-webview.md).

## Ownership, lifecycle, and threading

The runtime follows these invariants:

| Area | Invariant |
| --- | --- |
| Ownership | One arena owns logical nodes, native nodes, windows, and realized rows. |
| Structure | Every node has one parent; native relations use typed slots. |
| Reconciliation | The application layer is the only reconciler. |
| Mutation | Native changes are closed typed command batches after validation. |
| Callbacks | Native callbacks queue events and never render directly. |
| Stale work | Generations, slots, revisions, keys, and leases reject old work. |
| Controlled values | Expected native feedback is stored with its native owner. |
| Teardown | References clear first; descendants retire child-first. |
| Failure | Invariant and unrecoverable native failures are terminal. |
| Public API | Runtime protocol, bindings, mounted state, and fixtures remain private. |

Reactor runs WinUI on one STA thread. Native controls, event registrations, windows, and
Composition or Canvas resources stay on that thread. Worker resource closures receive only
`Send + 'static` data and return through the dispatcher.

The native runtime owns one native-node map and adapter modules for control families. It depends on
typed commands and events, not on hooks or application reconciliation. Native handles and event
revokers retire through structural ownership.

## Samples

The final Reactor sample tree contains 13 Cargo packages:

| Package | Purpose |
| --- | --- |
| `reactor_samples` | Focused controls, hooks, applications, and host examples. |
| `reactor_gallery` | A 65-route WinUI control gallery. |
| `reactor_composition` | Typed Composition host examples. |
| `reactor_direct2d` | Application-owned Direct2D swap-chain host. |
| `reactor_swap_chain_panel` | Raw swap-chain panel integration. |
| `reactor_webview` | Typed WebView2 host. |
| `reactor_framework_dependent` | Installed Windows App Runtime deployment. |
| `reactor_self_contained` | Private Windows App Runtime deployment. |
| `reactor_startup_perf` | Startup TraceLogging and MSIX layout. |
| `reactor_windows` | Multiple keyed windows and independent state. |
| `reactor_matched` | Public acceptance workload. |
| `reactor_matched_hooks` | Hook and resource acceptance workload. |
| `reactor_matched_canvas` | Canvas acceptance workload. |

Representative commands:

```text
cargo run -p reactor_samples --example counter
cargo run -p reactor_samples --example acceptance
cargo run -p reactor_gallery
cargo run -p reactor_composition --example host
cargo run -p reactor_direct2d
cargo run -p reactor_webview
cargo run -p reactor_framework_dependent
cargo run -p reactor_self_contained
cargo run -p reactor_windows
cargo run -p reactor_startup_perf --release
```

The focused examples also include Composition, Direct2D, WebView, dialogs, keyed collections,
virtualization, transitions, resources, and the five complete applications. The gallery test keeps
all 65 registered routes.

## Code organization

| Path | Responsibility |
| --- | --- |
| `src/element/` | Public elements, builders, values, windows, and tree records. |
| `src/hooks.rs` | Component hooks, state, effects, timers, and resources. |
| `src/resources.rs` | Context and application or element resource values. |
| `src/references.rs` | Generation-bound element and window references. |
| `src/app/` | Mounting, reconciliation, events, and control-family logic. |
| `src/arena.rs` | Generational ownership arena. |
| `src/mounted.rs` | Committed mounted-node state. |
| `src/runtime.rs` | Private typed native command and event protocol. |
| `src/engine/` | Validation, command routing, tree updates, and virtualization. |
| `src/winui/` | WinUI host, lifecycle, and native control adapters. |
| `src/composition.rs` | Typed Composition host. |
| `src/canvas.rs` | Canvas feature and typed swap-chain host. |
| `src/webview.rs` | WebView feature and generation-bound commands. |
| `testing/` | Unit, private WinUI, package, API, stability, and performance tests. |

Reactor control behavior, properties, event routing, and native adapters are hand-written. There
is no Reactor semantic generator or generated control catalog. `src/bindings.rs` is generated ABI
support from `tool_bindings` and is not the application model or a public API.

## Testing and maintenance

Use the smallest relevant command during development:

```text
cargo fmt -p windows-reactor -p tool_reactor_coverage
cargo clippy -p windows-reactor --all-targets
cargo clippy -p windows-reactor --all-features --all-targets
cargo test -p windows-reactor --no-default-features
cargo test -p windows-reactor --all-features
```

Native fixtures and public interaction tests require the Windows App Runtime and an interactive
desktop:

```text
cargo test -p windows-reactor --release winui::tests:: -- \
    --ignored --test-threads=1
powershell -File crates\tests\libs\reactor_selftest\native.ps1
powershell -File crates\tests\libs\reactor_selftest\native.ps1 -Case smoke
crates\samples\reactor\samples\native.ps1 -Profile release
```

Check packaged source and the public API:

```text
crates\libs\reactor\testing\package.ps1
crates\libs\reactor\testing\public-api.ps1
```

The public API script checks default, `canvas`, `webview`, and all-feature snapshots. Regenerate
the committed snapshots after an accepted API change:

```text
crates\libs\reactor\testing\public-api.ps1 -Update
```

Coverage uses `tool_reactor_coverage` after producing the JSON report:

```text
cargo run -p tool_reactor_coverage --quiet -- target\reactor-coverage.json
```

The package also owns model benchmarks, native performance scenarios, and a stability matrix:

```text
crates\libs\reactor\benchmark.ps1 -BaseRef <ref>
crates\libs\reactor\testing\performance.ps1 -BaseRef <ref>
crates\libs\reactor\testing\stability.ps1
```

The `windows-reactor` workflow runs formatting, Clippy, model and all-feature tests, package and API
checks, native fixtures, sample checks, coverage, performance, and stability.
