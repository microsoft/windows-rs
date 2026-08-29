use super::super::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn pointer_payloads_route_coordinates_and_button_state() {
    let observed = Rc::new(RefCell::new(Vec::new()));
    let pressed = Rc::clone(&observed);
    let moved = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Border::new()
            .on_pointer_pressed(move |info| pressed.borrow_mut().push(("pressed", info)))
            .on_pointer_moved(move |info| moved.borrow_mut().push(("moved", info)))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let pressed_revision = pump
        .event_revision(root, EventId::BorderPointerPressed)
        .unwrap();
    let moved_revision = pump
        .event_revision(root, EventId::BorderPointerMoved)
        .unwrap();
    let info = PointerEventInfo {
        x: 12.0,
        y: 24.0,
        window_x: 112.0,
        window_y: 224.0,
        is_left_button_pressed: true,
        ..PointerEventInfo::default()
    };
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::BorderPointerPressed,
        pressed_revision,
        EventPayload::PointerEventInfo(info),
    ));
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::BorderPointerMoved,
        moved_revision,
        EventPayload::PointerEventInfo(info),
    ));

    assert_eq!(pump.dispatch_events(), Ok(2));
    assert_eq!(&*observed.borrow(), &[("pressed", info), ("moved", info)]);
}

#[test]
fn removing_pointer_callback_retires_its_revision() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Border::new().on_pointer_entered(|_| {}).into())
        .unwrap();
    let root = pump.root().unwrap();
    let revision = pump
        .event_revision(root, EventId::BorderPointerEntered)
        .unwrap();

    pump.update(Border::new().into()).unwrap();
    pump.queue_event(QueuedEvent::new(
        root,
        EventId::BorderPointerEntered,
        revision,
        EventPayload::PointerEventInfo(PointerEventInfo::default()),
    ));

    assert_eq!(pump.dispatch_events(), Ok(0));
}

#[test]
fn pointer_capture_policy_and_completion_callbacks_reconcile() {
    let completed = Rc::new(RefCell::new(Vec::new()));
    let released = Rc::clone(&completed);
    let lost = Rc::clone(&completed);
    let canceled = Rc::clone(&completed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Border::new()
            .capture_pointer_on_press(true)
            .on_pointer_released(move |_| released.borrow_mut().push("released"))
            .on_pointer_capture_lost(move || lost.borrow_mut().push("lost"))
            .on_pointer_canceled(move || canceled.borrow_mut().push("canceled"))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    assert!(
        pump.event_revision(root, EventId::BorderPointerPressed)
            .is_some()
    );
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::BorderCapturePointerOnPress),
        Some(&PropertyValue::Bool(true))
    );

    for (event, payload) in [
        (
            EventId::BorderPointerReleased,
            EventPayload::PointerEventInfo(PointerEventInfo::default()),
        ),
        (EventId::BorderPointerCaptureLost, EventPayload::Unit),
        (EventId::BorderPointerCanceled, EventPayload::Unit),
    ] {
        let revision = pump.event_revision(root, event).unwrap();
        pump.queue_event(QueuedEvent::new(root, event, revision, payload));
    }

    assert_eq!(pump.dispatch_events(), Ok(3));
    assert_eq!(&*completed.borrow(), &["released", "lost", "canceled"]);

    pump.update(Border::new().into()).unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::BorderCapturePointerOnPress),
        None
    );
}

#[test]
fn drop_policy_activates_drag_negotiation_without_drag_callbacks() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Border::new()
            .drop_policy(DragDropPolicy::new().text(DragDropAction::new(DragDropOperation::Copy)))
            .on_drop(|_| {})
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();

    assert!(
        pump.event_revision(root, EventId::BorderDragEnter)
            .is_some()
    );
    assert!(pump.event_revision(root, EventId::BorderDragOver).is_some());
    assert!(pump.event_revision(root, EventId::BorderDrop).is_some());
}
