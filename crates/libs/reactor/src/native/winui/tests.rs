use super::{
    AsyncIngressQueue, AsyncIngressSender, Command, DroppedData, NodeId, PendingAsync,
    RuntimeError, SlotId, WinUiRuntime, WindowId, WindowToken, is_internal_detach,
    merge_retained_identities, native_number_box_value, native_rating_value,
    native_selection_index, number_box_value, physical_retained_index, rating_value,
    retained_subsequence, selection_index,
};
use std::cell::Cell;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[test]
fn releasing_encoded_image_source_cancels_pending_decode_and_ownership() {
    let node = NodeId::from_parts(1, 0);
    let ticket = 7;
    let canceled = Rc::new(Cell::new(false));
    let canceled_capture = Rc::clone(&canceled);
    let mut runtime = WinUiRuntime::default();
    runtime.encoded_image_nodes.borrow_mut().insert(node);
    runtime.image_decode_tickets.insert(node, ticket);
    runtime.async_state.borrow_mut().pending.insert(
        ticket,
        PendingAsync {
            node,
            cancel: Box::new(move || canceled_capture.set(true)),
            finalize: Box::new(|_, _| {}),
        },
    );

    runtime.release_encoded_image_source(node);

    assert!(canceled.get());
    assert!(!runtime.encoded_image_nodes.borrow().contains(&node));
    assert!(!runtime.image_decode_tickets.contains_key(&node));
    assert!(!runtime.async_state.borrow().pending.contains_key(&ticket));
}

#[test]
fn skips_only_internal_detaches_for_destroyed_subtrees() {
    let parent = NodeId::from_parts(1, 0);
    let child = NodeId::from_parts(2, 0);
    let survivor = NodeId::from_parts(3, 0);
    let destroyed = HashSet::from([parent, child]);

    assert!(is_internal_detach(
        &Command::RemoveChild {
            parent,
            slot: Some(SlotId::PivotItems),
            child,
        },
        &destroyed,
    ));
    assert!(!is_internal_detach(
        &Command::RemoveChild {
            parent,
            slot: Some(SlotId::PivotItems),
            child: survivor,
        },
        &destroyed,
    ));
    assert!(is_internal_detach(
        &Command::SetSlot {
            parent,
            slot: SlotId::ExpanderContent,
            child: None,
        },
        &destroyed,
    ));
    assert!(!is_internal_detach(
        &Command::SetSlot {
            parent,
            slot: SlotId::ExpanderContent,
            child: Some(survivor),
        },
        &destroyed,
    ));
}

#[test]
fn retained_subsequence_preserves_longest_native_order() {
    assert_eq!(
        retained_subsequence(&[1, 2, 3, 4, 5], &[5, 2, 3, 1, 4]),
        HashSet::from([2, 3, 4])
    );
    assert_eq!(
        retained_subsequence(&[1, 2, 3, 4], &[4, 3, 2, 1]),
        HashSet::from([1])
    );
    assert_eq!(
        retained_subsequence(&[1, 2, 3], &[4, 2, 5]),
        HashSet::from([2])
    );
}

#[test]
fn retained_children_stay_anchored_during_semantic_changes() {
    let retained = HashSet::from([2, 4]);
    assert_eq!(
        merge_retained_identities(&[1, 2, 3, 4, 5], &[3, 6, 5], &retained),
        [2, 3, 6, 4, 5]
    );
    assert_eq!(physical_retained_index(&[2, 3, 4, 5], &retained, 0), Ok(1));
    assert_eq!(physical_retained_index(&[2, 3, 4, 5], &retained, 1), Ok(3));
    assert_eq!(physical_retained_index(&[2, 4], &retained, 0), Ok(2));
}

#[test]
fn async_ingress_rejects_stale_and_failed_wakes() {
    let identity = WindowToken::new(WindowId::allocate());
    let ingress = Arc::new(Mutex::new(AsyncIngressQueue {
        identity: Some(identity),
        ..Default::default()
    }));
    let wakes = Arc::new(AtomicUsize::new(0));
    let wake_count = Arc::clone(&wakes);
    let sender = AsyncIngressSender {
        identity,
        ticket: 7,
        ingress: Arc::clone(&ingress),
        wake: Arc::new(move || {
            wake_count.fetch_add(1, Ordering::Relaxed);
            true
        }),
        marker: PhantomData,
    };

    assert!(sender.complete(Ok::<_, RuntimeError>(DroppedData::Text("value".into()))));
    let queued = ingress.lock().unwrap().completions.pop_front().unwrap();
    assert_eq!(queued.identity, identity);
    assert_eq!(queued.ticket, 7);
    assert_eq!(
        *queued
            .payload
            .downcast::<Result<DroppedData, RuntimeError>>()
            .unwrap(),
        Ok(DroppedData::Text("value".into()))
    );
    assert_eq!(wakes.load(Ordering::Relaxed), 1);

    ingress.lock().unwrap().identity = identity.next();
    let stale = AsyncIngressSender {
        identity,
        ticket: 8,
        ingress: Arc::clone(&ingress),
        wake: Arc::new(|| true),
        marker: PhantomData,
    };
    assert!(!stale.complete(Ok::<_, RuntimeError>(DroppedData::Unsupported)));
    assert!(ingress.lock().unwrap().completions.is_empty());

    ingress.lock().unwrap().identity = Some(identity);
    let rejected = AsyncIngressSender {
        identity,
        ticket: 9,
        ingress: Arc::clone(&ingress),
        wake: Arc::new(|| false),
        marker: PhantomData,
    };
    assert!(!rejected.complete(Err::<DroppedData, _>(RuntimeError::Injected)));
    assert!(ingress.lock().unwrap().completions.is_empty());
}

#[test]
fn semantic_values_round_trip_native_sentinels() {
    assert_eq!(native_selection_index(None), Ok(-1));
    assert_eq!(native_selection_index(Some(2)), Ok(2));
    assert_eq!(selection_index(-1), Ok(None));
    assert_eq!(selection_index(2), Ok(Some(2)));
    assert_eq!(selection_index(-2), Err(RuntimeError::IndexOutOfBounds));
    assert_eq!(
        native_selection_index(Some(i32::MAX as usize + 1)),
        Err(RuntimeError::IndexOutOfBounds)
    );

    assert!(native_number_box_value(None).is_nan());
    assert_eq!(number_box_value(f64::NAN), None);
    assert_eq!(number_box_value(2.5), Some(2.5));

    assert_eq!(native_rating_value(None), -1.0);
    assert_eq!(rating_value(-1.0), None);
    assert_eq!(rating_value(2.5), Some(2.5));
}
