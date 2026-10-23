use super::*;

#[test]
fn image_load_subscriptions_complete_once_and_do_not_retain_controls() {
    let state = ImageLoadState::default();
    let values = Rc::new(RefCell::new(Vec::new()));
    let result = Rc::clone(&values);
    let callback: Rc<dyn Fn(bool)> = Rc::new(move |value| result.borrow_mut().push(value));
    let removed: Rc<dyn Fn(bool)> = Rc::new(|_| panic!("removed subscription"));
    assert_eq!(state.subscribe(&removed), None);
    drop(removed);
    assert_eq!(state.subscribe(&callback), None);
    state.complete(true);
    state.complete(false);
    assert_eq!(*values.borrow(), [true]);
    assert_eq!(state.subscribe(&callback), Some(true));
    assert_eq!(*values.borrow(), [true]);
    assert!(state.subscribers.borrow().is_empty());
}

#[test]
fn native_queue_preview_uses_current_callback_and_preserves_order() {
    let object = ObjectId::test(1);
    let first = Callback::new(|_: Key| panic!("preview must not invoke callbacks"));
    let second = Callback::new(|_: Key| panic!("preview must not invoke callbacks"));
    let mut adapter = WinUiAdapter::default();
    adapter.menus.insert(
        object,
        NativeMenu {
            menu: Menu::new([], first),
            revision: 1,
            flyout: None,
            _revokers: Vec::new(),
        },
    );
    let observation = Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Before")),
        },
    };
    adapter.event_queue.queue(
        None,
        Some(QueuedEvent {
            object,
            event: EventId::MenuItemInvoked,
            handler: QueuedHandler::Presentation(0),
            payload: EventPayload::Key("stale".into()),
        }),
    );
    adapter.event_queue.queue(Some(observation.clone()), None);
    adapter.event_queue.queue(
        None,
        Some(QueuedEvent {
            object,
            event: EventId::MenuItemInvoked,
            handler: QueuedHandler::Presentation(1),
            payload: EventPayload::Key("current".into()),
        }),
    );
    adapter.event_queue.queue(Some(observation.clone()), None);

    let stale = adapter.peek_native_event();
    assert_eq!(stale, Some(NativeEvent::new(None, None)));
    assert_eq!(adapter.event_queue.events.borrow().len(), 4);
    assert_eq!(adapter.pop_native_event(), stale);
    assert_eq!(
        adapter.pop_native_event(),
        Some(NativeEvent::new(Some(observation.clone()), None))
    );

    // Callback replacement preserves the menu presentation revision.
    adapter.menus.get_mut(&object).unwrap().menu.on_click = second.clone();
    let current = Some(NativeEvent::new(
        None,
        Some(EventDispatch::new(
            object,
            EventId::MenuItemInvoked,
            EventValue::Key(second),
            EventPayload::Key("current".into()),
        )),
    ));
    assert_eq!(adapter.peek_native_event(), current);
    assert_eq!(adapter.pop_native_event(), current);
    assert_eq!(
        adapter.pop_native_event(),
        Some(NativeEvent::new(Some(observation), None))
    );
    assert_eq!(adapter.pop_native_event(), None);
}

#[test]
fn image_source_change_discards_only_queued_load_results() {
    let image = ObjectId::test(1);
    let other = ObjectId::test(2);
    let queue = NativeEventQueue::default();
    let event = |object, event| QueuedEvent {
        object,
        event,
        handler: QueuedHandler::Callback {
            callback: EventValue::Unit(Callback::new(|()| {})),
            live: Rc::new(Cell::new(true)),
        },
        payload: EventPayload::Unit,
    };
    queue.queue(None, Some(event(image, EventId::ImageOpened)));
    queue.queue(None, Some(event(image, EventId::ImageFailed)));
    queue.queue(None, Some(event(image, EventId::PointerPressed)));
    queue.queue(None, Some(event(other, EventId::ImageOpened)));

    queue.discard_events(image, &IMAGE_SOURCE_EVENTS);

    let events = queue.events.borrow();
    let remaining: Vec<_> = events
        .iter()
        .map(|queued| {
            let event = queued.event.as_ref().unwrap();
            (event.object, event.event)
        })
        .collect();
    assert_eq!(
        remaining,
        [
            (image, EventId::PointerPressed),
            (other, EventId::ImageOpened)
        ]
    );
}

