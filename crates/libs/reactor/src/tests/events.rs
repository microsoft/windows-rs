use super::*;

#[test]
fn property_observations_preserve_declaration_ownership() {
    let cases: [(View, View, Property); 7] = [
        (
            TextBox::new().into(),
            TextBox::new().text("").into(),
            Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("Edited")),
            },
        ),
        (
            CheckBox::new().into(),
            CheckBox::new().is_checked(None).into(),
            Property {
                id: PropertyId::IsChecked,
                value: PropertyValue::OptionalBool(Some(true)),
            },
        ),
        (
            Slider::new().into(),
            Slider::new().value(0.0).into(),
            Property {
                id: PropertyId::Value,
                value: PropertyValue::F64(42.0),
            },
        ),
        (
            NumberBox::new().into(),
            NumberBox::new().value(None).into(),
            Property {
                id: PropertyId::Value,
                value: PropertyValue::OptionalF64(Some(42.0)),
            },
        ),
        (
            ListView::new().into(),
            ListView::new().selected_index(None).into(),
            Property {
                id: PropertyId::SelectedIndex,
                value: PropertyValue::SelectionIndex(Some(1)),
            },
        ),
        (
            RichEditBox::new().into(),
            RichEditBox::new().text("").into(),
            Property {
                id: PropertyId::Document,
                value: PropertyValue::String(Rc::from("Edited")),
            },
        ),
        (
            CalendarDatePicker::new().into(),
            CalendarDatePicker::new().date(None).into(),
            Property {
                id: PropertyId::Date,
                value: PropertyValue::OptionalDateTime(Some(DateTime::from_unix_secs(
                    1_700_000_000,
                ))),
            },
        ),
    ];
    for (omitted, declared, observed) in cases {
        let mut runtime = Runtime::new(RecordingAdapter::default());
        runtime.update(omitted.clone());
        let object = runtime.graph().root().unwrap();
        let observation = Observation::SetProperty {
            object,
            property: observed.clone(),
        };
        runtime.adapter_mut().observe(observation.clone());
        assert!(runtime.update(omitted.clone()).is_empty());
        assert!(runtime.graph().properties(object).unwrap().is_empty());

        let mutations = runtime.update(declared.clone());
        let expected = runtime.graph().properties(object).unwrap()[0].clone();
        assert!(matches!(
            mutations.as_slice(),
            [Mutation::SetProperties { set, clear, .. }]
                if set.as_ref() == [expected.clone()] && clear.is_empty()
        ));
        runtime.adapter_mut().observe(observation.clone());
        runtime.dispatch_native_events();
        assert_eq!(runtime.graph().properties(object).unwrap(), [observed]);
        let mutations = runtime.update(declared.clone());
        assert!(matches!(
            mutations.as_slice(),
            [Mutation::SetProperties { set, clear, .. }]
                if set.as_ref() == [expected.clone()] && clear.is_empty()
        ));

        let mutations = runtime.update(omitted.clone());
        assert!(matches!(
            mutations.as_slice(),
            [Mutation::SetProperties { set, clear, .. }]
                if set.is_empty() && clear.as_ref() == [expected.id]
        ));
        runtime.adapter_mut().observe(Observation::SetProperty {
            object,
            property: expected.clone(),
        });
        runtime.adapter_mut().observe(observation);
        runtime.dispatch_native_events();
        assert!(runtime.graph().properties(object).unwrap().is_empty());
        assert!(runtime.update(omitted).is_empty());

        let mutations = runtime.update(declared);
        assert!(matches!(
            mutations.as_slice(),
            [Mutation::SetProperties { set, clear, .. }]
                if set.as_ref() == [expected] && clear.is_empty()
        ));
    }
}

#[test]
fn queued_callbacks_do_not_reclaim_a_removed_property() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let received = Rc::clone(&values);
    let callback = Callback::new(move |value| received.borrow_mut().push(value));
    let omitted = NumberBox::new().on_value_changed(callback.clone());
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(omitted.clone().value(1.0));
    let object = runtime.graph().root().unwrap();
    for value in [Some(2.0), None, Some(3.0)] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Value,
                    value: PropertyValue::OptionalF64(value),
                },
            }),
            Some(EventDispatch::new(
                object,
                EventId::ValueChanged,
                EventValue::OptionalF64(callback.clone()),
                EventPayload::OptionalF64(value),
            )),
        );
    }
    let mut first = runtime.next_native_event().unwrap();
    first.invoke();
    runtime.update(omitted.clone());
    drop(first);

    assert_eq!(runtime.dispatch_native_events(), 2);
    assert_eq!(*values.borrow(), [Some(2.0), None, Some(3.0)]);
    assert!(runtime.graph().properties(object).unwrap().is_empty());
    assert!(runtime.update(omitted.clone()).is_empty());
    assert!(matches!(
        runtime.update(omitted.value(3.0)).as_slice(),
        [Mutation::SetProperties { set, clear, .. }]
            if set.as_ref() == [Property {
                id: PropertyId::Value,
                value: PropertyValue::OptionalF64(Some(3.0)),
            }] && clear.is_empty()
    ));
}

