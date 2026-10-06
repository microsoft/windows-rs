use super::*;

#[test]
fn pointer_event_payload_round_trips_through_recording_protocol() {
    let received = Rc::new(RefCell::new(None));
    let received_for_callback = Rc::clone(&received);
    let callback = Callback::new(move |value| {
        *received_for_callback.borrow_mut() = Some(value);
    });
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().on_pointer_released(callback.clone()))
        .unwrap();
    let border = runtime.graph().root().unwrap();
    let payload = PointerEventInfo {
        x: 12.5,
        y: 24.5,
        window_x: 112.5,
        window_y: 224.5,
        pointer_id: 42,
        modifiers: InputModifiers::CONTROL,
        capture_succeeded: None,
        is_captured: true,
        is_left_button_pressed: false,
        is_right_button_pressed: true,
        is_middle_button_pressed: false,
    };
    runtime.adapter_mut().queue_event(EventDispatch::new(
        border,
        EventId::PointerReleased,
        EventValue::PointerEventInfo(callback),
        EventPayload::PointerEventInfo(payload),
    ));

    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);

    assert_eq!(*received.borrow(), Some(payload));
    assert!(
        event_contracts(ObjectType::Border).contains(&EventContract {
            id: EventId::PointerReleased,
            value: ValueType::PointerEventInfo,
        })
    );
}

#[test]
fn pointer_policy_records_set_and_clear() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .capture_pointer_on_press(true)
                .focus_on_pointer_release(true),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    assert!(
        runtime
            .graph()
            .properties(border)
            .unwrap()
            .contains(&Property {
                id: PropertyId::CapturePointerOnPress,
                value: PropertyValue::Bool(true),
            })
    );
    assert!(
        runtime
            .graph()
            .properties(border)
            .unwrap()
            .contains(&Property {
                id: PropertyId::FocusOnPointerRelease,
                value: PropertyValue::Bool(true),
            })
    );

    let mutations = runtime.update(Border::new()).unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, set, clear }
            if *object == border
                && set.is_empty()
                && clear.as_ref()
                    == [
                        PropertyId::CapturePointerOnPress,
                        PropertyId::FocusOnPointerRelease
                    ]
    )));
}

#[test]
fn routed_keyboard_payloads_preserve_the_handled_result() {
    let key = KeyEventInfo {
        key: VirtualKey::ENTER,
        original_key: VirtualKey::ENTER,
        status: PhysicalKeyStatus::default(),
        modifiers: InputModifiers::CONTROL,
    };
    let character = CharacterEventInfo {
        character: b'A' as u16,
        status: PhysicalKeyStatus::default(),
        modifiers: InputModifiers::SHIFT,
    };
    let key_callback = RoutedCallback::new(move |value| value == key);
    let character_callback = RoutedCallback::new(move |value| value == character);
    assert!(key_callback.call(key));
    assert!(character_callback.call(character));

    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .on_preview_key_down(key_callback.clone())
                .on_character_received(character_callback.clone()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        border,
        EventId::PreviewKeyDown,
        EventValue::KeyEventInfo(key_callback),
        EventPayload::KeyEventInfo(key),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        border,
        EventId::CharacterReceived,
        EventValue::CharacterEventInfo(character_callback),
        EventPayload::CharacterEventInfo(character),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
}

#[test]
fn focus_payloads_round_trip_through_recording_protocol() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let got = FocusEventInfo {
        state: ElementFocusState::Keyboard,
        is_direct: true,
    };
    let lost = FocusEventInfo {
        state: ElementFocusState::Unfocused,
        is_direct: false,
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .on_got_focus(callback.clone())
                .on_lost_focus(callback.clone()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    for (event, value) in [(EventId::GotFocus, got), (EventId::LostFocus, lost)] {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            border,
            event,
            EventValue::FocusEventInfo(callback.clone()),
            EventPayload::FocusEventInfo(value),
        ));
    }
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*values.borrow(), &[got, lost]);
}

