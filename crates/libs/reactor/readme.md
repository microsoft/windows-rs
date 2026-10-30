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

Apps use an installed Windows App Runtime by default. To ship the runtime with your app, configure
a [self-contained build](https://docs.rs/windows-reactor-setup).

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

Run it with `cargo run`. A click sends a message to `update`, which changes the state. Reactor
then calls `view` again and updates only the controls that changed.

## Showcase

Clone the repository and run these samples from its root.

### Gallery

Browse WinUI controls built with Reactor.

```sh
cargo run -p reactor-gallery
```

![Reactor gallery](https://raw.githubusercontent.com/microsoft/windows-rs/master/docs/reactor-gallery.png)

### Solitaire

A complete game built from standard controls.

```sh
cargo run -p reactor-solitaire
```

![Solitaire](https://raw.githubusercontent.com/microsoft/windows-rs/master/docs/reactor-solitaire.png)

### Stacker

Reactor, Canvas, and Composition together.

```sh
cargo run -p reactor-stacker
```

![Stacker](https://raw.githubusercontent.com/microsoft/windows-rs/master/docs/reactor-stacker.png)

## Next steps

| To | Use | Sample |
| --- | --- | --- |
| Edit text and other values | Set the value from state, handle its change event in `update` | [Controlled input][controlled] |
| Split the UI into parts | `View::component`, with input that implements `Clone + PartialEq` | [Component input][component] |
| Keep state with list items | `KeyedView` with a stable key per item, not its index | [Keyed list][keyed] |
| Run blocking work | `spawn_background`; the work returns a message | [Async state][async] |
| Start and stop a subscription | `use_effect` | [Effects][effects] |
| Add windows or a tray icon | `App::run_application` | [Notification icon][notifyicon] |
| Remember window placement | `initial_placement` and `on_window_placement` | [Window placement][placement] |

Updates and rendering run on the UI thread, so keep `update` and `view` quick.

## Ship the app

Add `#![windows_subsystem = "windows"]` to hide the console window; leave it visible while
diagnosing startup. For a self-contained app, use
[windows-reactor-setup](https://docs.rs/windows-reactor-setup) from `build.rs` and distribute the
staged runtime files alongside the executable.

[controlled]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/controlled
[component]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/component-input
[keyed]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/keyed-list-reorder
[effects]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/use-effect
[async]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/async-state
[notifyicon]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/notifyicon
[placement]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor/window-placement-registry
