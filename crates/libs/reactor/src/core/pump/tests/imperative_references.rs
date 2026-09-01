use super::super::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn focus_count(runtime: &RecordingRuntime) -> usize {
    runtime
        .commands()
        .iter()
        .flatten()
        .filter(|command| matches!(command, Command::Focus { .. }))
        .count()
}

fn swap_chain_panel_view(key: &'static str, reference: &ElementRef<SwapChainPanel>) -> View {
    StackPanel::new().keyed_children([KeyedView::new(
        key,
        SwapChainPanel::new().element_ref(reference),
    )])
}

fn image_view(key: &'static str, reference: &ElementRef<Image>) -> View {
    StackPanel::new().keyed_children([KeyedView::new(key, Image::new().element_ref(reference))])
}

fn composition_host_view(key: &'static str, reference: &ElementRef<Grid>) -> View {
    StackPanel::new().keyed_children([KeyedView::new(key, Grid::new().element_ref(reference))])
}

fn imperative_targets(
    focus: &ElementRef<TextBox>,
    webview: &ElementRef<WebView2>,
    surface: &ElementRef<SwapChainPanel>,
    image: &ElementRef<Image>,
    composition: &ElementRef<Grid>,
) -> View {
    StackPanel::new().children((
        TextBox::new().element_ref(focus),
        WebView2::new().element_ref(webview),
        SwapChainPanel::new().element_ref(surface),
        Image::new().element_ref(image),
        Grid::new().element_ref(composition),
    ))
}

#[test]
fn unmounted_reference_rejects_focus() {
    let reference = ElementRef::<TextBox>::new();
    assert!(!reference.request_focus());
}

#[test]
fn webview_initialization_uses_the_typed_imperative_path() {
    let result = Rc::new(RefCell::new(None));
    let reference = ElementRef::<WebView2>::new();
    assert!(!reference.request_core_web_view2(|_| {}));

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(WebView2::new().element_ref(&reference).into())
        .unwrap();
    let completed = Rc::clone(&result);
    assert!(reference.request_core_web_view2(move |value| {
        *completed.borrow_mut() = Some(value);
    }));

    assert_eq!(pump.process_imperatives(), Ok(1));
    assert_eq!(*result.borrow(), Some(Err(WebView2Error::Unavailable)));
    assert!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .any(|command| matches!(command, Command::InitializeWebView2 { .. }))
    );
}

#[test]
fn swap_chain_panel_observation_uses_the_typed_imperative_path() {
    let reference = ElementRef::<SwapChainPanel>::new();
    let _observation = reference.observe_surface(|_| {});

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(SwapChainPanel::new().element_ref(&reference).into())
        .unwrap();

    assert_eq!(pump.process_imperatives(), Ok(1));
    assert!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .any(|command| matches!(command, Command::ObserveSwapChainPanel { .. }))
    );
}

#[test]
fn image_integration_uses_the_typed_imperative_paths() {
    let result = Rc::new(RefCell::new(None));
    let reference = ElementRef::<Image>::new();
    assert!(!reference.request_set_native_source(None, |_| {}));
    let _observation = reference.observe_rasterization_scale(|_| {});

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Image::new().element_ref(&reference).into())
        .unwrap();
    let completed = Rc::clone(&result);
    assert!(reference.request_set_native_source(None, move |value| {
        *completed.borrow_mut() = Some(value);
    }));

    assert_eq!(pump.process_imperatives(), Ok(2));
    assert_eq!(*result.borrow(), Some(Ok(())));
    let mut commands = pump.runtime().commands().iter().flatten();
    assert!(
        commands
            .clone()
            .any(|command| matches!(command, Command::SetNativeImageSource { .. }))
    );
    assert!(commands.any(|command| matches!(command, Command::ObserveImageScale { .. })));
}

