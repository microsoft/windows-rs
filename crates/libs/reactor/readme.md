## windows-reactor

Windows Reactor is a declarative WinUI 3 library for Rust. A `Component` owns state, receives
parent-owned input, handles typed messages, and returns a `View`. Reactor reconciles each new view
with the native UI tree and applies the required WinUI changes.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)

Add Reactor:

```toml
[dependencies]
windows-reactor = "0.100"
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

Native operations that need the owning HWND can be staged from a component update:

```rust,ignore
let accepted = context.run_window(move |window| {
    let answer = unsafe {
        MessageBoxW(
            window.as_raw(),
            w!("Continue with this operation?"),
            w!("Confirm"),
            (MB_YESNO | MB_ICONQUESTION) as u32,
        )
    };
    Message::Answered(answer)
});
```

The operation runs on the UI thread in the next host dispatch after the current view publishes,
and its returned message is queued for a later update. Work is discarded if publication fails,
the component retires, or the window starts closing. Component dispatch for that window is
suspended until the operation returns, so use this for native modal UI rather than slow non-UI
work. Each window accepts one pending operation at a time. The
[`reactor-message-box`](../../samples/reactor/message-box) sample contains the complete component.
