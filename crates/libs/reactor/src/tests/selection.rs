use super::*;

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
            EventValue::Selection(callback),
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
fn grid_view_reorder_payload_preserves_item_tag_order() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            GridView::new()
                .items([
                    GridViewItem::new().tag("first").into(),
                    GridViewItem::new().tag("second").into(),
                ])
                .on_reordered(callback.clone()),
        )
        .unwrap();
    let grid = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        grid,
        EventId::DragItemsCompleted,
        EventValue::StringList(callback),
        EventPayload::StringList(vec!["second".to_string(), "first".to_string()]),
    ));

    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(
        &*values.borrow(),
        &[vec!["second".to_string(), "first".to_string()]]
    );
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
fn list_view_selection_and_reorder_callbacks_replace_remove_and_reject_stale_events() {
    let first_selections = Rc::new(RefCell::new(Vec::new()));
    let first_selection = {
        let values = Rc::clone(&first_selections);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let first_reorders = Rc::new(RefCell::new(Vec::new()));
    let first_reorder = {
        let values = Rc::clone(&first_reorders);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let second_selections = Rc::new(RefCell::new(Vec::new()));
    let second_selection = {
        let values = Rc::clone(&second_selections);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let second_reorders = Rc::new(RefCell::new(Vec::new()));
    let second_reorder = {
        let values = Rc::clone(&second_reorders);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            ListView::new()
                .on_selection_changed(first_selection.clone())
                .on_reordered(first_reorder.clone()),
        )
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
            EventValue::SelectionIndex(first_selection.clone()),
            EventPayload::SelectionIndex(Some(1)),
        )),
    );
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::StringList(first_reorder.clone()),
        EventPayload::StringList(vec!["second".to_string(), "first".to_string()]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*first_selections.borrow(), &[Some(1)]);
    assert_eq!(
        &*first_reorders.borrow(),
        &[vec!["second".to_string(), "first".to_string()]]
    );

    runtime
        .update(
            ListView::new()
                .selected_index(Some(1))
                .on_selection_changed(second_selection.clone())
                .on_reordered(second_reorder.clone()),
        )
        .unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(first_selection),
        EventPayload::SelectionIndex(None),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::StringList(first_reorder),
        EventPayload::StringList(vec!["stale".to_string()]),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(second_selection.clone()),
        EventPayload::SelectionIndex(None),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::StringList(second_reorder.clone()),
        EventPayload::StringList(vec!["current".to_string()]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*second_selections.borrow(), &[None]);
    assert_eq!(&*second_reorders.borrow(), &[vec!["current".to_string()]]);

    runtime.update(ListView::new()).unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(second_selection),
        EventPayload::SelectionIndex(Some(2)),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::StringList(second_reorder),
        EventPayload::StringList(vec!["removed".to_string()]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
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

#[test]
fn tab_view_events_deliver_item_tags() {
    let closed_values = Rc::new(RefCell::new(Vec::new()));
    let closed_callback = {
        let values = Rc::clone(&closed_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let reordered_values = Rc::new(RefCell::new(Vec::new()));
    let reordered_callback = {
        let values = Rc::clone(&reordered_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            TabView::new()
                .tab_items([
                    TabViewItem::new().tag("first").into(),
                    TabViewItem::new().tag("second").into(),
                ])
                .on_close_requested(closed_callback.clone())
                .on_reordered(reordered_callback.clone()),
        )
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TabCloseRequested,
        EventValue::String(closed_callback),
        EventPayload::String(Rc::from("second")),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TabItemsChanged,
        EventValue::StringList(reordered_callback),
        EventPayload::StringList(vec!["second".into(), "first".into()]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*closed_values.borrow(), &[Rc::from("second")]);
    assert_eq!(
        &*reordered_values.borrow(),
        &[vec!["second".to_string(), "first".to_string()]]
    );
}
