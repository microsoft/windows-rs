use super::*;

#[test]
fn exit_retirement_leaves_only_native_state_until_completion() {
    let reference: ElementRef = ElementRef::default();
    let calls = Rc::new(Cell::new(0));
    let callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |()| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        Grid::new().keyed_children([
            keyed(
                "item",
                Button::new()
                    .element_ref(&reference)
                    .exit_fade(Duration::from_millis(200))
                    .on_click(callback.clone())
                    .content(TextBlock::new().text("old")),
            ),
            keyed("tail", TextBlock::new().text("tail")),
        ]),
    );
    let root = runtime.graph().root().unwrap();
    let old = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let old_content = runtime.graph().child(old, RelationId::Content).unwrap();
    for _ in 0..2 {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            old,
            EventId::Click,
            EventCall::Unit(callback.clone()),
        ));
    }
    let mut active = runtime.next_native_event().unwrap();
    active.invoke();
    runtime.adapter_mut().record_batches(true);

    let mutations =
        runtime.update(Grid::new().keyed_children([keyed("tail", TextBlock::new().text("tail"))]));
    drop(active);
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(calls.get(), 1);
    runtime.update(Grid::new().keyed_children([
        keyed("item", Button::new().content(TextBlock::new().text("new"))),
        keyed("tail", TextBlock::new().text("tail")),
    ]));
    let replacement = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_ne!(replacement, old);
    assert_eq!(runtime.graph().kind(old), None);
    assert_eq!(runtime.graph().kind(old_content), None);
    assert_eq!(runtime.graph().retired_count(), 1);
    assert_eq!(reference.get(), None);
    assert!(!runtime.focus(&reference));
    assert!(
        mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Retire { root, .. } if *root == old))
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Remove { child, .. } if *child == old))
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Destroy { object } if *object == old))
    );
    assert_eq!(mutations.len(), 2);
    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        &[
            old,
            replacement,
            runtime
                .graph()
                .children(root, RelationId::Children)
                .unwrap()[1]
        ]
    );
    let tail = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[1];
    runtime.update(Grid::new().keyed_children([
        keyed("tail", TextBlock::new().text("tail")),
        keyed("item", Button::new().content(TextBlock::new().text("new"))),
    ]));
    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        &[old, tail, replacement]
    );

    assert!(runtime.adapter_mut().complete_retirement(old));
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert!(matches!(
        runtime.adapter().batches().last().unwrap().as_slice(),
        [Mutation::CompleteRetirement { root, .. }] if *root == old
    ));
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(
        runtime.adapter().object_count(),
        runtime.graph().object_count()
    );
    assert!(!runtime.adapter_mut().complete_retirement(old));
    assert_eq!(runtime.graph().full_reference_scan_count(), 0);
}

#[test]
fn owned_reorder_preserves_interleaved_retirement_slot_in_recording_adapter() {
    let children = |include_retiring: bool, reversed: bool| {
        let mut values = if reversed {
            vec![
                keyed("d", TextBlock::new().text("d")),
                keyed("c", TextBlock::new().text("c")),
                keyed("b", TextBlock::new().text("b")),
                keyed("a", TextBlock::new().text("a")),
            ]
        } else {
            vec![
                keyed("a", TextBlock::new().text("a")),
                keyed("b", TextBlock::new().text("b")),
                keyed("c", TextBlock::new().text("c")),
                keyed("d", TextBlock::new().text("d")),
            ]
        };
        if include_retiring {
            values.insert(
                2,
                keyed(
                    "retiring",
                    TextBlock::new()
                        .text("retiring")
                        .exit_fade(Duration::from_secs(1)),
                ),
            );
        }
        Grid::new().keyed_children(values)
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(children(true, false));
    let root = runtime.graph().root().unwrap();
    let initial = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    let retiring = initial[2];
    let active = [initial[4], initial[3], initial[1], initial[0]];

    runtime.update(children(false, true));

    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        &[active[0], active[1], retiring, active[2], active[3]]
    );
    assert!(runtime.adapter_mut().complete_retirement(retiring));
    runtime.dispatch_native_events();
    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        active
    );
}

