use super::*;

#[derive(Clone)]
struct VirtualEffectLog(Rc<RefCell<Vec<&'static str>>>);

impl PartialEq for VirtualEffectLog {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

struct VirtualEffect;

impl Component for VirtualEffect {
    type Input = VirtualEffectLog;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let setup = Rc::clone(&input.0);
        let cleanup = Rc::clone(&input.0);
        context.use_effect("virtual", (), move || {
            setup.borrow_mut().push("setup");
            Some(Box::new(move || cleanup.borrow_mut().push("cleanup")))
        });
        TextBlock::new().text("virtual").into()
    }
}

struct VirtualParent;

impl Component for VirtualParent {
    type Input = VirtualEffectLog;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let log = input.clone();
        ItemsRepeater::new()
            .virtual_source(VirtualSource::new(
                1,
                10_000,
                Key::from,
                move |index| -> View { component::<VirtualEffect>(index, log.clone()).into() },
            ))
            .into()
    }
}

struct VirtualContextParent;

impl Component for VirtualContextParent {
    type Input = ContextInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        ItemsRepeater::new()
            .item("row", component::<ContextReader>("row", input.clone()))
            .into()
    }
}

struct VirtualContextOwner {
    source: VirtualSource,
}

impl Component for VirtualContextOwner {
    type Input = ContextInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        let input = input.clone();
        Self {
            source: VirtualSource::new(1, 1, Key::from, move |index| {
                component::<ContextReader>(index, input.clone())
            }),
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let _ = context.use_context(&input.context);
        ItemsRepeater::new()
            .virtual_source(self.source.clone())
            .into()
    }
}

struct VirtualRemovingOwner {
    source: VirtualSource,
}

impl Component for VirtualRemovingOwner {
    type Input = ContextInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        let input = input.clone();
        Self {
            source: VirtualSource::new(1, 1, Key::from, move |index| {
                component::<ContextReader>(index, input.clone())
            }),
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if context.use_context(&input.context) == 0 {
            ItemsRepeater::new()
                .virtual_source(self.source.clone())
                .into()
        } else {
            TextBlock::new().into()
        }
    }
}

#[derive(Clone)]
struct VirtualCleanupInput {
    events: Rc<RefCell<Vec<(Option<ObjectId>, bool, ComponentTaskStatus)>>>,
    fail_apply: Rc<Cell<bool>>,
    fail_validate: Rc<Cell<bool>>,
    published: Rc<Cell<bool>>,
    reference: ElementRef,
    task: Arc<Mutex<Option<ComponentTask>>>,
}

impl PartialEq for VirtualCleanupInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.events, &other.events)
            && Rc::ptr_eq(&self.published, &other.published)
            && self.reference == other.reference
            && Arc::ptr_eq(&self.task, &other.task)
    }
}

struct VirtualCleanup;

impl Component for VirtualCleanup {
    type Input = VirtualCleanupInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.task.lock().unwrap() = Some(context.spawn_background(|_| ()));
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let events = Rc::clone(&input.events);
        let published = Rc::clone(&input.published);
        let reference = input.reference.clone();
        let task = Arc::clone(&input.task);
        context.use_effect("cleanup", (), move || {
            Some(Box::new(move || {
                events.borrow_mut().push((
                    reference.get(),
                    published.get(),
                    task.lock().unwrap().as_ref().unwrap().status(),
                ));
            }))
        });
        Button::new().element_ref(&input.reference).into()
    }
}

struct VirtualCleanupParent;

impl Component for VirtualCleanupParent {
    type Input = VirtualCleanupInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let input = input.clone();
        ItemsRepeater::new()
            .virtual_source(VirtualSource::new(1, 1, Key::from, move |_| {
                component::<VirtualCleanup>("row", input.clone())
            }))
            .into()
    }
}

struct VirtualLifecycleAdapter {
    inner: RecordingAdapter,
    fail_apply: Rc<Cell<bool>>,
    fail_validate: Rc<Cell<bool>>,
    published: Rc<Cell<bool>>,
}

