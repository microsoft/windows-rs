use super::*;
use crate::Element;

#[test]
fn adding_a_hook_after_mount_panics() {
    let enabled = Rc::new(RefCell::new(None::<State<bool>>));
    let enabled_for_render = Rc::clone(&enabled);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *enabled_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            _ = cx.use_state(|| 1usize);
        }
        text_block("hooks")
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let state = enabled.borrow().as_ref().unwrap().clone();
    assert!(state.try_set(true));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));

    assert!(result.is_err());
    _ = state;
}

#[test]
fn removing_a_hook_panics() {
    let enabled = Rc::new(RefCell::new(None::<State<bool>>));
    let trailing = Rc::new(RefCell::new(None::<State<usize>>));
    let enabled_for_render = Rc::clone(&enabled);
    let trailing_for_render = Rc::clone(&trailing);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *enabled_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            let value = cx.use_state(|| 1usize);
            *trailing_for_render.borrow_mut() = Some(value);
        }
        text_block("hooks")
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let enabled = enabled.borrow().as_ref().unwrap().clone();
    let trailing = trailing.borrow().as_ref().unwrap().clone();
    assert!(enabled.try_set(false));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));

    assert!(result.is_err());
    _ = (enabled, trailing);
}

#[test]
fn changing_hook_kind_with_the_same_value_type_is_rejected() {
    let use_reference = Rc::new(RefCell::new(None::<State<bool>>));
    let use_reference_for_render = Rc::clone(&use_reference);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *use_reference_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            _ = cx.use_ref(|| 0usize);
        } else {
            _ = cx.use_state(|| 0usize);
        }
        text_block("hooks")
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert!(use_reference.borrow().as_ref().unwrap().try_set(true));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));

    assert!(result.is_err());
}

#[test]
fn changing_memo_or_effect_hook_kind_is_rejected() {
    for use_effect in [false, true] {
        let changed = Rc::new(RefCell::new(None::<State<bool>>));
        let changed_for_render = Rc::clone(&changed);
        let root = component(move |cx| {
            let state = cx.use_state(|| false);
            *changed_for_render.borrow_mut() = Some(state.clone());
            if use_effect {
                if state.get().unwrap() {
                    cx.use_effect(0usize, || {});
                } else {
                    _ = cx.use_state(|| 0usize);
                }
            } else if state.get().unwrap() {
                _ = cx.use_memo((), || 0usize);
            } else {
                _ = cx.use_state(|| 0usize);
            }
            text_block("hooks")
        });
        let mut reactor = Reactor::new(RecordingRuntime::default(), root);
        reactor.pump();
        assert!(changed.borrow().as_ref().unwrap().try_set(true));

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));

        assert!(result.is_err());
    }
}

#[test]
fn component_identity_follows_the_render_closure_type() {
    fn first(node: Rc<Cell<Option<NodeId>>>) -> Element {
        component(move |cx| {
            let state = cx.use_state(|| 1usize);
            node.set(Some(state.node()));
            text_block("first")
        })
    }

    fn second(node: Rc<Cell<Option<NodeId>>>) -> Element {
        component(move |cx| {
            let state = cx.use_state(|| "second");
            node.set(Some(state.node()));
            text_block(state.get().unwrap())
        })
    }

    let alternate = Rc::new(RefCell::new(None::<State<bool>>));
    let first_node = Rc::new(Cell::new(None));
    let second_node = Rc::new(Cell::new(None));
    let alternate_for_render = Rc::clone(&alternate);
    let first_for_render = Rc::clone(&first_node);
    let second_for_render = Rc::clone(&second_node);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *alternate_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            second(Rc::clone(&second_for_render))
        } else {
            first(Rc::clone(&first_for_render))
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let old = first_node.get().unwrap();

    assert!(alternate.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let new = second_node.get().unwrap();
    assert_ne!(old, new);
    assert!(!reactor.engine().contains(old));
    assert!(reactor.engine().contains(new));
}

#[test]
fn one_component_callsite_preserves_identity_across_captured_values() {
    fn child(label: &'static str, node: Rc<Cell<Option<NodeId>>>) -> Element {
        component(move |cx| {
            let state = cx.use_state(|| 1usize);
            node.set(Some(state.node()));
            text_block(format!("{label}:{}", state.get().unwrap()))
        })
    }

    let alternate = Rc::new(RefCell::new(None::<State<bool>>));
    let child_node = Rc::new(Cell::new(None));
    let alternate_for_render = Rc::clone(&alternate);
    let node_for_render = Rc::clone(&child_node);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *alternate_for_render.borrow_mut() = Some(state.clone());
        child(
            if state.get().unwrap() {
                "second"
            } else {
                "first"
            },
            Rc::clone(&node_for_render),
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let before = child_node.get().unwrap();

    assert!(alternate.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert_eq!(child_node.get(), Some(before));
    assert!(reactor.engine().contains(before));
}

#[test]
fn fragments_project_children_without_native_nodes() {
    let root = stack_panel([
        text_block("first"),
        fragment([text_block("second"), text_block("third")]),
        fragment([]),
        text_block("fourth"),
    ]);
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    let stack = native_node(&reactor, NativeKind::StackPanel);
    let children = reactor.engine().runtime().children(stack);
    assert_eq!(children.len(), 4);
    assert_eq!(reactor.engine().node_count(), 7);
    let creates = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter(|command| matches!(command, Command::Create { .. }))
        .count();
    assert_eq!(creates, 5);
}

#[test]
fn keyed_fragments_move_as_groups_and_preserve_hook_state() {
    let order = Rc::new(RefCell::new(None::<State<Vec<u64>>>));
    let nodes = Rc::new(RefCell::new(BTreeMap::new()));
    let order_for_render = Rc::clone(&order);
    let nodes_for_render = Rc::clone(&nodes);
    let root = component(move |cx| {
        let state = cx.use_state(|| vec![1, 2]);
        *order_for_render.borrow_mut() = Some(state.clone());
        stack_panel(state.get().unwrap().into_iter().map(|key| {
            let nodes = Rc::clone(&nodes_for_render);
            fragment([
                component(move |cx| {
                    let value = cx.use_state(|| key * 10);
                    nodes.borrow_mut().insert(key, value.node());
                    text_block(format!("{key}:{}", value.get().unwrap()))
                }),
                text_block(format!("{key}:tail")),
            ])
            .key(key)
        }))
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let before = nodes.borrow().clone();
    let stack = native_node(&reactor, NativeKind::StackPanel);
    let native_before = reactor.engine().runtime().children(stack).to_vec();
    let batches = reactor.engine().runtime().batches().len();

    assert!(order.borrow().as_ref().unwrap().try_set(vec![2, 1]));
    reactor.pump();

    assert_eq!(*nodes.borrow(), before);
    assert_eq!(
        reactor.engine().runtime().children(stack),
        [
            native_before[2],
            native_before[3],
            native_before[0],
            native_before[1],
        ]
    );
    let commands = reactor.engine().runtime().batches()[batches..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    assert_eq!(
        commands
            .iter()
            .filter(|command| matches!(command, Command::Move { .. }))
            .count(),
        2
    );
}

#[test]
fn fragment_can_transition_between_empty_and_populated() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *visible_for_render.borrow_mut() = Some(state.clone());
        stack_panel([if state.get().unwrap() {
            fragment([text_block("first"), text_block("second")])
        } else {
            fragment([])
        }])
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stack = native_node(&reactor, NativeKind::StackPanel);
    assert!(reactor.engine().runtime().children(stack).is_empty());

    assert!(visible.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().children(stack).len(), 2);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert!(reactor.engine().runtime().children(stack).is_empty());
}

#[test]
fn application_root_fragment_must_project_exactly_one_native_node() {
    let mut valid = Reactor::new(
        RecordingRuntime::default(),
        fragment([fragment([text_block("root")])]),
    );
    valid.pump();
    assert!(valid.engine().is_valid());

    for root in [
        fragment([]),
        fragment([text_block("first"), text_block("second")]),
    ] {
        let mut invalid = Reactor::new(RecordingRuntime::default(), root);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| invalid.pump()));
        assert!(result.is_err());
    }
}

#[test]
fn replacing_a_component_native_root_updates_the_window_root() {
    let wrapped = Rc::new(RefCell::new(None::<State<bool>>));
    let wrapped_for_render = Rc::clone(&wrapped);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *wrapped_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            border(text_block("after"))
        } else {
            text_block("before")
        }
    });
    let root = Application::new([Window::new("Main", root, || {}).build()]).build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let window = reactor.engine().runtime().window_ids()[0];
    let window_root = reactor.engine().runtime().window_content(window).unwrap();
    let old_root = native_node(&reactor, NativeKind::TextBlock);
    let batches = reactor.engine().runtime().batches().len();

    assert!(wrapped.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let commands = reactor.engine().runtime().batches()[batches..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    let new_root = commands
        .iter()
        .find_map(|command| match command {
            Command::Create {
                id,
                kind: NativeKind::Border,
            } => Some(*id),
            _ => None,
        })
        .unwrap();
    assert!(
        commands
            .iter()
            .all(|command| !matches!(command, Command::SetWindowContent { .. }))
    );
    assert_eq!(
        reactor.engine().runtime().window_content(window),
        Some(window_root)
    );
    assert!(!reactor.engine().runtime().contains(old_root));
    assert!(reactor.engine().runtime().contains(new_root));
}

#[test]
fn root_projection_is_revalidated_after_component_updates() {
    for replacement in [0usize, 2] {
        let count = Rc::new(RefCell::new(None::<State<usize>>));
        let count_for_render = Rc::clone(&count);
        let root = component(move |cx| {
            let state = cx.use_state(|| 1usize);
            *count_for_render.borrow_mut() = Some(state.clone());
            fragment((0..state.get().unwrap()).map(|index| text_block(index.to_string())))
        });
        let mut reactor = Reactor::new(RecordingRuntime::default(), root);
        reactor.pump();

        assert!(count.borrow().as_ref().unwrap().try_set(replacement));

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));
        assert!(result.is_err());
    }
}

