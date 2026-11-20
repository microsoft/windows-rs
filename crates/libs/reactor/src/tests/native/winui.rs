use super::*;

#[test]
fn image_cache_keeps_decoded_bytes_while_the_app_holds_them() {
    let created = Cell::new(0);
    let get = |cache: &mut ImageCache<u32>, source: &ImageSourceValue| {
        cache
            .get_or_insert(source, || {
                created.set(created.get() + 1);
                Ok::<_, ()>(created.get())
            })
            .unwrap()
    };
    let svg = |bytes: &[u8]| ImageSourceValue::Svg(EncodedImage::new(bytes.to_vec()));
    let mut cache = ImageCache::default();

    // Bytes the app holds stay decoded after their controls go away, and clones share them.
    let bytes = EncodedImage::new(b"<svg/>".to_vec());
    let held = ImageSourceValue::Svg(bytes.clone());
    drop(get(&mut cache, &held));
    cache.prune(|_| false);
    assert_eq!(**get(&mut cache, &ImageSourceValue::Svg(bytes.clone())), 1);

    // Separate allocations decode separately, and dropping one does not release the other.
    let copy = svg(b"<svg/>");
    assert_eq!(**get(&mut cache, &copy), 2);
    drop(copy);
    cache.prune(|_| false);
    assert_eq!(**get(&mut cache, &held), 1);

    // The same bytes as a bitmap are a different source.
    assert_eq!(
        **get(&mut cache, &ImageSourceValue::Encoded(bytes.clone())),
        3
    );

    // Once the app drops the bytes, the source is released.
    drop((held, bytes));
    cache.prune(|_| false);
    assert!(cache.bytes.is_empty());

    // Static bytes are always held.
    let fixed = ImageSourceValue::Encoded(EncodedImage::from_static(b"png"));
    drop(get(&mut cache, &fixed));
    cache.prune(|_| false);
    assert_eq!(**get(&mut cache, &fixed), 4);

    // URI sources live only while a control uses them.
    let uri = ImageSourceValue::Uri(Rc::from("ms-appx:///icon.png"));
    let shown = get(&mut cache, &uri);
    assert!(Rc::ptr_eq(&get(&mut cache, &uri), &shown));
    drop(shown);
    cache.prune(|_| false);
    assert_eq!(**get(&mut cache, &uri), 6);
}

#[test]
fn image_cache_gives_deferred_sources_one_consumer_at_a_time() {
    let created = Cell::new(0);
    let get = |cache: &mut ImageCache<u32>, source: &ImageSourceValue| {
        cache
            .get_or_insert(source, || {
                created.set(created.get() + 1);
                Ok::<_, ()>(created.get())
            })
            .unwrap()
    };
    let mut cache = ImageCache::default();

    for source in [
        ImageSourceValue::Svg(EncodedImage::new(b"<svg/>".to_vec())),
        ImageSourceValue::Uri(Rc::from("ms-appx:///icon.SVG?v=1")),
        ImageSourceValue::Encoded(EncodedImage::new(b"png".to_vec())),
    ] {
        let first = get(&mut cache, &source);
        let second = get(&mut cache, &source);
        assert_ne!(**first, **second);

        // A released source goes to the next control while the other stays in use.
        let value = **first;
        let kept = first.release();
        let third = get(&mut cache, &source);
        assert_eq!(**third, value);
        drop((kept, second, third));
    }
}

#[test]
fn image_cache_drops_unused_sources_that_have_not_started_loading() {
    for source in [
        ImageSourceValue::Svg(EncodedImage::new(b"<svg/>".to_vec())),
        ImageSourceValue::Encoded(EncodedImage::new(b"png".to_vec())),
    ] {
        let mut cache = ImageCache::<u32>::default();
        let get = |cache: &mut ImageCache<u32>, value| {
            cache.get_or_insert(&source, || Ok::<_, ()>(value)).unwrap()
        };

        // A pending source in use stays; an unused one goes even while the app holds its bytes.
        let shown = get(&mut cache, 1);
        drop(get(&mut cache, 2));
        cache.prune(|_| true);
        assert_eq!(**get(&mut cache, 3), 3);
        drop(shown);
        cache.prune(|_| true);
        assert!(cache.bytes.is_empty());
    }
}

#[test]
fn image_load_subscriptions_complete_once_and_do_not_retain_controls() {
    let state = ImageLoadState::default();
    let values = Rc::new(RefCell::new(Vec::new()));
    let result = Rc::clone(&values);
    let callback: Rc<dyn Fn(bool)> = Rc::new(move |value| result.borrow_mut().push(value));
    let removed: Rc<dyn Fn(bool)> = Rc::new(|_| panic!("removed subscription"));
    assert_eq!(state.subscribe(&removed), None);
    drop(removed);
    assert_eq!(state.subscribe(&callback), None);
    state.complete(true);
    state.complete(false);
    assert_eq!(*values.borrow(), [true]);
    assert_eq!(state.subscribe(&callback), Some(true));
    assert_eq!(*values.borrow(), [true]);
    assert!(state.subscribers.borrow().is_empty());
}

