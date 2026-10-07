use super::*;

struct ContextParent;

impl Component for ContextParent {
    type Input = ContextInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        input.renders.fetch_add(1, Ordering::Relaxed);
        let _ = context.use_context(&input.context);
        Border::new()
            .content(component::<ContextReader>("reader", input.clone()))
            .into()
    }
}

#[derive(Clone)]
struct RenderCountInput(Arc<AtomicUsize>);

impl PartialEq for RenderCountInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

struct RenderCountChild;

impl Component for RenderCountChild {
    type Input = RenderCountInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        input.0.fetch_add(1, Ordering::Relaxed);
        TextBlock::new().into()
    }
}

struct RenderCountParent(RenderCountInput);

impl Component for RenderCountParent {
    type Input = RenderCountInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .children(
                (0..300usize)
                    .map(|index| component::<RenderCountChild>(index, self.0.clone()).into())
                    .collect::<Vec<View>>(),
            )
            .into()
    }
}

#[derive(Clone)]
struct ProviderInput {
    context: Rc<Context<usize>>,
    seen: Rc<RefCell<Vec<usize>>>,
    value: usize,
}

impl PartialEq for ProviderInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.seen, &other.seen)
            && self.value == other.value
    }
}

#[derive(Clone)]
struct ProviderChildInput {
    context: Rc<Context<usize>>,
    seen: Rc<RefCell<Vec<usize>>>,
}

impl PartialEq for ProviderChildInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context) && Rc::ptr_eq(&self.seen, &other.seen)
    }
}

struct ProviderChild;

impl Component for ProviderChild {
    type Input = ProviderChildInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let value = context.use_context(&input.context);
        input.seen.borrow_mut().push(value);
        TextBlock::new().text(value.to_string()).into()
    }
}

struct ProviderParent;

impl Component for ProviderParent {
    type Input = ProviderInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        provide(
            &input.context,
            input.value,
            Border::new().content(component::<ProviderChild>(
                "child",
                ProviderChildInput {
                    context: Rc::clone(&input.context),
                    seen: Rc::clone(&input.seen),
                },
            )),
        )
    }
}

struct DynamicProviderParent;

impl Component for DynamicProviderParent {
    type Input = ProviderInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let child = Border::new().content(component::<ProviderChild>(
            "child",
            ProviderChildInput {
                context: Rc::clone(&input.context),
                seen: Rc::clone(&input.seen),
            },
        ));
        if context.use_context(&input.context) == 0 {
            child.into()
        } else {
            provide(&input.context, input.value, child)
        }
    }
}

#[test]
fn context_change_renders_only_subscribers() {
    let context = Rc::new(Context::new(0usize));
    let subscriber_renders = Arc::new(AtomicUsize::new(0));
    let other_renders = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [
            component::<ContextReader>(
                "subscriber",
                ContextInput {
                    context: Rc::clone(&context),
                    renders: Arc::clone(&subscriber_renders),
                    subscribe: true,
                },
            ),
            component::<ContextReader>(
                "other",
                ContextInput {
                    context: Rc::clone(&context),
                    renders: Arc::clone(&other_renders),
                    subscribe: false,
                },
            ),
        ],
    )
    .unwrap();

    let report = host.set_context(&context, 1).unwrap();

    assert_eq!(report.dispatched, 1);
    assert_eq!(report.mutations, 1);
    assert_eq!(subscriber_renders.load(Ordering::Relaxed), 2);
    assert_eq!(other_renders.load(Ordering::Relaxed), 1);
    assert_eq!(
        host.set_context(&context, 1).unwrap(),
        ComponentDrain::default()
    );
}

#[test]
fn context_change_renders_nested_subscribers_once() {
    let context = Rc::new(Context::new(0usize));
    let renders = Arc::new(AtomicUsize::new(0));
    let input = ContextInput {
        context: Rc::clone(&context),
        renders: Arc::clone(&renders),
        subscribe: true,
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<ContextParent>("parent", input)],
    )
    .unwrap();
    assert_eq!(renders.load(Ordering::Relaxed), 2);

    let report = host.set_context(&context, 1).unwrap();
    assert_eq!(report.dispatched, 1);
    assert_eq!(renders.load(Ordering::Relaxed), 4);
}

