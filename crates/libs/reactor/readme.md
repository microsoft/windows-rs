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
        let content = StackPanel::new().spacing(8.0).children((
            TextBlock::new().text(format!("Count: {}", self.count)),
            Button::new()
                .on_click(context.forward())
                .content("Increment"),
        ));
        context.window_frame("Counter", content)
    }
}

fn main() {
    App::run_component::<Counter>(()).unwrap();
}
```

Applications with resources that outlive any one window can use `App::run_with`:

```rust,ignore
App::run_with(|app| {
    app.open_window(View::component::<Counter>(()))?;

    let exit = app.proxy();
    let tray = TrayIcon::new("app.ico")
        .menu(Menu::new().item(1, "Exit"))
        .on_event(move |event| {
            if matches!(event, TrayIconEvent::MenuItem { id: 1 }) {
                _ = exit.exit();
            }
        })
        .build()?;

    Ok(tray)
})
```

The returned value remains alive until explicit application exit. The application may start with
no Reactor windows, and closing its last window does not end the message loop. `AppContext` opens
windows and exits from the UI thread and may be cloned into UI-thread callbacks. Its cloneable
`AppProxy` posts work or requests exit from other threads.

`window_frame` creates an integrated WinUI title bar and places the application content below it:

```rust,ignore
context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Acrylic));
context.window_frame("Canvas keyboard input", content)
```

Use `ViewContext::window_title` directly when the window should retain the system title bar.

Focusable custom surfaces can make synchronous WinUI routing decisions without running component
updates inside native callbacks:

```rust,no_run
use windows_reactor::*;

fn keyboard_surface<C: Component<Message = Message>>(
    context: &mut ViewContext<C>,
    content: impl Into<View>,
) -> View {
    Border::new()
        .is_tab_stop(true)
        .focus_on_pointer_release(true)
        .on_preview_key_down(context.routed_callback(|info: KeyEventInfo| {
            if info.key == VirtualKey::LEFT {
                RoutedMessage::handled(Message::MoveLeft)
            } else {
                RoutedMessage::bubble_without_message()
            }
        }))
        .content(content)
}

enum Message {
    MoveLeft,
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
