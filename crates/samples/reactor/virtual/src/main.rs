#![windows_subsystem = "windows"]

use std::rc::Rc;
use std::time::Duration;
use windows_reactor::*;

const INITIAL_TASKS: usize = 100;
const LOAD_TASKS: usize = 100;
const STRESS_TASKS: usize = 1_000;

#[derive(Clone, PartialEq)]
struct Task {
    id: u64,
    title: String,
    done: bool,
}

struct TaskEditor {
    load_generation: u64,
    loading: bool,
    next_id: u64,
    selected: Option<u64>,
    sender: LocalSender<Message>,
    source_revision: u64,
    status: String,
    tasks: Vec<Rc<Task>>,
}

#[derive(Clone)]
enum Message {
    Add,
    Load,
    Loaded { generation: u64, tasks: Vec<Task> },
    LoadingCancelled(u64),
    MoveFirstToEnd,
    Reverse,
    Row(RowAction),
    Stress,
}

#[derive(Clone)]
enum RowAction {
    Remove(u64),
    Rename { id: u64, title: String },
    Select(u64),
    SetDone { id: u64, done: bool },
}

#[derive(Clone)]
struct RowInput {
    item: Rc<Task>,
    selected: bool,
    sender: LocalSender<Message>,
}

impl PartialEq for RowInput {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item && self.selected == other.selected
    }
}

struct TaskRow {
    draft: String,
    editing: bool,
    id: u64,
    input: ElementRef<TextBox>,
    sender: LocalSender<Message>,
}

#[derive(Clone)]
enum RowMessage {
    Draft(String),
    Edit,
    Remove,
    Save,
    Select,
    SetDone(bool),
}

impl TaskEditor {
    fn tasks(start: u64, count: usize) -> Vec<Task> {
        (0..count)
            .map(|offset| {
                let id = start + offset as u64;
                Task {
                    id,
                    title: format!("Task {id}"),
                    done: false,
                }
            })
            .collect()
    }

    fn select_after_remove(&mut self, removed: u64) {
        if self.selected == Some(removed) {
            self.selected = self.tasks.first().map(|task| task.id);
        }
    }

    fn bump_source_revision(&mut self) {
        self.source_revision = self.source_revision.checked_add(1).unwrap();
    }
}

impl Component for TaskEditor {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let sender = context.sender();
        Self {
            load_generation: 0,
            loading: false,
            next_id: INITIAL_TASKS as u64,
            selected: Some(0),
            sender,
            source_revision: 0,
            status: format!("{INITIAL_TASKS} tasks"),
            tasks: Self::tasks(0, INITIAL_TASKS)
                .into_iter()
                .map(Rc::new)
                .collect(),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Add => {
                let id = self.next_id;
                self.next_id += 1;
                self.tasks.insert(
                    0,
                    Rc::new(Task {
                        id,
                        title: format!("New task {id}"),
                        done: false,
                    }),
                );
                self.selected = Some(id);
                self.bump_source_revision();
                self.status = format!("Inserted task {id} at the front");
            }
            Message::Load if !self.loading => {
                self.loading = true;
                let generation = self.load_generation;
                let start = self.next_id;
                self.next_id += LOAD_TASKS as u64;
                self.status = "Loading 100 tasks in the background...".to_string();
                context.spawn_background(move |cancellation| {
                    std::thread::sleep(Duration::from_millis(250));
                    if cancellation.is_cancelled() {
                        Message::LoadingCancelled(generation)
                    } else {
                        Message::Loaded {
                            generation,
                            tasks: Self::tasks(start, LOAD_TASKS),
                        }
                    }
                });
            }
            Message::Loaded {
                generation,
                mut tasks,
            } if generation == self.load_generation => {
                self.tasks.extend(tasks.drain(..).map(Rc::new));
                self.bump_source_revision();
                self.loading = false;
                self.status = format!("Loaded; {} tasks", self.tasks.len());
            }
            Message::LoadingCancelled(generation) if generation == self.load_generation => {
                self.loading = false;
                self.status = "Loading cancelled".to_string();
            }
            Message::MoveFirstToEnd if self.tasks.len() > 1 => {
                let first = self.tasks.remove(0);
                self.status = format!("Moved task {} to the end", first.id);
                self.tasks.push(first);
                self.bump_source_revision();
            }
            Message::Reverse => {
                self.tasks.reverse();
                self.bump_source_revision();
                self.status = "Reversed all task keys".to_string();
            }
            Message::Stress => {
                self.load_generation = self.load_generation.checked_add(1).unwrap();
                self.loading = false;
                self.tasks = Self::tasks(self.next_id, STRESS_TASKS)
                    .into_iter()
                    .map(Rc::new)
                    .collect();
                self.next_id += STRESS_TASKS as u64;
                self.selected = self.tasks.first().map(|task| task.id);
                self.bump_source_revision();
                self.status = format!("Reset to {STRESS_TASKS} tasks");
            }
            Message::Row(RowAction::Remove(id)) => {
                let old_len = self.tasks.len();
                self.tasks.retain(|task| task.id != id);
                if self.tasks.len() != old_len {
                    self.bump_source_revision();
                }
                self.select_after_remove(id);
                self.status = format!("Removed task {id}; {} remain", self.tasks.len());
            }
            Message::Row(RowAction::Rename { id, title }) => {
                if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
                    Rc::make_mut(task).title = title;
                    self.status = format!("Saved task {id}");
                }
            }
            Message::Row(RowAction::Select(id)) => {
                self.selected = Some(id);
                self.status = format!("Selected task {id}");
            }
            Message::Row(RowAction::SetDone { id, done }) => {
                if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
                    Rc::make_mut(task).done = done;
                    self.status = format!("Task {id} is {}", if done { "done" } else { "open" });
                }
            }
            Message::Load
            | Message::Loaded { .. }
            | Message::LoadingCancelled(_)
            | Message::MoveFirstToEnd => {}
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let tasks = Rc::new(self.tasks.clone());
        let key_tasks = Rc::clone(&tasks);
        let selected = self.selected;
        let sender = self.sender.clone();
        let rows = VirtualSource::new(
            self.source_revision,
            tasks.len(),
            move |index| key_tasks[index].id,
            move |index| {
                let item = Rc::clone(&tasks[index]);
                let id = item.id;
                View::component::<TaskRow>(RowInput {
                    item,
                    selected: selected == Some(id),
                    sender: sender.clone(),
                })
            },
        );

        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                StackPanel::new().spacing(8.0).children((
                    TextBlock::new()
                        .text("Virtual task editor")
                        .text_wrapping(TextWrapping::Wrap),
                    self.status.clone(),
                    StackPanel::new().spacing(4.0).children((
                        Button::new()
                            .on_click(context.message(Message::Add))
                            .content("Add at front"),
                        Button::new()
                            .on_click(context.message(Message::MoveFirstToEnd))
                            .content("Move first to end"),
                        Button::new()
                            .on_click(context.message(Message::Reverse))
                            .content("Reverse"),
                        Button::new()
                            .is_enabled(!self.loading)
                            .on_click(context.message(Message::Load))
                            .content("Load 100"),
                        Button::new()
                            .on_click(context.message(Message::Stress))
                            .content("Reset to 1,000"),
                    )),
                    ProgressBar::new()
                        .minimum(0.0)
                        .maximum(1.0)
                        .is_indeterminate(self.loading),
                )),
                ScrollViewer::new()
                    .grid_row(1)
                    .margin(Thickness::new(0.0, 8.0, 0.0, 0.0))
                    .content(ItemsRepeater::new().virtual_source(rows)),
            ))
            .into()
    }
}

