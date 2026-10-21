use super::*;

#[test]
fn selection_observations_preserve_declaration_ownership() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let received = Rc::clone(&values);
    let callback = Callback::new(move |value| received.borrow_mut().push(value));
    let view = |selected| {
        let item = ListBoxItem::new().tag("item");
        ListBox::new()
            .items((match selected {
                Some(selected) => item.is_selected(selected),
                None => item,
            },))
            .on_selected_tag_changed(callback.clone())
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(view(None)).unwrap();
    let owner = runtime.graph().root().unwrap();
    let item = runtime.graph().children(owner, RelationId::Items).unwrap()[0];
    let observation = Observation::SetSelection {
        object: owner,
        selected: Some(item),
    };
    runtime.adapter_mut().queue_native_event(
        Some(observation.clone()),
        Some(EventDispatch::new(
            owner,
            EventId::SelectionChanged,
            EventValue::Selection(callback.clone()),
            EventPayload::Selection(SelectionChange {
                item: Some(item),
                value: Some(Rc::from("item")),
            }),
        )),
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(*values.borrow(), [Some(Rc::from("item"))]);
    assert!(runtime.update(view(None)).unwrap().is_empty());

    runtime.update(view(Some(false))).unwrap();
    runtime.adapter_mut().observe(observation.clone());
    runtime.dispatch_native_events().unwrap();
    assert!(runtime.update(view(Some(true))).unwrap().is_empty());
    assert!(matches!(
        runtime.update(view(None)).unwrap().as_slice(),
        [Mutation::SetProperties { object, set, clear }]
            if *object == item && set.is_empty() && clear.as_ref() == [PropertyId::IsSelected]
    ));
    runtime.adapter_mut().observe(observation);
    runtime.dispatch_native_events().unwrap();
    assert!(runtime.update(view(None)).unwrap().is_empty());
}

#[test]
fn retained_selection_tracks_keyed_items_across_reorder_and_removal() {
    let selected_values = Rc::new(RefCell::new(Vec::new()));
    let selected_values_for_callback = Rc::clone(&selected_values);
    let callback =
        Callback::new(move |value| selected_values_for_callback.borrow_mut().push(value));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            ListBox::new()
                .keyed_items([
                    keyed("first", ListBoxItem::new().tag("first").is_selected(true)),
                    keyed(
                        "second",
                        ListBoxItem::new().tag("second").is_selected(false),
                    ),
                ])
                .on_selected_tag_changed(callback.clone()),
        )
        .unwrap();
    let owner = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(owner, RelationId::Items)
        .unwrap()
        .to_vec();
    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetSelection {
            object: owner,
            selected: Some(before[1]),
        }),
        Some(EventDispatch::new(
            owner,
            EventId::SelectionChanged,
            EventValue::Selection(callback.clone()),
            EventPayload::Selection(SelectionChange {
                item: Some(before[1]),
                value: Some(Rc::from("second")),
            }),
        )),
    );

    let graph = runtime.graph().clone();
    let queued = runtime.adapter().clone();
    for _ in 0..2 {
        assert_eq!(
            runtime.prepare_update(),
            Err(UpdateError::PendingNativeEvent)
        );
        assert_eq!(runtime.graph(), &graph);
        assert_eq!(runtime.adapter(), &queued);
    }
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(
        runtime.graph().properties(before[0]).unwrap(),
        [
            Property {
                id: PropertyId::IsSelected,
                value: PropertyValue::Bool(false),
            },
            Property {
                id: PropertyId::Tag,
                value: PropertyValue::String(Rc::from("first")),
            },
        ]
    );
    assert_eq!(
        runtime.graph().properties(before[1]).unwrap(),
        [
            Property {
                id: PropertyId::IsSelected,
                value: PropertyValue::Bool(true),
            },
            Property {
                id: PropertyId::Tag,
                value: PropertyValue::String(Rc::from("second")),
            },
        ]
    );
    assert_eq!(&*selected_values.borrow(), &[Some(Rc::from("second"))]);

    let mutations = runtime
        .update(
            ListBox::new()
                .keyed_items([
                    keyed("second", ListBoxItem::new().tag("second").is_selected(true)),
                    keyed("first", ListBoxItem::new().tag("first").is_selected(false)),
                ])
                .on_selected_tag_changed(callback.clone()),
        )
        .unwrap();
    assert_eq!(
        runtime.graph().children(owner, RelationId::Items).unwrap(),
        [before[1], before[0]]
    );
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(!mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, .. } if before.contains(object)
    )));

    runtime
        .update(
            ListBox::new()
                .keyed_items([keyed(
                    "first",
                    ListBoxItem::new().tag("first").is_selected(true),
                )])
                .on_selected_tag_changed(callback.clone()),
        )
        .unwrap();
    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetSelection {
            object: owner,
            selected: Some(before[1]),
        }),
        Some(EventDispatch::new(
            owner,
            EventId::SelectionChanged,
            EventValue::Selection(callback.clone()),
            EventPayload::Selection(SelectionChange {
                item: Some(before[1]),
                value: Some(Rc::from("stale")),
            }),
        )),
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(&*selected_values.borrow(), &[Some(Rc::from("second"))]);
    assert_eq!(
        runtime.graph().properties(before[0]).unwrap(),
        [
            Property {
                id: PropertyId::IsSelected,
                value: PropertyValue::Bool(true),
            },
            Property {
                id: PropertyId::Tag,
                value: PropertyValue::String(Rc::from("first")),
            },
        ]
    );

    runtime
        .update(
            ListBox::new()
                .keyed_items([
                    keyed("first", ListBoxItem::new().tag("first").is_selected(false)),
                    keyed("replacement", ListBoxItem::new().is_selected(true)),
                ])
                .on_selected_tag_changed(callback),
        )
        .unwrap();
    let replacement = runtime.graph().children(owner, RelationId::Items).unwrap()[1];
    assert_eq!(replacement.index(), before[1].index());
    assert_ne!(replacement.generation(), before[1].generation());
    let graph = runtime.graph().clone();
    runtime.adapter_mut().observe(Observation::SetSelection {
        object: owner,
        selected: Some(before[1]),
    });
    runtime.prepare_update().unwrap();
    assert_eq!(runtime.graph(), &graph);
}

