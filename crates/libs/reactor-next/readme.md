# Windows Reactor Next

This unpublished crate is the experimental successor to `windows-reactor`. It is being developed
side by side with the current crate until its architecture, correctness, compile time, and runtime
performance are proven.

See [`reactor-next.md`](../../../reactor-next.md) for the current plan and gates.

The current slice generates `TextBlock`, `Button`, `StackPanel`, `TextBox`, `ScrollViewer`, and
`ItemsRepeater` from WinUI metadata plus a small curation schema. The private WinUI backend applies
properties and keyed structure, queues native work, and rerenders hook state. The recording
runtime remains the failure-injection and randomized-test backend.

Applications can use either the root hook frontend or an owned component:

```rust,no_run
use windows_reactor_next::*;

struct Root;

impl Component for Root {
    type Message = ();
    type Props = ();

    fn create(_: &(), _: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _: &(), _: &mut ComponentContext<Self>) {}

    fn update(&mut self, _: (), _: &mut ComponentContext<Self>) {}

    fn view(&self, _: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text("Hello"))
    }
}

App::run_component::<Root>(())?;
# Ok::<(), windows_core::Error>(())
```

Local component messages stay on the UI thread. Each window queues at most
`LOCAL_MESSAGE_QUEUE_CAPACITY` messages. `LocalSender::send` returns `false` when the owning scope
has retired, the window has closed, or the queue is full.

Props, messages, and thread ownership are checked at the public boundary:

```compile_fail
use windows_reactor_next::LocalSender;

fn wrong_message(sender: LocalSender<u32>) {
    sender.send("not a number");
}
```

```compile_fail
use windows_reactor_next::LocalSender;

fn cross_thread(sender: LocalSender<()>) {
    std::thread::spawn(move || {
        sender.send(());
    });
}
```
