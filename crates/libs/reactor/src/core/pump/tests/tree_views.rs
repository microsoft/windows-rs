use super::super::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

fn nodes(label: &str) -> Vec<TreeNode> {
    vec![TreeNode::new("root", label).expanded(true).children([
        TreeNode::new("first", "First"),
        TreeNode::new("second", "Second"),
    ])]
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
    assert_eq!(
        pump.runtime().tree_nodes(target),
        Some(nodes("Root").as_slice())
    );
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
    assert_eq!(
        pump.runtime().tree_nodes(target),
        Some(nodes("Changed").as_slice())
    );
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
