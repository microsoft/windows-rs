//! Queued event dispatch and controlled-feedback contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

struct UnitEventComponent;

impl Component for UnitEventComponent {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        Button::new().on_click(context.message(())).into()
    }
}

#[test]
fn navigation_selection_observes_item_state_and_preserves_missing_tags() {
    let selected = Rc::new(RefCell::new(Vec::new()));
    let selected_capture = Rc::clone(&selected);
    let view = NavigationView::new()
        .on_selected_tag_changed(move |tag| selected_capture.borrow_mut().push(tag))
        .slots([SlotView::collection(
            NavigationViewSlot::MenuItems,
            [
                KeyedView::new("empty", NavigationViewItem::new().tag("").is_selected(true)),
                KeyedView::new(
                    "home",
                    NavigationViewItem::new().tag("home").is_selected(false),
                ),
            ],
        )]);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view.clone()).unwrap();
    let navigation = pump.root().unwrap();
    let revision = pump
        .event_revision(navigation, EventId::NavigationViewSelectionChanged)
        .unwrap();
    let items = pump
        .runtime()
        .node(navigation)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)
        .to_vec();

    pump.queue_event(QueuedEvent::new(
        navigation,
        EventId::NavigationViewSelectionChanged,
        revision,
        EventPayload::SelectionChange(SelectionChange {
            item: Some(items[1]),
            tag: Some("home".into()),
        }),
    ));
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(&*selected.borrow(), &[Some("home".into())]);
    assert_eq!(
        pump.tree
            .native(items[0])
            .unwrap()
            .properties
            .get(&PropertyId::NavigationViewItemIsSelected),
        Some(&Some(PropertyValue::Bool(false)))
    );
    assert_eq!(
        pump.tree
            .native(items[1])
            .unwrap()
            .properties
            .get(&PropertyId::NavigationViewItemIsSelected),
        Some(&Some(PropertyValue::Bool(true)))
    );

    let batches = pump.runtime().commands().len();
    pump.update_view(view).unwrap();
    let commands = pump.runtime().commands()[batches..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    assert_eq!(commands.len(), 2);
    assert!(commands.iter().all(|command| matches!(
        command,
        Command::SetProperty {
            property: PropertyId::NavigationViewItemIsSelected,
            ..
        }
    )));
    pump.queue_event(QueuedEvent::new(
        navigation,
        EventId::NavigationViewSelectionChanged,
        revision,
        EventPayload::SelectionChange(SelectionChange {
            item: None,
            tag: None,
        }),
    ));
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(&*selected.borrow(), &[Some("home".into()), None]);
    assert!(items.iter().all(|item| {
        pump.tree
            .native(*item)
            .unwrap()
            .properties
            .get(&PropertyId::NavigationViewItemIsSelected)
            == Some(&Some(PropertyValue::Bool(false)))
    }));

    let mut passive = Pump::new(RecordingRuntime::default());
    passive
        .mount_view(NavigationView::new().slots([SlotView::collection(
            NavigationViewSlot::MenuItems,
            [KeyedView::new(
                "home",
                NavigationViewItem::new().tag("home").is_selected(false),
            )],
        )]))
        .unwrap();
    let navigation = passive.root().unwrap();
    let item = passive
        .runtime()
        .node(navigation)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)[0];
    let revision = passive
        .event_revision(navigation, EventId::NavigationViewSelectionChanged)
        .unwrap();
    passive.queue_event(QueuedEvent::new(
        navigation,
        EventId::NavigationViewSelectionChanged,
        revision,
        EventPayload::SelectionChange(SelectionChange {
            item: Some(item),
            tag: Some("home".into()),
        }),
    ));
    assert_eq!(passive.dispatch_events(), Ok(0));
    assert_eq!(
        passive
            .tree
            .native(item)
            .unwrap()
            .properties
            .get(&PropertyId::NavigationViewItemIsSelected),
        Some(&Some(PropertyValue::Bool(true)))
    );

    let mut uncontrolled = Pump::new(RecordingRuntime::default());
    uncontrolled
        .mount_view(NavigationView::new().slots([SlotView::collection(
            NavigationViewSlot::MenuItems,
            [KeyedView::new(
                "native",
                NavigationViewItem::new().tag("native"),
            )],
        )]))
        .unwrap();
    let navigation = uncontrolled.root().unwrap();
    let item = uncontrolled
        .runtime()
        .node(navigation)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)[0];
    let revision = uncontrolled
        .event_revision(navigation, EventId::NavigationViewSelectionChanged)
        .unwrap();
    uncontrolled.queue_event(QueuedEvent::new(
        navigation,
        EventId::NavigationViewSelectionChanged,
        revision,
        EventPayload::SelectionChange(SelectionChange {
            item: Some(item),
            tag: Some("native".into()),
        }),
    ));
    assert_eq!(uncontrolled.dispatch_events(), Ok(0));
    assert!(
        !uncontrolled
            .tree
            .native(item)
            .unwrap()
            .properties
            .contains_key(&PropertyId::NavigationViewItemIsSelected)
    );

    struct Item;

    impl Component for Item {
        type Message = ();
        type Input = (&'static str, bool);

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            NavigationViewItem::new()
                .tag(input.0)
                .is_selected(input.1)
                .into()
        }
    }

    let mut nested = Pump::new(RecordingRuntime::default());
    nested
        .mount_view(
            NavigationView::new()
                .on_selected_tag_changed(|_| {})
                .slots([SlotView::collection(
                    NavigationViewSlot::MenuItems,
                    [
                        KeyedView::new("first", View::component::<Item>(("first", true))),
                        KeyedView::new("second", View::component::<Item>(("second", false))),
                    ],
                )]),
        )
        .unwrap();
    let navigation = nested.root().unwrap();
    let items = nested
        .runtime()
        .node(navigation)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)
        .to_vec();
    let revision = nested
        .event_revision(navigation, EventId::NavigationViewSelectionChanged)
        .unwrap();
    nested.queue_event(QueuedEvent::new(
        navigation,
        EventId::NavigationViewSelectionChanged,
        revision,
        EventPayload::SelectionChange(SelectionChange {
            item: Some(items[1]),
            tag: Some("second".into()),
        }),
    ));
    assert_eq!(nested.dispatch_events(), Ok(1));
    assert_eq!(nested.dispatch_components(10), Ok(0));
    assert_eq!(
        nested
            .tree
            .native(items[0])
            .unwrap()
            .properties
            .get(&PropertyId::NavigationViewItemIsSelected),
        Some(&Some(PropertyValue::Bool(true)))
    );
    assert_eq!(
        nested
            .tree
            .native(items[1])
            .unwrap()
            .properties
            .get(&PropertyId::NavigationViewItemIsSelected),
        Some(&Some(PropertyValue::Bool(false)))
    );
}

