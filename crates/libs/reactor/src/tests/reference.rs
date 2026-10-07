use super::*;

#[test]
fn reference_only_changes_reconcile_without_native_mutations() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Button::new()).unwrap();
    let button = runtime.graph().root().unwrap();

    assert!(
        runtime
            .update(Button::new().element_ref(&reference))
            .unwrap()
            .is_empty()
    );
    assert_eq!(reference.get(), Some(button));

    assert!(runtime.update(Button::new()).unwrap().is_empty());
    assert_eq!(reference.get(), None);
}

struct HoldingImperativeAdapter {
    inner: RecordingAdapter,
    commands: Rc<RefCell<Vec<ImperativeRequest>>>,
}

impl Adapter for HoldingImperativeAdapter {
    type Error = AdapterError;

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.validate(mutations)
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.apply(mutations)
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object)
    }

    fn imperative(&mut self, request: ImperativeRequest) -> Result<(), Self::Error> {
        self.commands.borrow_mut().push(request);
        Ok(())
    }
}

#[test]
fn typed_references_cover_all_imperative_controls() {
    let text_box = ElementRef::<TextBox>::new();
    let grid = ElementRef::<Grid>::new();
    let image = ElementRef::<Image>::new();
    let webview = ElementRef::<WebView2>::new();
    let swap_chain = ElementRef::<SwapChainPanel>::new();

    let _: View = TextBox::new("").element_ref(&text_box).into();
    let _: View = Grid::new().element_ref(&grid).into();
    let _: View = Image::new().element_ref(&image).into();
    let _: View = WebView2::new().element_ref(&webview).into();
    let _: View = SwapChainPanel::new().element_ref(&swap_chain).into();
}

#[test]
fn typed_element_reference_queues_focus() {
    let reference = ElementRef::<TextBox>::new();
    let result = Rc::new(Cell::new(None));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("").element_ref(&reference))
        .unwrap();
    let completed = Rc::clone(&result);
    assert!(reference.request_focus_result(move |value| {
        completed.set(Some(value));
    }));
    runtime.dispatch_native_events().unwrap();
    assert_eq!(result.get(), Some(Ok(true)));
}

#[test]
fn stale_imperative_completion_reports_unavailable_after_rebinding() {
    let commands = Rc::new(RefCell::new(Vec::new()));
    let reference = ElementRef::<Image>::new();
    let replacement = ElementRef::<Image>::new();
    let results = Rc::new(RefCell::new(Vec::new()));
    let mut runtime = Runtime::new(HoldingImperativeAdapter {
        inner: RecordingAdapter::default(),
        commands: Rc::clone(&commands),
    });
    runtime
        .update(Image::new().element_ref(&reference))
        .unwrap();
    let callback_results = Rc::clone(&results);
    assert!(reference.request_set_native_source(None, move |result| {
        callback_results.borrow_mut().push(result);
    }));
    runtime.dispatch_native_events().unwrap();
    runtime
        .update(Image::new().element_ref(&replacement))
        .unwrap();

    let ImperativeRequest::SetNativeImageSource { completion, .. } =
        commands.borrow_mut().remove(0)
    else {
        panic!("expected image-source request");
    };
    completion.call(Ok(()));

    assert_eq!(
        results.borrow().as_slice(),
        [Err(IntegrationError::Unavailable)]
    );
    assert_eq!(reference.get(), None);
    assert!(replacement.get().is_some());
}

#[test]
fn surface_binding_tokens_follow_reference_bindings_not_panel_identity() {
    let commands = Rc::new(RefCell::new(Vec::new()));
    let reference = ElementRef::<SwapChainPanel>::new();
    let _observation = reference.observe_surface(|_| {});
    let mut runtime = Runtime::new(HoldingImperativeAdapter {
        inner: RecordingAdapter::default(),
        commands: Rc::clone(&commands),
    });
    let last_binding = || {
        commands
            .borrow()
            .iter()
            .rev()
            .find_map(|request| match request {
                ImperativeRequest::ObserveSwapChainPanel { binding, .. } => {
                    Some(SwapChainPanelBinding::new(*binding))
                }
                _ => None,
            })
            .unwrap()
    };

    runtime
        .update(SwapChainPanel::new().element_ref(&reference))
        .unwrap();
    runtime.dispatch_native_events().unwrap();
    let panel = reference.get().unwrap();
    let initial = last_binding();

    runtime
        .update(SwapChainPanel::new().element_ref(&reference).width(100.0))
        .unwrap();
    commands.borrow_mut().clear();
    let _second_observation = reference.observe_surface(|_| {});
    runtime.dispatch_native_events().unwrap();
    assert_eq!(last_binding(), initial);
    assert_eq!(reference.get(), Some(panel));

    runtime.update(SwapChainPanel::new()).unwrap();
    runtime.dispatch_native_events().unwrap();
    assert_eq!(reference.get(), None);
    assert_eq!(runtime.graph().root(), Some(panel));

    commands.borrow_mut().clear();
    runtime
        .update(SwapChainPanel::new().element_ref(&reference))
        .unwrap();
    runtime.dispatch_native_events().unwrap();
    let rebound = last_binding();
    assert_ne!(rebound, initial);
    assert_eq!(reference.get(), Some(panel));
    let bindings: Vec<_> = commands
        .borrow()
        .iter()
        .filter_map(|request| match request {
            ImperativeRequest::ObserveSwapChainPanel { binding, .. } => {
                Some(SwapChainPanelBinding::new(*binding))
            }
            _ => None,
        })
        .collect();
    assert_eq!(bindings, [rebound, rebound]);
}