impl Adapter for VirtualLifecycleAdapter {
    type Error = ();

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.fail_validate.get() {
            Err(())
        } else {
            self.inner.validate(mutations).map_err(|_| ())
        }
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.published.set(true);
        if self.fail_apply.get() {
            Err(())
        } else {
            self.inner.apply(mutations).map_err(|_| ())
        }
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object).map_err(|_| ())
    }
}

fn virtual_cleanup_host(
    fail_validate: bool,
    fail_apply: bool,
) -> (
    ComponentHost<VirtualLifecycleAdapter>,
    VirtualCleanupInput,
    ObjectId,
) {
    let input = VirtualCleanupInput {
        events: Rc::new(RefCell::new(Vec::new())),
        fail_apply: Rc::new(Cell::new(false)),
        fail_validate: Rc::new(Cell::new(false)),
        published: Rc::new(Cell::new(false)),
        reference: ElementRef::default(),
        task: Arc::new(Mutex::new(None)),
    };
    let adapter = VirtualLifecycleAdapter {
        inner: RecordingAdapter::default(),
        fail_apply: Rc::clone(&input.fail_apply),
        fail_validate: Rc::clone(&input.fail_validate),
        published: Rc::clone(&input.published),
    };
    let mut host = ComponentHost::mount_with_services(
        adapter,
        Arc::new(TestServices::default()),
        [component::<VirtualCleanupParent>("virtual", input.clone())],
    )
    .unwrap();
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    host.runtime
        .adapter_mut()
        .inner
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    host.drain(10).unwrap();
    input.published.set(false);
    input.fail_validate.set(fail_validate);
    input.fail_apply.set(fail_apply);
    (host, input, collection)
}

#[test]
fn virtual_rows_own_components_only_while_realized() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualParent>(
            "virtual",
            VirtualEffectLog(Rc::clone(&events)),
        )],
    )
    .unwrap();
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_eq!(host.runtime().adapter().object_count(), 2);

    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 9_999,
            source_revision: 0,
        });
    host.drain(10).unwrap();
    assert_eq!(events.borrow().as_slice(), ["setup"]);
    assert_eq!(host.runtime().adapter().object_count(), 3);

    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    host.drain(10).unwrap();
    assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
    assert_eq!(host.runtime().adapter().object_count(), 2);
}

#[test]
fn virtual_row_reuses_unchanged_view_and_tracks_context() {
    let context = Rc::new(Context::new(0usize));
    let renders = Arc::new(AtomicUsize::new(0));
    let input = ContextInput {
        context: Rc::clone(&context),
        renders: Arc::clone(&renders),
        subscribe: true,
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualContextParent>("virtual", input)],
    )
    .unwrap();
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let realize = RealizationRequest::Realize {
        collection,
        container: RealizedContainer(1),
        index: 0,
        source_revision: 0,
    };

    host.runtime.adapter_mut().queue_realization(realize);
    host.drain(10).unwrap();
    assert_eq!(renders.load(Ordering::Relaxed), 1);

    host.runtime.adapter_mut().queue_realization(realize);
    host.drain(10).unwrap();
    assert_eq!(renders.load(Ordering::Relaxed), 1);

    assert_eq!(host.set_context(&context, 1).unwrap().dispatched, 1);
    assert_eq!(renders.load(Ordering::Relaxed), 2);

    host.runtime.adapter_mut().queue_realization(realize);
    host.drain(10).unwrap();
    assert_eq!(renders.load(Ordering::Relaxed), 2);
}

#[test]
fn virtual_row_context_consumer_is_not_hidden_by_owner() {
    let context = Rc::new(Context::new(0usize));
    let renders = Arc::new(AtomicUsize::new(0));
    let input = ContextInput {
        context: Rc::clone(&context),
        renders: Arc::clone(&renders),
        subscribe: true,
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualContextOwner>("virtual", input)],
    )
    .unwrap();
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];

    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    host.drain(10).unwrap();
    assert_eq!(renders.load(Ordering::Relaxed), 1);

    assert_eq!(host.set_context(&context, 1).unwrap().dispatched, 2);
    assert_eq!(renders.load(Ordering::Relaxed), 2);
}

