## Windows Reactor

Windows Reactor is a declarative UI library for Rust developers, backed by WinUI 3. It uses a React-like component model — render functions, hooks for state, and a builder-pattern DSL for composing UI elements.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Reactor guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)

A minimal app defines a render function `fn(&mut RenderCx) -> Element` and passes it to `App::render`:

```rust,no_run
use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);

    vstack((
        text_block(format!("count = {count}")).font_size(18.0).bold(),
        button("Click").on_click(move || set_count.call(count + 1)),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> windows_core::Result<()> {
    bootstrap()?;
    App::new().title("My App").render(app)
}
```

`bootstrap()` initializes the Windows App SDK runtime and is required once at
startup for a framework-dependent app (self-contained deployments handle this
differently — see the samples). Widget builders convert to `Element` via `.into()`, and `cx.use_state` returns the current value plus a handle whose `call` schedules a rerender. Apps can open additional top-level windows at runtime with `ReactorWindow` (each hosting its own independent state). See the [reactor guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md) for components, hooks, layout, styling, multiple windows, and the full widget catalog.
