use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Reconciler;
use windows_reactor::TextBlock;
use windows_reactor::list_view;

fn noop() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

#[test]
fn deferred_unmount_flag_batches_into_queue() {
    let items: Vec<u32> = (0..10).collect();
    let el = list_view(items, |n, _| TextBlock::new(format!("row-{n}"))).build();

    let mut r = Reconciler::new(RecordingBackend::new());
    r.defer_templated_unmounts = true;
    let list_id = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();
    for i in 0..5 {
        r.backend.simulate_prepare_row(list_id, i);
    }
    r.drain_realizations();

    r.backend.clear_ops();

    for i in 0..5 {
        r.backend.simulate_clear_row(list_id, i);
    }
    r.drain_realizations();

    let sync_unmounts = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::Destroy { .. }))
        .count();
    assert_eq!(
        sync_unmounts, 0,
        "deferred flag should prevent sync unmounts"
    );

    r.flush_deferred_unmounts();
    let after_flush_unmounts = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::Destroy { .. }))
        .count();
    assert_eq!(
        after_flush_unmounts, 5,
        "flush should process all 5 deferred unmounts"
    );
}

#[test]
fn flushing_empty_deferred_queue_is_a_noop() {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.defer_templated_unmounts = true;
    r.flush_deferred_unmounts();
}
