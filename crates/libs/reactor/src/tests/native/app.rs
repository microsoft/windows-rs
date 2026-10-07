use super::*;

#[test]
fn initial_publication_is_taken_without_dispatching_other_window_work() {
    let services = LiveWindowServices::default();
    services.active.set(true);
    assert!(services.push(WindowWork::Close(1)));
    assert!(services.publish(
        WindowPublication {
            title: Some("old".into()),
            ..Default::default()
        },
        1,
    ));
    assert!(services.publish(WindowPublication::default(), 2));
    assert!(services.publish(
        WindowPublication {
            title: Some("initial".into()),
            ..Default::default()
        },
        1,
    ));
    assert_eq!(
        services.take_publication(1).unwrap().title.as_deref(),
        Some("initial")
    );
    assert!(services.take_publication(1).is_none());
    assert!(matches!(services.pop(), Some(WindowWork::Close(1))));
    assert!(matches!(
        services.pop(),
        Some(WindowWork::Publish { window: 2, .. })
    ));
    assert!(services.pop().is_none());
}

#[test]
fn placement_observations_coalesce_by_window_and_subscription() {
    let services = LiveWindowServices::default();
    services.active.set(true);
    for window in 0..WINDOW_WORK_CAPACITY {
        assert!(services.push(WindowWork::Activate(window as u64)));
    }
    let placement = WindowPlacement {
        x: 10,
        y: 20,
        width: 800,
        height: 600,
        maximized: false,
    };
    for x in 10..20 {
        assert!(services.placement(1, WindowPlacement { x, ..placement }, 1));
    }
    assert!(services.placement(2, placement, 1));
    assert!(services.placement(1, placement, 2));
    for _ in 0..WINDOW_WORK_CAPACITY {
        assert!(matches!(services.pop(), Some(WindowWork::Activate(_))));
    }
    assert!(matches!(
        services.pop(),
        Some(WindowWork::Placement {
            generation: 1,
            window: 1,
            placement: WindowPlacement { x: 19, .. },
        })
    ));
    assert!(matches!(
        services.pop(),
        Some(WindowWork::Placement {
            generation: 2,
            window: 1,
            ..
        })
    ));
    assert!(matches!(
        services.pop(),
        Some(WindowWork::Placement {
            generation: 1,
            window: 2,
            ..
        })
    ));
    assert!(services.pop().is_none());
}

#[test]
fn canceled_queued_app_callback_is_ignored() {
    let id = NEXT_APP_CALLBACK.fetch_add(1, Ordering::Relaxed);
    let invoked = Rc::new(Cell::new(false));
    let callback_invoked = Rc::clone(&invoked);
    APP_CALLBACKS.with(|callbacks| {
        callbacks.borrow_mut().insert(
            id,
            Rc::new(move || {
                callback_invoked.set(true);
                Ok(())
            }),
        );
        callbacks.borrow_mut().remove(&id);
    });

    invoke_registered_app_callback(id).unwrap();

    assert!(!invoked.get());
}

#[test]
fn component_window_lifecycle_serializes_close() {
    let mut lifecycle = ComponentWindowLifecycle::Open;
    assert!(lifecycle.begin_close());
    assert_eq!(lifecycle, ComponentWindowLifecycle::Closing);
    assert!(!lifecycle.begin_close());
    lifecycle.mark_closed();
    assert_eq!(lifecycle, ComponentWindowLifecycle::Closed);
    assert!(!lifecycle.begin_close());
}

#[test]
fn routine_window_work_is_coalesced_at_the_latest_position() {
    let services = LiveWindowServices::default();
    services.active.set(true);
    assert!(services.publish(
        WindowPublication {
            title: Some("first".to_string()),
            ..Default::default()
        },
        1,
    ));
    assert!(services.push(WindowWork::Activate(1)));
    assert!(services.publish(
        WindowPublication {
            title: Some("second".to_string()),
            ..Default::default()
        },
        1,
    ));
    assert!(services.size(
        7,
        WindowSize {
            width: 100.0,
            height: 100.0,
        },
        1,
    ));
    assert!(services.size(
        7,
        WindowSize {
            width: 200.0,
            height: 150.0,
        },
        1,
    ));

    assert!(matches!(services.pop(), Some(WindowWork::Activate(1))));
    assert!(matches!(
        services.pop(),
        Some(WindowWork::Publish {
            publication: WindowPublication {
                title: Some(title),
                ..
            },
            window: 1,
        }) if title == "second"
    ));
    assert!(matches!(
        services.pop(),
        Some(WindowWork::Size {
            generation: 7,
            size: WindowSize {
                width: 200.0,
                height: 150.0,
            },
            window: 1,
        })
    ));
    assert!(services.pop().is_none());
}

#[test]
fn durable_window_work_survives_routine_queue_saturation() {
    let services = LiveWindowServices::default();
    services.active.set(true);
    for window in 0..WINDOW_WORK_CAPACITY {
        assert!(services.push(WindowWork::Activate(window as u64)));
    }
    assert!(!services.push(WindowWork::Activate(WINDOW_WORK_CAPACITY as u64)));

    for title in ["first", "latest"] {
        assert!(services.publish(
            WindowPublication {
                title: Some(title.to_string()),
                ..Default::default()
            },
            7,
        ));
    }
    for width in [100.0, 200.0] {
        assert!(services.size(
            11,
            WindowSize {
                width,
                height: 150.0,
            },
            7,
        ));
    }
    for scheme in [ColorScheme::Light, ColorScheme::Dark] {
        assert!(services.color_scheme(11, scheme, 7));
    }

    assert_eq!(services.pending.borrow().len(), WINDOW_WORK_CAPACITY + 3);
    for _ in 0..WINDOW_WORK_CAPACITY {
        assert!(matches!(services.pop(), Some(WindowWork::Activate(_))));
    }
    assert!(matches!(
        services.pop(),
        Some(WindowWork::Publish {
            publication: WindowPublication {
                title: Some(title),
                ..
            },
            window: 7,
        }) if title == "latest"
    ));
    assert!(matches!(
        services.pop(),
        Some(WindowWork::Size {
            generation: 11,
            size: WindowSize { width: 200.0, .. },
            window: 7,
        })
    ));
    assert!(matches!(
        services.pop(),
        Some(WindowWork::ColorScheme {
            generation: 11,
            scheme: ColorScheme::Dark,
            window: 7,
        })
    ));
    assert!(services.pop().is_none());
}