#[test]
fn composition_integration_uses_the_typed_imperative_paths() {
    let result = Rc::new(RefCell::new(None));
    let reference = ElementRef::<Grid>::new();
    let _observation = reference.observe_composition_host(|_| {});
    assert!(!reference.request_set_child_visual(None, |_| {}));

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Grid::new().element_ref(&reference).into())
        .unwrap();
    let completed = Rc::clone(&result);
    assert!(reference.request_set_child_visual(None, move |value| {
        *completed.borrow_mut() = Some(value);
    }));

    assert_eq!(pump.process_imperatives(), Ok(2));
    assert_eq!(*result.borrow(), Some(Ok(())));
    let mut commands = pump.runtime().commands().iter().flatten();
    assert!(
        commands
            .clone()
            .any(|command| matches!(command, Command::ObserveCompositionHost { .. }))
    );
    assert!(commands.any(|command| matches!(command, Command::SetCompositionChildVisual { .. })));
}

#[test]
fn accepted_one_shot_requests_complete_when_targets_are_removed() {
    let focus = ElementRef::<TextBox>::new();
    let webview = ElementRef::<WebView2>::new();
    let surface = ElementRef::<SwapChainPanel>::new();
    let image = ElementRef::<Image>::new();
    let composition = ElementRef::<Grid>::new();
    let completed = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(imperative_targets(
        &focus,
        &webview,
        &surface,
        &image,
        &composition,
    ))
    .unwrap();

    let log = Rc::clone(&completed);
    assert!(focus.request_focus_result(move |result| {
        assert_eq!(result, Err(FocusError::Unavailable));
        log.borrow_mut().push("focus");
    }));
    let log = Rc::clone(&completed);
    assert!(webview.request_core_web_view2(move |result| {
        assert!(matches!(result, Err(WebView2Error::Unavailable)));
        log.borrow_mut().push("webview");
    }));
    let log = Rc::clone(&completed);
    assert!(surface.request_clear_swap_chain(move |result| {
        assert_eq!(result, Err(SwapChainPanelError::Unavailable));
        log.borrow_mut().push("surface");
    }));
    let log = Rc::clone(&completed);
    assert!(image.request_set_native_source(None, move |result| {
        assert_eq!(result, Err(ImageSourceError::Unavailable));
        log.borrow_mut().push("image");
    }));
    let log = Rc::clone(&completed);
    assert!(composition.request_set_child_visual(None, move |result| {
        assert_eq!(result, Err(CompositionHostError::Unavailable));
        log.borrow_mut().push("composition");
    }));

    pump.update_view(TextBlock::new().into()).unwrap();
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(
        &*completed.borrow(),
        &["focus", "webview", "surface", "image", "composition"]
    );
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(completed.borrow().len(), 5);
}

#[test]
fn accepted_one_shot_request_completes_on_window_shutdown() {
    let reference = ElementRef::<TextBox>::new();
    let completed = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    let count = Rc::clone(&completed);
    assert!(reference.request_focus_result(move |result| {
        assert_eq!(result, Err(FocusError::Unavailable));
        count.set(count.get() + 1);
    }));

    pump.shutdown();
    assert_eq!(completed.get(), 1);
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(completed.get(), 1);
}

#[test]
fn accepted_one_shot_request_completes_for_a_stale_window_identity() {
    let reference = ElementRef::<TextBox>::new();
    let completed = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    let count = Rc::clone(&completed);
    assert!(reference.request_focus_result(move |result| {
        assert_eq!(result, Err(FocusError::Unavailable));
        count.set(count.get() + 1);
    }));

    pump.identity = pump.identity.next();
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(completed.get(), 1);
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(completed.get(), 1);
}

