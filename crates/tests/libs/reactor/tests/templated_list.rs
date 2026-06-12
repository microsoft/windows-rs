use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::Reconciler;
use windows_reactor::{ControlKind, Op, RecordingBackend};
use windows_reactor::{Element, TextBlock};
use windows_reactor::{TemplatedKind, grid_view, list_view};

fn noop_request_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount_and_drain(el: Element) -> (Reconciler<RecordingBackend>, windows_reactor::ControlId) {
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &el, None, noop_request_rerender())
        .expect("mount produced an id");
    r.drain_realizations();
    (r, id)
}

#[test]
fn mounting_list_view_creates_list_control_and_no_rows() {
    let items: Vec<i32> = (0..5).collect();
    let el = list_view(items, |n, _| TextBlock::new(n.to_string())).build();
    let (r, list_id) = mount_and_drain(el);

    let ops = &r.backend.ops;
    assert!(
        ops.iter().any(|op| matches!(
            op,
            Op::Create {
                kind: ControlKind::ListView,
                ..
            }
        )),
        "expected a ListView create in {ops:?}"
    );
    assert!(
        ops.iter()
            .any(|op| matches!(op, Op::SetTemplatedItemCount { count: 5, .. })),
        "expected item-count=5 set in {ops:?}"
    );

    assert!(
        r.backend.row_contents_of(list_id).is_empty(),
        "no rows should be realized before scroll"
    );
}

#[test]
fn grid_view_builder_maps_to_grid_kind() {
    let el = grid_view(vec![1i32, 2], |_, _| Element::Empty).build();
    if let Element::TemplatedList(tl) = &el {
        assert_eq!(tl.kind, TemplatedKind::GridView);
    }
    let (r, _) = mount_and_drain(el);
    assert!(
        r.backend.ops.iter().any(|op| matches!(
            op,
            Op::Create {
                kind: ControlKind::GridView,
                ..
            }
        )),
        "expected a GridView create"
    );
}

#[test]
fn simulate_prepare_realizes_exactly_one_row() {
    let items: Vec<i32> = (0..5).collect();
    let el = list_view(items, |n, _| TextBlock::new(n.to_string())).build();
    let (mut r, list_id) = mount_and_drain(el);

    r.backend.clear_ops();
    r.backend.simulate_prepare_row(list_id, 2);
    r.drain_realizations();

    let text_creates = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::Create {
                    kind: ControlKind::TextBlock,
                    ..
                }
            )
        })
        .count();
    assert_eq!(
        text_creates, 1,
        "realize should create one TextBlock control"
    );

    let row_map = r.backend.row_contents_of(list_id);
    assert_eq!(row_map.len(), 1);
    assert!(row_map.contains_key(&2));

    assert!(
        r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::MountRowContent { row_idx: 2, list_id: lid, .. } if *lid == list_id)),
        "expected MountRowContent for row 2"
    );
}

#[test]
fn simulate_clear_recycles_row_content() {
    let items: Vec<i32> = (0..5).collect();
    let el = list_view(items, |n, _| TextBlock::new(n.to_string())).build();
    let (mut r, list_id) = mount_and_drain(el);

    r.backend.simulate_prepare_row(list_id, 0);
    r.drain_realizations();
    assert_eq!(r.backend.row_contents_of(list_id).len(), 1);

    r.backend.clear_ops();
    r.backend.simulate_clear_row(list_id, 0);
    r.drain_realizations();

    assert!(r.backend.row_contents_of(list_id).is_empty());
    assert!(r.backend.ops.iter().any(
        |op| matches!(op, Op::ClearRowContent { row_idx: 0, list_id: lid } if *lid == list_id)
    ));
    assert!(
        r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { .. }))
    );
}

#[test]
fn recycle_then_realize_does_not_recreate_list_container() {
    let items: Vec<i32> = (0..10).collect();
    let el = list_view(items, |n, _| TextBlock::new(n.to_string())).build();
    let (mut r, list_id) = mount_and_drain(el);

    r.backend.simulate_prepare_row(list_id, 0);
    r.drain_realizations();

    r.backend.simulate_clear_row(list_id, 0);
    r.drain_realizations();

    r.backend.clear_ops();

    r.backend.simulate_prepare_row(list_id, 5);
    r.drain_realizations();

    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { id } if *id == list_id))
    );
    assert_eq!(r.backend.row_contents_of(list_id).len(), 1);
}

#[test]
fn multiple_prepare_events_realize_all_rows() {
    let items: Vec<i32> = (0..4).collect();
    let el = list_view(items, |n, _| TextBlock::new(n.to_string())).build();
    let (mut r, list_id) = mount_and_drain(el);

    for i in 0..4 {
        r.backend.simulate_prepare_row(list_id, i);
    }
    r.drain_realizations();
    assert_eq!(r.backend.row_contents_of(list_id).len(), 4);
}

