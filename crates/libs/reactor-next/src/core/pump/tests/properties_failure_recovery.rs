//! Property application, failure, and recovery contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::collections::HashSet;

#[test]
fn mount_update_clear_and_no_change_follow_receipts() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let mounted = pump.mount(TextBlock::new().text("first").into()).unwrap();
    let root = pump.root().unwrap();

    assert_eq!(mounted.outcomes.len(), 6);
    assert!(pump.application().is_some());
    assert!(pump.window().is_some());
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("first".into()))
    );

    let updated = pump.update(TextBlock::new().text("second").into()).unwrap();
    assert_eq!(updated.outcomes, [CommandOutcome::Applied]);
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("second".into()))
    );

    let batches = pump.runtime().batches();
    let unchanged = pump.update(TextBlock::new().text("second").into()).unwrap();
    assert!(unchanged.outcomes.is_empty());
    assert_eq!(pump.runtime().batches(), batches);

    let cleared = pump.update(TextBlock::new().into()).unwrap();
    assert_eq!(cleared.outcomes, [CommandOutcome::Applied]);
    assert!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBlockText)
            .is_none()
    );
}

#[test]
fn failed_property_is_not_committed_and_retries() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBlock::new().text("first").into()).unwrap();
    let root = pump.root().unwrap();
    let version = pump.version();
    pump.runtime_mut().fail_at(0);

    let failed = pump
        .update(TextBlock::new().text("second").into())
        .unwrap_err();
    let PumpError::PropertyApplyFailed(failed) = failed else {
        panic!("expected property apply failure");
    };
    assert_eq!(
        failed.outcomes,
        [CommandOutcome::Failed(RuntimeError::Injected)]
    );
    assert_eq!(pump.version(), version);
    assert!(pump.retry_pending());
    assert_eq!(
        pump.tree
            .native(root)
            .unwrap()
            .properties
            .get(&PropertyId::TextBlockText),
        Some(&NativePropertyState::Divergent { attempts: 1 })
    );
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("first".into()))
    );

    let retried = pump.update(TextBlock::new().text("second").into()).unwrap();
    assert_eq!(retried.outcomes, [CommandOutcome::Applied]);
    assert_eq!(pump.version(), version + 1);
    assert!(!pump.retry_pending());
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("second".into()))
    );
    assert_eq!(
        pump.tree
            .native(root)
            .unwrap()
            .properties
            .get(&PropertyId::TextBlockText),
        Some(&NativePropertyState::Known(Some(PropertyValue::Str(
            "second".into()
        ))))
    );
}

#[test]
fn property_retry_exhaustion_is_tracked_by_the_property() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBlock::new().text("first").into()).unwrap();

    for attempt in 1..=MAX_PROPERTY_ATTEMPTS {
        pump.runtime_mut().fail_at(0);
        let error = pump
            .update(TextBlock::new().text("second").into())
            .unwrap_err();
        if attempt < MAX_PROPERTY_ATTEMPTS {
            assert!(matches!(error, PumpError::PropertyApplyFailed(_)));
        } else {
            assert!(matches!(error, PumpError::PropertyRetriesExhausted(_)));
            assert!(!error.recoverable());
        }
    }
}

#[test]
fn malformed_update_receipt_poisons_without_advancing_version() {
    let mut pump = Pump::new(ShortReceiptRuntime::default());
    pump.mount(TextBlock::new().text("first").into()).unwrap();
    let version = pump.version();
    pump.runtime_mut().short_next = true;

    assert_eq!(
        pump.update(TextBlock::new().text("second").into()),
        Err(PumpError::ApplyReceiptMismatch)
    );
    assert_eq!(pump.version(), version);
    assert!(pump.poisoned());
}