#[test]
fn exit_retirement_handles_concurrency_zero_duration_and_parent_removal() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Grid::new().keyed_children([
        keyed("first", Button::new().exit_fade(Duration::from_millis(100))),
        keyed(
            "second",
            Button::new().exit_fade(Duration::from_millis(200)),
        ),
        keyed("tail", TextBlock::new().text("tail")),
    ]));
    let root = runtime.graph().root().unwrap();
    let previous = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();

    runtime.update(Grid::new().keyed_children([keyed("tail", TextBlock::new().text("tail"))]));
    assert_eq!(runtime.graph().retired_count(), 2);
    assert!(runtime.adapter_mut().complete_retirement(previous[1]));
    runtime.dispatch_native_events();
    assert_eq!(runtime.graph().retired_count(), 1);
    assert!(runtime.adapter_mut().complete_retirement(previous[0]));
    runtime.dispatch_native_events();
    assert_eq!(runtime.graph().retired_count(), 0);

    runtime.update(
        Grid::new().keyed_children([keyed("zero", Button::new().exit_fade(Duration::ZERO))]),
    );
    let zero = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new());
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(runtime.graph().kind(zero), None);

    runtime.update(Grid::new().keyed_children([keyed(
        "parent",
        Grid::new().keyed_children([keyed(
            "child",
            Button::new().exit_fade(Duration::from_millis(200)),
        )]),
    )]));
    let parent = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let child = runtime
        .graph()
        .children(parent, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new().keyed_children([keyed("parent", Grid::new())]));
    assert_eq!(runtime.graph().retired_count(), 1);
    runtime.update(Grid::new());
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(runtime.graph().kind(child), None);
    assert!(!runtime.adapter_mut().complete_retirement(child));
}

#[test]
#[should_panic(expected = "exit transitions are not supported for children of relation Content")]
fn exit_transition_rejects_single_child_attachment() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        Button::new().content(
            TextBlock::new()
                .text("old")
                .exit_fade(Duration::from_millis(200)),
        ),
    );

    runtime.update(Button::new());
}

#[test]
#[should_panic(expected = "exit transitions are not supported for children of relation Content")]
fn exit_transition_rejects_nested_child_of_removed_parent() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Grid::new().keyed_children([
        keyed(
            "retiring",
            Button::new().exit_fade(Duration::from_millis(100)),
        ),
        keyed(
            "holder",
            Button::new().content(Button::new().exit_fade(Duration::from_millis(100))),
        ),
    ]));
    runtime.update(Grid::new().keyed_children([keyed("holder", Button::new())]));
}

#[test]
fn duplicate_retirement_completion_is_idempotent() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.adapter_mut().record_batches(true);
    runtime.update(Grid::new().keyed_children([keyed(
        "retiring",
        Button::new().exit_fade(Duration::from_millis(100)),
    )]));
    let root = runtime.graph().root().unwrap();
    let retired = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new());
    runtime.adapter_mut().queue_retirement_completion(retired);
    runtime.adapter_mut().queue_retirement_completion(retired);

    runtime.dispatch_native_events();

    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(
        runtime.adapter().batches().last().unwrap(),
        &[Mutation::CompleteRetirement {
            root: retired,
            nodes: vec![retired],
        }]
    );
}

