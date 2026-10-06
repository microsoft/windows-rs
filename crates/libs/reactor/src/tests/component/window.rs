use super::*;

#[derive(Default)]
struct TestUiServices {
    activated: Cell<usize>,
    closed: Cell<usize>,
    operations: Cell<usize>,
    publications: RefCell<Vec<WindowPublication>>,
    requests: RefCell<Vec<(ComponentNode, WindowPolicy)>>,
}

impl ComponentUiServices for TestUiServices {
    fn open_window(&self, root: ComponentNode, policy: WindowPolicy) -> bool {
        self.requests.borrow_mut().push((root, policy));
        true
    }

    fn publish_window(&self, publication: WindowPublication) {
        self.publications.borrow_mut().push(publication);
    }

    fn activate_window(&self) -> bool {
        self.activated.set(self.activated.get() + 1);
        true
    }

    fn close_window(&self) -> bool {
        self.closed.set(self.closed.get() + 1);
        true
    }

    fn run_window(&self, operation: Box<dyn FnOnce(*mut core::ffi::c_void)>) -> bool {
        self.operations.set(self.operations.get() + 1);
        operation(std::ptr::dangling_mut::<core::ffi::c_void>());
        true
    }
}

struct WindowContent;

impl Component for WindowContent {
    type Input = String;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(input.as_str()).into()
    }
}

#[derive(Clone)]
struct WindowRequesterInput {
    accepted: Rc<Cell<Option<bool>>>,
}

impl PartialEq for WindowRequesterInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.accepted, &other.accepted)
    }
}

struct WindowRequester {
    accepted: Rc<Cell<Option<bool>>>,
}

struct WindowPublisher;

impl Component for WindowPublisher {
    type Input = bool;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if *input {
            context.window_title("Published");
            context.window_visuals(
                WindowVisuals::new()
                    .theme(WindowTheme::Dark)
                    .backdrop(WindowBackdrop::Mica)
                    .client_size(640.0, 480.0)
                    .constraints(WindowConstraints {
                        min_width: Some(320.0),
                        min_height: Some(240.0),
                        max_width: Some(1280.0),
                        max_height: Some(960.0),
                    }),
            );
            context.on_color_scheme(|_| {});
            context.on_window_size(|_| {});
            context.on_window_placement(|_| {});
        }
        TextBlock::new().text("Window publisher").into()
    }
}

struct WindowTitlePublisher;

impl Component for WindowTitlePublisher {
    type Input = &'static str;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title(*input);
        TextBlock::new().text(*input).into()
    }
}

enum WindowRequestMessage {
    Activate,
    Close,
    Open,
    Run,
    Window(usize),
}

impl Component for WindowRequester {
    type Input = WindowRequesterInput;
    type Message = WindowRequestMessage;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            accepted: Rc::clone(&input.accepted),
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        let accepted = match message {
            WindowRequestMessage::Activate => context.activate_window(),
            WindowRequestMessage::Close => context.close_window(),
            WindowRequestMessage::Open => context.open_window_with_policy::<WindowContent>(
                "secondary".to_string(),
                WindowPolicy::new()
                    .title("Secondary")
                    .client_size(320.0, 200.0),
            ),
            WindowRequestMessage::Run => {
                context.run_window(|window| WindowRequestMessage::Window(window.as_raw() as usize))
            }
            WindowRequestMessage::Window(raw) => raw == 1,
        };
        self.accepted.set(Some(accepted));
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("primary").into()
    }
}