#[test]
fn every_mount_command_failure_reaches_a_defined_state() {
    let mut baseline = Pump::new(RecordingRuntime::default());
    let command_count = baseline
        .mount(representative_tree())
        .unwrap()
        .outcomes
        .len();
    assert!(command_count > 1);

    for failed_index in 0..command_count {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(failed_index);
        let mut pump = Pump::new(runtime);
        let error = pump.mount(representative_tree()).unwrap_err();

        match error {
            PumpError::PropertyApplyFailed(receipt) => {
                assert!(matches!(
                    receipt.outcomes[failed_index],
                    CommandOutcome::Failed(RuntimeError::Injected)
                ));
                assert_eq!(pump.version(), 0);
                assert!(pump.retry_pending());
                assert!(!pump.poisoned());
                assert!(pump.root().is_some());

                pump.update(representative_tree()).unwrap();
                assert_eq!(pump.version(), 1);
                assert!(!pump.retry_pending());
            }
            PumpError::StructuralApplyFailed(receipt) => {
                assert!(matches!(
                    receipt.outcomes[failed_index],
                    CommandOutcome::Failed(RuntimeError::Injected)
                ));
                assert_eq!(pump.version(), 0);
                assert!(!pump.retry_pending());
                assert!(pump.poisoned());
                assert_eq!(pump.root(), None);
                assert!(pump.runtime().is_empty());
                assert_eq!(pump.mount(representative_tree()), Err(PumpError::Poisoned));
            }
            error => panic!("unexpected mount failure: {error:?}"),
        }
    }
}

#[test]
fn every_update_command_failure_reaches_a_defined_state() {
    let before = keyed_text(&["a", "b", "c"]);
    let after: Element = StackPanel::new()
        .child("c", TextBlock::new().text("c updated"))
        .child("d", TextBlock::new().text("d"))
        .child("a", TextBlock::new().text("a"))
        .into();
    let mut baseline = Pump::new(RecordingRuntime::default());
    baseline.mount(before.clone()).unwrap();
    let command_count = baseline.update(after.clone()).unwrap().outcomes.len();
    assert!(command_count > 1);

    for failed_index in 0..command_count {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(before.clone()).unwrap();
        let version = pump.version();
        let old_root = pump.root().unwrap();
        let application = pump.application();
        let window = pump.window();
        pump.runtime_mut().fail_at(failed_index);
        let error = pump.update(after.clone()).unwrap_err();

        match error {
            PumpError::PropertyApplyFailed(receipt) => {
                assert!(matches!(
                    receipt.outcomes[failed_index],
                    CommandOutcome::Failed(RuntimeError::Injected)
                ));
                assert_eq!(pump.version(), version);
                assert!(pump.retry_pending());
                assert!(!pump.poisoned());
                assert_eq!(
                    arena_keys(&pump),
                    [Key::from("c"), Key::from("d"), Key::from("a")]
                );

                pump.update(after.clone()).unwrap();
                assert_eq!(pump.version(), version + 1);
                assert!(!pump.retry_pending());
                assert_eq!(
                    recorded_text(pump.runtime(), pump.root().unwrap())[0],
                    "c updated"
                );
            }
            PumpError::RecoveredStructure(recovery) => {
                assert!(matches!(
                    recovery.failure.outcomes[failed_index],
                    CommandOutcome::Failed(RuntimeError::Injected)
                ));
                assert_eq!(pump.version(), version + 1);
                assert!(!pump.retry_pending());
                assert!(!pump.poisoned());
                assert_ne!(pump.root(), Some(old_root));
                assert_eq!(pump.application(), application);
                assert_eq!(pump.window(), window);
                assert_eq!(
                    arena_keys(&pump),
                    [Key::from("c"), Key::from("d"), Key::from("a")]
                );
                assert_eq!(
                    recorded_text(pump.runtime(), pump.root().unwrap())[0],
                    "c updated"
                );
            }
            error => panic!("unexpected update failure: {error:?}"),
        }
    }
}

#[test]
fn every_recovery_command_failure_reaches_a_defined_state() {
    let before = keyed_text(&["a", "b", "c"]);
    let after: Element = StackPanel::new()
        .child("c", TextBlock::new().text("c updated"))
        .child("d", TextBlock::new().text("d"))
        .child("a", TextBlock::new().text("a"))
        .into();
    let mut baseline = Pump::new(RecordingRuntime::default());
    baseline.mount(before.clone()).unwrap();
    baseline.runtime_mut().fail_at(1);
    let recovered = recovered_structure(baseline.update(after.clone()).unwrap_err());
    let command_count = recovered.recovery.outcomes.len();
    let mut saw_property = false;
    let mut saw_structural = false;

    for failed_index in 0..command_count {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(before.clone()).unwrap();
        let version = pump.version();
        let old_root = pump.root();
        pump.runtime_mut().fail_at(1);
        pump.runtime_mut().fail_after(1, failed_index);

        match pump.update(after.clone()).unwrap_err() {
            PumpError::RecoveredStructure(recovery) => {
                saw_property = true;
                assert!(matches!(
                    recovery.recovery.outcomes[failed_index],
                    CommandOutcome::Failed(RuntimeError::Injected)
                ));
                assert_eq!(pump.version(), version);
                assert!(pump.retry_pending());
                assert!(!pump.poisoned());
                assert_ne!(pump.root(), old_root);
            }
            PumpError::RecoveryFailed(recovery) => {
                saw_structural = true;
                assert!(matches!(
                    recovery.recovery.outcomes[failed_index],
                    CommandOutcome::Failed(RuntimeError::Injected)
                ));
                assert_eq!(pump.version(), version);
                assert!(!pump.retry_pending());
                assert!(pump.poisoned());
                assert_eq!(pump.root(), old_root);
            }
            error => panic!("unexpected recovery failure: {error:?}"),
        }
    }

    assert!(saw_property);
    assert!(saw_structural);
}

