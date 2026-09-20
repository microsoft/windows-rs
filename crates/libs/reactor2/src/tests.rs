use super::*;
use std::cell::Cell;
use std::rc::Rc;

fn text(value: &str) -> Visual {
    TextBlock::new(value).into()
}

#[test]
fn disparate_controls_mount_into_the_same_retained_arena() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().children([
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
fn keyed_visual_reorder_preserves_objects_and_emits_one_generic_reorder() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().children([
            keyed("first", TextBlock::new("First")),
            keyed("second", TextBlock::new("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(Grid::new().children([
            keyed("second", TextBlock::new("Second")),
            keyed("first", TextBlock::new("Changed")),
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
fn keyed_insert_and_remove_produce_the_declared_order() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().children([
            keyed("first", TextBlock::new("First")),
            keyed("second", TextBlock::new("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let second = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[1];

    runtime
        .update(Grid::new().children([
            keyed("second", TextBlock::new("Second")),
            keyed("third", TextBlock::new("Third")),
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
                .content(TextBlock::new("Content"))
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
                .content(TextBlock::new("Updated"))
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
        .update(Border::new().content(TextBlock::new("Text")))
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
        .update(StackPanel::new().children([TextBlock::new("First").into(), Border::new().into()]))
        .unwrap();
    let panel = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(panel, RelationId::Children)
        .unwrap()
        .to_vec();

    runtime
        .update(StackPanel::new().children([
            TextBlock::new("Changed").into(),
            Grid::new().into(),
            TextBlock::new("Third").into(),
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
fn event_callbacks_update_without_recreating_the_object() {
    let first = Callback::new(|_: Rc<str>| {});
    let second = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed_callback(first))
        .unwrap();
    let root = runtime.graph().root().unwrap();

    let mutations = runtime
        .update(TextBox::new("Before").on_text_changed_callback(second))
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
        .update(TextBox::new("Text").on_text_changed_callback(callback))
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
        Err(UpdateError::Graph(GraphError::InvalidProperty(
            ObjectType::TextBox,
            PropertyId::Expanded
        )))
    ));
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
        Err(UpdateError::Graph(GraphError::InvalidPropertyValue(
            PropertyId::Orientation
        )))
    ));
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
        .update(TextBox::new("Text").on_text_changed_callback(first.clone()))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        root,
        EventId::TextChanged,
        EventValue::String(first),
        EventPayload::String(Rc::from("Stale")),
    ));
    runtime
        .update(TextBox::new("Text").on_text_changed_callback(second))
        .unwrap();

    let mut events = Vec::new();
    runtime.drain_events(&mut events).unwrap();
    for event in events {
        event.invoke();
    }

    assert_eq!(first_count.get(), 0);
    assert_eq!(second_count.get(), 0);
}

#[test]
fn button_content_and_click_use_generated_contracts() {
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let callback = Callback::new(move |()| callback_calls.set(callback_calls.get() + 1));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Button::new()
                .content(TextBlock::new("Deal"))
                .on_click_callback(callback.clone()),
        )
        .unwrap();
    let button = runtime.graph().root().unwrap();
    let content = runtime.graph().child(button, RelationId::Content).unwrap();
    assert_eq!(runtime.graph().kind(button), Some(ObjectType::Button));
    assert_eq!(runtime.graph().kind(content), Some(ObjectType::TextBlock));

    runtime.adapter_mut().queue_event(EventDispatch::new(
        button,
        EventId::Click,
        EventValue::Unit(callback),
        EventPayload::Unit,
    ));
    let mut events = Vec::new();
    runtime.drain_events(&mut events).unwrap();
    for event in events {
        event.invoke();
    }
    assert_eq!(calls.get(), 1);
}

#[test]
fn generated_controls_cover_distinct_native_value_and_relation_shapes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            StackPanel::new()
                .spacing(8.0)
                .orientation(Orientation::Horizontal)
                .children([
                    Slider::new().minimum(-10.0).maximum(10.0).value(2.5).into(),
                    CheckBox::new()
                        .is_checked(Some(true))
                        .content(TextBlock::new("Enabled"))
                        .into(),
                    ScrollViewer::new()
                        .content(
                            Canvas::new().children([TextBlock::new("Scrollable canvas")
                                .canvas_left(12.0)
                                .canvas_top(24.0)
                                .into()]),
                        )
                        .into(),
                ]),
        )
        .unwrap();

    let root = runtime.graph().root().unwrap();
    assert_eq!(
        runtime.graph().properties(root).unwrap(),
        [
            Property {
                id: PropertyId::Spacing,
                value: PropertyValue::F64(8.0),
            },
            Property {
                id: PropertyId::Orientation,
                value: PropertyValue::Enum {
                    kind: "Orientation",
                    variant: "Horizontal",
                },
            },
        ]
    );
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    assert_eq!(runtime.graph().kind(children[0]), Some(ObjectType::Slider));
    assert_eq!(
        runtime.graph().properties(children[0]).unwrap(),
        [
            Property {
                id: PropertyId::Minimum,
                value: PropertyValue::F64(-10.0),
            },
            Property {
                id: PropertyId::Maximum,
                value: PropertyValue::F64(10.0),
            },
            Property {
                id: PropertyId::Value,
                value: PropertyValue::F64(2.5),
            },
        ]
    );
    assert_eq!(
        runtime.graph().kind(children[1]),
        Some(ObjectType::CheckBox)
    );
    assert_eq!(
        runtime.graph().properties(children[1]).unwrap(),
        [Property {
            id: PropertyId::IsChecked,
            value: PropertyValue::OptionalBool(Some(true)),
        }]
    );
    let check_box_content = runtime
        .graph()
        .child(children[1], RelationId::Content)
        .unwrap();
    assert_eq!(
        runtime.graph().kind(check_box_content),
        Some(ObjectType::TextBlock)
    );
    assert_eq!(
        runtime.graph().kind(children[2]),
        Some(ObjectType::ScrollViewer)
    );
    let canvas = runtime
        .graph()
        .child(children[2], RelationId::Content)
        .unwrap();
    assert_eq!(runtime.graph().kind(canvas), Some(ObjectType::Canvas));
    assert_eq!(
        runtime
            .graph()
            .children(canvas, RelationId::Children)
            .unwrap()
            .len(),
        1
    );
    let canvas_child = runtime
        .graph()
        .children(canvas, RelationId::Children)
        .unwrap()[0];
    assert_eq!(
        runtime.graph().properties(canvas_child).unwrap(),
        [
            Property {
                id: PropertyId::CanvasLeft,
                value: PropertyValue::F64(12.0),
            },
            Property {
                id: PropertyId::CanvasTop,
                value: PropertyValue::F64(24.0),
            },
            Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("Scrollable canvas")),
            },
        ]
    );

    let mutations = runtime
        .update(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children([
                    Slider::new().value(2.5).maximum(10.0).minimum(-10.0).into(),
                    CheckBox::new()
                        .content(TextBlock::new("Enabled"))
                        .is_checked(Some(true))
                        .into(),
                    ScrollViewer::new()
                        .content(
                            Canvas::new().children([TextBlock::new("Scrollable canvas")
                                .canvas_top(24.0)
                                .canvas_left(12.0)
                                .into()]),
                        )
                        .into(),
                ]),
        )
        .unwrap();
    assert!(mutations.is_empty());

    let mutations = runtime
        .update(
            StackPanel::new().children([
                Slider::new().into(),
                CheckBox::new().content(TextBlock::new("Enabled")).into(),
                ScrollViewer::new()
                    .content(Canvas::new().children([TextBlock::new("Scrollable canvas").into()]))
                    .into(),
            ]),
        )
        .unwrap();
    assert!(mutations.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::SetProperties { object, clear, .. }
                if *object == root
                    && clear.len() == 2
                    && clear.contains(&PropertyId::Spacing)
                    && clear.contains(&PropertyId::Orientation)
        )
    }));
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    assert!(runtime.graph().properties(root).unwrap().is_empty());
    assert!(runtime.graph().properties(children[0]).unwrap().is_empty());
    assert!(runtime.graph().properties(children[1]).unwrap().is_empty());
    let canvas = runtime
        .graph()
        .child(children[2], RelationId::Content)
        .unwrap();
    let canvas_child = runtime
        .graph()
        .children(canvas, RelationId::Children)
        .unwrap()[0];
    assert_eq!(
        runtime.graph().properties(canvas_child).unwrap(),
        [Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Scrollable canvas")),
        }]
    );
}

