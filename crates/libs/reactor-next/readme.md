# Windows Reactor Next

This unpublished crate is the experimental successor to `windows-reactor`. It is being developed
side by side with the current crate until its architecture, correctness, compile time, and runtime
performance are proven.

See [`reactor-next.md`](../../../reactor-next.md) for the current plan and gates.

The current slice generates `TextBlock`, `Button`, `StackPanel`, `TextBox`, `NumberBox`, `Slider`,
`NavigationView`, `ProgressBar`, `ToggleSwitch`, `ScrollViewer`, and `ItemsRepeater` from WinUI
metadata plus a small curation schema. `ToggleSwitch` adds typed boolean controlled feedback.
`NavigationView` exposes typed `Content` and `Header` slots through `View::slots`. The private WinUI
backend applies properties and keyed structure and queues native work. The recording runtime
remains the failure-injection and randomized-test backend.

Applications use owned components:

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

Local component messages stay on the UI thread. Each window queues at most 4,096 messages.
`LocalSender::send` returns `false` when the owning scope has retired, the window has closed, or
the queue is full.

Components may run blocking work on an owned background thread:

```rust,ignore
let task = context.spawn_background(|cancellation| {
    match load_data(cancellation) {
        Ok(value) => Message::Loaded(value),
        Err(error) => Message::LoadFailed(error),
    }
});
```

The closure receives a cooperative `CancellationToken`, and its `Send` result returns as the
component's normal message on the UI thread. Retiring the component or closing its window cancels
ownership and discards late results. `ComponentTask::cancel` also removes a queued result.
Dropping the handle does not cancel its task. `ComponentTaskStatus` reports `Running`, `Queued`,
`Delivered`, `Cancelled`, or `Rejected`.

Each window permits at most 64 live task threads and 4,096 queued completions. Work beyond either
limit returns a task with `Rejected` status instead of starting another thread. Tasks that ignore
cancellation may finish their closure, but they cannot deliver after their scope retires.

Typed context values flow through logical provider nodes:

```rust,ignore
let value = cx.use_context(context);
View::provide(
    context,
    "provided".to_string(),
    View::native(TextBlock::new().text(value)),
)
```

`Context::new` defines a typed key and its default. `View::provide` shadows that key for descendant
components. Provider changes use a published reverse dependency index and recompose the exact
surviving consumers. Changing a provider to a different context key does not recompose consumers
shadowed by nearer providers.

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
