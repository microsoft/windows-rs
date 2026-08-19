use super::*;
use crate::tests::support::RecordingRuntime;

fn create_window_shell(
    engine: &mut Engine<RecordingRuntime>,
    title: &str,
) -> (NodeId, NodeId, NodeId) {
    let window = engine
        .create_window(WindowCreate {
            title: title.to_string(),
        })
        .unwrap();
    let content = engine.create_logical().unwrap();
    let owned = engine.create_logical().unwrap();
    engine.attach(window, content).unwrap();
    engine.attach(window, owned).unwrap();
    (window, content, owned)
}

#[test]
fn window_content_must_be_the_structurally_owned_projected_root() {
    let mut engine = Engine::new(RecordingRuntime::default());
    let (window, content, _) = create_window_shell(&mut engine, "Main");
    let root = engine.create_native(NativeKind::TextBlock).unwrap();
    engine.attach(content, root).unwrap();
    engine.set_window_content(window, root).unwrap();

    let unrelated = engine.create_native(NativeKind::Button).unwrap();
    assert!(matches!(
        engine.set_window_content(window, unrelated),
        Err(EngineError::InvalidWindowContent {
            window: current_window,
            content: current_content,
        }) if current_window == window && current_content == unrelated
    ));
}

#[test]
fn window_owner_must_match_the_structural_owned_window_slot() {
    let mut engine = Engine::new(RecordingRuntime::default());
    let (owner, _, owned) = create_window_shell(&mut engine, "Owner");
    let (child, _, _) = create_window_shell(&mut engine, "Child");
    engine.attach(owned, child).unwrap();
    engine.set_window_owner(owner, child).unwrap();
    engine.commit().unwrap();

    let mut engine = Engine::new(RecordingRuntime::default());
    let (owner, _, _) = create_window_shell(&mut engine, "Owner");
    let (child, _, _) = create_window_shell(&mut engine, "Unrelated");
    engine.set_window_owner(owner, child).unwrap();
    assert!(matches!(
        engine.commit(),
        Err(EngineError::InvalidWindowOwner {
            owner: current_owner,
            child: current_child,
        }) if current_owner == owner && current_child == child
    ));
}

#[test]
fn reorder_moves_are_minimal_for_small_permutations() {
    let current = (0..7)
        .map(|index| NodeId::new(index, 0))
        .collect::<Vec<_>>();
    let mut desired = current.clone();
    visit_permutations(&mut desired, 0, &mut |desired| {
        let mut working = current.clone();
        let mut moves = 0;
        apply_minimal_reorder(&mut working, desired, |_, _| moves += 1);
        assert_eq!(working, desired);
        assert_eq!(
            moves,
            desired.len() - longest_subsequence(&current, desired)
        );
    });
}

fn visit_permutations(values: &mut [NodeId], index: usize, visit: &mut impl FnMut(&[NodeId])) {
    if index == values.len() {
        visit(values);
        return;
    }
    for next in index..values.len() {
        values.swap(index, next);
        visit_permutations(values, index + 1, visit);
        values.swap(index, next);
    }
}

fn longest_subsequence(current: &[NodeId], desired: &[NodeId]) -> usize {
    let positions = desired
        .iter()
        .map(|id| {
            current
                .iter()
                .position(|candidate| candidate == id)
                .unwrap()
        })
        .collect::<Vec<_>>();
    let mut lengths = vec![1; positions.len()];
    for index in 0..positions.len() {
        for previous in 0..index {
            if positions[previous] < positions[index] {
                lengths[index] = lengths[index].max(lengths[previous] + 1);
            }
        }
    }
    lengths.into_iter().max().unwrap_or(0)
}
