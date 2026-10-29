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
    events: Rc<RefCell<Vec<(Option<ObjectId>, bool, ComponentTaskStatus, bool)>>>,
    published: Rc<Cell<bool>>,
    reference: ElementRef,
    revision: Rc<Cell<u32>>,
    rendered: u32,
    task: Arc<Mutex<Option<ComponentTask>>>,
}

impl PartialEq for VirtualCleanupInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.events, &other.events)
            && Rc::ptr_eq(&self.published, &other.published)
            && self.reference == other.reference
            && self.rendered == other.rendered
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
        let root = context.root();
        context.use_effect("cleanup", (), move || {
            Some(Box::new(move || {
                events.borrow_mut().push((
                    reference.get(),
                    published.get(),
                    task.lock().unwrap().as_ref().unwrap().status(),
                    root.get().is_some(),
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
                let input = VirtualCleanupInput {
                    rendered: input.revision.get(),
                    ..input.clone()
                };
                component::<VirtualCleanup>("row", input)
            }))
            .into()
    }
}

struct VirtualLifecycleAdapter {
    inner: RecordingAdapter,
    published: Rc<Cell<bool>>,
}

impl Adapter for VirtualLifecycleAdapter {
    fn peek_native_event(&self) -> Option<NativeEvent> {
        self.inner.peek_native_event()
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn peek_native_dispatch(&self) -> Option<NativeEvent> {
        self.inner.peek_native_dispatch()
    }

    fn apply(&mut self, mutations: &[Mutation]) {
        self.published.set(true);
        self.inner.apply(mutations);
    }
}

fn virtual_cleanup_host() -> (
    ComponentHost<VirtualLifecycleAdapter>,
    VirtualCleanupInput,
    ObjectId,
) {
    let input = VirtualCleanupInput {
        events: Rc::new(RefCell::new(Vec::new())),
        published: Rc::new(Cell::new(false)),
        reference: ElementRef::default(),
        revision: Rc::default(),
        rendered: 0,
        task: Arc::new(Mutex::new(None)),
    };
    let adapter = VirtualLifecycleAdapter {
        inner: RecordingAdapter::default(),
        published: Rc::clone(&input.published),
    };
    let mut host = ComponentHost::mount_with_services(
        adapter,
        Arc::new(TestServices::default()),
        [component::<VirtualCleanupParent>("virtual", input.clone())],
    );
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
    host.drain(10);
    input.published.set(false);
    (host, input, collection)
}

#[test]
fn root_inputs_wait_for_virtual_realization() {
    let first = Rc::new(RefCell::new(Vec::new()));
    let second = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualParent>(
            "virtual",
            VirtualEffectLog(Rc::clone(&first)),
        )],
    );
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    host.queue_realization(RealizationRequest::Realize {
        collection,
        container: RealizedContainer(1),
        index: 0,
        source_revision: 0,
    });
    host.update_root(component::<VirtualParent>(
        "virtual",
        VirtualEffectLog(Rc::clone(&second)),
    ));
    host.drain(64);
    assert_eq!(*first.borrow(), ["setup"]);
    assert_eq!(host.test_state().virtual_rows, 1);
    assert!(!host.has_pending_input());
    drop(host);
    assert_eq!(*first.borrow(), ["setup", "cleanup"]);
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
    );
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
    host.drain(10);
    assert_eq!(events.borrow().as_slice(), ["setup"]);
    assert_eq!(host.runtime().adapter().object_count(), 3);

    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    host.drain(10);
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
    );
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
    host.drain(10);
    assert_eq!(renders.load(Ordering::Relaxed), 1);

    host.runtime.adapter_mut().queue_realization(realize);
    host.drain(10);
    assert_eq!(renders.load(Ordering::Relaxed), 1);

    assert_eq!(host.set_context(&context, 1).dispatched, 1);
    assert_eq!(renders.load(Ordering::Relaxed), 2);

    host.runtime.adapter_mut().queue_realization(realize);
    host.drain(10);
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
    );
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
    host.drain(10);
    assert_eq!(renders.load(Ordering::Relaxed), 1);

    assert_eq!(host.set_context(&context, 1).dispatched, 2);
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
    );
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
    host.drain(10);
    assert_eq!(renders.load(Ordering::Relaxed), 1);
    assert_eq!(host.test_state().virtual_rows, 1);

    assert_eq!(host.set_context(&context, 1).dispatched, 1);
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
    );
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

    assert_eq!(host.drain(10).dispatched, 1);
    assert_eq!(events.borrow().as_slice(), ["setup"]);
}