#[test]
fn child_state_rerenders_only_the_owning_component() {
    let child_state = Rc::new(RefCell::new(None::<State<usize>>));
    let child_state_for_render = Rc::clone(&child_state);
    let app_renders = Rc::new(Cell::new(0));
    let app_renders_for_render = Rc::clone(&app_renders);
    let child_renders = Rc::new(Cell::new(0));
    let child_renders_for_render = Rc::clone(&child_renders);
    let root = component(move |_| {
        app_renders_for_render.set(app_renders_for_render.get() + 1);
        let child_state = Rc::clone(&child_state_for_render);
        let child_renders = Rc::clone(&child_renders_for_render);
        component(move |cx| {
            child_renders.set(child_renders.get() + 1);
            let state = cx.use_state(|| 0);
            *child_state.borrow_mut() = Some(state.clone());
            text_block(format!("Child {}", state.get().unwrap()))
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(child_state.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();

    assert_eq!(app_renders.get(), 1);
    assert_eq!(child_renders.get(), 2);
}

#[test]
fn state_value_update_and_stale_access_have_distinct_semantics() {
    let state = Rc::new(RefCell::new(None::<State<usize>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let current = cx.use_state(|| 1usize);
        *state_for_render.borrow_mut() = Some(current.clone());
        text_block(current.value().to_string())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let current = state.borrow().as_ref().unwrap().clone();
    assert_eq!(current.value(), 1);
    assert_eq!(current.try_value(), Some(1));
    assert!(current.try_update(|value| *value += 2));
    reactor.pump();
    assert_eq!(current.value(), 3);

    drop(reactor);
    assert_eq!(current.try_value(), None);
    assert!(!current.try_update(|value| *value += 1));
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| current.value())).is_err());
}

#[test]
fn component_state_updates_native_properties_and_item_count() {
    let state = Rc::new(RefCell::new(None::<State<usize>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let count = cx.use_state(|| 10usize);
        *state_for_render.borrow_mut() = Some(count.clone());
        let count = count.get().unwrap();
        stack_panel([
            border(text_block(format!("Rows: {count}"))),
            virtual_list(count, 300.0, |index| text_block(format!("Row {index}"))),
        ])
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();
    let batches = reactor.engine().runtime().batches();
    assert!(
        batches
            .iter()
            .flatten()
            .any(|command| item_count_update(command).is_some_and(|(_, count)| count == 10))
    );

    assert!(state.borrow().as_ref().unwrap().try_set(25));
    reactor.pump();
    let last = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        last.iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "Rows: 25"))
    );
    assert!(
        last.iter()
            .any(|command| item_count_update(command).is_some_and(|(_, count)| count == 25))
    );
}

#[test]
fn selector_controlled_values_update_clear_and_skip_unchanged_props() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let handler_version = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let handler_for_render = Rc::clone(&handler_version);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let handler_version = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(phase.clone());
        *handler_for_render.borrow_mut() = Some(handler_version.clone());
        let phase = phase.value();
        let handler_version = handler_version.value();
        let list_selection = match phase {
            0 => CollectionSelection::new([10]),
            1 => CollectionSelection::new([20, 30]),
            _ => CollectionSelection::default(),
        };
        let single_selection = match phase {
            0 => Some(10),
            1 => Some(30),
            _ => None,
        };
        let list_mode = match phase {
            0 => SelectionMode::Multiple,
            1 => SelectionMode::Extended,
            _ => SelectionMode::Single,
        };
        StackPanel::new([
            ListBox::new([(10, "Ten"), (20, "Twenty"), (30, "Thirty")], move |_| {
                _ = handler_version;
            })
            .selection_mode(list_mode)
            .selection(list_selection)
            .build(),
            ComboBox::new([(10, "Ten"), (20, "Twenty"), (30, "Thirty")], |_| {})
                .selected_key(single_selection)
                .build(),
            RadioButtons::new([(10, "Ten"), (20, "Twenty"), (30, "Thirty")], |_| {})
                .selected_key(single_selection)
                .build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let list = native_node(&reactor, NativeKind::ListBox);
    let combo = native_node(&reactor, NativeKind::ComboBox);
    let radio = native_node(&reactor, NativeKind::RadioButtons);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ListBox(
                ListBoxUpdate::SelectionMode(SelectionMode::Extended)
            )),
        } if *id == list
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ListBox(
                ListBoxUpdate::Selection(selection)
            )),
        } if *id == list && selection.as_slice() == [20, 30]
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ComboBox(update)),
        } if *id == combo
            && matches!(
                update.as_ref(),
                ComboBoxUpdate::Selection(Some(30))
            )
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::RadioButtons(update)),
        } if *id == radio
            && matches!(
                update,
                RadioButtonsUpdate::Selection(Some(30))
            )
    )));

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(handler_version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert!(
        reactor.engine().runtime().batches()[batch_count..]
            .iter()
            .flatten()
            .all(|command| !matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(
                        ControlUpdate::ListBox(_)
                            | ControlUpdate::ComboBox(_)
                            | ControlUpdate::RadioButtons(_)
                    ),
                    ..
                }
            ))
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ListBox(
                ListBoxUpdate::SelectionMode(SelectionMode::Single)
            )),
        } if *id == list
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ListBox(
                ListBoxUpdate::Selection(selection)
            )),
        } if *id == list && selection.is_empty()
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ComboBox(update)),
        } if *id == combo && matches!(update.as_ref(), ComboBoxUpdate::Selection(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::RadioButtons(update)),
        } if *id == radio && matches!(update, RadioButtonsUpdate::Selection(None))
    )));
}

