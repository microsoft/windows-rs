use std::rc::Rc;

use rustc_hash::FxHashMap;
use windows_reactor::{
    Button, Callback, ControlId, ControlKind, Element, Event, Op, Prop, PropValue, Reconciler,
    RecordingBackend, StackPanel, TextBlock, compute_lis,
};

#[test]
fn lis_empty() {
    let set = compute_lis(&[]);
    assert!(set.is_empty());
}

#[test]
fn lis_single() {
    let set = compute_lis(&[5]);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&0));
}

#[test]
fn lis_sorted() {
    let set = compute_lis(&[1, 2, 3, 4, 5]);
    assert_eq!(set.len(), 5);
}

#[test]
fn lis_reversed() {
    let set = compute_lis(&[5, 4, 3, 2, 1]);
    assert_eq!(set.len(), 1);
}

#[test]
fn lis_with_unmapped() {
    let set = compute_lis(&[-1, 2, -1, 4, -1]);
    assert_eq!(set.len(), 2);
}

#[test]
fn lis_all_unmapped() {
    let set = compute_lis(&[-1, -1, -1]);
    assert!(set.is_empty());
}

#[test]
fn lis_mixed() {
    let set = compute_lis(&[3, 1, 2, 4]);
    assert_eq!(set.len(), 3);
}

#[test]
fn lis_returned_indices_are_strictly_increasing_values() {
    let arr = [5, 1, 4, 2, 3];
    let set = compute_lis(&arr);
    let mut ordered: Vec<usize> = set.into_iter().collect();
    ordered.sort();
    for window in ordered.windows(2) {
        assert!(arr[window[1]] > arr[window[0]]);
    }
}

#[test]
fn lis_realistic_reconcile_shift_one() {
    let set = compute_lis(&[2, 0, 1, 3]);
    assert_eq!(set.len(), 3);
}

/// Regression: in a keyed reorder of mixed-kind children, the LIS
/// branch of `reconcile_keyed_middle` previously called
/// `update_child_tracked(parent, target_panel_idx, ..)` even though an
/// earlier non-LIS move had already shifted the LIS element to a
/// different actual slot. The reconciler then updated the wrong
/// control: e.g. it pushed a `Click` binding (from a Button element)
/// onto a TextBlock control, which crashed the WinUI backend with
/// `STATUS_STACK_BUFFER_OVERRUN` (see `winui/backend.rs::attach_event`).
///
/// Scenario: a vertical StackPanel with five keyed children
/// `[empty(TextBlock), X(Button), A(Button), B(Button), C(Button)]`
/// reordered to `[X, A, empty, B, C]`. Empty moves 0 -> 2, X moves
/// 1 -> 0, A moves 2 -> 1, B/C stay (LIS includes new indices 0, 1,
/// 3, 4 with old positions 1, 2, 3, 4). With the buggy left-to-right
/// pass, the LIS update for new[0]=X runs at `target_panel_idx=0`
/// while slot 0 still holds the empty TextBlock, so Click bindings
/// get attached to the TextBlock — that's the crash.
#[test]
fn keyed_reorder_with_mixed_kinds_keeps_controls_aligned() {
    fn tile(key: &'static str) -> Element {
        let mut button = Button::new(key);
        button.key = Some(key.to_string());
        button.on_click = Some(Callback::new(|()| {}));
        Element::Button(button)
    }

    fn empty() -> Element {
        let mut text = TextBlock::new("");
        text.key = Some("empty".to_string());
        Element::TextBlock(text)
    }

    fn stack(children: Vec<Element>) -> Element {
        let mut stack = StackPanel::vertical();
        stack.children = children;
        Element::StackPanel(stack)
    }

    let old = stack(vec![
        empty(),
        tile("tile-X"),
        tile("tile-A"),
        tile("tile-B"),
        tile("tile-C"),
    ]);
    let new = stack(vec![
        tile("tile-X"),
        tile("tile-A"),
        empty(),
        tile("tile-B"),
        tile("tile-C"),
    ]);

    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let noop: Rc<dyn Fn()> = Rc::new(|| {});
    let root = reconciler
        .reconcile(None, &old, None, Rc::clone(&noop))
        .unwrap();

    let mut kind_of: FxHashMap<ControlId, ControlKind> = FxHashMap::default();
    for op in &reconciler.backend.ops {
        if let Op::Create { id, kind } = op {
            kind_of.insert(*id, *kind);
        }
    }

    let mount_button_ids: Vec<ControlId> = reconciler
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Create {
                id,
                kind: ControlKind::Button,
            } => Some(*id),
            _ => None,
        })
        .collect();
    let mount_button_labels = ["tile-X", "tile-A", "tile-B", "tile-C"];
    assert_eq!(mount_button_ids.len(), mount_button_labels.len());

    reconciler.backend.clear_ops();
    reconciler
        .reconcile(Some(&old), &new, Some(root), Rc::clone(&noop))
        .unwrap();

    for op in &reconciler.backend.ops {
        if let Op::Create { id, kind } = op {
            kind_of.insert(*id, *kind);
        }
    }

    for op in &reconciler.backend.ops {
        if let Op::AttachEvent {
            id,
            event: Event::Click,
            ..
        } = op
        {
            let kind = kind_of.get(id).copied();
            assert_eq!(
                kind,
                Some(ControlKind::Button),
                "Click attach landed on non-Button control {id} (kind={kind:?}). \
                 The LIS branch of reconcile_keyed_middle is desynced from \
                 key_to_panel — see this test's doc comment."
            );
        }
    }

    let mut latest_label: FxHashMap<ControlId, String> = FxHashMap::default();
    for op in &reconciler.backend.ops {
        if let Op::SetProp {
            id,
            prop: Prop::Content,
            value: PropValue::Str(label),
        } = op
        {
            latest_label.insert(*id, label.clone());
        }
    }

    for (id, want) in mount_button_ids.iter().zip(mount_button_labels.iter()) {
        if let Some(got) = latest_label.get(id) {
            assert_eq!(
                got, want,
                "Button control {id} should still carry label {want} after the \
                 reorder, but the latest ButtonContent set was {got:?}. The LIS \
                 branch of reconcile_keyed_middle is pushing the wrong element's \
                 bindings onto displaced controls."
            );
        }
    }
}
