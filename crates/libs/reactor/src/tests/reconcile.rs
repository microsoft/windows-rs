use super::*;

#[test]
fn initial_mount_reserves_the_validated_object_count() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children((0usize..512).map(|index| keyed(index, Border::new()))))
        .unwrap();

    let memory = runtime.graph().retained_memory();
    assert_eq!(memory.slot_len, 513);
    assert_eq!(memory.slot_capacity, 513);
}

#[test]
fn disparate_controls_mount_into_the_same_retained_arena() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().keyed_children([
                keyed("border", Border::new().content(text("content"))),
                keyed(
                    "tree",
                    TreeView::new().nodes([TreeNode::new("root", "Root")
                        .content(text("label"))
                        .children([TreeNode::new("child", "Child")])]),
                ),
                keyed(
                    "list",
                    ListView::new().items([
                        DataItem::new("first", "First"),
                        DataItem::new("second", "Second"),
                    ]),
                ),
            ]),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();

    assert_eq!(runtime.graph().object_count(), 10);
    assert_eq!(runtime.graph().kind(root), Some(ObjectType::Grid));
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .map(<[ObjectId]>::len),
        Some(3)
    );
}

#[test]
fn stack_panel_keyed_reorder_preserves_objects_and_emits_one_generic_reorder() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().keyed_children([
            keyed("first", TextBlock::new().text("First")),
            keyed("second", TextBlock::new().text("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(StackPanel::new().keyed_children([
            keyed("second", TextBlock::new().text("Second")),
            keyed("first", TextBlock::new().text("Changed")),
        ]))
        .unwrap();
    let after = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();

    assert_eq!(after, [before[1], before[0]]);
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Create { .. } | Mutation::Destroy { .. }))
    );
    assert!(mutations
        .iter()
        .any(|mutation| matches!(mutation, Mutation::SetProperties { object, .. } if *object == before[0])));
}

#[test]
fn tab_view_keyed_reorder_preserves_tab_objects() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TabView::new().keyed_tab_items([
            keyed("first", TabViewItem::new().header("First")),
            keyed("second", TabViewItem::new().header("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(root, RelationId::TabItems)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(TabView::new().keyed_tab_items([
            keyed("second", TabViewItem::new().header("Second")),
            keyed("first", TabViewItem::new().header("Changed")),
        ]))
        .unwrap();
    let after = runtime
        .graph()
        .children(root, RelationId::TabItems)
        .unwrap();

    assert_eq!(after, [before[1], before[0]]);
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Create { .. } | Mutation::Destroy { .. }))
    );
}

#[test]
fn keyed_insert_and_remove_produce_the_declared_order() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", TextBlock::new().text("First")),
            keyed("second", TextBlock::new().text("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let second = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[1];

    runtime
        .update(Grid::new().keyed_children([
            keyed("second", TextBlock::new().text("Second")),
            keyed("third", TextBlock::new().text("Third")),
        ]))
        .unwrap();

    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    assert_eq!(children[0], second);
    assert_eq!(
        runtime.graph().kind(children[1]),
        Some(ObjectType::TextBlock)
    );
    assert_eq!(
        runtime.adapter().children(root, RelationId::Children),
        Some(children)
    );
}

#[test]
fn hierarchical_objects_use_the_same_keyed_relation_reconciler() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            TreeView::new().nodes([TreeNode::new("root", "Root")
                .content(TextBlock::new().text("Content"))
                .children([
                    TreeNode::new("first", "First"),
                    TreeNode::new("second", "Second"),
                ])]),
        )
        .unwrap();
    let tree = runtime.graph().root().unwrap();
    let root = runtime.graph().children(tree, RelationId::Roots).unwrap()[0];
    let content = runtime.graph().child(root, RelationId::Content).unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(
            TreeView::new().nodes([TreeNode::new("root", "Renamed")
                .content(TextBlock::new().text("Updated"))
                .children([
                    TreeNode::new("second", "Second"),
                    TreeNode::new("first", "First"),
                ])]),
        )
        .unwrap();

    assert_eq!(
        runtime.graph().children(root, RelationId::Children),
        Some([children[1], children[0]].as_slice())
    );
    assert_eq!(
        runtime.graph().child(root, RelationId::Content),
        Some(content)
    );
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Create { .. } | Mutation::Destroy { .. }))
    );
}

#[test]
fn container_generated_data_uses_the_same_keyed_relation_reconciler() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ListView::new().items([
            DataItem::new("first", "First"),
            DataItem::new("second", "Second"),
        ]))
        .unwrap();
    let list = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(list, RelationId::Items)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(ListView::new().items([
            DataItem::new("second", "Second"),
            DataItem::new("first", "Changed"),
        ]))
        .unwrap();

    assert_eq!(
        runtime.graph().children(list, RelationId::Items),
        Some([before[1], before[0]].as_slice())
    );
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
}

