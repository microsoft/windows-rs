# windows-reactor

`windows-reactor` is a declarative WinUI 3 library. Applications own state in `Component` values
and describe native UI with `View` values. The runtime reconciles each view against the retained
tree, publishes native commands, and applies them on the UI thread.

The [crate readme](../../crates/libs/reactor/readme.md) contains the dependency setup and a complete
counter. Samples are under `crates/samples/reactor`.

## Architecture

| Layer | Location |
| --- | --- |
| Public frontend | `src/core/public.rs`, `src/generated.rs` |
| Reconciler | `src/core/pump` |
| Scheduling | `src/core/engine.rs`, `src/core/scheduler.rs` |
| Native runtime | `src/native/winui` |
| Test runtime | `src/native/recording.rs` |
| Integration API | `src/reference.rs` |

Owned `Component` values produce the one public `View` frontend. One structural tree and command
path handle controls, components, slots, overlays, dialogs, and windows. Keyed views retain
ownership across reorder and replacement. Messages enter through queued dispatch, and
generational identities reject work for retired components and nodes.

The public frontend defines components, control builders, callbacks, and values. The pump plans
tree changes, lifecycle work, and command publication. The WinUI runtime creates native objects and
handles properties, events, and windows, while `RecordingRuntime` records the same command stream
for deterministic tests. Typed references expose the Canvas, Composition, WebView, and focus
integration boundaries.

Unexpected native command failures are fatal. Continuing after a partially applied WinUI update
would leave the retained and native trees out of sync.

When one command batch destroys a native subtree, the WinUI runtime detaches only the subtree's
external edge. It does not clear internal slots or collections that are destroyed in the same
batch. This keeps stateful controls intact while WinUI drains deferred visual-state callbacks.
`RecordingRuntime` still applies every logical detach and checks the complete command sequence.

Debug builds print a reconciliation trace before applying each nonempty component update. Each
line identifies the composed component types, the native event that triggered observation
reconciliation, and counts for property, topology, subscription, creation, and destruction
commands. The trace is emitted before calling WinUI, so the last line remains useful if native
application does not return. Release builds omit this output.

### Component model

`Component::Input` is parent-owned declarative data. The runtime compares it with the retained
input and calls `Component::input_changed` when it differs. `Component::Message` is the typed
request channel for local updates.

`create`, `input_changed`, and `update` receive a shared `ComponentContext<Self>` for senders,
background tasks, and window requests. `view` receives a mutable `ViewContext<Self>` because
rendering records declarations, dependencies, observations, and effects. Inputs remain explicit
arguments to `create`, `input_changed`, and `view`.

Generated controls convert directly into `View`. The internal `Element` representation and
structural state remain private so applications depend on builders and capability traits rather
than generated enum variants or reconciliation storage.

### Migration from the render-and-hook API

The Component/View API replaces the earlier render-function and hook frontend:

| Render-and-hook API | Component/View API |
| --- | --- |
| `App::new().render(render)` | `App::run_component::<C>(input)` |
| `fn render(&mut RenderCx) -> Element` | `Component::view(..) -> View` |
| `RenderCx::use_state` | Component fields changed by typed messages |
| Hook callbacks | `ViewContext::callback`, `message`, or `forward` |
| Hook effects | `ViewContext::use_effect` |
| Widget factory functions | Generated typed control builders |
| `ReactorWindow` | `ComponentContext::open_window` and `WindowRef` |
| `windows-reactor/canvas` | `windows-canvas` with its `reactor` feature |

Applications now stage the Windows App Runtime from `build.rs` with `windows-reactor-setup`.
Framework-dependent builds call `as_framework_dependent`; self-contained builds call
`as_self_contained`.

### Native integration boundary

Reactor-owned XAML objects do not cross the public API boundary. An integration must not receive a
raw `IInspectable` from a mount callback and mutate it outside reconciliation. Such mutation
bypasses the command path and cannot be represented by `RecordingRuntime`.

An integration may add a narrow typed command when the operation is part of a public contract.
The command may carry an application-owned native payload, such as a swap chain, but it must
target a typed `ElementRef`, define its lifetime behavior, and remain observable in the recording
runtime. Sample compatibility alone does not justify a new public command.

`ReferenceControl` is the sealed capability implemented by generated controls that support
`ElementRef`. `FocusControl` narrows that set to controls that accept focus requests.

The `observe_surface`, `observe_rasterization_scale`, and `observe_composition_host` methods
register against an `ElementRef`, not one native node. Registration may happen before publication.
A published binding queues the native subscription, and structural replacement queues it again
for the replacement node. Native teardown retires the old subscription. Callbacks check current
window and node identities before delivery.

