use super::super::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone)]
struct Input {
    accepted: Rc<Cell<bool>>,
    sender: Rc<RefCell<Option<LocalSender<Message>>>>,
    window: Rc<RefCell<Option<WindowRef>>>,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.accepted, &other.accepted)
            && Rc::ptr_eq(&self.sender, &other.sender)
            && Rc::ptr_eq(&self.window, &other.window)
    }
}

#[derive(Clone)]
enum Message {
    Close,
    CloseWithInvalidView,
    Fix,
}

struct ClosingComponent {
    closing: bool,
    invalid: bool,
    input: Input,
    window: WindowRef,
}

impl Component for ClosingComponent {
    type Message = Message;
    type Input = Input;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let window = context.window();
        *input.sender.borrow_mut() = Some(context.sender());
        *input.window.borrow_mut() = Some(window.clone());
        Self {
            closing: false,
            invalid: false,
            input: input.clone(),
            window,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Close => {
                self.closing = true;
                self.input.accepted.set(self.window.request_close());
            }
            Message::CloseWithInvalidView => {
                self.closing = true;
                self.invalid = true;
                self.input.accepted.set(self.window.request_close());
            }
            Message::Fix => {
                self.closing = false;
                self.invalid = false;
            }
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if self.invalid {
            View::fragment((
                TextBlock::new().text("first"),
                TextBlock::new().text("second"),
            ))
        } else {
            TextBlock::new()
                .text(if self.closing { "closing" } else { "published" })
                .into()
        }
    }
}

fn input() -> Input {
    Input {
        accepted: Rc::new(Cell::new(false)),
        sender: Rc::new(RefCell::new(None)),
        window: Rc::new(RefCell::new(None)),
    }
}

#[test]
fn close_request_runs_after_component_publication() {
    let input = input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(input.clone()))
        .unwrap();
    let window = pump.window.unwrap();

    assert!(!input.window.borrow().as_ref().unwrap().request_close());
    assert!(input.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.accepted.get());
    assert_eq!(pump.runtime().close_requests(), &[window]);
    assert!(matches!(
        pump.runtime().commands()[1].as_slice(),
        [Command::SetProperty {
            property: PropertyId::TextBlockText,
            value: PropertyValue::Str(value),
            ..
        }] if value == "closing"
    ));
    assert!(matches!(
        pump.runtime().commands()[2].as_slice(),
        [Command::CloseWindow { node }] if *node == window
    ));
}

#[test]
fn committed_close_rejects_requests_from_later_turns() {
    let input = input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(input.clone()))
        .unwrap();

    assert!(input.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.accepted.get());
    assert!(input.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert!(!input.accepted.get());
    assert_eq!(pump.runtime().close_requests().len(), 1);
}

#[test]
fn failed_post_publication_close_keeps_local_native_state_committed() {
    let input = input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(input.clone()))
        .unwrap();
    pump.runtime_mut().fail_after(1, 0);

    assert!(input.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert!(matches!(
        pump.dispatch_components(1),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.poisoned());
    assert!(!pump.native_work_pending());
    assert!(input.window.borrow().as_ref().unwrap().close_committed());
    let native = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
    assert_eq!(
        pump.tree.native(native).unwrap().desired,
        Element::from(TextBlock::new().text("closing"))
            .into_parts()
            .props
    );
    let window = input.window.borrow().as_ref().unwrap().clone();
    pump.shutdown();
    assert!(window.close_committed());
}

#[test]
fn native_failure_before_publication_does_not_close() {
    let input = input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(input.clone()))
        .unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(input.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert!(matches!(
        pump.dispatch_components(1),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.runtime().close_requests().is_empty());
}

#[test]
fn failed_candidate_discards_close_request() {
    let input = input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::CloseWithInvalidView)
    );
    assert_eq!(
        pump.dispatch_components(1),
        Err(PumpError::StructureUnsupported)
    );
    assert!(input.accepted.get());
    assert!(pump.runtime().close_requests().is_empty());

    assert!(input.sender.borrow().as_ref().unwrap().send(Message::Fix));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(pump.runtime().close_requests().is_empty());
}

struct CloseOnCreate;

impl Component for CloseOnCreate {
    type Message = ();
    type Input = Rc<Cell<bool>>;

    fn create(accepted: &Self::Input, context: &ComponentContext<Self>) -> Self {
        accepted.set(context.window().request_close());
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("created").into()
    }
}

#[test]
fn create_request_closes_only_after_initial_mount() {
    let accepted = Rc::new(Cell::new(false));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<CloseOnCreate>(Rc::clone(&accepted)))
        .unwrap();

    assert!(accepted.get());
    assert_eq!(pump.runtime().close_requests(), &[pump.window.unwrap()]);
    assert_eq!(pump.runtime().commands().len(), 2);
    assert!(matches!(
        pump.runtime().commands()[1].as_slice(),
        [Command::CloseWindow { .. }]
    ));
}

struct CloseOnChanged;