#[test]
fn native_queue_preview_uses_current_callback_and_preserves_order() {
    let object = ObjectId::test(1);
    let first = Callback::new(|_: Key| panic!("preview must not invoke callbacks"));
    let second = Callback::new(|_: Key| panic!("preview must not invoke callbacks"));
    let mut adapter = WinUiAdapter::default();
    adapter.menus.insert(
        object,
        NativeMenu {
            menu: Menu::new([], first),
            revision: 1,
            flyout: None,
            _revokers: Vec::new(),
        },
    );
    let observation = Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Before")),
        },
    };
    adapter.event_queue.queue(
        None,
        Some(QueuedEvent {
            object,
            event: EventId::MenuItemInvoked,
            handler: QueuedHandler::Presentation {
                revision: 0,
                key: "stale".into(),
            },
        }),
    );
    adapter.event_queue.queue(Some(observation.clone()), None);
    adapter.event_queue.queue(
        None,
        Some(QueuedEvent {
            object,
            event: EventId::MenuItemInvoked,
            handler: QueuedHandler::Presentation {
                revision: 1,
                key: "current".into(),
            },
        }),
    );
    adapter.event_queue.queue(Some(observation.clone()), None);

    // A stale click resolves to nothing, so peek and pop both skip it.
    let next = Some(NativeEvent::Observation(observation.clone()));
    assert_eq!(adapter.peek_native_event(), next);
    assert_eq!(adapter.event_queue.events.borrow().len(), 4);
    assert_eq!(adapter.pop_native_event(), next);
    assert_eq!(adapter.event_queue.events.borrow().len(), 2);

    // Callback replacement preserves the menu presentation revision.
    adapter.menus.get_mut(&object).unwrap().menu.on_click = second.clone();
    let current = Some(NativeEvent::Event {
        observation: None,
        event: EventDispatch::new(
            object,
            EventId::MenuItemInvoked,
            EventCall::Key(second, "current".into()),
        ),
    });
    assert_eq!(adapter.peek_native_event(), current);
    assert_eq!(adapter.pop_native_event(), current);
    assert_eq!(
        adapter.pop_native_event(),
        Some(NativeEvent::Observation(observation))
    );
    assert_eq!(adapter.pop_native_event(), None);
}

#[test]
fn native_queue_expired_report_keeps_only_its_observation() {
    let object = ObjectId::test(1);
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = Callback::new(move |value: Rc<str>| {
        values.borrow_mut().push(value);
    });
    let observation = |value: &str| Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from(value)),
        },
    };
    let report = |value: &str, live: bool| QueuedNativeEvent::Event {
        observation: Some(observation(value)),
        event: QueuedEvent {
            object,
            event: EventId::TextChanged,
            handler: QueuedHandler::Callback {
                call: EventCall::String(callback.clone(), Rc::from(value)),
                live: Rc::new(Cell::new(live)),
            },
        },
    };
    let mut adapter = WinUiAdapter::default();
    // `on_text_changed` was removed and added again between the two reports.
    adapter
        .event_queue
        .events
        .borrow_mut()
        .extend([report("Expired", false), report("Live", true)]);
    let expected = Some(NativeEvent::Event {
        observation: Some(observation("Live")),
        event: EventDispatch::new(
            object,
            EventId::TextChanged,
            EventCall::String(callback.clone(), Rc::from("Live")),
        ),
    });
    let expired = Some(NativeEvent::Observation(observation("Expired")));
    assert_eq!(adapter.peek_native_event(), expired);
    assert_eq!(adapter.pop_native_event(), expired);
    assert_eq!(adapter.peek_native_event(), expected);
    assert_eq!(adapter.pop_native_event(), expected);
    assert_eq!(adapter.pop_native_event(), None);
}

#[test]
fn image_source_change_discards_only_queued_load_results() {
    let image = ObjectId::test(1);
    let other = ObjectId::test(2);
    let queue = Rc::new(NativeEventQueue::default());
    let event = |object, event| QueuedEvent {
        object,
        event,
        handler: QueuedHandler::Callback {
            call: EventCall::Unit(Callback::new(|()| {})),
            live: Rc::new(Cell::new(true)),
        },
    };
    queue.queue(None, Some(event(image, EventId::ImageOpened)));
    queue.queue(None, Some(event(image, EventId::ImageFailed)));
    queue.queue(None, Some(event(image, EventId::PointerPressed)));
    queue.queue(None, Some(event(other, EventId::ImageOpened)));

    queue.discard_events(image, &IMAGE_SOURCE_EVENTS);

    let events = queue.events.borrow();
    let remaining: Vec<_> = events
        .iter()
        .map(|queued| {
            let event = queued.event().unwrap();
            (event.object, event.event)
        })
        .collect();
    assert_eq!(
        remaining,
        [
            (image, EventId::PointerPressed),
            (other, EventId::ImageOpened)
        ]
    );
}

