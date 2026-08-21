#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use windows_reactor_next::*;

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
    selected_context: Rc<Context<bool>>,
    sender: LocalSender<Message>,
    status: String,
    tasks: Vec<Task>,
}

#[derive(Clone, Default)]
struct EditorProps {
    metrics: Rc<RowMetrics>,
    sender: Rc<RefCell<Option<LocalSender<Message>>>>,
}

impl PartialEq for EditorProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.metrics, &other.metrics) && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

#[derive(Default)]
struct RowMetrics {
    cleanups: Cell<usize>,
    creates: Cell<usize>,
    #[cfg(test)]
    sender: RefCell<Option<LocalSender<RowMessage>>>,
    setups: Cell<usize>,
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
struct RowProps {
    item: Task,
    metrics: Rc<RowMetrics>,
    selected_context: Rc<Context<bool>>,
    sender: LocalSender<Message>,
}

impl PartialEq for RowProps {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
            && Rc::ptr_eq(&self.metrics, &other.metrics)
            && Rc::ptr_eq(&self.selected_context, &other.selected_context)
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
}

impl Component for TaskEditor {
    type Message = Message;
    type Props = EditorProps;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        let sender = context.sender();
        *props.sender.borrow_mut() = Some(sender.clone());
        Self {
            load_generation: 0,
            loading: false,
            next_id: INITIAL_TASKS as u64,
            selected: Some(0),
            selected_context: Rc::new(Context::new(false)),
            sender,
            status: format!("{INITIAL_TASKS} tasks"),
            tasks: Self::tasks(0, INITIAL_TASKS),
        }
    }

    fn update(&mut self, message: Message, context: &mut ComponentContext<Self>) {
        match message {
            Message::Add => {
                let id = self.next_id;
                self.next_id += 1;
                self.tasks.insert(
                    0,
                    Task {
                        id,
                        title: format!("New task {id}"),
                        done: false,
                    },
                );
                self.selected = Some(id);
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
                self.tasks.append(&mut tasks);
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
            }
            Message::Reverse => {
                self.tasks.reverse();
                self.status = "Reversed all task keys".to_string();
            }
            Message::Stress => {
                self.load_generation = self.load_generation.checked_add(1).unwrap();
                self.loading = false;
                self.tasks = Self::tasks(self.next_id, STRESS_TASKS);
                self.next_id += STRESS_TASKS as u64;
                self.selected = self.tasks.first().map(|task| task.id);
                self.status = format!("Reset to {STRESS_TASKS} tasks");
            }
            Message::Row(RowAction::Remove(id)) => {
                self.tasks.retain(|task| task.id != id);
                self.select_after_remove(id);
                self.status = format!("Removed task {id}; {} remain", self.tasks.len());
            }
            Message::Row(RowAction::Rename { id, title }) => {
                if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
                    task.title = title;
                    self.status = format!("Saved task {id}");
                }
            }
            Message::Row(RowAction::Select(id)) => {
                self.selected = Some(id);
                self.status = format!("Selected task {id}");
            }
            Message::Row(RowAction::SetDone { id, done }) => {
                if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
                    task.done = done;
                    self.status = format!("Task {id} is {}", if done { "done" } else { "open" });
                }
            }
            Message::Load
            | Message::Loaded { .. }
            | Message::LoadingCancelled(_)
            | Message::MoveFirstToEnd => {}
        }
    }

    fn view(&self, props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        let rows = self.tasks.iter().map(|item| {
            let id = item.id;
            KeyedView::new(
                id,
                View::provide(
                    &self.selected_context,
                    self.selected == Some(id),
                    View::component::<TaskRow>(RowProps {
                        item: item.clone(),
                        metrics: Rc::clone(&props.metrics),
                        selected_context: Rc::clone(&self.selected_context),
                        sender: self.sender.clone(),
                    }),
                ),
            )
        });

        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text("Virtual task editor")
                .text_wrapping(TextWrapping::Wrap),
            TextBlock::new().text(self.status.clone()),
            StackPanel::new().spacing(4.0).children((
                Button::new()
                    .on_click(context.message(Message::Add))
                    .content(TextBlock::new().text("Add at front")),
                Button::new()
                    .on_click(context.message(Message::MoveFirstToEnd))
                    .content(TextBlock::new().text("Move first to end")),
                Button::new()
                    .on_click(context.message(Message::Reverse))
                    .content(TextBlock::new().text("Reverse")),
                Button::new()
                    .is_enabled(!self.loading)
                    .on_click(context.message(Message::Load))
                    .content(TextBlock::new().text("Load 100")),
                Button::new()
                    .on_click(context.message(Message::Stress))
                    .content(TextBlock::new().text("Reset to 1,000")),
            )),
            ProgressBar::new()
                .minimum(0.0)
                .maximum(1.0)
                .is_indeterminate(self.loading),
            ScrollViewer::new().content(ItemsRepeater::new().items(rows)),
        ))
    }
}