#[test]
fn replacement_discards_queued_reorders_without_callbacks() {
    let replaced = ObjectId::test(1);
    let other = ObjectId::test(2);
    let queue = NativeEventQueue::default();
    let reorder = |object| Observation::Reorder {
        object,
        relation: RelationId::Items,
        children: Vec::new(),
    };
    queue.queue(Some(reorder(replaced)), None);
    queue.queue(Some(reorder(other)), None);

    queue.discard_object(replaced);

    let events = queue.events.borrow();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].observation, Some(reorder(other)));
}

#[test]
fn placement_coordinates_round_trip_negative_monitors_and_taskbar_offsets() {
    let screen = native::RECT {
        left: -1600,
        top: -200,
        right: -800,
        bottom: 400,
    };
    for (x, y) in [(0, 0), (48, 0), (0, 40), (48, 40)] {
        let workspace = window_placement::translate_rect(screen, -x, -y).unwrap();
        let restored = window_placement::translate_rect(workspace, x, y).unwrap();
        assert_eq!(restored, screen);
        assert_eq!(workspace.left, screen.left - x);
        assert_eq!(workspace.top, screen.top - y);
    }
    assert!(
        window_placement::translate_rect(
            native::RECT {
                left: i32::MAX,
                ..Default::default()
            },
            1,
            0,
        )
        .is_err()
    );
}

#[test]
fn initial_placement_rejects_invalid_extents_but_accepts_negative_positions() {
    let placement = WindowPlacement {
        x: -1200,
        y: -600,
        width: 800,
        height: 500,
        maximized: true,
    };
    let visuals = WindowVisuals::new().initial_placement(placement);
    assert_eq!(visuals.initial_placement, Some(placement));
    for invalid in [
        WindowPlacement {
            width: 0,
            ..placement
        },
        WindowPlacement {
            height: -1,
            ..placement
        },
        WindowPlacement {
            x: i32::MAX,
            ..placement
        },
        WindowPlacement {
            y: i32::MAX,
            ..placement
        },
    ] {
        assert!(
            std::panic::catch_unwind(|| WindowVisuals::new().initial_placement(invalid)).is_err()
        );
    }
}

#[test]
fn changed_initial_placement_does_not_become_a_reactive_resize() {
    let initial = WindowVisuals::new()
        .client_size(640.0, 480.0)
        .initial_position(ScreenPoint::new(50, 60));
    let updated = initial.clone().initial_placement(WindowPlacement {
        x: -1200,
        y: -600,
        width: 800,
        height: 500,
        maximized: true,
    });
    let changes = window_visual_changes(&initial, &updated);
    assert!(!changes.client_size);
    assert!(!changes.constraints);
    assert!(!changes.backdrop);
    assert!(!changes.icon);
    assert!(!changes.theme);
}

#[test]
fn content_dialog_schedule_is_fifo_and_cancellable() {
    let first = ObjectId::test(1);
    let second = ObjectId::test(2);
    let third = ObjectId::test(3);
    let mut schedule = ContentDialogSchedule::default();

    schedule.enqueue(first);
    schedule.enqueue(second);
    schedule.enqueue(second);
    schedule.enqueue(third);
    schedule.cancel(second);

    assert_eq!(schedule.next(), Some(first));
    assert_eq!(schedule.next(), Some(third));
    assert_eq!(schedule.next(), None);
}

#[test]
fn content_dialog_schedule_ignores_stale_completion_generation() {
    let dialog = ObjectId::test(1);
    let mut schedule = ContentDialogSchedule::default();
    schedule.begin(dialog, 7);

    assert!(!schedule.complete(dialog, 6));
    assert_eq!(schedule.active, Some((dialog, 7)));
    assert!(schedule.complete(dialog, 7));
    assert_eq!(schedule.active, None);
}