#[test]
fn queued_event_uses_latest_callback_without_revision_change() {
    let first = Rc::new(Cell::new(0));
    let first_capture = Rc::clone(&first);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Button::new()
            .on_click(move || first_capture.set(first_capture.get() + 1))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::ButtonClick,
        revision,
        EventPayload::Unit,
    ));

    let second = Rc::new(Cell::new(0));
    let second_capture = Rc::clone(&second);
    pump.update(
        Button::new()
            .on_click(move || second_capture.set(second_capture.get() + 1))
            .into(),
    )
    .unwrap();
    assert_eq!(
        pump.event_revision(root, EventId::ButtonClick),
        Some(revision)
    );
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(first.get(), 0);
    assert_eq!(second.get(), 1);
}

#[test]
fn grouped_callbacks_are_clone_isolated_and_keep_revisions() {
    let back = Rc::new(Cell::new(0));
    let back_capture = Rc::clone(&back);
    let original =
        TitleBar::new().on_back_requested(move || back_capture.set(back_capture.get() + 1));
    let pane = Rc::new(Cell::new(0));
    let pane_capture = Rc::clone(&pane);
    let extended = original
        .clone()
        .on_pane_toggle_requested(move || pane_capture.set(pane_capture.get() + 1));
    let mut pump = Pump::new(RecordingRuntime::default());

    pump.mount(original.into()).unwrap();
    let root = pump.root().unwrap();
    let back_revision = pump
        .event_revision(root, EventId::TitleBarBackRequested)
        .unwrap();
    assert_eq!(
        pump.event_revision(root, EventId::TitleBarPaneToggleRequested),
        None
    );
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TitleBarBackRequested,
        back_revision,
        EventPayload::Unit,
    ));

    pump.update(extended.into()).unwrap();
    assert_eq!(
        pump.event_revision(root, EventId::TitleBarBackRequested),
        Some(back_revision)
    );
    let pane_revision = pump
        .event_revision(root, EventId::TitleBarPaneToggleRequested)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TitleBarPaneToggleRequested,
        pane_revision,
        EventPayload::Unit,
    ));

    assert_eq!(pump.dispatch_events(), Ok(2));
    assert_eq!(back.get(), 1);
    assert_eq!(pane.get(), 1);
}

#[test]
fn observed_dependency_property_feedback_updates_known_state() {
    let observed = Rc::new(Cell::new(true));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        NavigationView::new()
            .is_pane_open(true)
            .on_is_pane_open_changed(move |value| capture.set(value))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::NavigationViewIsPaneOpenChanged)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::NavigationViewIsPaneOpenChanged,
        revision,
        EventPayload::Bool(false),
    ));
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert!(!observed.get());
    assert_eq!(
        pump.tree
            .native(root)
            .unwrap()
            .properties
            .get(&PropertyId::NavigationViewIsPaneOpen),
        Some(&Some(PropertyValue::Bool(false)))
    );

    let setter_count = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .filter(|command| {
            matches!(
                command,
                Command::SetProperty {
                    property: PropertyId::NavigationViewIsPaneOpen,
                    ..
                }
            )
        })
        .count();
    pump.update(NavigationView::new().is_pane_open(false).into())
        .unwrap();
    assert_eq!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .filter(|command| {
                matches!(
                    command,
                    Command::SetProperty {
                        property: PropertyId::NavigationViewIsPaneOpen,
                        ..
                    }
                )
            })
            .count(),
        setter_count
    );
}

#[test]
fn observed_controlled_property_subscribes_without_callback() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(NavigationView::new().is_pane_open(true).into())
        .unwrap();
    let root = pump.root().unwrap();

    assert!(
        pump.event_revision(root, EventId::NavigationViewIsPaneOpenChanged)
            .is_some()
    );
}

#[test]
fn removed_callback_rejects_queued_revision() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Button::new().on_click(|| {}).into()).unwrap();
    let root = pump.root().unwrap();
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::ButtonClick,
        revision,
        EventPayload::Unit,
    ));

    pump.update(Button::new().into()).unwrap();

    assert_eq!(pump.event_revision(root, EventId::ButtonClick), None);
    assert_eq!(pump.dispatch_events(), Ok(0));
}

#[test]
fn event_payload_read_failure_is_reported() {
    let mut pump = Pump::new(EventErrorRuntime::default());
    pump.mount(TextBox::new().on_text_changed(|_| {}).into())
        .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TextBoxTextChanged)
        .unwrap();
    let identity = pump.window_token();
    pump.runtime_mut().error = Some(NativeWork {
        identity,
        work: QueuedEventError {
            node: root,
            event: EventId::TextBoxTextChanged,
            revision,
            error: RuntimeError::Injected,
        },
    });

    assert_eq!(
        pump.dispatch_events(),
        Err(PumpError::EventReadFailed(RuntimeError::Injected))
    );
}