#[test]
fn owned_content_replacement_uses_generic_relation_mutations() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().content(TextBlock::new().text("Text")))
        .unwrap();
    let border = runtime.graph().root().unwrap();
    let previous = runtime.graph().child(border, RelationId::Content).unwrap();

    let mutations = runtime.update(Border::new().content(Grid::new())).unwrap();
    let next = runtime.graph().child(border, RelationId::Content).unwrap();

    assert_ne!(next, previous);
    assert_eq!(runtime.graph().kind(next), Some(ObjectType::Grid));
    assert!(matches!(
        mutations.as_slice(),
        [
            Mutation::Detach { .. },
            Mutation::Destroy { .. },
            Mutation::Create { .. },
            Mutation::Attach { .. }
        ]
    ));
}

#[test]
fn positional_children_reconcile_by_index_without_keys() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            StackPanel::new()
                .children([TextBlock::new().text("First").into(), Border::new().into()]),
        )
        .unwrap();
    let panel = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(panel, RelationId::Children)
        .unwrap()
        .to_vec();

    runtime
        .update(StackPanel::new().children([
            TextBlock::new().text("Changed").into(),
            Grid::new().into(),
            TextBlock::new().text("Third").into(),
        ]))
        .unwrap();

    let after = runtime
        .graph()
        .children(panel, RelationId::Children)
        .unwrap();
    assert_eq!(after[0], before[0]);
    assert_ne!(after[1], before[1]);
    assert_eq!(runtime.graph().kind(after[1]), Some(ObjectType::Grid));
    assert_eq!(
        runtime.adapter().children(panel, RelationId::Children),
        Some(after)
    );
}

#[test]
fn property_updates_only_emit_changed_values() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TreeView::new().nodes([TreeNode::new("root", "Before").expanded(true)]))
        .unwrap();

    let mutations = runtime
        .update(TreeView::new().nodes([TreeNode::new("root", "After").expanded(true)]))
        .unwrap();

    let (set, clear) = mutations
        .iter()
        .find_map(|mutation| match mutation {
            Mutation::SetProperties { set, clear, .. } => Some((set, clear)),
            _ => None,
        })
        .unwrap();
    assert_eq!(set.len(), 1);
    assert_eq!(set[0].id, PropertyId::Text);
    assert!(clear.is_empty());
}

#[test]
fn visual_transitions_are_retained_stable_and_removed_when_omitted() {
    let transitions = || [ThemeTransition::Reposition];
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .margin(Thickness::new(10.0, 20.0, 0.0, 0.0))
                .transitions(transitions()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    assert!(matches!(
        runtime
            .graph()
            .properties(border)
            .unwrap()
            .iter()
            .find(|property| property.id == PropertyId::Transitions),
        Some(Property {
            value: PropertyValue::ThemeTransitions(value),
            ..
        }) if value.as_ref() == transitions().as_slice()
    ));

    assert!(
        runtime
            .update(
                Border::new()
                    .margin(Thickness::new(10.0, 20.0, 0.0, 0.0))
                    .transitions(transitions()),
            )
            .unwrap()
            .is_empty()
    );

    let mutations = runtime
        .update(Border::new().margin(Thickness::new(30.0, 40.0, 0.0, 0.0)))
        .unwrap();
    assert_eq!(
        mutations,
        [Mutation::SetProperties {
            object: border,
            set: Rc::from([Property {
                id: PropertyId::Margin,
                value: PropertyValue::Thickness(Thickness::new(30.0, 40.0, 0.0, 0.0)),
            }]),
            clear: Rc::from([PropertyId::Transitions]),
        }]
    );
}

#[test]
fn keyed_visual_moves_preserve_identity_and_only_update_changed_state() {
    let first_callback = Callback::new(|_: PointerEventInfo| {});
    let second_callback = Callback::new(|_: PointerEventInfo| {});
    let card = |key: &str, left: f64, background: Color, callback: Callback<PointerEventInfo>| {
        keyed(
            key,
            Border::new()
                .margin(Thickness::new(left, 0.0, 0.0, 0.0))
                .background(background)
                .transitions([ThemeTransition::Reposition])
                .on_pointer_released(callback),
        )
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            card("first", 0.0, Color::rgb(255, 255, 255), first_callback),
            card(
                "second",
                100.0,
                Color::rgb(255, 255, 255),
                second_callback.clone(),
            ),
        ]))
        .unwrap();
    let grid = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(grid, RelationId::Children)
        .unwrap()
        .to_vec();
    let replacement_callback = Callback::new(|_: PointerEventInfo| {});

    let mutations = runtime
        .update(Grid::new().keyed_children([
            card("second", 100.0, Color::rgb(255, 255, 255), second_callback),
            card(
                "first",
                200.0,
                Color::rgb(255, 220, 220),
                replacement_callback,
            ),
        ]))
        .unwrap();
    let after = runtime
        .graph()
        .children(grid, RelationId::Children)
        .unwrap();

    assert_eq!(after, [before[1], before[0]]);
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Create { .. } | Mutation::Destroy { .. }))
    );
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, set, clear }
            if *object == before[0]
                && clear.is_empty()
                && set.len() == 2
                && set.iter().any(|property| property.id == PropertyId::Margin)
                && set.iter().any(|property| property.id == PropertyId::Background)
    )));
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetEvents { object, set, clear }
            if *object == before[0]
                && clear.is_empty()
                && set.len() == 1
                && set[0].id == EventId::PointerReleased
    )));
    assert!(!mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { set, .. }
            if set.iter().any(|property| property.id == PropertyId::Transitions)
    )));
}