#[test]
fn selection_contracts_cover_navigation_list_box_and_selector_bar() {
    for (owner, item, relations, payload) in [
        (
            ObjectType::NavigationView,
            ObjectType::NavigationViewItem,
            &[RelationId::MenuItems, RelationId::FooterMenuItems][..],
            PropertyId::Tag,
        ),
        (
            ObjectType::ListBox,
            ObjectType::ListBoxItem,
            &[RelationId::Items][..],
            PropertyId::Tag,
        ),
        (
            ObjectType::SelectorBar,
            ObjectType::SelectorBarItem,
            &[RelationId::Items][..],
            PropertyId::Text,
        ),
    ] {
        let contract = selection_contract(owner).unwrap();
        assert_eq!(contract.item, item);
        assert_eq!(contract.relations, relations);
        assert_eq!(contract.selected_property, PropertyId::IsSelected);
        assert_eq!(contract.event, EventId::SelectionChanged);
        assert_eq!(contract.payload_property, payload);
    }
}

#[test]
fn list_view_properties_update_and_clear() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mutations = runtime
        .update(
            ListView::new()
                .selected_index(Some(1))
                .selection_mode(ListViewSelectionMode::Extended)
                .can_drag_items(true)
                .can_reorder_items(true)
                .allow_drop(true),
        )
        .unwrap();
    let list = runtime.graph().root().unwrap();
    let set = mutations
        .iter()
        .find_map(|mutation| match mutation {
            Mutation::SetProperties { object, set, .. } if *object == list => Some(set),
            _ => None,
        })
        .unwrap();
    for property in [
        PropertyId::SelectedIndex,
        PropertyId::SelectionMode,
        PropertyId::CanDragItems,
        PropertyId::CanReorderItems,
        PropertyId::AllowDrop,
    ] {
        assert!(set.iter().any(|value| value.id == property));
    }

    let mutations = runtime.update(ListView::new()).unwrap();
    let clear = mutations
        .iter()
        .find_map(|mutation| match mutation {
            Mutation::SetProperties { object, clear, .. } if *object == list => Some(clear),
            _ => None,
        })
        .unwrap();
    assert_eq!(clear.len(), 5);
    for property in [
        PropertyId::SelectedIndex,
        PropertyId::SelectionMode,
        PropertyId::CanDragItems,
        PropertyId::CanReorderItems,
        PropertyId::AllowDrop,
    ] {
        assert!(clear.contains(&property));
    }
}

