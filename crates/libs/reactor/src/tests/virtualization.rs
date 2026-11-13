use super::*;

#[test]
fn virtual_refreshes_do_not_revive_released_containers_after_reset() {
    struct ResetAdapter {
        inner: RecordingAdapter,
        released: Option<ObjectId>,
        reclaim: bool,
    }

    impl Adapter for ResetAdapter {
        fn peek_native_event(&self) -> Option<NativeEvent> {
            self.inner.peek_native_event()
        }

        fn pop_native_event(&mut self) -> Option<NativeEvent> {
            self.inner.pop_native_event()
        }

        fn container_leased(&self, collection: ObjectId, container: RealizedContainer) -> bool {
            self.released != Some(collection) || container != RealizedContainer(1)
        }

        fn apply(&mut self, mutations: &[Mutation]) {
            for mutation in mutations {
                if let Mutation::SetVirtualSource {
                    object,
                    keys,
                    source_revision: 1,
                } = mutation
                {
                    // Reset ends the old native lease before LayoutUpdated queues its recycle.
                    self.released = Some(*object);
                    if self.reclaim {
                        self.inner
                            .queue_realization(RealizationRequest::RealizeKey {
                                collection: *object,
                                container: RealizedContainer(3),
                                index: 1,
                                token: keys[1],
                            });
                    }
                }
                if let Mutation::Realize {
                    parent, container, ..
                } = mutation
                {
                    assert!(
                        self.container_leased(*parent, *container),
                        "refresh into ended lease"
                    );
                }
            }
            self.inner.apply(mutations);
        }
    }

    for reclaim in [false, true] {
        let mut runtime = Runtime::new(ResetAdapter {
            inner: RecordingAdapter::new(),
            released: None,
            reclaim,
        });
        runtime.update(
            ListView::new()
                .item(0_u64, TextBlock::new().text("zero"))
                .item(1_u64, TextBlock::new().text("one")),
        );
        let collection = runtime.graph().root().unwrap();
        for index in 0..2 {
            runtime
                .adapter_mut()
                .inner
                .queue_realization(RealizationRequest::Realize {
                    collection,
                    container: RealizedContainer(index as u64 + 1),
                    index,
                    source_revision: 0,
                });
        }
        runtime.dispatch_native_events();
        let old = runtime
            .graph()
            .virtual_realization(collection, RealizedContainer(1))
            .unwrap();
        let retained = runtime
            .graph()
            .virtual_realization(collection, RealizedContainer(2))
            .unwrap();

        runtime.update(
            ListView::new()
                .item(1_u64, TextBlock::new().text("updated one"))
                .item(0_u64, TextBlock::new().text("updated zero")),
        );
        runtime.dispatch_native_events();
        assert_eq!(
            runtime.graph().properties(retained).unwrap()[0].value,
            PropertyValue::String(Rc::from("updated one"))
        );
        assert_eq!(runtime.graph().kind(old), Some(ObjectType::TextBlock));
        if reclaim {
            assert_eq!(
                runtime
                    .graph()
                    .virtual_realization(collection, RealizedContainer(3)),
                Some(old)
            );
            assert_eq!(
                runtime.graph().properties(old).unwrap()[0].value,
                PropertyValue::String(Rc::from("updated zero"))
            );
        }

        runtime
            .adapter_mut()
            .inner
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 1,
            });
        runtime.dispatch_native_events();
        assert_eq!(
            runtime.graph().kind(old),
            reclaim.then_some(ObjectType::TextBlock)
        );
        assert_eq!(
            runtime.adapter().inner.realized_count(collection),
            if reclaim { 2 } else { 1 }
        );
    }
}

#[test]
fn virtual_rows_follow_keys_after_native_reorder_before_render() {
    check_native_reorder_before_render(false);
}

#[test]
fn lazy_virtual_rows_follow_keys_after_native_reorder_before_render() {
    check_native_reorder_before_render(true);
}