#[test]
fn mixed_duplicate_and_stale_retirement_completions_are_idempotent() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.adapter_mut().record_batches(true);
    runtime.update(Grid::new().keyed_children([
        keyed("first", Button::new().exit_fade(Duration::from_millis(100))),
        keyed(
            "second",
            Button::new().exit_fade(Duration::from_millis(100)),
        ),
    ]));
    let root = runtime.graph().root().unwrap();
    let retired = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    runtime.update(Grid::new());
    let completion_batches = runtime.adapter().batches().len();

    for object in [retired[0], retired[0], retired[1], retired[1]] {
        runtime.adapter_mut().queue_retirement_completion(object);
    }
    runtime.dispatch_native_events();
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    let completed = runtime.adapter().batches()[completion_batches..]
        .iter()
        .flatten()
        .filter_map(|mutation| match mutation {
            Mutation::CompleteRetirement { root, .. } => Some(*root),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(completed, retired);

    runtime.update(Grid::new().keyed_children([keyed("first", Button::new())]));
    let replacement = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_ne!(replacement, retired[0]);
    let batches = runtime.adapter().batches().len();
    runtime
        .adapter_mut()
        .queue_retirement_completion(retired[0]);
    runtime
        .adapter_mut()
        .queue_retirement_completion(retired[0]);
    runtime.dispatch_native_events();
    assert_eq!(runtime.adapter().batches().len(), batches);
    assert_eq!(runtime.graph().kind(replacement), Some(ObjectType::Button));
}

#[test]
fn native_queue_retirement_boundary_precedes_invalid_observation() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(Grid::new().width(100.0).keyed_children([keyed(
        "retiring",
        Button::new().exit_fade(Duration::from_millis(100)),
    )]));
    let root = runtime.graph().root().unwrap();
    let retiring = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new().width(100.0));
    assert_eq!(runtime.graph().retired_count(), 1);
    assert!(runtime.adapter_mut().complete_retirement(retiring));
    for property in [
        Property {
            id: PropertyId::Width,
            value: PropertyValue::F64(200.0),
        },
        Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Invalid")),
        },
    ] {
        runtime.adapter_mut().observe(Observation::SetProperty {
            object: root,
            property,
        });
    }

    assert!(matches!(
        runtime.next_native_work(),
        Some(NativeWork::Maintenance)
    ));
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_panics("invalid property Text for Grid", || {
        runtime.prepare_update();
    });
}

#[test]
fn native_queue_retirement_cleanup_does_not_consume_later_callback() {
    let calls = Rc::new(Cell::new(0));
    let called = Rc::clone(&calls);
    let callback = Callback::new(move |()| called.set(called.get() + 1));
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        Grid::new().keyed_children([
            keyed(
                "retiring",
                TextBox::new()
                    .text("Retiring")
                    .exit_fade(Duration::from_millis(100)),
            ),
            keyed("active", Button::new().on_click(callback.clone())),
        ]),
    );
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    let (retiring, active) = (children[0], children[1]);
    runtime.update(
        Grid::new().keyed_children([keyed("active", Button::new().on_click(callback.clone()))]),
    );
    runtime.adapter_mut().queue_retirement_completion(retiring);
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: retiring,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Stale")),
        },
    });
    runtime.adapter_mut().queue_event(EventDispatch::new(
        active,
        EventId::Click,
        EventCall::Unit(callback),
    ));

    assert!(!runtime.prepare_update());
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    let Some(NativeWork::Event(mut event)) = runtime.next_native_work() else {
        panic!("expected callback after native cleanup");
    };
    event.invoke();
    drop(event);
    assert_eq!(calls.get(), 1);
}

#[test]
fn native_event_before_retirement_completion_keeps_chronological_order() {
    let calls = Rc::new(Cell::new(0));
    let callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |()| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Grid::new().keyed_children([
        keyed(
            "retiring",
            Button::new().exit_fade(Duration::from_millis(100)),
        ),
        keyed("active", Button::new().on_click(callback.clone())),
    ]));
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    runtime.update(
        Grid::new().keyed_children([keyed("active", Button::new().on_click(callback.clone()))]),
    );
    runtime.adapter_mut().queue_event(EventDispatch::new(
        children[1],
        EventId::Click,
        EventCall::Unit(callback),
    ));
    runtime
        .adapter_mut()
        .queue_retirement_completion(children[0]);

    let mut event = runtime.next_native_event().unwrap();
    assert_eq!(runtime.graph().retired_count(), 1);
    event.invoke();
    drop(event);
    assert_eq!(calls.get(), 1);
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(runtime.graph().retired_count(), 0);
}

