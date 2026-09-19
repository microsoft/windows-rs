use super::*;
use std::cell::RefCell;
use std::rc::Rc;

fn nodes(label: &str) -> Vec<TreeNode> {
    vec![TreeNode::new("root", label).expanded(true).children([
        TreeNode::new("first", "First"),
        TreeNode::new("second", "Second"),
    ])]
}

fn flat_nodes(count: usize) -> Vec<TreeNode> {
    (0..count)
        .map(|index| TreeNode::new(index.to_string(), format!("Node {index}")))
        .collect()
}

#[test]
fn mounts_updates_and_routes_recursive_tree_nodes() {
    let invoked = Rc::new(RefCell::new(String::new()));
    let capture = Rc::clone(&invoked);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TreeView::new()
            .on_item_invoked(move |label| *capture.borrow_mut() = label)
            .nodes(nodes("Root")),
    )
    .unwrap();

    let owner = pump.root().unwrap();
    let target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let root = pump.runtime().tree_roots(target).unwrap()[0];
    let children = pump.runtime().tree_node_children(root).unwrap().to_vec();
    assert_eq!(pump.runtime().tree_node_text(root), Some("Root"));
    assert_eq!(pump.runtime().tree_node_text(children[0]), Some("First"));
    assert_eq!(pump.runtime().tree_node_text(children[1]), Some("Second"));
    let revision = pump
        .event_revision(target, EventId::TreeViewItemInvoked)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        target,
        EventId::TreeViewItemInvoked,
        revision,
        EventPayload::Str("Second".to_string()),
    ));
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(&*invoked.borrow(), "Second");

    pump.update_view(
        TreeView::new()
            .on_item_invoked(|_| {})
            .nodes(nodes("Changed")),
    )
    .unwrap();
    assert_eq!(pump.root().unwrap(), owner);
    assert_eq!(
        Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap(),
        target
    );
    assert_eq!(pump.runtime().tree_roots(target), Some([root].as_slice()));
    assert_eq!(pump.runtime().tree_node_text(root), Some("Changed"));
    assert_eq!(
        pump.runtime().tree_node_children(root),
        Some(children.as_slice())
    );
}

#[test]
fn reconciles_content_and_reorders_nodes_without_recreating_them() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TreeView::new().nodes([
        TreeNode::new("first", "First").content(TextBlock::new().text("One")),
        TreeNode::new("second", "Second"),
    ]))
    .unwrap();

    let owner = pump.root().unwrap();
    let target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let roots = pump.runtime().tree_roots(target).unwrap().to_vec();
    let content = pump.runtime().tree_node_content(roots[0]).unwrap();

    pump.update_view(TreeView::new().nodes([
        TreeNode::new("second", "Second"),
        TreeNode::new("first", "First").content(TextBlock::new().text("Updated")),
    ]))
    .unwrap();

    assert_eq!(
        pump.runtime().tree_roots(target),
        Some([roots[1], roots[0]].as_slice())
    );
    assert_eq!(pump.runtime().tree_node_content(roots[0]), Some(content));
    assert_eq!(
        pump.runtime()
            .node(content)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("Updated".into()))
    );
    let commands = pump.runtime().commands().last().unwrap();
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::SynchronizeTreeNodes { nodes, .. } if nodes.as_slice() == [roots[1], roots[0]]
    )));
    assert!(!commands.iter().any(|command| matches!(
        command,
        Command::CreateTreeNode { .. } | Command::Destroy { .. }
    )));
}

#[test]
fn large_reorder_emits_one_native_synchronization_without_recreating_nodes() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let mut definitions = flat_nodes(256);
    pump.mount_view(TreeView::new().nodes(definitions.clone()))
        .unwrap();
    definitions.rotate_left(1);

    pump.update_view(TreeView::new().nodes(definitions))
        .unwrap();

    let commands = pump.runtime().commands().last().unwrap();
    assert_eq!(
        commands
            .iter()
            .filter(|command| matches!(command, Command::SynchronizeTreeNodes { .. }))
            .count(),
        1
    );
    assert!(!commands.iter().any(|command| matches!(
        command,
        Command::CreateTreeNode { .. } | Command::Destroy { .. }
    )));
}

#[test]
fn adds_replaces_and_removes_custom_content() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TreeView::new().nodes([TreeNode::new("root", "Root")]))
        .unwrap();
    let owner = pump.root().unwrap();
    let target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let root = pump.runtime().tree_roots(target).unwrap()[0];

    pump.update_view(
        TreeView::new()
            .nodes([TreeNode::new("root", "Root").content(TextBlock::new().text("Content"))]),
    )
    .unwrap();
    let first = pump.runtime().tree_node_content(root).unwrap();

    pump.update_view(TreeView::new().nodes([TreeNode::new("root", "Root").content(Button::new())]))
        .unwrap();
    let second = pump.runtime().tree_node_content(root).unwrap();
    assert_ne!(second, first);
    assert!(pump.runtime().node(first).is_none());

    pump.update_view(TreeView::new().nodes([TreeNode::new("root", "Root")]))
        .unwrap();
    assert_eq!(pump.runtime().tree_node_content(root), None);
    assert!(pump.runtime().node(second).is_none());
}

