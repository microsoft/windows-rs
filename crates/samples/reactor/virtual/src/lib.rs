use std::cell::{Cell, RefCell};
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
struct EditorInput {
    metrics: Rc<RowMetrics>,
    render_revision: u64,
    sender: Rc<RefCell<Option<LocalSender<Message>>>>,
    task_count: usize,
}

impl Default for EditorInput {
    fn default() -> Self {
        Self {
            metrics: Rc::default(),
            render_revision: 0,
            sender: Rc::default(),
            task_count: INITIAL_TASKS,
        }
    }
}

impl PartialEq for EditorInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.metrics, &other.metrics)
            && self.render_revision == other.render_revision
            && Rc::ptr_eq(&self.sender, &other.sender)
            && self.task_count == other.task_count
    }
}

#[derive(Default)]
struct RowMetrics {
    cleanups: Cell<usize>,
    creates: Cell<usize>,
    #[cfg(any(test, feature = "perf"))]
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
struct RowInput {
    item: Rc<Task>,
    metrics: Rc<RowMetrics>,
    selected: bool,
    sender: LocalSender<Message>,
}

impl PartialEq for RowInput {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
            && Rc::ptr_eq(&self.metrics, &other.metrics)
            && self.selected == other.selected
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
    type Input = EditorInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let sender = context.sender();
        *input.sender.borrow_mut() = Some(sender.clone());
        Self {
            load_generation: 0,
            loading: false,
            next_id: input.task_count as u64,
            selected: Some(0),
            sender,
            source_revision: 0,
            status: format!("{} tasks", input.task_count),
            tasks: Self::tasks(0, input.task_count)
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

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let tasks = Rc::new(self.tasks.clone());
        let key_tasks = Rc::clone(&tasks);
        let metrics = Rc::clone(&input.metrics);
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
                    metrics: Rc::clone(&metrics),
                    selected: selected == Some(id),
                    sender: sender.clone(),
                })
            },
        );

        StackPanel::new()
            .spacing(8.0)
            .children((
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
                ScrollViewer::new().content(ItemsRepeater::new().virtual_source(rows)),
            ))
            .into()
    }
}