#[test]
fn active_slot_reorder_preserves_interleaved_retiring_indices() {
    let mut current = vec![(1, 1), (99, 99), (2, 2), (3, 3), (4, 4)];
    let active = HashSet::from([1, 2, 3, 4]);

    let swaps = simulate_active_slot_reorder(&mut current, &active, &[4, 3, 2, 1]).unwrap();

    assert_eq!(current, [(4, 4), (99, 99), (3, 3), (2, 2), (1, 1)]);
    assert_eq!(swaps.len(), 2);
    assert!(swaps.iter().all(|(left, right, _, _)| left < right));
}

#[test]
fn active_slot_reorder_rejects_missing_or_duplicate_active_identity() {
    let mut current = vec![(1, ()), (99, ()), (2, ())];
    let active = HashSet::from([1, 2]);

    assert!(simulate_active_slot_reorder(&mut current, &active, &[2, 3]).is_none());
    assert!(simulate_active_slot_reorder(&mut current, &active, &[2, 2]).is_none());
}

#[test]
fn controlled_property_feedback_uses_exact_incumbent_clear_values() {
    let cases = [
        (
            ObjectType::CheckBox,
            PropertyId::IsChecked,
            EventId::IsCheckedChanged,
            PropertyValue::OptionalBool(Some(false)),
        ),
        (
            ObjectType::ToggleButton,
            PropertyId::IsChecked,
            EventId::IsCheckedChanged,
            PropertyValue::OptionalBool(Some(false)),
        ),
        (
            ObjectType::Expander,
            PropertyId::IsExpanded,
            EventId::IsExpandedChanged,
            PropertyValue::Bool(false),
        ),
        (
            ObjectType::NavigationView,
            PropertyId::IsPaneOpen,
            EventId::IsPaneOpenChanged,
            PropertyValue::Bool(true),
        ),
    ];

    for (kind, property, event, value) in cases {
        assert_eq!(
            GeneratedHandle::feedback_expectation(kind, property, None),
            Some((
                event,
                FeedbackExpectation::Exact(Property {
                    id: property,
                    value,
                }),
            ))
        );
    }
}

fn virtual_index(value: IInspectable) -> i32 {
    value
        .cast::<windows_reference::IReference<i32>>()
        .unwrap()
        .Value()
        .unwrap()
}

#[test]
fn recycled_physical_shell_keeps_old_token_until_acknowledged() {
    let mut shells = ShellPool::<usize>::default();
    let (old, physical) = shells.take(|| Ok::<_, windows_core::Error>(42)).unwrap();

    assert!(shells.retire(old));
    assert!(shells.retired.contains(&old));
    let (new, reused) = shells
        .take(|| panic!("retired physical shell was not reused"))
        .unwrap();

    assert_ne!(new, old);
    assert_eq!(reused, physical);
    assert!(shells.retired.contains(&old));
    assert!(shells.shells.contains_key(&new));

    shells.acknowledge_recycle(old);
    assert!(!shells.retired.contains(&old));
    assert!(shells.shells.contains_key(&new));
}