#[test]
#[should_panic(expected = "invalid value for property Value")]
fn undeclared_property_observations_are_still_validated() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Slider::new());
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Value,
            value: PropertyValue::OptionalF64(None),
        },
    });
    runtime.update(Slider::new());
}

#[test]
fn event_callbacks_update_without_recreating_the_object() {
    let first = Callback::new(|_: Rc<str>| {});
    let second = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new().text("Before").on_text_changed(first));
    let root = runtime.graph().root().unwrap();

    let mutations = runtime.update(TextBox::new().text("Before").on_text_changed(second));

    assert!(matches!(
        mutations.as_slice(),
        [Mutation::SetEvents { object, .. }] if *object == root
    ));
}

#[test]
fn event_callbacks_can_be_removed_without_recreating_the_object() {
    let callback = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new().text("Text").on_text_changed(callback));
    let root = runtime.graph().root().unwrap();

    let mutations = runtime.update(TextBox::new().text("Text"));

    assert!(matches!(
        mutations.as_slice(),
        [Mutation::SetEvents { object, set, clear }]
            if *object == root && set.is_empty() && clear.as_ref() == [EventId::TextChanged]
    ));
}

#[test]
fn native_property_observation_updates_retained_state_before_reconciliation() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new().text("Before"));
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });

    let mutations = runtime.update(TextBox::new().text("After"));

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
    runtime.update(Grid::new().keyed_children([keyed("text", TextBox::new().text("Before"))]));
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

    let mutations = runtime.update_subtree(text, TextBox::new().text("After"));

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
    runtime.update(Grid::new().keyed_children([keyed("text", TextBox::new().text("Before"))]));
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

    runtime.remove_child(root, RelationId::Children, text);

    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        []
    );
    assert_eq!(runtime.dispatch_native_events(), 0);
}

#[test]
#[should_panic(expected = "invalid property Expanded for TextBox")]
fn native_queue_invalid_callback_observation_panics_before_dispatch() {
    let callback = Callback::new(|_: Rc<str>| panic!("invalid prefix must not dispatch"));
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        TextBox::new()
            .text("Before")
            .on_text_changed(callback.clone()),
    );
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Pending")),
        },
    });
    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Expanded,
                value: PropertyValue::Bool(true),
            },
        }),
        Some(EventDispatch::new(
            object,
            EventId::TextChanged,
            EventValue::String(callback),
            EventPayload::String(Rc::from("Invalid")),
        )),
    );
    while runtime.next_native_work().is_some() {}
}

#[test]
fn native_queue_callback_boundary_rejects_later_invalid_observations() {
    let calls = Rc::new(Cell::new(0));
    let called = Rc::clone(&calls);
    let callback = Callback::new(move |_: Rc<str>| called.set(called.get() + 1));
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        TextBox::new()
            .text("Before")
            .on_text_changed(callback.clone()),
    );
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TextChanged,
        EventValue::String(callback),
        EventPayload::String(Rc::from("First")),
    ));
    for property in [
        Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
        Property {
            id: PropertyId::Expanded,
            value: PropertyValue::Bool(true),
        },
    ] {
        runtime
            .adapter_mut()
            .observe(Observation::SetProperty { object, property });
    }

    let Some(NativeWork::Event(mut event)) = runtime.next_native_work() else {
        panic!("expected earlier valid callback");
    };
    event.invoke();
    drop(event);
    assert_eq!(calls.get(), 1);
    assert_panics("invalid property Expanded for TextBox", || {
        while runtime.next_native_work().is_some() {}
    });
}

#[test]
fn native_queue_callback_boundary_precedes_later_realization_validation() {
    let calls = Rc::new(Cell::new(0));
    let called = Rc::clone(&calls);
    let callback = Callback::new(move |()| called.set(called.get() + 1));
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        Grid::new().children([
            Button::new().on_click(callback.clone()).into(),
            ItemsRepeater::new()
                .item("only", TextBlock::new().text("Only"))
                .into(),
        ]),
    );
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    let (button, collection) = (children[0], children[1]);
    runtime.adapter_mut().queue_event(EventDispatch::new(
        button,
        EventId::Click,
        EventValue::Unit(callback),
        EventPayload::Unit,
    ));
    for (container, index) in [(1, 0), (2, 1)] {
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(container),
                index,
                source_revision: 0,
            });
    }

    let Some(NativeWork::Event(mut event)) = runtime.next_native_work() else {
        panic!("expected earlier valid callback");
    };
    event.invoke();
    drop(event);
    assert_eq!(calls.get(), 1);
    let Some(NativeWork::Virtual(VirtualWork::Realize {
        lease, index, view, ..
    })) = runtime.next_native_work()
    else {
        panic!("expected earlier valid realization");
    };
    runtime.realize_virtual(&lease, index, *view);
    assert_panics(&format!("invalid realization 1 for {collection:?}"), || {
        runtime.next_native_work();
    });
}