#[test]
fn old_window_event_payload_read_failure_is_ignored() {
    let mut pump = Pump::new(EventErrorRuntime::default());
    pump.mount(TextBox::new().into()).unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TextBoxTextChanged)
        .unwrap();
    let stale_identity = pump.window_token();
    pump.shutdown();
    pump.mount(TextBox::new().into()).unwrap();
    assert_eq!(pump.root(), Some(root));
    pump.runtime_mut().error = Some(NativeWork {
        identity: stale_identity,
        work: QueuedEventError {
            node: root,
            event: EventId::TextBoxTextChanged,
            revision,
            error: RuntimeError::Injected,
        },
    });

    assert_eq!(pump.dispatch_events(), Ok(0));
}

#[test]
fn retired_subscription_event_payload_read_failure_is_ignored() {
    let mut pump = Pump::new(EventErrorRuntime::default());
    pump.mount(Button::new().on_click(|| {}).into()).unwrap();
    let root = pump.root().unwrap();
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    let identity = pump.window_token();
    pump.update(Button::new().into()).unwrap();
    pump.runtime_mut().error = Some(NativeWork {
        identity,
        work: QueuedEventError {
            node: root,
            event: EventId::ButtonClick,
            revision,
            error: RuntimeError::Injected,
        },
    });

    assert_eq!(pump.dispatch_events(), Ok(0));
}

#[test]
fn retired_node_rejects_queued_event() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        StackPanel::new()
            .native_child("button", Button::new().on_click(|| {}))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let button = pump.runtime().node(root).unwrap().children()[0];
    let revision = pump.event_revision(button, EventId::ButtonClick).unwrap();
    pump.queue_event(QueuedEvent::new(
        button,
        EventId::ButtonClick,
        revision,
        EventPayload::Unit,
    ));

    pump.update(StackPanel::new().into()).unwrap();

    assert_eq!(pump.dispatch_events(), Ok(0));
}

#[test]
fn full_message_queue_defers_native_event_until_capacity_is_available() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<UnitEventComponent>(()))
        .unwrap();
    let component = pump.root().unwrap();
    let root = Pump::<RecordingRuntime>::native_root(&pump.tree, component).unwrap();
    let token = pump
        .components
        .token(pump.tree.component_scope(component).unwrap())
        .unwrap();
    let sender = pump.components.sender::<()>(token).unwrap();
    for _ in 0..component::LOCAL_MESSAGE_QUEUE_CAPACITY {
        assert!(sender.send(()));
    }
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::ButtonClick,
        revision,
        EventPayload::Unit,
    ));

    assert_eq!(pump.dispatch_events(), Ok(0));
    assert_eq!(pump.events.len(), 1);
    assert!(pump.native_work_pending());

    assert_eq!(
        pump.dispatch_components(component::LOCAL_MESSAGE_QUEUE_CAPACITY),
        Ok(component::LOCAL_MESSAGE_QUEUE_CAPACITY)
    );
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert!(pump.events.is_empty());
    assert_eq!(pump.dispatch_components(1), Ok(1));
}

#[test]
fn stale_message_callback_event_is_rejected_before_dispatch() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<UnitEventComponent>(()))
        .unwrap();
    let root = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::ButtonClick,
        revision,
        EventPayload::Unit,
    ));

    pump.update_view(View::native(TextBlock::new())).unwrap();

    assert_eq!(pump.dispatch_events(), Ok(0));
}

#[test]
fn generated_event_payload_reaches_callback() {
    let value = Rc::new(RefCell::new(String::new()));
    let capture = Rc::clone(&value);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        TextBox::new()
            .on_text_changed(move |text| *capture.borrow_mut() = text)
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TextBoxTextChanged)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TextBoxTextChanged,
        revision,
        EventPayload::Str("updated".into()),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(&*value.borrow(), "updated");
}

#[test]
fn rich_edit_box_document_text_and_feedback_are_owned() {
    let value = Rc::new(RefCell::new(String::new()));
    let capture = Rc::clone(&value);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        RichEditBox::new()
            .text("initial")
            .on_text_changed(move |text| *capture.borrow_mut() = text)
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::RichEditBoxDocument),
        Some(&PropertyValue::Str("initial".to_string()))
    );
    let revision = pump
        .event_revision(root, EventId::RichEditBoxTextChanged)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::RichEditBoxTextChanged,
        revision,
        EventPayload::Str("updated".into()),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(&*value.borrow(), "updated");
}

#[test]
fn navigation_display_mode_payload_reaches_owner() {
    let observed = Rc::new(Cell::new(NavigationViewDisplayMode::Expanded));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        NavigationView::new()
            .on_display_mode_changed(move |mode| capture.set(mode))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::NavigationViewDisplayModeChanged)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::NavigationViewDisplayModeChanged,
        revision,
        EventPayload::NavigationViewDisplayMode(NavigationViewDisplayMode::Compact),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(observed.get(), NavigationViewDisplayMode::Compact);
}

#[test]
fn event_work_budget_preserves_and_reports_pending_work() {
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Button::new()
            .on_click(move || callback_calls.set(callback_calls.get() + 1))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    for _ in 0..=EVENT_WORK_BUDGET {
        pump.queue_event(QueuedEvent::new(
            root,
            EventId::ButtonClick,
            revision,
            EventPayload::Unit,
        ));
    }

    assert_eq!(pump.dispatch_events(), Ok(EVENT_WORK_BUDGET));
    assert_eq!(calls.get(), EVENT_WORK_BUDGET);
    assert!(pump.native_work_pending());
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(calls.get(), EVENT_WORK_BUDGET + 1);
    assert!(!pump.native_work_pending());
}

