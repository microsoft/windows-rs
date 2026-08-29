use super::super::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
struct TitleInput {
    duplicate: bool,
    title: Option<String>,
}

struct TitleComponent;

impl Component for TitleComponent {
    type Message = ();
    type Input = TitleInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if let Some(title) = &input.title {
            context.window_title(title);
            if input.duplicate {
                context.window_title(title);
            }
        }
        TextBlock::new().text("content").into()
    }
}

fn title(value: Option<&str>) -> TitleInput {
    TitleInput {
        duplicate: false,
        title: value.map(str::to_string),
    }
}

fn sibling_titles(first: (&str, Option<&str>), second: (&str, Option<&str>)) -> View {
    StackPanel::new().keyed_children([
        KeyedView::new(first.0, View::component::<TitleComponent>(title(first.1))),
        KeyedView::new(second.0, View::component::<TitleComponent>(title(second.1))),
    ])
}

fn title_command_count(runtime: &RecordingRuntime) -> usize {
    runtime
        .commands()
        .iter()
        .flatten()
        .filter(|command| matches!(command, Command::SetWindowTitle { .. }))
        .count()
}

#[test]
fn initial_title_is_applied_before_activation() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<TitleComponent>(title(Some("Initial"))))
        .unwrap();
    let window = pump.window.unwrap();

    assert_eq!(pump.runtime().window_title(window), Some("Initial"));
    let commands = &pump.runtime().commands()[0];
    let title = commands
        .iter()
        .position(|command| matches!(command, Command::SetWindowTitle { .. }))
        .unwrap();
    let activation = commands
        .iter()
        .position(|command| matches!(command, Command::ActivateWindow { .. }))
        .unwrap();
    assert!(title < activation);
}

#[test]
fn same_owner_updates_and_clears_title_without_redundant_commands() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<TitleComponent>(title(Some("First"))))
        .unwrap();
    let window = pump.window.unwrap();

    pump.update_view(View::component::<TitleComponent>(title(Some("Second"))))
        .unwrap();
    assert_eq!(pump.runtime().window_title(window), Some("Second"));
    let commands = title_command_count(pump.runtime());

    pump.update_view(View::component::<TitleComponent>(title(Some("Second"))))
        .unwrap();
    assert_eq!(title_command_count(pump.runtime()), commands);

    pump.update_view(View::component::<TitleComponent>(title(None)))
        .unwrap();
    assert_eq!(pump.runtime().window_title(window), Some(""));
}

#[test]
fn retiring_owner_clears_title() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<TitleComponent>(title(Some("Owned"))))
        .unwrap();
    let window = pump.window.unwrap();

    pump.update_view(TextBlock::new().into()).unwrap();

    assert_eq!(pump.runtime().window_title(window), Some(""));
    assert!(pump.tree.window_title().is_none());
}

#[test]
fn title_ownership_transfers_after_owner_retirement() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(StackPanel::new().keyed_children([KeyedView::new(
        "old",
        View::component::<TitleComponent>(title(Some("Old"))),
    )]))
    .unwrap();
    let window = pump.window.unwrap();

    pump.update_view(StackPanel::new().keyed_children([KeyedView::new(
        "new",
        View::component::<TitleComponent>(title(Some("New"))),
    )]))
    .unwrap();

    assert_eq!(pump.runtime().window_title(window), Some("New"));
}

#[test]
fn surviving_siblings_transfer_title_in_either_child_order() {
    for (initial, transferred) in [
        (
            sibling_titles(("a", None), ("b", Some("B"))),
            sibling_titles(("a", Some("A")), ("b", None)),
        ),
        (
            sibling_titles(("b", Some("B")), ("a", None)),
            sibling_titles(("b", None), ("a", Some("A"))),
        ),
    ] {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(initial).unwrap();
        let window = pump.window.unwrap();

        pump.update_view(transferred).unwrap();

        assert_eq!(pump.runtime().window_title(window), Some("A"));
    }
}

#[test]
fn failed_sibling_handoff_can_retry_with_one_declarer() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(sibling_titles(("a", None), ("b", Some("B"))))
        .unwrap();
    let window = pump.window.unwrap();

    assert_eq!(
        pump.update_view(sibling_titles(("a", Some("A")), ("b", Some("B")))),
        Err(PumpError::DuplicateWindowTitle)
    );
    assert_eq!(pump.runtime().window_title(window), Some("B"));

    pump.update_view(sibling_titles(("a", Some("A")), ("b", None)))
        .unwrap();
    assert_eq!(pump.runtime().window_title(window), Some("A"));
}