#[test]
fn rejected_selection_feedback_is_restored_on_rerender() {
    let revision = Rc::new(RefCell::new(None::<State<usize>>));
    let revision_for_render = Rc::clone(&revision);
    let events = Rc::new(Cell::new(0usize));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *revision_for_render.borrow_mut() = Some(state.clone());
        _ = state.value();
        macro_rules! callback {
            () => {{
                let events = Rc::clone(&events_for_render);
                move |_| events.set(events.get() + 1)
            }};
        }
        StackPanel::new([
            ListBox::new([(1, "one"), (2, "two")], callback!())
                .selection(CollectionSelection::new([1]))
                .build(),
            ComboBox::new([(1, "one"), (2, "two")], callback!())
                .selected_key(Some(1))
                .build(),
            RadioButtons::new([(1, "one"), (2, "two")], callback!())
                .selected_key(Some(1))
                .build(),
            Pivot::new(
                [
                    PivotItem::new(1, "one", text_block("one")),
                    PivotItem::new(2, "two", text_block("two")),
                ],
                callback!(),
            )
            .selected_index(Some(0))
            .build(),
            SelectorBar::new(
                [
                    SelectorBarItem::new(1, "one"),
                    SelectorBarItem::new(2, "two"),
                ],
                callback!(),
            )
            .selected_key(Some(1))
            .build(),
            NavigationView::new(
                [NavigationItem::new(1, "one"), NavigationItem::new(2, "two")],
                text_block("content"),
                callback!(),
            )
            .selected_key(Some(1))
            .build(),
            VirtualList::new(2, 100.0, |index| text_block(index.to_string()))
                .item_keys(VirtualItemKeys::new([1, 2]))
                .selection(CollectionSelection::new([1]), callback!())
                .build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let list = native_node(&reactor, NativeKind::ListBox);
    let combo = native_node(&reactor, NativeKind::ComboBox);
    let radio = native_node(&reactor, NativeKind::RadioButtons);
    let pivot = native_node(&reactor, NativeKind::Pivot);
    let selector = native_node(&reactor, NativeKind::SelectorBar);
    let navigation = native_node(&reactor, NativeKind::NavigationView);
    let virtual_list = native_node(&reactor, NativeKind::ListView);

    for event in [
        NativeEvent::SelectionChanged {
            target: list,
            selection: CollectionSelection::new([2]),
        },
        NativeEvent::SelectedKeyChanged {
            target: combo,
            key: Some(2),
        },
        NativeEvent::SelectedKeyChanged {
            target: radio,
            key: Some(2),
        },
        NativeEvent::IndexChanged {
            target: pivot,
            index: Some(1),
        },
        NativeEvent::SelectedKeyChanged {
            target: selector,
            key: Some(2),
        },
        NativeEvent::SelectedKeyChanged {
            target: navigation,
            key: Some(2),
        },
        NativeEvent::SelectionChanged {
            target: virtual_list,
            selection: CollectionSelection::new([2]),
        },
    ] {
        reactor.engine().runtime().queue_event(event);
    }
    reactor.pump();
    assert_eq!(events.get(), 7);

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(revision.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let commands = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .skip(batch_count)
        .flatten()
        .collect::<Vec<_>>();
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ListBox(
                ListBoxUpdate::Selection(value)
            )),
        } if *id == list && value.as_slice() == [1]
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ComboBox(update)),
        } if *id == combo && matches!(update.as_ref(), ComboBoxUpdate::Selection(Some(1)))
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::RadioButtons(update)),
        } if *id == radio && matches!(update, RadioButtonsUpdate::Selection(Some(1)))
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::IndexSelector(0)),
        } if *id == pivot
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::SelectorBarSelection(Some(1))),
        } if *id == selector
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::NavigationView(
                NavigationUpdate::Selection(Some(1))
            )),
        } if *id == navigation
    )));
    assert!(commands.iter().any(|command| {
        selection_update(command)
            .is_some_and(|(id, value)| id == virtual_list && value.as_slice() == [1])
    }));
}

#[test]
fn single_key_selection_uses_replacement_callbacks() {
    let version = Rc::new(RefCell::new(None::<State<u64>>));
    let version_for_render = Rc::clone(&version);
    let events = Rc::new(RefCell::new(Vec::new()));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let current = cx.use_state(|| 1u64);
        *version_for_render.borrow_mut() = Some(current.clone());
        let current = current.value();
        let combo_events = Rc::clone(&events_for_render);
        let radio_events = Rc::clone(&events_for_render);
        StackPanel::new([
            ComboBox::new([(1, "one"), (2, "two")], move |key| {
                combo_events.borrow_mut().push(("combo", current, key));
            })
            .selected_key(Some(1))
            .build(),
            RadioButtons::new([(1, "one"), (2, "two")], move |key| {
                radio_events.borrow_mut().push(("radio", current, key));
            })
            .selected_key(Some(1))
            .build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let combo = native_node(&reactor, NativeKind::ComboBox);
    let radio = native_node(&reactor, NativeKind::RadioButtons);

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(
        reactor.engine().runtime().batches()[batch_count..]
            .iter()
            .flatten()
            .all(|command| !matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(
                        ControlUpdate::ComboBox(_) | ControlUpdate::RadioButtons(_)
                    ),
                    ..
                }
            ))
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectedKeyChanged {
            target: combo,
            key: None,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectedKeyChanged {
            target: radio,
            key: Some(2),
        });
    reactor.pump();

    assert_eq!(
        &*events.borrow(),
        &[("combo", 2, None), ("radio", 2, Some(2))]
    );
}

#[test]
fn virtual_host_values_update_clear_and_skip_unchanged_props() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let handler_version = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let handler_for_render = Rc::clone(&handler_version);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let handler_version = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(phase.clone());
        *handler_for_render.borrow_mut() = Some(handler_version.clone());
        let phase = phase.value();
        let handler_version = handler_version.value();
        let list = VirtualList::new(3, if phase == 1 { 420.0 } else { 300.0 }, |index| {
            text_block(index.to_string())
        });
        match phase {
            0 => list
                .automation_name("Initial rows")
                .help_text("Initial help")
                .on_item_invoked(move |_| {
                    _ = handler_version;
                })
                .build(),
            1 => list
                .automation_name("Updated rows")
                .help_text("Updated help")
                .on_item_invoked(move |_| {
                    _ = handler_version;
                })
                .build(),
            _ => list.build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::ListView);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        updates
            .iter()
            .any(|command| height_update(command) == Some((target, Dimension::Pixels(420.0))))
    );
    assert!(updates.iter().any(|command| {
        accessibility_update(command)
            == Some((
                target,
                AccessibilityUpdate::AutomationName("Updated rows".into()),
            ))
    }));
    assert!(updates.iter().any(|command| {
        accessibility_update(command)
            == Some((target, AccessibilityUpdate::HelpText("Updated help".into())))
    }));

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(handler_version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert!(
        reactor.engine().runtime().batches()[batch_count..]
            .iter()
            .flatten()
            .all(|command| {
                height_update(command).is_none()
                    && accessibility_update(command).is_none()
                    && !matches!(
                        command,
                        Command::Update {
                            id,
                            update: NativeUpdate::Control(ControlUpdate::Collection(
                                CollectionUpdate::ItemClickEnabled(_)
                            )),
                        } if *id == target
                    )
            })
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        updates
            .iter()
            .any(|command| height_update(command) == Some((target, Dimension::Pixels(300.0))))
    );
    assert!(updates.iter().any(|command| {
        accessibility_update(command)
            == Some((target, AccessibilityUpdate::AutomationName(String::new())))
    }));
    assert!(updates.iter().any(|command| {
        accessibility_update(command)
            == Some((target, AccessibilityUpdate::HelpText(String::new())))
    }));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::Collection(
                CollectionUpdate::ItemClickEnabled(false)
            )),
        } if *id == target
    )));
}