fn check_native_reorder_before_render(lazy: bool) {
    let keys = ["first", "second", "third"];
    let view = |keys: [&'static str; 3], prefix: &'static str| {
        let list = ListView::new().can_drag_items(true).can_reorder_items(true);
        if lazy {
            list.virtual_source(VirtualSource::new(
                0,
                keys.len(),
                move |index| keys[index],
                move |index| TextBlock::new().text(format!("{prefix}{}", keys[index])),
            ))
        } else {
            list.items(keys.map(|key| keyed(key, TextBlock::new().text(format!("{prefix}{key}")))))
        }
    };
    for adopt in [false, true] {
        let mut runtime = Runtime::new(RecordingAdapter::new());
        let mutations = runtime.update(view(keys, ""));
        let collection = runtime.graph().root().unwrap();
        let tokens = mutations
            .into_iter()
            .find_map(|mutation| match mutation {
                Mutation::SetVirtualSource { keys, .. } => Some(keys),
                _ => None,
            })
            .unwrap();
        for order in [[1, 2, 0], [2, 0, 1]] {
            runtime.adapter_mut().observe(Observation::VirtualReorder {
                object: collection,
                tokens: Arc::new(order.map(|index| tokens[index]).to_vec()),
            });
            runtime
                .adapter_mut()
                .queue_realization(RealizationRequest::RealizeKey {
                    collection,
                    container: RealizedContainer(1),
                    index: 0,
                    token: tokens[order[0]],
                });
            runtime.dispatch_native_events();
            let row = runtime
                .graph()
                .virtual_realization(collection, RealizedContainer(1))
                .unwrap();
            assert_eq!(
                runtime.graph().properties(row).unwrap()[0].value,
                PropertyValue::String(Rc::from(keys[order[0]])),
                "lazy={lazy}, order={order:?}"
            );
        }

        let next = if adopt {
            [keys[2], keys[0], keys[1]]
        } else {
            keys
        };
        runtime.update(view(next, "updated "));
        runtime.dispatch_native_events();
        let row = runtime
            .graph()
            .virtual_realization(collection, RealizedContainer(1))
            .unwrap();
        assert_eq!(
            runtime.graph().properties(row).unwrap()[0].value,
            PropertyValue::String(Rc::from("updated third")),
            "lazy={lazy}, adopt={adopt}"
        );
    }
}

#[test]
fn virtual_row_kind_changes_detach_once_before_destruction() {
    for container in [RealizedContainer(1), RealizedContainer(2)] {
        let mut adapter = RecordingAdapter::new();
        adapter.record_batches(true);
        let mut runtime = Runtime::new(adapter);
        runtime.update(
            ListView::new().item("row", Border::new().content(TextBlock::new().text("row"))),
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
        let previous = runtime
            .graph()
            .virtual_realization(collection, RealizedContainer(1))
            .unwrap();
        let start = runtime.adapter().batches().len();
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index: 0,
                source_revision: 0,
            });
        let Some(NativeWork::Virtual(VirtualWork::Realize { lease, index, .. })) =
            runtime.next_native_work()
        else {
            panic!("expected realization");
        };
        runtime.realize_virtual(&lease, index, TextBox::new());
        let mutations = runtime.adapter().batches()[start..]
            .iter()
            .flatten()
            .collect::<Vec<_>>();
        let detached = mutations.iter().enumerate().filter_map(|(index, mutation)| {
            matches!(mutation, Mutation::Recycle { child: Some(child), .. } if *child == previous)
                .then_some(index)
        }).collect::<Vec<_>>();
        assert_eq!(detached.len(), 1);
        let destroyed = mutations
            .iter()
            .position(|mutation| {
                matches!(
                    mutation, Mutation::Destroy { object } if *object == previous
                )
            })
            .unwrap();
        assert!(detached[0] < destroyed);
        let replacement = runtime
            .graph()
            .virtual_realization(collection, container)
            .unwrap();
        assert_eq!(runtime.graph().kind(previous), None);
        assert_eq!(runtime.graph().kind(replacement), Some(ObjectType::TextBox));
        assert_eq!(runtime.adapter().object_count(), 2);
    }
}

