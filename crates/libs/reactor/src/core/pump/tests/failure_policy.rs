use super::super::*;
use super::support::Root;
use crate::test::RecordingRuntime;

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
fn planning_discard_removes_reservations_without_rearming_or_poison() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let (changes, token) = candidate_with_reservation(&mut pump);

    pump.fail_component_candidate(&changes, PlanningFailure::Discard);

    assert!(pump.components.publish(token).is_err());
    assert!(pump.planning_dirty.is_empty());
    assert!(!pump.poisoned());
}

#[test]
fn planning_rearm_removes_reservations_and_retains_touched_scopes() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let (changes, token) = candidate_with_reservation(&mut pump);

    pump.fail_component_candidate(&changes, PlanningFailure::Rearm);

    assert!(pump.components.publish(token).is_err());
    assert!(pump.planning_dirty.contains(&token));
    assert!(!pump.poisoned());
}

#[test]
fn post_planning_abort_removes_reservations_and_fail_stops() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let (changes, token) = candidate_with_reservation(&mut pump);

    pump.abort_frontend_candidate(&FrontendChanges::Component(changes));

    assert!(pump.components.publish(token).is_err());
    assert!(pump.poisoned());
    pump.dirty_components.insert(token);
    pump.native_observation_pending = true;
    assert!(!pump.native_work_pending());
}