#[test]
fn subtree_update_reconciles_only_the_target_object() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().children([
            keyed("first", TextBlock::new("First")),
            keyed("second", TextBlock::new("Second")),
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
        .update_subtree(first, TextBlock::new("Changed"))
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
        .update(Grid::new().children([keyed("child", TextBlock::new("Text"))]))
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
        runtime.update_subtree(node, TextBlock::new("Invalid")),
        Err(UpdateError::Graph(GraphError::InvalidChildCategory(
            RelationId::Roots
        )))
    );
    assert_eq!(runtime.graph().kind(node), Some(ObjectType::TreeNode));
}

#[test]
fn recording_adapter_rejects_invalid_replacement_batches() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBlock::new("Root")).unwrap();
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
        .update(Grid::new().children([keyed(
            "child",
            TextBox::new("Old").on_text_changed_callback(callback.clone()),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let child = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: child,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Stale")),
        },
    });
    runtime.adapter_mut().queue_event(EventDispatch::new(
        child,
        EventId::TextChanged,
        EventValue::String(callback),
        EventPayload::String(Rc::from("Stale")),
    ));

    runtime.update_subtree(child, Border::new()).unwrap();
    let mut events = Vec::new();
    runtime.drain_events(&mut events).unwrap();

    assert!(events.is_empty());
    assert_eq!(calls.get(), 0);
    assert_eq!(runtime.graph().kind(child), Some(ObjectType::Border));
    assert!(runtime.graph().properties(child).unwrap().is_empty());
}