#[test]
fn rejected_controlled_edit_restores_the_desired_value() {
    let observed = Rc::new(RefCell::new(String::new()));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        TextBox::new()
            .text("desired")
            .on_text_changed(move |text| *capture.borrow_mut() = text)
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TextBoxTextChanged)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TextBoxTextChanged,
        revision,
        EventPayload::Str("native".into()),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(&*observed.borrow(), "native");
    assert_eq!(
        pump.tree
            .native(root)
            .unwrap()
            .properties
            .get(&PropertyId::TextBoxText),
        Some(&Some(PropertyValue::Str("native".into())))
    );

    pump.update(
        TextBox::new()
            .text("desired")
            .on_text_changed(|_| {})
            .into(),
    )
    .unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBoxText),
        Some(&PropertyValue::Str("desired".into()))
    );
}

#[test]
fn component_rejected_controlled_edit_restores_the_desired_value() {
    struct Controlled;

    impl Component for Controlled {
        type Message = String;
        type Input = ();

        fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn input_changed(&mut self, _input: &(), _context: &ComponentContext<Self>) {}

        fn update(&mut self, _message: String, _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            View::native(
                TextBox::new()
                    .text("desired")
                    .on_text_changed(context.callback(std::convert::identity)),
            )
        }
    }

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Controlled>(())).unwrap();
    let root = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
    let revision = pump
        .event_revision(root, EventId::TextBoxTextChanged)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TextBoxTextChanged,
        revision,
        EventPayload::Str("native".into()),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBoxText),
        Some(&PropertyValue::Str("desired".into()))
    );
}

#[test]
fn number_box_orders_bounds_before_value_and_repairs_rejected_input() {
    let observed = Rc::new(Cell::new(0.0));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        NumberBox::new()
            .minimum(0.0)
            .maximum(10.0)
            .value(5.0)
            .on_value_changed(move |value| capture.set(value))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let commands = pump.runtime().commands().last().unwrap();
    let position = |property| {
        commands
            .iter()
            .position(|command| {
                matches!(
                    command,
                    Command::SetProperty {
                        property: current,
                        ..
                    } if *current == property
                )
            })
            .unwrap()
    };
    assert!(
        position(PropertyId::NumberBoxMinimum) < position(PropertyId::NumberBoxMaximum)
            && position(PropertyId::NumberBoxMaximum) < position(PropertyId::NumberBoxValue)
    );

    let revision = pump
        .event_revision(root, EventId::NumberBoxValueChanged)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::NumberBoxValueChanged,
        revision,
        EventPayload::F64(12.0),
    ));
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(observed.get(), 12.0);

    pump.update(
        NumberBox::new()
            .minimum(0.0)
            .maximum(10.0)
            .value(5.0)
            .on_value_changed(|_| {})
            .into(),
    )
    .unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::NumberBoxValue),
        Some(&PropertyValue::F64(5.0))
    );
}

#[test]
fn normalized_feedback_updates_known_state_without_invoking_the_callback() {
    let callbacks = Rc::new(Cell::new(0));
    let initial_capture = Rc::clone(&callbacks);
    let update_capture = Rc::clone(&callbacks);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        NumberBox::new()
            .minimum(0.0)
            .maximum(100.0)
            .value(50.0)
            .on_value_changed(move |_| initial_capture.set(initial_capture.get() + 1))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::NumberBoxValueChanged)
        .unwrap();

    pump.update(
        NumberBox::new()
            .minimum(0.0)
            .maximum(40.0)
            .value(50.0)
            .on_value_changed(move |_| update_capture.set(update_capture.get() + 1))
            .into(),
    )
    .unwrap();
    pump.queue_event(QueuedEvent::observation(
        root,
        EventId::NumberBoxValueChanged,
        revision,
        EventPayload::F64(40.0),
    ));

    assert_eq!(pump.dispatch_events(), Ok(0));
    assert_eq!(callbacks.get(), 0);
    assert!(!pump.native_work_pending());
    assert_eq!(
        pump.tree
            .native(root)
            .unwrap()
            .properties
            .get(&PropertyId::NumberBoxValue),
        Some(&Some(PropertyValue::F64(40.0)))
    );

    pump.update(
        NumberBox::new()
            .minimum(0.0)
            .maximum(100.0)
            .value(50.0)
            .into(),
    )
    .unwrap();
    let commands = pump.runtime().commands().last().unwrap();
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::SetProperty {
            property: PropertyId::NumberBoxValue,
            value: PropertyValue::F64(50.0),
            ..
        }
    )));
}

#[test]
fn number_box_nan_value_is_idempotent() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(NumberBox::new().value(f64::NAN).into()).unwrap();
    let batches = pump.runtime().batches();

    pump.update(NumberBox::new().value(f64::NAN).into())
        .unwrap();

    assert_eq!(pump.runtime().batches(), batches);
    assert_eq!(PropertyValue::F64(f64::NAN), PropertyValue::F64(f64::NAN));
    assert_eq!(EventPayload::F64(f64::NAN), EventPayload::F64(f64::NAN));
}

