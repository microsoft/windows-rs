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
    runtime.update(view(None));
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
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(*values.borrow(), [Some(Rc::from("item"))]);
    assert!(runtime.update(view(None)).is_empty());

    runtime.update(view(Some(false)));
    runtime.adapter_mut().observe(observation.clone());
    runtime.dispatch_native_events();
    assert!(runtime.update(view(Some(true))).is_empty());
    assert!(matches!(
        runtime.update(view(None)).as_slice(),
        [Mutation::SetProperties { object, set, clear }]
            if *object == item && set.is_empty() && clear.as_ref() == [PropertyId::IsSelected]
    ));
    runtime.adapter_mut().observe(observation);
    runtime.dispatch_native_events();
    assert!(runtime.update(view(None)).is_empty());
}

#[test]
fn retained_selection_tracks_keyed_items_across_reorder_and_removal() {
    let selected_values = Rc::new(RefCell::new(Vec::new()));
    let selected_values_for_callback = Rc::clone(&selected_values);
    let callback =
        Callback::new(move |value| selected_values_for_callback.borrow_mut().push(value));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        ListBox::new()
            .keyed_items([
                keyed("first", ListBoxItem::new().tag("first").is_selected(true)),
                keyed(
                    "second",
                    ListBoxItem::new().tag("second").is_selected(false),
                ),
            ])
            .on_selected_tag_changed(callback.clone()),
    );
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
        assert!(!runtime.prepare_update());
        assert_eq!(runtime.graph(), &graph);
        assert_eq!(runtime.adapter(), &queued);
    }
    assert_eq!(runtime.dispatch_native_events(), 1);
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

    let mutations = runtime.update(
        ListBox::new()
            .keyed_items([
                keyed("second", ListBoxItem::new().tag("second").is_selected(true)),
                keyed("first", ListBoxItem::new().tag("first").is_selected(false)),
            ])
            .on_selected_tag_changed(callback.clone()),
    );
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

    runtime.update(
        ListBox::new()
            .keyed_items([keyed(
                "first",
                ListBoxItem::new().tag("first").is_selected(true),
            )])
            .on_selected_tag_changed(callback.clone()),
    );
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
    assert_eq!(runtime.dispatch_native_events(), 0);
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

    runtime.update(
        ListBox::new()
            .keyed_items([
                keyed("first", ListBoxItem::new().tag("first").is_selected(false)),
                keyed("replacement", ListBoxItem::new().is_selected(true)),
            ])
            .on_selected_tag_changed(callback),
    );
    let replacement = runtime.graph().children(owner, RelationId::Items).unwrap()[1];
    assert_eq!(replacement.index(), before[1].index());
    assert_ne!(replacement.generation(), before[1].generation());
    let graph = runtime.graph().clone();
    runtime.adapter_mut().observe(Observation::SetSelection {
        object: owner,
        selected: Some(before[1]),
    });
    runtime.prepare_update();
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
    let mutations = runtime.update(
        ListView::new()
            .selected_index(Some(1))
            .selection_mode(ListViewSelectionMode::Extended)
            .can_drag_items(true)
            .can_reorder_items(true)
            .allow_drop(true),
    );
    let list = runtime.graph().root().unwrap();
    let sets: Vec<_> = mutations
        .iter()
        .filter_map(|mutation| match mutation {
            Mutation::SetProperties { object, set, .. } if *object == list => Some(set),
            _ => None,
        })
        .collect();
    assert_eq!(sets.len(), 2);
    assert_eq!(sets[1].len(), 1);
    assert_eq!(sets[1][0].id, PropertyId::SelectedIndex);
    let set: Vec<_> = sets.iter().flat_map(|set| set.iter()).collect();
    for property in [
        PropertyId::SelectedIndex,
        PropertyId::SelectionMode,
        PropertyId::CanDragItems,
        PropertyId::CanReorderItems,
        PropertyId::AllowDrop,
    ] {
        assert!(set.iter().any(|value| value.id == property));
    }

    let mutations = runtime.update(ListView::new());
    let clear: Vec<_> = mutations
        .iter()
        .filter_map(|mutation| match mutation {
            Mutation::SetProperties { object, clear, .. } if *object == list => Some(clear),
            _ => None,
        })
        .flat_map(|clear| clear.iter().copied())
        .collect();
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
fn index_is_written_after_its_items_and_reasserted_when_they_change() {
    let view = |keys: &[&'static str], index: Option<usize>, suffix: &str| {
        let view = FlipView::new().keyed_items(
            keys.iter()
                .map(|key| keyed(*key, TextBlock::new().text(format!("{key}{suffix}")))),
        );
        match index {
            Some(index) => view.selected_index(Some(index)),
            None => view,
        }
    };
    // The index write as `Some(set)` or `None` for a clear, after every item change.
    let index_write = |stats: &UpdateStats| {
        let mutations: Vec<_> = stats.iter().collect();
        let (position, write) = mutations
            .iter()
            .enumerate()
            .find_map(|(position, mutation)| match mutation {
                Mutation::SetProperties { set, clear, .. } => {
                    if let Some(property) = set
                        .iter()
                        .find(|property| property.id == PropertyId::SelectedIndex)
                    {
                        Some((position, Some(property.value.clone())))
                    } else {
                        clear
                            .contains(&PropertyId::SelectedIndex)
                            .then_some((position, None))
                    }
                }
                _ => None,
            })?;
        assert!(mutations[position + 1..].iter().all(|mutation| !matches!(
            mutation,
            Mutation::Insert { .. } | Mutation::Remove { .. } | Mutation::Reorder { .. }
        )));
        Some(write)
    };
    let selected = |index| Some(Some(PropertyValue::SelectionIndex(Some(index))));
    let mut runtime = Runtime::new(RecordingAdapter::default());

    let stats = runtime.update(view(&["a", "b", "c"], Some(1), ""));
    assert_eq!(index_write(&stats), selected(1));
    let stats = runtime.update(view(&["a", "b", "c", "d"], Some(1), ""));
    assert_eq!(index_write(&stats), selected(1));
    let stats = runtime.update(view(&["b", "a", "c", "d"], Some(1), ""));
    assert_eq!(index_write(&stats), selected(1));
    let stats = runtime.update(view(&["b", "a", "c", "d"], Some(1), "!"));
    assert_eq!(index_write(&stats), None);
    let none = Some(Some(PropertyValue::SelectionIndex(None)));
    let stats = runtime.update(view(&["b"], Some(1), "!"));
    assert_eq!(index_write(&stats), none);
    let stats = runtime.update(view(&["b"], Some(1), ""));
    assert_eq!(index_write(&stats), None);
    let stats = runtime.update(view(&["b", "a"], Some(1), ""));
    assert_eq!(index_write(&stats), selected(1));
    let stats = runtime.update(view(&["b", "a"], Some(5), ""));
    assert_eq!(index_write(&stats), none);
    let stats = runtime.update(view(&["b", "a"], None, ""));
    assert_eq!(index_write(&stats), Some(None));

    let mut runtime = Runtime::new(RecordingAdapter::default());
    let stats = runtime.update(view(&["a"], Some(1), ""));
    assert_eq!(index_write(&stats), none);
}

