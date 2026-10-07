use super::*;

#[test]
fn event_callbacks_update_without_recreating_the_object() {
    let first = Callback::new(|_: Rc<str>| {});
    let second = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed(first))
        .unwrap();
    let root = runtime.graph().root().unwrap();

    let mutations = runtime
        .update(TextBox::new("Before").on_text_changed(second))
        .unwrap();

    assert!(matches!(
        mutations.as_slice(),
        [Mutation::SetEvents { object, .. }] if *object == root
    ));
}

#[test]
fn event_callbacks_can_be_removed_without_recreating_the_object() {
    let callback = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Text").on_text_changed(callback))
        .unwrap();
    let root = runtime.graph().root().unwrap();

    let mutations = runtime.update(TextBox::new("Text")).unwrap();

    assert!(matches!(
        mutations.as_slice(),
        [Mutation::SetEvents { object, set, clear }]
            if *object == root && set.is_empty() && clear.as_ref() == [EventId::TextChanged]
    ));
}

#[test]
fn native_property_observation_updates_retained_state_before_reconciliation() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });

    let mutations = runtime.update(TextBox::new("After")).unwrap();

    assert!(mutations.is_empty());
    assert_eq!(
        runtime.graph().properties(root).unwrap(),
        [Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        }]
    );
}

#[test]
fn subtree_update_consumes_pending_controlled_observation() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed("text", TextBox::new("Before"))]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let text = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: text,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });

    let mutations = runtime.update_subtree(text, TextBox::new("After")).unwrap();

    assert!(mutations.is_empty());
    assert_eq!(
        runtime.graph().properties(text).unwrap(),
        [Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        }]
    );
}

#[test]
fn remove_child_consumes_pending_observation_before_planning() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed("text", TextBox::new("Before"))]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let text = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: text,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });

    runtime
        .remove_child(root, RelationId::Children, text)
        .unwrap();

    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        []
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn invalid_observation_batch_rolls_back_and_remains_queued() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Expanded,
            value: PropertyValue::Bool(true),
        },
    });
    let graph = runtime.graph().clone();
    let adapter = runtime.adapter().clone();

    assert_eq!(
        runtime.update(TextBox::new("After")),
        Err(UpdateError::InvalidNativeEvent(
            GraphError::InvalidProperty(ObjectType::TextBox, PropertyId::Expanded)
        ))
    );
    assert_eq!(runtime.graph(), &graph);
    assert_eq!(runtime.adapter(), &adapter);
    assert_eq!(
        runtime.update(TextBox::new("Before")),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn ordered_native_events_hide_future_observations_from_earlier_callbacks() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed(callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    for value in ["A", "B"] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                object,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }

    let mut first = runtime.next_native_event().unwrap().unwrap();
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("A"))
    );
    first.invoke();
    assert_eq!(&*values.borrow(), &[Rc::from("A")]);
    drop(first);

    let mut second = runtime.next_native_event().unwrap().unwrap();
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("B"))
    );
    second.invoke();
    drop(second);
    assert_eq!(&*values.borrow(), &[Rc::from("A"), Rc::from("B")]);
}

#[test]
fn callback_reconciliation_completes_before_the_next_native_occurrence() {
    let rendered = Rc::new(Cell::new(false));
    let callback = {
        let rendered = Rc::clone(&rendered);
        Callback::new(move |_: Rc<str>| rendered.set(true))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed(callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    for value in ["A", "B"] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                object,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }

    let mut first = runtime.next_native_event().unwrap().unwrap();
    first.invoke();
    assert!(rendered.get());
    runtime
        .update(TextBox::new("Rendered").on_text_changed(callback))
        .unwrap();
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("Rendered"))
    );
    drop(first);

    let mut second = runtime.next_native_event().unwrap().unwrap();
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("B"))
    );
    second.invoke();
    drop(second);
}

#[test]
fn callback_panic_releases_the_native_boundary_and_remaining_events_continue() {
    let panic_callback = Callback::new(|_: Rc<str>| panic!("test callback panic"));
    let calls = Rc::new(Cell::new(0));
    let next_callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |_: Rc<str>| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed(panic_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TextChanged,
        EventValue::String(panic_callback),
        EventPayload::String(Rc::from("A")),
    ));
    runtime.adapter_mut().observe(Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("B")),
        },
    });

    let mut event = runtime.next_native_event().unwrap().unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| event.invoke()));

    assert!(result.is_err());
    runtime
        .update(TextBox::new("B").on_text_changed(next_callback.clone()))
        .unwrap();
    drop(event);
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("B"))
    );
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TextChanged,
        EventValue::String(next_callback),
        EventPayload::String(Rc::from("C")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(calls.get(), 1);
}

#[test]
fn exact_feedback_suppresses_matching_observation_until_write_finishes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let object = runtime.graph().root().unwrap();
    let property = Property {
        id: PropertyId::Text,
        value: PropertyValue::String(Rc::from("After")),
    };
    let mut feedback = FeedbackState::default();
    feedback.begin(
        object,
        EventId::TextChanged,
        FeedbackExpectation::Exact(property.clone()),
    );

    assert!(!feedback.observe(
        object,
        EventId::TextChanged,
        Observation::SetProperty {
            object,
            property: property.clone(),
        }
    ));
    assert_eq!(feedback.finish(object, EventId::TextChanged), None);

    feedback.begin(
        object,
        EventId::TextChanged,
        FeedbackExpectation::Exact(property),
    );
    assert!(feedback.observe(
        object,
        EventId::TextChanged,
        Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("Normalized")),
            },
        }
    ));
}