#[test]
fn native_apply_failure_completes_the_failed_command_and_batch_tail() {
    let reference = ElementRef::<TextBox>::new();
    let completed = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();

    for index in 0..3 {
        let completed = Rc::clone(&completed);
        assert!(reference.request_focus_result(move |result| {
            completed.borrow_mut().push((index, result));
        }));
    }
    pump.runtime_mut().fail_at(1);

    assert!(matches!(
        pump.process_imperatives(),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(
        &*completed.borrow(),
        &[
            (0, Ok(true)),
            (1, Err(FocusError::Unavailable)),
            (2, Err(FocusError::Unavailable)),
        ]
    );
    assert_eq!(pump.process_imperatives(), Err(PumpError::Poisoned));
    assert_eq!(completed.borrow().len(), 3);
}

#[derive(Clone)]
struct ObserverRecreationInput {
    composition: ElementRef<Grid>,
    effects: Rc<Cell<usize>>,
    image: ElementRef<Image>,
    sender: Rc<RefCell<Option<LocalSender<()>>>>,
    surface: ElementRef<SwapChainPanel>,
}

impl PartialEq for ObserverRecreationInput {
    fn eq(&self, other: &Self) -> bool {
        self.composition == other.composition
            && Rc::ptr_eq(&self.effects, &other.effects)
            && self.image == other.image
            && Rc::ptr_eq(&self.sender, &other.sender)
            && self.surface == other.surface
    }
}

struct ObserverRecreation {
    recreated: bool,
}

impl Component for ObserverRecreation {
    type Input = ObserverRecreationInput;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self { recreated: false }
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.recreated = true;
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let effects = Rc::clone(&input.effects);
        let surface = input.surface.clone();
        context.use_effect("observe-surface", (), move || {
            effects.set(effects.get() + 1);
            let observation = surface.observe_surface(|_| {});
            Some(Box::new(move || drop(observation)))
        });
        let effects = Rc::clone(&input.effects);
        let image = input.image.clone();
        context.use_effect("observe-image", (), move || {
            effects.set(effects.get() + 1);
            let observation = image.observe_rasterization_scale(|_| {});
            Some(Box::new(move || drop(observation)))
        });
        let effects = Rc::clone(&input.effects);
        let composition = input.composition.clone();
        context.use_effect("observe-composition", (), move || {
            effects.set(effects.get() + 1);
            let observation = composition.observe_composition_host(|_| {});
            Some(Box::new(move || drop(observation)))
        });

        let generation = if self.recreated { "second" } else { "first" };
        StackPanel::new().keyed_children([
            KeyedView::new(
                format!("surface-{generation}"),
                SwapChainPanel::new().element_ref(&input.surface),
            ),
            KeyedView::new(
                format!("image-{generation}"),
                Image::new().element_ref(&input.image),
            ),
            KeyedView::new(
                format!("composition-{generation}"),
                Grid::new().element_ref(&input.composition),
            ),
        ])
    }
}

#[test]
fn constant_effect_observations_follow_all_recreated_references() {
    let effects = Rc::new(Cell::new(0));
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ObserverRecreation>(
        ObserverRecreationInput {
            composition: ElementRef::new(),
            effects: Rc::clone(&effects),
            image: ElementRef::new(),
            sender: Rc::clone(&sender),
            surface: ElementRef::new(),
        },
    ))
    .unwrap();
    assert_eq!(effects.get(), 3);
    assert_eq!(pump.process_imperatives(), Ok(3));

    assert!(sender.borrow().as_ref().unwrap().send(()));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(effects.get(), 3);
    assert_eq!(pump.process_imperatives(), Ok(6));

    let surface_nodes = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .filter_map(|command| match command {
            Command::ObserveSwapChainPanel { node, .. } => Some(*node),
            _ => None,
        })
        .collect::<Vec<_>>();
    let image_nodes = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .filter_map(|command| match command {
            Command::ObserveImageScale { node, .. } => Some(*node),
            _ => None,
        })
        .collect::<Vec<_>>();
    let composition_nodes = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .filter_map(|command| match command {
            Command::ObserveCompositionHost { node, .. } => Some(*node),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(surface_nodes.len(), 2);
    assert_ne!(surface_nodes[0], surface_nodes[1]);
    assert_eq!(image_nodes.len(), 2);
    assert_ne!(image_nodes[0], image_nodes[1]);
    assert_eq!(composition_nodes.len(), 2);
    assert_ne!(composition_nodes[0], composition_nodes[1]);
}

#[test]
fn swap_chain_panel_observation_follows_structural_recreation() {
    let observed = Rc::new(Cell::new(0));
    let reference = ElementRef::<SwapChainPanel>::new();
    let callback_observed = Rc::clone(&observed);
    let _observation = reference.observe_surface(move |_| {
        callback_observed.set(callback_observed.get() + 1);
    });

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(swap_chain_panel_view("first", &reference))
        .unwrap();
    assert!(reference.request_clear_swap_chain(|_| {}));
    assert_eq!(pump.process_imperatives(), Ok(2));
    let (old_node, old_callback) = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::ObserveSwapChainPanel { node, callback, .. } => {
                Some((*node, callback.clone()))
            }
            _ => None,
        })
        .unwrap();

    pump.update_view(swap_chain_panel_view("second", &reference))
        .unwrap();
    assert!(!old_callback.call(SwapChainPanelEvent::Rendering));
    assert_eq!(observed.get(), 0);
    assert_eq!(pump.process_imperatives(), Ok(2));
    let (new_node, new_callback) = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::ObserveSwapChainPanel { node, callback, .. } => {
                Some((*node, callback.clone()))
            }
            _ => None,
        })
        .unwrap();
    assert_ne!(new_node, old_node);
    assert!(new_callback.call(SwapChainPanelEvent::Rendering));
    assert_eq!(observed.get(), 1);
    assert_eq!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .filter(|command| matches!(command, Command::SetSwapChain { .. }))
            .count(),
        1
    );

    pump.update_view(TextBlock::new().into()).unwrap();
    assert!(!new_callback.call(SwapChainPanelEvent::Rendering));
    assert_eq!(observed.get(), 1);
    pump.update_view(swap_chain_panel_view("third", &reference))
        .unwrap();
    assert_eq!(pump.process_imperatives(), Ok(2));
}