#[test]
fn selector_items_reject_exit_transitions() {
    let item = |key: &'static str| {
        TextBlock::new()
            .text(key)
            .exit_fade(Duration::from_millis(100))
    };
    let flip = |keys: &[&'static str]| {
        FlipView::new()
            .selected_index(Some(1))
            .keyed_items(keys.iter().map(|key| keyed(*key, item(key))))
    };
    let grid = |keys: &[&'static str]| {
        GridView::new().keyed_items(keys.iter().map(|key| {
            keyed(
                *key,
                GridViewItem::new()
                    .content(TextBlock::new().text(*key))
                    .exit_fade(Duration::from_millis(100)),
            )
        }))
    };
    let unsupported = "exit transitions are not supported";
    assert_panics(unsupported, || {
        let mut runtime = Runtime::new(RecordingAdapter::default());
        runtime.update(flip(&["a", "b", "c"]));
        runtime.update(flip(&["b", "c"]));
    });
    assert_panics(unsupported, || {
        let mut runtime = Runtime::new(RecordingAdapter::default());
        runtime.update(grid(&["a", "b"]));
        runtime.update(grid(&["b"]));
    });

    let list = |keys: &[&'static str]| {
        ListBox::new().keyed_items(keys.iter().map(|key| {
            keyed(
                *key,
                ListBoxItem::new()
                    .content(*key)
                    .exit_fade(Duration::from_millis(100)),
            )
        }))
    };
    assert_panics(unsupported, || {
        let mut runtime = Runtime::new(RecordingAdapter::default());
        runtime.update(list(&["a", "b"]));
        runtime.update(list(&["b"]));
    });
}