#[test]
fn realize_pending_applies_only_leading_realizations() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualParent>(
            "virtual",
            VirtualEffectLog(Rc::clone(&events)),
        )],
    );
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let realize = |container| RealizationRequest::Realize {
        collection,
        container: RealizedContainer(container),
        index: container as usize,
        source_revision: 0,
    };
    let sender = host.sender::<VirtualParent>(&Key::from("virtual")).unwrap();

    assert!(sender.send(()));
    host.runtime.adapter_mut().queue_realization(realize(1));
    host.runtime.adapter_mut().queue_event(EventDispatch::new(
        ObjectId::test(10_000),
        EventId::Click,
        EventValue::Unit(Callback::new(|()| {})),
        EventPayload::Unit,
    ));
    host.runtime.adapter_mut().queue_realization(realize(2));

    // The leading realization runs; the earlier message, the event, and the work behind it wait.
    assert_ne!(host.realize_pending().mutations, 0);
    assert_eq!(events.borrow().as_slice(), ["setup"]);
    assert_eq!(host.realize_pending().mutations, 0);

    assert_eq!(host.drain(10).dispatched, 1);
    assert_eq!(events.borrow().as_slice(), ["setup", "setup"]);
}

#[test]
fn component_update_drains_pending_virtual_work_exactly_once() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let input = VirtualEffectLog(Rc::clone(&events));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<VirtualParent>("virtual", input.clone())],
    );
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
    host.update_input::<VirtualParent>(&Key::from("virtual"), input.clone());
    assert_eq!(events.borrow().as_slice(), ["setup"]);
    assert_eq!(host.runtime().adapter().object_count(), 3);
    host.drain(10);
    assert_eq!(events.borrow().as_slice(), ["setup"]);

    host.runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    host.update_input::<VirtualParent>(&Key::from("virtual"), input);
    assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
    assert_eq!(host.runtime().adapter().object_count(), 2);
    host.drain(10);
    assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
}

#[test]
fn virtual_cleanup_precedes_native_recycle_publication() {
    let (mut host, input, collection) = virtual_cleanup_host();
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
    host.drain(10);

    assert_eq!(
        input.events.borrow().as_slice(),
        [(object, false, ComponentTaskStatus::Cancelled, true)]
    );
    assert!(input.published.get());
    assert_eq!(input.reference.get(), None);
}

/// Runs `layout` once inside the next batch it applies, as a WinUI layout forced by a native call
/// realizes rows while Reactor applies a batch.
#[derive(Clone, Default)]
struct LayoutAdapter {
    inner: Rc<RefCell<RecordingAdapter>>,
    layout: Rc<RefCell<Option<Box<dyn FnOnce()>>>>,
}

impl LayoutAdapter {
    fn on_next_apply(&self, layout: impl FnOnce() + 'static) {
        *self.layout.borrow_mut() = Some(Box::new(layout));
    }
}

impl Adapter for LayoutAdapter {
    fn peek_native_event(&self) -> Option<NativeEvent> {
        self.inner.borrow().peek_native_event()
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.borrow_mut().pop_native_event()
    }

    fn peek_native_dispatch(&self) -> Option<NativeEvent> {
        self.inner.borrow().peek_native_dispatch()
    }