#[test]
fn replacement_discards_queued_reorders_without_callbacks() {
    let replaced = ObjectId::test(1);
    let other = ObjectId::test(2);
    let queue = Rc::new(NativeEventQueue::default());
    let reorder = |object| Observation::Reorder {
        object,
        relation: RelationId::Items,
        children: Vec::new(),
    };
    queue.queue(Some(reorder(replaced)), None);
    queue.queue(Some(reorder(other)), None);

    queue.discard_object(replaced);

    let events = queue.events.borrow();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].observation(), Some(&reorder(other)));
}

#[test]
fn placement_coordinates_round_trip_negative_monitors_and_taskbar_offsets() {
    let screen = native::RECT {
        left: -1600,
        top: -200,
        right: -800,
        bottom: 400,
    };
    for (x, y) in [(0, 0), (48, 0), (0, 40), (48, 40)] {
        let workspace = placement::translate_rect(screen, -x, -y).unwrap();
        let restored = placement::translate_rect(workspace, x, y).unwrap();
        assert_eq!(restored, screen);
        assert_eq!(workspace.left, screen.left - x);
        assert_eq!(workspace.top, screen.top - y);
    }
    assert!(
        placement::translate_rect(
            native::RECT {
                left: i32::MAX,
                ..Default::default()
            },
            1,
            0,
        )
        .is_err()
    );
}

#[test]
fn initial_placement_rejects_invalid_extents_but_accepts_negative_positions() {
    let placement = WindowPlacement {
        x: -1200,
        y: -600,
        width: 800,
        height: 500,
        maximized: true,
    };
    let visuals = WindowVisuals::new().initial_placement(placement);
    assert_eq!(visuals.initial_placement, Some(placement));
    for invalid in [
        WindowPlacement {
            width: 0,
            ..placement
        },
        WindowPlacement {
            height: -1,
            ..placement
        },
        WindowPlacement {
            x: i32::MAX,
            ..placement
        },
        WindowPlacement {
            y: i32::MAX,
            ..placement
        },
    ] {
        assert!(
            std::panic::catch_unwind(|| WindowVisuals::new().initial_placement(invalid)).is_err()
        );
    }
}

#[test]
fn changed_initial_placement_does_not_become_a_reactive_resize() {
    let initial = WindowVisuals::new()
        .client_size(640.0, 480.0)
        .initial_position(ScreenPoint::new(50, 60));
    let updated = initial.clone().initial_placement(WindowPlacement {
        x: -1200,
        y: -600,
        width: 800,
        height: 500,
        maximized: true,
    });
    let changes = window_visual_changes(&initial, &updated);
    assert!(!changes.client_size);
    assert!(!changes.constraints);
    assert!(!changes.backdrop);
    assert!(!changes.icon);
    assert!(!changes.theme);
}

#[test]
fn content_dialog_schedule_is_fifo_and_cancellable() {
    let first = ObjectId::test(1);
    let second = ObjectId::test(2);
    let third = ObjectId::test(3);
    let mut schedule = ContentDialogSchedule::default();

    schedule.enqueue(first);
    schedule.enqueue(second);
    schedule.enqueue(second);
    schedule.enqueue(third);
    schedule.cancel(second);

    assert_eq!(schedule.next(), Some(first));
    assert_eq!(schedule.next(), Some(third));
    assert_eq!(schedule.next(), None);
}

#[test]
fn content_dialog_schedule_ignores_stale_completion_generation() {
    let dialog = ObjectId::test(1);
    let mut schedule = ContentDialogSchedule::default();
    schedule.begin(dialog, 7);

    assert!(!schedule.complete(dialog, 6));
    assert_eq!(schedule.active, Some((dialog, 7)));
    assert!(schedule.complete(dialog, 7));
    assert_eq!(schedule.active, None);
}

fn virtual_key(value: IInspectable) -> u64 {
    value
        .cast::<windows_reference::IReference<u64>>()
        .unwrap()
        .Value()
        .unwrap()
}

#[test]
fn recycled_physical_shell_keeps_old_token_until_acknowledged() {
    let mut shells = ShellPool::<usize>::default();
    let (old, physical) = shells.take(|| Ok::<_, windows_core::Error>(42)).unwrap();

    assert!(shells.retire(old));
    assert!(shells.retired.contains(&old));
    let (new, reused) = shells
        .take(|| panic!("retired physical shell was not reused"))
        .unwrap();

    assert_ne!(new, old);
    assert_eq!(reused, physical);
    assert!(shells.retired.contains(&old));
    assert!(shells.shells.contains_key(&new));

    shells.acknowledge_recycle(old);
    assert!(!shells.retired.contains(&old));
    assert!(shells.shells.contains_key(&new));
}