#[test]
fn selection_changed_invokes_user_callback() {
    let sink = Rc::new(Cell::new(-1_i32));
    let sink_c = Rc::clone(&sink);

    let el = list_view(vec![10i32, 20, 30], |n, _| TextBlock::new(n.to_string()))
        .on_selection_changed(move |i| sink_c.set(i))
        .build();
    let (r, list_id) = mount_and_drain(el);

    r.backend.fire_templated_selection_changed(list_id, 2);
    assert_eq!(sink.get(), 2);
}

#[test]
fn selected_index_propagates_on_mount() {
    let el = list_view(vec![1i32, 2, 3], |_, _| Element::Empty)
        .selected_index(1)
        .build();
    let (r, list_id) = mount_and_drain(el);
    assert!(
        r.backend.ops.iter().any(
            |op| matches!(op, Op::SetTemplatedSelectedIndex { id, index: 1 } if *id == list_id)
        )
    );
}

#[test]
fn updating_item_count_resizes_and_tells_backend() {
    let old_items: Vec<i32> = (0..5).collect();
    let new_items: Vec<i32> = (0..8).collect();

    let old_el = list_view(old_items, |n, _| TextBlock::new(n.to_string())).build();
    let new_el = list_view(new_items, |n, _| TextBlock::new(n.to_string())).build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r
        .reconcile(None, &old_el, None, noop_request_rerender())
        .unwrap();
    r.drain_realizations();

    r.backend.clear_ops();
    let _ = r.reconcile(
        Some(&old_el),
        &new_el,
        Some(list_id),
        noop_request_rerender(),
    );
    r.drain_realizations();

    assert!(r.backend.ops.iter().any(
        |op| matches!(op, Op::SetTemplatedItemCount { count: 8, list_id: lid } if *lid == list_id)
    ));
    assert!(!r.backend.ops.iter().any(|op| matches!(
        op,
        Op::Create {
            kind: ControlKind::ListView,
            ..
        }
    )));
}

#[test]
fn updating_with_same_items_rc_refreshes_nothing_for_unrealized() {
    let items: Vec<i32> = (0..5).collect();
    let old_el = list_view(items, |n, _| TextBlock::new(n.to_string())).build();
    let new_el = old_el.clone();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r
        .reconcile(None, &old_el, None, noop_request_rerender())
        .unwrap();
    r.drain_realizations();

    r.backend.clear_ops();
    let _ = r.reconcile(
        Some(&old_el),
        &new_el,
        Some(list_id),
        noop_request_rerender(),
    );
    r.drain_realizations();

    assert_eq!(
        r.backend.ops.len(),
        0,
        "same-items update should be a no-op, got {:?}",
        r.backend.ops
    );
}

#[test]
fn shrinking_list_unmounts_rows_beyond_new_tail() {
    let old_items: Vec<i32> = (0..5).collect();
    let new_items: Vec<i32> = (0..2).collect();

    let old_el = list_view(old_items, |n, _| TextBlock::new(n.to_string())).build();
    let new_el = list_view(new_items, |n, _| TextBlock::new(n.to_string())).build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r
        .reconcile(None, &old_el, None, noop_request_rerender())
        .unwrap();
    r.drain_realizations();

    for i in 0..5 {
        r.backend.simulate_prepare_row(list_id, i);
    }
    r.drain_realizations();
    assert_eq!(r.backend.row_contents_of(list_id).len(), 5);

    r.backend.clear_ops();
    let _ = r.reconcile(
        Some(&old_el),
        &new_el,
        Some(list_id),
        noop_request_rerender(),
    );
    r.drain_realizations();

    let unmounts = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::Destroy { .. }))
        .count();
    assert_eq!(
        unmounts, 3,
        "expected 3 row unmounts (destroy) for shrunk rows, got {:?}",
        r.backend.ops
    );

    assert_eq!(r.backend.row_contents_of(list_id).len(), 2);
}

#[test]
fn updating_to_new_items_refreshes_realized_row_content() {
    let mk_el =
        |items: Vec<i32>| list_view(items, |n, _| TextBlock::new(format!("row-{n}"))).build();
    let old_el = mk_el(vec![10, 20, 30]);
    let new_el = mk_el(vec![10, 99, 30]);

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r
        .reconcile(None, &old_el, None, noop_request_rerender())
        .unwrap();
    r.drain_realizations();
    r.backend.simulate_prepare_row(list_id, 1);
    r.drain_realizations();

    r.backend.clear_ops();
    let _ = r.reconcile(
        Some(&old_el),
        &new_el,
        Some(list_id),
        noop_request_rerender(),
    );
    r.drain_realizations();

    let text_sets = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetProp {
                prop: windows_reactor::Prop::Text,
                value: windows_reactor::PropValue::Str(s),
                ..
            } => Some(s.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(
        text_sets.contains(&"row-99".to_string()),
        "expected row-99 text update, got {text_sets:?}"
    );
}