impl Component for CloseOnChanged {
    type Message = ();
    type Input = bool;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn input_changed(&mut self, close: &Self::Input, context: &ComponentContext<Self>) {
        if *close {
            assert!(context.window().request_close());
        }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, close: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(close.to_string()).into()
    }
}

#[test]
fn changed_request_closes_after_input_publish() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<CloseOnChanged>(false))
        .unwrap();
    pump.update_view(View::component::<CloseOnChanged>(true))
        .unwrap();

    assert_eq!(pump.runtime().close_requests(), &[pump.window.unwrap()]);
}

#[test]
fn window_reference_rejects_outside_lifecycle_and_after_shutdown() {
    let input = input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(input.clone()))
        .unwrap();
    let window = input.window.borrow().as_ref().unwrap().clone();

    assert!(!window.request_close());
    pump.shutdown();
    assert!(!window.request_close());
}

#[derive(Clone)]
struct OpenInput {
    accepted: Rc<Cell<bool>>,
    sender: Rc<RefCell<Option<LocalSender<OpenMessage>>>>,
}

impl PartialEq for OpenInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.accepted, &other.accepted) && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

#[derive(Clone)]
enum OpenMessage {
    Open,
    OpenAndClose,
    OpenWithInvalidView,
}

struct OpeningComponent {
    invalid: bool,
    opened: bool,
    input: OpenInput,
}

impl Component for OpeningComponent {
    type Message = OpenMessage;
    type Input = OpenInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self {
            invalid: false,
            opened: false,
            input: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, message: OpenMessage, context: &ComponentContext<Self>) {
        match message {
            OpenMessage::Open => {
                self.opened = true;
                self.input
                    .accepted
                    .set(context.open_window(TextBlock::new().text("runtime window").into()));
            }
            OpenMessage::OpenAndClose => {
                self.opened = true;
                let opened = context.open_window(TextBlock::new().text("replacement").into());
                self.input
                    .accepted
                    .set(opened && context.window().request_close());
            }
            OpenMessage::OpenWithInvalidView => {
                self.invalid = true;
                self.input
                    .accepted
                    .set(context.open_window(TextBlock::new().text("discarded window").into()));
            }
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if self.invalid {
            View::fragment((TextBlock::new(), TextBlock::new()))
        } else {
            TextBlock::new()
                .text(if self.opened { "opened" } else { "opener" })
                .into()
        }
    }
}

fn open_input() -> OpenInput {
    OpenInput {
        accepted: Rc::new(Cell::new(false)),
        sender: Rc::new(RefCell::new(None)),
    }
}

#[test]
fn window_open_runs_after_component_publication() {
    let input = open_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::Open)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert!(input.accepted.get());
    assert_eq!(
        pump.runtime().opened_windows(),
        &[TextBlock::new().text("runtime window").into()]
    );
}

#[test]
fn failed_candidate_discards_window_open() {
    let input = open_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::OpenWithInvalidView)
    );
    assert_eq!(
        pump.dispatch_components(1),
        Err(PumpError::StructureUnsupported)
    );

    assert!(input.accepted.get());
    assert!(pump.runtime().opened_windows().is_empty());
}

#[test]
fn native_failure_before_publication_discards_window_open() {
    let input = open_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(input.clone()))
        .unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::Open)
    );
    assert!(matches!(
        pump.dispatch_components(1),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.runtime().opened_windows().is_empty());
}

#[test]
fn window_open_failure_occurs_after_candidate_publication() {
    let input = open_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(input.clone()))
        .unwrap();
    let native = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root.unwrap()).unwrap();
    pump.runtime_mut().fail_window_open();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::Open)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert!(pump.runtime().opened_windows().is_empty());
    assert_eq!(
        pump.drain_diagnostics(),
        [PumpDiagnostic::WindowOpenRejected {
            error: RuntimeError::Injected,
        }]
    );
    assert_eq!(
        pump.runtime()
            .node(native)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("opened".into()))
    );
}

#[test]
fn open_is_registered_before_same_turn_close() {
    let input = open_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::OpenAndClose)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert!(input.accepted.get());
    assert_eq!(pump.runtime().opened_windows().len(), 1);
    assert_eq!(pump.runtime().close_requests().len(), 1);
}

struct OpenOnCreate;

impl Component for OpenOnCreate {
    type Message = ();
    type Input = Rc<Cell<bool>>;

    fn create(accepted: &Self::Input, context: &ComponentContext<Self>) -> Self {
        accepted.set(context.open_window(TextBlock::new().text("created window").into()));
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("creator").into()
    }
}

#[test]
fn create_can_stage_runtime_window_open() {
    let accepted = Rc::new(Cell::new(false));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpenOnCreate>(Rc::clone(&accepted)))
        .unwrap();

    assert!(accepted.get());
    assert_eq!(pump.runtime().opened_windows().len(), 1);
}

#[test]
fn committed_close_rejects_later_window_open() {
    let input = open_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(input.clone()))
        .unwrap();
    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::OpenAndClose)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(pump.runtime().opened_windows().len(), 1);

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::Open)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(!input.accepted.get());
    assert_eq!(pump.runtime().opened_windows().len(), 1);
}