#[test]
fn tree_view_selection_mode_and_item_invocation_replace_remove_and_reject_stale_events() {
    let first = Rc::new(RefCell::new(Vec::new()));
    let first_callback = {
        let values = Rc::clone(&first);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let second = Rc::new(RefCell::new(Vec::new()));
    let second_callback = {
        let values = Rc::clone(&second);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mutations = runtime
        .update(
            TreeView::new()
                .selection_mode(TreeViewSelectionMode::Multiple)
                .nodes([TreeNode::new("root", "Root")])
                .on_item_invoked(first_callback.clone()),
        )
        .unwrap();
    let tree = runtime.graph().root().unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, set, .. }
            if *object == tree
                && set.iter().any(|property| property.id == PropertyId::SelectionMode)
    )));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(first_callback.clone()),
        EventPayload::String(Rc::from("Root")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*first.borrow(), &[Rc::from("Root")]);

    runtime
        .update(
            TreeView::new()
                .selection_mode(TreeViewSelectionMode::Single)
                .nodes([TreeNode::new("root", "Root")])
                .on_item_invoked(second_callback.clone()),
        )
        .unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(first_callback),
        EventPayload::String(Rc::from("Stale")),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(second_callback.clone()),
        EventPayload::String(Rc::from("Current")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*second.borrow(), &[Rc::from("Current")]);

    let mutations = runtime
        .update(TreeView::new().nodes([TreeNode::new("root", "Root")]))
        .unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, clear, .. }
            if *object == tree && clear.contains(&PropertyId::SelectionMode)
    )));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(second_callback),
        EventPayload::String(Rc::from("Removed")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn inspectable_string_item_events_deliver_typed_values() {
    let breadcrumb_values = Rc::new(RefCell::new(Vec::new()));
    let breadcrumb_callback = {
        let values = Rc::clone(&breadcrumb_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut breadcrumb = Runtime::new(RecordingAdapter::default());
    breadcrumb
        .update(
            BreadcrumbBar::new()
                .items_source(["Root", "Current"])
                .on_item_clicked(breadcrumb_callback.clone()),
        )
        .unwrap();
    let object = breadcrumb.graph().root().unwrap();
    breadcrumb.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::ItemClicked,
        EventValue::String(breadcrumb_callback),
        EventPayload::String(Rc::from("Current")),
    ));
    assert_eq!(breadcrumb.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*breadcrumb_values.borrow(), &[Rc::from("Current")]);

    let suggestion_values = Rc::new(RefCell::new(Vec::new()));
    let suggestion_callback = {
        let values = Rc::clone(&suggestion_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut suggestions = Runtime::new(RecordingAdapter::default());
    suggestions
        .update(
            AutoSuggestBox::new()
                .items_source(["First", "Second"])
                .on_suggestion_chosen(suggestion_callback.clone()),
        )
        .unwrap();
    let object = suggestions.graph().root().unwrap();
    suggestions.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::SuggestionChosen,
        EventValue::String(suggestion_callback),
        EventPayload::String(Rc::from("Second")),
    ));
    assert_eq!(suggestions.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*suggestion_values.borrow(), &[Rc::from("Second")]);
}

fn recorder<T: 'static>() -> (Rc<RefCell<Vec<T>>>, Callback<T>) {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    (values, callback)
}

fn data_list(keys: &[&str], on_reordered: &Callback<Vec<Key>>) -> ListView {
    ListView::new()
        .items(keys.iter().map(|key| DataItem::new(*key, *key)))
        .on_reordered(on_reordered.clone())
}

fn reorder_count(mutations: &[Mutation]) -> usize {
    mutations
        .iter()
        .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
        .count()
}