#[test]
fn parent_render_reuses_unchanged_child_views() {
    let renders = Arc::new(AtomicUsize::new(0));
    let input = RenderCountInput(Arc::clone(&renders));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<RenderCountParent>("parent", input)],
    )
    .unwrap();
    assert_eq!(renders.load(Ordering::Relaxed), 300);
    let state = host.test_state();

    let sender = host
        .sender::<RenderCountParent>(&Key::from("parent"))
        .unwrap();
    for _ in 0..20 {
        assert!(sender.send(()));
    }
    assert_eq!(host.drain(20).unwrap().dispatched, 20);
    assert_eq!(renders.load(Ordering::Relaxed), 300);
    assert_eq!(host.test_state(), state);
}

#[test]
fn provided_context_shadows_host_context_across_updates() {
    let context = Rc::new(Context::new(0usize));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let input = ProviderInput {
        context: Rc::clone(&context),
        seen: Rc::clone(&seen),
        value: 1,
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<ProviderParent>("parent", input.clone())],
    )
    .unwrap();
    assert_eq!(&*seen.borrow(), &[1]);

    assert_eq!(
        host.set_context(&context, 9).unwrap(),
        ComponentDrain::default()
    );
    assert_eq!(&*seen.borrow(), &[1]);

    host.update_input::<ProviderParent>(&Key::from("parent"), ProviderInput { value: 2, ..input })
        .unwrap();
    assert_eq!(&*seen.borrow(), &[1, 2]);

    let sender = host
        .sender_at::<ProviderChild>(&[Key::from("parent"), Key::from("child")])
        .unwrap();
    assert!(sender.send(()));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(&*seen.borrow(), &[1, 2, 2]);
}

#[test]
fn ancestor_can_shadow_context_during_host_update() {
    let context = Rc::new(Context::new(0usize));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let input = ProviderInput {
        context: Rc::clone(&context),
        seen: Rc::clone(&seen),
        value: 42,
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<DynamicProviderParent>("parent", input)],
    )
    .unwrap();
    assert_eq!(&*seen.borrow(), &[0]);

    assert_eq!(host.set_context(&context, 1).unwrap().dispatched, 1);
    assert_eq!(&*seen.borrow(), &[0, 42]);
}

#[test]
fn broad_context_update_does_not_scan_all_graph_references_per_consumer() {
    let count = 16_384;
    let context = Rc::new(Context::new(0usize));
    let renders = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        (0..count).map(|index| {
            component::<ContextReader>(
                index,
                ContextInput {
                    context: Rc::clone(&context),
                    renders: Arc::clone(&renders),
                    subscribe: true,
                },
            )
        }),
    )
    .unwrap();
    let scans = host.runtime().graph().full_reference_scan_count();

    let report = host.set_context(&context, 1).unwrap();

    assert_eq!(report.dispatched, count);
    assert_eq!(host.runtime().graph().full_reference_scan_count(), scans);
}

#[test]
fn context_failure_after_one_consumer_commit_poisons_every_consumer() {
    let context = Rc::new(Context::new(0usize));
    let fail_after = Rc::new(Cell::new(None));
    let successful = Rc::new(Cell::new(0));
    let first_renders = Arc::new(AtomicUsize::new(0));
    let second_renders = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        ControlledApplyAdapter {
            fail_after: Rc::clone(&fail_after),
            inner: RecordingAdapter::default(),
            successful: Rc::clone(&successful),
        },
        [
            component::<ContextReader>(
                "first",
                ContextInput {
                    context: Rc::clone(&context),
                    renders: Arc::clone(&first_renders),
                    subscribe: true,
                },
            ),
            component::<ContextReader>(
                "second",
                ContextInput {
                    context: Rc::clone(&context),
                    renders: Arc::clone(&second_renders),
                    subscribe: true,
                },
            ),
        ],
    )
    .unwrap();
    let first = host.reference(&Key::from("first")).unwrap();
    let second = host.reference(&Key::from("second")).unwrap();
    let baseline = successful.get();
    fail_after.set(Some(1));

    assert!(matches!(
        host.set_context(&context, 1),
        Err(ComponentError::Runtime(UpdateError::Adapter(())))
    ));
    assert_eq!(successful.get(), baseline + 1);
    assert_eq!(first_renders.load(Ordering::Relaxed), 2);
    assert_eq!(second_renders.load(Ordering::Relaxed), 2);
    assert_eq!(first.get(), None);
    assert_eq!(second.get(), None);
    assert!(matches!(
        host.set_context(&context, 1),
        Err(ComponentError::Runtime(UpdateError::Poisoned))
    ));
}