#[test]
fn inserts_and_removes_nested_nodes_incrementally() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TreeView::new().nodes([TreeNode::new("root", "Root").children([
            TreeNode::new("keep", "Keep").content(TextBlock::new().text("Keep")),
            TreeNode::new("remove", "Remove").content(TextBlock::new().text("Remove")),
        ])]),
    )
    .unwrap();

    let owner = pump.root().unwrap();
    let target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let root = pump.runtime().tree_roots(target).unwrap()[0];
    let children = pump.runtime().tree_node_children(root).unwrap().to_vec();
    let keep_content = pump.runtime().tree_node_content(children[0]).unwrap();

    pump.update_view(
        TreeView::new().nodes([TreeNode::new("root", "Root").children([
            TreeNode::new("keep", "Keep").content(TextBlock::new().text("Kept")),
            TreeNode::new("insert", "Insert"),
        ])]),
    )
    .unwrap();

    let updated = pump.runtime().tree_node_children(root).unwrap();
    assert_eq!(updated[0], children[0]);
    assert_eq!(
        pump.runtime().tree_node_content(updated[0]),
        Some(keep_content)
    );
    assert_eq!(pump.runtime().tree_node_text(updated[1]), Some("Insert"));
    assert!(pump.runtime().tree_node(children[1]).is_none());
    let commands = pump.runtime().commands().last().unwrap();
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::RemoveTreeNode { node, .. } if *node == children[1]
    )));
    assert!(commands.iter().any(
        |command| matches!(command, Command::InsertTreeNode { node, .. } if *node == updated[1])
    ));
}

#[test]
fn removes_complete_tree_node_subtrees_postorder() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TreeView::new().nodes([TreeNode::new("root", "Root")
            .child(TreeNode::new("child", "Child").content(TextBlock::new().text("Content")))]),
    )
    .unwrap();
    let owner = pump.root().unwrap();
    let target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let root = pump.runtime().tree_roots(target).unwrap()[0];
    let child = pump.runtime().tree_node_children(root).unwrap()[0];

    pump.update_view(TreeView::new().nodes([])).unwrap();

    assert_eq!(pump.runtime().tree_roots(target), Some([].as_slice()));
    assert!(pump.runtime().tree_node(root).is_none());
    assert!(pump.runtime().tree_node(child).is_none());
}

#[test]
fn replacing_the_tree_retires_nodes_before_the_native_control() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TreeView::new().nodes([TreeNode::new("root", "Root")
            .child(TreeNode::new("child", "Child").content(TextBlock::new().text("Content")))]),
    )
    .unwrap();
    let owner = pump.root().unwrap();
    let target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let root = pump.runtime().tree_roots(target).unwrap()[0];
    let child = pump.runtime().tree_node_children(root).unwrap()[0];

    pump.update_view(Button::new().into()).unwrap();

    assert!(pump.runtime().tree_roots(target).is_none());
    assert!(pump.runtime().tree_node(root).is_none());
    assert!(pump.runtime().tree_node(child).is_none());
}

#[test]
fn replacing_the_wrapped_tree_rebuilds_nodes_on_the_new_target() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TreeView::new().nodes(nodes("Root")))
        .unwrap();
    let owner = pump.root().unwrap();
    let previous_target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let previous_root = pump.runtime().tree_roots(previous_target).unwrap()[0];

    pump.update_view(View::fragment([TreeView::new()]).nodes(nodes("Root")))
        .unwrap();

    let target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let root = pump.runtime().tree_roots(target).unwrap()[0];
    assert_ne!(target, previous_target);
    assert_ne!(root, previous_root);
    assert!(pump.runtime().tree_roots(previous_target).is_none());
    assert!(pump.runtime().tree_node(previous_root).is_none());
    assert_eq!(pump.runtime().tree_node_text(root), Some("Root"));
}

#[test]
fn preserves_native_expansion_when_the_declarative_value_is_unchanged() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TreeView::new().nodes([TreeNode::new("root", "Root")]))
        .unwrap();
    let owner = pump.root().unwrap();
    let target = Pump::<RecordingRuntime>::native_root(&pump.tree, owner).unwrap();
    let root = pump.runtime().tree_roots(target).unwrap()[0];
    pump.runtime_mut().set_tree_node_expanded(root, true);

    pump.update_view(TreeView::new().nodes([TreeNode::new("root", "Root")]))
        .unwrap();

    assert_eq!(pump.runtime().tree_node_expanded(root), Some(true));
}

#[test]
fn rejects_duplicate_sibling_keys_at_any_depth() {
    let mut root = Pump::new(RecordingRuntime::default());
    assert_eq!(
        root.mount_view(TreeView::new().nodes([
            TreeNode::new("duplicate", "First"),
            TreeNode::new("duplicate", "Second"),
        ])),
        Err(PumpError::DuplicateKey(Key::from("duplicate")))
    );

    let mut nested = Pump::new(RecordingRuntime::default());
    assert_eq!(
        nested.mount_view(
            TreeView::new().nodes([TreeNode::new("root", "Root").children([
                TreeNode::new("duplicate", "First"),
                TreeNode::new("duplicate", "Second"),
            ])])
        ),
        Err(PumpError::DuplicateKey(Key::from("duplicate")))
    );
}

#[test]
fn repeated_content_churn_returns_to_the_mounted_tree_baseline() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TreeView::new().nodes([])).unwrap();
    let arena_nodes = pump.tree.len();
    let native_nodes = pump.runtime().node_count();
    for _ in 0..20 {
        pump.update_view(TreeView::new().nodes((0..32).map(|index| {
            TreeNode::new(index.to_string(), format!("Node {index}"))
                .content(TextBlock::new().text(format!("Content {index}")))
        })))
        .unwrap();
        pump.update_view(TreeView::new().nodes([])).unwrap();
        assert_eq!(pump.tree.len(), arena_nodes);
        assert_eq!(pump.runtime().node_count(), native_nodes);
        assert_eq!(pump.runtime().tree_node_count(), 0);
    }
}
