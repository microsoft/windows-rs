use super::super::*;
use crate::native::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone)]
struct Props {
    accepted: Rc<Cell<bool>>,
    sender: Rc<RefCell<Option<LocalSender<Message>>>>,
    window: Rc<RefCell<Option<WindowRef>>>,
}

impl PartialEq for Props {
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
    props: Props,
    window: WindowRef,
}

impl Component for ClosingComponent {
    type Message = Message;
    type Props = Props;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        let window = context.window();
        *props.sender.borrow_mut() = Some(context.sender());
        *props.window.borrow_mut() = Some(window.clone());
        Self {
            closing: false,
            invalid: false,
            props: props.clone(),
            window,
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.props = props.clone();
    }

    fn update(&mut self, message: Message, _context: &mut ComponentContext<Self>) {
        match message {
            Message::Close => {
                self.closing = true;
                self.props.accepted.set(self.window.request_close());
            }
            Message::CloseWithInvalidView => {
                self.closing = true;
                self.invalid = true;
                self.props.accepted.set(self.window.request_close());
            }
            Message::Fix => {
                self.closing = false;
                self.invalid = false;
            }
        }
    }

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
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

fn props() -> Props {
    Props {
        accepted: Rc::new(Cell::new(false)),
        sender: Rc::new(RefCell::new(None)),
        window: Rc::new(RefCell::new(None)),
    }
}

#[test]
fn close_request_runs_after_component_publication() {
    let props = props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(props.clone()))
        .unwrap();
    let window = pump.window.unwrap();

    assert!(!props.window.borrow().as_ref().unwrap().request_close());
    assert!(props.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(props.accepted.get());
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
    let props = props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(props.clone()))
        .unwrap();

    assert!(props.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(props.accepted.get());
    assert!(props.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert!(!props.accepted.get());
    assert_eq!(pump.runtime().close_requests().len(), 1);
}

#[test]
fn native_failure_before_publication_does_not_close() {
    let props = props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(props.clone()))
        .unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(props.sender.borrow().as_ref().unwrap().send(Message::Close));
    assert!(matches!(
        pump.dispatch_components(1),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.runtime().close_requests().is_empty());
}

#[test]
fn failed_candidate_discards_close_request() {
    let props = props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(props.clone()))
        .unwrap();

    assert!(
        props
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
    assert!(props.accepted.get());
    assert!(pump.runtime().close_requests().is_empty());

    assert!(props.sender.borrow().as_ref().unwrap().send(Message::Fix));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(pump.runtime().close_requests().is_empty());
}

struct CloseOnCreate;

impl Component for CloseOnCreate {
    type Message = ();
    type Props = Rc<Cell<bool>>;

    fn create(accepted: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        accepted.set(context.window().request_close());
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
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
    type Props = bool;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn changed(&mut self, close: &Self::Props, context: &mut ComponentContext<Self>) {
        if *close {
            assert!(context.window().request_close());
        }
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, close: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(close.to_string()).into()
    }
}

#[test]
fn changed_request_closes_after_props_publish() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<CloseOnChanged>(false))
        .unwrap();
    pump.update_view(View::component::<CloseOnChanged>(true))
        .unwrap();

    assert_eq!(pump.runtime().close_requests(), &[pump.window.unwrap()]);
}

#[test]
fn window_reference_rejects_outside_lifecycle_and_after_shutdown() {
    let props = props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(props.clone()))
        .unwrap();
    let window = props.window.borrow().as_ref().unwrap().clone();

    assert!(!window.request_close());
    pump.shutdown();
    assert!(!window.request_close());
}

#[derive(Clone)]
struct OpenProps {
    accepted: Rc<Cell<bool>>,
    sender: Rc<RefCell<Option<LocalSender<OpenMessage>>>>,
}

impl PartialEq for OpenProps {
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
    props: OpenProps,
}

impl Component for OpeningComponent {
    type Message = OpenMessage;
    type Props = OpenProps;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        *props.sender.borrow_mut() = Some(context.sender());
        Self {
            invalid: false,
            opened: false,
            props: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.props = props.clone();
    }

    fn update(&mut self, message: OpenMessage, context: &mut ComponentContext<Self>) {
        match message {
            OpenMessage::Open => {
                self.opened = true;
                self.props
                    .accepted
                    .set(context.open_window(TextBlock::new().text("runtime window").into()));
            }
            OpenMessage::OpenAndClose => {
                self.opened = true;
                let opened = context.open_window(TextBlock::new().text("replacement").into());
                self.props
                    .accepted
                    .set(opened && context.window().request_close());
            }
            OpenMessage::OpenWithInvalidView => {
                self.invalid = true;
                self.props
                    .accepted
                    .set(context.open_window(TextBlock::new().text("discarded window").into()));
            }
        }
    }

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        if self.invalid {
            View::fragment((TextBlock::new(), TextBlock::new()))
        } else {
            TextBlock::new()
                .text(if self.opened { "opened" } else { "opener" })
                .into()
        }
    }
}

fn open_props() -> OpenProps {
    OpenProps {
        accepted: Rc::new(Cell::new(false)),
        sender: Rc::new(RefCell::new(None)),
    }
}

#[test]
fn window_open_runs_after_component_publication() {
    let props = open_props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(props.clone()))
        .unwrap();

    assert!(
        props
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::Open)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert!(props.accepted.get());
    assert_eq!(
        pump.runtime().opened_windows(),
        &[TextBlock::new().text("runtime window").into()]
    );
}

#[test]
fn failed_candidate_discards_window_open() {
    let props = open_props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(props.clone()))
        .unwrap();

    assert!(
        props
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

    assert!(props.accepted.get());
    assert!(pump.runtime().opened_windows().is_empty());
}

#[test]
fn native_failure_before_publication_discards_window_open() {
    let props = open_props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(props.clone()))
        .unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(
        props
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
    let props = open_props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(props.clone()))
        .unwrap();
    let native = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root.unwrap()).unwrap();
    pump.runtime_mut().fail_window_open();

    assert!(
        props
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::Open)
    );
    assert!(matches!(
        pump.dispatch_components(1),
        Err(PumpError::NativeApplyFailed(NativeApplyError {
            error: RuntimeError::Injected,
            ..
        }))
    ));

    assert!(pump.runtime().opened_windows().is_empty());
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
    let props = open_props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(props.clone()))
        .unwrap();

    assert!(
        props
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::OpenAndClose)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert!(props.accepted.get());
    assert_eq!(pump.runtime().opened_windows().len(), 1);
    assert_eq!(pump.runtime().close_requests().len(), 1);
}

struct OpenOnCreate;

impl Component for OpenOnCreate {
    type Message = ();
    type Props = Rc<Cell<bool>>;

    fn create(accepted: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        accepted.set(context.open_window(TextBlock::new().text("created window").into()));
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
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
    let props = open_props();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<OpeningComponent>(props.clone()))
        .unwrap();
    assert!(
        props
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::OpenAndClose)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(pump.runtime().opened_windows().len(), 1);

    assert!(
        props
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(OpenMessage::Open)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(!props.accepted.get());
    assert_eq!(pump.runtime().opened_windows().len(), 1);
}
