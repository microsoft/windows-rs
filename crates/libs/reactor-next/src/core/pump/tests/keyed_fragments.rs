//! Keyed reconciliation and fragment contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::collections::HashSet;

#[test]
fn multi_root_fragment_is_rejected_in_window_and_content_slots() {
    let fragment = || {
        View::fragment([
            KeyedView::new("a", View::native(TextBlock::new().text("A"))),
            KeyedView::new("b", View::native(TextBlock::new().text("B"))),
        ])
    };
    let mut window = Pump::new(RecordingRuntime::default());
    assert_eq!(
        window.mount_view(fragment()),
        Err(PumpError::StructureUnsupported)
    );
    assert!(window.root().is_none());

    let mut content = Pump::new(RecordingRuntime::default());
    assert_eq!(
        content.mount_view(View::content(Button::new(), fragment())),
        Err(PumpError::StructureUnsupported)
    );
    assert!(content.root().is_none());
}

#[test]
fn fragment_splices_into_children_and_retains_keyed_component_scope() {
    let view = |reverse: bool| {
        let fragment = if reverse {
            View::fragment([
                KeyedView::new("text", View::native(TextBlock::new().text("text"))),
                KeyedView::new("leaf", View::component::<Leaf>("leaf".to_string())),
            ])
        } else {
            View::fragment([
                KeyedView::new("leaf", View::component::<Leaf>("leaf".to_string())),
                KeyedView::new("text", View::native(TextBlock::new().text("text"))),
            ])
        };
        View::children(
            StackPanel::new(),
            [
                KeyedView::new("empty", View::empty()),
                KeyedView::new("group", fragment),
            ],
        )
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view(false)).unwrap();
    let root = pump.root().unwrap();
    let fragment = pump.tree.children(root).unwrap()[1];
    let leaf = pump.tree.children(fragment).unwrap()[0];
    let scope = pump.tree.component_scope(leaf).unwrap();

    assert_eq!(recorded_text(pump.runtime(), root), ["leaf", "text"]);
    pump.update_view(view(true)).unwrap();

    let fragment = pump.tree.children(root).unwrap()[1];
    let leaf = pump.tree.children(fragment).unwrap()[1];
    assert_eq!(pump.tree.component_scope(leaf), Ok(scope));
    assert_eq!(recorded_text(pump.runtime(), root), ["text", "leaf"]);
}

#[test]
fn fragment_synchronization_failure_is_fatal_without_publishing_candidate() {
    let view = |reverse: bool| {
        let children = if reverse {
            [
                KeyedView::new("b", View::native(TextBlock::new().text("B"))),
                KeyedView::new("a", View::native(TextBlock::new().text("A"))),
            ]
        } else {
            [
                KeyedView::new("a", View::native(TextBlock::new().text("A"))),
                KeyedView::new("b", View::native(TextBlock::new().text("B"))),
            ]
        };
        View::children(
            StackPanel::new(),
            [KeyedView::new("fragment", View::fragment(children))],
        )
    };
    let mut probe = Pump::new(RecordingRuntime::default());
    probe.mount_view(view(false)).unwrap();
    probe.update_view(view(true)).unwrap();
    let synchronize = probe.runtime().commands()[1]
        .iter()
        .enumerate()
        .filter(|(_, command)| matches!(command, Command::SynchronizeChildren { .. }))
        .collect::<Vec<_>>();
    assert_eq!(synchronize.len(), 1);
    let failed_index = synchronize[0].0;

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view(false)).unwrap();
    pump.runtime_mut().fail_at(failed_index);
    assert_eq!(
        pump.update_view(view(true)),
        Err(PumpError::NativeApplyFailed(NativeApplyError {
            command: failed_index,
            error: RuntimeError::Injected,
        }))
    );
    assert_eq!(
        recorded_text(pump.runtime(), pump.root().unwrap()),
        ["A", "B"]
    );
    assert!(pump.poisoned());
}

