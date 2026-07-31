## Windows Reactor

Windows Reactor is a declarative WinUI 3 library with render functions, state hooks, and widget
builders.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Reactor
  guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)

A minimal app defines a render function `fn(&mut RenderCx) -> Element` and passes it to
`App::render`:

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

`bootstrap()` initializes the Windows App SDK runtime for a framework-dependent app. Widget
builders convert to `Element` with `.into()`. `cx.use_state` returns the current value and a handle
whose `call` schedules a rerender. `ReactorWindow` opens more top-level windows. See the [reactor
guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md) for
components, hooks, layout, styling, and widgets.
