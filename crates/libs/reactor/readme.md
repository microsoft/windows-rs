# Windows Reactor

Windows Reactor is a declarative WinUI 3 library for Rust. A `Component` owns state, receives
parent-owned input, handles typed messages, and returns a `View`. Reactor reconciles each new view
with the native UI tree and applies the required WinUI changes.

* [Guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)

Add Reactor:

```toml
[dependencies]
windows-reactor = "0.100"
```

Framework-dependent apps need nothing else: Reactor detects the deployment mode at startup
and either loads the staged runtime or resolves the installed Windows App Runtime framework
package. For a self-contained deployment, add the Windows App Runtime setup helper and call
`windows_reactor_setup::as_self_contained` from `build.rs`:

```toml
[build-dependencies]
windows-reactor-setup = "0.100"
```

```rust,ignore
fn main() {
    windows_reactor_setup::as_self_contained();
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

`ComponentContext` supplies message senders, background work, and window requests. `ViewContext`
builds the view and wires typed events to component messages. Generated control builders expose
typed properties, events, content, and child collections, and `ElementRef` provides typed
imperative operations such as focus. See the
[guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md) for
components, layout, controlled input, child components, changing lists, and background work.