impl Component for TaskRow {
    type Message = RowMessage;
    type Input = RowInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        input.metrics.creates.set(input.metrics.creates.get() + 1);
        #[cfg(any(test, feature = "perf"))]
        input.metrics.sender.replace(Some(_context.sender()));
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
        let metrics = Rc::clone(&input.metrics);
        context.use_effect("edit-focus", editing, move || {
            metrics.setups.set(metrics.setups.get() + 1);
            if editing {
                _ = input_ref.request_focus();
            }
            Some(Box::new(move || {
                metrics.cleanups.set(metrics.cleanups.get() + 1);
            }))
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
            TextBox::new(self.draft.clone())
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

pub fn run() {
    App::run_component::<TaskEditor>(EditorInput::default()).unwrap();
}

#[cfg(feature = "perf")]
#[doc(hidden)]
pub mod performance {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Instant;

    const REALIZED_ROWS: usize = 32;
    const SETTLE_TICKS: usize = 5;
    const WARMUP_TICKS: usize = 30;

    pub struct Scenario {
        collection: ObjectId,
        containers: Vec<RealizedContainer>,
        edit: usize,
        host: ComponentHost<RecordingAdapter>,
        next_container: u64,
        next_index: usize,
        input: EditorInput,
        selected: u64,
        task_count: usize,
    }

    impl Scenario {
        pub fn new() -> Self {
            Self::with_task_count(STRESS_TASKS)
        }

        pub fn with_task_count(task_count: usize) -> Self {
            assert!(task_count > REALIZED_ROWS);
            let input = EditorInput {
                task_count,
                ..Default::default()
            };
            let mut host = ComponentHost::mount(
                RecordingAdapter::default(),
                [component::<TaskEditor>("root", input.clone())],
            )
            .unwrap();
            let collection = host
                .runtime()
                .graph()
                .objects()
                .find(|object| {
                    host.runtime().graph().kind(*object) == Some(ObjectType::ItemsRepeater)
                })
                .unwrap();
            let source_revision = host
                .runtime()
                .graph()
                .virtual_source_revision(collection)
                .unwrap();

            let containers = (0..REALIZED_ROWS)
                .map(|value| RealizedContainer(value as u64))
                .collect::<Vec<_>>();
            for (index, container) in containers.iter().copied().enumerate() {
                host.test_adapter_mut()
                    .queue_realization(RealizationRequest::Realize {
                        collection,
                        container,
                        index,
                        source_revision,
                    });
            }
            host.drain(usize::MAX).unwrap();

            assert!(
                input
                    .metrics
                    .sender
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .send(RowMessage::Edit)
            );
            host.drain(usize::MAX).unwrap();

            Self {
                collection,
                containers,
                edit: 0,
                host,
                next_container: REALIZED_ROWS as u64,
                next_index: REALIZED_ROWS,
                input,
                selected: task_count as u64 / 10,
                task_count,
            }
        }

        fn editor_sender(&self) -> LocalSender<Message> {
            self.input.sender.borrow().as_ref().unwrap().clone()
        }

        pub fn local_edit(&mut self) {
            self.edit += 1;
            assert!(
                self.input
                    .metrics
                    .sender
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .send(RowMessage::Draft(format!("Edited task {}", self.edit)))
            );
            self.host.drain(usize::MAX).unwrap();
        }

        pub fn broad_selection_change(&mut self) {
            self.selected = if self.selected == self.task_count as u64 / 10 {
                self.task_count as u64 / 10 + 1
            } else {
                self.task_count as u64 / 10
            };
            assert!(
                self.editor_sender()
                    .send(Message::Row(RowAction::Select(self.selected)))
            );
            self.host.drain(usize::MAX).unwrap();
        }

        pub fn redundant_parent_message(&mut self) {
            assert!(
                self.editor_sender()
                    .send(Message::Row(RowAction::Select(self.selected)))
            );
            self.host.drain(usize::MAX).unwrap();
        }

        pub fn unchanged_root_component_memo_hit(&mut self) {
            self.host
                .update_input::<TaskEditor>(&Key::from("root"), self.input.clone())
                .unwrap();
        }

        pub fn value_equal_root_recomposition(&mut self) {
            let mut input = self.input.clone();
            input.render_revision = input.render_revision.checked_add(1).unwrap();
            self.host
                .update_input::<TaskEditor>(&Key::from("root"), input.clone())
                .unwrap();
            self.input = input;
        }

        pub fn realize_recycle_batch(&mut self) {
            let source_revision = self
                .host
                .runtime()
                .graph()
                .virtual_source_revision(self.collection)
                .unwrap();
            for container in self.containers.drain(..) {
                self.host
                    .test_adapter_mut()
                    .queue_realization(RealizationRequest::Recycle {
                        collection: self.collection,
                        container,
                        source_revision,
                    });
            }

            self.next_index = (self.next_index + REALIZED_ROWS) % (self.task_count - REALIZED_ROWS);
            self.containers = (0..REALIZED_ROWS)
                .map(|offset| {
                    let container = RealizedContainer(self.next_container);
                    self.next_container += 1;
                    self.host
                        .test_adapter_mut()
                        .queue_realization(RealizationRequest::Realize {
                            collection: self.collection,
                            container,
                            index: self.next_index + offset,
                            source_revision,
                        });
                    container
                })
                .collect();
            self.host.drain(usize::MAX).unwrap();
        }

        pub fn background_completion(&mut self) {
            assert!(self.editor_sender().send(Message::Loaded {
                generation: 0,
                tasks: Vec::new(),
            }));
            self.host.drain(usize::MAX).unwrap();
        }

        pub fn mixed_virtual_cycle(&mut self) {
            self.background_completion();
            self.broad_selection_change();
            self.realize_recycle_batch();
        }
    }

    impl Default for Scenario {
        fn default() -> Self {
            Self::new()
        }
    }

    struct LivePerformance {
        _editor: EditorInput,
        _tick: LiveTickSubscription,
    }

    struct LiveStats {
        active: bool,
        complete: bool,
        last_tick: Option<Instant>,
        samples: Vec<f64>,
        settle_ticks: Option<usize>,
        tick: usize,
    }

    #[derive(Clone, Copy, PartialEq)]
    struct LiveInput {
        active: bool,
        samples: usize,
    }

    impl Component for LivePerformance {
        type Message = ();
        type Input = LiveInput;

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            let editor = EditorInput::default();
            let target_samples = input.samples;
            let active = input.active;
            let stats = Rc::new(RefCell::new(LiveStats {
                active,
                complete: false,
                last_tick: None,
                samples: Vec::with_capacity(target_samples),
                settle_ticks: None,
                tick: 0,
            }));
            let done = Arc::new(AtomicBool::new(false));
            let tick = subscribe_live_tick({
                let done = Arc::clone(&done);
                let editor = editor.clone();
                let stats = Rc::clone(&stats);
                move || {
                    let now = Instant::now();
                    let mut stats = stats.borrow_mut();
                    if stats.complete {
                        return;
                    }
                    stats.tick += 1;
                    if let Some(remaining) = stats.settle_ticks {
                        if remaining != 0 {
                            stats.settle_ticks = Some(remaining - 1);
                            return;
                        }
                        let result = match live_virtual_shell_counts() {
                            Ok((_, 0)) => Ok(()),
                            Ok((live, retired)) => Err(format!(
                                "{retired} retired virtual shells remain after settling \
                                 ({live} live)"
                            )),
                            Err(error) => {
                                Err(format!("could not inspect virtual shells: {error:?}"))
                            }
                        };
                        finish_live(&mut stats, &done, result);
                        return;
                    }
                    if stats.tick <= WARMUP_TICKS {
                        stats.last_tick = Some(now);
                        return;
                    }
                    if let Some(last) = stats.last_tick.replace(now) {
                        stats
                            .samples
                            .push(now.duration_since(last).as_secs_f64() * 1_000.0);
                    }

                    if stats.active {
                        let measured_tick = stats.tick - WARMUP_TICKS;
                        let index = (measured_tick * REALIZED_ROWS) % STRESS_TASKS;
                        if let Err(error) = bring_live_virtual_index(index) {
                            finish_live(
                                &mut stats,
                                &done,
                                Err(format!("virtual scroll failed: {error:?}")),
                            );
                            return;
                        }
                        if let Some(sender) = editor.sender.borrow().as_ref() {
                            _ = sender.send(Message::Row(RowAction::Select(
                                STRESS_TASKS as u64 / 10 + (measured_tick % 2) as u64,
                            )));
                            if measured_tick.is_multiple_of(30) {
                                _ = sender.send(Message::Loaded {
                                    generation: 1,
                                    tasks: Vec::new(),
                                });
                            }
                        }
                        if measured_tick.is_multiple_of(6)
                            && let Some(sender) = editor.metrics.sender.borrow().as_ref()
                        {
                            _ = sender
                                .send(RowMessage::Draft(format!("Live edit {measured_tick}")));
                        }
                    }
                    if stats.samples.len() >= target_samples {
                        stats.settle_ticks = Some(SETTLE_TICKS);
                    }
                }
            })
            .unwrap();

            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(20));
                if !done.swap(true, Ordering::AcqRel) {
                    _ = std::fs::write(
                        live_report_path(active),
                        "windows-reactor live virtual editor\nno dispatcher ticks captured\n",
                    );
                    std::process::exit(2);
                }
            });

            Self {
                _editor: editor,
                _tick: tick,
            }
        }

        fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &LiveInput, context: &mut ViewContext<Self>) -> View {
            let editor = self._editor.clone();
            context.use_effect("start", (), move || {
                if let Some(sender) = editor.sender.borrow().as_ref() {
                    _ = sender.send(Message::Stress);
                }
                None
            });
            Grid::new()
                .children([component::<TaskEditor>("editor", self._editor.clone()).into()])
                .into()
        }
    }

    fn finish_live(stats: &mut LiveStats, done: &AtomicBool, result: Result<(), String>) {
        stats.complete = true;
        if done.swap(true, Ordering::AcqRel) {
            return;
        }
        let success = result.is_ok();
        let report = match result {
            Ok(()) => live_report(&stats.samples, stats.active),
            Err(error) => format!("windows-reactor live virtual editor\n{error}\n"),
        };
        std::fs::write(live_report_path(stats.active), report).unwrap();
        schedule_live_test_exit(success).unwrap();
    }

    fn live_report(samples: &[f64], active: bool) -> String {
        let mut sorted = samples.to_vec();
        sorted.sort_by(f64::total_cmp);
        let percentile = |value: f64| {
            let index = ((sorted.len() - 1) as f64 * value).ceil() as usize;
            sorted[index]
        };
        let average = sorted.iter().sum::<f64>() / sorted.len() as f64;
        let over_25_ms = sorted.iter().filter(|value| **value > 25.0).count();
        let over_two_frames = sorted.iter().filter(|value| **value > 33.4).count();
        format!(
            "windows-reactor live virtual editor ({})\n\
             ticks: {}\n\
             average: {:.2} ms\n\
             median: {:.2} ms\n\
             p95: {:.2} ms\n\
             p99: {:.2} ms\n\
             max: {:.2} ms\n\
             >25 ms: {}\n\
             >33.4 ms: {}\n",
            if active { "active" } else { "baseline" },
            sorted.len(),
            average,
            percentile(0.50),
            percentile(0.95),
            percentile(0.99),
            sorted.last().unwrap(),
            over_25_ms,
            over_two_frames,
        )
    }

    fn live_report_path(active: bool) -> std::path::PathBuf {
        let name = if active {
            "reactor-virtual-live-perf.txt"
        } else {
            "reactor-virtual-live-baseline.txt"
        };
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(name)
    }

    pub fn run_live(samples: usize, active: bool) {
        assert!(samples > 0);
        App::run_component::<LivePerformance>(LiveInput { active, samples }).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mount(input: EditorInput) -> ComponentHost<RecordingAdapter> {
        ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<TaskEditor>("root", input)],
        )
        .unwrap()
    }

    fn virtual_collection(host: &ComponentHost<RecordingAdapter>) -> ObjectId {
        host.runtime()
            .graph()
            .objects()
            .find(|object| host.runtime().graph().kind(*object) == Some(ObjectType::ItemsRepeater))
            .unwrap()
    }

    fn live_text(host: &ComponentHost<RecordingAdapter>, expected: &str) -> bool {
        host.runtime().graph().objects().any(|object| {
            host.runtime()
                .graph()
                .properties(object)
                .is_some_and(|properties| {
                    properties.iter().any(|property| {
                        property.id == PropertyId::Text
                            && property.value == PropertyValue::String(expected.into())
                    })
                })
        })
    }

    fn event(
        host: &ComponentHost<RecordingAdapter>,
        object: ObjectId,
        event: EventId,
    ) -> EventValue {
        host.runtime()
            .graph()
            .events(object)
            .unwrap()
            .iter()
            .find(|candidate| candidate.id == event)
            .unwrap()
            .value
            .clone()
    }

    fn realize(
        host: &mut ComponentHost<RecordingAdapter>,
        collection: ObjectId,
        container: RealizedContainer,
        index: usize,
    ) {
        let source_revision = host
            .runtime()
            .graph()
            .virtual_source_revision(collection)
            .unwrap();
        host.test_adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index,
                source_revision,
            });
        host.drain(64).unwrap();
    }

    fn recycle(
        host: &mut ComponentHost<RecordingAdapter>,
        collection: ObjectId,
        container: RealizedContainer,
    ) {
        let source_revision = host
            .runtime()
            .graph()
            .virtual_source_revision(collection)
            .unwrap();
        host.test_adapter_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container,
                source_revision,
            });
        host.drain(64).unwrap();
    }

    #[test]
    fn edit_survives_reorder_recycle_and_realization() {
        let editor_input = EditorInput::default();
        let mut host = mount(editor_input.clone());
        let collection = virtual_collection(&host);
        realize(&mut host, collection, RealizedContainer(1), 0);

        assert_eq!(editor_input.metrics.creates.get(), 1);
        assert_eq!(editor_input.metrics.setups.get(), 1);
        let input = host
            .runtime()
            .graph()
            .objects()
            .find(|object| {
                host.runtime().graph().kind(*object) == Some(ObjectType::TextBox)
                    && host
                        .runtime()
                        .graph()
                        .events(*object)
                        .is_some_and(|events| {
                            events.iter().any(|event| event.id == EventId::TextChanged)
                        })
            })
            .unwrap();

        assert!(
            editor_input
                .metrics
                .sender
                .borrow()
                .as_ref()
                .unwrap()
                .send(RowMessage::Edit)
        );
        host.drain(64).unwrap();
        assert_eq!(editor_input.metrics.setups.get(), 2);
        assert_eq!(editor_input.metrics.cleanups.get(), 1);
        assert!(
            host.runtime()
                .graph()
                .properties(input)
                .unwrap()
                .iter()
                .any(|property| {
                    property.id == PropertyId::IsEnabled
                        && property.value == PropertyValue::Bool(true)
                })
        );
        assert_eq!(
            host.runtime()
                .adapter()
                .imperatives()
                .iter()
                .filter(|request| {
                    matches!(
                        request,
                        ImperativeRequest::Focus { object, .. } if *object == input
                    )
                })
                .count(),
            1
        );

        let callback = event(&host, input, EventId::TextChanged);
        host.test_adapter_mut().queue_event(EventDispatch::new(
            input,
            EventId::TextChanged,
            callback,
            EventPayload::String("Edited task".into()),
        ));
        let drain = host.drain(64).unwrap();
        assert!(drain.dispatched >= 1);
        assert!(live_text(&host, "Edited task"));

        recycle(&mut host, collection, RealizedContainer(1));
        assert_eq!(editor_input.metrics.cleanups.get(), 2);

        realize(&mut host, collection, RealizedContainer(2), 0);
        assert_eq!(editor_input.metrics.creates.get(), 2);
        assert_eq!(editor_input.metrics.setups.get(), 3);
        assert_eq!(editor_input.metrics.cleanups.get(), 2);
        assert!(live_text(&host, "Edited task"));

        assert!(
            editor_input
                .sender
                .borrow()
                .as_ref()
                .unwrap()
                .send(Message::Reverse)
        );
        host.drain(64).unwrap();
        assert_eq!(editor_input.metrics.cleanups.get(), 3);
        assert_eq!(
            host.runtime().graph().virtual_source_revision(collection),
            Some(1)
        );

        realize(
            &mut host,
            collection,
            RealizedContainer(3),
            INITIAL_TASKS - 1,
        );
        assert_eq!(editor_input.metrics.creates.get(), 4);
        assert_eq!(editor_input.metrics.setups.get(), 5);
        assert_eq!(editor_input.metrics.cleanups.get(), 3);
        assert!(live_text(&host, "Edited task"));
    }

    #[test]
    fn reset_ignores_an_in_flight_load_result() {
        let input = EditorInput::default();
        let mut host = mount(input.clone());
        let collection = virtual_collection(&host);
        let sender = input.sender.borrow().as_ref().unwrap().clone();

        assert!(sender.send(Message::Load));
        host.drain(1).unwrap();
        assert!(sender.send(Message::Stress));
        host.drain(1).unwrap();
        assert_eq!(
            host.runtime().graph().virtual_source_revision(collection),
            Some(1)
        );

        assert!(sender.send(Message::Loaded {
            generation: 0,
            tasks: TaskEditor::tasks(INITIAL_TASKS as u64, LOAD_TASKS),
        }));
        host.drain(1).unwrap();
        assert_eq!(
            host.runtime().graph().virtual_source_revision(collection),
            Some(1)
        );
    }
}