#[test]
fn slider_reuses_the_normalized_range_contract() {
    let callbacks = Rc::new(Cell::new(0));
    let initial_capture = Rc::clone(&callbacks);
    let update_capture = Rc::clone(&callbacks);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Slider::new()
            .minimum(0.0)
            .maximum(100.0)
            .value(50.0)
            .on_value_changed(move |_| initial_capture.set(initial_capture.get() + 1))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let commands = pump.runtime().commands().last().unwrap();
    let position = |property| {
        commands
            .iter()
            .position(|command| {
                matches!(
                    command,
                    Command::SetProperty {
                        property: current,
                        ..
                    } if *current == property
                )
            })
            .unwrap()
    };
    assert!(
        position(PropertyId::SliderMinimum) < position(PropertyId::SliderMaximum)
            && position(PropertyId::SliderMaximum) < position(PropertyId::SliderValue)
    );
    let revision = pump
        .event_revision(root, EventId::SliderValueChanged)
        .unwrap();

    pump.update(
        Slider::new()
            .minimum(0.0)
            .maximum(40.0)
            .value(50.0)
            .on_value_changed(move |_| update_capture.set(update_capture.get() + 1))
            .into(),
    )
    .unwrap();
    pump.queue_event(QueuedEvent::observation(
        root,
        EventId::SliderValueChanged,
        revision,
        EventPayload::F64(40.0),
    ));

    assert_eq!(pump.dispatch_events(), Ok(0));
    assert_eq!(callbacks.get(), 0);
    assert!(!pump.native_work_pending());

    pump.update(Slider::new().minimum(0.0).maximum(100.0).value(50.0).into())
        .unwrap();
    let commands = pump.runtime().commands().last().unwrap();
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::SetProperty {
            property: PropertyId::SliderValue,
            value: PropertyValue::F64(50.0),
            ..
        }
    )));
}

#[test]
fn slider_nan_value_is_idempotent() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Slider::new().value(f64::NAN).into()).unwrap();
    let batches = pump.runtime().batches();

    pump.update(Slider::new().value(f64::NAN).into()).unwrap();

    assert_eq!(pump.runtime().batches(), batches);
}

#[test]
fn rating_control_routes_normalized_value_feedback() {
    let observed = Rc::new(Cell::new(0.0));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        RatingControl::new()
            .max_rating(10)
            .value(4.0)
            .on_value_changed(move |value| capture.set(value))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::RatingControlValueChanged)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::RatingControlValueChanged,
        revision,
        EventPayload::F64(4.5),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(observed.get(), 4.5);
}

#[test]
fn combo_box_routes_selected_index_feedback() {
    let observed = Rc::new(Cell::new(-1));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ComboBox::new()
            .items_source(["Red", "Green", "Blue"])
            .selected_index(0)
            .on_selection_changed(move |index| capture.set(index))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::ComboBoxSelectionChanged)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::ComboBoxSelectionChanged,
        revision,
        EventPayload::I32(2),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(observed.get(), 2);
}

#[test]
fn radio_buttons_routes_selected_index_feedback() {
    let observed = Rc::new(Cell::new(-1));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        RadioButtons::new()
            .items_source(["Email", "SMS", "None"])
            .selected_index(0)
            .on_selection_changed(move |index| capture.set(index))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::RadioButtonsSelectionChanged)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::RadioButtonsSelectionChanged,
        revision,
        EventPayload::I32(2),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(observed.get(), 2);

    pump.update(
        RadioButtons::new()
            .items_source(["Email", "SMS", "None"])
            .selected_index(0)
            .into(),
    )
    .unwrap();
    assert!(
        pump.runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::SetProperty {
                    property: PropertyId::RadioButtonsSelectedIndex,
                    value: PropertyValue::I32(0),
                    ..
                }
            ))
    );
}

#[test]
fn split_view_routes_pane_closed_feedback() {
    let observed = Rc::new(Cell::new(true));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        SplitView::new()
            .is_pane_open(true)
            .on_pane_closed(move |open| capture.set(open))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::SplitViewPaneClosed)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::SplitViewPaneClosed,
        revision,
        EventPayload::Bool(false),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert!(!observed.get());
}

#[test]
fn auto_suggest_routes_text_and_chosen_item_feedback() {
    let text = Rc::new(RefCell::new(String::new()));
    let chosen = Rc::new(RefCell::new(String::new()));
    let text_capture = Rc::clone(&text);
    let chosen_capture = Rc::clone(&chosen);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        AutoSuggestBox::new()
            .text("")
            .items_source(["Apple", "Apricot"])
            .on_text_changed(move |value| *text_capture.borrow_mut() = value)
            .on_suggestion_chosen(move |value| *chosen_capture.borrow_mut() = value)
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let text_revision = pump
        .event_revision(root, EventId::AutoSuggestBoxTextChanged)
        .unwrap();
    let chosen_revision = pump
        .event_revision(root, EventId::AutoSuggestBoxSuggestionChosen)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::AutoSuggestBoxTextChanged,
        text_revision,
        EventPayload::Str("ap".into()),
    ));
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::AutoSuggestBoxSuggestionChosen,
        chosen_revision,
        EventPayload::Str("Apricot".into()),
    ));

    assert_eq!(pump.dispatch_events(), Ok(2));
    assert_eq!(&*text.borrow(), "ap");
    assert_eq!(&*chosen.borrow(), "Apricot");
}

