//! Budgeted structural recovery contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Builds a StackPanel with `n` keyed TextBlock children, each with a text
/// property. A recovery plan for this tree produces roughly 3*n+3 commands
/// (ResetWindowContent, n Create, n SetProperty, n InsertChild,
/// SynchronizeChildren, InsertChild root).
fn wide_element(n: usize) -> Element {
    StackPanel::new()
        .children(
            (0..n).map(|index| {
                KeyedElement::new(index as u64, TextBlock::new().text(index.to_string()))
            }),
        )
        .into()
}

#[test]
fn small_recovery_completes_in_one_turn() {
    let before = keyed_text(&["a", "b"]);
    let after: Element = StackPanel::new()
        .child("c", TextBlock::new().text("c"))
        .child("a", TextBlock::new().text("a"))
        .into();

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    pump.runtime_mut().fail_at(0);

    let error = pump.update(after).unwrap_err();
    assert!(
        matches!(error, PumpError::RecoveredStructure(_)),
        "small recovery should complete immediately: {error:?}"
    );
    assert!(!pump.recovery_pending());
    assert!(!pump.poisoned());
}

#[test]
fn each_recovery_batch_respects_command_budget() {
    // Need enough children that recovery generates >RECOVERY_COMMAND_BUDGET
    // commands: ~3 commands per child (Create + SetProperty + InsertChild)
    // plus overhead. 30 children => ~93 commands.
    let n = 30;
    let before = TextBlock::new().text("before").into();
    let after = wide_element(n);

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    // Fail the first structural command of the update to trigger recovery
    pump.runtime_mut().fail_at(0);

    let error = pump.update(after).unwrap_err();
    assert_eq!(error, PumpError::RecoveryPending);
    assert!(pump.recovery_pending());

    // Each batch after mount (batch 0) and failed update (batch 1) is a
    // recovery chunk that must be <= RECOVERY_COMMAND_BUDGET.
    let batches = pump.runtime().commands();
    // batch 2 is the first recovery chunk
    assert!(batches.len() >= 3, "expected at least one recovery batch");
    assert!(
        batches[2].len() <= RECOVERY_COMMAND_BUDGET,
        "first recovery chunk {} exceeds budget {}",
        batches[2].len(),
        RECOVERY_COMMAND_BUDGET
    );
}

#[test]
fn large_recovery_requires_multiple_turns_then_publishes() {
    let n = 50;
    let before = TextBlock::new().text("before").into();
    let after = wide_element(n);

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    pump.runtime_mut().fail_at(0);

    let error = pump.update(after).unwrap_err();
    assert_eq!(error, PumpError::RecoveryPending);
    assert!(pump.native_work_pending());
    let version_before = pump.version();

    // Drive recovery to completion through dispatch_events calls.
    let mut turns = 0;
    loop {
        turns += 1;
        assert!(turns < 100, "recovery did not converge");
        match pump.dispatch_events() {
            Ok(_) => break,
            Err(PumpError::RecoveryPending) => {
                assert!(pump.recovery_pending());
                continue;
            }
            Err(PumpError::RecoveredStructure(_)) => break,
            Err(other) => panic!("unexpected error during recovery: {other:?}"),
        }
    }
    assert!(turns > 1, "expected multi-turn recovery but got {turns}");
    assert!(!pump.recovery_pending());
    assert!(!pump.poisoned());
    assert!(
        pump.version() > version_before,
        "version should advance after recovery"
    );

    // Verify the final tree state matches the desired element.
    let root = pump.root().unwrap();
    let children = pump.runtime().node(root).unwrap().children();
    assert_eq!(children.len(), n);
    for (index, child) in children.iter().enumerate() {
        assert_eq!(
            pump.runtime()
                .node(*child)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str(index.to_string()))
        );
    }
}

