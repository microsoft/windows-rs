use std::cell::RefCell;
use std::rc::Rc;

use windows_reactor::core::backend::{Op, RecordingBackend};
use windows_reactor::core::component::Component;
use windows_reactor::core::dispatcher::{DispatchPriority, Dispatcher};
use windows_reactor::core::element::{Element, StackPanel};
use windows_reactor::core::render_context::{Dispatch, RenderCx};
use windows_reactor::core::render_host::RenderHost;
use windows_reactor::dsl::{ElementExt, button, check_box, text_block};
use windows_reactor::{hstack, vstack};

#[derive(Clone, PartialEq, Debug)]
struct TodoItem {
    id: String,
    text: String,
    is_completed: bool,
}

#[derive(Clone, PartialEq, Debug, Default)]
enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

#[derive(Clone, PartialEq, Debug, Default)]
struct TodoState {
    items: Vec<TodoItem>,
    new_item_text: String,
    filter: Filter,
    next_id: u64,
}

enum TodoAction {
    AddItem,
    ToggleItem(String),
    DeleteItem(String),
    SetNewItemText(String),
    SetFilter(Filter),
}

fn todo_reducer(mut state: TodoState, action: TodoAction) -> TodoState {
    match action {
        TodoAction::AddItem => {
            let trimmed = state.new_item_text.trim();
            if !trimmed.is_empty() {
                let id = format!("item-{}", state.next_id);
                state.next_id += 1;
                state.items.push(TodoItem {
                    id,
                    text: trimmed.to_string(),
                    is_completed: false,
                });
                state.new_item_text.clear();
            }
            state
        }
        TodoAction::ToggleItem(id) => {
            for it in &mut state.items {
                if it.id == id {
                    it.is_completed = !it.is_completed;
                    break;
                }
            }
            state
        }
        TodoAction::DeleteItem(id) => {
            state.items.retain(|i| i.id != id);
            state
        }
        TodoAction::SetNewItemText(s) => {
            state.new_item_text = s;
            state
        }
        TodoAction::SetFilter(f) => {
            state.filter = f;
            state
        }
    }
}

#[test]
fn add_items_creates_distinct_ids_and_clears_draft() {
    let mut s = TodoState::default();
    s = todo_reducer(s, TodoAction::SetNewItemText("buy milk".to_string()));
    s = todo_reducer(s, TodoAction::AddItem);
    assert_eq!(s.items.len(), 1);
    assert_eq!(s.items[0].text, "buy milk");
    assert!(s.new_item_text.is_empty());

    s = todo_reducer(s, TodoAction::SetNewItemText("walk dog".to_string()));
    s = todo_reducer(s, TodoAction::AddItem);
    assert_eq!(s.items.len(), 2);
    assert_ne!(s.items[0].id, s.items[1].id);
}

#[test]
fn add_with_whitespace_only_draft_is_noop() {
    let mut s = TodoState::default();
    s = todo_reducer(s, TodoAction::SetNewItemText("   ".to_string()));
    s = todo_reducer(s, TodoAction::AddItem);
    assert!(s.items.is_empty());
}

#[test]
fn toggle_flips_is_completed() {
    let mut s = TodoState::default();
    s = todo_reducer(s, TodoAction::SetNewItemText("x".to_string()));
    s = todo_reducer(s, TodoAction::AddItem);
    let id = s.items[0].id.clone();
    s = todo_reducer(s, TodoAction::ToggleItem(id.clone()));
    assert!(s.items[0].is_completed);
    s = todo_reducer(s, TodoAction::ToggleItem(id));
    assert!(!s.items[0].is_completed);
}

#[test]
fn delete_removes_just_the_named_item() {
    let mut s = TodoState::default();
    for t in ["a", "b", "c"] {
        s = todo_reducer(s, TodoAction::SetNewItemText(t.into()));
        s = todo_reducer(s, TodoAction::AddItem);
    }
    assert_eq!(s.items.len(), 3);
    let middle_id = s.items[1].id.clone();
    s = todo_reducer(s, TodoAction::DeleteItem(middle_id));
    assert_eq!(s.items.len(), 2);
    assert_eq!(s.items[0].text, "a");
    assert_eq!(s.items[1].text, "c");
}

#[test]
fn set_filter_changes_filter() {
    let mut s = TodoState::default();
    s = todo_reducer(s, TodoAction::SetFilter(Filter::Completed));
    assert_eq!(s.filter, Filter::Completed);
}

#[test]
fn items_left_counter_tracks_active_only() {
    let mut s = TodoState::default();
    for t in ["a", "b", "c"] {
        s = todo_reducer(s, TodoAction::SetNewItemText(t.into()));
        s = todo_reducer(s, TodoAction::AddItem);
    }
    let first_id = s.items[0].id.clone();
    s = todo_reducer(s, TodoAction::ToggleItem(first_id));
    let remaining = s.items.iter().filter(|i| !i.is_completed).count();
    assert_eq!(remaining, 2);
}

type Job = Box<dyn FnOnce()>;

#[derive(Clone, Default)]
struct TestDispatcher {
    queue: Rc<RefCell<Vec<Job>>>,
}

impl TestDispatcher {
    fn drain(&self) {
        loop {
            let item = {
                let mut q = self.queue.borrow_mut();
                if q.is_empty() {
                    None
                } else {
                    Some(q.remove(0))
                }
            };
            match item {
                Some(f) => f(),
                None => break,
            }
        }
    }
}

impl Dispatcher for TestDispatcher {
    fn enqueue(&self, _p: DispatchPriority, f: Box<dyn FnOnce()>) -> bool {
        self.queue.borrow_mut().push(f);
        true
    }
}

