//! Mount, update, and publication contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::rc::Rc;

#[test]
fn component_slot_adapters_reject_incompatible_control_roles() {
    let mut children = Pump::new(RecordingRuntime::default());
    assert_eq!(
        children.mount_view(View::Children {
            control: TextBlock::new().into(),
            children: Rc::new(Vec::new()),
        }),
        Err(PumpError::StructureUnsupported)
    );
    assert!(!children.poisoned());

    let mut content = Pump::new(RecordingRuntime::default());
    assert_eq!(
        content.mount_view(View::Content {
            control: StackPanel::new().into(),
            content: Box::new(View::native(TextBlock::new())),
        }),
        Err(PumpError::StructureUnsupported)
    );
    assert!(!content.poisoned());
}

#[test]
fn empty_root_mounts_without_native_window_content_and_can_toggle() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::Empty).unwrap();
    let root = pump.root().unwrap();
    let window = pump.window().unwrap();

    assert_eq!(pump.tree.kind(root), Ok(NodeKind::Fragment));
    assert!(pump.runtime().node(window).unwrap().children().is_empty());

    pump.update_view(View::native(TextBlock::new().text("visible")))
        .unwrap();
    assert_eq!(pump.runtime().node(window).unwrap().children().len(), 1);
    pump.update_view(View::Empty).unwrap();
    assert!(pump.runtime().node(window).unwrap().children().is_empty());
}

#[test]
fn failed_create_does_not_publish_a_root() {
    let mut runtime = RecordingRuntime::default();
    runtime.fail_at(0);
    let mut pump = Pump::new(runtime);

    let failed = structural_receipt(
        pump.mount(TextBlock::new().text("first").into())
            .unwrap_err(),
    );

    assert_eq!(
        failed.outcomes,
        [
            CommandOutcome::Failed(RuntimeError::Injected),
            CommandOutcome::Skipped,
            CommandOutcome::Skipped,
            CommandOutcome::Skipped,
            CommandOutcome::Skipped,
            CommandOutcome::Skipped,
        ]
    );
    assert_eq!(pump.root(), None);
    assert!(pump.runtime().is_empty());
    assert_eq!(pump.version(), 0);
    assert!(!pump.retry_pending());
    assert!(pump.poisoned());
    assert_eq!(
        pump.mount(TextBlock::new().text("first").into()),
        Err(PumpError::Poisoned)
    );
}

#[test]
fn duplicate_mount_keys_fail_before_native_apply() {
    let mut pump = Pump::new(RecordingRuntime::default());

    assert_eq!(
        pump.mount(
            StackPanel::new()
                .child("duplicate", TextBlock::new())
                .child("duplicate", TextBlock::new())
                .into()
        ),
        Err(PumpError::DuplicateKey(Key::from("duplicate")))
    );
    assert_eq!(pump.runtime().batches(), 0);
    assert!(pump.runtime().is_empty());
    assert_eq!(pump.root(), None);
}

#[test]
fn mounts_content_and_keyed_children_recursively() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let tree = StackPanel::new()
        .child("text", TextBlock::new().text("value"))
        .child(
            "button",
            Button::new().content(TextBlock::new().text("increment")),
        );

    pump.mount(tree.into()).unwrap();

    let root = pump.root().unwrap();
    let children = pump.runtime().node(root).unwrap().children();
    assert_eq!(children.len(), 2);
    let button = children[1];
    assert_eq!(pump.runtime().node(button).unwrap().children().len(), 1);
}

#[test]
fn application_window_and_root_share_one_arena_lifetime() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBlock::new().text("root").into()).unwrap();
    let application = pump.application().unwrap();
    let window = pump.window().unwrap();
    let root = pump.root().unwrap();

    assert_eq!(pump.tree.parent(application), Ok(None));
    assert_eq!(pump.tree.parent(window), Ok(Some(application)));
    assert_eq!(pump.tree.parent(root), Ok(Some(window)));
    assert!(pump.runtime().node(application).is_some());
    assert!(pump.runtime().node(window).is_some());
    assert!(pump.runtime().node(root).is_some());

    pump.shutdown();

    assert_eq!(pump.application(), None);
    assert_eq!(pump.window(), None);
    assert_eq!(pump.root(), None);
    assert_eq!(pump.version(), 0);
    assert!(pump.runtime().is_empty());
}

#[test]
fn structural_mount_failure_removes_created_nodes() {
    let mut runtime = RecordingRuntime::default();
    runtime.fail_at(1);
    let mut pump = Pump::new(runtime);
    let tree = StackPanel::new().child("text", TextBlock::new().text("value"));

    let failed = structural_receipt(pump.mount(tree.into()).unwrap_err());

    assert!(matches!(
        failed.outcomes[1],
        CommandOutcome::Failed(RuntimeError::Injected)
    ));
    assert_eq!(pump.root(), None);
    assert!(pump.runtime().is_empty());
    assert!(pump.poisoned());
}

#[test]
fn root_kind_replacement_updates_arena_and_native_parent() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBlock::new().text("first").into()).unwrap();
    let old = pump.root().unwrap();
    let window = pump.window().unwrap();

    pump.update(Button::new().into()).unwrap();

    let root = pump.root().unwrap();
    assert_ne!(root, old);
    assert_eq!(
        pump.tree.kind(root),
        Ok(NodeKind::Native(MountedKind::Button))
    );
    assert_eq!(pump.runtime().node(window).unwrap().children(), &[root]);
}

#[test]
fn content_transitions_support_insert_replace_and_remove() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Button::new().into()).unwrap();
    let root = pump.root().unwrap();

    pump.update(Button::new().content(TextBlock::new().text("text")).into())
        .unwrap();
    let text = pump.tree.children(root).unwrap()[0];
    assert_eq!(
        pump.tree.kind(text),
        Ok(NodeKind::Native(MountedKind::TextBlock))
    );

    pump.update(Button::new().content(Button::new()).into())
        .unwrap();
    let button = pump.tree.children(root).unwrap()[0];
    assert_ne!(button, text);
    assert_eq!(
        pump.tree.kind(button),
        Ok(NodeKind::Native(MountedKind::Button))
    );

    pump.update(Button::new().into()).unwrap();
    assert!(pump.tree.children(root).unwrap().is_empty());
    assert!(pump.runtime().node(root).unwrap().children().is_empty());
}
