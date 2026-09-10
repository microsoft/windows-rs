use super::*;

const ROOT: NodeId = NodeId::from_parts(0, 0);
const CHILD: NodeId = NodeId::from_parts(1, 0);

#[test]
fn command_history_can_be_disabled() {
    let mut runtime = RecordingRuntime::default();
    runtime.record_commands(false);
    runtime
        .apply(&[Command::Create {
            node: ROOT,
            kind: MountedKind::TextBlock,
        }])
        .unwrap();

    assert!(runtime.commands().is_empty());
    assert!(runtime.node(ROOT).is_some());
}

#[test]
fn records_native_property_observations_without_commands() {
    let mut runtime = RecordingRuntime::default();
    runtime
        .apply(&[Command::Create {
            node: ROOT,
            kind: MountedKind::TextBox,
        }])
        .unwrap();
    let batches = runtime.batches();

    runtime
        .record_property_observation(
            ROOT,
            PropertyId::TextBoxText,
            PropertyValue::Str("edited".into()),
        )
        .unwrap();

    assert_eq!(runtime.batches(), batches);
    assert_eq!(
        runtime
            .node(ROOT)
            .unwrap()
            .property(PropertyId::TextBoxText),
        Some(&PropertyValue::Str("edited".into()))
    );
}

#[test]
fn records_tree_and_property_mutations() {
    let mut runtime = RecordingRuntime::default();
    runtime
        .apply(&[
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
            Command::SetProperty {
                node: CHILD,
                property: PropertyId::TextBlockText,
                value: PropertyValue::Str("hello".into()),
            },
            Command::InsertChild {
                parent: ROOT,
                slot: None,
                child: CHILD,
                index: 0,
            },
        ])
        .unwrap();
    assert_eq!(runtime.batches, 1);
    assert_eq!(
        runtime.node(ROOT).unwrap().kind(),
        Some(MountedKind::StackPanel)
    );
    assert_eq!(runtime.node(ROOT).unwrap().children, [CHILD]);
    assert_eq!(runtime.node(CHILD).unwrap().parent, Some(ROOT));
    assert_eq!(
        runtime.node(CHILD).unwrap().properties[&PropertyId::TextBlockText],
        PropertyValue::Str("hello".into())
    );
}