#[test]
fn image_scale_observation_follows_structural_recreation() {
    let observed = Rc::new(Cell::new(0));
    let reference = ElementRef::<Image>::new();
    let callback_observed = Rc::clone(&observed);
    let _observation = reference.observe_rasterization_scale(move |_| {
        callback_observed.set(callback_observed.get() + 1);
    });

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(image_view("first", &reference)).unwrap();
    assert!(reference.request_set_native_source(None, |_| {}));
    assert_eq!(pump.process_imperatives(), Ok(2));
    let (old_node, old_callback) = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::ObserveImageScale { node, callback, .. } => Some((*node, callback.clone())),
            _ => None,
        })
        .unwrap();

    pump.update_view(image_view("second", &reference)).unwrap();
    assert!(!old_callback.call(2.0));
    assert_eq!(observed.get(), 0);
    assert_eq!(pump.process_imperatives(), Ok(2));
    let (new_node, new_callback) = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::ObserveImageScale { node, callback, .. } => Some((*node, callback.clone())),
            _ => None,
        })
        .unwrap();
    assert_ne!(new_node, old_node);
    assert!(new_callback.call(2.0));
    assert_eq!(observed.get(), 1);
    assert_eq!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .filter(|command| matches!(command, Command::SetNativeImageSource { .. }))
            .count(),
        1
    );

    pump.update_view(TextBlock::new().into()).unwrap();
    assert!(!new_callback.call(2.0));
    assert_eq!(observed.get(), 1);
    pump.update_view(image_view("third", &reference)).unwrap();
    assert_eq!(pump.process_imperatives(), Ok(2));
}