#[test]
fn virtual_row_consumer_is_skipped_when_owner_retires_it() {
    let context = Rc::new(Context::new(0usize));
    let renders = Arc::new(AtomicUsize::new(0));
    let input = ContextInput {
        context: Rc::clone(&context),
        renders: Arc::clone(&renders),
        subscribe: true,
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualRemovingOwner>("virtual", input)],
    )
    .unwrap();
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    host.drain(10).unwrap();
    assert_eq!(renders.load(Ordering::Relaxed), 1);
    assert_eq!(host.test_state().virtual_rows, 1);

    assert_eq!(host.set_context(&context, 1).unwrap().dispatched, 1);
    assert_eq!(renders.load(Ordering::Relaxed), 1);
    assert_eq!(host.test_state().virtual_rows, 0);
}

#[test]
fn pending_virtual_work_precedes_queued_component_messages() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualParent>(
            "virtual",
            VirtualEffectLog(Rc::clone(&events)),
        )],
    )
    .unwrap();
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];

    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 9_999,
            source_revision: 0,
        });
    assert!(
        host.sender::<VirtualParent>(&Key::from("virtual"))
            .unwrap()
            .send(())
    );

    assert_eq!(host.drain(10).unwrap().dispatched, 1);
    assert_eq!(events.borrow().as_slice(), ["setup"]);
}

#[test]
fn component_update_drains_pending_virtual_work_exactly_once() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let input = VirtualEffectLog(Rc::clone(&events));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualParent>("virtual", input.clone())],
    )
    .unwrap();
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];

    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 9_999,
            source_revision: 0,
        });
    host.update_input::<VirtualParent>(&Key::from("virtual"), input.clone())
        .unwrap();
    assert_eq!(events.borrow().as_slice(), ["setup"]);
    assert_eq!(host.runtime().adapter().object_count(), 3);
    host.drain(10).unwrap();
    assert_eq!(events.borrow().as_slice(), ["setup"]);

    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    host.update_input::<VirtualParent>(&Key::from("virtual"), input)
        .unwrap();
    assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
    assert_eq!(host.runtime().adapter().object_count(), 2);
    host.drain(10).unwrap();
    assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
}

#[test]
fn virtual_cleanup_precedes_native_recycle_publication() {
    let (mut host, input, collection) = virtual_cleanup_host(false, false);
    let object = input.reference.get();
    assert!(object.is_some());
    assert_eq!(
        input.task.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Running
    );

    host.runtime
        .adapter_mut()
        .inner
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    host.drain(10).unwrap();

    assert_eq!(
        input.events.borrow().as_slice(),
        [(object, false, ComponentTaskStatus::Cancelled)]
    );
    assert!(input.published.get());
    assert_eq!(input.reference.get(), None);
}

#[test]
fn virtual_cleanup_is_committed_once_when_native_recycle_apply_fails() {
    let (mut host, input, collection) = virtual_cleanup_host(false, true);
    let object = input.reference.get();

    host.runtime
        .adapter_mut()
        .inner
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    assert!(matches!(
        host.drain(10),
        Err(ComponentError::Runtime(UpdateError::Adapter(())))
    ));
    assert_eq!(
        input.events.borrow().as_slice(),
        [(object, false, ComponentTaskStatus::Cancelled)]
    );
    assert!(input.published.get());
    assert_eq!(input.reference.get(), None);

    drop(host);
    assert_eq!(input.events.borrow().len(), 1);
}

#[test]
fn virtual_cleanup_waits_for_successful_recycle_validation() {
    let (mut host, input, collection) = virtual_cleanup_host(true, false);

    host.runtime
        .adapter_mut()
        .inner
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    assert!(matches!(
        host.drain(10),
        Err(ComponentError::Runtime(UpdateError::Adapter(())))
    ));
    assert_eq!(input.events.borrow().len(), 1);
    assert!(!input.published.get());
    assert_eq!(
        input.task.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Cancelled
    );

    drop(host);
    assert_eq!(input.events.borrow().len(), 1);
}
