# windows-reactor

Build native WinUI 3 applications with Rust components, state, and typed messages.
Reactor is a preview; public APIs may change.

- 📦 [crates.io](https://crates.io/crates/windows-reactor)
- 📖 [docs.rs](https://docs.rs/windows-reactor)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor)

## Get started

```toml
[dependencies]
windows-reactor = "0.100"
```

Use the default features and an installed Windows App SDK runtime, or configure a
[self-contained build](https://docs.rs/windows-reactor-setup).

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

Run with `cargo run`. Events enqueue messages, `update` changes state, and `view` describes the
controls for that state.

## Grow the component

Declare editable values from state and send changes back as messages. Handle those messages in
`update`, not in `view`. Reactor applies declared values without echoing its own writes as user
edits; only properties you declare are controlled.

Use `View::component` for stateful children whose input implements `Clone + PartialEq`.
Give movable items stable application keys, not list indices, so state and focus follow the item.

Updates and rendering run on the UI thread. Use `spawn_background` for blocking work,
`set_timeout` for delayed messages, and effects for subscriptions tied to a component's lifetime.
Workers return messages rather than touching controls.

## Application lifetime and deployment

Use `App::run_application` when state must outlive individual windows. Its `ApplicationView`
declares keyed windows and notification icons. Load saved placement before opening a window and
save observations as they arrive.

For self-contained deployment, use `windows-reactor-setup` from `build.rs` and package the staged
runtime alongside the executable. Otherwise the matching Windows App SDK runtime must be installed.
Add `#![windows_subsystem = "windows"]` to hide the console; leave it visible while diagnosing
startup.

## Samples

| Sample | Shows |
| --- | --- |
| [Controlled input][controlled] | Editable state and callbacks |
| [Effects][effects] | Subscription lifetime |
| [Async state][async] | Background work |
| [Notification icon][notifyicon] | Application and window lifetimes |
| [Window placement][placement] | Persisted window bounds |
| [Gallery][gallery] | Controls and navigation |
| [Stacker][stacker] | Reactor, Canvas, and Composition together |

[controlled]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/controlled
[effects]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/use-effect
[async]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/async-state
[notifyicon]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/notifyicon
[placement]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/window-placement-registry
[gallery]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/gallery
[stacker]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/stacker