#[test]
fn index_that_requires_a_selection_falls_back_to_the_first_item() {
    let view = |count: usize, index: Option<Option<usize>>| {
        let view =
            Pivot::new().items((0..count).map(|item| PivotItem::new().header(item.to_string())));
        match index {
            Some(index) => view.selected_index(index),
            None => view,
        }
    };
    let index_writes = |stats: &UpdateStats| {
        stats
            .iter()
            .filter_map(|mutation| match mutation {
                Mutation::SetProperties { set, clear, .. } => set
                    .iter()
                    .find_map(|property| match property.value {
                        PropertyValue::SelectionIndex(index) => Some(Some(index)),
                        _ => None,
                    })
                    .or_else(|| clear.contains(&PropertyId::SelectedIndex).then_some(None)),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let stats = runtime.update(view(0, Some(None)));
    assert_eq!(index_writes(&stats), [Some(None)]);
    let stats = runtime.update(view(2, Some(None)));
    assert_eq!(index_writes(&stats), [Some(Some(0))]);
    let stats = runtime.update(view(2, Some(Some(5))));
    assert_eq!(index_writes(&stats), [Some(Some(0))]);
    let stats = runtime.update(view(2, Some(Some(1))));
    assert_eq!(index_writes(&stats), [Some(Some(1))]);
    let stats = runtime.update(view(2, None));
    assert_eq!(index_writes(&stats), []);
}
#[test]
fn index_over_a_list_property_is_written_after_it_and_reasserted_when_it_changes() {
    let view = |items: &[&str], index| {
        ComboBox::new()
            .selected_index(index)
            .items_source(items.to_vec())
    };
    let writes = |stats: &UpdateStats| {
        stats
            .iter()
            .filter_map(|mutation| match mutation {
                Mutation::SetProperties { set, .. } => Some(
                    set.iter()
                        .map(|property| match &property.value {
                            PropertyValue::SelectionIndex(index) => format!("index {index:?}"),
                            _ => format!("{:?}", property.id),
                        })
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let stats = runtime.update(view(&["a", "b"], Some(1)));
    assert_eq!(writes(&stats), ["ItemsSource", "index Some(1)"]);
    let stats = runtime.update(view(&["a", "b", "c"], Some(1)));
    assert_eq!(writes(&stats), ["ItemsSource", "index Some(1)"]);
    let stats = runtime.update(view(&["a"], Some(1)));
    assert_eq!(writes(&stats), ["ItemsSource", "index None"]);
    let stats = runtime.update(view(&["a"], Some(0)));
    assert_eq!(writes(&stats), ["index Some(0)"]);
}
#[test]
fn tree_view_selection_mode_and_item_invocation_replace_and_remove() {
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
    let mutations = runtime.update(
        TreeView::new()
            .selection_mode(TreeViewSelectionMode::Multiple)
            .nodes([TreeNode::new("root", "Root")])
            .on_item_invoked(first_callback.clone()),
    );
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
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(&*first.borrow(), &[Rc::from("Root")]);

    runtime.update(
        TreeView::new()
            .selection_mode(TreeViewSelectionMode::Single)
            .nodes([TreeNode::new("root", "Root")])
            .on_item_invoked(second_callback.clone()),
    );
    // An invocation queued before the replacement reaches the callback it fired under.
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(first_callback),
        EventPayload::String(Rc::from("Queued")),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(second_callback.clone()),
        EventPayload::String(Rc::from("Current")),
    ));
    assert_eq!(runtime.dispatch_native_events(), 2);
    assert_eq!(&*first.borrow(), &[Rc::from("Root"), Rc::from("Queued")]);
    assert_eq!(&*second.borrow(), &[Rc::from("Current")]);

    let mutations = runtime.update(TreeView::new().nodes([TreeNode::new("root", "Root")]));
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
    assert_eq!(runtime.dispatch_native_events(), 0);
}

#[test]
fn inspectable_string_item_events_deliver_typed_values() {
    let breadcrumb_values = Rc::new(RefCell::new(Vec::new()));
    let breadcrumb_callback = {
        let values = Rc::clone(&breadcrumb_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut breadcrumb = Runtime::new(RecordingAdapter::default());
    breadcrumb.update(
        BreadcrumbBar::new()
            .items_source(["Root", "Current"])
            .on_item_clicked(breadcrumb_callback.clone()),
    );
    let object = breadcrumb.graph().root().unwrap();
    breadcrumb.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::ItemClicked,
        EventValue::String(breadcrumb_callback),
        EventPayload::String(Rc::from("Current")),
    ));
    assert_eq!(breadcrumb.dispatch_native_events(), 1);
    assert_eq!(&*breadcrumb_values.borrow(), &[Rc::from("Current")]);

    let suggestion_values = Rc::new(RefCell::new(Vec::new()));
    let suggestion_callback = {
        let values = Rc::clone(&suggestion_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut suggestions = Runtime::new(RecordingAdapter::default());
    suggestions.update(
        AutoSuggestBox::new()
            .items_source(["First", "Second"])
            .on_suggestion_chosen(suggestion_callback.clone()),
    );
    let object = suggestions.graph().root().unwrap();
    suggestions.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::SuggestionChosen,
        EventValue::String(suggestion_callback),
        EventPayload::String(Rc::from("Second")),
    ));
    assert_eq!(suggestions.dispatch_native_events(), 1);
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
    runtime.update(data_list(&["a", "b", "c"], &callback));
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
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(
        *values.borrow(),
        [vec![Key::from("c"), Key::from("a"), Key::from("b")]]
    );
    assert_eq!(
        runtime.graph().children(list, RelationId::Items).unwrap(),
        [items[2], items[0], items[1]]
    );

    // The native collection already has the adopted order.
    let mutations = runtime.update(data_list(&["c", "a", "b"], &callback));
    assert_eq!(reorder_count(mutations.as_slice()), 0);

    // Removal indexes follow the observed order rather than the original declaration.
    let mutations = runtime.update(data_list(&["c", "b"], &callback));
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
    runtime.update(view());
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
    assert_eq!(runtime.dispatch_native_events(), 1);
    // Non-keyed builders key items by declared position.
    assert_eq!(
        *values.borrow(),
        [vec![Key::from(1usize), Key::from(0usize)]]
    );

    let mutations = runtime.update(view());
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
    runtime.update(data_list(&["a", "b", "c"], &first));
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
    assert_eq!(runtime.dispatch_native_events(), 0);
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
    assert_eq!(runtime.dispatch_native_events(), 0);

    // A payload that no longer matches the retained order is stale.
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::Keys(first.clone()),
        EventPayload::Items(vec![items[2], items[1], items[0]]),
    ));
    assert_eq!(runtime.dispatch_native_events(), 0);

    // A drag queued before a callback replacement reaches the callback it fired under.
    runtime.update(data_list(&["a", "b", "c"], &second));
    drag(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &first,
        vec![items[1], items[0], items[2]],
    );
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(
        *first_values.borrow(),
        [vec![Key::from("b"), Key::from("a"), Key::from("c")]]
    );
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
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(first_values.borrow().len(), 1);
    assert_eq!(
        *second_values.borrow(),
        [vec![Key::from("a"), Key::from("b"), Key::from("c")]]
    );

    // Native collections may still hold removed items while they retire.
    runtime.update(data_list(&["a", "c"], &second));
    drag(
        &mut runtime,
        list,
        RelationId::Items,
        EventId::DragItemsCompleted,
        &second,
        vec![items[2], items[1], items[0]],
    );
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(
        second_values.borrow().last().unwrap(),
        &[Key::from("c"), Key::from("a")]
    );
}

struct IgnoredReorder {
    reorders: Rc<Cell<usize>>,
}

impl Component for IgnoredReorder {
    type Input = Rc<Cell<usize>>;
    type Message = ();

    fn create(reorders: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            reorders: Rc::clone(reorders),
        }
    }

    fn update(&mut self, (): (), _: &ComponentContext<Self>) {
        self.reorders.set(self.reorders.get() + 1);
    }

    fn view(&self, _: &Self::Input, context: &mut ViewContext<Self>) -> View {
        ListView::new()
            .items(["a", "b", "c"].map(|key| DataItem::new(key, key)))
            .on_reordered(context.callback(|_: Vec<Key>| ()))
            .into()
    }
}

#[test]
fn queued_reorder_after_a_restoring_render_reports_the_current_native_order() {
    let reorders = Rc::new(Cell::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<IgnoredReorder>("list", Rc::clone(&reorders))],
    );
    host.test_adapter_mut().read_reorders(true);
    let list = host.reference(&Key::from("list")).unwrap().get().unwrap();
    let items = host
        .runtime()
        .graph()
        .children(list, RelationId::Items)
        .unwrap()
        .to_vec();
    let callback = host.runtime().graph().events(list).unwrap()[0]
        .value
        .clone();
    let dragged = vec![items[2], items[0], items[1]];
    host.test_adapter_mut()
        .move_children(list, RelationId::Items, &dragged);

    // One drag can raise several notifications. The first one's render restores the declared
    // order, so the later one must not replay the dragged order.
    for _ in 0..2 {
        host.test_adapter_mut().queue_native_event(
            Some(Observation::Reorder {
                object: list,
                relation: RelationId::Items,
                children: dragged.clone(),
            }),
            Some(EventDispatch::new(
                list,
                EventId::DragItemsCompleted,
                callback.clone(),
                EventPayload::Items(dragged.clone()),
            )),
        );
    }
    host.drain(8);

    assert_eq!(reorders.get(), 1);
    assert_eq!(
        host.runtime()
            .graph()
            .children(list, RelationId::Items)
            .unwrap(),
        items
    );
    assert_eq!(
        host.runtime()
            .adapter()
            .children(list, RelationId::Items)
            .unwrap(),
        items
    );
}

#[test]
#[should_panic(expected = "invalid relation Children for StackPanel")]
fn reorder_observation_for_an_untracked_relation_panics() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(StackPanel::new().children([TextBlock::new().text("a").into()]));
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
    runtime.dispatch_native_events();
}