#[test]
fn virtual_list_selection_mode_mounts_updates_and_skips_unchanged_values() {
    let mode = Rc::new(RefCell::new(None::<State<SelectionMode>>));
    let mode_for_render = Rc::clone(&mode);
    let root = component(move |cx| {
        let state = cx.use_state(|| SelectionMode::Multiple);
        *mode_for_render.borrow_mut() = Some(state.clone());
        VirtualList::new(100, 300.0, |index| text_block(format!("Row {index}")))
            .selection_mode(state.get().unwrap())
            .selection(CollectionSelection::default(), |_| {})
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| selection_mode_update(command)
                .is_some_and(|(_, value)| value == SelectionMode::Multiple))
    );

    assert!(
        mode.borrow()
            .as_ref()
            .unwrap()
            .try_set(SelectionMode::Extended)
    );
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(selection_mode_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [SelectionMode::Extended]
    );

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(
        mode.borrow()
            .as_ref()
            .unwrap()
            .try_set(SelectionMode::Extended)
    );
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .skip(batch_count)
            .flatten()
            .all(|command| selection_mode_update(command).is_none())
    );
}

#[test]
fn virtual_list_invocation_uses_stable_keys_and_current_handlers() {
    let keys = Rc::new(RefCell::new(None::<State<VirtualItemKeys>>));
    let version = Rc::new(RefCell::new(None::<State<u64>>));
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let invoked = Rc::new(RefCell::new(Vec::new()));
    let keys_for_render = Rc::clone(&keys);
    let version_for_render = Rc::clone(&version);
    let visible_for_render = Rc::clone(&visible);
    let invoked_for_render = Rc::clone(&invoked);
    let root = component(move |cx| {
        let item_keys = cx.use_state(|| VirtualItemKeys::new([10, 20, 30]));
        let handler_version = cx.use_state(|| 1u64);
        let is_visible = cx.use_state(|| true);
        *keys_for_render.borrow_mut() = Some(item_keys.clone());
        *version_for_render.borrow_mut() = Some(handler_version.clone());
        *visible_for_render.borrow_mut() = Some(is_visible.clone());
        if !is_visible.get().unwrap() {
            return text_block("removed");
        }
        let item_keys = item_keys.get().unwrap();
        let count = item_keys.len();
        let invoked = Rc::clone(&invoked_for_render);
        let version = handler_version.get().unwrap();
        VirtualList::new(count, 300.0, |index| text_block(format!("Row {index}")))
            .item_keys(item_keys)
            .on_item_invoked(move |key| invoked.borrow_mut().push((version, key)))
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| items_update(command).map(|(id, _)| id))
        .unwrap();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemInvoked {
            target: host,
            key: 20,
        });
    reactor.pump();

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    assert!(
        keys.borrow()
            .as_ref()
            .unwrap()
            .try_set(VirtualItemKeys::new([30, 10, 20]))
    );
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| items_update(command).is_some_and(|(_, keys)| keys == [30, 10, 20]))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemInvoked {
            target: host,
            key: 20,
        });
    reactor.pump();
    assert_eq!(&*invoked.borrow(), &[(1, 20), (2, 20)]);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemInvoked {
            target: host,
            key: 20,
        });
    reactor.pump();
    assert_eq!(&*invoked.borrow(), &[(1, 20), (2, 20)]);
}

#[test]
fn virtual_list_selection_is_controlled_by_stable_keys() {
    let keys = Rc::new(RefCell::new(None::<State<VirtualItemKeys>>));
    let selection = Rc::new(RefCell::new(None::<State<CollectionSelection>>));
    let version = Rc::new(RefCell::new(None::<State<u64>>));
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let changed = Rc::new(RefCell::new(Vec::new()));
    let keys_for_render = Rc::clone(&keys);
    let selection_for_render = Rc::clone(&selection);
    let version_for_render = Rc::clone(&version);
    let visible_for_render = Rc::clone(&visible);
    let changed_for_render = Rc::clone(&changed);
    let root = component(move |cx| {
        let item_keys = cx.use_state(|| VirtualItemKeys::new([10, 20, 30]));
        let selected = cx.use_state(|| CollectionSelection::new([20]));
        let handler_version = cx.use_state(|| 1u64);
        let is_visible = cx.use_state(|| true);
        *keys_for_render.borrow_mut() = Some(item_keys.clone());
        *selection_for_render.borrow_mut() = Some(selected.clone());
        *version_for_render.borrow_mut() = Some(handler_version.clone());
        *visible_for_render.borrow_mut() = Some(is_visible.clone());
        if !is_visible.get().unwrap() {
            return text_block("removed");
        }
        let item_keys = item_keys.get().unwrap();
        let count = item_keys.len();
        let changed = Rc::clone(&changed_for_render);
        let version = handler_version.get().unwrap();
        VirtualList::new(count, 300.0, |index| text_block(format!("Row {index}")))
            .item_keys(item_keys)
            .selection_mode(SelectionMode::Multiple)
            .selection(selected.get().unwrap(), move |selection| {
                changed
                    .borrow_mut()
                    .push((version, selection.as_slice().to_vec()));
            })
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| selection_update(command).map(|(id, _)| id))
        .unwrap();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| selection_update(command)
                .is_some_and(|(_, value)| value.as_slice() == [20]))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectionChanged {
            target: host,
            selection: CollectionSelection::new([30, 10, 30]),
        });
    reactor.pump();
    assert_eq!(&*changed.borrow(), &[(1, vec![10, 30])]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    assert!(
        selection
            .borrow()
            .as_ref()
            .unwrap()
            .try_set(CollectionSelection::new([30]))
    );
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| selection_update(command)
                .is_some_and(|(_, value)| value.as_slice() == [30]))
    );

    assert!(
        keys.borrow()
            .as_ref()
            .unwrap()
            .try_set(VirtualItemKeys::new([30, 10, 20]))
    );
    reactor.pump();
    let last = reactor.engine().runtime().batches().last().unwrap();
    let item_position = last
        .iter()
        .position(|command| items_update(command).is_some())
        .unwrap();
    let selection_position = last
        .iter()
        .position(|command| selection_update(command).is_some())
        .unwrap();
    assert!(item_position < selection_position);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectionChanged {
            target: host,
            selection: CollectionSelection::new([10, 30]),
        });
    reactor.pump();
    assert_eq!(&*changed.borrow(), &[(1, vec![10, 30]), (2, vec![10, 30])]);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectionChanged {
            target: host,
            selection: CollectionSelection::new([20]),
        });
    reactor.pump();
    assert_eq!(&*changed.borrow(), &[(1, vec![10, 30]), (2, vec![10, 30])]);
}

#[test]
#[should_panic(expected = "VirtualList item keys must be unique")]
fn virtual_list_rejects_duplicate_item_keys() {
    _ = VirtualList::new(2, 300.0, |_| text_block("row"))
        .item_keys(VirtualItemKeys::new([7, 7]))
        .build();
}

#[test]
#[should_panic(expected = "VirtualList single-selection mode accepts at most one selected key")]
fn virtual_list_rejects_multiple_keys_in_single_selection_mode() {
    _ = VirtualList::new(2, 300.0, |_| text_block("row"))
        .selection(CollectionSelection::new([1, 2]), |_| {})
        .build();
}

#[test]
#[should_panic(expected = "VirtualList selection must be empty when selection mode is None")]
fn virtual_list_rejects_selection_in_none_mode() {
    _ = VirtualList::new(1, 300.0, |_| text_block("row"))
        .selection(CollectionSelection::new([1]), |_| {})
        .selection_mode(SelectionMode::None)
        .build();
}