#[test]
fn subtree_update_reconciles_only_the_target_object() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", TextBlock::new().text("First")),
            keyed("second", TextBlock::new().text("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    let first = children[0];
    let second = children[1];

    let mutations = runtime
        .update_subtree(first, TextBlock::new().text("Changed"))
        .unwrap();

    assert_eq!(
        mutations,
        vec![Mutation::SetProperties {
            object: first,
            set: Rc::from([Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("Changed")),
            }]),
            clear: Rc::from([]),
        }]
    );
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        [first, second]
    );
}

#[test]
fn subtree_root_type_replacement_preserves_identity() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed("child", TextBlock::new().text("Text"))]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let child = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];

    let mutations = runtime.update_subtree(child, Border::new()).unwrap();

    assert_eq!(
        mutations,
        vec![Mutation::Replace {
            object: child,
            kind: ObjectType::Border,
        }]
    );
    assert_eq!(runtime.graph().kind(child), Some(ObjectType::Border));
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        [child]
    );
}

#[test]
fn subtree_root_type_replacement_rejects_incompatible_relations() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TreeView::new().nodes([TreeNode::new("node", "Node")]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let node = runtime.graph().children(root, RelationId::Roots).unwrap()[0];

    assert_eq!(
        runtime.update_subtree(node, TextBlock::new().text("Invalid")),
        Err(UpdateError::Graph(GraphError::InvalidChildCategory(
            RelationId::Roots
        )))
    );
    assert_eq!(runtime.graph().kind(node), Some(ObjectType::TreeNode));
}

#[test]
fn recording_adapter_rejects_invalid_replacement_batches() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBlock::new().text("Root")).unwrap();
    let root = runtime.graph().root().unwrap();
    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::Replace {
            object: root,
            kind: ObjectType::Border,
        }]),
        Err(AdapterError::InvalidReplacement(root))
    );

    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TreeView::new().nodes([TreeNode::new("node", "Node")]))
        .unwrap();
    let tree = runtime.graph().root().unwrap();
    let node = runtime.graph().children(tree, RelationId::Roots).unwrap()[0];
    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::Replace {
            object: node,
            kind: ObjectType::TextBlock,
        }]),
        Err(AdapterError::InvalidReplacement(node))
    );
}

#[test]
fn replacement_discards_observations_and_events_from_the_old_native_object() {
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let callback = Callback::new(move |_| callback_calls.set(callback_calls.get() + 1));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed(
            "child",
            TextBox::new("Old").on_text_changed(callback.clone()),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let child = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    for value in ["First", "Stale"] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object: child,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                child,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }
    let mut active = runtime.next_native_event().unwrap().unwrap();
    active.invoke();
    runtime.update_subtree(child, Border::new()).unwrap();
    drop(active);

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(calls.get(), 1);
    assert_eq!(runtime.graph().kind(child), Some(ObjectType::Border));
    assert!(runtime.graph().properties(child).unwrap().is_empty());
}

#[test]
fn targeted_child_removal_preserves_sibling_identity() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", TextBlock::new().text("First")),
            keyed("second", TextBlock::new().text("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    let first = children[0];
    let second = children[1];

    let mutations = runtime
        .remove_child(root, RelationId::Children, first)
        .unwrap();

    assert_eq!(
        mutations,
        vec![
            Mutation::Remove {
                parent: root,
                relation: RelationId::Children,
                child: first,
                index: 0,
            },
            Mutation::Destroy { object: first },
        ]
    );
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        [second]
    );
    assert_eq!(runtime.graph().kind(first), None);
}

#[test]
fn no_change_produces_no_mutations() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().content(TextBlock::new().text("Text")))
        .unwrap();

    assert!(
        runtime
            .update(Border::new().content(TextBlock::new().text("Text")))
            .unwrap()
            .is_empty()
    );
}
