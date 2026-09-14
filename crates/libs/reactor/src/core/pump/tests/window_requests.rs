use super::*;
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
    Activate,
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
            Message::Activate => {
                self.input.accepted.set(self.window.request_activate());
            }
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
fn activate_request_runs_after_component_publication() {
    let input = input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ClosingComponent>(input.clone()))
        .unwrap();
    let initial = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .filter(|command| matches!(command, Command::RequestWindowActivation { .. }))
        .count();

    assert!(!input.window.borrow().as_ref().unwrap().request_activate());
    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::Activate)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.accepted.get());
    assert_eq!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .filter(|command| matches!(command, Command::RequestWindowActivation { .. }))
            .count(),
        initial + 1
    );
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
        pump.tree.native(native).desired,
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
struct RunInput {
    accepted: Rc<Cell<bool>>,
    completed: Rc<Cell<bool>>,
    operations: Rc<Cell<usize>>,
    second_accepted: Rc<Cell<bool>>,
    sender: Rc<RefCell<Option<LocalSender<RunMessage>>>>,
    trace: Rc<RefCell<Vec<&'static str>>>,
}

impl PartialEq for RunInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.accepted, &other.accepted)
            && Rc::ptr_eq(&self.completed, &other.completed)
            && Rc::ptr_eq(&self.operations, &other.operations)
            && Rc::ptr_eq(&self.second_accepted, &other.second_accepted)
            && Rc::ptr_eq(&self.sender, &other.sender)
            && Rc::ptr_eq(&self.trace, &other.trace)
    }
}

#[derive(Clone)]
enum RunMessage {
    Run,
    RunAndClose,
    RunTwice,
    RunWithInvalidView,
    Complete,
    Fix,
}

struct RunningComponent {
    invalid: bool,
    input: RunInput,
}

impl Component for RunningComponent {
    type Message = RunMessage;
    type Input = RunInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self {
            invalid: false,
            input: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, message: RunMessage, context: &ComponentContext<Self>) {
        match message {
            RunMessage::Run | RunMessage::RunAndClose | RunMessage::RunWithInvalidView => {
                self.input.trace.borrow_mut().push("update");
                let operations = Rc::clone(&self.input.operations);
                let trace = Rc::clone(&self.input.trace);
                self.input.accepted.set(context.run_window(move |window| {
                    assert_eq!(window.as_raw() as isize, 1);
                    operations.set(operations.get() + 1);
                    trace.borrow_mut().push("operation");
                    RunMessage::Complete
                }));
                self.input.trace.borrow_mut().push("update-returned");
                if matches!(message, RunMessage::RunAndClose) {
                    assert!(context.window().request_close());
                }
                self.invalid = matches!(message, RunMessage::RunWithInvalidView);
            }
            RunMessage::RunTwice => {
                let operations = Rc::clone(&self.input.operations);
                self.input.accepted.set(context.run_window(move |_| {
                    operations.set(operations.get() + 1);
                    RunMessage::Complete
                }));
                let operations = Rc::clone(&self.input.operations);
                self.input.second_accepted.set(context.run_window(move |_| {
                    operations.set(operations.get() + 100);
                    RunMessage::Complete
                }));
            }
            RunMessage::Complete => {
                self.input.completed.set(true);
            }
            RunMessage::Fix => self.invalid = false,
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        self.input.trace.borrow_mut().push("view");
        if self.invalid {
            View::fragment((TextBlock::new(), TextBlock::new()))
        } else {
            TextBlock::new().text("running").into()
        }
    }
}

fn run_input() -> RunInput {
    RunInput {
        accepted: Rc::new(Cell::new(false)),
        completed: Rc::new(Cell::new(false)),
        operations: Rc::new(Cell::new(0)),
        second_accepted: Rc::new(Cell::new(false)),
        sender: Rc::new(RefCell::new(None)),
        trace: Rc::new(RefCell::new(Vec::new())),
    }
}

#[test]
fn window_work_runs_after_publication_and_queues_its_message() {
    let input = run_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RunningComponent>(input.clone()))
        .unwrap();
    input.trace.borrow_mut().clear();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert!(input.accepted.get());
    assert_eq!(
        input.trace.borrow().as_slice(),
        ["update", "update-returned", "view"]
    );
    assert_eq!(input.operations.get(), 0);
    assert!(!input.completed.get());

    assert_eq!(pump.process_window_operations(), Ok(1));
    assert_eq!(
        input.trace.borrow().as_slice(),
        ["update", "update-returned", "view", "operation"]
    );
    assert_eq!(input.operations.get(), 1);
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.completed.get());
}

#[test]
fn only_one_window_operation_is_pending_per_window() {
    let input = run_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RunningComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::RunTwice)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.accepted.get());
    assert!(!input.second_accepted.get());
    assert_eq!(input.operations.get(), 0);

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(!input.accepted.get());

    assert_eq!(pump.process_window_operations(), Ok(1));
    assert_eq!(input.operations.get(), 1);

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert_eq!(pump.dispatch_components(2), Ok(2));
    assert!(input.accepted.get());
    assert_eq!(pump.process_window_operations(), Ok(1));
    assert_eq!(input.operations.get(), 2);
}

#[test]
fn failed_publication_discards_window_work() {
    let input = run_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RunningComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::RunWithInvalidView)
    );
    assert_eq!(
        pump.dispatch_components(1),
        Err(PumpError::StructureUnsupported)
    );
    assert!(input.accepted.get());
    assert_eq!(input.operations.get(), 0);

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Fix)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(input.operations.get(), 0);

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.accepted.get());
    assert_eq!(pump.process_window_operations(), Ok(1));
    assert_eq!(input.operations.get(), 1);
}