#[test]
fn observations_follow_reference_rebinding_and_drop() {
    let commands = Rc::new(RefCell::new(Vec::new()));
    let reference = ElementRef::<Grid>::new();
    let observation = reference.observe_composition_host(|_| {});
    let mut runtime = Runtime::new(HoldingImperativeAdapter {
        inner: RecordingAdapter::default(),
        commands: Rc::clone(&commands),
    });

    runtime.update(Grid::new().element_ref(&reference)).unwrap();
    runtime.dispatch_native_events().unwrap();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));

    runtime.update(Grid::new()).unwrap();
    runtime.dispatch_native_events().unwrap();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));

    runtime.update(Grid::new().element_ref(&reference)).unwrap();
    runtime.dispatch_native_events().unwrap();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));

    drop(observation);
    runtime.dispatch_native_events().unwrap();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));
}

#[test]
fn observation_lifecycle_survives_imperative_queue_saturation() {
    let commands = Rc::new(RefCell::new(Vec::new()));
    let reference = ElementRef::<Grid>::new();
    let mut runtime = Runtime::new(HoldingImperativeAdapter {
        inner: RecordingAdapter::default(),
        commands: Rc::clone(&commands),
    });
    runtime.update(Grid::new().element_ref(&reference)).unwrap();

    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    assert!(!reference.request_focus());
    let observation = reference.observe_composition_host(|_| {});
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events().unwrap();
    }
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));
    commands.borrow_mut().clear();

    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    runtime.update(Grid::new()).unwrap();
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events().unwrap();
    }
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));
    commands.borrow_mut().clear();

    runtime.update(Grid::new().element_ref(&reference)).unwrap();
    runtime.dispatch_native_events().unwrap();
    commands.borrow_mut().clear();
    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    drop(observation);
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events().unwrap();
    }
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));
}

#[test]
fn destroying_observed_object_discards_queued_revocation() {
    let reference = ElementRef::<Grid>::new();
    let _observation = reference.observe_composition_host(|_| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().keyed_children([keyed("observed", Grid::new().element_ref(&reference))]),
        )
        .unwrap();
    runtime.dispatch_native_events().unwrap();

    runtime.update(Grid::new()).unwrap();

    assert_eq!(reference.get(), None);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn dropping_runtime_clears_typed_reference_and_pending_completion() {
    let reference = ElementRef::<SwapChainPanel>::new();
    let results = Rc::new(RefCell::new(Vec::new()));
    {
        let mut runtime = Runtime::new(RecordingAdapter::default());
        runtime
            .update(SwapChainPanel::new().element_ref(&reference))
            .unwrap();
        let callback_results = Rc::clone(&results);
        assert!(reference.request_clear_swap_chain(move |result| {
            callback_results.borrow_mut().push(result);
        }));
    }
    assert_eq!(reference.get(), None);
    assert_eq!(
        results.borrow().as_slice(),
        [Err(IntegrationError::Unavailable)]
    );
}

#[test]
fn imperative_budget_rearms_pending_work() {
    let reference = ElementRef::<Image>::new();
    let wakes = Rc::new(Cell::new(0));
    let wake_count = Rc::clone(&wakes);
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.set_imperative_waker(move || wake_count.set(wake_count.get() + 1));
    runtime
        .update(Image::new().element_ref(&reference))
        .unwrap();
    for _ in 0..65 {
        assert!(reference.request_set_native_source(None, |_| {}));
    }
    wakes.set(0);

    runtime.dispatch_native_events().unwrap();

    assert_eq!(wakes.get(), 1);
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.adapter().imperatives().len(), 65);
}

#[test]
fn reference_transfers_are_order_independent() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().children([
            Button::new().into(),
            Button::new().element_ref(&reference).into(),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    assert_eq!(reference.get(), Some(children[1]));

    assert!(
        runtime
            .update(StackPanel::new().children([
                Button::new().element_ref(&reference).into(),
                Button::new().into(),
            ]),)
            .unwrap()
            .is_empty()
    );
    assert_eq!(reference.get(), Some(children[0]));

    assert!(
        runtime
            .update(StackPanel::new().children([
                Button::new().into(),
                Button::new().element_ref(&reference).into(),
            ]),)
            .unwrap()
            .is_empty()
    );
    assert_eq!(reference.get(), Some(children[1]));
}

#[test]
fn references_survive_keyed_replacement_and_clear_on_destruction() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().keyed_children([keyed("focused", Button::new().element_ref(&reference))]),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let child = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_eq!(reference.get(), Some(child));

    runtime
        .update(Grid::new().keyed_children([keyed(
            "focused",
            TextBox::new("replacement").element_ref(&reference),
        )]))
        .unwrap();
    let replacement = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_ne!(replacement, child);
    assert_eq!(reference.get(), Some(replacement));
    assert_eq!(runtime.graph().kind(replacement), Some(ObjectType::TextBox));

    runtime.update(Grid::new()).unwrap();
    assert_eq!(reference.get(), None);
}

#[test]
fn duplicate_references_are_rejected_before_reconciliation() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().children([Button::new().into()]))
        .unwrap();
    let root = runtime.graph().root().unwrap();

    assert_eq!(
        runtime.update(StackPanel::new().children([
            Button::new().element_ref(&reference).into(),
            Button::new().element_ref(&reference).into(),
        ]),),
        Err(UpdateError::Graph(GraphError::DuplicateReference))
    );
    assert_eq!(reference.get(), None);
    assert_eq!(runtime.graph().root(), Some(root));
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap()
            .len(),
        1
    );
}
