use super::*;

#[test]
fn virtual_items_realize_only_requested_rows() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    let source = VirtualSource::new(1, 10_000, Key::from, |index| -> View {
        TextBlock::new().text(index.to_string()).into()
    });
    runtime
        .update(ItemsRepeater::new().virtual_source(source))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    assert_eq!(runtime.adapter().object_count(), 1);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(7),
            index: 9_999,
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Realize {
        lease, index, view, ..
    })) = runtime.next_native_work().unwrap()
    else {
        panic!("expected realization");
    };
    runtime.realize_virtual(&lease, index, *view).unwrap();

    assert_eq!(runtime.adapter().object_count(), 2);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(
        runtime
            .adapter()
            .children(collection, RelationId::Items)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn virtual_items_recycle_and_reuse_containers_without_stale_ownership() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(
            ItemsRepeater::new()
                .item(1_u64, TextBlock::new().text("one"))
                .item(2_u64, TextBlock::new().text("two")),
        )
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    let realize = |runtime: &mut Runtime<RecordingAdapter>, container, index| -> RealizationLease {
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index,
                source_revision: 0,
            });
        let Some(NativeWork::Virtual(VirtualWork::Realize {
            lease, index, view, ..
        })) = runtime.next_native_work().unwrap()
        else {
            panic!("expected realization");
        };
        runtime.realize_virtual(&lease, index, *view).unwrap();
        lease
    };
    let first = realize(&mut runtime, RealizedContainer(1), 0);
    let second = realize(&mut runtime, RealizedContainer(1), 1);
    assert_ne!(first.key, second.key);
    assert_eq!(runtime.adapter().object_count(), 2);
    assert_eq!(runtime.adapter().realized_count(collection), 1);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Recycle { lease })) =
        runtime.next_native_work().unwrap()
    else {
        panic!("expected recycle");
    };
    runtime.recycle_virtual(&lease).unwrap();
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
}

#[test]
fn virtual_items_preserve_keyed_child_across_reorder() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("a"))
                .item("b", TextBlock::new().text("b")),
        )
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    for index in 0..2 {
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(index as u64),
                index,
                source_revision: 0,
            });
        let Some(NativeWork::Virtual(VirtualWork::Realize {
            lease, index, view, ..
        })) = runtime.next_native_work().unwrap()
        else {
            panic!("expected realization");
        };
        runtime.realize_virtual(&lease, index, *view).unwrap();
    }
    let before = runtime
        .adapter()
        .children(collection, RelationId::Items)
        .unwrap()
        .to_vec();

    runtime
        .update(
            ItemsRepeater::new()
                .item("b", TextBlock::new().text("b"))
                .item("a", TextBlock::new().text("updated")),
        )
        .unwrap();
    for _ in 0..2 {
        let Some(NativeWork::Virtual(VirtualWork::Realize {
            lease, index, view, ..
        })) = runtime.next_native_work().unwrap()
        else {
            panic!("expected refresh");
        };
        runtime.realize_virtual(&lease, index, *view).unwrap();
    }
    assert_eq!(
        runtime
            .adapter()
            .children(collection, RelationId::Items)
            .unwrap(),
        &[before[1], before[0]]
    );
}

#[test]
fn virtual_items_reject_duplicate_keys_without_mutation() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ItemsRepeater::new()).unwrap();
    let before = runtime.graph().clone();

    assert_eq!(
        runtime.update(
            ItemsRepeater::new()
                .item("duplicate", TextBlock::new().text("first"))
                .item("duplicate", TextBlock::new().text("second"))
        ),
        Err(UpdateError::Graph(GraphError::DuplicateKey(Key::from(
            "duplicate"
        ))))
    );
    assert_eq!(runtime.graph(), &before);
    assert_eq!(runtime.adapter().object_count(), 1);
}

#[test]
fn virtual_items_ignore_stale_source_requests() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(ItemsRepeater::new().item("first", TextBlock::new().text("first")))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .update(ItemsRepeater::new().item("second", TextBlock::new().text("second")))
        .unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(matches!(
        runtime.next_native_work().unwrap(),
        Some(NativeWork::Maintenance)
    ));
    assert!(runtime.next_native_work().unwrap().is_none());
    assert_eq!(runtime.adapter().object_count(), 1);
}