#[test]
fn composition_host_observation_follows_structural_recreation() {
    let observed = Rc::new(Cell::new(0));
    let reference = ElementRef::<Grid>::new();
    let callback_observed = Rc::clone(&observed);
    let _observation = reference.observe_composition_host(move |_| {
        callback_observed.set(callback_observed.get() + 1);
    });

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(composition_host_view("first", &reference))
        .unwrap();
    assert!(reference.request_set_child_visual(None, |_| {}));
    assert_eq!(pump.process_imperatives(), Ok(2));
    let (old_node, old_callback) = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::ObserveCompositionHost { node, callback, .. } => {
                Some((*node, callback.clone()))
            }
            _ => None,
        })
        .unwrap();

    pump.update_view(composition_host_view("second", &reference))
        .unwrap();
    let event = CompositionHostEvent::Metrics {
        width: 1.0,
        height: 1.0,
        scale: 1.0,
    };
    assert!(!old_callback.call(event.clone()));
    assert_eq!(observed.get(), 0);
    assert_eq!(pump.process_imperatives(), Ok(2));
    let (new_node, new_callback) = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::ObserveCompositionHost { node, callback, .. } => {
                Some((*node, callback.clone()))
            }
            _ => None,
        })
        .unwrap();
    assert_ne!(new_node, old_node);
    assert!(new_callback.call(event.clone()));
    assert_eq!(observed.get(), 1);
    assert_eq!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .filter(|command| matches!(command, Command::SetCompositionChildVisual { .. }))
            .count(),
        1
    );

    pump.update_view(TextBlock::new().into()).unwrap();
    assert!(!new_callback.call(event));
    assert_eq!(observed.get(), 1);
    pump.update_view(composition_host_view("third", &reference))
        .unwrap();
    assert_eq!(pump.process_imperatives(), Ok(2));
}

#[test]
fn observation_handle_controls_registration_lifetime() {
    let reference = ElementRef::<Image>::new();
    let observation = reference.observe_rasterization_scale(|_| {});
    drop(observation);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Image::new().element_ref(&reference).into())
        .unwrap();
    assert_eq!(pump.process_imperatives(), Ok(0));

    let observed = Rc::new(Cell::new(false));
    let callback_observed = Rc::clone(&observed);
    let observation = reference.observe_rasterization_scale(move |_| {
        callback_observed.set(true);
    });
    assert_eq!(pump.process_imperatives(), Ok(1));
    let callback = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::ObserveImageScale { callback, .. } => Some(callback.clone()),
            _ => None,
        })
        .unwrap();
    drop(observation);
    assert!(!callback.call(2.0));
    assert!(!observed.get());
    assert_eq!(pump.process_imperatives(), Ok(1));
    assert!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .any(|command| matches!(command, Command::RevokeObservation { .. }))
    );
}

