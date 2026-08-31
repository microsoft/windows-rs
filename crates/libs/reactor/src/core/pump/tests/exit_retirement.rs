use std::time::Duration;

use super::super::*;
use crate::test::RecordingRuntime;

fn collection_view(exiting: bool, replacement: bool) -> View {
    let mut children = Vec::new();
    if exiting {
        children.push(KeyedView::new(
            "old",
            Border::new()
                .exit_transition(ExitTransition::fade(Duration::from_millis(200)))
                .content(TextBlock::new().text("old")),
        ));
    }
    if replacement {
        children.push(KeyedView::new(
            "new",
            Border::new().content(TextBlock::new().text("new")),
        ));
    }
    children.push(KeyedView::new("tail", TextBlock::new().text("tail")));
    StackPanel::new().keyed_children(children)
}

#[test]
fn exit_retirement_owns_native_subtree_until_completion() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(collection_view(true, false)).unwrap();
    let panel = pump.root().unwrap();
    let old = pump.runtime().node(panel).unwrap().children()[0];
    let old_content = pump.runtime().node(old).unwrap().children()[0];

    pump.update_view(collection_view(false, true)).unwrap();

    assert_eq!(pump.runtime().retained_subtrees(), 1);
    assert!(pump.tree.native(old).is_err());
    assert!(pump.runtime().node(old).is_some());
    assert!(pump.runtime().node(old_content).is_some());
    let children = pump.runtime().node(panel).unwrap().children();
    assert_eq!(children[0], old);
    assert_eq!(children.len(), 3);

    assert!(pump.runtime_mut().complete_retirement(old));
    assert_eq!(pump.runtime().retained_subtrees(), 0);
    assert!(pump.runtime().node(old).is_none());
    assert!(pump.runtime().node(old_content).is_none());
    assert_eq!(pump.runtime().node(panel).unwrap().children().len(), 2);
}

#[test]
fn exit_transition_rejects_single_content_attachment() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Border::new()
            .exit_transition(ExitTransition::fade(Duration::from_millis(200)))
            .into(),
    )
    .unwrap();

    assert_eq!(
        pump.update(TextBlock::new().into()),
        Err(PumpError::ExitTransitionUnsupported)
    );
}