#[test]
fn optional_controlled_values_clear_and_remain_idempotent() {
    let mut navigation = Pump::new(RecordingRuntime::default());
    navigation
        .mount(NavigationView::new().is_pane_open(false).into())
        .unwrap();
    navigation
        .update(NavigationView::new().is_pane_open(None).into())
        .unwrap();
    assert!(
        navigation
            .runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::ClearProperty {
                    property: PropertyId::NavigationViewIsPaneOpen,
                    ..
                }
            ))
    );
    assert!(matches!(
        expected_feedback(PropertyId::NavigationViewIsPaneOpen, None),
        Some((
            EventId::NavigationViewIsPaneOpenChanged,
            FeedbackExpectation::Exact(EventPayload::Bool(true))
        ))
    ));
    let batches = navigation.runtime().batches();
    navigation
        .update(NavigationView::new().is_pane_open(None).into())
        .unwrap();
    assert_eq!(navigation.runtime().batches(), batches);

    let mut text_box = Pump::new(RecordingRuntime::default());
    text_box.mount(TextBox::new().text("value").into()).unwrap();
    text_box
        .update(TextBox::new().text_optional(None::<String>).into())
        .unwrap();
    assert!(
        text_box
            .runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::ClearProperty {
                    property: PropertyId::TextBoxText,
                    ..
                }
            ))
    );
    let batches = text_box.runtime().batches();
    text_box
        .update(TextBox::new().text_optional(None::<String>).into())
        .unwrap();
    assert_eq!(text_box.runtime().batches(), batches);

    let mut number_box = Pump::new(RecordingRuntime::default());
    number_box
        .mount(NumberBox::new().value(5.0).into())
        .unwrap();
    number_box
        .update(NumberBox::new().value(None).into())
        .unwrap();
    assert!(
        number_box
            .runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::ClearProperty {
                    property: PropertyId::NumberBoxValue,
                    ..
                }
            ))
    );
    let batches = number_box.runtime().batches();
    number_box
        .update(NumberBox::new().value(None).into())
        .unwrap();
    assert_eq!(number_box.runtime().batches(), batches);

    let mut slider = Pump::new(RecordingRuntime::default());
    slider.mount(Slider::new().value(5.0).into()).unwrap();
    slider.update(Slider::new().value(None).into()).unwrap();
    assert!(
        slider
            .runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::ClearProperty {
                    property: PropertyId::SliderValue,
                    ..
                }
            ))
    );
    let batches = slider.runtime().batches();
    slider.update(Slider::new().value(None).into()).unwrap();
    assert_eq!(slider.runtime().batches(), batches);

    let mut toggle = Pump::new(RecordingRuntime::default());
    toggle
        .mount(ToggleSwitch::new().is_on(true).into())
        .unwrap();
    toggle
        .update(ToggleSwitch::new().is_on(None).into())
        .unwrap();
    assert!(
        toggle
            .runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::ClearProperty {
                    property: PropertyId::ToggleSwitchIsOn,
                    ..
                }
            ))
    );
    let batches = toggle.runtime().batches();
    toggle
        .update(ToggleSwitch::new().is_on(None).into())
        .unwrap();
    assert_eq!(toggle.runtime().batches(), batches);
}

#[test]
fn toggle_switch_routes_bool_feedback_and_restores_desired_state() {
    let observed = Rc::new(Cell::new(false));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ToggleSwitch::new()
            .is_on(false)
            .is_enabled(true)
            .on_toggled(move |value| capture.set(value))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let commands = &pump.runtime().commands()[0];
    let position = |property| {
        commands
            .iter()
            .position(|command| {
                matches!(
                    command,
                    Command::SetProperty {
                        property: current,
                        ..
                    } if *current == property
                )
            })
            .unwrap()
    };
    assert!(position(PropertyId::ToggleSwitchIsOn) < position(PropertyId::ToggleSwitchIsEnabled));

    let revision = pump
        .event_revision(root, EventId::ToggleSwitchToggled)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::ToggleSwitchToggled,
        revision,
        EventPayload::Bool(true),
    ));
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert!(observed.get());
    assert_eq!(
        pump.tree
            .native(root)
            .unwrap()
            .properties
            .get(&PropertyId::ToggleSwitchIsOn),
        Some(&Some(PropertyValue::Bool(true)))
    );

    pump.update(
        ToggleSwitch::new()
            .is_on(false)
            .is_enabled(true)
            .on_toggled(|_| {})
            .into(),
    )
    .unwrap();
    assert!(
        pump.runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::SetProperty {
                    property: PropertyId::ToggleSwitchIsOn,
                    value: PropertyValue::Bool(false),
                    ..
                }
            ))
    );
}

#[test]
fn tab_view_routes_selected_index_feedback() {
    let observed = Rc::new(Cell::new(-1));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TabView::new()
            .selected_index(0)
            .on_selection_changed(move |index| capture.set(index))
            .slots([SlotView::collection(
                TabViewSlot::TabItems,
                [
                    KeyedView::new("a", TabViewItem::new().header("A").tag("a")),
                    KeyedView::new("b", TabViewItem::new().header("B").tag("b")),
                ],
            )]),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TabViewSelectionChanged)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TabViewSelectionChanged,
        revision,
        EventPayload::I32(1),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(observed.get(), 1);
}

#[test]
fn tab_view_close_requested_routes_key_string() {
    let observed = Rc::new(RefCell::new(String::new()));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TabView::new()
            .on_close_requested(move |key: String| *capture.borrow_mut() = key)
            .slots([SlotView::collection(
                TabViewSlot::TabItems,
                [
                    KeyedView::new("first", TabViewItem::new().header("First").tag("first")),
                    KeyedView::new("second", TabViewItem::new().header("Second").tag("second")),
                ],
            )]),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TabViewTabCloseRequested)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TabViewTabCloseRequested,
        revision,
        EventPayload::Str("second".into()),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(*observed.borrow(), "second");
}

#[test]
fn tab_view_add_button_click_routes_unit() {
    let observed = Rc::new(Cell::new(false));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TabView::new()
            .is_add_tab_button_visible(true)
            .on_add_tab_button_click(move || capture.set(true))
            .slots([SlotView::collection(
                TabViewSlot::TabItems,
                [KeyedView::new("tab", TabViewItem::new().header("Tab"))],
            )]),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TabViewAddTabButtonClick)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TabViewAddTabButtonClick,
        revision,
        EventPayload::Unit,
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert!(observed.get());
}