#[test]
fn keyed_reorder_moves_survivors_without_recreation() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(keyed_text(&["a", "b", "c", "d"])).unwrap();
    let root = pump.root().unwrap();

    pump.update(keyed_text(&["d", "c", "b", "a"])).unwrap();
    assert_eq!(recorded_text(pump.runtime(), root), ["d", "c", "b", "a"]);
}

#[test]
fn dense_keyed_reorder_resets_collection_without_recreating_children() {
    let labels = (0..512).map(|index| index.to_string()).collect::<Vec<_>>();
    let mut reversed = labels.clone();
    reversed.reverse();
    let element = |labels: &[String]| {
        StackPanel::new()
            .children(
                labels
                    .iter()
                    .map(|label| KeyedElement::new(label.clone(), TextBlock::new().text(label))),
            )
            .into()
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(element(&labels)).unwrap();
    let root = pump.root().unwrap();
    let original = pump
        .tree
        .children(root)
        .unwrap()
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    pump.update(element(&reversed)).unwrap();

    assert!(pump.runtime().commands()[1].contains(&Command::ResetChildren { parent: root }));
    assert_eq!(
        pump.tree
            .children(root)
            .unwrap()
            .iter()
            .copied()
            .collect::<HashSet<_>>(),
        original
    );
    assert_eq!(
        pump.runtime().node(root).unwrap().children(),
        pump.tree.children(root).unwrap()
    );
}

#[test]
fn retained_key_recurses_into_property_update() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        StackPanel::new()
            .child("value", TextBlock::new().text("first"))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();

    pump.update(
        StackPanel::new()
            .child("value", TextBlock::new().text("second"))
            .into(),
    )
    .unwrap();
    assert_eq!(recorded_text(pump.runtime(), root), ["second"]);
}

#[test]
fn keyed_insert_mounts_only_the_new_subtree() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(keyed_text(&["a", "c"])).unwrap();
    let root = pump.root().unwrap();

    pump.update(keyed_text(&["a", "b", "c"])).unwrap();
    assert_eq!(recorded_text(pump.runtime(), root), ["a", "b", "c"]);
}

#[test]
fn keyed_remove_retires_the_old_subtree_child_first() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(keyed_text(&["a", "b", "c"])).unwrap();
    let root = pump.root().unwrap();

    pump.update(keyed_text(&["a", "c"])).unwrap();
    assert_eq!(recorded_text(pump.runtime(), root), ["a", "c"]);
}

#[test]
fn randomized_keyed_updates_match_recording_tree() {
    let mut seed = 0x5eed_u64;
    let mut next_random = || {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        (seed >> 32) as usize
    };
    let mut current = (0_u64..8).collect::<Vec<_>>();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(keyed_numbers(&current)).unwrap();
    let root = pump.root().unwrap();

    for _ in 0..1_000 {
        let mut pool = (0_u64..16).collect::<Vec<_>>();
        for index in (1..pool.len()).rev() {
            let other = next_random() % (index + 1);
            pool.swap(index, other);
        }
        current = pool[..next_random() % 16].to_vec();

        pump.update(keyed_numbers(&current)).unwrap();

        assert_eq!(
            recorded_text(pump.runtime(), root),
            current.iter().map(u64::to_string).collect::<Vec<_>>()
        );
    }
}

#[test]
fn same_key_child_can_change_kind_without_replacing_panel() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        StackPanel::new()
            .child("item", TextBlock::new().text("text"))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let old = pump.tree.children(root).unwrap()[0];

    pump.update(StackPanel::new().child("item", Button::new()).into())
        .unwrap();

    let child = pump.tree.children(root).unwrap()[0];
    assert_eq!(pump.root(), Some(root));
    assert_ne!(child, old);
    assert_eq!(
        pump.tree.kind(child),
        Ok(NodeKind::Native(MountedKind::Button))
    );
    assert_eq!(pump.runtime().node(root).unwrap().children(), &[child]);
}