#[test]
fn multiple_observations_share_an_element_without_overwriting() {
    let first_count = Rc::new(Cell::new(0));
    let second_count = Rc::new(Cell::new(0));
    let reference = ElementRef::<Image>::new();
    let first_callback = Rc::clone(&first_count);
    let first = reference.observe_rasterization_scale(move |_| {
        first_callback.set(first_callback.get() + 1);
    });
    let second_callback = Rc::clone(&second_count);
    let _second = reference.observe_rasterization_scale(move |_| {
        second_callback.set(second_callback.get() + 1);
    });
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Image::new().element_ref(&reference).into())
        .unwrap();

    assert_eq!(pump.process_imperatives(), Ok(2));
    let observations = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .filter_map(|command| match command {
            Command::ObserveImageScale {
                observation,
                callback,
                ..
            } => Some((*observation, callback.clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(observations.len(), 2);
    assert_ne!(observations[0].0, observations[1].0);
    assert!(observations[0].1.call(1.0));
    assert!(observations[1].1.call(1.0));

    drop(first);
    assert_eq!(pump.process_imperatives(), Ok(1));
    assert!(!observations[0].1.call(1.0));
    assert!(observations[1].1.call(1.0));
    assert_eq!(first_count.get(), 1);
    assert_eq!(second_count.get(), 2);
}

struct ObservationCaptureDrop(Rc<Cell<bool>>);

impl Drop for ObservationCaptureDrop {
    fn drop(&mut self) {
        self.0.set(true);
    }
}

#[test]
fn observation_callback_can_capture_its_reference_without_a_cycle() {
    let dropped = Rc::new(Cell::new(false));
    let reference = ElementRef::<Image>::new();
    let callback_reference = reference.clone();
    let capture = ObservationCaptureDrop(Rc::clone(&dropped));
    let observation = reference.observe_rasterization_scale(move |_| {
        let _ = &callback_reference;
        let _ = &capture;
    });

    drop(reference);
    assert!(!dropped.get());
    drop(observation);
    assert!(dropped.get());
}

#[test]
fn focus_completion_reports_the_native_result() {
    let result = Rc::new(RefCell::new(None));
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    let completed = Rc::clone(&result);
    assert!(reference.request_focus_result(move |value| {
        *completed.borrow_mut() = Some(value);
    }));

    assert_eq!(pump.process_imperatives(), Ok(1));
    assert_eq!(*result.borrow(), Some(Ok(true)));
}

#[test]
fn imperative_queue_rejects_excess_work_and_drains_with_a_budget() {
    const WORK_BUDGET: usize = 64;
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();

    for _ in 0..4_096 {
        assert!(reference.request_focus());
    }
    assert!(!reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(WORK_BUDGET));
    assert!(pump.native_work_pending());
    assert_eq!(focus_count(pump.runtime()), WORK_BUDGET);
}

#[test]
fn mount_binds_only_after_successful_native_apply() {
    let reference = ElementRef::<TextBox>::new();
    let mut failed = RecordingRuntime::default();
    failed.fail_at(0);
    let mut pump = Pump::new(failed);
    assert!(matches!(
        pump.mount(TextBox::new().element_ref(&reference).into()),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(!reference.request_focus());

    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    assert!(reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(1));
    assert_eq!(focus_count(pump.runtime()), 1);
}

#[test]
fn reference_swap_and_removal_unbind_the_published_owner() {
    let first = ElementRef::<TextBox>::new();
    let second = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&first).into())
        .unwrap();

    pump.update(TextBox::new().element_ref(&second).into())
        .unwrap();
    assert!(!first.request_focus());
    assert!(second.request_focus());

    pump.update(TextBlock::new().into()).unwrap();
    assert!(!second.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(0));
}

#[test]
fn stale_focus_request_is_not_applied_after_replacement_and_window_epoch_change() {
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    assert!(reference.request_focus());
    pump.update(TextBlock::new().into()).unwrap();
    assert_eq!(pump.process_imperatives(), Ok(0));

    pump.shutdown();
    assert!(!reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(focus_count(pump.runtime()), 0);
}

#[test]
fn failed_update_does_not_publish_candidate_reference() {
    let current = ElementRef::<TextBox>::new();
    let candidate = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&current).into())
        .unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update(
            TextBox::new()
                .text("native mutation")
                .element_ref(&candidate)
                .into()
        ),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(current.request_focus());
    assert!(!candidate.request_focus());
}

#[test]
fn failed_planning_does_not_publish_candidate_reference() {
    let candidate = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TextBlock::new().into()).unwrap();

    assert_eq!(
        pump.update_view(View::fragment((
            TextBox::new().element_ref(&candidate),
            TextBlock::new(),
        ))),
        Err(PumpError::StructureUnsupported)
    );
    assert!(!candidate.request_focus());
}

#[test]
fn shutdown_and_window_close_clear_references() {
    let shutdown = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&shutdown).into())
        .unwrap();
    pump.shutdown();
    assert!(!shutdown.request_focus());

    let closed = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&closed).into())
        .unwrap();
    pump.native_window_closed();
    assert!(!closed.request_focus());
}

#[test]
fn windows_have_isolated_imperative_queues() {
    let first = ElementRef::<TextBox>::new();
    let second = ElementRef::<TextBox>::new();
    let mut left = Pump::new(RecordingRuntime::default());
    let mut right = Pump::new(RecordingRuntime::default());
    left.mount(TextBox::new().element_ref(&first).into())
        .unwrap();
    right
        .mount(TextBox::new().element_ref(&second).into())
        .unwrap();

    assert!(first.request_focus());
    assert_eq!(right.process_imperatives(), Ok(0));
    assert_eq!(left.process_imperatives(), Ok(1));
    assert_eq!(focus_count(right.runtime()), 0);
    assert_eq!(focus_count(left.runtime()), 1);
}

