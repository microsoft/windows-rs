use super::*;

#[test]
fn reference_only_changes_reconcile_without_native_mutations() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Button::new());
    let button = runtime.graph().root().unwrap();

    assert!(
        runtime
            .update(Button::new().element_ref(&reference))
            .is_empty()
    );
    assert_eq!(reference.get(), Some(button));

    assert!(runtime.update(Button::new()).is_empty());
    assert_eq!(reference.get(), None);
}

struct HoldingImperativeAdapter {
    inner: RecordingAdapter,
    commands: Rc<RefCell<Vec<ImperativeRequest>>>,
}

impl Adapter for HoldingImperativeAdapter {
    fn peek_native_event(&self) -> Option<NativeEvent> {
        self.inner.peek_native_event()
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn apply(&mut self, mutations: &[Mutation]) {
        self.inner.apply(mutations);
    }

    fn imperative(&mut self, request: ImperativeRequest) {
        self.commands.borrow_mut().push(request);
    }
}

#[test]
fn typed_references_cover_all_imperative_controls() {
    let text_box = ElementRef::<TextBox>::new();
    let grid = ElementRef::<Grid>::new();
    let image = ElementRef::<Image>::new();
    let webview = ElementRef::<WebView2>::new();
    let swap_chain = ElementRef::<SwapChainPanel>::new();

    let _: View = TextBox::new().text("").element_ref(&text_box).into();
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
    runtime.update(TextBox::new().text("").element_ref(&reference));
    let completed = Rc::clone(&result);
    assert!(reference.request_focus_result(move |value| {
        completed.set(Some(value));
    }));
    runtime.dispatch_native_events();
    assert_eq!(result.get(), Some(Ok(true)));
}

#[test]
fn element_reference_queues_automation_notification() {
    let commands = Rc::new(RefCell::new(Vec::new()));
    let reference = ElementRef::<Button>::new();
    let result = Rc::new(Cell::new(None));
    assert!(
        !reference.request_notification(AutomationNotification::new("unbound")),
        "unbound references must not queue notifications"
    );
    let mut runtime = Runtime::new(HoldingImperativeAdapter {
        inner: RecordingAdapter::default(),
        commands: Rc::clone(&commands),
    });
    runtime.update(Button::new().element_ref(&reference));
    let completed = Rc::clone(&result);
    assert!(
        reference.request_notification_result(
            AutomationNotification::new("Saved")
                .kind(AutomationNotificationKind::ActionCompleted)
                .processing(AutomationNotificationProcessing::All)
                .activity_id("save"),
            move |value| completed.set(Some(value)),
        )
    );
    runtime.dispatch_native_events();

    let ImperativeRequest::Notify {
        notification,
        completion,
        ..
    } = commands.borrow_mut().remove(0)
    else {
        panic!("expected notification request");
    };
    assert_eq!(&*notification.text, "Saved");
    assert_eq!(&*notification.activity_id, "save");
    assert_eq!(
        notification.kind,
        AutomationNotificationKind::ActionCompleted
    );
    assert_eq!(
        notification.processing,
        AutomationNotificationProcessing::All
    );
    completion.call(Ok(()));
    assert_eq!(result.get(), Some(Ok(())));

    let defaults = AutomationNotification::new("Default");
    assert_eq!(defaults.kind, AutomationNotificationKind::Other);
    assert_eq!(
        defaults.processing,
        AutomationNotificationProcessing::MostRecent
    );
    assert!(defaults.activity_id.is_empty());
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
    runtime.update(Image::new().element_ref(&reference));
    let callback_results = Rc::clone(&results);
    assert!(reference.request_set_native_source(None, move |result| {
        callback_results.borrow_mut().push(result);
    }));
    runtime.dispatch_native_events();
    runtime.update(Image::new().element_ref(&replacement));

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

    runtime.update(SwapChainPanel::new().element_ref(&reference));
    runtime.dispatch_native_events();
    let panel = reference.get().unwrap();
    let initial = last_binding();

    runtime.update(SwapChainPanel::new().element_ref(&reference).width(100.0));
    commands.borrow_mut().clear();
    let _second_observation = reference.observe_surface(|_| {});
    runtime.dispatch_native_events();
    assert_eq!(last_binding(), initial);
    assert_eq!(reference.get(), Some(panel));

    runtime.update(SwapChainPanel::new());
    runtime.dispatch_native_events();
    assert_eq!(reference.get(), None);
    assert_eq!(runtime.graph().root(), Some(panel));

    commands.borrow_mut().clear();
    runtime.update(SwapChainPanel::new().element_ref(&reference));
    runtime.dispatch_native_events();
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

    runtime.update(Grid::new().element_ref(&reference));
    runtime.dispatch_native_events();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));

    runtime.update(Grid::new());
    runtime.dispatch_native_events();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));

    runtime.update(Grid::new().element_ref(&reference));
    runtime.dispatch_native_events();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));

    drop(observation);
    runtime.dispatch_native_events();
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
    runtime.update(Grid::new().element_ref(&reference));

    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    assert!(!reference.request_focus());
    let observation = reference.observe_composition_host(|_| {});
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events();
    }
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));
    commands.borrow_mut().clear();

    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    runtime.update(Grid::new());
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events();
    }
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));
    commands.borrow_mut().clear();

    runtime.update(Grid::new().element_ref(&reference));
    runtime.dispatch_native_events();
    commands.borrow_mut().clear();
    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    drop(observation);
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events();
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
    runtime.update(
        Grid::new().keyed_children([keyed("observed", Grid::new().element_ref(&reference))]),
    );
    runtime.dispatch_native_events();

    runtime.update(Grid::new());

    assert_eq!(reference.get(), None);
    assert_eq!(runtime.dispatch_native_events(), 0);
}

