use super::*;

#[test]
fn rich_edit_text_is_canonical_and_reports_native_edits() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        RichEditBox::new()
            .text("one\r\ntwo\rthree")
            .on_text_changed(callback.clone()),
    );
    let object = runtime.graph().root().unwrap();
    assert!(
        runtime
            .graph()
            .properties(object)
            .unwrap()
            .contains(&Property {
                id: PropertyId::Document,
                value: PropertyValue::String(Rc::from("one\ntwo\nthree")),
            })
    );
    assert!(
        runtime
            .update(
                RichEditBox::new()
                    .text("one\ntwo\nthree")
                    .on_text_changed(callback.clone()),
            )
            .is_empty()
    );

    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Document,
                value: PropertyValue::String(Rc::from("native\ntext")),
            },
        }),
        Some(EventDispatch::new(
            object,
            EventId::TextChanged,
            EventValue::String(callback),
            EventPayload::String(Rc::from("native\ntext")),
        )),
    );
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(&*values.borrow(), &[Rc::from("native\ntext")]);
}

#[test]
fn rich_edit_document_removal_clears_once_and_stays_omitted() {
    let callback = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        RichEditBox::new()
            .text("Before")
            .on_text_changed(callback.clone()),
    );
    let object = runtime.graph().root().unwrap();

    let mutations = runtime.update(RichEditBox::new().on_text_changed(callback.clone()));
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object: changed, clear, .. }
            if *changed == object && clear.as_ref() == [PropertyId::Document]
    )));
    assert!(
        runtime
            .graph()
            .properties(object)
            .unwrap()
            .iter()
            .all(|property| property.id != PropertyId::Document)
    );
    assert!(
        runtime
            .update(RichEditBox::new().on_text_changed(callback))
            .is_empty()
    );
}

#[test]
fn observation_matching_declared_state_is_not_dispatched() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        RichEditBox::new()
            .text("After")
            .on_text_changed(callback.clone()),
    );
    let object = runtime.graph().root().unwrap();
    let mut report = |value: &str| {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Document,
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
        runtime.dispatch_native_events()
    };

    assert_eq!(report("After"), 0);
    assert_eq!(report("After"), 0);
    assert_eq!(report("Different"), 1);
    assert_eq!(report("Other"), 1);
    assert_eq!(report("After"), 0);
    assert_eq!(
        &*values.borrow(),
        &[Rc::from("Different"), Rc::from("Other")]
    );
}

#[test]
fn recorded_value_of_an_omitted_property_is_not_dispatched() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        RichEditBox::new()
            .text("Before")
            .on_text_changed(callback.clone()),
    );
    let object = runtime.graph().root().unwrap();
    runtime.update(RichEditBox::new().on_text_changed(callback.clone()));
    let document = |value: &str| Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Document,
            value: PropertyValue::String(Rc::from(value)),
        },
    };
    let report = |runtime: &mut Runtime<RecordingAdapter>, value: &str| {
        runtime.adapter_mut().queue_native_event(
            Some(document(value)),
            Some(EventDispatch::new(
                object,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
        runtime.dispatch_native_events()
    };

    runtime.adapter_mut().queue_record(document(""));
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(report(&mut runtime, ""), 0);
    assert_eq!(report(&mut runtime, "Typed"), 1);
    assert_eq!(&*values.borrow(), &[Rc::from("Typed")]);
}

#[test]
fn callback_free_notification_does_not_set_a_baseline() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(RichEditBox::new().on_text_changed(callback.clone()));
    let object = runtime.graph().root().unwrap();
    let document = Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Document,
            value: PropertyValue::String(Rc::from("Typed")),
        },
    };
    runtime
        .adapter_mut()
        .queue_native_event(Some(document.clone()), None);
    runtime.adapter_mut().queue_native_event(
        Some(document),
        Some(EventDispatch::new(
            object,
            EventId::TextChanged,
            EventValue::String(callback),
            EventPayload::String(Rc::from("Typed")),
        )),
    );
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(&*values.borrow(), &[Rc::from("Typed")]);
}