#[test]
fn one_reference_cannot_own_two_published_elements() {
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(StackPanel::new().children((
            TextBox::new().element_ref(&reference),
            TextBox::new().element_ref(&reference),
        ))),
        Err(PumpError::DuplicateElementRef)
    );
    assert!(!reference.request_focus());

    let mut first = Pump::new(RecordingRuntime::default());
    first
        .mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    let mut second = Pump::new(RecordingRuntime::default());
    assert_eq!(
        second.mount(TextBox::new().element_ref(&reference).into()),
        Err(PumpError::DuplicateElementRef)
    );
    assert!(reference.request_focus());
}

#[derive(Clone)]
struct DuplicateOwnerInput {
    dropped: Rc<Cell<usize>>,
    reference: ElementRef<TextBox>,
}

impl PartialEq for DuplicateOwnerInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.dropped, &other.dropped) && self.reference == other.reference
    }
}

struct DuplicateOwner(DuplicateOwnerInput);

impl Drop for DuplicateOwner {
    fn drop(&mut self) {
        self.0.dropped.set(self.0.dropped.get() + 1);
    }
}

impl Component for DuplicateOwner {
    type Message = ();
    type Input = DuplicateOwnerInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().children((
            TextBox::new().element_ref(&self.0.reference),
            TextBox::new().element_ref(&self.0.reference),
        ))
    }
}

#[test]
fn duplicate_reference_validation_removes_component_reservations() {
    let dropped = Rc::new(Cell::new(0));
    let reference = ElementRef::new();
    let mut pump = Pump::new(RecordingRuntime::default());

    assert_eq!(
        pump.mount_view(View::component::<DuplicateOwner>(DuplicateOwnerInput {
            dropped: Rc::clone(&dropped),
            reference,
        })),
        Err(PumpError::DuplicateElementRef)
    );
    assert_eq!(dropped.get(), 1);
}

struct PropReference;

impl Component for PropReference {
    type Message = ();
    type Input = ElementRef<TextBox>;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, reference: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBox::new().element_ref(reference).into()
    }
}

#[test]
fn duplicate_reference_recomposes_staged_component_input() {
    let shared = ElementRef::<TextBox>::new();
    let mut owner = Pump::new(RecordingRuntime::default());
    owner
        .mount(TextBox::new().element_ref(&shared).into())
        .unwrap();

    let original = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<PropReference>(original.clone()))
        .unwrap();

    assert_eq!(
        pump.update_view(View::component::<PropReference>(shared.clone())),
        Err(PumpError::DuplicateElementRef)
    );
    assert!(original.request_focus());
    assert!(shared.request_focus());
    assert_eq!(owner.process_imperatives(), Ok(1));
    assert_eq!(pump.process_imperatives(), Ok(1));

    owner.shutdown();
    pump.update_view(View::component::<PropReference>(shared.clone()))
        .unwrap();
    assert!(!original.request_focus());
    assert!(shared.request_focus());
}

#[derive(Clone)]
struct EffectInput {
    accepted: Rc<Cell<bool>>,
    reference: ElementRef<TextBox>,
}

impl PartialEq for EffectInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.accepted, &other.accepted) && self.reference == other.reference
    }
}

struct EffectFocus;

impl Component for EffectFocus {
    type Message = ();
    type Input = EffectInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let reference = input.reference.clone();
        let accepted = Rc::clone(&input.accepted);
        context.use_effect("focus", (), move || {
            accepted.set(reference.request_focus());
            None
        });
        TextBox::new().element_ref(&input.reference).into()
    }
}

#[test]
fn effect_setup_can_enqueue_focus_after_publication() {
    let accepted = Rc::new(Cell::new(false));
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<EffectFocus>(EffectInput {
        accepted: Rc::clone(&accepted),
        reference,
    }))
    .unwrap();

    assert!(accepted.get());
    assert_eq!(pump.process_imperatives(), Ok(1));
    assert_eq!(focus_count(pump.runtime()), 1);
}

#[derive(Clone)]
struct LocalInput {
    exposed: Rc<RefCell<Option<(ElementRef<TextBox>, ElementRef<TextBox>)>>>,
    sender: Rc<RefCell<Option<LocalSender<bool>>>>,
}