    fn apply(&mut self, mutations: &[Mutation]) {
        self.inner.borrow_mut().apply(mutations);
        let layout = self.layout.borrow_mut().take();
        if let Some(layout) = layout {
            layout();
        }
    }
}

type SharedHost = Rc<RefCell<ComponentHost<LayoutAdapter>>>;

fn shared_host<A: Adapter>(host: &mut ComponentHost<A>) -> Option<&mut ComponentHost<A>> {
    Some(host)
}

fn realize_shared(host: &RefCell<ComponentHost<LayoutAdapter>>) {
    drive_host(host, shared_host, ComponentHost::next_realization_batch);
}

fn drain_shared(host: &RefCell<ComponentHost<LayoutAdapter>>) {
    let mut drain = host.borrow_mut().begin_drain(10);
    drive_host(host, shared_host, |host| host.drain_step(&mut drain));
    host.borrow_mut().end_drain(drain);
}

fn layout_host() -> (
    SharedHost,
    LayoutAdapter,
    Rc<RefCell<Vec<&'static str>>>,
    ObjectId,
) {
    let events = Rc::new(RefCell::new(Vec::new()));
    let adapter = LayoutAdapter::default();
    let host = ComponentHost::mount(
        adapter.clone(),
        [component::<VirtualParent>(
            "virtual",
            VirtualEffectLog(Rc::clone(&events)),
        )],
    );
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    (Rc::new(RefCell::new(host)), adapter, events, collection)
}

fn realize_row(collection: ObjectId, container: u64) -> RealizationRequest {
    RealizationRequest::Realize {
        collection,
        container: RealizedContainer(container),
        index: container as usize,
        source_revision: 0,
    }
}

fn recycle_row(collection: ObjectId, container: u64) -> RealizationRequest {
    RealizationRequest::Recycle {
        collection,
        container: RealizedContainer(container),
        source_revision: 0,
    }
}

#[test]
fn layout_during_a_batch_realizes_rows_before_effects_commit() {
    let (host, adapter, events, collection) = layout_host();
    let observed = Rc::new(RefCell::new(None));
    let layout = {
        let host = Rc::downgrade(&host);
        let adapter = adapter.clone();
        let events = Rc::clone(&events);
        let observed = Rc::clone(&observed);
        move || {
            let host = host.upgrade().unwrap();
            adapter
                .inner
                .borrow_mut()
                .queue_realization(realize_row(collection, 1));
            realize_shared(&host);
            *observed.borrow_mut() = Some((
                host.borrow().test_state().virtual_rows,
                events.borrow().clone(),
            ));
        }
    };
    adapter.on_next_apply(layout);
    let sender = host
        .borrow()
        .sender::<VirtualParent>(&Key::from("virtual"))
        .unwrap();

    assert!(sender.send(()));
    drain_shared(&host);

    // The row is realized inside the batch, and its effect runs once that batch completes.
    assert_eq!(*observed.borrow(), Some((1, Vec::new())));
    assert_eq!(events.borrow().as_slice(), ["setup"]);
}

#[test]
fn row_recycled_before_its_batch_completes_skips_its_effects() {
    let (host, adapter, events, collection) = layout_host();
    let layout = {
        let host = Rc::downgrade(&host);
        let adapter = adapter.clone();
        move || {
            let host = host.upgrade().unwrap();
            let mut inner = adapter.inner.borrow_mut();
            inner.queue_realization(realize_row(collection, 1));
            inner.queue_realization(recycle_row(collection, 1));
            drop(inner);
            realize_shared(&host);
        }
    };
    adapter.on_next_apply(layout);
    let sender = host
        .borrow()
        .sender::<VirtualParent>(&Key::from("virtual"))
        .unwrap();

    assert!(sender.send(()));
    drain_shared(&host);

    assert_eq!(host.borrow().test_state().virtual_rows, 0);
    assert!(events.borrow().is_empty());
}