fn queue_reorder(
    runtime: &mut Runtime<RecordingAdapter>,
    object: ObjectId,
    relation: RelationId,
    event: EventId,
    callback: &Callback<Vec<Key>>,
    children: Vec<ObjectId>,
) {
    runtime.adapter_mut().queue_native_event(
        Some(Observation::Reorder {
            object,
            relation,
            children: children.clone(),
        }),
        Some(EventDispatch::new(
            object,
            event,
            EventValue::Keys(callback.clone()),
            EventPayload::Items(children),
        )),
    );
}

/// Moves the native items as a user drag would, then queues the resulting observation and event.
fn drag(
    runtime: &mut Runtime<RecordingAdapter>,
    object: ObjectId,
    relation: RelationId,
    event: EventId,
    callback: &Callback<Vec<Key>>,
    children: Vec<ObjectId>,
) {
    let live = children
        .iter()
        .copied()
        .filter(|child| runtime.graph().kind(*child).is_some())
        .collect::<Vec<_>>();
    runtime.adapter_mut().move_children(object, relation, &live);
    queue_reorder(runtime, object, relation, event, callback, children);
}

#[test]
fn list_view_reorder_delivers_item_keys_and_adopting_it_needs_no_mutation() {
    let (values, callback) = recorder();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(data_list(&["a", "b", "c"], &callback))
        .unwrap();
    let list = runtime.graph().root().unwrap();
    let items = runtime
        .graph()
        .children(list, RelationId::Items)
        .unwrap()
        .to_vec();

    drag(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &callback,
        vec![items[2], items[0], items[1]],
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(
        *values.borrow(),
        [vec![Key::from("c"), Key::from("a"), Key::from("b")]]
    );
    assert_eq!(
        runtime.graph().children(list, RelationId::Items).unwrap(),
        [items[2], items[0], items[1]]
    );

    // The native collection already has the adopted order.
    let mutations = runtime
        .update(data_list(&["c", "a", "b"], &callback))
        .unwrap();
    assert_eq!(reorder_count(mutations.as_slice()), 0);

    // Removal indexes follow the observed order rather than the original declaration.
    let mutations = runtime.update(data_list(&["c", "b"], &callback)).unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::Remove { parent, relation: RelationId::Items, child, index: 1 }
            if *parent == list && *child == items[0]
    )));
    assert_eq!(reorder_count(mutations.as_slice()), 0);
}