#[test]
fn tab_view_reorder_routes_item_tags() {
    let observed = Rc::new(RefCell::new(Vec::new()));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TabView::new()
            .on_reordered(move |order: Rc<Vec<String>>| {
                *capture.borrow_mut() = order.as_ref().clone();
            })
            .slots([SlotView::collection(
                TabViewSlot::TabItems,
                [
                    KeyedView::new("first", TabViewItem::new().header("First").tag("first")),
                    KeyedView::new("second", TabViewItem::new().header("Second").tag("second")),
                ],
            )]),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TabViewTabItemsChanged)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TabViewTabItemsChanged,
        revision,
        EventPayload::StrList(Rc::new(vec!["second".into(), "first".into()])),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(*observed.borrow(), ["second", "first"]);
}

#[test]
fn date_and_time_pickers_route_typed_values() {
    let date = DateTime {
        universal_time: 123,
    };
    let time = TimeSpan { duration: 456 };
    let selected_date = Rc::new(Cell::new(DateTime::default()));
    let selected_time = Rc::new(Cell::new(TimeSpan::default()));
    let calendar_date = Rc::new(Cell::new(DateTime::default()));

    let date_capture = Rc::clone(&selected_date);
    let mut date_pump = Pump::new(RecordingRuntime::default());
    date_pump
        .mount_view(
            DatePicker::new()
                .on_selected_date_changed(move |value| date_capture.set(value))
                .into(),
        )
        .unwrap();
    let root = date_pump.root().unwrap();
    let revision = date_pump
        .event_revision(root, EventId::DatePickerSelectedDateChanged)
        .unwrap();
    date_pump.queue_event(QueuedEvent::new(
        root,
        EventId::DatePickerSelectedDateChanged,
        revision,
        EventPayload::DateTime(date),
    ));
    assert_eq!(date_pump.dispatch_events(), Ok(1));
    assert_eq!(selected_date.get(), date);

    let time_capture = Rc::clone(&selected_time);
    let mut time_pump = Pump::new(RecordingRuntime::default());
    time_pump
        .mount_view(
            TimePicker::new()
                .on_selected_time_changed(move |value| time_capture.set(value))
                .into(),
        )
        .unwrap();
    let root = time_pump.root().unwrap();
    let revision = time_pump
        .event_revision(root, EventId::TimePickerSelectedTimeChanged)
        .unwrap();
    time_pump.queue_event(QueuedEvent::new(
        root,
        EventId::TimePickerSelectedTimeChanged,
        revision,
        EventPayload::TimeSpan(time),
    ));
    assert_eq!(time_pump.dispatch_events(), Ok(1));
    assert_eq!(selected_time.get(), time);

    let calendar_capture = Rc::clone(&calendar_date);
    let mut calendar_pump = Pump::new(RecordingRuntime::default());
    calendar_pump
        .mount_view(
            CalendarDatePicker::new()
                .on_date_changed(move |value| calendar_capture.set(value))
                .into(),
        )
        .unwrap();
    let root = calendar_pump.root().unwrap();
    let revision = calendar_pump
        .event_revision(root, EventId::CalendarDatePickerDateChanged)
        .unwrap();
    calendar_pump.queue_event(QueuedEvent::new(
        root,
        EventId::CalendarDatePickerDateChanged,
        revision,
        EventPayload::DateTime(date),
    ));
    assert_eq!(calendar_pump.dispatch_events(), Ok(1));
    assert_eq!(calendar_date.get(), date);
}

#[test]
fn color_picker_routes_controlled_argb_feedback() {
    let initial = Color::argb(255, 0, 120, 215);
    let changed = Color::argb(128, 10, 20, 30);
    let observed = Rc::new(Cell::new(Color::default()));
    let capture = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ColorPicker::new()
            .color(initial)
            .on_color_changed(move |value| capture.set(value))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::ColorPickerColorChanged)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::ColorPickerColorChanged,
        revision,
        EventPayload::Color(changed),
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(observed.get(), changed);
    assert_eq!(
        pump.tree
            .native(root)
            .unwrap()
            .properties
            .get(&PropertyId::ColorPickerColor),
        Some(&Some(PropertyValue::Color(changed)))
    );
}

#[test]
fn teaching_tip_routes_action_and_closed_events() {
    let action_count = Rc::new(Cell::new(0));
    let action_capture = Rc::clone(&action_count);
    let closed_count = Rc::new(Cell::new(0));
    let closed_capture = Rc::clone(&closed_count);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TeachingTip::new()
            .title("Welcome")
            .is_open(true)
            .action_button_content("Continue")
            .close_button_content("Dismiss")
            .on_action_button_click(move || action_capture.set(action_capture.get() + 1))
            .on_closed(move || closed_capture.set(closed_capture.get() + 1))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let action_revision = pump
        .event_revision(root, EventId::TeachingTipActionButtonClick)
        .unwrap();
    let closed_revision = pump
        .event_revision(root, EventId::TeachingTipClosed)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TeachingTipActionButtonClick,
        action_revision,
        EventPayload::Unit,
    ));
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::TeachingTipClosed,
        closed_revision,
        EventPayload::Unit,
    ));

    assert_eq!(pump.dispatch_events(), Ok(2));
    assert_eq!(action_count.get(), 1);
    assert_eq!(closed_count.get(), 1);
}

#[test]
fn calendar_view_routes_selected_dates_changed() {
    let changes = Rc::new(Cell::new(0));
    let capture = Rc::clone(&changes);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        CalendarView::new()
            .is_today_highlighted(true)
            .is_group_label_visible(true)
            .on_selected_dates_changed(move || capture.set(capture.get() + 1))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::CalendarViewSelectedDatesChanged)
        .unwrap();

    pump.queue_event(QueuedEvent::new(
        root,
        EventId::CalendarViewSelectedDatesChanged,
        revision,
        EventPayload::Unit,
    ));

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(changes.get(), 1);
}