#[test]
fn virtual_source_generates_indices_on_demand() {
    let source = ComObject::new(NativeVirtualSource::new(3).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();

    assert_eq!(vector.Size().unwrap(), 3);
    assert_eq!(virtual_index(vector.GetAt(0).unwrap()), 0);
    assert_eq!(virtual_index(vector.GetAt(2).unwrap()), 2);
    assert_eq!(vector.GetAt(3).unwrap_err().code(), E_BOUNDS);

    let mut index = u32::MAX;
    assert!(
        vector
            .IndexOf(&virtual_item_value(2).unwrap(), &mut index)
            .unwrap()
    );
    assert_eq!(index, 2);
    let missing: IInspectable = windows_reference::IReference::<i32>::from(9).into();
    assert!(!vector.IndexOf(&missing, &mut index).unwrap());
    assert_eq!(index, 0);
    let negative: IInspectable = windows_reference::IReference::<i32>::from(-1).into();
    assert!(!vector.IndexOf(&negative, &mut index).unwrap());
    let invalid: IInspectable =
        windows_reference::IReference::<HSTRING>::from(HSTRING::from("invalid")).into();
    assert!(vector.IndexOf(&invalid, &mut index).is_err());

    let mut values = vec![None; 4];
    assert_eq!(vector.GetMany(1, &mut values).unwrap(), 2);
    assert_eq!(virtual_index(values[0].take().unwrap()), 1);
    assert_eq!(virtual_index(values[1].take().unwrap()), 2);
    assert!(values[2].is_none());

    let view = vector.GetView().unwrap();
    assert_eq!(view.Size().unwrap(), 3);
    assert_eq!(virtual_index(view.GetAt(1).unwrap()), 1);

    let iterator = vector.First().unwrap();
    assert!(iterator.HasCurrent().unwrap());
    assert_eq!(virtual_index(iterator.Current().unwrap()), 0);
    assert!(iterator.MoveNext().unwrap());
    assert_eq!(virtual_index(iterator.Current().unwrap()), 1);
    let mut tail = vec![None; 3];
    assert_eq!(iterator.GetMany(&mut tail).unwrap(), 2);
    assert_eq!(virtual_index(tail[0].take().unwrap()), 1);
    assert_eq!(virtual_index(tail[1].take().unwrap()), 2);
    assert!(!iterator.HasCurrent().unwrap());
    assert_eq!(iterator.Current().unwrap_err().code(), E_BOUNDS);
}

#[test]
fn virtual_source_is_read_only_and_validates_index_limits() {
    let max_count = i32::MAX as usize + 1;
    let source = ComObject::new(NativeVirtualSource::new(max_count).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();

    assert_eq!(vector.Size().unwrap(), max_count as u32);
    assert_eq!(
        virtual_index(vector.GetAt(i32::MAX as u32).unwrap()),
        i32::MAX
    );
    assert_eq!(
        vector.GetAt(i32::MAX as u32 + 1).unwrap_err().code(),
        E_BOUNDS
    );
    assert!(NativeVirtualSource::new(max_count + 1).is_err());

    let value = virtual_item_value(0).unwrap();
    assert_eq!(
        vector.SetAt(0, &value).unwrap_err().code(),
        E_ILLEGAL_METHOD_CALL
    );
    assert_eq!(
        vector.Append(&value).unwrap_err().code(),
        E_ILLEGAL_METHOD_CALL
    );
    assert_eq!(vector.Clear().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
    assert_eq!(
        vector.ReplaceAll(&[Some(value)]).unwrap_err().code(),
        E_ILLEGAL_METHOD_CALL
    );
}

#[test]
fn virtual_source_reset_notifies_once_and_honors_event_removal() {
    let source = ComObject::new(NativeVirtualSource::new(0).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();
    let first = Arc::new(AtomicUsize::new(0));
    let second = Arc::new(AtomicUsize::new(0));
    let first_observed = Arc::clone(&first);
    let first_revoker = vector
        .VectorChanged(move |_, args| {
            let args = args.ok().unwrap();
            assert_eq!(args.CollectionChange().unwrap(), CollectionChange::Reset);
            assert_eq!(args.Index().unwrap(), 0);
            first_observed.fetch_add(1, Ordering::Relaxed);
        })
        .unwrap();
    let second_observed = Arc::clone(&second);
    let second_revoker = vector
        .VectorChanged(move |_, _| {
            second_observed.fetch_add(1, Ordering::Relaxed);
        })
        .unwrap();

    source.reset(10_000);
    assert_eq!(vector.Size().unwrap(), 10_000);
    assert_eq!(first.load(Ordering::Relaxed), 1);
    assert_eq!(second.load(Ordering::Relaxed), 1);

    drop(first_revoker);
    source.reset(10_000);
    assert_eq!(first.load(Ordering::Relaxed), 1);
    assert_eq!(second.load(Ordering::Relaxed), 2);

    drop(second_revoker);
    source.reset(0);
    assert_eq!(vector.Size().unwrap(), 0);
    assert_eq!(first.load(Ordering::Relaxed), 1);
    assert_eq!(second.load(Ordering::Relaxed), 2);
}

#[test]
fn policy_preserves_requested_window_state() {
    let policy = WindowPolicy::new()
        .title("Solitaire")
        .theme(WindowTheme::Dark)
        .client_size(800.0, 600.0)
        .minimum_client_size(800.0, 600.0);

    assert_eq!(policy.title.as_deref(), Some("Solitaire"));
    assert_eq!(policy.theme, WindowTheme::Dark);
    assert_eq!(policy.client_size, Some((800.0, 600.0)));
    assert_eq!(policy.minimum_client_size, Some((800.0, 600.0)));
}

#[test]
fn policy_rejects_invalid_window_sizes() {
    for (width, height) in [
        (0.0, 1.0),
        (1.0, -1.0),
        (f64::NAN, 1.0),
        (1.0, f64::INFINITY),
    ] {
        assert!(
            std::panic::catch_unwind(|| WindowPolicy::new().client_size(width, height)).is_err()
        );
        assert!(
            std::panic::catch_unwind(|| { WindowPolicy::new().minimum_client_size(width, height) })
                .is_err()
        );
    }
}

#[test]
fn window_visual_changes_only_apply_changed_contracts() {
    let previous = WindowVisuals::new()
        .theme(WindowTheme::System)
        .backdrop(WindowBackdrop::Mica)
        .client_size(800.0, 600.0);
    let next = WindowVisuals::new()
        .theme(WindowTheme::Dark)
        .backdrop(WindowBackdrop::Mica)
        .client_size(800.0, 600.0);

    let changes = window_visual_changes(&previous, &next);
    assert!(!changes.backdrop);
    assert!(!changes.client_size);
    assert!(!changes.constraints);
    assert!(!changes.icon);
    assert!(changes.theme);
    assert!(changes.any());
    assert!(!window_visual_changes(&next, &next).any());
}

#[test]
fn window_visual_changes_detect_icon_removal() {
    let previous = WindowVisuals::new().icon("test.ico");
    let changes = window_visual_changes(&previous, &WindowVisuals::new());
    assert!(changes.icon);
}

#[test]
fn pointer_event_queue_preserves_payload_and_callback() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Border::new());
    let object = runtime.graph().root().unwrap();
    let callback = Callback::new(|_| {});
    let event = Rc::new(RefCell::new(NativePointerEventInfoEvent::default()));
    event.borrow_mut().set(Some(callback.clone()));
    let event_queue = Rc::new(NativeEventQueue::default());
    let payload = PointerEventInfo {
        pointer_id: 42,
        modifiers: InputModifiers::CONTROL,
        is_captured: true,
        is_right_button_pressed: true,
        ..Default::default()
    };

    assert!(WinUiAdapter::queue_event(
        &event,
        &event_queue,
        object,
        EventId::PointerReleased,
        None,
        payload,
    ));
    let queued = event_queue.events.borrow();
    assert_eq!(queued.len(), 1);
    assert!(queued[0].observation.is_none());
    let event = queued[0].event.as_ref().unwrap();
    assert_eq!(event.object, object);
    assert_eq!(event.event, EventId::PointerReleased);
    assert!(matches!(
        &event.handler,
        QueuedHandler::Callback {
            callback: EventValue::PointerEventInfo(queued),
            ..
        } if *queued == callback
    ));
    assert_eq!(event.payload, EventPayload::PointerEventInfo(payload));
}

#[test]
fn queued_occurrences_follow_their_subscription() {
    let object = ObjectId::test(1);
    let first = Callback::new(|_: ()| {});
    let second = Callback::new(|_: ()| {});
    let third = Callback::new(|_: ()| {});
    let event = Rc::new(RefCell::new(NativeUnitEvent::default()));
    let mut adapter = WinUiAdapter::default();
    let queue = |adapter: &WinUiAdapter| {
        assert!(WinUiAdapter::queue_event(
            &event,
            &adapter.event_queue,
            object,
            EventId::Click,
            None,
            (),
        ));
    };
    let delivered = |callback: &Callback<()>| {
        Some(NativeEvent::new(
            None,
            Some(EventDispatch::new(
                object,
                EventId::Click,
                EventValue::Unit(callback.clone()),
                EventPayload::Unit,
            )),
        ))
    };

    // Replacing the callback keeps the subscription, so the occurrence reaches the callback
    // subscribed when it fired.
    event.borrow_mut().set(Some(first.clone()));
    queue(&adapter);
    event.borrow_mut().set(Some(second));
    assert_eq!(adapter.pop_native_event(), delivered(&first));

    // Removing the event ends the subscription even if it is re-added before delivery.
    queue(&adapter);
    event.borrow_mut().set(None);
    event.borrow_mut().set(Some(third.clone()));
    assert_eq!(
        adapter.pop_native_event(),
        Some(NativeEvent::new(None, None))
    );

    queue(&adapter);
    assert_eq!(adapter.pop_native_event(), delivered(&third));

    // An occurrence completing asynchronously captures its handler when the event fires.
    let pending = event.borrow().handler().unwrap();
    event.borrow_mut().set(None);
    assert!(matches!(pending, QueuedHandler::Callback { live, .. } if !live.get()));
}

#[test]
fn pointer_event_modifiers_map_native_flags() {
    let mut expected = InputModifiers::SHIFT;
    expected |= InputModifiers::CONTROL;
    expected |= InputModifiers::ALT;
    expected |= InputModifiers::WINDOWS;

    assert_eq!(
        WinUiAdapter::input_modifiers_from_virtual_keys(
            native::VirtualKeyModifiers::Shift
                | native::VirtualKeyModifiers::Control
                | native::VirtualKeyModifiers::Menu
                | native::VirtualKeyModifiers::Windows
        ),
        expected
    );
    assert_eq!(
        WinUiAdapter::input_modifiers_from_virtual_keys(native::VirtualKeyModifiers::None),
        InputModifiers::NONE
    );
}

#[test]
fn optional_numeric_adapters_use_distinct_sentinels() {
    assert!(native_number_box_value(None).is_nan());
    assert_eq!(number_box_value(f64::NAN), None);
    assert_eq!(number_box_value(-1.0), Some(-1.0));

    assert_eq!(native_rating_value(None), -1.0);
    assert_eq!(rating_value(-1.0), None);
    assert!(rating_value(f64::NAN).unwrap().is_nan());
}

#[test]
fn selection_index_conversion_reports_overflow() {
    assert_eq!(native_selection_index(None), Ok(-1));
    assert_eq!(
        native_selection_index(Some(i32::MAX as usize)),
        Ok(i32::MAX)
    );
    let overflow = i32::MAX as usize + 1;
    assert_eq!(
        native_selection_index(Some(overflow)),
        Err(WinUiError::IndexOverflow(overflow))
    );
}

#[test]
fn generated_feedback_contracts_distinguish_exact_normalized_and_clear() {
    let exact = GeneratedHandle::feedback_expectation(
        ObjectType::ToggleSwitch,
        PropertyId::IsOn,
        Some(&PropertyValue::Bool(true)),
    );
    assert_eq!(
        exact,
        Some((
            EventId::Toggled,
            FeedbackExpectation::Exact(Property {
                id: PropertyId::IsOn,
                value: PropertyValue::Bool(true),
            })
        ))
    );
    assert_eq!(
        GeneratedHandle::feedback_expectation(ObjectType::ToggleSwitch, PropertyId::IsOn, None,),
        Some((
            EventId::Toggled,
            FeedbackExpectation::Exact(Property {
                id: PropertyId::IsOn,
                value: PropertyValue::Bool(false),
            })
        ))
    );
    assert_eq!(
        GeneratedHandle::feedback_expectation(
            ObjectType::Slider,
            PropertyId::Value,
            Some(&PropertyValue::F64(0.5)),
        ),
        Some((
            EventId::ValueChanged,
            FeedbackExpectation::Normalized { observation: None }
        ))
    );
    assert_eq!(
        GeneratedHandle::feedback_expectation(
            ObjectType::ProgressBar,
            PropertyId::Value,
            Some(&PropertyValue::F64(0.5)),
        ),
        None
    );
    assert_eq!(
        GeneratedHandle::feedback_expectation(ObjectType::RichEditBox, PropertyId::Document, None,),
        Some((
            EventId::TextChanged,
            FeedbackExpectation::DeferredExact(Property {
                id: PropertyId::Document,
                value: PropertyValue::String(Rc::from("")),
            })
        ))
    );
}