#[test]
fn list_view_selection_callbacks_replace_and_remove() {
    let (first_values, first) = recorder();
    let (second_values, second) = recorder();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(ListView::new().on_selection_changed(first.clone()));
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
    assert_eq!(runtime.dispatch_native_events(), 1);
    assert_eq!(*first_values.borrow(), [Some(1)]);

    runtime.update(
        ListView::new()
            .selected_index(Some(1))
            .on_selection_changed(second.clone()),
    );
    // A change queued before the replacement reaches the callback it fired under.
    for callback in [&first, &second] {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            list,
            EventId::SelectionChanged,
            EventValue::SelectionIndex(callback.clone()),
            EventPayload::SelectionIndex(None),
        ));
    }
    assert_eq!(runtime.dispatch_native_events(), 2);
    assert_eq!(*first_values.borrow(), [Some(1), None]);
    assert_eq!(*second_values.borrow(), [None]);

    runtime.update(ListView::new());
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(second),
        EventPayload::SelectionIndex(Some(2)),
    ));
    assert_eq!(runtime.dispatch_native_events(), 0);
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
    runtime.update(view(&[1, 2]));
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
    assert_eq!(runtime.dispatch_native_events(), 2);
    assert_eq!(*closed_values.borrow(), [Key::from(2u64)]);
    assert_eq!(
        *reordered_values.borrow(),
        [vec![Key::from(2u64), Key::from(1u64)]]
    );

    // A close request for a tab removed by a later render is dropped.
    runtime.update(view(&[1]));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tabs,
        EventId::TabCloseRequested,
        EventValue::Key(closed),
        EventPayload::Item(items[1]),
    ));
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(closed_values.borrow().len(), 1);
}
