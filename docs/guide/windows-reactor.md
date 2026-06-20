# windows-reactor

> A declarative, React-style UI library for Rust, backed by WinUI 3.

- 📦 Not published (experimental, `0.0.0`)
- 🛠 [Internals](../internals/windows-reactor.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)

## Overview

`windows-reactor` lets you describe a WinUI 3 user interface as a function of
state. You write a render function that takes a `RenderCx` and returns an
`Element`; the reactor diffs the result against the live visual tree and applies
only the changes. State lives in hooks such as `cx.use_state`, and updating it
schedules a re-render.

Elements are built with plain functions and builder methods — `text_block`,
`button`, `vstack`, `hstack`, `grid`, and so on — and an `App` drives the
render loop.

> **Status**: experimental and unpublished. The API evolves alongside the
> samples in the repository.

## Example

A counter with state and a click handler:

```rust,no_run
use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);
    let increment = move || set_count.call(count + 1);

    vstack((
        text_block(format!("Count: {count}"))
            .font_size(28.0)
            .bold(),
        button("+").on_click(increment),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Counter").render(app)
}
```

Building a real app also needs a `build.rs` that calls
[`windows-reactor-setup`](windows-reactor-setup.md) to stage the Windows App SDK
runtime. See the [samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
for complete, runnable projects.