use super::*;
use crate::{StackPanel, TextBlock};

struct ObserverUiServices(Rc<LiveWindowServices>);

impl ComponentUiServices for ObserverUiServices {
    fn open_window(&self, _: ComponentNode, _: WindowPolicy) -> bool {
        false
    }

    fn publish_window(&self, publication: WindowPublication) {
        assert!(self.0.publish(publication, 1));
    }
}

struct ObserverComponent([bool; 3]);

impl Component for ObserverComponent {
    type Input = [bool; 3];
    type Message = [bool; 3];

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self(*input)
    }

    fn update(&mut self, subscribed: Self::Message, _: &ComponentContext<Self>) {
        self.0 = subscribed;
    }

    fn view(&self, _: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if self.0[0] {
            context.on_color_scheme(Callback::new(|_| {}));
        }
        if self.0[1] {
            context.on_window_size(Callback::new(|_| {}));
        }
        if self.0[2] {
            context.on_window_placement(Callback::new(|_| {}));
        }
        TextBlock::new().into()
    }
}

struct ObserverParent(bool);

impl Component for ObserverParent {
    type Input = ();
    type Message = bool;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, second: bool, _: &ComponentContext<Self>) {
        self.0 = second;
    }

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .keyed_children([keyed(
                if self.0 { "second" } else { "first" },
                View::component::<ObserverComponent>([true; 3]),
            )])
            .into()
    }
}

fn observer_host<C: Component>(
    input: C::Input,
) -> (ComponentHost<RecordingAdapter>, Rc<LiveWindowServices>) {
    let services = Rc::new(LiveWindowServices::default());
    services.active.set(true);
    let host = ComponentHost::mount_with_all_services(
        RecordingAdapter::default(),
        Arc::new(DefaultComponentServices),
        Rc::new(ObserverUiServices(Rc::clone(&services))),
        [component::<C>("root", input)],
    )
    .unwrap();
    (host, services)
}

fn queue_observer_notifications(services: &LiveWindowServices, publication: &WindowPublication) {
    assert!(services.color_scheme(
        publication.on_color_scheme.as_ref().unwrap().generation,
        ColorScheme::Dark,
        1,
    ));
    assert!(services.size(
        publication.on_size.as_ref().unwrap().generation,
        WindowSize {
            width: 100.0,
            height: 100.0,
        },
        1,
    ));
    assert!(services.placement(
        publication.on_placement.as_ref().unwrap().generation,
        WindowPlacement {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            maximized: false,
        },
        1,
    ));
}

fn assert_queued_observer_delivery(
    host: &ComponentHost<RecordingAdapter>,
    services: &LiveWindowServices,
    expected: [bool; 3],
) -> WindowPublication {
    let mut publication = None;
    let mut notifications = [0; 3];
    while let Some(work) = services.pop() {
        let (index, delivered) = match work {
            WindowWork::Publish {
                publication: next, ..
            } => {
                assert!(publication.replace(next).is_none());
                continue;
            }
            WindowWork::ColorScheme {
                generation, scheme, ..
            } => (
                0,
                host.window_color_scheme_callback(generation)
                    .map(|callback| callback.call(scheme))
                    .is_some(),
            ),
            WindowWork::Size {
                generation, size, ..
            } => (
                1,
                host.window_size_callback(generation)
                    .map(|callback| callback.call(size))
                    .is_some(),
            ),
            WindowWork::Placement {
                generation,
                placement,
                ..
            } => (
                2,
                host.window_placement_callback(generation)
                    .map(|callback| callback.call(placement))
                    .is_some(),
            ),
            _ => panic!("unexpected observer work"),
        };
        assert_eq!(delivered, expected[index], "observer {index}");
        notifications[index] += 1;
    }
    assert_eq!(notifications, [1; 3]);
    publication.unwrap()
}

fn assert_observer_unchanged<T>(
    first: &WindowObserverDeclaration<T>,
    current: &WindowObserverDeclaration<T>,
) {
    assert_eq!(first.owner, current.owner);
    assert_eq!(first.generation, current.generation);
    assert_ne!(first.callback, current.callback);
}

#[test]
fn committed_removal_rejects_observations_in_both_queue_orders() {
    for queued_before_commit in [true, false] {
        let (mut host, services) = observer_host::<ObserverComponent>([true; 3]);
        let first = services.take_publication(1).unwrap();
        if queued_before_commit {
            queue_observer_notifications(&services, &first);
        }
        let sender = host
            .sender::<ObserverComponent>(&Key::from("root"))
            .unwrap();
        assert!(sender.send([false; 3]));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        if !queued_before_commit {
            queue_observer_notifications(&services, &first);
        }
        assert_eq!(services.pending.borrow().len(), 4);
        assert_eq!(
            matches!(
                services.pending.borrow().front(),
                Some(WindowWork::Publish { .. })
            ),
            !queued_before_commit,
        );
        let current = assert_queued_observer_delivery(&host, &services, [false; 3]);
        assert!(current.on_color_scheme.is_none());
        assert!(current.on_size.is_none());
        assert!(current.on_placement.is_none());
    }
}

#[test]
fn observer_lifetimes_survive_coalesced_removal_and_resubscription() {
    for queued_before_commit in [true, false] {
        let (mut host, services) = observer_host::<ObserverComponent>([true; 3]);
        let first = services.take_publication(1).unwrap();
        if queued_before_commit {
            queue_observer_notifications(&services, &first);
        }
        let sender = host
            .sender::<ObserverComponent>(&Key::from("root"))
            .unwrap();
        assert!(sender.send([false; 3]));
        assert!(sender.send([true; 3]));
        assert_eq!(host.drain(2).unwrap().dispatched, 2);
        if !queued_before_commit {
            queue_observer_notifications(&services, &first);
        }
        assert_eq!(services.pending.borrow().len(), 4);
        let current = assert_queued_observer_delivery(&host, &services, [false; 3]);
        queue_observer_notifications(&services, &current);
        assert!(services.publish(current, 1));
        assert_queued_observer_delivery(&host, &services, [true; 3]);
    }
}

#[test]
fn observer_lifetimes_change_when_the_subscribing_component_is_replaced() {
    let (mut host, services) = observer_host::<ObserverParent>(());
    let first = services.take_publication(1).unwrap();
    queue_observer_notifications(&services, &first);
    let sender = host.sender::<ObserverParent>(&Key::from("root")).unwrap();
    assert!(sender.send(true));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    let current = assert_queued_observer_delivery(&host, &services, [false; 3]);
    assert_ne!(
        first.on_color_scheme.as_ref().unwrap().owner,
        current.on_color_scheme.as_ref().unwrap().owner,
    );
    queue_observer_notifications(&services, &current);
    assert!(services.publish(current, 1));
    assert_queued_observer_delivery(&host, &services, [true; 3]);
}

#[test]
fn committed_callback_replacements_preserve_observer_lifetimes() {
    let (mut host, services) = observer_host::<ObserverComponent>([true; 3]);
    let first = services.take_publication(1).unwrap();
    queue_observer_notifications(&services, &first);
    let sender = host
        .sender::<ObserverComponent>(&Key::from("root"))
        .unwrap();
    assert!(sender.send([true; 3]));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    let current = assert_queued_observer_delivery(&host, &services, [true; 3]);
    let scheme = current.on_color_scheme.as_ref().unwrap();
    let size = current.on_size.as_ref().unwrap();
    let placement = current.on_placement.as_ref().unwrap();
    assert_observer_unchanged(first.on_color_scheme.as_ref().unwrap(), scheme);
    assert_observer_unchanged(first.on_size.as_ref().unwrap(), size);
    assert_observer_unchanged(first.on_placement.as_ref().unwrap(), placement);
    assert_eq!(
        host.window_color_scheme_callback(scheme.generation),
        Some(scheme.callback.clone())
    );
    assert_eq!(
        host.window_size_callback(size.generation),
        Some(size.callback.clone())
    );
    assert_eq!(
        host.window_placement_callback(placement.generation),
        Some(placement.callback.clone())
    );
}

#[test]
fn changing_one_committed_observer_preserves_other_queued_observations() {
    let (mut host, services) = observer_host::<ObserverComponent>([true; 3]);
    let first = services.take_publication(1).unwrap();
    queue_observer_notifications(&services, &first);
    let sender = host
        .sender::<ObserverComponent>(&Key::from("root"))
        .unwrap();
    assert!(sender.send([false, true, true]));
    assert!(sender.send([true; 3]));
    assert_eq!(host.drain(2).unwrap().dispatched, 2);
    let current = assert_queued_observer_delivery(&host, &services, [false, true, true]);
    assert_ne!(
        first.on_color_scheme.as_ref().unwrap().generation,
        current.on_color_scheme.as_ref().unwrap().generation,
    );
    assert_observer_unchanged(
        first.on_size.as_ref().unwrap(),
        current.on_size.as_ref().unwrap(),
    );
    assert_observer_unchanged(
        first.on_placement.as_ref().unwrap(),
        current.on_placement.as_ref().unwrap(),
    );
}

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