#[test]
fn ordered_native_events_hide_future_observations_from_earlier_callbacks() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        TextBox::new()
            .text("Before")
            .on_text_changed(callback.clone()),
    );
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

    let before = runtime.graph().clone();
    let queued = runtime.adapter().clone();
    for _ in 0..2 {
        assert!(!runtime.prepare_update());
        assert_eq!(runtime.graph(), &before);
        assert_eq!(runtime.adapter(), &queued);
    }
    let Some(NativeWork::Event(mut first)) = runtime.next_native_work() else {
        panic!("expected first text change");
    };
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("A"))
    );
    first.invoke();
    assert_eq!(&*values.borrow(), &[Rc::from("A")]);
    drop(first);

    let before = runtime.graph().clone();
    assert!(!runtime.prepare_update());
    assert_eq!(runtime.graph(), &before);
    let Some(NativeWork::Event(mut second)) = runtime.next_native_work() else {
        panic!("expected second text change");
    };
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
    runtime.update(
        TextBox::new()
            .text("Before")
            .on_text_changed(callback.clone()),
    );
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

    let mut first = runtime.next_native_event().unwrap();
    first.invoke();
    assert!(rendered.get());
    runtime.update(TextBox::new().text("Rendered").on_text_changed(callback));
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("Rendered"))
    );
    drop(first);

    let mut second = runtime.next_native_event().unwrap();
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
    runtime.update(
        TextBox::new()
            .text("Before")
            .on_text_changed(panic_callback.clone()),
    );
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

    let mut event = runtime.next_native_event().unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| event.invoke()));

    assert!(result.is_err());
    runtime.update(
        TextBox::new()
            .text("B")
            .on_text_changed(next_callback.clone()),
    );
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
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(calls.get(), 1);
}

#[test]
fn exact_feedback_suppresses_matching_observation_until_write_finishes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new().text("Before"));
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
    runtime.update(Slider::new());
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
fn suppressed_feedback_drops_selection_observations_and_keeps_expectations() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(ListBox::new().keyed_items([
        keyed("first", ListBoxItem::new().tag("first").is_selected(true)),
        keyed(
            "second",
            ListBoxItem::new().tag("second").is_selected(false),
        ),
    ]));
    let owner = runtime.graph().root().unwrap();
    let selected = runtime.graph().children(owner, RelationId::Items).unwrap()[1];
    let observation = Observation::SetSelection {
        object: owner,
        selected: Some(selected),
    };
    let mut feedback = FeedbackState::default();
    feedback.suppress(owner, EventId::SelectionChanged);
    feedback.suppress(owner, EventId::SelectionChanged);
    assert!(!feedback.observe(owner, EventId::SelectionChanged, observation.clone()));
    feedback.release(owner, EventId::SelectionChanged);
    assert!(!feedback.observe(owner, EventId::SelectionChanged, observation.clone()));
    feedback.release(owner, EventId::SelectionChanged);
    assert!(feedback.observe(owner, EventId::SelectionChanged, observation));

    let index = |value| Observation::SetProperty {
        object: owner,
        property: Property {
            id: PropertyId::SelectedIndex,
            value: PropertyValue::SelectionIndex(value),
        },
    };
    feedback.begin(
        owner,
        EventId::SelectionChanged,
        FeedbackExpectation::DeferredExact(Property {
            id: PropertyId::SelectedIndex,
            value: PropertyValue::SelectionIndex(Some(1)),
        }),
    );
    assert_eq!(feedback.finish(owner, EventId::SelectionChanged), None);
    feedback.suppress(owner, EventId::SelectionChanged);
    assert!(!feedback.observe(owner, EventId::SelectionChanged, index(Some(0))));
    feedback.release(owner, EventId::SelectionChanged);
    assert!(!feedback.observe(owner, EventId::SelectionChanged, index(Some(1))));
    assert!(feedback.observe(owner, EventId::SelectionChanged, index(Some(2))));
}