#[test]
fn drag_drop_policy_and_payloads_round_trip_through_recording_protocol() {
    let policy = DragDropPolicy::new()
        .storage_items(DragDropAction::new(DragDropOperation::Move))
        .text(DragDropAction::new(DragDropOperation::Copy).caption("Copy text"));
    let drag_values = Rc::new(RefCell::new(Vec::new()));
    let dropped_values = Rc::new(RefCell::new(Vec::new()));
    let drag_callback = {
        let values = Rc::clone(&drag_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let drop_callback = {
        let values = Rc::clone(&dropped_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .drop_policy(policy.clone())
                .on_drag_enter(drag_callback.clone())
                .on_drag_over(drag_callback.clone())
                .on_drop(drop_callback.clone()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    assert!(
        runtime
            .graph()
            .properties(border)
            .unwrap()
            .contains(&Property {
                id: PropertyId::DropPolicy,
                value: PropertyValue::DragDropPolicy(Rc::new(policy)),
            })
    );
    for (event, value) in [
        (EventId::DragEnter, DragKind::Text),
        (EventId::DragOver, DragKind::StorageItems),
    ] {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            border,
            event,
            EventValue::DragKind(drag_callback.clone()),
            EventPayload::DragKind(value),
        ));
    }
    let dropped = DroppedData::Text("value".to_string());
    runtime.adapter_mut().queue_event(EventDispatch::new(
        border,
        EventId::Drop,
        EventValue::DroppedData(drop_callback),
        EventPayload::DroppedData(dropped.clone()),
    ));

    assert_eq!(runtime.dispatch_native_events().unwrap(), 3);
    assert_eq!(
        &*drag_values.borrow(),
        &[DragKind::Text, DragKind::StorageItems]
    );
    assert_eq!(&*dropped_values.borrow(), &[dropped]);

    let mutations = runtime.update(Border::new()).unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, set, clear }
            if *object == border
                && set.is_empty()
                && clear.as_ref() == [PropertyId::DropPolicy]
    )));
}

#[test]
fn pointer_motion_events_use_the_shared_typed_payload() {
    let received = Rc::new(RefCell::new(Vec::new()));
    let callback = |event| {
        let received = Rc::clone(&received);
        Callback::new(move |_: PointerEventInfo| received.borrow_mut().push(event))
    };
    let moved = callback(EventId::PointerMoved);
    let entered = callback(EventId::PointerEntered);
    let exited = callback(EventId::PointerExited);
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .on_pointer_moved(moved.clone())
                .on_pointer_entered(entered.clone())
                .on_pointer_exited(exited.clone()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    for (event, callback) in [
        (EventId::PointerMoved, moved),
        (EventId::PointerEntered, entered),
        (EventId::PointerExited, exited),
    ] {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            border,
            event,
            EventValue::PointerEventInfo(callback),
            EventPayload::PointerEventInfo(PointerEventInfo::default()),
        ));
    }

    assert_eq!(runtime.dispatch_native_events().unwrap(), 3);
    assert_eq!(
        &*received.borrow(),
        &[
            EventId::PointerMoved,
            EventId::PointerEntered,
            EventId::PointerExited
        ]
    );
}

#[test]
fn stale_pointer_event_does_not_reach_replacement_callback() {
    let first_count = Rc::new(Cell::new(0));
    let first_count_for_callback = Rc::clone(&first_count);
    let first = Callback::new(move |_: PointerEventInfo| {
        first_count_for_callback.set(first_count_for_callback.get() + 1);
    });
    let second_count = Rc::new(Cell::new(0));
    let second_count_for_callback = Rc::clone(&second_count);
    let second = Callback::new(move |_: PointerEventInfo| {
        second_count_for_callback.set(second_count_for_callback.get() + 1);
    });
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().on_pointer_released(first.clone()))
        .unwrap();
    let border = runtime.graph().root().unwrap();
    for _ in 0..2 {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            border,
            EventId::PointerReleased,
            EventValue::PointerEventInfo(first.clone()),
            EventPayload::PointerEventInfo(PointerEventInfo::default()),
        ));
    }
    let mut active = runtime.next_native_event().unwrap().unwrap();
    active.invoke();
    runtime
        .update(Border::new().on_pointer_released(second))
        .unwrap();
    drop(active);

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(first_count.get(), 1);
    assert_eq!(second_count.get(), 0);
}