#[test]
fn window_handle_failure_is_fail_stop() {
    let input = run_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RunningComponent>(input.clone()))
        .unwrap();
    pump.runtime_mut().fail_window_handle();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(
        pump.process_window_operations(),
        Err(PumpError::WindowHandleFailed(RuntimeError::Injected))
    );
    assert!(pump.poisoned());
    assert_eq!(input.operations.get(), 0);
}

#[test]
fn closing_window_discards_window_work() {
    let input = run_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RunningComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::RunAndClose)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.accepted.get());
    assert_eq!(input.operations.get(), 0);
    assert_eq!(pump.runtime().close_requests(), &[pump.window.unwrap()]);
}

#[test]
fn later_close_discards_pending_window_work() {
    let input = run_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RunningComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.accepted.get());

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::RunAndClose)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(!input.accepted.get());
    assert_eq!(pump.process_window_operations(), Ok(0));
    assert_eq!(input.operations.get(), 0);
    assert_eq!(pump.runtime().close_requests(), &[pump.window.unwrap()]);
}

#[test]
fn native_window_close_discards_pending_window_work() {
    let input = run_input();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RunningComponent>(input.clone()))
        .unwrap();

    assert!(
        input
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(input.accepted.get());
    assert!(pump.native_work_pending());

    pump.native_window_closed();

    assert_eq!(pump.process_window_operations(), Ok(0));
    assert_eq!(input.operations.get(), 0);
    assert!(!pump.native_work_pending());
}

#[test]
fn separate_windows_have_independent_operation_slots() {
    let first = run_input();
    let second = run_input();
    let mut first_pump = Pump::new(RecordingRuntime::default());
    let mut second_pump = Pump::new(RecordingRuntime::default());
    first_pump
        .mount_view(View::component::<RunningComponent>(first.clone()))
        .unwrap();
    second_pump
        .mount_view(View::component::<RunningComponent>(second.clone()))
        .unwrap();

    assert!(
        first
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert!(
        second
            .sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(RunMessage::Run)
    );
    assert_eq!(first_pump.dispatch_components(1), Ok(1));
    assert_eq!(second_pump.dispatch_components(1), Ok(1));
    assert!(first.accepted.get());
    assert!(second.accepted.get());

    assert_eq!(first_pump.process_window_operations(), Ok(1));
    assert_eq!(first.operations.get(), 1);
    assert_eq!(second.operations.get(), 0);
    assert_eq!(second_pump.process_window_operations(), Ok(1));
    assert_eq!(second.operations.get(), 1);
}

struct RunOnCreate {
    completed: Rc<Cell<bool>>,
}

impl Component for RunOnCreate {
    type Message = ();
    type Input = (Rc<Cell<usize>>, Rc<Cell<bool>>);

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let operations = Rc::clone(&input.0);
        assert!(context.run_window(move |_| {
            operations.set(operations.get() + 1);
        }));
        Self {
            completed: Rc::clone(&input.1),
        }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.completed.set(true);
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("created").into()
    }
}

#[test]
fn create_can_queue_window_work_for_initial_publication() {
    let operations = Rc::new(Cell::new(0));
    let completed = Rc::new(Cell::new(false));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RunOnCreate>((
        Rc::clone(&operations),
        Rc::clone(&completed),
    )))
    .unwrap();

    assert_eq!(operations.get(), 0);
    assert!(pump.native_work_pending());
    assert_eq!(pump.process_window_operations(), Ok(1));
    assert_eq!(operations.get(), 1);
    assert!(!completed.get());
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert!(completed.get());
}

#[derive(Clone)]
struct RetirementInput {
    child_sender: Rc<RefCell<Option<LocalSender<()>>>>,
    operations: Rc<Cell<usize>>,
    parent_sender: Rc<RefCell<Option<LocalSender<()>>>>,
}

impl PartialEq for RetirementInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.child_sender, &other.child_sender)
            && Rc::ptr_eq(&self.operations, &other.operations)
            && Rc::ptr_eq(&self.parent_sender, &other.parent_sender)
    }
}

struct RetiringChild {
    operations: Rc<Cell<usize>>,
}

impl Component for RetiringChild {
    type Message = ();
    type Input = RetirementInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.child_sender.borrow_mut() = Some(context.sender());
        Self {
            operations: Rc::clone(&input.operations),
        }
    }

    fn update(&mut self, _message: (), context: &ComponentContext<Self>) {
        let operations = Rc::clone(&self.operations);
        assert!(context.run_window(move |_| {
            operations.set(operations.get() + 1);
        }));
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("child").into()
    }
}

struct RetiringParent {
    input: RetirementInput,
    show_child: bool,
}

impl Component for RetiringParent {
    type Message = ();
    type Input = RetirementInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.parent_sender.borrow_mut() = Some(context.sender());
        Self {
            input: input.clone(),
            show_child: true,
        }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.show_child = false;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if self.show_child {
            View::component::<RetiringChild>(self.input.clone())
        } else {
            TextBlock::new().text("retired").into()
        }
    }
}

#[test]
fn retiring_component_discards_its_window_work() {
    let input = RetirementInput {
        child_sender: Rc::new(RefCell::new(None)),
        operations: Rc::new(Cell::new(0)),
        parent_sender: Rc::new(RefCell::new(None)),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RetiringParent>(input.clone()))
        .unwrap();

    assert!(input.child_sender.borrow().as_ref().unwrap().send(()));
    assert!(input.parent_sender.borrow().as_ref().unwrap().send(()));
    assert_eq!(pump.dispatch_components(2), Ok(2));
    assert_eq!(pump.process_window_operations(), Ok(0));
    assert_eq!(input.operations.get(), 0);
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