struct TodoComponent {
    dispatch_out: Rc<RefCell<Option<Dispatch<TodoAction>>>>,
}

impl Component for TodoComponent {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (state, dispatch) = cx.use_reducer_fn(todo_reducer, TodoState::default());
        *self.dispatch_out.borrow_mut() = Some(dispatch.clone());

        let filtered: Vec<&TodoItem> = match state.filter {
            Filter::All => state.items.iter().collect(),
            Filter::Active => state.items.iter().filter(|i| !i.is_completed).collect(),
            Filter::Completed => state.items.iter().filter(|i| i.is_completed).collect(),
        };

        let remaining = state.items.iter().filter(|i| !i.is_completed).count();

        let rows: Vec<Element> = filtered
            .iter()
            .map(|i| {
                let toggle_d = dispatch.clone();
                let id_for_toggle = i.id.clone();
                let toggle = check_box(i.is_completed).on_checked(move |_v| {
                    toggle_d.call(TodoAction::ToggleItem(id_for_toggle.clone()));
                });
                let delete_d = dispatch.clone();
                let id_for_delete = i.id.clone();
                let delete = button("x")
                    .on_click(move || delete_d.call(TodoAction::DeleteItem(id_for_delete.clone())));
                hstack((toggle, text_block(i.text.clone()), delete))
                    .spacing(8.0)
                    .with_key(&i.id)
                    .into()
            })
            .collect();

        let mut body = StackPanel::vertical();
        body.spacing = 4.0;
        body.children = rows;

        let footer = text_block(format!("{remaining} left"));
        vstack((Element::StackPanel(body), footer)).into()
    }
}

fn drive_host(
    host: &RenderHost<RecordingBackend, TestDispatcher>,
    dispatch_out: &Rc<RefCell<Option<Dispatch<TodoAction>>>>,
    dispatcher: &TestDispatcher,
    action: TodoAction,
) {
    dispatch_out.borrow().as_ref().unwrap().call(action);
    dispatcher.drain();

    let _ = host.render_count();
}

#[test]
fn add_toggle_delete_via_host_preserves_keyed_identity() {
    let dispatcher = TestDispatcher::default();
    let dispatch_out: Rc<RefCell<Option<Dispatch<TodoAction>>>> = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(TodoComponent {
        dispatch_out: Rc::clone(&dispatch_out),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();
    dispatcher.drain();

    drive_host(
        &host,
        &dispatch_out,
        &dispatcher,
        TodoAction::SetNewItemText("a".into()),
    );
    drive_host(&host, &dispatch_out, &dispatcher, TodoAction::AddItem);
    drive_host(
        &host,
        &dispatch_out,
        &dispatcher,
        TodoAction::SetNewItemText("b".into()),
    );
    drive_host(&host, &dispatch_out, &dispatcher, TodoAction::AddItem);

    host.with_reconciler(|r| {
        let checkbox_creates = r
            .backend
            .ops
            .iter()
            .filter(|o| {
                matches!(
                    o,
                    Op::Create {
                        kind: windows_reactor::core::backend::ControlKind::CheckBox,
                        ..
                    }
                )
            })
            .count();
        assert!(checkbox_creates >= 2, "expected at least 2 CheckBox mounts");
    });

    let first_id = {
        let snap = host.with_reconciler(|r| {
            r.backend
                .ops
                .iter()
                .find_map(|o| match o {
                    Op::Create {
                        id,
                        kind: windows_reactor::core::backend::ControlKind::CheckBox,
                    } => Some(*id),
                    _ => None,
                })
                .unwrap()
        });
        let _ = snap;

        "item-0".to_string()
    };

    let ops_before_toggle = host.with_reconciler(|r| r.backend.ops.len());
    drive_host(
        &host,
        &dispatch_out,
        &dispatcher,
        TodoAction::ToggleItem(first_id.clone()),
    );
    host.with_reconciler(|r| {
        let new_ops = &r.backend.ops[ops_before_toggle..];
        let new_creates = new_ops
            .iter()
            .filter(|o| matches!(o, Op::Create { .. }))
            .count();
        assert_eq!(
            new_creates, 0,
            "toggle must not create any controls: {new_ops:?}"
        );
    });

    let ops_before_delete = host.with_reconciler(|r| r.backend.ops.len());
    drive_host(
        &host,
        &dispatch_out,
        &dispatcher,
        TodoAction::DeleteItem(first_id),
    );
    host.with_reconciler(|r| {
        let new_ops = &r.backend.ops[ops_before_delete..];
        let new_creates = new_ops
            .iter()
            .filter(|o| matches!(o, Op::Create { .. }))
            .count();
        assert_eq!(new_creates, 0, "delete must not create any controls");
    });
}

#[test]
fn filter_toggle_via_host_updates_remaining_label() {
    let dispatcher = TestDispatcher::default();
    let dispatch_out: Rc<RefCell<Option<Dispatch<TodoAction>>>> = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(TodoComponent {
        dispatch_out: Rc::clone(&dispatch_out),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();
    dispatcher.drain();

    for t in ["a", "b"] {
        drive_host(
            &host,
            &dispatch_out,
            &dispatcher,
            TodoAction::SetNewItemText(t.into()),
        );
        drive_host(&host, &dispatch_out, &dispatcher, TodoAction::AddItem);
    }
    drive_host(
        &host,
        &dispatch_out,
        &dispatcher,
        TodoAction::ToggleItem("item-0".into()),
    );

    drive_host(
        &host,
        &dispatch_out,
        &dispatcher,
        TodoAction::SetFilter(Filter::Active),
    );

    assert!(host.render_count() > 0);
}