#[test]
fn ignored_reorder_restores_the_declared_order() {
    let (values, callback) = recorder();
    let view = || {
        GridView::new()
            .items([
                GridViewItem::new().tag("first").into(),
                GridViewItem::new().tag("second").into(),
            ])
            .on_reordered(callback.clone())
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(view()).unwrap();
    let grid = runtime.graph().root().unwrap();
    let items = runtime
        .graph()
        .children(grid, RelationId::Items)
        .unwrap()
        .to_vec();

    drag(
        &mut runtime,
        grid,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &callback,
        vec![items[1], items[0]],
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    // Non-keyed builders key items by declared position.
    assert_eq!(
        *values.borrow(),
        [vec![Key::from(1usize), Key::from(0usize)]]
    );

    let mutations = runtime.update(view()).unwrap();
    assert!(matches!(
        mutations.as_slice(),
        [Mutation::Reorder { parent, relation: RelationId::Items, children, .. }]
            if *parent == grid && *children == items
    ));
}

#[test]
fn reorder_rejects_echoed_stale_and_partial_native_orders() {
    let (first_values, first) = recorder();
    let (second_values, second) = recorder();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(data_list(&["a", "b", "c"], &first)).unwrap();
    let list = runtime.graph().root().unwrap();
    let items = runtime
        .graph()
        .children(list, RelationId::Items)
        .unwrap()
        .to_vec();

    // A native snapshot taken mid-move is not a permutation of the retained items.
    queue_reorder(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &first,
        vec![items[1], items[0]],
    );
    queue_reorder(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &first,
        vec![items[1], items[1], items[0]],
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(
        runtime.graph().children(list, RelationId::Items).unwrap(),
        items
    );

    // An unchanged native order echoes Reactor's own mutation or a drop in place.
    queue_reorder(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &first,
        items.clone(),
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);

    // A payload that no longer matches the retained order is stale.
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::Keys(first.clone()),
        EventPayload::Items(vec![items[2], items[1], items[0]]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);

    // A replaced callback receives the observation but the old callback does not.
    runtime
        .update(data_list(&["a", "b", "c"], &second))
        .unwrap();
    drag(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &first,
        vec![items[1], items[0], items[2]],
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(
        runtime.graph().children(list, RelationId::Items).unwrap(),
        [items[1], items[0], items[2]]
    );
    drag(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &second,
        vec![items[0], items[1], items[2]],
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert!(first_values.borrow().is_empty());
    assert_eq!(
        *second_values.borrow(),
        [vec![Key::from("a"), Key::from("b"), Key::from("c")]]
    );

    // Native collections may still hold removed items while they retire.
    runtime.update(data_list(&["a", "c"], &second)).unwrap();
    drag(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &second,
        vec![items[2], items[1], items[0]],
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(
        second_values.borrow().last().unwrap(),
        &[Key::from("c"), Key::from("a")]
    );
}

#[test]
fn reorder_observation_for_an_untracked_relation_poisons_the_runtime() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().children([TextBlock::new().text("a").into()]))
        .unwrap();
    let panel = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(panel, RelationId::Children)
        .unwrap()
        .to_vec();
    runtime.adapter_mut().observe(Observation::Reorder {
        object: panel,
        relation: RelationId::Children,
        children,
    });
    assert!(matches!(
        runtime.dispatch_native_events(),
        Err(UpdateError::InvalidNativeEvent(
            GraphError::InvalidRelation(ObjectType::StackPanel, RelationId::Children)
        ))
    ));
    assert_eq!(runtime.prepare_update(), Err(UpdateError::Poisoned));
}

#[test]
fn list_view_selection_callbacks_replace_remove_and_reject_stale_events() {
    let (first_values, first) = recorder();
    let (second_values, second) = recorder();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ListView::new().on_selection_changed(first.clone()))
        .unwrap();
    let list = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetProperty {
            object: list,
            property: Property {
                id: PropertyId::SelectedIndex,
                value: PropertyValue::SelectionIndex(Some(1)),
            },
        }),
        Some(EventDispatch::new(
            list,
            EventId::SelectionChanged,
            EventValue::SelectionIndex(first.clone()),
            EventPayload::SelectionIndex(Some(1)),
        )),
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(*first_values.borrow(), [Some(1)]);

    runtime
        .update(
            ListView::new()
                .selected_index(Some(1))
                .on_selection_changed(second.clone()),
        )
        .unwrap();
    for callback in [&first, &second] {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            list,
            EventId::SelectionChanged,
            EventValue::SelectionIndex(callback.clone()),
            EventPayload::SelectionIndex(None),
        ));
    }
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(*second_values.borrow(), [None]);

    runtime.update(ListView::new()).unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(second),
        EventPayload::SelectionIndex(Some(2)),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn tab_view_events_deliver_item_keys() {
    let (closed_values, closed) = recorder();
    let (reordered_values, reordered) = recorder();
    let view = |keys: &[u64]| {
        TabView::new()
            .keyed_tab_items(
                keys.iter()
                    .map(|key| keyed(*key, TabViewItem::new().header(key.to_string()))),
            )
            .on_close_requested(closed.clone())
            .on_reordered(reordered.clone())
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(view(&[1, 2])).unwrap();
    let tabs = runtime.graph().root().unwrap();
    let items = runtime
        .graph()
        .children(tabs, RelationId::TabItems)
        .unwrap()
        .to_vec();

    runtime.adapter_mut().queue_event(EventDispatch::new(
        tabs,
        EventId::TabCloseRequested,
        EventValue::Key(closed.clone()),
        EventPayload::Item(items[1]),
    ));
    drag(
        &mut runtime,
        tabs,
        RelationId::TabItems,
        EventId::TabItemsChanged,
        &reordered,
        vec![items[1], items[0]],
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(*closed_values.borrow(), [Key::from(2u64)]);
    assert_eq!(
        *reordered_values.borrow(),
        [vec![Key::from(2u64), Key::from(1u64)]]
    );

    // A close request for a tab removed by a later render is dropped.
    runtime.update(view(&[1])).unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tabs,
        EventId::TabCloseRequested,
        EventValue::Key(closed),
        EventPayload::Item(items[1]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(closed_values.borrow().len(), 1);
}