#[test]
fn virtual_items_remove_active_rows_when_source_becomes_empty() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(ItemsRepeater::new().item("first", TextBlock::new().text("first")))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Realize {
        lease, index, view, ..
    })) = runtime.next_native_work().unwrap()
    else {
        panic!("expected realization");
    };
    runtime.realize_virtual(&lease, index, *view).unwrap();

    runtime.update(ItemsRepeater::new()).unwrap();
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert!(
        runtime
            .adapter()
            .children(collection, RelationId::Items)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn virtual_realization_batches_validate_before_consumption() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(ItemsRepeater::new().item("only", TextBlock::new().text("only")))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(2),
            index: 1,
            source_revision: 0,
        });

    assert!(matches!(
        runtime.next_native_work(),
        Err(UpdateError::InvalidNativeEvent(GraphError::InvalidRealization(
            object,
            1
        ))) if object == collection
    ));
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(
        runtime.update(ItemsRepeater::new()),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn duplicate_virtual_recycle_is_idempotent() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(ItemsRepeater::new().item("only", TextBlock::new().text("only")))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events().unwrap();
    for _ in 0..2 {
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 0,
            });
    }
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
}

#[test]
fn realized_virtual_children_recycle_before_repeater_destruction() {
    let mut adapter = RecordingAdapter::new();
    adapter.record_batches(true);
    let mut runtime = Runtime::new(adapter);
    runtime
        .update(Grid::new().keyed_children([keyed(
            "repeater",
            ItemsRepeater::new().item("row", TextBlock::new().text("row")),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let repeater = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection: repeater,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events().unwrap();

    runtime.update(Grid::new()).unwrap();
    let batch = runtime.adapter().batches().last().unwrap();
    let recycle = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Recycle { parent, .. } if *parent == repeater
            )
        })
        .unwrap();
    let destroy = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Destroy { object } if *object == repeater
            )
        })
        .unwrap();
    assert!(recycle < destroy);
    assert!(!batch.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::Remove {
                parent,
                relation: RelationId::Items,
                ..
            } if *parent == repeater
        )
    }));
}

#[test]
fn realized_virtual_children_recycle_before_repeater_replacement() {
    let mut adapter = RecordingAdapter::new();
    adapter.record_batches(true);
    let mut runtime = Runtime::new(adapter);
    runtime
        .update(Grid::new().keyed_children([keyed(
            "slot",
            ItemsRepeater::new().item("row", TextBlock::new().text("row")),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let repeater = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection: repeater,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events().unwrap();

    runtime
        .update(Grid::new().keyed_children([keyed("slot", Border::new())]))
        .unwrap();
    let batch = runtime.adapter().batches().last().unwrap();
    let recycle = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Recycle { parent, .. } if *parent == repeater
            )
        })
        .unwrap();
    let replace = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Destroy { object } if *object == repeater
            )
        })
        .unwrap();
    assert!(recycle < replace);
    assert!(!batch.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::Remove {
                parent,
                relation: RelationId::Items,
                ..
            } if *parent == repeater
        )
    }));
}

#[test]
fn update_peeks_pending_virtual_work_without_consuming_it() {
    let view = || ItemsRepeater::new().item("row", TextBlock::new().text("row"));
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(view()).unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(matches!(
        runtime.update(view()),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    assert!(matches!(
        runtime.update(view()),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn public_event_api_preserves_virtual_work_until_ordered_dispatch() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(
            ItemsRepeater::new()
                .item("old", TextBlock::new().text("old"))
                .item("new", TextBlock::new().text("new")),
        )
        .unwrap();
    let collection = runtime.graph().root().unwrap();

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    assert!(matches!(
        runtime.next_native_event(),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    assert!(matches!(
        runtime.next_native_event(),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(2),
            index: 1,
            source_revision: 0,
        });
    assert!(matches!(
        runtime.next_native_event(),
        Err(UpdateError::PendingNativeEvent)
    ));
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(2),
            source_revision: 0,
        });
    assert!(matches!(
        runtime.next_native_event(),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn recycle_before_dispatch_cancels_realization_and_allows_token_reuse() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(
            ItemsRepeater::new()
                .item("old", TextBlock::new().text("old"))
                .item("new", TextBlock::new().text("new")),
        )
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    assert!(matches!(
        runtime.update(
            ItemsRepeater::new()
                .item("old", TextBlock::new().text("old"))
                .item("new", TextBlock::new().text("new"))
        ),
        Err(UpdateError::PendingNativeEvent)
    ));
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(2),
            index: 1,
            source_revision: 0,
        });

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.graph().object_count(), 2);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert!(runtime.update(ItemsRepeater::new()).is_ok());
}
