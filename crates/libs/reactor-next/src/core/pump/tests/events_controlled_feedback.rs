//! Queued event dispatch and controlled-feedback contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

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
    pump.queue_event(QueuedEvent {
        node: root,
        event: EventId::ButtonClick,
        revision,
        payload: EventPayload::Unit,
    });

    let second = Rc::new(Cell::new(0));
    let second_capture = Rc::clone(&second);
    let updated = pump
        .update(
            Button::new()
                .on_click(move || second_capture.set(second_capture.get() + 1))
                .into(),
        )
        .unwrap();

    assert!(updated.outcomes.is_empty());
    assert_eq!(
        pump.event_revision(root, EventId::ButtonClick),
        Some(revision)
    );
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(first.get(), 0);
    assert_eq!(second.get(), 1);
}

#[test]
fn removed_callback_rejects_queued_revision() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Button::new().on_click(|| {}).into()).unwrap();
    let root = pump.root().unwrap();
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    pump.queue_event(QueuedEvent {
        node: root,
        event: EventId::ButtonClick,
        revision,
        payload: EventPayload::Unit,
    });

    pump.update(Button::new().into()).unwrap();

    assert_eq!(pump.event_revision(root, EventId::ButtonClick), None);
    assert_eq!(pump.dispatch_events(), Ok(0));
}

#[test]
fn recovery_failure_poisons_and_discards_queued_events() {
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
    pump.runtime_mut().fail_at(0);
    pump.runtime_mut().fail_after(1, 0);
    assert!(matches!(
        pump.update(Button::new().into()),
        Err(PumpError::RecoveryFailed(_))
    ));
    pump.queue_event(QueuedEvent {
        node: root,
        event: EventId::ButtonClick,
        revision,
        payload: EventPayload::Unit,
    });

    assert_eq!(pump.dispatch_events(), Ok(0));
    assert_eq!(calls.get(), 0);
}

#[test]
fn recovered_root_rejects_pre_failure_event() {
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let element = || {
        Button::new()
            .on_click({
                let callback_calls = Rc::clone(&callback_calls);
                move || callback_calls.set(callback_calls.get() + 1)
            })
            .into()
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(element()).unwrap();
    let old_root = pump.root().unwrap();
    let revision = pump.event_revision(old_root, EventId::ButtonClick).unwrap();
    pump.queue_event(QueuedEvent {
        node: old_root,
        event: EventId::ButtonClick,
        revision,
        payload: EventPayload::Unit,
    });
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update(Button::new().into()),
        Err(PumpError::RecoveredStructure(_))
    ));
    assert_ne!(pump.root(), Some(old_root));
    assert_eq!(pump.dispatch_events(), Ok(0));
    assert_eq!(calls.get(), 0);
}

#[test]
fn native_remount_rejects_late_work_without_replacing_window_identity() {
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Button::new().on_click(|| {}).into()).unwrap();
    let old_identity = pump.native_identity();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update(
            Button::new()
                .content(TextBlock::new())
                .on_click(move || callback_calls.set(callback_calls.get() + 1))
                .into()
        ),
        Err(PumpError::RecoveredStructure(_))
    ));

    let new_identity = pump.native_identity();
    assert_eq!(old_identity.window(), new_identity.window());
    assert_ne!(
        old_identity.realization_epoch(),
        new_identity.realization_epoch()
    );
    let root = pump.root().unwrap();
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    pump.queue_event_with_identity(
        old_identity,
        QueuedEvent {
            node: root,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        },
    );

    assert_eq!(pump.dispatch_events(), Ok(0));
    assert_eq!(calls.get(), 0);
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
    let identity = pump.native_identity();
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
    let stale_identity = pump.native_identity();
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
fn old_realization_event_payload_read_failure_is_ignored() {
    let mut pump = Pump::new(EventErrorRuntime::default());
    pump.mount(TextBox::new().into()).unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::TextBoxTextChanged)
        .unwrap();
    let stale_identity = pump.native_identity();
    pump.identity = stale_identity.next_realization().unwrap();
    let identity = pump.identity;
    pump.runtime_mut().set_identity(identity);
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
    let identity = pump.native_identity();
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
            .child("button", Button::new().on_click(|| {}))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let button = pump.runtime().node(root).unwrap().children()[0];
    let revision = pump.event_revision(button, EventId::ButtonClick).unwrap();
    pump.queue_event(QueuedEvent {
        node: button,
        event: EventId::ButtonClick,
        revision,
        payload: EventPayload::Unit,
    });

    pump.update(StackPanel::new().into()).unwrap();

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

    pump.queue_event(QueuedEvent {
        node: root,
        event: EventId::TextBoxTextChanged,
        revision,
        payload: EventPayload::Str("updated".into()),
    });

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(&*value.borrow(), "updated");
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
        pump.queue_event(QueuedEvent {
            node: root,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        });
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
    pump.queue_event(QueuedEvent {
        node: root,
        event: EventId::TextBoxTextChanged,
        revision,
        payload: EventPayload::Str("native".into()),
    });

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(&*observed.borrow(), "native");
    assert!(pump.retry_pending());
    assert_eq!(
        pump.tree
            .native(root)
            .unwrap()
            .properties
            .get(&PropertyId::TextBoxText),
        Some(&NativePropertyState::Known(Some(PropertyValue::Str(
            "native".into()
        ))))
    );

    let restored = pump
        .update(
            TextBox::new()
                .text("desired")
                .on_text_changed(|_| {})
                .into(),
        )
        .unwrap();

    assert_eq!(restored.outcomes, [CommandOutcome::Applied]);
    assert!(!pump.retry_pending());
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
        type Props = ();

        fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
            Self
        }

        fn changed(&mut self, _props: &(), _context: &mut ComponentContext<Self>) {}

        fn update(&mut self, _message: String, _context: &mut ComponentContext<Self>) {}

        fn view(&self, context: &mut ViewContext<Self>) -> View {
            let sender = context.sender();
            View::native(
                TextBox::new()
                    .text("desired")
                    .on_text_changed(move |value| {
                        _ = sender.send(value);
                    }),
            )
        }
    }

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Controlled>(())).unwrap();
    let root = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
    let revision = pump
        .event_revision(root, EventId::TextBoxTextChanged)
        .unwrap();
    pump.queue_event(QueuedEvent {
        node: root,
        event: EventId::TextBoxTextChanged,
        revision,
        payload: EventPayload::Str("native".into()),
    });

    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBoxText),
        Some(&PropertyValue::Str("desired".into()))
    );
    assert!(!pump.retry_pending());
}