#[test]
fn layout_during_a_row_batch_defers_that_row() {
    let (host, adapter, events, collection) = layout_host();
    let observed = Rc::new(Cell::new(None));
    let layout = {
        let host = Rc::downgrade(&host);
        let adapter = adapter.clone();
        let observed = Rc::clone(&observed);
        move || {
            let host = host.upgrade().unwrap();
            adapter
                .inner
                .borrow_mut()
                .queue_realization(recycle_row(collection, 1));
            realize_shared(&host);
            observed.set(Some(host.borrow().test_state().virtual_rows));
        }
    };
    adapter
        .inner
        .borrow_mut()
        .queue_realization(realize_row(collection, 1));
    adapter.on_next_apply(layout);

    realize_shared(&host);

    // The recycle waits for the row's own batch to complete and commit its effect.
    assert_eq!(observed.get(), Some(1));
    assert_eq!(host.borrow().test_state().virtual_rows, 0);
    assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
}

#[test]
fn row_recycled_after_a_nested_rerender_cleans_up_before_recycle() {
    let input = VirtualCleanupInput {
        events: Rc::default(),
        published: Rc::default(),
        reference: ElementRef::default(),
        revision: Rc::default(),
        rendered: 0,
        task: Arc::new(Mutex::new(None)),
    };
    let adapter = LayoutAdapter::default();
    let host = ComponentHost::mount_with_services(
        adapter.clone(),
        Arc::new(TestServices::default()),
        [component::<VirtualCleanupParent>("virtual", input.clone())],
    );
    let root = host.runtime().graph().root().unwrap();
    let collection = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let host = Rc::new(RefCell::new(host));
    adapter
        .inner
        .borrow_mut()
        .queue_realization(realize_row(collection, 0));
    realize_shared(&host);
    let object = input.reference.get();
    assert!(object.is_some());

    let observed = Rc::new(RefCell::new(None));
    let layout = {
        let host = Rc::downgrade(&host);
        let adapter = adapter.clone();
        let input = input.clone();
        let observed = Rc::clone(&observed);
        move || {
            let host = host.upgrade().unwrap();
            input.revision.set(1);
            let queue = |request| adapter.inner.borrow_mut().queue_realization(request);
            queue(realize_row(collection, 0));
            realize_shared(&host);
            queue(recycle_row(collection, 0));
            realize_shared(&host);
            *observed.borrow_mut() = Some(input.events.borrow().clone());
        }
    };
    adapter.on_next_apply(layout);
    let sender = host
        .borrow()
        .sender::<VirtualCleanupParent>(&Key::from("virtual"))
        .unwrap();

    // The nested realization re-renders the row and retains its effect until the outer batch
    // completes. Recycling the row runs that retained cleanup while its root and reference are
    // still live.
    assert!(sender.send(()));
    drain_shared(&host);

    assert_eq!(
        *observed.borrow(),
        Some(vec![(object, false, ComponentTaskStatus::Cancelled, true)])
    );
    assert_eq!(input.events.borrow().len(), 1);
    assert_eq!(input.reference.get(), None);
    assert_eq!(host.borrow().test_state().virtual_rows, 0);
}

struct NestedVirtualParent;

impl Component for NestedVirtualParent {
    type Input = VirtualEffectLog;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let log = input.clone();
        ItemsRepeater::new()
            .virtual_source(VirtualSource::new(1, 10, Key::from, move |outer| {
                component::<VirtualParent>(outer, log.clone())
            }))
            .into()
    }
}