#[test]
fn retiring_observed_object_forwards_queued_revocation() {
    let reference = ElementRef::<Grid>::new();
    let _observation = reference.observe_composition_host(|_| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        Grid::new().keyed_children([keyed(
            "observed",
            Grid::new()
                .element_ref(&reference)
                .exit_fade(Duration::from_millis(50)),
        )]),
    );
    runtime.dispatch_native_events();

    runtime.update(Grid::new());
    runtime.dispatch_native_events();

    assert_eq!(reference.get(), None);
    assert!(matches!(
        runtime.adapter().imperatives().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));
}

#[test]
fn dropping_runtime_clears_typed_reference_and_pending_completion() {
    let reference = ElementRef::<SwapChainPanel>::new();
    let results = Rc::new(RefCell::new(Vec::new()));
    {
        let mut runtime = Runtime::new(RecordingAdapter::default());
        runtime.update(SwapChainPanel::new().element_ref(&reference));
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
    runtime.update(Image::new().element_ref(&reference));
    for _ in 0..65 {
        assert!(reference.request_set_native_source(None, |_| {}));
    }
    wakes.set(0);

    runtime.dispatch_native_events();

    assert_eq!(wakes.get(), 1);
    runtime.dispatch_native_events();
    assert_eq!(runtime.adapter().imperatives().len(), 65);
}

#[test]
fn reference_transfers_are_order_independent() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(StackPanel::new().children([
        Button::new().into(),
        Button::new().element_ref(&reference).into(),
    ]));
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
            .is_empty()
    );
    assert_eq!(reference.get(), Some(children[0]));

    assert!(
        runtime
            .update(StackPanel::new().children([
                Button::new().into(),
                Button::new().element_ref(&reference).into(),
            ]),)
            .is_empty()
    );
    assert_eq!(reference.get(), Some(children[1]));
}

#[test]
fn references_survive_keyed_replacement_and_clear_on_destruction() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(
        Grid::new().keyed_children([keyed("focused", Button::new().element_ref(&reference))]),
    );
    let root = runtime.graph().root().unwrap();
    let child = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_eq!(reference.get(), Some(child));

    runtime.update(Grid::new().keyed_children([keyed(
        "focused",
        TextBox::new().text("replacement").element_ref(&reference),
    )]));
    let replacement = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_ne!(replacement, child);
    assert_eq!(reference.get(), Some(replacement));
    assert_eq!(runtime.graph().kind(replacement), Some(ObjectType::TextBox));

    runtime.update(Grid::new());
    assert_eq!(reference.get(), None);
}

#[test]
#[should_panic(expected = "element reference declared more than once")]
fn duplicate_references_panic() {
    let reference: ElementRef = ElementRef::default();
    Runtime::new(RecordingAdapter::default()).update(StackPanel::new().children([
        Button::new().element_ref(&reference).into(),
        Button::new().element_ref(&reference).into(),
    ]));
}