impl Component for TaskRow {
    type Message = RowMessage;
    type Props = RowProps;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        props.metrics.creates.set(props.metrics.creates.get() + 1);
        #[cfg(test)]
        props.metrics.sender.replace(Some(_context.sender()));
        Self {
            draft: props.item.title.clone(),
            editing: false,
            id: props.item.id,
            input: ElementRef::new(),
            sender: props.sender.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.id = props.item.id;
        self.sender = props.sender.clone();
        if !self.editing {
            self.draft.clone_from(&props.item.title);
        }
    }

    fn update(&mut self, message: RowMessage, _context: &mut ComponentContext<Self>) {
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

    fn view(&self, props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        let selected = context.use_context(&props.selected_context);
        let editing = self.editing;
        let input = self.input.clone();
        let metrics = Rc::clone(&props.metrics);
        context.use_effect("edit-focus", editing, move || {
            metrics.setups.set(metrics.setups.get() + 1);
            if editing {
                _ = input.request_focus();
            }
            Some(Box::new(move || {
                metrics.cleanups.set(metrics.cleanups.get() + 1);
            }))
        });
        let edit_status = if editing {
            TextBlock::new()
                .text("Editing; title must not be empty")
                .into()
        } else {
            View::empty()
        };

        StackPanel::new().spacing(4.0).children((
            TextBlock::new().text(format!(
                "#{}{}",
                props.item.id,
                if selected { " selected" } else { "" }
            )),
            ToggleSwitch::new()
                .is_on(props.item.done)
                .on_toggled(context.callback(RowMessage::SetDone)),
            TextBox::new()
                .element_ref(&self.input)
                .text(self.draft.clone())
                .is_enabled(editing)
                .on_text_changed(context.callback(RowMessage::Draft)),
            edit_status,
            StackPanel::new().spacing(2.0).children((
                Button::new()
                    .on_click(context.message(RowMessage::Select))
                    .content(TextBlock::new().text("Select")),
                Button::new()
                    .is_enabled(!editing)
                    .on_click(context.message(RowMessage::Edit))
                    .content(TextBlock::new().text("Edit")),
                Button::new()
                    .is_enabled(editing)
                    .on_click(context.message(RowMessage::Save))
                    .content(TextBlock::new().text("Save")),
                Button::new()
                    .on_click(context.message(RowMessage::Remove))
                    .content(TextBlock::new().text("Remove")),
            )),
        ))
    }
}

impl TaskRow {
    fn item_id(&self) -> u64 {
        self.id
    }
}

fn main() {
    bootstrap().unwrap();
    App::run_component::<TaskEditor>(EditorProps::default()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn virtual_collection(pump: &Pump<RecordingRuntime>) -> NodeId {
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .find_map(|command| match command {
                Command::CreateVirtualCollection { node, .. } => Some(*node),
                _ => None,
            })
            .unwrap()
    }

    fn live_text(pump: &Pump<RecordingRuntime>, expected: &str) -> bool {
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .filter_map(|command| match command {
                Command::SetProperty {
                    node,
                    property: PropertyId::TextBoxText,
                    ..
                } => Some(*node),
                _ => None,
            })
            .any(|node| {
                pump.runtime()
                    .node(node)
                    .and_then(|node| node.property(PropertyId::TextBoxText))
                    == Some(&PropertyValue::Str(expected.into()))
            })
    }

    #[test]
    fn edit_survives_reorder_recycle_and_realization() {
        let props = EditorProps::default();
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<TaskEditor>(props.clone()))
            .unwrap();
        let collection = virtual_collection(&pump);
        pump.runtime_mut()
            .queue_realize(collection, RealizedContainer(1), 0);
        pump.process_realizations().unwrap();

        assert_eq!(props.metrics.creates.get(), 1);
        assert_eq!(props.metrics.setups.get(), 1);
        let input = pump
            .runtime()
            .commands()
            .iter()
            .flatten()
            .find_map(|command| match command {
                Command::SubscribeEvent {
                    node,
                    event: EventId::TextBoxTextChanged,
                    ..
                } if pump.runtime().node(*node).is_some() => Some(*node),
                _ => None,
            })
            .unwrap();

        assert!(
            props
                .metrics
                .sender
                .borrow()
                .as_ref()
                .unwrap()
                .send(RowMessage::Edit)
        );
        assert_eq!(pump.dispatch_components(1), Ok(1));
        assert_eq!(props.metrics.setups.get(), 2);
        assert_eq!(props.metrics.cleanups.get(), 1);
        assert_eq!(
            pump.runtime()
                .node(input)
                .and_then(|node| node.property(PropertyId::TextBoxIsEnabled)),
            Some(&PropertyValue::Bool(true))
        );
        assert_eq!(pump.process_imperatives(), Ok(1));
        assert_eq!(
            pump.runtime()
                .commands()
                .iter()
                .flatten()
                .filter(|command| matches!(command, Command::Focus { node } if *node == input))
                .count(),
            1
        );

        let revision = pump
            .event_revision(input, EventId::TextBoxTextChanged)
            .unwrap();
        pump.queue_event(QueuedEvent::new(
            input,
            EventId::TextBoxTextChanged,
            revision,
            EventPayload::Str("Edited task".into()),
        ));
        assert_eq!(pump.dispatch_events(), Ok(1));
        assert!(pump.dispatch_components(64).unwrap() >= 2);
        assert!(live_text(&pump, "Edited task"));

        pump.runtime_mut()
            .queue_recycle(collection, RealizedContainer(1));
        pump.process_realizations().unwrap();
        assert_eq!(props.metrics.cleanups.get(), 2);

        pump.runtime_mut()
            .queue_realize(collection, RealizedContainer(2), 0);
        pump.process_realizations().unwrap();
        assert_eq!(props.metrics.creates.get(), 2);
        assert_eq!(props.metrics.setups.get(), 3);
        assert_eq!(props.metrics.cleanups.get(), 2);
        assert!(live_text(&pump, "Edited task"));

        assert!(
            props
                .sender
                .borrow()
                .as_ref()
                .unwrap()
                .send(Message::Reverse)
        );
        assert_eq!(pump.dispatch_components(64), Ok(1));
        assert_eq!(props.metrics.cleanups.get(), 3);
        assert_eq!(pump.runtime().source_revision(collection), Some(1));

        pump.runtime_mut()
            .queue_realize(collection, RealizedContainer(3), INITIAL_TASKS - 1);
        pump.process_realizations().unwrap();
        assert_eq!(props.metrics.creates.get(), 3);
        assert_eq!(props.metrics.setups.get(), 4);
        assert_eq!(props.metrics.cleanups.get(), 3);
        assert!(live_text(&pump, "Edited task"));
    }

    #[test]
    fn reset_ignores_an_in_flight_load_result() {
        let props = EditorProps::default();
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<TaskEditor>(props.clone()))
            .unwrap();
        let collection = virtual_collection(&pump);
        let sender = props.sender.borrow().as_ref().unwrap().clone();

        assert!(sender.send(Message::Load));
        assert_eq!(pump.dispatch_components(1), Ok(1));
        assert!(sender.send(Message::Stress));
        assert_eq!(pump.dispatch_components(1), Ok(1));
        assert_eq!(pump.runtime().source_revision(collection), Some(1));

        assert!(sender.send(Message::Loaded {
            generation: 0,
            tasks: TaskEditor::tasks(INITIAL_TASKS as u64, LOAD_TASKS),
        }));
        assert_eq!(pump.dispatch_components(1), Ok(1));
        assert_eq!(pump.runtime().source_revision(collection), Some(1));
    }
}