#[test]
fn recovery_does_not_reuse_ids_created_by_failed_batch() {
    let before = keyed_text(&["a", "b", "c"]);
    let after: Element = StackPanel::new()
        .child("c", TextBlock::new().text("c"))
        .child("d", TextBlock::new().text("d"))
        .child("a", TextBlock::new().text("a"))
        .into();
    let mut probe = Pump::new(RecordingRuntime::default());
    probe.mount(before.clone()).unwrap();
    probe.update(after.clone()).unwrap();
    let update = &probe.runtime().commands()[1];
    let failed_index = update.iter().rposition(Command::structural).unwrap();

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    pump.runtime_mut().fail_at(failed_index);
    assert!(matches!(
        pump.update(after),
        Err(PumpError::RecoveredStructure(_))
    ));
    let batches = pump.runtime().commands();
    let failed_created = batches[1]
        .iter()
        .filter_map(|command| match command {
            Command::Create { node, .. } => Some(*node),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let recovered_created = batches[2]
        .iter()
        .filter_map(|command| match command {
            Command::Create { node, .. } => Some(*node),
            _ => None,
        })
        .collect::<HashSet<_>>();

    assert!(!failed_created.is_empty());
    assert!(failed_created.is_disjoint(&recovered_created));
}

#[test]
fn failed_root_replacement_recovers_from_candidate_root() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBlock::new().text("first").into()).unwrap();
    let old = pump.root().unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update(Button::new().into()),
        Err(PumpError::RecoveredStructure(_))
    ));
    let root = pump.root().unwrap();
    assert_ne!(root, old);
    assert_eq!(
        pump.tree.kind(root),
        Ok(NodeKind::Native(MountedKind::Button))
    );
}

#[test]
fn every_root_replacement_command_failure_recovers_desired_kind() {
    let command_count = {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(TextBlock::new().into()).unwrap();
        pump.update(Button::new().into()).unwrap();
        pump.runtime().commands().last().unwrap().len()
    };

    for failed in 0..command_count {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(TextBlock::new().into()).unwrap();
        pump.runtime_mut().fail_at(failed);

        assert!(matches!(
            pump.update(Button::new().into()),
            Err(PumpError::RecoveredStructure(_))
        ));
        assert_eq!(
            pump.tree.kind(pump.root().unwrap()),
            Ok(NodeKind::Native(MountedKind::Button)),
            "command {failed}"
        );
        assert!(!pump.poisoned(), "command {failed}");
    }
}

#[test]
fn every_content_replacement_command_failure_recovers_desired_tree() {
    let before = || {
        Button::new()
            .content(TextBlock::new().text("before"))
            .into()
    };
    let after = || Button::new().content(Button::new()).into();
    let command_count = {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(before()).unwrap();
        pump.update(after()).unwrap();
        pump.runtime().commands().last().unwrap().len()
    };

    for failed in 0..command_count {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(before()).unwrap();
        pump.runtime_mut().fail_at(failed);

        assert!(matches!(
            pump.update(after()),
            Err(PumpError::RecoveredStructure(_))
        ));
        let root = pump.root().unwrap();
        let child = pump.tree.children(root).unwrap()[0];
        assert_eq!(
            pump.tree.kind(child),
            Ok(NodeKind::Native(MountedKind::Button)),
            "command {failed}"
        );
        assert!(!pump.poisoned(), "command {failed}");
    }
}