#[test]
fn virtual_row_content_updates_do_not_reset_the_native_source() {
    for lazy in [false, true] {
        let view = |text: &'static str| {
            if lazy {
                ListView::new().virtual_source(VirtualSource::new(
                    0,
                    1,
                    |_| "row",
                    move |_| TextBlock::new().text(text),
                ))
            } else {
                ListView::new().item("row", TextBlock::new().text(text))
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
            ListView::new().virtual_source(VirtualSource::new(
                0,
                1,
                |_| "row",
                |_| TextBlock::new().text("After"),
            ))
        } else {
            ListView::new().item("row", TextBlock::new().text("After"))
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
    runtime.update(ListView::new().virtual_source(source));
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
        ListView::new()
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
        ListView::new()
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
        ListView::new()
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
        ListView::new()
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
    for (index, child) in before.iter().enumerate() {
        assert_eq!(
            runtime
                .graph()
                .virtual_realization(collection, RealizedContainer(index as u64)),
            Some(*child),
            "a retained key must keep its native shell"
        );
    }
}

#[test]
fn virtual_items_acknowledge_recycle_after_source_reset() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ListView::new().item("a", TextBlock::new().text("a")));
    let collection = runtime.graph().root().unwrap();
    let container = RealizedContainer(1);
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container,
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events();
    runtime.update(ListView::new());
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container,
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Cancel {
        collection: owner,
        relation,
        container: recycled,
    })) = runtime.next_native_work()
    else {
        panic!("native recycling must be acknowledged after logical removal");
    };
    assert_eq!(owner, collection);
    assert_eq!(recycled, container);
    let batch = runtime.plan_cancel_virtual(owner, relation, recycled);
    assert!(matches!(
        batch.mutations(),
        [Mutation::Recycle { child: None, .. }]
    ));
}

#[test]
fn virtual_items_resolve_native_keys_without_reviving_removed_rows() {
    let view = |keys: &[Key]| {
        let mut view = ListView::new();
        for key in keys {
            view = view.item(key.clone(), TextBlock::new().text("row"));
        }
        view
    };
    let mut runtime = Runtime::new(RecordingAdapter::new());
    let keys = [Key::from(1u64), Key::from("1"), Key::from("a\0b")];
    let update = runtime.update(view(&keys));
    let collection = runtime.graph().root().unwrap();
    let tokens = update
        .into_iter()
        .find_map(|mutation| match mutation {
            Mutation::SetVirtualSource { keys, .. } => Some(keys),
            _ => None,
        })
        .unwrap();
    assert_ne!(tokens[0], tokens[1]);
    assert_ne!(tokens[1], tokens[2]);

    runtime.update(view(&[keys[2].clone(), keys[0].clone()]));
    let current = runtime
        .update(view(&[keys[2].clone(), keys[1].clone(), keys[0].clone()]))
        .into_iter()
        .find_map(|mutation| match mutation {
            Mutation::SetVirtualSource { keys, .. } => Some(keys),
            _ => None,
        })
        .unwrap();
    assert_eq!(current[0], tokens[2]);
    assert_eq!(current[2], tokens[0]);
    assert_ne!(current[1], tokens[1]);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::RealizeKey {
            collection,
            container: RealizedContainer(1),
            index: 2,
            token: tokens[2],
        });
    let Some(NativeWork::Virtual(VirtualWork::Realize {
        lease, index, view, ..
    })) = runtime.next_native_work()
    else {
        panic!("a surviving native key must realize at its new index");
    };
    assert_eq!(index, 0);
    assert_eq!(lease.key, keys[2]);
    runtime.realize_virtual(&lease, index, *view);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::RealizeKey {
            collection,
            container: RealizedContainer(2),
            index: 1,
            token: tokens[1],
        });
    assert!(matches!(
        runtime.next_native_work(),
        Some(NativeWork::Maintenance)
    ));
    assert!(runtime.next_native_work().is_none());
    assert_eq!(runtime.adapter().realized_count(collection), 1);
}