#[test]
fn targeted_child_removal_preserves_sibling_identity() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().children([
            keyed("first", TextBlock::new("First")),
            keyed("second", TextBlock::new("Second")),
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
        .update(Border::new().content(TextBlock::new("Text")))
        .unwrap();

    assert!(
        runtime
            .update(Border::new().content(TextBlock::new("Text")))
            .unwrap()
            .is_empty()
    );
}

#[derive(Default)]
struct NeverCalledAdapter;

impl Adapter for NeverCalledAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("invalid declarations must not reach the adapter")
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("invalid declarations must not reach the adapter")
    }
}

#[test]
fn excessive_depth_is_rejected_before_reconciliation() {
    let mut child: Visual = TextBlock::new("leaf").into();
    for _ in 0..10_000 {
        child = Border::new().content(child).into();
    }
    let mut runtime = Runtime::new(NeverCalledAdapter);

    assert_eq!(
        runtime.update(child),
        Err(UpdateError::Graph(GraphError::DepthExceeded))
    );
}

#[test]
fn shared_declarations_cannot_expand_beyond_the_graph_limit() {
    let mut child: Visual = TextBlock::new("leaf").into();
    for _ in 0..17 {
        child = StackPanel::new().children([child.clone(), child]).into();
    }
    let mut runtime = Runtime::new(NeverCalledAdapter);

    assert_eq!(
        runtime.update(child),
        Err(UpdateError::Graph(GraphError::SizeExceeded))
    );
}

#[derive(Default)]
struct ApplyOnceAdapter {
    applied: bool,
}

impl Adapter for ApplyOnceAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.applied {
            Err(())
        } else {
            self.applied = true;
            Ok(())
        }
    }
}

#[test]
fn root_type_changes_are_rejected_before_native_mutation() {
    let mut runtime = Runtime::new(ApplyOnceAdapter::default());
    runtime.update(Border::new()).unwrap();

    assert_eq!(
        runtime.update(Grid::new()),
        Err(UpdateError::Graph(GraphError::RootTypeChanged {
            previous: ObjectType::Border,
            next: ObjectType::Grid,
        }))
    );
}

#[derive(Default)]
struct RejectingAdapter;

impl Adapter for RejectingAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Err(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("an invalid plan must not be applied")
    }
}

#[test]
fn adapter_validation_failure_poisons_the_runtime() {
    let mut runtime = Runtime::new(RejectingAdapter);

    assert_eq!(
        runtime.update(Border::new().content(TextBlock::new("Text"))),
        Err(UpdateError::Adapter(()))
    );
    assert_eq!(runtime.update(Border::new()), Err(UpdateError::Poisoned));
}

#[derive(Default)]
struct FailingApplyAdapter;

impl Adapter for FailingApplyAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Err(())
    }
}

#[test]
fn adapter_apply_failure_poisons_the_runtime() {
    let mut runtime = Runtime::new(FailingApplyAdapter);

    assert_eq!(
        runtime.update(Border::new().content(TextBlock::new("Text"))),
        Err(UpdateError::Adapter(()))
    );
    assert_eq!(runtime.update(Border::new()), Err(UpdateError::Poisoned));
    assert_eq!(runtime.graph().root(), None);
}