#[test]
fn virtual_grid_reuses_collection_identity_selection_and_invocation() {
    let keys = Rc::new(RefCell::new(None::<State<VirtualItemKeys>>));
    let row_nodes = Rc::new(RefCell::new(BTreeMap::new()));
    let selected = Rc::new(RefCell::new(Vec::new()));
    let invoked = Rc::new(RefCell::new(Vec::new()));
    let keys_for_render = Rc::clone(&keys);
    let row_nodes_for_render = Rc::clone(&row_nodes);
    let selected_for_render = Rc::clone(&selected);
    let invoked_for_render = Rc::clone(&invoked);
    let root = component(move |cx| {
        let item_keys = cx.use_state(|| VirtualItemKeys::new([10, 20, 30]));
        *keys_for_render.borrow_mut() = Some(item_keys.clone());
        let item_keys = item_keys.get().unwrap();
        let row_keys = item_keys.clone();
        let row_nodes = Rc::clone(&row_nodes_for_render);
        let selected = Rc::clone(&selected_for_render);
        let invoked = Rc::clone(&invoked_for_render);
        VirtualGrid::new(item_keys.len(), 300.0, move |index| {
            let key = row_keys.as_slice()[index];
            let row_nodes = Rc::clone(&row_nodes);
            component(move |cx| {
                let local = cx.use_state(|| key);
                row_nodes.borrow_mut().insert(key, local.node());
                text_block(key.to_string())
            })
        })
        .item_keys(item_keys)
        .selection(CollectionSelection::new([20]), move |value| {
            selected.borrow_mut().push(value.as_slice().to_vec());
        })
        .on_item_invoked(move |key| invoked.borrow_mut().push(key))
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = native_node(&reactor, NativeKind::GridView);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host,
            index: 1,
            lease: 1,
        });
    reactor.pump();
    let row = row_nodes.borrow()[&20];

    assert!(
        keys.borrow()
            .as_ref()
            .unwrap()
            .try_set(VirtualItemKeys::new([30, 10, 20]))
    );
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Recycle {
            host,
            index: 1,
            lease: 1,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host,
            index: 2,
            lease: 2,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectionChanged {
            target: host,
            selection: CollectionSelection::new([30]),
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemInvoked {
            target: host,
            key: 30,
        });
    reactor.pump();

    assert_eq!(row_nodes.borrow()[&20], row);
    assert_eq!(&*selected.borrow(), &[vec![30]]);
    assert_eq!(&*invoked.borrow(), &[30]);
}

#[test]
fn list_box_keeps_stable_key_selection_without_virtual_row_ownership() {
    let items = Rc::new(RefCell::new(None::<State<ListBoxItems>>));
    let selected = Rc::new(RefCell::new(Vec::new()));
    let items_for_render = Rc::clone(&items);
    let selected_for_render = Rc::clone(&selected);
    let root = component(move |cx| {
        let current =
            cx.use_state(|| ListBoxItems::new([(10, "Ten"), (20, "Twenty"), (30, "Thirty")]));
        *items_for_render.borrow_mut() = Some(current.clone());
        let selected = Rc::clone(&selected_for_render);
        ListBox::from_items(current.get().unwrap(), move |value| {
            selected.borrow_mut().push(value.as_slice().to_vec());
        })
        .selection_mode(SelectionMode::Multiple)
        .selection(CollectionSelection::new([20]))
        .height(200.0)
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let list = native_node(&reactor, NativeKind::ListBox);
    assert!(
        reactor
            .engine()
            .arena
            .get(list)
            .unwrap()
            .children
            .is_empty()
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectionChanged {
            target: list,
            selection: CollectionSelection::new([10, 30]),
        });
    reactor.pump();
    assert_eq!(&*selected.borrow(), &[vec![10, 30]]);

    let batches = reactor.engine().runtime().batches().len();
    assert!(items.borrow().as_ref().unwrap().try_set(ListBoxItems::new([
        (30, "Thirty"),
        (10, "Ten"),
        (20, "Twenty"),
    ])));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ListBox), list);
    let commands = reactor.engine().runtime().batches()[batches..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    let items = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::ListBox(
                        ListBoxUpdate::Items(_)
                    ))
                } if *id == list
            )
        })
        .unwrap();
    let selection = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::ListBox(
                        ListBoxUpdate::Selection(_)
                    ))
                } if *id == list
            )
        })
        .unwrap();
    assert!(items < selection);
}

#[test]
#[should_panic(expected = "selector item keys must be unique")]
fn list_box_rejects_duplicate_item_keys() {
    _ = ListBox::display([(7, "first"), (7, "second")]).build();
}

#[test]
#[should_panic(expected = "ListBox does not support SelectionMode::None")]
fn list_box_rejects_none_selection_mode() {
    _ = ListBox::display([(1, "one")])
        .selection_mode(SelectionMode::None)
        .build();
}

#[test]
fn combo_box_reuses_keyed_items_and_controlled_selection_without_owned_rows() {
    let items = Rc::new(RefCell::new(None::<State<SelectorItems>>));
    let selected = Rc::new(RefCell::new(Vec::new()));
    let items_for_render = Rc::clone(&items);
    let selected_for_render = Rc::clone(&selected);
    let root = component(move |cx| {
        let current =
            cx.use_state(|| SelectorItems::new([(10, "Ten"), (20, "Twenty"), (30, "Thirty")]));
        *items_for_render.borrow_mut() = Some(current.clone());
        let selected = Rc::clone(&selected_for_render);
        ComboBox::from_items(current.get().unwrap(), move |value| {
            selected.borrow_mut().push(value);
        })
        .selected_key(Some(20))
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let combo = native_node(&reactor, NativeKind::ComboBox);
    assert!(
        reactor
            .engine()
            .arena
            .get(combo)
            .unwrap()
            .children
            .is_empty()
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectedKeyChanged {
            target: combo,
            key: Some(30),
        });
    reactor.pump();
    assert_eq!(&*selected.borrow(), &[Some(30)]);

    let batches = reactor.engine().runtime().batches().len();
    assert!(
        items
            .borrow()
            .as_ref()
            .unwrap()
            .try_set(SelectorItems::new([
                (30, "Thirty"),
                (10, "Ten"),
                (20, "Twenty"),
            ]))
    );
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ComboBox), combo);
    let commands = reactor.engine().runtime().batches()[batches..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    let items = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::ComboBox(update))
                } if *id == combo && matches!(update.as_ref(), ComboBoxUpdate::Items(_))
            )
        })
        .unwrap();
    let selection = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::ComboBox(update))
                } if *id == combo && matches!(update.as_ref(), ComboBoxUpdate::Selection(_))
            )
        })
        .unwrap();
    assert!(items < selection);
}

#[test]
fn combo_box_reconciles_header_placeholder_and_editable_state() {
    let changed = Rc::new(RefCell::new(None::<State<bool>>));
    let changed_for_render = Rc::clone(&changed);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *changed_for_render.borrow_mut() = Some(state.clone());
        let combo = ComboBox::display([(1, "one")]);
        if state.get().unwrap() {
            combo.build()
        } else {
            combo
                .header("Header")
                .placeholder_text("Placeholder")
                .editable(true)
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::ComboBox);

    assert!(changed.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ComboBox(update)),
        } if *id == target && matches!(update.as_ref(), ComboBoxUpdate::Header(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ComboBox(update)),
        } if *id == target && matches!(update.as_ref(), ComboBoxUpdate::Placeholder(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ComboBox(update)),
        } if *id == target && matches!(update.as_ref(), ComboBoxUpdate::Editable(false))
    )));
}

#[test]
fn combo_box_accepts_a_temporarily_stale_selected_key() {
    _ = ComboBox::display([(1, "one"), (2, "two")])
        .selected_key(Some(3))
        .build();
}