#[test]
fn normalized_feedback_defers_latest_observation_until_write_finishes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Slider::new()).unwrap();
    let object = runtime.graph().root().unwrap();
    let mut feedback = FeedbackState::default();
    feedback.begin(
        object,
        EventId::ValueChanged,
        FeedbackExpectation::Normalized { observation: None },
    );
    let observation = Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Value,
            value: PropertyValue::F64(0.75),
        },
    };

    assert!(!feedback.observe(object, EventId::ValueChanged, observation.clone()));
    assert_eq!(
        feedback.finish(object, EventId::ValueChanged),
        Some(observation)
    );
}

#[test]
fn suppressed_feedback_drops_selection_observations_during_native_writes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ListBox::new().keyed_items([
            keyed("first", ListBoxItem::new().tag("first").is_selected(true)),
            keyed(
                "second",
                ListBoxItem::new().tag("second").is_selected(false),
            ),
        ]))
        .unwrap();
    let owner = runtime.graph().root().unwrap();
    let selected = runtime.graph().children(owner, RelationId::Items).unwrap()[1];
    let observation = Observation::SetSelection {
        object: owner,
        selected: Some(selected),
    };
    let mut feedback = FeedbackState::default();
    feedback.begin(
        owner,
        EventId::SelectionChanged,
        FeedbackExpectation::Suppressed,
    );

    assert!(!feedback.observe(owner, EventId::SelectionChanged, observation));
    assert_eq!(feedback.finish(owner, EventId::SelectionChanged), None);
}

#[test]
fn straightforward_event_payloads_round_trip_through_recording_protocol() {
    let boolean = Rc::new(Cell::new(false));
    let boolean_callback = {
        let boolean = Rc::clone(&boolean);
        Callback::new(move |value| boolean.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ToggleSwitch::new().on_toggled(boolean_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::Toggled,
        EventValue::Bool(boolean_callback),
        EventPayload::Bool(true),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert!(boolean.get());

    let text = Rc::new(RefCell::new(None));
    let text_callback = {
        let text = Rc::clone(&text);
        Callback::new(move |value| *text.borrow_mut() = Some(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(PasswordBox::new().on_password_changed(text_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::PasswordChanged,
        EventValue::String(text_callback),
        EventPayload::String(Rc::from("secret")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(*text.borrow(), Some(Rc::from("secret")));

    let number = Rc::new(Cell::new(None));
    let number_callback = {
        let number = Rc::clone(&number);
        Callback::new(move |value| number.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(RatingControl::new().on_value_changed(number_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::ValueChanged,
        EventValue::OptionalF64(number_callback),
        EventPayload::OptionalF64(Some(4.0)),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(number.get(), Some(4.0));

    let date = DateTime::from_unix_secs(1_700_000_000);
    let dates = Rc::new(RefCell::new(Vec::new()));
    let date_callback = {
        let dates = Rc::clone(&dates);
        Callback::new(move |value| dates.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(CalendarDatePicker::new().on_date_changed(date_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::DateChanged,
        EventValue::OptionalDateTime(date_callback.clone()),
        EventPayload::OptionalDateTime(Some(date)),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::DateChanged,
        EventValue::OptionalDateTime(date_callback),
        EventPayload::OptionalDateTime(None),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*dates.borrow(), &[Some(date), None]);

    let index = Rc::new(Cell::new(None));
    let index_callback = {
        let index = Rc::clone(&index);
        Callback::new(move |value| index.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ComboBox::new().on_selection_changed(index_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(index_callback),
        EventPayload::SelectionIndex(Some(2)),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(index.get(), Some(2));
}

#[test]
fn invalid_native_property_observation_is_rejected() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Text")).unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Expanded,
            value: PropertyValue::Bool(true),
        },
    });

    assert!(matches!(
        runtime.update(TextBox::new("Text")),
        Err(UpdateError::InvalidNativeEvent(
            GraphError::InvalidProperty(ObjectType::TextBox, PropertyId::Expanded)
        ))
    ));
    assert_eq!(
        runtime.update(TextBox::new("Text")),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn invalid_generated_enum_observation_is_rejected() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().orientation(Orientation::Vertical))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Orientation,
            value: PropertyValue::Enum {
                kind: "Orientation",
                variant: "Diagonal",
            },
        },
    });

    assert!(matches!(
        runtime.update(StackPanel::new()),
        Err(UpdateError::InvalidNativeEvent(
            GraphError::InvalidPropertyValue(PropertyId::Orientation)
        ))
    ));
    assert_eq!(
        runtime.update(StackPanel::new()),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn stale_queued_event_does_not_reach_replacement_callback() {
    let first_count = Rc::new(Cell::new(0));
    let first_count_for_callback = Rc::clone(&first_count);
    let first = Callback::new(move |_: Rc<str>| {
        first_count_for_callback.set(first_count_for_callback.get() + 1);
    });
    let second_count = Rc::new(Cell::new(0));
    let second_count_for_callback = Rc::clone(&second_count);
    let second = Callback::new(move |_: Rc<str>| {
        second_count_for_callback.set(second_count_for_callback.get() + 1);
    });
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Text").on_text_changed(first.clone()))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    for value in ["First", "Stale"] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object: root,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                root,
                EventId::TextChanged,
                EventValue::String(first.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }
    let mut active = runtime.next_native_event().unwrap().unwrap();
    active.invoke();
    runtime
        .update(TextBox::new("First").on_text_changed(second.clone()))
        .unwrap();
    drop(active);

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(
        runtime.graph().properties(root).unwrap()[0].value,
        PropertyValue::String(Rc::from("Stale"))
    );
    assert!(
        runtime
            .update(TextBox::new("Stale").on_text_changed(second))
            .unwrap()
            .is_empty()
    );
    assert_eq!(first_count.get(), 1);
    assert_eq!(second_count.get(), 0);
}
