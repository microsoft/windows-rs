use super::super::*;
use super::support::Root;
use crate::native::RecordingRuntime;

fn candidate_with_reservation(
    pump: &mut Pump<RecordingRuntime>,
) -> (ComponentChanges, ComponentToken) {
    let token = pump
        .components
        .reserve_component::<Root>("reserved".to_string())
        .unwrap();
    let mut changes = ComponentChanges::default();
    changes.reserved.push(token);
    changes.touched.insert(token);
    (changes, token)
}

#[test]
fn planning_discard_removes_reservations_without_retry_or_poison() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let (changes, token) = candidate_with_reservation(&mut pump);

    pump.fail_component_candidate(&changes, CandidateFailureStage::PlanningDiscard);

    assert!(pump.components.publish(token).is_err());
    assert!(pump.planning_dirty.is_empty());
    assert!(!pump.poisoned());
}

#[test]
fn planning_retry_removes_reservations_and_retains_touched_scopes() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let (changes, token) = candidate_with_reservation(&mut pump);

    pump.fail_component_candidate(&changes, CandidateFailureStage::PlanningRetry);

    assert!(pump.components.publish(token).is_err());
    assert!(pump.planning_dirty.contains(&token));
    assert!(!pump.poisoned());
}

#[test]
fn post_planning_failures_remove_reservations_and_fail_stop() {
    for stage in [
        CandidateFailureStage::EffectPreparation,
        CandidateFailureStage::NativeApply,
        CandidateFailureStage::Publication,
    ] {
        let mut pump = Pump::new(RecordingRuntime::default());
        let (changes, token) = candidate_with_reservation(&mut pump);

        pump.fail_component_candidate(&changes, stage);

        assert!(pump.components.publish(token).is_err());
        assert!(pump.poisoned());
        pump.dirty_components.insert(token);
        pump.native_observation_pending = true;
        assert!(!pump.native_work_pending());
    }
}