#[test]
fn list_and_grid_views_route_selection_and_reordered_tags() {
    let selected = Rc::new(Cell::new(-1));
    let selected_capture = Rc::clone(&selected);
    let reordered = Rc::new(RefCell::new(Vec::new()));
    let reordered_capture = Rc::clone(&reordered);
    let mut list = Pump::new(RecordingRuntime::default());
    list.mount_view(
        ListView::new()
            .selected_index(-1)
            .on_selection_changed(move |index| selected_capture.set(index))
            .on_reordered(move |items: Rc<Vec<String>>| {
                *reordered_capture.borrow_mut() = items.as_ref().clone();
            })
            .slots([SlotView::collection(
                ListViewSlot::Items,
                [
                    KeyedView::new("a", ListViewItem::new().tag("a")),
                    KeyedView::new("b", ListViewItem::new().tag("b")),
                ],
            )]),
    )
    .unwrap();
    let root = list.root().unwrap();
    let selection_revision = list
        .event_revision(root, EventId::ListViewSelectionChanged)
        .unwrap();
    let reorder_revision = list
        .event_revision(root, EventId::ListViewDragItemsCompleted)
        .unwrap();
    list.queue_event(QueuedEvent::new(
        root,
        EventId::ListViewSelectionChanged,
        selection_revision,
        EventPayload::I32(1),
    ));
    list.queue_event(QueuedEvent::new(
        root,
        EventId::ListViewDragItemsCompleted,
        reorder_revision,
        EventPayload::StrList(Rc::new(vec!["b".to_string(), "a".to_string()])),
    ));
    assert_eq!(list.dispatch_events(), Ok(2));
    assert_eq!(selected.get(), 1);
    assert_eq!(&*reordered.borrow(), &["b", "a"]);

    let grid_order = Rc::new(RefCell::new(Vec::new()));
    let capture = Rc::clone(&grid_order);
    let mut grid = Pump::new(RecordingRuntime::default());
    grid.mount_view(
        GridView::new()
            .on_reordered(move |items: Rc<Vec<String>>| {
                *capture.borrow_mut() = items.as_ref().clone();
            })
            .slots([SlotView::collection(
                GridViewSlot::Items,
                [
                    KeyedView::new("x", GridViewItem::new().tag("x")),
                    KeyedView::new("y", GridViewItem::new().tag("y")),
                ],
            )]),
    )
    .unwrap();
    let root = grid.root().unwrap();
    let revision = grid
        .event_revision(root, EventId::GridViewDragItemsCompleted)
        .unwrap();
    grid.queue_event(QueuedEvent::new(
        root,
        EventId::GridViewDragItemsCompleted,
        revision,
        EventPayload::StrList(Rc::new(vec!["y".to_string(), "x".to_string()])),
    ));
    assert_eq!(grid.dispatch_events(), Ok(1));
    assert_eq!(&*grid_order.borrow(), &["y", "x"]);
}

#[test]
fn drop_down_and_split_buttons_route_clicks() {
    let drop_down_clicks = Rc::new(Cell::new(0));
    let capture = Rc::clone(&drop_down_clicks);
    let mut drop_down = Pump::new(RecordingRuntime::default());
    drop_down
        .mount_view(
            DropDownButton::new()
                .on_click(move || capture.set(capture.get() + 1))
                .content(TextBlock::new().text("Options")),
        )
        .unwrap();
    let root = drop_down.root().unwrap();
    let revision = drop_down
        .event_revision(root, EventId::DropDownButtonClick)
        .unwrap();
    drop_down.queue_event(QueuedEvent::new(
        root,
        EventId::DropDownButtonClick,
        revision,
        EventPayload::Unit,
    ));
    assert_eq!(drop_down.dispatch_events(), Ok(1));
    assert_eq!(drop_down_clicks.get(), 1);

    let split_clicks = Rc::new(Cell::new(0));
    let capture = Rc::clone(&split_clicks);
    let mut split = Pump::new(RecordingRuntime::default());
    split
        .mount_view(
            SplitButton::new()
                .on_click(move || capture.set(capture.get() + 1))
                .content(TextBlock::new().text("Primary")),
        )
        .unwrap();
    let root = split.root().unwrap();
    let revision = split
        .event_revision(root, EventId::SplitButtonClick)
        .unwrap();
    split.queue_event(QueuedEvent::new(
        root,
        EventId::SplitButtonClick,
        revision,
        EventPayload::Unit,
    ));
    assert_eq!(split.dispatch_events(), Ok(1));
    assert_eq!(split_clicks.get(), 1);
}

#[test]
fn tab_view_collection_preserves_identity_through_reorder() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TabView::new()
            .selected_index(0)
            .slots([SlotView::collection(
                TabViewSlot::TabItems,
                [
                    KeyedView::new("a", TabViewItem::new().header("A").tag("a")),
                    KeyedView::new("b", TabViewItem::new().header("B").tag("b")),
                    KeyedView::new("c", TabViewItem::new().header("C").tag("c")),
                ],
            )]),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let initial_children: Vec<_> = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::TabViewTabItems)
        .to_vec();
    assert_eq!(initial_children.len(), 3);

    pump.update_view(
        TabView::new()
            .selected_index(0)
            .slots([SlotView::collection(
                TabViewSlot::TabItems,
                [
                    KeyedView::new("c", TabViewItem::new().header("C").tag("c")),
                    KeyedView::new("a", TabViewItem::new().header("A").tag("a")),
                    KeyedView::new("b", TabViewItem::new().header("B").tag("b")),
                ],
            )]),
    )
    .unwrap();
    let reordered_children: Vec<_> = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::TabViewTabItems)
        .to_vec();
    assert_eq!(reordered_children.len(), 3);
    assert_eq!(reordered_children[0], initial_children[2]);
    assert_eq!(reordered_children[1], initial_children[0]);
    assert_eq!(reordered_children[2], initial_children[1]);
}