#[test]
fn ui_local_services_accept_typed_window_requests() {
    let accepted = Rc::new(Cell::new(None));
    let input = WindowRequesterInput {
        accepted: Rc::clone(&accepted),
    };
    let ui_services = Rc::new(TestUiServices::default());
    let mut host = ComponentHost::mount_with_all_services(
        RecordingAdapter::default(),
        Arc::new(DefaultComponentServices),
        ui_services.clone(),
        [component::<WindowRequester>("requester", input.clone())],
    )
    .unwrap();
    let sender = host
        .sender::<WindowRequester>(&Key::from("requester"))
        .unwrap();

    assert!(sender.send(WindowRequestMessage::Open));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(accepted.get(), Some(true));
    let (root, policy) = ui_services.requests.borrow_mut().pop().unwrap();
    assert_eq!(
        policy,
        WindowPolicy::new()
            .title("Secondary")
            .client_size(320.0, 200.0)
    );
    let secondary = ComponentHost::mount(RecordingAdapter::default(), [root]).unwrap();
    let root = secondary
        .reference_at(&[Key::from("root")])
        .unwrap()
        .get()
        .unwrap();
    assert_eq!(
        secondary.runtime().graph().kind(root),
        Some(ObjectType::TextBlock)
    );
    assert!(sender.send(WindowRequestMessage::Activate));
    assert!(sender.send(WindowRequestMessage::Close));
    assert_eq!(host.drain(2).unwrap().dispatched, 2);
    assert_eq!(ui_services.activated.get(), 1);
    assert_eq!(ui_services.closed.get(), 1);
    assert!(sender.send(WindowRequestMessage::Run));
    assert_eq!(host.drain(2).unwrap().dispatched, 2);
    assert_eq!(accepted.get(), Some(true));
    assert_eq!(ui_services.operations.get(), 1);

    let mut headless = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<WindowRequester>("requester", input)],
    )
    .unwrap();
    let sender = headless
        .sender::<WindowRequester>(&Key::from("requester"))
        .unwrap();
    assert!(sender.send(WindowRequestMessage::Open));
    assert_eq!(headless.drain(1).unwrap().dispatched, 1);
    assert_eq!(accepted.get(), Some(false));
}

#[test]
fn component_window_declarations_follow_scope_updates() {
    let ui_services = Rc::new(TestUiServices::default());
    let mut host = ComponentHost::mount_with_all_services(
        RecordingAdapter::default(),
        Arc::new(DefaultComponentServices),
        ui_services.clone(),
        [component::<WindowPublisher>("publisher", true)],
    )
    .unwrap();

    let published = ui_services.publications.borrow().last().unwrap().clone();
    assert_eq!(published.title.as_deref(), Some("Published"));
    assert_eq!(
        published.visuals,
        Some(
            WindowVisuals::new()
                .theme(WindowTheme::Dark)
                .backdrop(WindowBackdrop::Mica)
                .client_size(640.0, 480.0)
                .constraints(WindowConstraints {
                    min_width: Some(320.0),
                    min_height: Some(240.0),
                    max_width: Some(1280.0),
                    max_height: Some(960.0),
                })
        )
    );
    assert!(published.on_color_scheme.is_some());
    assert!(published.on_size.is_some());
    assert!(published.on_placement.is_some());

    host.update_input::<WindowPublisher>(&Key::from("publisher"), false)
        .unwrap();
    assert_eq!(
        ui_services.publications.borrow().last().unwrap(),
        &WindowPublication::default()
    );
}

struct PlacementPublisher;

impl Component for PlacementPublisher {
    type Input = bool;
    type Message = ();

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, duplicate: &bool, context: &mut ViewContext<Self>) -> View {
        context.on_window_placement(|_| {});
        if *duplicate {
            context.on_window_placement(|_| {});
        }
        "Placement".into()
    }
}

#[test]
fn duplicate_window_placement_observers_are_rejected_within_and_across_components() {
    for roots in [
        vec![component::<PlacementPublisher>("first", true)],
        vec![
            component::<PlacementPublisher>("first", false),
            component::<PlacementPublisher>("second", false),
        ],
    ] {
        assert!(matches!(
            ComponentHost::mount(RecordingAdapter::default(), roots),
            Err(ComponentError::DuplicateWindowPlacement)
        ));
    }
}

#[test]
fn duplicate_component_window_declarations_are_rejected() {
    let result = ComponentHost::mount(
        RecordingAdapter::default(),
        [
            component::<WindowTitlePublisher>("first", "First"),
            component::<WindowTitlePublisher>("second", "Second"),
        ],
    );
    assert!(matches!(result, Err(ComponentError::DuplicateWindowTitle)));

    let result = ComponentHost::mount(
        RecordingAdapter::default(),
        [
            component::<WindowPublisher>("first", true),
            component::<WindowPublisher>("second", true),
        ],
    );
    assert!(matches!(
        result,
        Err(ComponentError::DuplicateWindowColorScheme)
    ));
}
