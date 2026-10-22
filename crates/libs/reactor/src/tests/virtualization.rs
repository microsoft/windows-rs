use super::*;

#[test]
fn virtual_row_content_updates_do_not_reset_the_native_source() {
    for lazy in [false, true] {
        let view = |text: &'static str| {
            if lazy {
                ItemsRepeater::new().virtual_source(VirtualSource::new(
                    0,
                    1,
                    |_| "row",
                    move |_| TextBlock::new().text(text),
                ))
            } else {
                ItemsRepeater::new().item("row", TextBlock::new().text(text))
            }
        };
        let mut runtime = Runtime::new(RecordingAdapter::new());
        runtime.update(view("Before"));
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
        })) = runtime.next_native_work()
        else {
            panic!("expected initial realization");
        };
        runtime.realize_virtual(&lease, index, *view);
        let before = runtime
            .graph()
            .virtual_realization(collection, RealizedContainer(1))
            .unwrap();

        // Fresh view/source closures must not reset the native key/index sequence.
        let next = if lazy {
            ItemsRepeater::new().virtual_source(VirtualSource::new(
                0,
                1,
                |_| "row",
                |_| TextBlock::new().text("After"),
            ))
        } else {
            ItemsRepeater::new().item("row", TextBlock::new().text("After"))
        };
        let mutations = runtime.update(next);
        assert!(
            !mutations
                .iter()
                .any(|mutation| matches!(mutation, Mutation::SetVirtualSource { .. }))
        );
        let Some(NativeWork::Virtual(VirtualWork::Realize {
            lease, index, view, ..
        })) = runtime.next_native_work()
        else {
            panic!("expected row content refresh");
        };
        runtime.realize_virtual(&lease, index, *view);
        assert_eq!(
            runtime
                .graph()
                .virtual_realization(collection, RealizedContainer(1)),
            Some(before)
        );
        assert_eq!(
            runtime.graph().properties(before).unwrap()[0].value,
            PropertyValue::String(Rc::from("After"))
        );
    }
}

#[test]
fn virtual_items_realize_only_requested_rows() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    let source = VirtualSource::new(1, 10_000, Key::from, |index| -> View {
        TextBlock::new().text(index.to_string()).into()
    });
    runtime.update(ItemsRepeater::new().virtual_source(source));
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
    })) = runtime.next_native_work()
    else {
        panic!("expected realization");
    };
    runtime.realize_virtual(&lease, index, *view);

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
    runtime.update(
        ItemsRepeater::new()
            .item(1_u64, TextBlock::new().text("one"))
            .item(2_u64, TextBlock::new().text("two")),
    );
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
        })) = runtime.next_native_work()
        else {
            panic!("expected realization");
        };
        runtime.realize_virtual(&lease, index, *view);
        lease
    };
    let first = realize(&mut runtime, RealizedContainer(1), 0);
    let child = runtime
        .adapter()
        .children(collection, RelationId::Items)
        .unwrap()[0];
    let moved = realize(&mut runtime, RealizedContainer(2), 0);
    assert_eq!(moved.key, first.key);
    assert_eq!(moved.revision, first.revision);
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    runtime.dispatch_native_events();
    assert_eq!(
        runtime
            .adapter()
            .children(collection, RelationId::Items)
            .unwrap(),
        &[child]
    );
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    let second = realize(&mut runtime, RealizedContainer(2), 1);
    assert_ne!(first.key, second.key);
    assert_eq!(runtime.adapter().object_count(), 2);
    assert_eq!(runtime.adapter().realized_count(collection), 1);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(2),
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Recycle { lease })) = runtime.next_native_work()
    else {
        panic!("expected recycle");
    };
    runtime.recycle_virtual(&lease);
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
}

#[test]
fn invalid_virtual_replacement_rolls_back_container_ownership() {
    let original: ElementRef = ElementRef::default();
    let replacement: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        ItemsRepeater::new()
            .item("original", Button::new().element_ref(&original))
            .item("replacement", TextBox::new().element_ref(&replacement)),
    );
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events();
    let original_id = original.get().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 1,
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Realize {
        lease, index, view, ..
    })) = runtime.next_native_work()
    else {
        panic!("expected replacement realization");
    };
    let mut invalid = lease.clone();
    invalid.revision += 1;
    let invalid_view = view.as_ref().clone();
    assert_panics(&format!("invalid realization 1 for {collection:?}"), || {
        runtime.realize_virtual(&invalid, index, invalid_view);
    });
    assert_eq!(original.get(), Some(original_id));
    assert_eq!(replacement.get(), None);
    runtime.realize_virtual(&lease, index, *view);
    assert_eq!(original.get(), None);
    assert!(replacement.get().is_some());
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.adapter().object_count(), 2);
}