#[test]
fn retirement_completion_before_later_event_keeps_chronological_order() {
    let calls = Rc::new(Cell::new(0));
    let callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |()| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Grid::new().keyed_children([
        keyed(
            "retiring",
            Button::new().exit_fade(Duration::from_millis(100)),
        ),
        keyed("active", Button::new().on_click(callback.clone())),
    ]));
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    runtime.update(
        Grid::new().keyed_children([keyed("active", Button::new().on_click(callback.clone()))]),
    );
    runtime
        .adapter_mut()
        .queue_retirement_completion(children[0]);
    runtime.adapter_mut().queue_event(EventDispatch::new(
        children[1],
        EventId::Click,
        EventCall::Unit(callback),
    ));

    let mut event = runtime.next_native_event().unwrap();
    assert_eq!(runtime.graph().retired_count(), 0);
    event.invoke();
    drop(event);
    assert_eq!(calls.get(), 1);
    assert_eq!(runtime.dispatch_native_events(), 0);
}

#[test]
fn events_queued_after_logical_retirement_are_stale_before_completion() {
    let calls = Rc::new(Cell::new(0));
    let callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |()| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        Grid::new().keyed_children([keyed(
            "retiring",
            Button::new()
                .exit_fade(Duration::from_millis(100))
                .on_click(callback.clone()),
        )]),
    );
    let root = runtime.graph().root().unwrap();
    let retired = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new());
    runtime.adapter_mut().queue_event(EventDispatch::new(
        retired,
        EventId::Click,
        EventCall::Unit(callback),
    ));
    runtime.adapter_mut().queue_retirement_completion(retired);

    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(calls.get(), 0);
    assert_eq!(runtime.graph().retired_count(), 0);
}

/// Drives random keyed (`Grid`) and positional (`Canvas`) updates with fading children. The
/// recording adapter applies the reconciler's native indices and moves as given and rejects a
/// mismatched removal, a reorder that does not produce the declared order, and a move of a
/// retiring child.
#[test]
fn native_order_tracks_interleaved_retirements() {
    struct Random(u64);
    impl Random {
        fn next(&mut self, bound: usize) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((self.0 >> 33) % bound as u64) as usize
        }
    }
    fn child(random: &mut Random, label: usize) -> View {
        let fade = (random.next(3) != 0).then_some(Duration::from_millis(100));
        let text = label.to_string();
        let button = random.next(2) == 0;
        match fade {
            Some(fade) if button => Button::new().content(text).exit_fade(fade).into(),
            Some(fade) => TextBlock::new().text(text).exit_fade(fade).into(),
            None if button => Button::new().content(text).into(),
            None => TextBlock::new().text(text).into(),
        }
    }
    for seed in 0..64 {
        let mut random = Random(seed);
        let keyed_parent = seed % 2 == 0;
        let mut runtime = Runtime::new(RecordingAdapter::default());
        let mut previous_native = Vec::new();
        for _ in 0..40 {
            let mut keys = (0..8).filter(|_| random.next(3) != 0).collect::<Vec<_>>();
            for index in (1..keys.len()).rev() {
                keys.swap(index, random.next(index + 1));
            }
            let view: View = if keyed_parent {
                Grid::new()
                    .keyed_children(
                        keys.iter()
                            .map(|key| keyed(*key, child(&mut random, *key)))
                            .collect::<Vec<_>>(),
                    )
                    .into()
            } else {
                Canvas::new()
                    .children(
                        keys.iter()
                            .map(|key| child(&mut random, *key))
                            .collect::<Vec<_>>(),
                    )
                    .into()
            };
            runtime.update(view);
            let root = runtime.graph().root().unwrap();
            let active = runtime
                .graph()
                .children(root, RelationId::Children)
                .unwrap()
                .to_vec();
            let native = runtime
                .adapter()
                .children(root, RelationId::Children)
                .unwrap()
                .to_vec();
            assert_eq!(
                native
                    .iter()
                    .copied()
                    .filter(|child| active.contains(child))
                    .collect::<Vec<_>>(),
                active
            );
            let retiring = native
                .iter()
                .copied()
                .filter(|child| !active.contains(child))
                .collect::<Vec<_>>();
            assert_eq!(retiring.len(), runtime.adapter().retirement_count());
            assert_eq!(retiring.len(), runtime.graph().retired_count());
            // Retiring children keep their relative native order.
            let mut previous = previous_native.iter();
            assert!(
                retiring
                    .iter()
                    .all(|child| previous.any(|previous| previous == child))
            );
            for child in &retiring {
                if random.next(3) == 0 {
                    assert!(runtime.adapter_mut().complete_retirement(*child));
                }
            }
            runtime.dispatch_native_events();
            previous_native = runtime
                .adapter()
                .children(root, RelationId::Children)
                .unwrap()
                .to_vec();
        }
    }
}