#[test]
fn all_recovery_batches_are_within_budget() {
    let n = 50;
    let before = TextBlock::new().text("before").into();
    let after = wide_element(n);

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    pump.runtime_mut().fail_at(0);

    let _ = pump.update(after).unwrap_err();
    let mut turns = 0;
    loop {
        turns += 1;
        if turns > 100 {
            break;
        }
        match pump.dispatch_events() {
            Ok(_) | Err(PumpError::RecoveredStructure(_)) => break,
            Err(PumpError::RecoveryPending) => continue,
            Err(other) => panic!("unexpected: {other:?}"),
        }
    }

    // Every batch after the first two (mount + failed update) is a recovery
    // chunk and must respect the budget.
    for (batch_index, batch) in pump.runtime().commands().iter().enumerate().skip(2) {
        assert!(
            batch.len() <= RECOVERY_COMMAND_BUDGET,
            "recovery batch {batch_index} has {} commands, exceeding budget {RECOVERY_COMMAND_BUDGET}",
            batch.len()
        );
    }
}

#[test]
fn effects_commit_only_after_final_recovery_chunk() {
    #[derive(Clone)]
    struct Props {
        alternate: bool,
        log: Rc<RefCell<Vec<String>>>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            self.alternate == other.alternate && Rc::ptr_eq(&self.log, &other.log)
        }
    }

    struct BigComponent(Props);

    impl Component for BigComponent {
        type Message = ();
        type Props = Props;

        fn create(props: &Props, _cx: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn update(&mut self, _message: (), _cx: &mut ComponentContext<Self>) {}

        fn changed(&mut self, props: &Props, _cx: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn view(&self, cx: &mut ViewContext<Self>) -> View {
            let log = Rc::clone(&self.0.log);
            let alternate = self.0.alternate;
            cx.use_effect(alternate, move || {
                log.borrow_mut().push(format!("setup {alternate}"));
                Some(Box::new(move || {
                    log.borrow_mut().push(format!("cleanup {alternate}"));
                }))
            });
            // Generate enough children that recovery exceeds the budget
            if alternate {
                Element::from(StackPanel::new().children((0..30).map(|index| {
                    KeyedElement::new(index as u64, TextBlock::new().text(index.to_string()))
                })))
                .into()
            } else {
                View::native(TextBlock::new())
            }
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<BigComponent>(Props {
        alternate: false,
        log: Rc::clone(&log),
    }))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup false"]);

    // Force structural failure on the update to trigger recovery
    pump.runtime_mut().fail_at(0);
    let result = pump.update_view(View::component::<BigComponent>(Props {
        alternate: true,
        log: Rc::clone(&log),
    }));

    // Whether this is immediate or multi-turn, effects should only commit
    // after full recovery completes.
    match result {
        Err(PumpError::RecoveredStructure(_)) => {
            // Small recovery completed immediately - effects committed
            assert!(
                log.borrow().contains(&"cleanup false".to_string()),
                "cleanup should have run"
            );
            assert!(
                log.borrow().contains(&"setup true".to_string()),
                "setup should have run after recovery"
            );
        }
        Err(PumpError::RecoveryPending) => {
            // Effects must NOT have committed yet during pending recovery
            assert_eq!(
                &*log.borrow(),
                &["setup false", "cleanup false"],
                "only cleanup should have run before recovery starts; no setup during pending"
            );

            // Drive to completion
            loop {
                match pump.dispatch_events() {
                    Ok(_) | Err(PumpError::RecoveredStructure(_)) => break,
                    Err(PumpError::RecoveryPending) => continue,
                    Err(other) => panic!("unexpected: {other:?}"),
                }
            }
            assert!(
                log.borrow().contains(&"setup true".to_string()),
                "setup should run after final recovery chunk"
            );
        }
        other => panic!("unexpected result: {other:?}"),
    }
}

#[test]
fn stale_native_work_rejected_across_recovery_identity() {
    let n = 50;
    let before = TextBlock::new().text("before").into();
    let after = wide_element(n);

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    let root = pump.root().unwrap();
    let old_identity = pump.native_identity();

    // Queue an event with the old identity
    pump.queue_event(QueuedEvent {
        node: root,
        event: EventId::ButtonClick,
        revision: 0,
        payload: EventPayload::Unit,
    });

    pump.runtime_mut().fail_at(0);
    let _ = pump.update(after).unwrap_err();

    // Complete recovery
    loop {
        match pump.dispatch_events() {
            Ok(_) | Err(PumpError::RecoveredStructure(_)) => break,
            Err(PumpError::RecoveryPending) => continue,
            Err(other) => panic!("unexpected: {other:?}"),
        }
    }

    // The old identity event should not have been dispatched because recovery
    // clears events and advances realization identity.
    assert_ne!(pump.native_identity(), old_identity);
}

#[test]
fn failure_in_later_recovery_chunk_poisons_and_never_publishes() {
    let n = 50;
    let before = TextBlock::new().text("before").into();
    let after = wide_element(n);

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    let version_before = pump.version();
    pump.runtime_mut().fail_at(0);

    let error = pump.update(after).unwrap_err();
    assert_eq!(error, PumpError::RecoveryPending);

    // Inject a failure in the next recovery batch
    pump.runtime_mut().fail_at(0);

    let result = pump.dispatch_events();
    assert!(
        matches!(result, Err(PumpError::RecoveryFailed(_))),
        "expected RecoveryFailed, got {result:?}"
    );
    assert!(pump.poisoned());
    assert_eq!(pump.version(), version_before);
    assert!(!pump.recovery_pending());
}

#[test]
fn no_ordinary_events_run_ahead_of_recovery() {
    let n = 50;
    let dispatched = Rc::new(std::cell::Cell::new(false));
    let dispatched_capture = Rc::clone(&dispatched);
    let before: Element = Button::new()
        .on_click(move || dispatched_capture.set(true))
        .into();
    let after = wide_element(n);

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    let root = pump.root().unwrap();
    let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
    let identity = pump.native_identity();

    // Trigger recovery
    pump.runtime_mut().fail_at(0);
    let _ = pump.update(after).unwrap_err();

    // Queue an event on the new identity - it should be deferred until
    // recovery completes, and since recovery clears events, it won't run.
    pump.queue_event_with_identity(
        identity,
        QueuedEvent {
            node: root,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        },
    );

    // Dispatch one turn - recovery should run first, events are cleared
    match pump.dispatch_events() {
        Err(PumpError::RecoveryPending) | Ok(_) | Err(PumpError::RecoveredStructure(_)) => {}
        Err(other) => panic!("unexpected: {other:?}"),
    }
    assert!(
        !dispatched.get(),
        "event callback should not run during recovery"
    );
}

#[test]
fn updates_blocked_while_recovery_pending() {
    let n = 50;
    let before = TextBlock::new().text("before").into();
    let after = wide_element(n);

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    pump.runtime_mut().fail_at(0);

    let error = pump.update(after).unwrap_err();
    assert_eq!(error, PumpError::RecoveryPending);

    // Attempting another update should be blocked
    assert_eq!(
        pump.update(TextBlock::new().text("blocked").into()),
        Err(PumpError::RecoveryPending)
    );

    // Attempting update_view should also be blocked
    assert_eq!(
        pump.update_view(View::native(TextBlock::new())),
        Err(PumpError::RecoveryPending)
    );

    // Complete recovery and verify updates work again
    loop {
        match pump.dispatch_events() {
            Ok(_) | Err(PumpError::RecoveredStructure(_)) => break,
            Err(PumpError::RecoveryPending) => continue,
            Err(other) => panic!("unexpected: {other:?}"),
        }
    }
    assert!(!pump.recovery_pending());

    // Update should now succeed
    pump.update(TextBlock::new().text("after recovery").into())
        .unwrap();
}

#[test]
fn recovery_pending_reported_as_native_work_pending() {
    let n = 50;
    let before = TextBlock::new().text("before").into();
    let after = wide_element(n);

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(before).unwrap();
    pump.runtime_mut().fail_at(0);

    let _ = pump.update(after).unwrap_err();
    assert!(pump.native_work_pending());
    assert!(pump.recovery_pending());
}

#[test]
fn recovery_pending_is_recoverable() {
    assert!(PumpError::RecoveryPending.recoverable());
}