#[test]
fn virtual_items_preserve_keyed_child_across_reorder() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("a"))
            .item("b", TextBlock::new().text("b")),
    );
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
        })) = runtime.next_native_work()
        else {
            panic!("expected realization");
        };
        runtime.realize_virtual(&lease, index, *view);
    }
    let before = runtime
        .adapter()
        .children(collection, RelationId::Items)
        .unwrap()
        .to_vec();

    runtime.update(
        ItemsRepeater::new()
            .item("b", TextBlock::new().text("b"))
            .item("a", TextBlock::new().text("updated")),
    );
    for _ in 0..2 {
        let Some(NativeWork::Virtual(VirtualWork::Realize {
            lease, index, view, ..
        })) = runtime.next_native_work()
        else {
            panic!("expected refresh");
        };
        runtime.realize_virtual(&lease, index, *view);
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
#[should_panic(expected = "duplicate key")]
fn virtual_items_reject_duplicate_keys() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ItemsRepeater::new());
    runtime.update(
        ItemsRepeater::new()
            .item("duplicate", TextBlock::new().text("first"))
            .item("duplicate", TextBlock::new().text("second")),
    );
}

#[test]
fn virtual_items_ignore_stale_source_requests() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ItemsRepeater::new().item("first", TextBlock::new().text("first")));
    let collection = runtime.graph().root().unwrap();
    runtime.update(ItemsRepeater::new().item("second", TextBlock::new().text("second")));
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(matches!(
        runtime.next_native_work(),
        Some(NativeWork::Maintenance)
    ));
    assert!(runtime.next_native_work().is_none());
    assert_eq!(runtime.adapter().object_count(), 1);
}

#[test]
fn virtual_items_remove_active_rows_when_source_becomes_empty() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ItemsRepeater::new().item("first", TextBlock::new().text("first")));
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
    })) = runtime.next_native_work()
    else {
        panic!("expected realization");
    };
    runtime.realize_virtual(&lease, index, *view);

    runtime.update(ItemsRepeater::new());
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
fn virtual_realization_requests_validate_at_work_boundaries() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        ItemsRepeater::new()
            .width(100.0)
            .item("only", TextBlock::new().text("only")),
    );
    let collection = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: collection,
        property: Property {
            id: PropertyId::Width,
            value: PropertyValue::F64(200.0),
        },
    });
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: collection,
        property: Property {
            id: PropertyId::Width,
            value: PropertyValue::F64(300.0),
        },
    });
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(2),
            index: 1,
            source_revision: 0,
        });

    assert!(!runtime.prepare_update());
    let Some(NativeWork::Virtual(VirtualWork::Realize {
        lease, index, view, ..
    })) = runtime.next_native_work()
    else {
        panic!("expected first realization");
    };
    assert_eq!(index, 0);
    runtime.realize_virtual(&lease, index, *view);
    assert_eq!(
        runtime.graph().properties(collection).unwrap(),
        &[Property {
            id: PropertyId::Width,
            value: PropertyValue::F64(200.0),
        }]
    );
    assert_panics(&format!("invalid realization 1 for {collection:?}"), || {
        while runtime.next_native_work().is_some() {}
    });
}

#[test]
fn duplicate_virtual_recycle_is_idempotent() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ItemsRepeater::new().item("only", TextBlock::new().text("only")));
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events();
    for _ in 0..2 {
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 0,
            });
    }
    runtime.dispatch_native_events();
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
}

#[test]
fn realized_virtual_children_recycle_before_repeater_destruction() {
    let mut adapter = RecordingAdapter::new();
    adapter.record_batches(true);
    let mut runtime = Runtime::new(adapter);
    runtime.update(Grid::new().keyed_children([keyed(
        "repeater",
        ItemsRepeater::new().item("row", TextBlock::new().text("row")),
    )]));
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
    runtime.dispatch_native_events();

    runtime.update(Grid::new());
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
    runtime.update(Grid::new().keyed_children([keyed(
        "slot",
        ItemsRepeater::new().item("row", TextBlock::new().text("row")),
    )]));
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
    runtime.dispatch_native_events();

    runtime.update(Grid::new().keyed_children([keyed("slot", Border::new())]));
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
    runtime.update(view());
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(!runtime.prepare_update());
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.dispatch_native_events(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    assert!(!runtime.prepare_update());
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events(), 0);
}

#[test]
fn public_event_api_preserves_virtual_work_until_ordered_dispatch() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        ItemsRepeater::new()
            .item("old", TextBlock::new().text("old"))
            .item("new", TextBlock::new().text("new")),
    );
    let collection = runtime.graph().root().unwrap();

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    assert_panics("virtual realization is pending", || {
        runtime.next_native_event();
    });
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.dispatch_native_events(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    assert_panics("virtual realization is pending", || {
        runtime.next_native_event();
    });
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(2),
            index: 1,
            source_revision: 0,
        });
    assert_panics("virtual realization is pending", || {
        runtime.next_native_event();
    });
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(2),
            source_revision: 0,
        });
    assert_panics("virtual realization is pending", || {
        runtime.next_native_event();
    });
    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events(), 0);
}

#[test]
fn recycle_before_dispatch_cancels_realization_and_allows_token_reuse() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(
        ItemsRepeater::new()
            .item("old", TextBlock::new().text("old"))
            .item("new", TextBlock::new().text("new")),
    );
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    assert!(!runtime.prepare_update());
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

    assert_eq!(runtime.dispatch_native_events(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.graph().object_count(), 2);
    assert_eq!(runtime.dispatch_native_events(), 0);
    runtime.update(ItemsRepeater::new());
    assert_eq!(runtime.graph().full_reference_scan_count(), 0);
}