#[test]
fn single_key_selectors_reapply_stale_keys_after_item_changes() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let items = match current.value() {
            0 => SelectorItems::new([(10, "Ten"), (20, "Twenty")]),
            1 => SelectorItems::new([(10, "Ten")]),
            _ => SelectorItems::new([(20, "Twenty"), (10, "Ten")]),
        };
        StackPanel::new([
            ComboBox::from_items(items.clone(), |_| {})
                .selected_key(Some(20))
                .build(),
            RadioButtons::from_items(items, |_| {})
                .selected_key(Some(20))
                .build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let combo = native_node(&reactor, NativeKind::ComboBox);
    let radio = native_node(&reactor, NativeKind::RadioButtons);

    for next in [1, 2] {
        assert!(phase.borrow().as_ref().unwrap().try_set(next));
        reactor.pump();
        let commands = reactor.engine().runtime().batches().last().unwrap();
        assert!(commands.iter().any(|command| matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::ComboBox(update)),
            } if *id == combo && matches!(update.as_ref(), ComboBoxUpdate::Selection(Some(20)))
        )));
        assert!(commands.iter().any(|command| matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::RadioButtons(update)),
            } if *id == radio && matches!(update, RadioButtonsUpdate::Selection(Some(20)))
        )));
    }
}

#[test]
fn radio_buttons_translate_native_indices_to_stable_keys() {
    let items = Rc::new(RefCell::new(None::<State<SelectorItems>>));
    let header = Rc::new(RefCell::new(None::<State<Option<String>>>));
    let columns = Rc::new(RefCell::new(None::<State<i32>>));
    let selected = Rc::new(RefCell::new(Vec::new()));
    let items_for_render = Rc::clone(&items);
    let header_for_render = Rc::clone(&header);
    let columns_for_render = Rc::clone(&columns);
    let selected_for_render = Rc::clone(&selected);
    let root = component(move |cx| {
        let current =
            cx.use_state(|| SelectorItems::new([(10, "Ten"), (20, "Twenty"), (30, "Thirty")]));
        let current_header = cx.use_state(|| Some("Numbers".to_string()));
        let current_columns = cx.use_state(|| 1);
        *items_for_render.borrow_mut() = Some(current.clone());
        *header_for_render.borrow_mut() = Some(current_header.clone());
        *columns_for_render.borrow_mut() = Some(current_columns.clone());
        let selected = Rc::clone(&selected_for_render);
        let buttons = RadioButtons::from_items(current.get().unwrap(), move |value| {
            selected.borrow_mut().push(value);
        });
        let buttons = if let Some(header) = current_header.get().unwrap() {
            buttons.header(header)
        } else {
            buttons
        };
        buttons
            .selected_key(Some(20))
            .max_columns(current_columns.get().unwrap())
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let radio = native_node(&reactor, NativeKind::RadioButtons);
    assert!(
        reactor
            .engine()
            .arena
            .get(radio)
            .unwrap()
            .children
            .is_empty()
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectedKeyChanged {
            target: radio,
            key: Some(30),
        });
    reactor.pump();
    assert_eq!(&*selected.borrow(), &[Some(30)]);

    let batches = reactor.engine().runtime().batches().len();
    assert!(
        items
            .borrow()
            .as_ref()
            .unwrap()
            .try_set(SelectorItems::new([
                (30, "Thirty"),
                (10, "Ten"),
                (20, "Twenty"),
            ]))
    );
    assert!(header.borrow().as_ref().unwrap().try_set(None));
    assert!(columns.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::RadioButtons), radio);
    let commands = reactor.engine().runtime().batches()[batches..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    let items = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::RadioButtons(update))
                } if *id == radio && matches!(update, RadioButtonsUpdate::Items(_))
            )
        })
        .unwrap();
    let selection = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::RadioButtons(update))
                } if *id == radio
                    && matches!(update, RadioButtonsUpdate::Selection(_))
            )
        })
        .unwrap();
    assert!(items < selection);
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::RadioButtons(update))
            } if *id == radio
                && matches!(update, RadioButtonsUpdate::MaxColumns(2))
        )
    }));
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::RadioButtons(update))
            } if *id == radio && matches!(update, RadioButtonsUpdate::Header(None))
        )
    }));
}

#[test]
fn radio_buttons_accept_a_temporarily_stale_selected_key() {
    _ = RadioButtons::display([(1, "one"), (2, "two")])
        .selected_key(Some(3))
        .build();
}

#[test]
#[should_panic(expected = "RadioButtons max columns must be positive")]
fn radio_buttons_reject_nonpositive_column_count() {
    _ = RadioButtons::display([(1, "one")]).max_columns(0).build();
}

#[test]
fn radio_button_updates_group_and_dispatches_controlled_checked_state() {
    let checked = Rc::new(RefCell::new(None::<State<bool>>));
    let group = Rc::new(RefCell::new(None::<State<Option<String>>>));
    let checked_for_render = Rc::clone(&checked);
    let group_for_render = Rc::clone(&group);
    let root = component(move |cx| {
        let checked_state = cx.use_state(|| false);
        let group_state = cx.use_state(|| Some("size".to_string()));
        *checked_for_render.borrow_mut() = Some(checked_state.clone());
        *group_for_render.borrow_mut() = Some(group_state.clone());
        let update_checked = checked_state.clone();
        let button = RadioButton::new("Medium", checked_state.value(), move |value| {
            assert!(update_checked.try_set(value));
        });
        match group_state.value() {
            Some(group) => button.group_name(group).build(),
            None => button.build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::RadioButton);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target,
            value: true,
        });
    reactor.pump();
    assert!(checked.borrow().as_ref().unwrap().value());
    assert_eq!(native_node(&reactor, NativeKind::RadioButton), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .all(|command| checked_update(command) != Some((target, true)))
    );

    assert!(group.borrow().as_ref().unwrap().try_set(None));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::RadioButton), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::RadioButtonGroupName(None)),
                } if *id == target
            ))
    );
}