impl Component for TaskRow {
    type Message = RowMessage;
    type Input = RowInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            draft: input.item.title.clone(),
            editing: false,
            id: input.item.id,
            input: ElementRef::new(),
            sender: input.sender.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.id = input.item.id;
        self.sender = input.sender.clone();
        if !self.editing {
            self.draft.clone_from(&input.item.title);
        }
    }

    fn update(&mut self, message: RowMessage, _context: &ComponentContext<Self>) {
        match message {
            RowMessage::Draft(value) => {
                self.draft.clone_from(&value);
                if !value.trim().is_empty() {
                    _ = self.sender.send(Message::Row(RowAction::Rename {
                        id: self.item_id(),
                        title: value,
                    }));
                }
            }
            RowMessage::Edit => self.editing = true,
            RowMessage::Remove => {
                _ = self
                    .sender
                    .send(Message::Row(RowAction::Remove(self.item_id())));
            }
            RowMessage::Save if self.draft.trim().is_empty() => {
                _ = self.input.request_focus();
            }
            RowMessage::Save => {
                self.editing = false;
                _ = self.sender.send(Message::Row(RowAction::Rename {
                    id: self.item_id(),
                    title: self.draft.clone(),
                }));
            }
            RowMessage::Select => {
                _ = self
                    .sender
                    .send(Message::Row(RowAction::Select(self.item_id())));
            }
            RowMessage::SetDone(done) => {
                _ = self.sender.send(Message::Row(RowAction::SetDone {
                    id: self.item_id(),
                    done,
                }));
            }
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let editing = self.editing;
        let input_ref = self.input.clone();
        context.use_effect("edit-focus", editing, move || {
            if editing {
                _ = input_ref.request_focus();
            }
            None
        });
        let mut children = vec![
            format!(
                "#{}{}",
                input.item.id,
                if input.selected { " selected" } else { "" }
            )
            .into(),
            ToggleSwitch::new()
                .is_on(input.item.done)
                .on_toggled(context.callback(RowMessage::SetDone))
                .into(),
            TextBox::new()
                .text(self.draft.clone())
                .element_ref(&self.input)
                .is_enabled(editing)
                .on_text_changed(
                    context.callback(|value: Rc<str>| RowMessage::Draft(value.to_string())),
                )
                .into(),
        ];
        if editing {
            children.push("Editing; title must not be empty".into());
        }
        children.push(
            StackPanel::new()
                .spacing(2.0)
                .children((
                    Button::new()
                        .on_click(context.message(RowMessage::Select))
                        .content("Select"),
                    Button::new()
                        .is_enabled(!editing)
                        .on_click(context.message(RowMessage::Edit))
                        .content("Edit"),
                    Button::new()
                        .is_enabled(editing)
                        .on_click(context.message(RowMessage::Save))
                        .content("Save"),
                    Button::new()
                        .on_click(context.message(RowMessage::Remove))
                        .content("Remove"),
                ))
                .into(),
        );

        StackPanel::new().spacing(4.0).children(children).into()
    }
}

impl TaskRow {
    fn item_id(&self) -> u64 {
        self.id
    }
}

fn main() {
    App::run_component::<TaskEditor>(()).unwrap();
}