#[test]
fn action_observing_settled_state_is_dispatched() {
    let clicks = Rc::new(Cell::new(0));
    let callback = {
        let clicks = Rc::clone(&clicks);
        Callback::new(move |()| clicks.set(clicks.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(CheckBox::new().on_click(callback.clone()));
    let object = runtime.graph().root().unwrap();
    let checked = Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::IsChecked,
            value: PropertyValue::OptionalBool(Some(true)),
        },
    };
    // A callback-free `IsCheckedChanged` reports the state before the `Click` that carries it.
    runtime
        .adapter_mut()
        .queue_native_event(Some(checked.clone()), None);
    for _ in 0..2 {
        runtime.adapter_mut().queue_native_event(
            Some(checked.clone()),
            Some(EventDispatch::new(
                object,
                EventId::Click,
                EventValue::Unit(callback.clone()),
                EventPayload::Unit,
            )),
        );
    }
    assert_eq!(runtime.dispatch_native_events(), 2);
    assert_eq!(clicks.get(), 2);
}

#[test]
fn repeated_state_reports_dispatch_where_the_state_settled() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        RichEditBox::new()
            .text("A")
            .on_text_changed(callback.clone()),
    );
    let object = runtime.graph().root().unwrap();
    for value in ["B", "C", "D"] {
        let observation = Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Document,
                value: PropertyValue::String(Rc::from(value)),
            },
        };
        runtime.adapter_mut().queue_native_event(
            Some(observation.clone()),
            Some(EventDispatch::new(
                object,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
        // A callback-free observation between reports does not stop them coalescing.
        runtime
            .adapter_mut()
            .queue_native_event(Some(observation), None);
    }
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(&*values.borrow(), &[Rc::from("D")]);
}

#[test]
fn accepted_departure_does_not_settle_a_later_edit() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let declare = |text: &str| {
        RichEditBox::new()
            .text(text)
            .on_text_changed(callback.clone())
    };
    runtime.update(declare("A"));
    let object = runtime.graph().root().unwrap();
    let report = |runtime: &mut Runtime<RecordingAdapter>, value: &str| {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Document,
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
        runtime.dispatch_native_events()
    };

    assert_eq!(report(&mut runtime, "B"), 1);
    // The app accepts the edit, so the declaration matches the retained native value.
    assert!(runtime.update(declare("B")).is_empty());
    assert_eq!(report(&mut runtime, "A"), 1);
    assert_eq!(&*values.borrow(), &[Rc::from("B"), Rc::from("A")]);
}

#[test]
fn range_changes_rewrite_the_value_they_coerce() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(RatingControl::new().max_rating(5).value(4.0));
    let object = runtime.graph().root().unwrap();
    let max_rating = |value| Property {
        id: PropertyId::MaxRating,
        value: PropertyValue::I32(value),
    };
    let value = Property {
        id: PropertyId::Value,
        value: PropertyValue::OptionalF64(Some(4.0)),
    };
    let properties = |mutations: UpdateStats| match mutations.as_slice() {
        [
            Mutation::SetProperties {
                object: changed,
                set,
                clear,
            },
        ] if *changed == object => (set.to_vec(), clear.to_vec()),
        mutations => panic!("unexpected mutations {mutations:?}"),
    };

    // The native control clamps the value without reporting it, so the unchanged declared value
    // is written after each range change.
    for range in [2, 5] {
        let mutations = runtime.update(RatingControl::new().max_rating(range).value(4.0));
        assert_eq!(
            properties(mutations),
            (vec![max_rating(range), value.clone()], vec![])
        );
    }
    let mutations = runtime.update(RatingControl::new().value(4.0));
    assert_eq!(
        properties(mutations),
        (vec![value], vec![PropertyId::MaxRating])
    );
    assert!(runtime.update(RatingControl::new().value(4.0)).is_empty());
}