#[test]
fn virtual_source_exposes_keys_as_values() {
    let source = ComObject::new(NativeVirtualSource::new(Arc::new(vec![10, 11, 12])).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();

    assert_eq!(vector.Size().unwrap(), 3);
    assert_eq!(virtual_key(vector.GetAt(0).unwrap()), 10);
    assert_eq!(virtual_key(vector.GetAt(2).unwrap()), 12);
    assert_eq!(vector.GetAt(3).unwrap_err().code(), E_BOUNDS);

    let mut index = u32::MAX;
    assert!(vector.IndexOf(&virtual_item_value(12), &mut index).unwrap());
    assert_eq!(index, 2);
    assert!(!vector.IndexOf(&virtual_item_value(2), &mut index).unwrap());
    assert_eq!(index, 0);
    let invalid: IInspectable =
        windows_reference::IReference::<HSTRING>::from(HSTRING::from("invalid")).into();
    assert!(vector.IndexOf(&invalid, &mut index).is_err());

    let mut values = vec![None; 4];
    assert_eq!(vector.GetMany(1, &mut values).unwrap(), 2);
    assert_eq!(virtual_key(values[0].take().unwrap()), 11);
    assert_eq!(virtual_key(values[1].take().unwrap()), 12);
    assert!(values[2].is_none());
    assert_eq!(vector.GetMany(3, &mut values).unwrap(), 0);

    let view = vector.GetView().unwrap();
    assert_eq!(view.Size().unwrap(), 3);
    assert_eq!(virtual_key(view.GetAt(1).unwrap()), 11);

    let iterator = vector.First().unwrap();
    assert!(iterator.HasCurrent().unwrap());
    assert_eq!(virtual_key(iterator.Current().unwrap()), 10);
    assert!(iterator.MoveNext().unwrap());
    assert_eq!(virtual_key(iterator.Current().unwrap()), 11);
    let mut tail = vec![None; 3];
    assert_eq!(iterator.GetMany(&mut tail).unwrap(), 2);
    assert_eq!(virtual_key(tail[0].take().unwrap()), 11);
    assert_eq!(virtual_key(tail[1].take().unwrap()), 12);
    assert!(!iterator.HasCurrent().unwrap());
    assert_eq!(iterator.Current().unwrap_err().code(), E_BOUNDS);
}

#[test]
fn native_recycling_cancels_pending_keyed_realization() {
    let queue = Rc::new(NativeEventQueue::default());
    let collection = ObjectId::test(1);
    let container = RealizedContainer(2);
    queue.queue_realization(RealizationRequest::RealizeKey {
        collection,
        container,
        index: 4,
        token: 9,
    });
    queue.queue_realization(RealizationRequest::Recycle {
        collection,
        container,
        source_revision: 3,
    });
    let events = queue.events.borrow();
    assert_eq!(events.len(), 1);
    assert!(matches!(
        events.front(),
        Some(NativeEvent::Realization(RealizationRequest::Cancel {
            collection: owner,
            container: recycled,
            source_revision: 3,
        })) if *owner == collection && *recycled == container
    ));
}

#[test]
fn virtual_source_is_read_only_and_validates_index_limits() {
    let max_count = i32::MAX as usize + 1;
    let source = ComObject::new(NativeVirtualSource::new(Arc::new(vec![0])).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();

    assert_eq!(virtual_item_count(max_count).unwrap(), max_count as u32);
    assert!(virtual_item_count(max_count + 1).is_err());

    let value = virtual_item_value(0);
    assert_eq!(
        vector.SetAt(0, &value).unwrap_err().code(),
        E_ILLEGAL_METHOD_CALL
    );
    assert_eq!(
        vector.Append(&value).unwrap_err().code(),
        E_ILLEGAL_METHOD_CALL
    );
    assert_eq!(vector.Clear().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
    assert_eq!(
        vector.ReplaceAll(&[Some(value)]).unwrap_err().code(),
        E_ILLEGAL_METHOD_CALL
    );
}

#[test]
fn virtual_source_reset_notifies_once_and_honors_event_removal() {
    let source = ComObject::new(NativeVirtualSource::new(Arc::default()).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();
    let first = Arc::new(AtomicUsize::new(0));
    let second = Arc::new(AtomicUsize::new(0));
    let first_observed = Arc::clone(&first);
    let first_revoker = vector
        .VectorChanged(move |_, args| {
            let args = args.ok().unwrap();
            assert_eq!(args.CollectionChange().unwrap(), CollectionChange::Reset);
            assert_eq!(args.Index().unwrap(), 0);
            first_observed.fetch_add(1, Ordering::Relaxed);
        })
        .unwrap();
    let second_observed = Arc::clone(&second);
    let second_revoker = vector
        .VectorChanged(move |_, _| {
            second_observed.fetch_add(1, Ordering::Relaxed);
        })
        .unwrap();

    let keys = Arc::new((0..10_000).collect::<Vec<_>>());
    source.set_keys(keys).unwrap();
    assert_eq!(vector.Size().unwrap(), 10_000);
    assert_eq!(first.load(Ordering::Relaxed), 1);
    assert_eq!(second.load(Ordering::Relaxed), 1);

    drop(first_revoker);
    source
        .set_keys(Arc::new((0..10_000).rev().collect()))
        .unwrap();
    assert_eq!(first.load(Ordering::Relaxed), 1);
    assert_eq!(second.load(Ordering::Relaxed), 2);

    drop(second_revoker);
    source.set_keys(Arc::default()).unwrap();
    assert_eq!(vector.Size().unwrap(), 0);
    assert_eq!(first.load(Ordering::Relaxed), 1);
    assert_eq!(second.load(Ordering::Relaxed), 2);
}

#[test]
fn virtual_source_reports_ordered_changes_individually() {
    let source = ComObject::new(NativeVirtualSource::new(Arc::new(vec![1, 2, 3])).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();
    let changes = Arc::new(Mutex::new(Vec::new()));
    let observed = Arc::clone(&changes);
    let _revoker = vector
        .VectorChanged(move |sender, args| {
            let args = args.ok().unwrap();
            let size = sender.ok().unwrap().Size().unwrap();
            observed.lock().unwrap().push((
                args.CollectionChange().unwrap(),
                args.Index().unwrap(),
                size,
            ));
        })
        .unwrap();

    source.set_keys(Arc::new(vec![0, 1, 3, 4])).unwrap();
    assert_eq!(
        std::mem::take(&mut *changes.lock().unwrap()),
        [
            (CollectionChange::ItemRemoved, 1, 2),
            (CollectionChange::ItemInserted, 0, 3),
            (CollectionChange::ItemInserted, 3, 4),
        ]
    );

    source.set_keys(Arc::new(vec![0, 1, 3, 4])).unwrap();
    assert!(changes.lock().unwrap().is_empty());

    source.set_keys(Arc::new(vec![1, 0, 3, 4])).unwrap();
    assert_eq!(
        std::mem::take(&mut *changes.lock().unwrap()),
        [(CollectionChange::Reset, 0, 4)]
    );

    source.set_keys(Arc::new(vec![5])).unwrap();
    assert_eq!(
        std::mem::take(&mut *changes.lock().unwrap()),
        [(CollectionChange::Reset, 0, 1)]
    );
}

#[test]
fn virtual_source_changes_replay_to_current_values() {
    fn values(vector: &IObservableVector<IInspectable>) -> Vec<u64> {
        (0..vector.Size().unwrap())
            .map(|index| virtual_key(vector.GetAt(index).unwrap()))
            .collect()
    }

    let source = ComObject::new(NativeVirtualSource::new(Arc::new(vec![10, 20, 30, 40])).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();
    let replica = Arc::new(Mutex::new(values(&vector)));
    let observed = Arc::clone(&replica);
    let _revoker = vector
        .VectorChanged(move |sender, args| {
            let sender = sender.ok().unwrap();
            let args = args.ok().unwrap();
            let index = args.Index().unwrap();
            let mut replica = observed.lock().unwrap();
            match args.CollectionChange().unwrap() {
                CollectionChange::ItemRemoved => {
                    let removed = replica.remove(index as usize);
                    let mut position = 0;
                    assert!(
                        !sender
                            .IndexOf(&virtual_item_value(removed), &mut position)
                            .unwrap()
                    );
                }
                CollectionChange::ItemInserted => {
                    replica.insert(index as usize, virtual_key(sender.GetAt(index).unwrap()));
                }
                change => panic!("unexpected {change:?}"),
            }
            assert_eq!(*replica, values(sender));
            for (expected, &key) in replica.iter().enumerate() {
                let mut position = u32::MAX;
                assert!(
                    sender
                        .IndexOf(&virtual_item_value(key), &mut position)
                        .unwrap()
                );
                assert_eq!(position as usize, expected);
            }
        })
        .unwrap();

    for keys in [
        vec![5, 10, 30, 35, 40, 45],
        vec![30, 45],
        vec![1, 2, 30, 3, 45, 4],
    ] {
        source.set_keys(Arc::new(keys)).unwrap();
        let after = values(&vector);
        assert_eq!(*replica.lock().unwrap(), after);
        for position in 0..vector.Size().unwrap() {
            let mut index = u32::MAX;
            assert!(
                vector
                    .IndexOf(&vector.GetAt(position).unwrap(), &mut index)
                    .unwrap()
            );
            assert_eq!(index, position);
        }
    }
}

#[test]
fn virtual_source_continues_from_partially_reported_change() {
    use crate::native::bindings::E_FAIL;
    let source = ComObject::new(NativeVirtualSource::new(Arc::new(vec![1, 2, 3])).unwrap());
    let vector: IObservableVector<IInspectable> = source.to_interface();
    let failing = VectorChangedEventHandler::<IInspectable>::new(|_, _| {
        Err(windows_core::Error::new(E_FAIL, "failed"))
    });
    let token = source
        .VectorChanged(<Ref<VectorChangedEventHandler<IInspectable>>>::from(
            &failing,
        ))
        .unwrap();
    assert_eq!(
        source.set_keys(Arc::new(vec![1, 3, 4])).unwrap_err().code(),
        E_FAIL
    );
    source.RemoveVectorChanged(token).unwrap();

    let changes = Arc::new(Mutex::new(Vec::new()));
    let observed = Arc::clone(&changes);
    let _revoker = vector
        .VectorChanged(move |_, args| {
            let args = args.ok().unwrap();
            observed
                .lock()
                .unwrap()
                .push((args.CollectionChange().unwrap(), args.Index().unwrap()));
        })
        .unwrap();
    source.set_keys(Arc::new(vec![1, 3, 4])).unwrap();
    assert_eq!(
        *changes.lock().unwrap(),
        [(CollectionChange::ItemInserted, 2)]
    );
    let values: Vec<u64> = (0..vector.Size().unwrap())
        .map(|index| virtual_key(vector.GetAt(index).unwrap()))
        .collect();
    assert_eq!(values, [1, 3, 4]);
}

#[test]
fn virtual_source_reset_returns_handler_failure() {
    use crate::native::bindings::E_FAIL;
    let source = ComObject::new(NativeVirtualSource::new(Arc::default()).unwrap());
    let failing = VectorChangedEventHandler::<IInspectable>::new(|_, _| {
        Err(windows_core::Error::new(E_FAIL, "not unique"))
    });
    let observed = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&observed);
    let counting = VectorChangedEventHandler::<IInspectable>::new(move |_, _| {
        counted.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    source
        .VectorChanged(<Ref<VectorChangedEventHandler<IInspectable>>>::from(
            &failing,
        ))
        .unwrap();
    source
        .VectorChanged(<Ref<VectorChangedEventHandler<IInspectable>>>::from(
            &counting,
        ))
        .unwrap();

    let error = source.set_keys(Arc::new(vec![0])).unwrap_err();
    assert_eq!(error.code(), E_FAIL);
    assert_eq!(observed.load(Ordering::Relaxed), 1);
}

#[test]
fn virtual_source_maps_native_tokens_across_reset() {
    use native::IKeyIndexMapping_Impl;

    let source = ComObject::new(NativeVirtualSource::new(Arc::new(vec![0, 1, 2])).unwrap());
    let mapping = &*source;
    let tokens = [0, 1, 2].map(|index| mapping.KeyFromIndex(index).unwrap());
    assert_ne!(tokens[0], tokens[1]);
    assert_ne!(tokens[1], tokens[2]);
    assert_eq!(mapping.KeyFromIndex(-1).unwrap_err().code(), E_BOUNDS);
    assert_eq!(mapping.KeyFromIndex(3).unwrap_err().code(), E_BOUNDS);
    for missing in ["", "unknown", "+0", "00"] {
        assert_eq!(mapping.IndexFromKey(&HSTRING::from(missing)).unwrap(), -1);
    }

    source.set_keys(Arc::new(vec![2, 0])).unwrap();
    assert_eq!(mapping.KeyFromIndex(0).unwrap(), tokens[2]);
    assert_eq!(mapping.KeyFromIndex(1).unwrap(), tokens[0]);
    assert_eq!(mapping.IndexFromKey(&tokens[2]).unwrap(), 0);
    assert_eq!(mapping.IndexFromKey(&tokens[0]).unwrap(), 1);
    assert_eq!(mapping.IndexFromKey(&tokens[1]).unwrap(), -1);

    source.set_keys(Arc::new(vec![0, 3, 2])).unwrap();
    assert_eq!(mapping.KeyFromIndex(0).unwrap(), tokens[0]);
    assert_eq!(mapping.KeyFromIndex(2).unwrap(), tokens[2]);
    assert_ne!(mapping.KeyFromIndex(1).unwrap(), tokens[1]);
    assert_eq!(mapping.IndexFromKey(&tokens[1]).unwrap(), -1);
}

#[test]
fn policy_preserves_requested_window_state() {
    let policy = WindowPolicy::new()
        .title("Solitaire")
        .theme(WindowTheme::Dark)
        .client_size(800.0, 600.0)
        .minimum_client_size(800.0, 600.0);

    assert_eq!(policy.title.as_deref(), Some("Solitaire"));
    assert_eq!(policy.theme, WindowTheme::Dark);
    assert_eq!(policy.client_size, Some((800.0, 600.0)));
    assert_eq!(policy.minimum_client_size, Some((800.0, 600.0)));
}

#[test]
fn policy_rejects_invalid_window_sizes() {
    for (width, height) in [
        (0.0, 1.0),
        (1.0, -1.0),
        (f64::NAN, 1.0),
        (1.0, f64::INFINITY),
    ] {
        assert!(
            std::panic::catch_unwind(|| WindowPolicy::new().client_size(width, height)).is_err()
        );
        assert!(
            std::panic::catch_unwind(|| { WindowPolicy::new().minimum_client_size(width, height) })
                .is_err()
        );
    }
}

#[test]
fn window_visual_changes_only_apply_changed_contracts() {
    let previous = WindowVisuals::new()
        .theme(WindowTheme::System)
        .backdrop(WindowBackdrop::Mica)
        .client_size(800.0, 600.0);
    let next = WindowVisuals::new()
        .theme(WindowTheme::Dark)
        .backdrop(WindowBackdrop::Mica)
        .client_size(800.0, 600.0);

    let changes = window_visual_changes(&previous, &next);
    assert!(!changes.backdrop);
    assert!(!changes.client_size);
    assert!(!changes.constraints);
    assert!(!changes.icon);
    assert!(changes.theme);
    assert!(changes.any());
    assert!(!window_visual_changes(&next, &next).any());
}

#[test]
fn window_presentation_changes_reset_to_defaults() {
    let restricted = WindowVisuals::new()
        .always_on_top(true)
        .minimizable(false)
        .maximizable(false);
    for (previous, next) in [
        (WindowVisuals::new(), restricted.clone()),
        (restricted, WindowVisuals::new()),
    ] {
        let changes = window_visual_changes(&previous, &next);
        assert!(changes.always_on_top);
        assert!(changes.minimizable);
        assert!(changes.maximizable);
        assert!(changes.any());
        assert!(!changes.client_size);
    }
    assert!(
        !window_visual_changes(
            &WindowVisuals::new(),
            &WindowVisuals::new().minimizable(true).maximizable(true)
        )
        .any()
    );
}

#[test]
fn window_visual_changes_detect_icon_removal() {
    let previous = WindowVisuals::new().icon("test.ico");
    let changes = window_visual_changes(&previous, &WindowVisuals::new());
    assert!(changes.icon);
}

#[test]
fn queued_occurrences_follow_their_subscription() {
    let object = ObjectId::test(1);
    let first = Callback::new(|_: ()| {});
    let second = Callback::new(|_: ()| {});
    let third = Callback::new(|_: ()| {});
    let event = Rc::new(RefCell::new(NativeUnitEvent::default()));
    let mut adapter = WinUiAdapter::default();
    let queue = |adapter: &WinUiAdapter| {
        WinUiAdapter::dispatch(
            &event,
            &adapter.event_queue,
            object,
            EventId::Click,
            None,
            (),
        );
    };
    let delivered = |callback: &Callback<()>| {
        Some(NativeEvent::Event {
            observation: None,
            event: EventDispatch::new(object, EventId::Click, EventCall::Unit(callback.clone())),
        })
    };

    // Replacing the callback keeps the subscription, so the occurrence reaches the callback
    // subscribed when it fired.
    event.borrow_mut().set(Some(first.clone()));
    queue(&adapter);
    event.borrow_mut().set(Some(second));
    assert_eq!(adapter.pop_native_event(), delivered(&first));

    // Removing the event ends the subscription even if it is re-added before delivery.
    queue(&adapter);
    event.borrow_mut().set(None);
    event.borrow_mut().set(Some(third.clone()));
    assert_eq!(adapter.pop_native_event(), None);

    queue(&adapter);
    assert_eq!(adapter.pop_native_event(), delivered(&third));

    // An occurrence completing asynchronously captures its handler when the event fires.
    let pending = event.borrow().handler(()).unwrap();
    event.borrow_mut().set(None);
    assert!(matches!(pending, QueuedHandler::Callback { live, .. } if !live.get()));
}

#[test]
fn pointer_mode_replacement_preserves_queued_occurrences() {
    let object = ObjectId::test(1);
    let event = Rc::new(RefCell::new(NativePointerEventInfoEvent::default()));
    let mut adapter = WinUiAdapter::default();
    let first = Callback::new(|_| {});
    let second = Callback::new(|_| {});
    let routed = RoutedCallback::new(|_| false);
    let payload = PointerEventInfo {
        pointer_id: 42,
        ..Default::default()
    };
    let queue = |adapter: &WinUiAdapter| {
        assert!(!WinUiAdapter::dispatch_pointer(
            &event,
            &adapter.event_queue,
            object,
            EventId::PointerPressed,
            payload,
        ));
    };
    let delivered = |callback: &Callback<PointerEventInfo>| {
        Some(NativeEvent::Event {
            observation: None,
            event: EventDispatch::new(
                object,
                EventId::PointerPressed,
                EventCall::PointerEventInfo(callback.clone(), payload),
            ),
        })
    };

    event.borrow_mut().set(Some(first.clone().into()));
    queue(&adapter);
    event.borrow_mut().set(Some(routed.clone().into()));
    assert_eq!(adapter.pop_native_event(), delivered(&first));

    event.borrow_mut().set(Some(first.clone().into()));
    queue(&adapter);
    event.borrow_mut().set(Some(routed.clone().into()));
    event
        .borrow_mut()
        .set(Some(RoutedCallback::new(|_| true).into()));
    event.borrow_mut().set(Some(second.clone().into()));
    queue(&adapter);
    assert_eq!(adapter.pop_native_event(), delivered(&first));
    assert_eq!(adapter.pop_native_event(), delivered(&second));
    assert_eq!(adapter.pop_native_event(), None);

    event.borrow_mut().set(Some(first.into()));
    queue(&adapter);
    event.borrow_mut().set(Some(routed.into()));
    event.borrow_mut().set(None);
    event.borrow_mut().set(Some(second.clone().into()));
    assert_eq!(adapter.pop_native_event(), None);
    queue(&adapter);
    assert_eq!(adapter.pop_native_event(), delivered(&second));

    queue(&adapter);
    event
        .borrow_mut()
        .set(Some(RoutedCallback::new(|_| false).into()));
    adapter.event_queue.discard_object(object);
    assert_eq!(adapter.pop_native_event(), None);
}

#[test]
fn routed_pointer_captures_before_deferral_and_allows_replacement() {
    let object = ObjectId::test(1);
    let event = Rc::new(RefCell::new(NativePointerEventInfoEvent::default()));
    let mut adapter = WinUiAdapter::default();
    let frame = Rc::new(Cell::new(7));
    let captured = Rc::new(Cell::new(None));
    let dispatch = |adapter: &WinUiAdapter| {
        WinUiAdapter::dispatch_pointer(
            &event,
            &adapter.event_queue,
            object,
            EventId::PointerPressed,
            PointerEventInfo::default(),
        )
    };
    event.borrow_mut().set(Some(Callback::new(|_| {}).into()));
    assert!(!dispatch(&adapter));

    let weak = Rc::downgrade(&event);
    let current = Rc::clone(&frame);
    let result = Rc::clone(&captured);
    event.borrow_mut().set(Some(
        RoutedCallback::new(move |_| {
            result.set(Some(current.get()));
            weak.upgrade().unwrap().borrow_mut().set(None);
            true
        })
        .into(),
    ));
    assert!(dispatch(&adapter));
    frame.set(8);
    assert_eq!(captured.get(), Some(7));
    assert!(adapter.pop_native_event().is_none());
    assert!(!dispatch(&adapter));

    event
        .borrow_mut()
        .set(Some(RoutedCallback::new(|_| false).into()));
    assert!(!dispatch(&adapter));
    assert!(adapter.pop_native_event().is_none());
    event.borrow_mut().set(Some(Callback::new(|_| {}).into()));
    assert!(!dispatch(&adapter));
    assert!(adapter.pop_native_event().is_some());
}

#[test]
fn pointer_event_modifiers_map_native_flags() {
    let mut expected = InputModifiers::SHIFT;
    expected |= InputModifiers::CONTROL;
    expected |= InputModifiers::ALT;
    expected |= InputModifiers::WINDOWS;

    assert_eq!(
        WinUiAdapter::input_modifiers_from_virtual_keys(
            native::VirtualKeyModifiers::Shift
                | native::VirtualKeyModifiers::Control
                | native::VirtualKeyModifiers::Menu
                | native::VirtualKeyModifiers::Windows
        ),
        expected
    );
    assert_eq!(
        WinUiAdapter::input_modifiers_from_virtual_keys(native::VirtualKeyModifiers::None),
        InputModifiers::NONE
    );
}

#[test]
fn optional_numeric_adapters_use_distinct_sentinels() {
    assert!(native_number_box_value(None).is_nan());
    assert_eq!(number_box_value(f64::NAN), None);
    assert_eq!(number_box_value(-1.0), Some(-1.0));

    assert_eq!(native_rating_value(None), -1.0);
    assert_eq!(rating_value(-1.0), None);
    assert!(rating_value(f64::NAN).unwrap().is_nan());
}

#[test]
fn selection_index_conversion_reports_overflow() {
    assert_eq!(native_selection_index(None), Ok(-1));
    assert_eq!(
        native_selection_index(Some(i32::MAX as usize)),
        Ok(i32::MAX)
    );
    let overflow = i32::MAX as usize + 1;
    assert_eq!(
        native_selection_index(Some(overflow)),
        Err(WinUiError::IndexOverflow(overflow))
    );
}

#[test]
fn generated_feedback_pairs_written_properties_with_their_reporting_event() {
    let cases = [
        (
            ObjectType::ToggleSwitch,
            PropertyId::IsOn,
            Some(EventId::Toggled),
        ),
        (
            ObjectType::Slider,
            PropertyId::Value,
            Some(EventId::ValueChanged),
        ),
        (
            ObjectType::Slider,
            PropertyId::Maximum,
            Some(EventId::ValueChanged),
        ),
        (
            ObjectType::RichEditBox,
            PropertyId::Document,
            Some(EventId::TextChanged),
        ),
        (
            ObjectType::NavigationView,
            PropertyId::IsPaneOpen,
            Some(EventId::IsPaneOpenChanged),
        ),
        (ObjectType::ProgressBar, PropertyId::Value, None),
    ];
    for (kind, property, event) in cases {
        assert_eq!(reporting_event(kind, property), event);
    }
}