#[test]
fn changing_virtual_collection_kind_replaces_the_native_host() {
    let grid = Rc::new(RefCell::new(None::<State<bool>>));
    let grid_for_render = Rc::clone(&grid);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *grid_for_render.borrow_mut() = Some(value.clone());
        if value.get().unwrap() {
            VirtualGrid::new(3, 300.0, |index| text_block(index.to_string())).build()
        } else {
            VirtualList::new(3, 300.0, |index| text_block(index.to_string())).build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let list = native_node(&reactor, NativeKind::ListView);

    assert!(grid.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let grid = native_node(&reactor, NativeKind::GridView);
    assert_ne!(grid, list);
    assert!(!reactor.engine().runtime().contains(list));
}

#[test]
fn realized_rows_use_the_declarative_row_renderer() {
    let root = component(|_| {
        virtual_list(100, 300.0, |index| {
            text_block(format!("Declarative row {index}"))
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| item_count_update(command).map(|(id, _)| id))
        .unwrap();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host,
            index: 42,
            lease: 1,
        });

    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| {
                text_update(command).is_some_and(|(_, text)| text == "Declarative row 42")
            })
    );
}

#[test]
fn virtual_list_reorder_moves_realized_component_state_by_item_key() {
    let keys = Rc::new(RefCell::new(None::<State<VirtualItemKeys>>));
    let nodes = Rc::new(RefCell::new(BTreeMap::new()));
    let keys_for_render = Rc::clone(&keys);
    let nodes_for_render = Rc::clone(&nodes);
    let root = component(move |cx| {
        let item_keys = cx.use_state(|| VirtualItemKeys::new([10, 20, 30]));
        *keys_for_render.borrow_mut() = Some(item_keys.clone());
        let item_keys = item_keys.get().unwrap();
        let row_keys = item_keys.clone();
        let nodes = Rc::clone(&nodes_for_render);
        VirtualList::new(item_keys.len(), 300.0, move |index| {
            let item_key = row_keys.as_slice()[index];
            let nodes = Rc::clone(&nodes);
            component(move |cx| {
                let local = cx.use_state(|| item_key * 10);
                nodes.borrow_mut().insert(item_key, local.node());
                text_block(format!("{item_key}:{}", local.get().unwrap()))
            })
        })
        .item_keys(item_keys)
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| items_update(command).map(|(id, _)| id))
        .unwrap();
    for index in 0..3 {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Realize {
                host,
                index,
                lease: index as u64 + 1,
            });
    }
    reactor.pump();
    let before = nodes.borrow().clone();

    assert!(
        keys.borrow()
            .as_ref()
            .unwrap()
            .try_set(VirtualItemKeys::new([30, 10, 20]))
    );
    reactor.pump();
    assert!(
        keys.borrow()
            .as_ref()
            .unwrap()
            .try_set(VirtualItemKeys::new([20, 30, 10]))
    );
    reactor.pump();
    for index in 0..3 {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Recycle {
                host,
                index,
                lease: index as u64 + 1,
            });
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Realize {
                host,
                index,
                lease: index as u64 + 11,
            });
    }
    reactor.pump();

    assert_eq!(*nodes.borrow(), before);
    let NodeKind::VirtualHost { realized } = &reactor.engine().arena.get(host).unwrap().kind else {
        unreachable!()
    };
    assert_eq!(
        realized
            .iter()
            .map(|(index, row)| (*index, row.key, row.root))
            .collect::<Vec<_>>(),
        [
            (0, 20, before[&20]),
            (1, 30, before[&30]),
            (2, 10, before[&10]),
        ]
    );

    assert!(
        keys.borrow()
            .as_ref()
            .unwrap()
            .try_set(VirtualItemKeys::new([40, 30, 10]))
    );
    reactor.pump();
    for index in 0..3 {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Recycle {
                host,
                index,
                lease: index as u64 + 11,
            });
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Realize {
                host,
                index,
                lease: index as u64 + 21,
            });
    }
    reactor.pump();
    let after = nodes.borrow().clone();
    assert_eq!(after[&30], before[&30]);
    assert_eq!(after[&10], before[&10]);
    assert_ne!(after[&40], before[&20]);
    assert!(!reactor.engine().contains(before[&20]));
    let NodeKind::VirtualHost { realized } = &reactor.engine().arena.get(host).unwrap().kind else {
        unreachable!()
    };
    assert_eq!(
        realized
            .iter()
            .map(|(index, row)| (*index, row.key, row.root))
            .collect::<Vec<_>>(),
        [
            (0, 40, after[&40]),
            (1, 30, before[&30]),
            (2, 10, before[&10]),
        ]
    );
}

#[test]
fn virtual_list_mutation_retires_unclaimed_parked_rows_after_a_new_window_realizes() {
    let keys = Rc::new(RefCell::new(None::<State<VirtualItemKeys>>));
    let nodes = Rc::new(RefCell::new(BTreeMap::new()));
    let keys_for_render = Rc::clone(&keys);
    let nodes_for_render = Rc::clone(&nodes);
    let root = component(move |cx| {
        let item_keys = cx.use_state(|| VirtualItemKeys::new([0, 1, 2, 3, 4, 5]));
        *keys_for_render.borrow_mut() = Some(item_keys.clone());
        let item_keys = item_keys.get().unwrap();
        let row_keys = item_keys.clone();
        let nodes = Rc::clone(&nodes_for_render);
        VirtualList::new(item_keys.len(), 300.0, move |index| {
            let key = row_keys.as_slice()[index];
            let nodes = Rc::clone(&nodes);
            component(move |cx| {
                let local = cx.use_state(|| key);
                nodes.borrow_mut().insert(key, local.node());
                text_block(key.to_string())
            })
        })
        .item_keys(item_keys)
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| items_update(command).map(|(id, _)| id))
        .unwrap();
    for index in 0..2 {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Realize {
                host,
                index,
                lease: index as u64 + 1,
            });
    }
    reactor.pump();
    let first_window = nodes.borrow().clone();

    assert!(
        keys.borrow()
            .as_ref()
            .unwrap()
            .try_set(VirtualItemKeys::new([2, 3, 4, 5, 0, 1]))
    );
    reactor.pump();
    for index in 0..2 {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Realize {
                host,
                index,
                lease: index as u64 + 11,
            });
    }
    reactor.pump();
    assert_eq!(reactor.engine().parked_virtual_rows(host).len(), 2);

    assert!(
        keys.borrow()
            .as_ref()
            .unwrap()
            .try_set(VirtualItemKeys::new([4, 5, 0, 1, 2, 3]))
    );
    reactor.pump();

    assert_eq!(reactor.engine().parked_virtual_rows(host).len(), 2);
    assert!(!reactor.engine().contains(first_window[&0]));
    assert!(!reactor.engine().contains(first_window[&1]));
}

#[test]
fn virtual_list_empty_state_mounts_reconciles_and_cleans_up_lazily() {
    let count = Rc::new(RefCell::new(None::<State<usize>>));
    let label = Rc::new(RefCell::new(None::<State<String>>));
    let stacked = Rc::new(RefCell::new(None::<State<bool>>));
    let cleanups = Rc::new(Cell::new(0));
    let count_for_render = Rc::clone(&count);
    let label_for_render = Rc::clone(&label);
    let stacked_for_render = Rc::clone(&stacked);
    let cleanups_for_render = Rc::clone(&cleanups);
    let root = component(move |cx| {
        let count = cx.use_state(|| 0);
        let label = cx.use_state(|| "nothing here".to_string());
        let stacked = cx.use_state(|| false);
        *count_for_render.borrow_mut() = Some(count.clone());
        *label_for_render.borrow_mut() = Some(label.clone());
        *stacked_for_render.borrow_mut() = Some(stacked.clone());
        let current_label = label.get().unwrap();
        let current_stacked = stacked.get().unwrap();
        let cleanups = Rc::clone(&cleanups_for_render);
        VirtualList::new(count.get().unwrap(), 300.0, |index| {
            text_block(format!("row {index}"))
        })
        .empty_state(component(move |cx| {
            let cleanups = Rc::clone(&cleanups);
            cx.use_effect_with_cleanup((), move || move || cleanups.set(cleanups.get() + 1));
            if current_stacked {
                stack_panel([text_block(current_label.clone())])
            } else {
                text_block(current_label.clone())
            }
        }))
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| item_count_update(command).map(|(id, _)| id))
        .unwrap();
    let empty = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::Attach {
                parent,
                child,
                attachment: Attachment::Header,
            } if *parent == host => Some(*child),
            _ => None,
        })
        .unwrap();
    assert_eq!(cleanups.get(), 0);

    assert!(
        label
            .borrow()
            .as_ref()
            .unwrap()
            .try_set("still nothing".to_string())
    );
    reactor.pump();
    assert!(reactor.engine().contains(empty));
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command) == Some((empty, "still nothing")))
    );

    assert!(stacked.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    let replacement = reactor
        .engine()
        .runtime()
        .batches()
        .last()
        .unwrap()
        .iter()
        .find_map(|command| match command {
            Command::Attach {
                parent,
                child,
                attachment: Attachment::Header,
            } if *parent == host => Some(*child),
            _ => None,
        })
        .unwrap();
    assert_ne!(replacement, empty);
    assert!(!reactor.engine().contains(empty));
    assert_eq!(cleanups.get(), 0);

    assert!(count.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert!(!reactor.engine().contains(replacement));
    assert_eq!(cleanups.get(), 1);

    assert!(count.borrow().as_ref().unwrap().try_set(0));
    reactor.pump();
    let remounted = reactor
        .engine()
        .runtime()
        .batches()
        .last()
        .unwrap()
        .iter()
        .find_map(|command| match command {
            Command::Attach {
                parent,
                child,
                attachment: Attachment::Header,
            } if *parent == host => Some(*child),
            _ => None,
        })
        .unwrap();
    assert_ne!(remounted, replacement);
}

#[test]
#[should_panic(expected = "NativeParentRejectsChildren")]
fn virtual_list_empty_state_requires_one_native_root() {
    let root = VirtualList::new(0, 300.0, |_| unreachable!())
        .empty_state(fragment([text_block("one"), text_block("two")]))
        .build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();
}

#[test]
fn realized_row_components_can_replace_their_projected_native_root() {
    let stacked = Rc::new(RefCell::new(None::<State<bool>>));
    let stacked_for_render = Rc::clone(&stacked);
    let root = component(move |cx| {
        let stacked = cx.use_state(|| false);
        *stacked_for_render.borrow_mut() = Some(stacked.clone());
        let current = stacked.get().unwrap();
        VirtualList::new(1, 300.0, move |_| {
            component(move |_| {
                if current {
                    stack_panel([text_block("row")])
                } else {
                    text_block("row")
                }
            })
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| item_count_update(command).map(|(id, _)| id))
        .unwrap();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host,
            index: 0,
            lease: 1,
        });
    reactor.pump();
    let NodeKind::VirtualHost { realized } = reactor.engine().node_kind(host).unwrap() else {
        unreachable!()
    };
    let row = realized[&0].root;

    assert!(stacked.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let NodeKind::VirtualHost { realized } = reactor.engine().node_kind(host).unwrap() else {
        unreachable!()
    };
    assert_eq!(realized[&0].root, row);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Attach {
                    parent,
                    attachment: Attachment::VirtualItem {
                        index: 0,
                        lease: 1
                    },
                    ..
                } if *parent == host
            ))
    );
}