#[test]
fn duplicate_live_owner_is_rejected_transactionally() {
    let mut pump = Pump::new(RecordingRuntime::default());
    let result = pump.mount_view(StackPanel::new().keyed_children([
        KeyedView::new(
            "first",
            View::component::<TitleComponent>(title(Some("First"))),
        ),
        KeyedView::new(
            "second",
            View::component::<TitleComponent>(title(Some("Second"))),
        ),
    ]));

    assert_eq!(result, Err(PumpError::DuplicateWindowTitle));
    assert!(pump.runtime().is_empty());
}

#[test]
fn duplicate_declaration_from_one_component_is_rejected() {
    let mut input = title(Some("Duplicate"));
    input.duplicate = true;
    let mut pump = Pump::new(RecordingRuntime::default());

    assert_eq!(
        pump.mount_view(View::component::<TitleComponent>(input)),
        Err(PumpError::DuplicateWindowTitle)
    );
    assert!(pump.runtime().is_empty());
}

#[test]
fn failed_native_title_apply_does_not_publish_candidate_title() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<TitleComponent>(title(Some("Published"))))
        .unwrap();
    let window = pump.window.unwrap();
    let owner = pump.tree.window_title().unwrap().owner;
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update_view(View::component::<TitleComponent>(title(Some("Candidate")))),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(pump.runtime().window_title(window), Some("Published"));
    assert_eq!(pump.tree.window_title().unwrap().owner, owner);
    assert_eq!(
        pump.tree.window_title().unwrap().title.as_ref(),
        "Published"
    );
}

#[derive(Clone)]
struct LocalInput {
    sender: Rc<RefCell<Option<LocalSender<String>>>>,
}

impl PartialEq for LocalInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct LocalTitle {
    title: String,
}

#[derive(Clone)]
struct IndependentInput {
    initial: Option<String>,
    sender: Rc<RefCell<Option<LocalSender<Option<String>>>>>,
}

impl PartialEq for IndependentInput {
    fn eq(&self, other: &Self) -> bool {
        self.initial == other.initial && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct IndependentTitle {
    title: Option<String>,
}

impl Component for IndependentTitle {
    type Message = Option<String>;
    type Input = IndependentInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self {
            title: input.initial.clone(),
        }
    }

    fn update(&mut self, title: Option<String>, _context: &ComponentContext<Self>) {
        self.title = title;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if let Some(title) = &self.title {
            context.window_title(title);
        }
        TextBlock::new().into()
    }
}

#[test]
fn independently_dirty_siblings_transfer_title() {
    let a = Rc::new(RefCell::new(None));
    let b = Rc::new(RefCell::new(None));
    let view = StackPanel::new().keyed_children([
        KeyedView::new(
            "a",
            View::component::<IndependentTitle>(IndependentInput {
                initial: None,
                sender: Rc::clone(&a),
            }),
        ),
        KeyedView::new(
            "b",
            View::component::<IndependentTitle>(IndependentInput {
                initial: Some("B".to_string()),
                sender: Rc::clone(&b),
            }),
        ),
    ]);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view).unwrap();
    let window = pump.window.unwrap();

    assert!(a.borrow().as_ref().unwrap().send(Some("A".to_string())));
    assert!(b.borrow().as_ref().unwrap().send(None));
    assert_eq!(pump.dispatch_components(2), Ok(2));

    assert_eq!(pump.runtime().window_title(window), Some("A"));
}

impl Component for LocalTitle {
    type Message = String;
    type Input = LocalInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self {
            title: "First".to_string(),
        }
    }

    fn update(&mut self, title: String, _context: &ComponentContext<Self>) {
        self.title = title;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title(&self.title);
        TextBlock::new().text(&self.title).into()
    }
}

#[test]
fn local_native_update_falls_back_when_title_changes() {
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<LocalTitle>(LocalInput {
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    let window = pump.window.unwrap();

    assert!(sender.borrow().as_ref().unwrap().send("Second".to_string()));
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert_eq!(pump.runtime().window_title(window), Some("Second"));
    assert!(pump.runtime().commands().last().unwrap().iter().any(
        |command| matches!(command, Command::SetWindowTitle { title, .. } if title == "Second")
    ));
}