#[test]
fn straightforward_event_payloads_round_trip_through_recording_protocol() {
    let boolean = Rc::new(Cell::new(false));
    let boolean_callback = {
        let boolean = Rc::clone(&boolean);
        Callback::new(move |value| boolean.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(ToggleSwitch::new().on_toggled(boolean_callback.clone()));
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::Toggled,
        EventValue::Bool(boolean_callback),
        EventPayload::Bool(true),
    ));
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert!(boolean.get());

    let text = Rc::new(RefCell::new(None));
    let text_callback = {
        let text = Rc::clone(&text);
        Callback::new(move |value| *text.borrow_mut() = Some(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(PasswordBox::new().on_password_changed(text_callback.clone()));
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::PasswordChanged,
        EventValue::String(text_callback),
        EventPayload::String(Rc::from("secret")),
    ));
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(*text.borrow(), Some(Rc::from("secret")));

    let number = Rc::new(Cell::new(None));
    let number_callback = {
        let number = Rc::clone(&number);
        Callback::new(move |value| number.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(RatingControl::new().on_value_changed(number_callback.clone()));
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::ValueChanged,
        EventValue::OptionalF64(number_callback),
        EventPayload::OptionalF64(Some(4.0)),
    ));
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(number.get(), Some(4.0));

    let date = DateTime::from_unix_secs(1_700_000_000);
    let dates = Rc::new(RefCell::new(Vec::new()));
    let date_callback = {
        let dates = Rc::clone(&dates);
        Callback::new(move |value| dates.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(CalendarDatePicker::new().on_date_changed(date_callback.clone()));
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
    assert_eq!(runtime.dispatch_native_events(), 2);
    assert_eq!(&*dates.borrow(), &[Some(date), None]);

    let index = Rc::new(Cell::new(None));
    let index_callback = {
        let index = Rc::clone(&index);
        Callback::new(move |value| index.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(ComboBox::new().on_selection_changed(index_callback.clone()));
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(index_callback),
        EventPayload::SelectionIndex(Some(2)),
    ));
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(index.get(), Some(2));
}

#[test]
#[should_panic(expected = "invalid property Expanded for TextBox")]
fn invalid_native_property_observation_panics() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new().text("Text"));
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Expanded,
            value: PropertyValue::Bool(true),
        },
    });

    runtime.update(TextBox::new().text("Text"));
}

#[test]
#[should_panic(expected = "invalid value for property Orientation")]
fn invalid_generated_enum_observation_panics() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(StackPanel::new().orientation(Orientation::Vertical));
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

    runtime.update(StackPanel::new());
}

fn counting_callback() -> (Callback<Rc<str>>, Rc<RefCell<Vec<Rc<str>>>>) {
    let values = Rc::new(RefCell::new(Vec::new()));
    let values_for_callback = Rc::clone(&values);
    let callback = Callback::new(move |value| values_for_callback.borrow_mut().push(value));
    (callback, values)
}

fn queue_text_changes(
    runtime: &mut Runtime<RecordingAdapter>,
    root: ObjectId,
    callback: &Callback<Rc<str>>,
    values: &[&str],
) {
    for value in values {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object: root,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(*value)),
                },
            }),
            Some(EventDispatch::new(
                root,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(*value)),
            )),
        );
    }
}

#[test]
fn queued_event_reaches_callback_subscribed_when_it_fired() {
    let (first, first_values) = counting_callback();
    let (second, second_values) = counting_callback();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new().text("Text").on_text_changed(first.clone()));
    let root = runtime.graph().root().unwrap();
    queue_text_changes(&mut runtime, root, &first, &["a", "ab"]);
    let mut active = runtime.next_native_event().unwrap();
    active.invoke();
    // The render after the first change replaces the callback, as a capturing closure does.
    runtime.update(TextBox::new().text("a").on_text_changed(second.clone()));
    drop(active);

    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(*first_values.borrow(), [Rc::from("a"), Rc::from("ab")]);
    assert!(second_values.borrow().is_empty());
    assert_eq!(
        runtime.graph().properties(root).unwrap()[0].value,
        PropertyValue::String(Rc::from("ab"))
    );

    queue_text_changes(&mut runtime, root, &second, &["abc"]);
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(*second_values.borrow(), [Rc::from("abc")]);
}

#[test]
fn queued_event_is_dropped_when_subscription_is_removed() {
    let (callback, values) = counting_callback();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        TextBox::new()
            .text("Text")
            .on_text_changed(callback.clone()),
    );
    let root = runtime.graph().root().unwrap();
    queue_text_changes(&mut runtime, root, &callback, &["a", "ab"]);
    let mut active = runtime.next_native_event().unwrap();
    active.invoke();
    runtime.update(TextBox::new().text("a"));
    drop(active);

    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(*values.borrow(), [Rc::from("a")]);
    // The observation still records the native value.
    assert_eq!(
        runtime.graph().properties(root).unwrap()[0].value,
        PropertyValue::String(Rc::from("ab"))
    );
}
