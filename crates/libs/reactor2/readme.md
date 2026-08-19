# windows-reactor

[![crates.io badge][crates-badge]][crates]
[![docs.rs badge][docs-badge]][docs]

`windows-reactor` is a declarative WinUI 3 library for Windows applications. Render functions
produce an `Element` tree from state. Reactor retains native controls, reconciles later renders,
and applies typed updates while preserving WinUI-owned state such as focus and scroll position.

- [Guide](../../../docs/crates/windows-reactor.md)
- [API documentation](https://docs.rs/windows-reactor)
- [Samples](../../samples/reactor)
- [Runtime setup](../reactor-setup/readme.md)

## Features

WinUI support is always available. Optional features add integrations:

| Feature | Adds |
| --- | --- |
| `canvas` | `windows-canvas` drawing elements and `SwapChainHost`. |
| `webview` | `WebViewHost` backed by `windows-webview`. |

docs.rs builds both optional features.

## Add the package

```toml
[dependencies]
windows-reactor = "0.100"

[build-dependencies]
windows-reactor-setup = "0.100"
```

Framework-dependent applications stage bootstrap files in `build.rs`:

```rust,ignore
fn main() {
    windows_reactor_setup::as_framework_dependent();
}
```

Use `as_self_contained()` to stage a private Windows App Runtime. Self-contained applications do
not call `windows_reactor::bootstrap()`.

## Counter

```rust,no_run
use windows_reactor2::{
    Application, Element, RenderCx, Window, button, component, run_reactor_winui_app, text_block,
    vstack,
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
                        text_block(format!("Count: {}", count.value())),
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

fn main() {
    windows_reactor2::bootstrap().unwrap();
    run_reactor_winui_app(component(app)).unwrap();
}
```

`Window` close callbacks update application state. Removing the final window ends the application
run.

## Components and hooks

Use ordinary Rust functions for stateless composition. Use `component` for a subtree with hooks or
an independent render lifecycle. `component_with_props` owns props and borrows them during
rendering. `memo_component` and `memo_component_with_props` skip equal parent-driven renders.

`RenderCx` provides:

- `use_state`, `use_async_state`, `use_ref`, `use_memo`, `use_callback`, and `use_reducer`;
- `use_effect` and `use_effect_with_cleanup`;
- `use_timeout` and `use_interval`;
- `use_resource` for cancellable Windows thread-pool work;
- `use_mutation` for triggered worker operations;
- `use_context` and `use_context_key`; and
- typed element, window, Composition, Canvas, and WebView references.

State handles are generation-bound. `set` and `update` are safe to call from retained callbacks;
they do nothing after the owning component is removed. Effects run after native commit and clean
up when dependencies change or the component unmounts.

## Builders, keys, and controlled values

Typed builders expose only compatible layout, styling, accessibility, input, resource, and
attached-layout modifiers. Apply modifiers before `.build()`:

```rust,ignore
Button::new("Save")
    .on_click(save)
    .width(120.0)
    .automation_name("Save document")
    .build()
```

Use `.key(u64)` when child identity must survive insertion or reorder. Keyed collection APIs report
application keys rather than native positions. `VirtualList` and `VirtualGrid` use WinUI
realization while Reactor owns each realized row subtree.

Interactive values pair a declared value with a callback:

```rust,ignore
TextBox::new(name, move |value| set_name.set(value)).build();
ToggleSwitch::new(enabled, move |value| set_enabled.set(value)).build();
Slider::new(volume, move |value| set_volume.set(value))
    .range(0.0, 100.0)
    .build();
```

Reactor-originated writes do not echo through the callback. Display-only controls use
`display(...)` or a display-state modifier where supported.

## Windows and structural content

`Application` can declare multiple keyed windows. `Window` supports owned windows, themes,
backdrops, presenters, constraints, custom title bars, resources, and a generation-bound
`WindowRef`.

Headers, panes, dialog content, tooltips, TeachingTips, flyouts, command sections, and native-host
content are explicit structural slots. Their child elements keep ordinary component, hook, key,
and cleanup behavior.

## Native hosts

The default build includes `CompositionHost` and `CompositionHostRef<T>`. The optional features add
typed Canvas and WebView hosts:

- `CompositionHost` is in the default build. See
  [`composition.rs`](../../samples/reactor/samples/examples/composition.rs).
- `SwapChainHost` is in the `canvas` feature. See
  [`direct2d_host.rs`](../../samples/reactor/samples/examples/direct2d_host.rs).
- `WebViewHost` is in the `webview` feature. See
  [`webview_host.rs`](../../samples/reactor/samples/examples/webview_host.rs).

Run them with:

```text
cargo run -p reactor_samples --example composition
cargo run -p reactor_samples --example direct2d_host
cargo run -p reactor_samples --example webview_host
```

The `canvas` feature also provides:

- `animated_canvas` for continuous drawing;
- `swap_chain_canvas` for size or scale driven drawing;
- `swap_chain_canvas_invalidated` for explicit repaint; and
- `canvas_image` and `canvas_image_invalidated` for image content.

`WebViewRef` provides generation-bound navigation, reload, stop, back, and forward commands.

## Samples

The [Reactor sample tree](../../samples/reactor) has 13 packages. It includes focused examples,
five complete applications, a 65-route gallery, deployment modes, startup tracing, multiple
windows, acceptance workloads, and typed Composition, Direct2D, swap-chain, and WebView hosts.

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
```

Run the public sample smoke suite and native selftests with:

```text
crates\samples\reactor\samples\native.ps1 -Profile release
powershell -File crates\tests\libs\reactor_selftest\native.ps1
```

See the [windows-reactor guide](../../../docs/crates/windows-reactor.md) for architecture,
reconciliation, lifecycle, testing, API snapshots, and maintenance commands.

[crates]: https://crates.io/crates/windows-reactor
[crates-badge]: https://img.shields.io/crates/v/windows-reactor.svg
[docs]: https://docs.rs/windows-reactor
[docs-badge]: https://docs.rs/windows-reactor/badge.svg
