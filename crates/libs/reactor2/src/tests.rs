use super::*;

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