#[test]
fn reorder_moves_active_children_around_retiring_slots() {
    let view = |keys: &[&str]| {
        Grid::new().keyed_children(
            keys.iter()
                .map(|key| {
                    keyed(
                        *key,
                        TextBlock::new()
                            .text(*key)
                            .exit_fade(Duration::from_millis(100)),
                    )
                })
                .collect::<Vec<_>>(),
        )
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(view(&["a", "r", "b"]));
    let root = runtime.graph().root().unwrap();
    let [a, r, b] = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .try_into()
        .unwrap();
    runtime.update(view(&["a", "b"]));
    runtime.adapter_mut().record_batches(true);

    runtime.update(view(&["b", "a"]));

    // Each active child crosses the retiring slot, which stays at native index 1.
    let [
        Mutation::Reorder {
            moves, children, ..
        },
    ] = runtime.adapter().batches().last().unwrap().as_slice()
    else {
        panic!("expected one reorder");
    };
    assert_eq!(children, &[b, r, a]);
    assert!(moves.iter().all(|movement| movement.child != r));
    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        &[b, r, a]
    );
}

#[test]
fn replacing_a_parent_completes_its_child_retirements() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Grid::new().children((StackPanel::new().keyed_children([
        keyed(
            "fading",
            Button::new().exit_fade(Duration::from_millis(100)),
        ),
        keyed("kept", Button::new()),
    ]),)));
    let root = runtime.graph().root().unwrap();
    let panel = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let fading = runtime
        .graph()
        .children(panel, RelationId::Children)
        .unwrap()[0];
    runtime.update_subtree(
        panel,
        StackPanel::new().keyed_children([keyed("kept", Button::new())]),
    );
    assert_eq!(runtime.graph().retired_count(), 1);

    runtime.update_subtree(panel, Grid::new().children((Button::new(),)));

    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(runtime.graph().kind(fading), None);
    assert!(!runtime.adapter_mut().complete_retirement(fading));
}

#[test]
fn direct_removal_tracks_native_slots() {
    let view = |keys: &[&str]| {
        Grid::new().keyed_children(
            keys.iter()
                .map(|key| {
                    keyed(
                        *key,
                        TextBlock::new()
                            .text(*key)
                            .exit_fade(Duration::from_millis(100)),
                    )
                })
                .collect::<Vec<_>>(),
        )
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(view(&["a", "b", "c"]));
    let root = runtime.graph().root().unwrap();
    let [a, b, c] = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .try_into()
        .unwrap();

    runtime.remove_child(root, RelationId::Children, a);
    runtime.remove_child(root, RelationId::Children, b);
    runtime.update(view(&["d", "c"]));

    let d = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        &[a, b, d, c]
    );
}
