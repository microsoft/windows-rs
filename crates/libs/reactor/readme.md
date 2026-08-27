# Windows Reactor

Windows Reactor is a declarative WinUI 3 library for Rust. A `Component` owns state, receives
parent-owned input, handles typed messages, and returns a `View`. Reactor reconciles each new view
with the native UI tree and applies the required WinUI changes.

* [Guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)

Add Reactor and the Windows App Runtime setup helper:

```toml
[dependencies]
windows-reactor = "0.100"

[build-dependencies]
windows-reactor-setup = "0.100"
```

For a framework-dependent application, stage the runtime from `build.rs`:

```rust,ignore
fn main() {
    windows_reactor_setup::as_framework_dependent();
}
```

This counter shows the Component/View model:

```rust,no_run
use windows_reactor::*;

struct Counter {
    count: u32,
}

impl Component for Counter {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.count += 1;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(8.0).children((
            TextBlock::new().text(format!("Count: {}", self.count)),
            Button::new()
                .on_click(context.forward())
                .content("Increment"),
        ))
    }
}

fn main() {
    App::run_component::<Counter>(()).unwrap();
}
```

`ComponentContext` supplies message senders, background work, and window requests.
`ViewContext::forward` sends an event payload that already matches the component message.
`ViewContext::message` maps a payload-free event to a fixed message. Generated control builders
expose typed properties, events, content, and child collections. `ElementRef` provides typed
imperative operations for focus and integration points that must run after publication.