#[test]
fn keyed_reorder_preserves_component_nodes_and_hook_state() {
    let order = Rc::new(RefCell::new(None::<State<Vec<u64>>>));
    let nodes = Rc::new(RefCell::new(BTreeMap::new()));
    let order_for_render = Rc::clone(&order);
    let nodes_for_render = Rc::clone(&nodes);
    let root = component(move |cx| {
        let state = cx.use_state(|| vec![1, 2, 3]);
        *order_for_render.borrow_mut() = Some(state.clone());
        stack_panel(state.get().unwrap().into_iter().map(|key| {
            let nodes = Rc::clone(&nodes_for_render);
            component(move |cx| {
                let value = cx.use_state(|| key * 10);
                nodes.borrow_mut().insert(key, value.node());
                text_block(format!("{key}:{}", value.get().unwrap()))
            })
            .key(key)
        }))
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let before = nodes.borrow().clone();
    let batches = reactor.engine().runtime().batches().len();

    assert!(order.borrow().as_ref().unwrap().try_set(vec![3, 1, 2]));
    reactor.pump();

    assert_eq!(*nodes.borrow(), before);
    assert!(before.values().all(|node| reactor.engine().contains(*node)));
    let commands = reactor.engine().runtime().batches()[batches..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    assert_eq!(commands.len(), 1);
    assert!(matches!(commands[0], Command::Move { index: 0, .. }));
}

#[test]
fn unchanged_key_order_reconciles_in_place_without_reorder_commands() {
    let alternate = Rc::new(RefCell::new(None::<State<bool>>));
    let alternate_for_render = Rc::clone(&alternate);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *alternate_for_render.borrow_mut() = Some(state.clone());
        let suffix = if state.get().unwrap() {
            "after"
        } else {
            "before"
        };
        stack_panel((0..3).map(|key| text_block(format!("{key}-{suffix}")).key(key)))
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stack = native_node(&reactor, NativeKind::StackPanel);
    let children = reactor.engine().runtime().children(stack).to_vec();
    let batches = reactor.engine().runtime().batches().len();

    assert!(alternate.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert_eq!(reactor.engine().runtime().children(stack), children);
    let commands = reactor.engine().runtime().batches()[batches..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    assert_eq!(
        commands
            .iter()
            .filter(|command| matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(ControlUpdate::TextBlockText(_)),
                    ..
                }
            ))
            .count(),
        3
    );
    assert!(commands.iter().all(|command| !matches!(
        command,
        Command::Create { .. } | Command::Move { .. } | Command::Destroy { .. }
    )));
}

#[test]
fn keyed_children_mount_and_remove_after_initial_reconciliation() {
    let extra = Rc::new(RefCell::new(None::<State<bool>>));
    let extra_for_render = Rc::clone(&extra);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *extra_for_render.borrow_mut() = Some(state.clone());
        let mut children = vec![text_block("first").key(1)];
        if state.get().unwrap() {
            children.push(text_block("second").key(2));
        }
        StackPanel::new(children).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stack = native_node(&reactor, NativeKind::StackPanel);
    assert_eq!(reactor.engine().arena.get(stack).unwrap().children.len(), 1);

    assert!(extra.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    assert_eq!(reactor.engine().arena.get(stack).unwrap().children.len(), 2);

    assert!(extra.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert_eq!(reactor.engine().arena.get(stack).unwrap().children.len(), 1);
}

#[test]
#[should_panic(expected = "DuplicateSiblingKey { key: 7")]
fn duplicate_sibling_keys_fail_before_initial_commit() {
    let root = StackPanel::new([text_block("first").key(7), text_block("second").key(7)]).build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();
}

#[test]
#[should_panic(expected = "DuplicateSiblingKey { key: 1")]
fn duplicate_sibling_keys_fail_before_retained_mutation() {
    let duplicate = Rc::new(RefCell::new(None::<State<bool>>));
    let duplicate_for_render = Rc::clone(&duplicate);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *duplicate_for_render.borrow_mut() = Some(state.clone());
        let second_key = if state.get().unwrap() { 1 } else { 2 };
        StackPanel::new([
            text_block("first").key(1),
            text_block("second").key(second_key),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let batches = reactor.engine().runtime().batches().len();

    assert!(duplicate.borrow().as_ref().unwrap().try_set(true));
    _ = batches;
    reactor.pump();
}

#[test]
fn row_renderer_updates_already_realized_rows() {
    let prefix = Rc::new(RefCell::new(None::<State<String>>));
    let prefix_for_render = Rc::clone(&prefix);
    let root = component(move |cx| {
        let state = cx.use_state(|| "Before".to_string());
        *prefix_for_render.borrow_mut() = Some(state.clone());
        let prefix = state.get().unwrap();
        virtual_list(100, 300.0, move |index| {
            text_block(format!("{prefix} row {index}"))
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = virtual_host(&reactor);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host,
            index: 7,
            lease: 1,
        });
    reactor.pump();

    assert!(
        prefix
            .borrow()
            .as_ref()
            .unwrap()
            .try_set("After".to_string())
    );
    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "After row 7"))
    );
}

#[test]
fn shrinking_a_virtual_list_removes_out_of_range_rows() {
    let count = Rc::new(RefCell::new(None::<State<usize>>));
    let count_for_render = Rc::clone(&count);
    let root = component(move |cx| {
        let state = cx.use_state(|| 100usize);
        *count_for_render.borrow_mut() = Some(state.clone());
        virtual_list(state.get().unwrap(), 300.0, |index| {
            text_block(format!("Row {index}"))
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = virtual_host(&reactor);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host,
            index: 90,
            lease: 1,
        });
    reactor.pump();
    let row = match reactor.engine().node_kind(host).unwrap() {
        NodeKind::VirtualHost { realized } => realized[&90].root,
        _ => unreachable!(),
    };

    assert!(count.borrow().as_ref().unwrap().try_set(50));
    reactor.pump();

    assert!(!reactor.engine().contains(row));
    let NodeKind::VirtualHost { realized } = reactor.engine().node_kind(host).unwrap() else {
        unreachable!()
    };
    assert!(!realized.contains_key(&90));
}
