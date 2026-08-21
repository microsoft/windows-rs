# Windows Reactor Next

This unpublished crate is the experimental successor to `windows-reactor`. It is being developed
side by side with the current crate until its architecture, correctness, compile time, and runtime
performance are proven.

See [`reactor-next.md`](../../../reactor-next.md) for the current plan and gates.

The current slice generates `TextBlock`, `Button`, `StackPanel`, `TextBox`, `NumberBox`, `Slider`,
`NavigationView`, `ProgressBar`, `ToggleSwitch`, `ScrollViewer`, and `ItemsRepeater` from WinUI
metadata plus a small curation schema. `ToggleSwitch` adds typed boolean controlled feedback.
`NavigationView` exposes typed `Content` and `Header` slots through `SlotsControl::slots`. The
private WinUI backend applies properties and keyed structure and queues native work. The recording
runtime remains the failure-injection and randomized-test backend.

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

    fn update(&mut self, _: (), _: &mut ComponentContext<Self>) {}

    fn view(&self, _: &Self::Props, _: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("Hello").into()
    }
}

App::run_component::<Root>(())?;
# Ok::<(), windows_core::Error>(())
```

Generated controls convert directly into `View`. Structural capability traits keep composition on
that same core type:

```rust,ignore
StackPanel::new().spacing(8.0).children((
    TextBlock::new().text("Ready"),
    TextBox::new().placeholder_text("Name"),
    View::component::<Summary>(summary_props),
    Button::new()
        .is_enabled(true)
        .on_click(submit)
        .content(TextBlock::new().text("Submit")),
))
```

`ContentControl::content`, `ChildrenControl::children`, and `SlotsControl::slots` are terminal
builders that return `View`, so set control properties and events before calling them.
`ChildrenControl::children` and `View::fragment` accept the sealed `IntoViews` trait. `()` is
empty, fixed-size arrays provide homogeneous shapes, and tuples of up to 16 elements provide
heterogeneous shapes. Each tuple element implements `Into<View>`, so controls and component or
content views do not need per-leaf `.into()` calls.

Positional identity is available only to these statically shaped expressions. A dynamic list can
insert or remove an item and shift every later index, which would attach retained component state
to the wrong item - the React index-as-key state bug. Dynamic `Vec`, slice, and iterator inputs
therefore do not implement `IntoViews`. Use `ChildrenControl::keyed_children` with `KeyedView`, or
`View::keyed_fragment`, for dynamic collections. Positional keys occupy a private key domain and
cannot collide with public numeric or string keys.

These methods construct the core `View` variants consumed by the existing planner. They are not a
wrapper frontend and do not add another tree or reconciliation path.

`ItemsRepeater` accepts explicitly keyed `View` rows:

```rust,ignore
ItemsRepeater::new()
    .item("summary", View::component::<Summary>(summary_props))
    .items(records.map(|record| {
        KeyedView::new(record.id, Row::new().content(TextBlock::new().text(record.name)))
    }))
```

Rows are realized lazily. A row may contain native controls, components, providers, fragments, and
ordinary `View` composition. The logical row and its component, effect, and reference state remain
realized when it flattens to zero or multiple native roots. Exactly one root attaches to the native
shell. Zero roots leave it empty. Multiple roots leave it empty and publish
`PumpDiagnostic::VirtualRowRootCount`; the application host writes this diagnostic as a warning
without stopping the window. A later update that returns to one root reattaches the same logical
row. Recycling always retires the full logical subtree and detaches only its optional shell root.
Test and custom hosts can inspect committed diagnostics with `Pump::drain_diagnostics`.

Event setters accept ordinary unit-returning closures and typed message callbacks:

```rust,ignore
TextBox::new().on_text_changed(context.callback(Message::NameChanged));
NumberBox::new().on_value_changed(context.callback(Message::AmountChanged));
Button::new().on_click(context.message(Message::Submit));
```

`ViewContext::callback` maps an event payload into a component message.
`ViewContext::message` clones one message for each invocation and requires the component message
type to implement `Clone`. Both methods forward to the same methods on `LocalSender` and enqueue
through the existing local component queue.

Ordinary event closures are always accepted. A message callback retains the `bool` returned by
`LocalSender::send`. If the bounded component queue is full, the Pump leaves the native event at
the front of its queue, drains component work, and retries the callback on a later turn. Events
with stale window, node, subscription, or event revisions are discarded before callback
invocation.

Components can own typed imperative references as fields:

```rust,ignore
struct Form {
    name_ref: ElementRef<TextBox>,
}

TextBox::new().element_ref(&self.name_ref);
_ = self.name_ref.request_focus();
```

`request_focus` returns `true` only when the reference is bound to a published element and the
request enters its window's imperative queue. It never calls WinUI directly. The queued request
retains the exact window epoch and generational node ID, so replacement, removal, shutdown, and
window close discard stale work. WinUI returning `false` from `Focus(Programmatic)` is a completed
request, not a host error. `Button`, `TextBox`, `NumberBox`, `Slider`, and `ToggleSwitch` currently
carry the generated sealed focus capability. One reference can own one published element;
duplicate use returns `PumpError::DuplicateElementRef` before native mutation. Each window accepts
up to 4,096 pending imperative requests and applies at most 64 in one host turn.

Raw native handles remain intentionally absent. Specialized Canvas, WebView, and similar subsystem
adapters need separate ownership and documented-failure designs rather than cloned COM handles in
render, update, event, or effect callbacks.

The [`form sample`](../../samples/reactor-next/form/src/main.rs) exercises controlled input,
focus-first-invalid validation, a nested component, and scope-owned background submission.
The [`virtual task editor`](../../samples/reactor-next/virtual/readme.md) exercises keyed row
components, durable controlled edits, selection context, focus, effects, source reordering,
background loading, recycling, and a 1,000-row stress path.
The [`navigation and multi-window workspace`](../../samples/reactor-next/navigation/readme.md)
exercises retained page models, context propagation, independent window queues and references,
shared application updates, background cancellation, and peer cleanup.

The component store owns current props and passes them by reference to `Component::view`.
Components can render from that argument without copying props into their own fields.
`Component::changed` defaults to a no-op and is only needed for prop-driven state updates.

Effects use an explicit key separately from their typed dependency:

```rust,ignore
context.use_effect("subscription", topic.clone(), move || {
    let subscription = subscribe(topic);
    Some(Box::new(move || subscription.close()))
});
```

`EffectKey` is opaque and accepts `u32`, `u64`, `usize`, `String`, and `&str`. Keys are unique
within one component view. Conditional omission cleans the omitted effect without changing other
effects, and reordering calls with the same keys retains them. A changed dependency under one key
cleans the published effect before native mutation and runs its replacement setup after successful
publication. Duplicate keys reject candidate planning before cleanup, native mutation, or setup.
Owned components therefore have no positional hook-order contract.

Local component messages stay on the UI thread. Each window queues at most 4,096 messages.
`LocalSender::send` returns `false` when the owning scope has retired, the window has closed, or
the queue is full. `Callback::call` returns the same acceptance result for callbacks created by
`LocalSender::callback` or `LocalSender::message`.

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
