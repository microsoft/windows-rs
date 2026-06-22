use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::ControlKind;
use windows_reactor::Reconciler;
use windows_reactor::list_view;
use windows_reactor::{Element, TextBlock};

fn noop() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn make_list(items: Vec<u32>) -> Element {
    list_view(items, |n, _| TextBlock::new(format!("row-{n}")))
        .with_key_selector(|n| format!("k{n}"))
        .build()
}

fn mount_list(
    items: Vec<u32>,
) -> (
    Reconciler<RecordingBackend>,
    Element,
    windows_reactor::ControlId,
) {
    let el = make_list(items);
    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r
        .reconcile(None, &el, None, noop())
        .expect("mount produced an id");
    r.drain_realizations();
    (r, el, list_id)
}

#[test]
fn inserting_one_item_in_10k_emits_one_item_count_op() {
    let items: Vec<u32> = (0..10_000).collect();
    let (mut r, old_el, list_id) = mount_list(items.clone());

    let mut new_items = items;
    new_items.insert(5000, 99999);
    let new_el = make_list(new_items);

    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old_el), &new_el, Some(list_id), noop());
    r.drain_realizations();

    let item_count_ops: Vec<&Op> = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::SetTemplatedItemCount { .. }))
        .collect();
    assert_eq!(
        item_count_ops.len(),
        1,
        "expected exactly one SetTemplatedItemCount, got {} ops: {:?}",
        item_count_ops.len(),
        r.backend.ops
    );
    match item_count_ops[0] {
        Op::SetTemplatedItemCount { count, .. } => assert_eq!(*count, 10_001),
        _ => unreachable!(),
    }

    assert!(
        !r.backend.ops.iter().any(|op| matches!(
            op,
            Op::Create {
                kind: ControlKind::TextBlock,
                ..
            }
        )),
        "no row-level TextBlock creates expected"
    );
    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::MountRowContent { .. })),
        "no MountRowContent expected when no rows are realized"
    );
}

#[test]
fn removing_one_item_emits_one_item_count_op_when_unrealized() {
    let items: Vec<u32> = (0..100).collect();
    let (mut r, old_el, list_id) = mount_list(items.clone());

    let mut new_items = items;
    new_items.remove(50);
    let new_el = make_list(new_items);

    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old_el), &new_el, Some(list_id), noop());
    r.drain_realizations();

    let item_count_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::SetTemplatedItemCount { .. }))
        .collect();
    assert_eq!(item_count_ops.len(), 1);
    match item_count_ops[0] {
        Op::SetTemplatedItemCount { count, .. } => assert_eq!(*count, 99),
        _ => unreachable!(),
    }

    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::Destroy { .. })),
        "no row destroys expected when no rows are realized"
    );
}

#[test]
fn identical_list_update_emits_zero_ops() {
    let items: Vec<u32> = (0..100).collect();
    let (mut r, old_el, list_id) = mount_list(items);
    let new_el = old_el.clone();

    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old_el), &new_el, Some(list_id), noop());
    r.drain_realizations();

    assert_eq!(
        r.backend.ops.len(),
        0,
        "expected zero ops, got {:?}",
        r.backend.ops
    );
}

#[test]
fn removing_realized_row_frees_its_content() {
    let items: Vec<u32> = (0..5).collect();
    let (mut r, old_el, list_id) = mount_list(items.clone());

    for i in 0..5 {
        r.backend.simulate_prepare_row(list_id, i);
    }
    r.drain_realizations();
    assert_eq!(r.backend.row_contents_of(list_id).len(), 5);

    let mut new_items = items;
    new_items.pop();
    let new_el = make_list(new_items);

    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old_el), &new_el, Some(list_id), noop());
    r.drain_realizations();

    assert_eq!(r.backend.row_contents_of(list_id).len(), 4);
    let unmount_ops = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::Destroy { .. }))
        .count();
    assert_eq!(
        unmount_ops, 1,
        "expected exactly 1 row unmount, got {:?}",
        r.backend.ops
    );
}

#[test]
fn updating_items_with_same_key_but_new_value_reconciles_realized_row() {
    let items: Vec<u32> = (0..3).collect();
    let (mut r, old_el, list_id) = mount_list(items);
    r.backend.simulate_prepare_row(list_id, 1);
    r.drain_realizations();

    let new_el = list_view(vec![0u32, 1, 2], |n, i| {
        if i == 1 {
            TextBlock::new(format!("NEW:row-{n}"))
        } else {
            TextBlock::new(format!("row-{n}"))
        }
    })
    .with_key_selector(|n| format!("k{n}"))
    .build();

    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old_el), &new_el, Some(list_id), noop());
    r.drain_realizations();

    let text_updates: Vec<String> = r
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
        .collect();
    assert!(
        text_updates.iter().any(|s| s == "NEW:row-1"),
        "expected 'NEW:row-1' text update; saw {text_updates:?}"
    );

    assert!(
        !r.backend.ops.iter().any(|op| matches!(
            op,
            Op::Create {
                kind: ControlKind::TextBlock,
                ..
            }
        )),
        "row 1 should have been updated, not recreated"
    );
}

#[test]
fn ten_thousand_item_middle_insert_emits_single_backend_op() {
    let items: Vec<u32> = (0..10_000).collect();
    let (mut r, old_el, list_id) = mount_list(items.clone());

    let mut new_items = items;
    new_items.insert(5_000, 42);
    let new_el = make_list(new_items);

    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old_el), &new_el, Some(list_id), noop());
    r.drain_realizations();

    assert_eq!(
        r.backend.ops.len(),
        1,
        "expected a single backend op for a single-item insert, got {} ops: \
         {:?}",
        r.backend.ops.len(),
        r.backend.ops
    );
    assert!(matches!(
        r.backend.ops[0],
        Op::SetTemplatedItemCount {
            count: 10_001,
            list_id: lid
        } if lid == list_id
    ));
}
