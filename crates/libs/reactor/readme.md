## windows-reactor

Windows Reactor is a typed declarative UI library for building native WinUI 3 applications in
Rust. Components own Rust state, describe the current view, and receive typed messages from
controls.

Applications use the default features. The `test` feature exposes headless hosts, runtime
protocols, and diagnostics for framework tests and benchmarks; it is not needed to build a UI.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)

Start by adding the following to your Cargo.toml file:

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

Ordinary unpackaged applications use an installed Windows App SDK framework package. For a
self-contained application, add `windows-reactor-setup` as a build dependency and call
`windows_reactor_setup::as_self_contained()` from `build.rs`.

Control icon slots accept the shared `Icon` content value. Reactor realizes it as the native
`IconElement` or `IconSource` required by each control, so the same value works with APIs such as
`AutoSuggestBox::query_icon`, `AppBarButton::icon`, and `TitleBar::icon`. Standalone visual icons
continue to use `SymbolIcon`, `FontIcon`, `BitmapIcon`, `ImageIcon`, and `PathIcon`.
