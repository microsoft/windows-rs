## windows-reactor

Windows Reactor is a typed declarative UI library for building native WinUI 3 applications in
Rust. Components own Rust state, describe the current view, and receive typed messages from
controls.

Add Reactor to `Cargo.toml`:

```toml
[dependencies.windows-reactor]
version = "0.100"
```

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

Run with `cargo run`. Applications use the default features and an installed Windows App SDK
runtime. See the [setup guide][setup] for runtime prerequisites and self-contained deployment.

The [getting-started guide][guide] covers editable state, child components, background work, images,
and application windows. Browse the [samples] for complete applications and focused examples.

[setup]: https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor-setup.md
[guide]: https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md
[samples]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor
