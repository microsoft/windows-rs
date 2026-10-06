use super::*;

#[derive(Clone)]
struct RootSwitchEffects(Rc<RefCell<Vec<(&'static str, Option<ObjectId>)>>>);

impl PartialEq for RootSwitchEffects {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

struct NestedEffects;

impl Component for NestedEffects {
    type Input = (RootSwitchEffects, bool);
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let root = Border::new();
        if input.1 {
            root.content(component::<EffectRootSwitch>("effect", input.0.clone()))
                .into()
        } else {
            root.into()
        }
    }
}

struct EffectRootSwitch(bool);

impl Component for EffectRootSwitch {
    type Input = RootSwitchEffects;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 = true;
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let events = Rc::clone(&input.0);
        let reference = context.root();
        context.use_effect("root", self.0, move || {
            events.borrow_mut().push(("setup", reference.get()));
            Some(Box::new(move || {
                events.borrow_mut().push(("cleanup", reference.get()));
            }))
        });
        if self.0 {
            Border::new().into()
        } else {
            TextBlock::new().text("Stable").into()
        }
    }
}

struct InvalidEffectUpdate(bool);

impl Component for InvalidEffectUpdate {
    type Input = RootSwitchEffects;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 = true;
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let events = Rc::clone(&input.0);
        context.use_effect("root", self.0, move || {
            events.borrow_mut().push(("setup", None));
            Some(Box::new(move || {
                events.borrow_mut().push(("cleanup", None));
            }))
        });
        if self.0 {
            Grid::new()
                .keyed_children([
                    keyed("duplicate", TextBlock::new().text("First")),
                    keyed("duplicate", TextBlock::new().text("Second")),
                ])
                .into()
        } else {
            TextBlock::new().text("Valid").into()
        }
    }
}

struct CallbackRenderFailure(bool);

impl Component for CallbackRenderFailure {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 = true;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if self.0 {
            Grid::new()
                .keyed_children([
                    keyed("duplicate", TextBlock::new().text("first")),
                    keyed("duplicate", TextBlock::new().text("second")),
                ])
                .into()
        } else {
            Button::new().on_click(context.message(())).into()
        }
    }
}

#[test]
fn retired_component_drops_effects_and_async_completion() {
    let cleanup = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<Counter>(
            "counter",
            CounterInput {
                cleanup: Arc::clone(&cleanup),
                value: 0,
            },
        )],
    )
    .unwrap();
    let reference = host.reference(&Key::from("counter")).unwrap();
    let completion = host
        .sender::<Counter>(&Key::from("counter"))
        .unwrap()
        .completion();

    host.remove(&Key::from("counter")).unwrap();
    assert!(
        std::thread::spawn(move || completion.complete(1))
            .join()
            .unwrap()
    );
    let report = host.drain(1).unwrap();

    assert_eq!(reference.get(), None);
    assert_eq!(cleanup.load(Ordering::Relaxed), 1);
    assert_eq!(report.dropped, 1);
    assert_eq!(report.dispatched, 0);
}

#[test]
fn exit_retirement_does_not_retain_component_or_effect_ownership() {
    #[derive(Clone)]
    struct Input(Arc<AtomicUsize>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.0, &other.0)
        }
    }

    struct Exiting;

    impl Component for Exiting {
        type Input = Input;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let cleanup = Arc::clone(&input.0);
            context.use_effect("cleanup", (), move || {
                Some(Box::new(move || {
                    cleanup.fetch_add(1, Ordering::Relaxed);
                }))
            });
            Button::new().exit_fade(Duration::from_millis(200)).into()
        }
    }

    let cleanup = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<Exiting>("exiting", Input(Arc::clone(&cleanup)))],
    )
    .unwrap();
    let reference = host.reference(&Key::from("exiting")).unwrap();
    let root = reference.get().unwrap();

    host.remove(&Key::from("exiting")).unwrap();
    assert_eq!(reference.get(), None);
    assert_eq!(cleanup.load(Ordering::Relaxed), 1);
    assert!(host.sender::<Exiting>(&Key::from("exiting")).is_none());
    assert_eq!(host.runtime().graph().retired_count(), 1);
    assert_eq!(host.runtime().adapter().retirement_count(), 1);

    assert!(host.runtime.adapter_mut().complete_retirement(root));
    host.drain(1).unwrap();
    assert_eq!(host.runtime().graph().retired_count(), 0);
    assert_eq!(host.runtime().adapter().retirement_count(), 0);
}

#[test]
fn root_effect_cleanup_precedes_replacement_and_setup_follows_it() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<EffectRootSwitch>(
            "switch",
            RootSwitchEffects(Rc::clone(&events)),
        )],
    )
    .unwrap();
    let sender = host
        .sender::<EffectRootSwitch>(&Key::from("switch"))
        .unwrap();
    let reference = host.reference(&Key::from("switch")).unwrap();
    let previous = reference.get();

    assert!(sender.send(()));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    let next = reference.get();

    assert_eq!(previous, next);
    assert_eq!(
        events.borrow().as_slice(),
        [("setup", previous), ("cleanup", previous), ("setup", next)]
    );
}

#[test]
fn nested_effect_cleanup_sees_root_before_removal() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let input = RootSwitchEffects(Rc::clone(&events));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<NestedEffects>("parent", (input.clone(), true))],
    )
    .unwrap();
    let path = [Key::from("parent"), Key::from("effect")];
    let reference = host.reference_at(&path).unwrap();
    let root = reference.get();
    assert_eq!(events.borrow().as_slice(), [("setup", root)]);

    host.update_input::<NestedEffects>(&Key::from("parent"), (input, false))
        .unwrap();
    assert_eq!(
        events.borrow().as_slice(),
        [("setup", root), ("cleanup", root)]
    );
    assert_eq!(reference.get(), None);
}

#[test]
fn invalid_update_poisons_host_and_cleans_active_effects() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<InvalidEffectUpdate>(
            "invalid",
            RootSwitchEffects(Rc::clone(&events)),
        )],
    )
    .unwrap();
    let sender = host
        .sender::<InvalidEffectUpdate>(&Key::from("invalid"))
        .unwrap();
    let wakes = Arc::new(AtomicUsize::new(0));
    let callback_wakes = Arc::clone(&wakes);
    host.set_waker(move || {
        callback_wakes.fetch_add(1, Ordering::Relaxed);
    });

    assert!(sender.send(()));
    assert!(sender.send(()));
    assert_eq!(wakes.load(Ordering::Relaxed), 1);
    assert!(matches!(
        host.drain(1),
        Err(ComponentError::Runtime(UpdateError::Graph(
            GraphError::DuplicateKey(_)
        )))
    ));
    assert_eq!(wakes.load(Ordering::Relaxed), 1);
    assert_eq!(
        events.borrow().as_slice(),
        [("setup", None), ("cleanup", None)]
    );
    assert!(
        host.sender::<InvalidEffectUpdate>(&Key::from("invalid"))
            .is_none()
    );
    assert!(matches!(
        host.drain(1),
        Err(ComponentError::Runtime(UpdateError::Poisoned))
    ));

    drop(host);
    assert_eq!(
        events.borrow().as_slice(),
        [("setup", None), ("cleanup", None)]
    );
}

#[test]
fn input_state_change_followed_by_adapter_error_poisons_same_input_retry() {
    let fail_after = Rc::new(Cell::new(None));
    let successful = Rc::new(Cell::new(0));
    let changes = Rc::new(Cell::new(0));
    let mut host = ComponentHost::mount(
        ControlledApplyAdapter {
            fail_after: Rc::clone(&fail_after),
            inner: RecordingAdapter::default(),
            successful,
        },
        [component::<StatefulInputComponent>(
            "input",
            StatefulInput {
                changes: Rc::clone(&changes),
                value: 0,
            },
        )],
    )
    .unwrap();
    let reference = host.reference(&Key::from("input")).unwrap();
    fail_after.set(Some(0));
    let next = StatefulInput {
        changes: Rc::clone(&changes),
        value: 1,
    };

    assert!(matches!(
        host.update_input::<StatefulInputComponent>(&Key::from("input"), next.clone()),
        Err(ComponentError::Runtime(UpdateError::Adapter(())))
    ));
    assert_eq!(changes.get(), 1);
    assert_eq!(reference.get(), None);
    assert!(matches!(
        host.update_input::<StatefulInputComponent>(&Key::from("input"), next),
        Err(ComponentError::Runtime(UpdateError::Poisoned))
    ));
}

#[test]
fn callback_triggered_render_failure_poisons_host() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<CallbackRenderFailure>("callback", ())],
    )
    .unwrap();
    let reference = host.reference(&Key::from("callback")).unwrap();
    let object = reference.get().unwrap();
    let event = host.runtime().graph().events(object).unwrap()[0].clone();
    host.queue_event(EventDispatch::new(
        object,
        event.id,
        event.value,
        EventPayload::Unit,
    ));

    assert!(matches!(
        host.drain(usize::MAX),
        Err(ComponentError::Runtime(UpdateError::Graph(
            GraphError::DuplicateKey(_)
        )))
    ));
    assert_eq!(reference.get(), None);
    assert!(matches!(
        host.drain(1),
        Err(ComponentError::Runtime(UpdateError::Poisoned))
    ));
}