The returned `ElementObservation` owns the registration. Dropping it stops delivery and prevents
later rebinding. This separate owner lets a callback capture its `ElementRef` without forming a
reference cycle. Attachment and focus methods remain one-shot commands for the currently
published node and return `false` while unbound.

`CompositionHostEvent::Ready` exposes the host compositor and layout metrics. The host `Grid` and
its element visual remain Reactor-owned and cannot be mutated through the public boundary.

Accepted one-shot requests complete exactly once with a value or `IntegrationError`. `Native`
preserves an HRESULT, while `Unavailable` means the published target retired or could not accept
the request. Focus, WebView2, swap-chain, image-source, and Composition error names remain aliases
for this shared contract. Higher-level integrations use a result-bearing readiness callback for a
one-shot operation and `on_error` for a recurring surface.

## Code generation

`crates/tools/reactor` refreshes the committed WinUI, Windows App SDK, and WebView2 metadata plus
the `windows-reactor-setup` bootstrap DLLs from pinned NuGet packages. It then reads
`src/winui.toml` and the metadata under `winmd` and generates:

| Output | Contents |
| --- | --- |
| `crates/libs/reactor/src/generated.rs` | Public builders and retained property/event data |
| `crates/libs/reactor/src/native/winui/generated.rs` | WinUI command application |
| `crates/libs/reactor/src/native/winui/bindings.rs` | Minimal WinUI bindings |
| `crates/libs/canvas/src/reactor_bindings.rs` | Minimal Canvas integration bindings |

`crates/tools/reactor/src/bindings.txt` contains the hand-maintained runtime binding filter.
`control_bindings.txt` is generated from `winui.toml` and supplies control-specific entries.

These Rust files are committed generated output. Do not edit them by hand. After changing
`winui.toml`, the schema resolver, or either generator, run:

```text
cargo run -p tool_reactor --quiet
cargo run -p tool_reactor --quiet
cargo check -p windows-reactor --quiet
```

The second run must leave the tree unchanged.

## Validation

The validation layers have separate owners:

| Layer | Command or location |
| --- | --- |
| Internal deterministic tests | `cargo test -p windows-reactor` |
| External API tests | `cargo test -p test_reactor` |
| Generator tests | `cargo test -p tool_reactor` |
| Live WinUI fixtures | `cargo run -p test_reactor_selftest -- --headless` |
| Planner benchmarks | `cargo run -p test_reactor_bench --release` |
| Live grid benchmark | `cargo run -p test_reactor_bench --bin reactor-live-grid --release` |
| Consumer coverage | `sample_reactor_virtual`, `sample_reactor_navigation`, and the gallery |

`crates/libs/reactor/public-api.txt` is the checked public API snapshot. Regenerate it with the
repository's pinned `cargo-public-api` process after an intentional API change.

## Deployment

Applications use `windows-reactor-setup` from `build.rs` to stage the Windows App Runtime.
Framework-dependent and self-contained examples live in
`crates/samples/reactor/framework_dependent` and `crates/samples/reactor/self_contained`.
`windows-reactor-setup` is a separate crate and is not part of Reactor's generated API.

## Non-blocking API follow-ups

- **Swap-chain surface:** `ElementRef<SwapChainPanel>` has separate attachment and observation
  requests. `observe_surface` combines metrics and frame notifications, invokes the callback on
  the UI thread for external GPU work, and follows each published binding. A future review may
  prefer a narrower DXGI wrapper over an application-owned `IUnknown`.
- **Native image source:** Displaying a `CanvasImageSource` requires
  `ElementRef<Image>::request_set_native_source` plus an `observe_rasterization_scale`
  subscription so Canvas can allocate physical pixels. A future API may combine attachment and
  scale observation.
- **Composition host:** Lifted Composition uses `ElementRef<Grid>` rather than a dedicated host
  type. `observe_composition_host` reports the compositor, size, and rasterization scale, while
  `request_set_child_visual` remains a separate command. A dedicated host object may communicate
  the ownership contract more directly.
- **Custom title bar:** Every `TitleBar` automatically becomes the window title bar at standard
  height; `preferred_height` only selects standard or tall system chrome. Removing the control
  restores the system title bar, and multiple controls are rejected. A final API review should
  confirm that disallowing decorative `TitleBar` controls is the right tradeoff and decide whether
  multi-row hosts justify a broader contract.