#[test]
fn attach_realized_replaces_content_for_the_same_shell_lifetime() {
    let mut runtime = RecordingRuntime::default();
    let second = NodeId::from_parts(2, 0);
    let container = RealizedContainer(7);
    runtime
        .apply(&[
            Command::CreateVirtualCollection {
                node: ROOT,
                item_count: 1,
                source_revision: 0,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
            Command::Create {
                node: second,
                kind: MountedKind::Button,
            },
            Command::AttachRealized {
                collection: ROOT,
                container,
                child: CHILD,
            },
            Command::AttachRealized {
                collection: ROOT,
                container,
                child: second,
            },
        ])
        .unwrap();

    assert_eq!(runtime.node(ROOT).unwrap().children(), &[second]);
    assert_eq!(runtime.node(CHILD).unwrap().parent, None);
    assert_eq!(runtime.node(second).unwrap().parent, Some(ROOT));

    runtime
        .apply(&[Command::DetachRealized {
            collection: ROOT,
            container,
            child: second,
        }])
        .unwrap();
    assert!(runtime.node(ROOT).unwrap().children().is_empty());
}

#[test]
fn records_clear_move_remove_and_child_first_destroy() {
    let mut runtime = RecordingRuntime::default();
    let second = NodeId::from_parts(2, 0);
    runtime
        .apply(&[
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
            Command::Create {
                node: second,
                kind: MountedKind::TextBlock,
            },
            Command::InsertChild {
                parent: ROOT,
                slot: None,
                child: CHILD,
                index: 0,
            },
            Command::InsertChild {
                parent: ROOT,
                slot: None,
                child: second,
                index: 1,
            },
            Command::SetProperty {
                node: CHILD,
                property: PropertyId::TextBlockText,
                value: PropertyValue::Str("temporary".into()),
            },
        ])
        .unwrap();

    runtime
        .apply(&[
            Command::ClearProperty {
                node: CHILD,
                property: PropertyId::TextBlockText,
            },
            Command::MoveChild {
                parent: ROOT,
                slot: None,
                child: second,
                index: 0,
            },
            Command::RemoveChild {
                parent: ROOT,
                slot: None,
                child: CHILD,
            },
            Command::Destroy { node: CHILD },
        ])
        .unwrap();
    assert_eq!(runtime.node(ROOT).unwrap().children, [second]);
    assert!(runtime.node(CHILD).is_none());
}

#[test]
fn tooltip_attachments_are_exclusive_ownership() {
    let tooltip = NodeId::from_parts(2, 0);
    let other = NodeId::from_parts(3, 0);
    let mut runtime = RecordingRuntime::default();
    runtime
        .apply(&[
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
            Command::Create {
                node: tooltip,
                kind: MountedKind::ToolTip,
            },
            Command::Create {
                node: other,
                kind: MountedKind::TextBlock,
            },
            Command::SetTooltip {
                target: CHILD,
                tooltip: Some(tooltip),
                placement: TooltipPlacement::Top,
            },
        ])
        .unwrap();

    for command in [
        Command::InsertChild {
            parent: ROOT,
            slot: None,
            child: tooltip,
            index: 0,
        },
        Command::SetSlot {
            parent: ROOT,
            slot: SlotId::TextBoxHeader,
            child: Some(tooltip),
        },
        Command::AttachRealized {
            collection: ROOT,
            container: RealizedContainer(0),
            child: tooltip,
        },
        Command::SetTooltip {
            target: other,
            tooltip: Some(tooltip),
            placement: TooltipPlacement::Top,
        },
        Command::SetTooltip {
            target: tooltip,
            tooltip: Some(other),
            placement: TooltipPlacement::Top,
        },
    ] {
        assert_eq!(
            runtime.apply(&[command]).unwrap_err().error,
            RuntimeError::AlreadyParented(tooltip)
        );
    }
    assert_eq!(
        runtime
            .apply(&[Command::SetTooltip {
                target: CHILD,
                tooltip: Some(CHILD),
                placement: TooltipPlacement::Top,
            }])
            .unwrap_err()
            .error,
        RuntimeError::SelfParent(CHILD)
    );
}

#[test]
fn attached_tooltip_and_target_must_be_cleared_before_destroy() {
    let tooltip = NodeId::from_parts(2, 0);
    let mut runtime = RecordingRuntime::default();
    runtime
        .apply(&[
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
            Command::Create {
                node: tooltip,
                kind: MountedKind::ToolTip,
            },
            Command::SetTooltip {
                target: CHILD,
                tooltip: Some(tooltip),
                placement: TooltipPlacement::Top,
            },
        ])
        .unwrap();

    for node in [CHILD, tooltip] {
        assert_eq!(
            runtime
                .apply(&[Command::Destroy { node }])
                .unwrap_err()
                .error,
            RuntimeError::StillParented(node)
        );
    }

    runtime
        .apply(&[
            Command::SetTooltip {
                target: CHILD,
                tooltip: None,
                placement: TooltipPlacement::Top,
            },
            Command::Destroy { node: tooltip },
            Command::Destroy { node: CHILD },
        ])
        .unwrap();
}

#[test]
fn flyout_content_is_exclusively_owned_until_cleared() {
    let content = NodeId::from_parts(2, 0);
    let other = NodeId::from_parts(3, 0);
    let mut runtime = RecordingRuntime::default();
    runtime
        .apply(&[
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::Button,
            },
            Command::Create {
                node: content,
                kind: MountedKind::TextBlock,
            },
            Command::Create {
                node: other,
                kind: MountedKind::Button,
            },
            Command::SetFlyout {
                target: CHILD,
                content: Some(content),
                placement: FlyoutPlacement::Bottom,
            },
        ])
        .unwrap();

    for command in [
        Command::InsertChild {
            parent: ROOT,
            slot: None,
            child: content,
            index: 0,
        },
        Command::SetTooltip {
            target: other,
            tooltip: Some(content),
            placement: TooltipPlacement::Top,
        },
        Command::SetFlyout {
            target: other,
            content: Some(content),
            placement: FlyoutPlacement::Top,
        },
        Command::Destroy { node: CHILD },
        Command::Destroy { node: content },
    ] {
        assert!(matches!(
            runtime.apply(&[command]).unwrap_err().error,
            RuntimeError::AlreadyParented(_) | RuntimeError::StillParented(_)
        ));
    }

    runtime
        .apply(&[
            Command::SetFlyout {
                target: CHILD,
                content: None,
                placement: FlyoutPlacement::Bottom,
            },
            Command::Destroy { node: content },
            Command::Destroy { node: CHILD },
        ])
        .unwrap();
}

#[test]
fn failure_stops_before_later_commands() {
    let mut runtime = RecordingRuntime::default();
    let error = runtime
        .apply(&[
            Command::SetProperty {
                node: CHILD,
                property: PropertyId::TextBlockText,
                value: PropertyValue::Str("missing".into()),
            },
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
        ])
        .unwrap_err();

    assert_eq!(
        error,
        NativeApplyError {
            command: 0,
            error: RuntimeError::MissingNode(CHILD),
        }
    );
    assert!(runtime.node(ROOT).is_none());
}

#[test]
fn structural_failure_skips_dependent_commands() {
    let mut runtime = RecordingRuntime::default();
    runtime.fail_at(0);

    let error = runtime
        .apply(&[
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
        ])
        .unwrap_err();

    assert_eq!(
        error,
        NativeApplyError {
            command: 0,
            error: RuntimeError::Injected,
        }
    );
    assert!(runtime.nodes.is_empty());
}
