use super::super::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
struct TitleProps {
    duplicate: bool,
    title: Option<String>,
}

struct TitleComponent;

impl Component for TitleComponent {
    type Message = ();
    type Props = TitleProps;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        if let Some(title) = &props.title {
            context.window_title(title);
            if props.duplicate {
                context.window_title(title);
            }
        }
        TextBlock::new().text("content").into()
    }
}

fn title(value: Option<&str>) -> TitleProps {
    TitleProps {
        duplicate: false,
        title: value.map(str::to_string),
    }
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
    let mut props = title(Some("Duplicate"));
    props.duplicate = true;
    let mut pump = Pump::new(RecordingRuntime::default());

    assert_eq!(
        pump.mount_view(View::component::<TitleComponent>(props)),
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
struct LocalProps {
    sender: Rc<RefCell<Option<LocalSender<String>>>>,
}

impl PartialEq for LocalProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct LocalTitle {
    title: String,
}

impl Component for LocalTitle {
    type Message = String;
    type Props = LocalProps;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        *props.sender.borrow_mut() = Some(context.sender());
        Self {
            title: "First".to_string(),
        }
    }

    fn update(&mut self, title: String, _context: &mut ComponentContext<Self>) {
        self.title = title;
    }

    fn view(&self, _props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        context.window_title(&self.title);
        TextBlock::new().text(&self.title).into()
    }
}

#[test]
fn local_native_update_falls_back_when_title_changes() {
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<LocalTitle>(LocalProps {
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