#[test]
fn layout_during_a_nested_row_batch_defers_its_enclosing_row() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let adapter = LayoutAdapter::default();
    let host = ComponentHost::mount(
        adapter.clone(),
        [component::<NestedVirtualParent>(
            "virtual",
            VirtualEffectLog(Rc::clone(&events)),
        )],
    );
    let root = host.runtime().graph().root().unwrap();
    let outer = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let host = Rc::new(RefCell::new(host));
    adapter
        .inner
        .borrow_mut()
        .queue_realization(realize_row(outer, 0));
    realize_shared(&host);
    let inner = host
        .borrow()
        .runtime()
        .graph()
        .virtual_realization(outer, RealizedContainer(0))
        .unwrap();

    let observed = Rc::new(Cell::new(None));
    let layout = {
        let host = Rc::downgrade(&host);
        let adapter = adapter.clone();
        let observed = Rc::clone(&observed);
        move || {
            let host = host.upgrade().unwrap();
            adapter
                .inner
                .borrow_mut()
                .queue_realization(recycle_row(outer, 0));
            realize_shared(&host);
            observed.set(Some(host.borrow().test_state().virtual_rows));
        }
    };
    adapter
        .inner
        .borrow_mut()
        .queue_realization(realize_row(inner, 0));
    adapter.on_next_apply(layout);

    realize_shared(&host);

    // Recycling the outer row would destroy the inner collection while its row's batch applies,
    // so the recycle waits for that batch to complete.
    assert_eq!(observed.get(), Some(2));
    assert_eq!(host.borrow().test_state().virtual_rows, 0);
    assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
}

struct NestedCleanupParent;

impl Component for NestedCleanupParent {
    type Input = VirtualCleanupInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let input = input.clone();
        ItemsRepeater::new()
            .virtual_source(VirtualSource::new(1, 1, Key::from, move |_| {
                component::<VirtualCleanupParent>("inner", input.clone())
            }))
            .into()
    }
}

#[test]
fn enclosing_row_recycled_after_a_nested_rerender_cleans_up_inner_rows_first() {
    let input = VirtualCleanupInput {
        events: Rc::default(),
        published: Rc::default(),
        reference: ElementRef::default(),
        revision: Rc::default(),
        rendered: 0,
        task: Arc::new(Mutex::new(None)),
    };
    let adapter = LayoutAdapter::default();
    let host = ComponentHost::mount_with_services(
        adapter.clone(),
        Arc::new(TestServices::default()),
        [component::<NestedCleanupParent>("virtual", input.clone())],
    );
    let root = host.runtime().graph().root().unwrap();
    let outer = host
        .runtime()
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let host = Rc::new(RefCell::new(host));
    let queue = |request| adapter.inner.borrow_mut().queue_realization(request);
    queue(realize_row(outer, 0));
    realize_shared(&host);
    let inner = host
        .borrow()
        .runtime()
        .graph()
        .virtual_realization(outer, RealizedContainer(0))
        .unwrap();
    queue(realize_row(inner, 0));
    realize_shared(&host);
    let object = input.reference.get();
    assert!(object.is_some());

    let observed = Rc::new(RefCell::new(None));
    let layout = {
        let host = Rc::downgrade(&host);
        let adapter = adapter.clone();
        let observed = Rc::clone(&observed);
        move || {
            let host = host.upgrade().unwrap();
            let queue = |request| adapter.inner.borrow_mut().queue_realization(request);
            input.revision.set(1);
            queue(realize_row(inner, 0));
            realize_shared(&host);
            queue(recycle_row(outer, 0));
            realize_shared(&host);
            *observed.borrow_mut() = Some(input.events.borrow().clone());
        }
    };
    adapter.on_next_apply(layout);
    let sender = host
        .borrow()
        .sender::<NestedCleanupParent>(&Key::from("virtual"))
        .unwrap();

    // Recycling the outer row retires the inner row, whose effect a nested batch retained. Its
    // cleanup runs while the inner row's root and reference are still live.
    assert!(sender.send(()));
    drain_shared(&host);

    assert_eq!(
        *observed.borrow(),
        Some(vec![(object, false, ComponentTaskStatus::Cancelled, true)])
    );
    assert_eq!(host.borrow().test_state().virtual_rows, 0);
}
