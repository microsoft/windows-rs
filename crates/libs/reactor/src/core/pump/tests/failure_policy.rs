use super::super::*;
use super::support::Root;
use crate::test::RecordingRuntime;

fn candidate_with_reservation(
    pump: &mut Pump<RecordingRuntime>,
) -> (ComponentChanges, ComponentToken) {
    let token = pump
        .components
        .reserve_component::<Root>("reserved".to_string());
    let mut changes = ComponentChanges::default();
    changes.reserved.push(token);
    changes.touched.insert(token);
    (changes, token)
}

#[test]
fn planning_discard_removes_reservations_without_rearming_or_poison() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let (changes, token) = candidate_with_reservation(&mut pump);

    pump.fail_component_candidate(&changes, PlanningFailure::Discard);

    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pump.components.publish(token);
        }))
        .is_err()
    );
    assert!(pump.planning_dirty.is_empty());
    assert!(!pump.poisoned());
}

#[test]
fn planning_rearm_removes_reservations_and_retains_touched_scopes() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let (changes, token) = candidate_with_reservation(&mut pump);

    pump.fail_component_candidate(&changes, PlanningFailure::Rearm);

    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pump.components.publish(token);
        }))
        .is_err()
    );
    assert!(pump.planning_dirty.contains(&token));
    assert!(!pump.poisoned());
}

#[test]
fn declaration_rejections_are_classified_together() {
    for error in [
        PumpError::DuplicateEffectKey(EffectKey::from("effect")),
        PumpError::DuplicateElementRef,
        PumpError::DuplicateKey(Key::from("key")),
        PumpError::DuplicateColorSchemeObservation,
        PumpError::DuplicateWindowSizeObservation,
        PumpError::DuplicateWindowTitle,
        PumpError::DuplicateWindowTitleBar,
        PumpError::DuplicateWindowVisuals,
        PumpError::ExitTransitionUnsupported,
        PumpError::StructureUnsupported,
    ] {
        assert!(error.is_declaration_rejection());
    }

    for error in [
        PumpError::AlreadyMounted,
        PumpError::NotMounted,
        PumpError::Poisoned,
    ] {
        assert!(!error.is_declaration_rejection());
    }
}