impl PartialEq for LocalInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.exposed, &other.exposed) && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct LocalReference {
    first: ElementRef<TextBox>,
    second: ElementRef<TextBox>,
    use_second: bool,
}

impl Component for LocalReference {
    type Message = bool;
    type Input = LocalInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let first = ElementRef::new();
        let second = ElementRef::new();
        *input.exposed.borrow_mut() = Some((first.clone(), second.clone()));
        *input.sender.borrow_mut() = Some(context.sender());
        Self {
            first,
            second,
            use_second: false,
        }
    }

    fn update(&mut self, message: bool, _context: &ComponentContext<Self>) {
        self.use_second = message;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let reference = if self.use_second {
            &self.second
        } else {
            &self.first
        };
        TextBox::new().element_ref(reference).into()
    }
}

#[test]
fn local_component_fast_path_commits_reference_changes() {
    let exposed = Rc::new(RefCell::new(None));
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<LocalReference>(LocalInput {
        exposed: Rc::clone(&exposed),
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    let (first, second) = exposed.borrow().clone().unwrap();
    assert!(first.request_focus());
    assert!(!second.request_focus());
    pump.process_imperatives().unwrap();

    assert!(sender.borrow().as_ref().unwrap().send(true));
    pump.dispatch_components(1).unwrap();
    assert!(!first.request_focus());
    assert!(second.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(1));
}

struct Removal {
    reference: ElementRef<TextBox>,
    removed: bool,
}

#[derive(Clone)]
struct RemovalInput(Rc<RefCell<Option<(ElementRef<TextBox>, LocalSender<()>)>>>);

impl PartialEq for RemovalInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Component for Removal {
    type Message = ();
    type Input = RemovalInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let reference = ElementRef::new();
        *input.0.borrow_mut() = Some((reference.clone(), context.sender()));
        Self {
            reference,
            removed: false,
        }
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.removed = true;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if self.removed {
            TextBlock::new().into()
        } else {
            TextBox::new().element_ref(&self.reference).into()
        }
    }
}

#[test]
fn component_publication_precedes_queued_imperative_work() {
    let exposed = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Removal>(RemovalInput(Rc::clone(
        &exposed,
    ))))
    .unwrap();
    let (reference, sender) = exposed.borrow().clone().unwrap();

    assert!(reference.request_focus());
    assert!(sender.send(()));
    pump.dispatch_components(1).unwrap();
    assert!(!reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(focus_count(pump.runtime()), 0);
}

struct BacklogRemoval {
    reference: ElementRef<TextBox>,
    removed: bool,
}

#[derive(Clone)]
struct BacklogInput(Rc<RefCell<Option<(ElementRef<TextBox>, LocalSender<bool>)>>>);

impl PartialEq for BacklogInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Component for BacklogRemoval {
    type Message = bool;
    type Input = BacklogInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let reference = ElementRef::new();
        *input.0.borrow_mut() = Some((reference.clone(), context.sender()));
        Self {
            reference,
            removed: false,
        }
    }

    fn update(&mut self, remove: bool, _context: &ComponentContext<Self>) {
        self.removed |= remove;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if self.removed {
            TextBlock::new().into()
        } else {
            TextBox::new().element_ref(&self.reference).into()
        }
    }
}

#[test]
fn imperative_work_waits_for_the_component_message_backlog() {
    let exposed = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<BacklogRemoval>(BacklogInput(Rc::clone(
        &exposed,
    ))))
    .unwrap();
    let (reference, sender) = exposed.borrow().clone().unwrap();
    assert!(reference.request_focus());
    for _ in 0..64 {
        assert!(sender.send(false));
    }
    assert!(sender.send(true));

    assert_eq!(pump.dispatch_components(64), Ok(64));
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert!(reference.request_focus());

    assert_eq!(pump.dispatch_components(64), Ok(1));
    assert!(!reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(focus_count(pump.runtime()), 0);
}