#[test]
#[should_panic(expected = "duplicate key")]
fn virtual_items_reject_duplicate_keys() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ListView::new());
    runtime.update(
        ListView::new()
            .item("duplicate", TextBlock::new().text("first"))
            .item("duplicate", TextBlock::new().text("second")),
    );
}

fn virtual_key_view(revision: u64, keys: &[Key]) -> ListView {
    let keys = keys.to_vec();
    ListView::new().virtual_source(VirtualSource::new(
        revision,
        keys.len(),
        move |index| keys[index].clone(),
        |_| TextBlock::new().text("row"),
    ))
}

#[test]
#[should_panic(expected = "duplicate key")]
fn virtual_source_rejects_duplicate_keys_on_mount() {
    Runtime::new(RecordingAdapter::new()).update(virtual_key_view(
        0,
        &[Key::from("duplicate"), Key::from("duplicate")],
    ));
}

#[test]
#[should_panic(expected = "duplicate key")]
fn virtual_source_rejects_duplicate_keys_after_revision_change() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(virtual_key_view(
        0,
        &[Key::from("first"), Key::from("second")],
    ));
    runtime.update(virtual_key_view(
        1,
        &[Key::from("first"), Key::from("first")],
    ));
}

#[test]
fn virtual_key_revisions_preserve_only_surviving_tokens() {
    let tokens = |update: UpdateStats| {
        update
            .into_iter()
            .find_map(|mutation| match mutation {
                Mutation::SetVirtualSource { keys, .. } => Some(keys),
                _ => None,
            })
            .unwrap()
    };
    let mut runtime = Runtime::new(RecordingAdapter::new());
    let keys = [Key::from(1u64), Key::from("1"), Key::from("")];
    let original = tokens(runtime.update(virtual_key_view(0, &keys)));
    let unchanged = runtime.update(virtual_key_view(1, &keys));
    assert!(
        !unchanged
            .iter()
            .any(|mutation| matches!(mutation, Mutation::SetVirtualSource { .. }))
    );
    let reordered =
        tokens(runtime.update(virtual_key_view(2, &[keys[2].clone(), keys[0].clone()])));
    assert_eq!(reordered.as_slice(), &[original[2], original[0]]);
    assert!(tokens(runtime.update(virtual_key_view(3, &[]))).is_empty());
    let restored = tokens(runtime.update(virtual_key_view(4, &keys)));
    assert!(restored.iter().all(|token| !original.contains(token)));
}

#[test]
fn virtual_items_ignore_stale_source_requests() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ListView::new().item("first", TextBlock::new().text("first")));
    let collection = runtime.graph().root().unwrap();
    runtime.update(ListView::new().item("second", TextBlock::new().text("second")));
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
    runtime.update(ListView::new().item("first", TextBlock::new().text("first")));
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

    runtime.update(ListView::new());
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
        ListView::new()
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
    runtime.update(ListView::new().item("only", TextBlock::new().text("only")));
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
        "list",
        ListView::new().item("row", TextBlock::new().text("row")),
    )]));
    let root = runtime.graph().root().unwrap();
    let list = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection: list,
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
                Mutation::Recycle { parent, .. } if *parent == list
            )
        })
        .unwrap();
    let destroy = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Destroy { object } if *object == list
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
            } if *parent == list
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
        ListView::new().item("row", TextBlock::new().text("row")),
    )]));
    let root = runtime.graph().root().unwrap();
    let list = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection: list,
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
                Mutation::Recycle { parent, .. } if *parent == list
            )
        })
        .unwrap();
    let replace = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Destroy { object } if *object == list
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
            } if *parent == list
        )
    }));
}

#[test]
fn update_peeks_pending_virtual_work_without_consuming_it() {
    let view = || ListView::new().item("row", TextBlock::new().text("row"));
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
        ListView::new()
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
        ListView::new()
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
    runtime.update(ListView::new());
    assert_eq!(runtime.graph().full_reference_scan_count(), 0);
}
