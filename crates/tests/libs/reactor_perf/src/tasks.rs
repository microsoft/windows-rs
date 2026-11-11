use super::*;

const SCRIPT: [Action; 12] = [
    Action::Select,
    Action::Type,
    Action::Type,
    Action::Status,
    Action::Filter,
    Action::ClearFilter,
    Action::UpdateRow,
    Action::Reverse,
    Action::Reverse,
    Action::Close,
    Action::Open,
    Action::Status,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Action {
    Select,
    Type,
    Status,
    Filter,
    ClearFilter,
    UpdateRow,
    Reverse,
    Close,
    Open,
}

impl Action {
    fn target(self) -> usize {
        match self {
            Self::Type => 2,
            Self::Status => 3,
            _ => 0,
        }
    }
}

struct Pending {
    action: Action,
    started: Instant,
    bytes: u64,
    allocations: u64,
    views: [usize; 5],
    warmup: bool,
    app_ms: f64,
    app_bytes: u64,
    committed: Option<(f64, u64, u64)>,
}

struct Sample {
    action: Action,
    commit_ms: f64,
    app_ms: f64,
    app_bytes: u64,
    bytes: u64,
    allocations: u64,
    views: [usize; 5],
}

#[derive(Default)]
struct Measurements {
    root: Option<LocalSender<Action>>,
    detail: Option<LocalSender<String>>,
    status: Option<LocalSender<()>>,
    views: [usize; 5],
    detail_mounts: usize,
    pending: Option<Pending>,
    samples: Vec<Sample>,
    last_frame: Option<Instant>,
    frame_gaps: Vec<f64>,
}

#[derive(Clone, Default)]
struct Shared(Rc<RefCell<Measurements>>);

struct AppWork {
    shared: Shared,
    started: Instant,
    bytes: u64,
}

impl Drop for AppWork {
    fn drop(&mut self) {
        let elapsed = self.started.elapsed().as_secs_f64() * 1_000.0;
        let bytes = ALLOCATED_BYTES.load(Ordering::Relaxed) - self.bytes;
        if let Some(pending) = &mut self.shared.0.borrow_mut().pending
            && pending.committed.is_none()
        {
            pending.app_ms += elapsed;
            pending.app_bytes += bytes;
        }
    }
}

impl PartialEq for Shared {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Shared {
    fn app_work(&self) -> AppWork {
        AppWork {
            shared: self.clone(),
            started: Instant::now(),
            bytes: ALLOCATED_BYTES.load(Ordering::Relaxed),
        }
    }

    fn begin(&self, action: Action, warmup: bool) {
        let mut state = self.0.borrow_mut();
        assert!(state.pending.is_none());
        state.pending = Some(Pending {
            action,
            started: Instant::now(),
            bytes: ALLOCATED_BYTES.load(Ordering::Relaxed),
            allocations: ALLOCATIONS.load(Ordering::Relaxed),
            views: state.views,
            warmup,
            app_ms: 0.0,
            app_bytes: 0,
            committed: None,
        });
    }

    fn commit(&self, target: usize) {
        let mut state = self.0.borrow_mut();
        if let Some(pending) = &mut state.pending
            && pending.action.target() == target
            && pending.committed.is_none()
        {
            pending.committed = Some((
                pending.started.elapsed().as_secs_f64() * 1_000.0,
                ALLOCATED_BYTES.load(Ordering::Relaxed) - pending.bytes,
                ALLOCATIONS.load(Ordering::Relaxed) - pending.allocations,
            ));
        }
    }

    fn settle(&self) -> bool {
        let mut state = self.0.borrow_mut();
        let Some(pending) = &state.pending else {
            return true;
        };
        if pending.committed.is_none() {
            assert!(
                pending.started.elapsed() < Duration::from_secs(5),
                "script action did not commit: {:?}",
                pending.action
            );
            return false;
        }
        let pending = state.pending.take().unwrap();
        if !pending.warmup {
            let (commit_ms, bytes, allocations) = pending.committed.unwrap();
            let views = std::array::from_fn(|i| state.views[i] - pending.views[i]);
            state.samples.push(Sample {
                action: pending.action,
                commit_ms,
                app_ms: pending.app_ms,
                app_bytes: pending.app_bytes,
                bytes,
                allocations,
                views,
            });
        }
        true
    }

    fn dispatch(&self, action: Action, step: usize) {
        let state = self.0.borrow();
        let sent = match action {
            Action::Type => state.detail.as_ref().unwrap().send(format!("Draft {step}")),
            Action::Status => state.status.as_ref().unwrap().send(()),
            _ => state.root.as_ref().unwrap().send(action),
        };
        assert!(sent, "script message was rejected");
    }

    fn rendered(&self) {
        let now = Instant::now();
        let mut state = self.0.borrow_mut();
        if state.pending.as_ref().is_some_and(|p| !p.warmup)
            && let Some(previous) = state.last_frame
        {
            state
                .frame_gaps
                .push(now.duration_since(previous).as_secs_f64() * 1_000.0);
        }
        state.last_frame = Some(now);
    }

    fn report(&self, options: Options) -> String {
        let state = self.0.borrow();
        assert_eq!(state.samples.len(), options.cycles * SCRIPT.len());
        let mut report = format!(
            "Task viewer: {} items, {} cycles, {} actions (one warmup cycle excluded)\n\
             Commit = dispatch through post-commit effect; excludes later layout/rendering.\n\
             App = component create/update/input_changed/view time and bytes within that interval.\n\
             Bytes/calls = process-wide Rust allocations through that effect, not native allocations.\n\
             Views = workspace/list/detail/status/row calls through the following timer tick.\n\
             action       samples   median_ms      p95_ms  mean_app_ms   mean_bytes   app_bytes  mean_calls  mean_views\n",
            options.count,
            options.cycles,
            state.samples.len(),
        );
        for action in [
            Action::Select,
            Action::Type,
            Action::Status,
            Action::Filter,
            Action::ClearFilter,
            Action::UpdateRow,
            Action::Reverse,
            Action::Close,
            Action::Open,
        ] {
            let samples: Vec<_> = state
                .samples
                .iter()
                .filter(|s| s.action == action)
                .collect();
            let mut times: Vec<_> = samples.iter().map(|s| s.commit_ms).collect();
            let count = samples.len() as f64;
            let views: [f64; 5] = std::array::from_fn(|i| {
                samples.iter().map(|s| s.views[i]).sum::<usize>() as f64 / count
            });
            writeln!(
                report,
                "{action:<12?} {:>7} {:>11.4} {:>11.4} {:>12.4} {:>12.0} {:>11.0} {:>11.1} {views:.1?}",
                samples.len(),
                percentile(&mut times, 50),
                percentile(&mut times, 95),
                samples.iter().map(|s| s.app_ms).sum::<f64>() / count,
                samples.iter().map(|s| s.bytes).sum::<u64>() as f64 / count,
                samples.iter().map(|s| s.app_bytes).sum::<u64>() as f64 / count,
                samples.iter().map(|s| s.allocations).sum::<u64>() as f64 / count,
            )
            .unwrap();
        }
        let mut gaps = state.frame_gaps.clone();
        if !gaps.is_empty() {
            writeln!(
                report,
                "Rendering callback gaps: {} samples, p95 {:.2} ms, max {:.2} ms (not presentation latency)",
                gaps.len(),
                percentile(&mut gaps, 95),
                percentile(&mut gaps, 100),
            )
            .unwrap();
        }
        writeln!(
            report,
            "Detail mounts: {} (expected {}); final working set: {:.1} MB",
            state.detail_mounts,
            options.cycles + 2,
            working_set() as f64 / (1024.0 * 1024.0),
        )
        .unwrap();
        assert_eq!(state.detail_mounts, options.cycles + 2);
        report
    }
}

fn percentile(values: &mut [f64], percent: usize) -> f64 {
    assert!(!values.is_empty());
    values.sort_by(f64::total_cmp);
    values[(values.len() * percent).div_ceil(100) - 1]
}

#[derive(Clone, PartialEq)]
struct Task {
    id: usize,
    title: String,
    done: bool,
}

struct Model {
    tasks: Vec<Rc<Task>>,
    visible: Rc<Vec<Rc<Task>>>,
    selected: usize,
    filter: String,
    key_revision: u64,
    detail_open: bool,
}

impl Model {
    fn new(count: usize) -> Self {
        let tasks: Vec<_> = (0..count)
            .map(|id| {
                Rc::new(Task {
                    id,
                    title: format!("Task {id}"),
                    done: false,
                })
            })
            .collect();
        Self {
            visible: Rc::new(tasks.clone()),
            tasks,
            selected: 0,
            filter: String::new(),
            key_revision: 0,
            detail_open: true,
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Select => self.selected = 1 - self.selected,
            Action::Filter => self.filter = "Task 1".into(),
            Action::ClearFilter => self.filter.clear(),
            Action::UpdateRow => {
                let task = Rc::make_mut(self.tasks.iter_mut().find(|t| t.id == 0).unwrap());
                task.done = !task.done;
            }
            Action::Reverse => self.tasks.reverse(),
            Action::Close => self.detail_open = false,
            Action::Open => self.detail_open = true,
            Action::Type | Action::Status => unreachable!(),
        }
        if matches!(
            action,
            Action::Filter | Action::ClearFilter | Action::UpdateRow | Action::Reverse
        ) {
            let visible: Vec<_> = self
                .tasks
                .iter()
                .filter(|task| task.title.contains(&self.filter))
                .cloned()
                .collect();
            if !self
                .visible
                .iter()
                .map(|task| task.id)
                .eq(visible.iter().map(|task| task.id))
            {
                self.key_revision += 1;
            }
            self.visible = Rc::new(visible);
        }
    }
}

#[derive(Clone, PartialEq)]
struct Input {
    options: Options,
    live: bool,
    shared: Shared,
}

struct Workspace {
    model: Model,
    revision: usize,
    _timer: Option<LiveTickSubscription>,
    _rendering: Option<LiveRenderingSubscription>,
}

impl Component for Workspace {
    type Input = Input;
    type Message = Action;

    fn create(input: &Input, context: &ComponentContext<Self>) -> Self {
        input.shared.0.borrow_mut().root = Some(context.sender());
        let (timer, rendering) = if input.live {
            let shared = input.shared.clone();
            let options = input.options;
            let step = Cell::new(0);
            let finished = Cell::new(false);
            let timer = subscribe_live_interval(Duration::from_millis(33), move || {
                if finished.get() || !shared.settle() {
                    return;
                }
                let current = step.get();
                if current == (options.cycles + 1) * SCRIPT.len() {
                    finished.set(true);
                    let path = std::env::current_exe()
                        .unwrap()
                        .with_file_name("TaskViewer.Reactor.report.txt");
                    std::fs::write(path, shared.report(options)).unwrap();
                    schedule_live_test_exit(true).unwrap();
                } else {
                    let action = SCRIPT[current % SCRIPT.len()];
                    shared.begin(action, current < SCRIPT.len());
                    shared.dispatch(action, current);
                    step.set(current + 1);
                }
            })
            .unwrap();
            let shared = input.shared.clone();
            let rendering = subscribe_live_rendering(move || shared.rendered()).unwrap();
            (Some(timer), Some(rendering))
        } else {
            (None, None)
        };
        Self {
            model: Model::new(input.options.count),
            revision: 0,
            _timer: timer,
            _rendering: rendering,
        }
    }

    fn update(&mut self, action: Action, context: &ComponentContext<Self>) {
        let _work = context.input().shared.app_work();
        self.model.apply(action);
        self.revision += 1;
    }

    fn view(&self, input: &Input, context: &mut ViewContext<Self>) -> View {
        let _work = input.shared.app_work();
        input.shared.0.borrow_mut().views[0] += 1;
        let shared = input.shared.clone();
        context.use_effect("commit", self.revision, move || {
            shared.commit(0);
            None
        });
        context.window_title("Reactor task viewer benchmark");
        let selected = self
            .model
            .tasks
            .iter()
            .find(|t| t.id == self.model.selected)
            .unwrap();
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR, GridLength::Auto])
            .columns([GridLength::STAR, GridLength::STAR])
            .margin(12.0)
            .children((
                StackPanel::new()
                    .grid_column_span(2)
                    .orientation(Orientation::Horizontal)
                    .spacing(12.0)
                    .children((
                        TextBox::new()
                            .header("Filter tasks")
                            .text(self.model.filter.clone())
                            .is_read_only(true)
                            .width(240.0),
                        Button::new()
                            .content("Reverse")
                            .on_click(context.message(Action::Reverse))
                            .tooltip("Reverse task order"),
                        format!("{} tasks", self.model.visible.len()),
                    )),
                Border::new().grid_row(1).content(component::<TaskList>(
                    "list",
                    ListInput {
                        tasks: Rc::clone(&self.model.visible),
                        key_revision: self.model.key_revision,
                        selected: self.model.selected,
                        shared: input.shared.clone(),
                    },
                )),
                Border::new()
                    .grid_row(1)
                    .grid_column(1)
                    .padding(12.0)
                    .content(if self.model.detail_open {
                        View::from(component::<Detail>(
                            "detail",
                            (Rc::clone(selected), input.shared.clone()),
                        ))
                    } else {
                        TextBlock::new().text("Detail closed").into()
                    }),
                Border::new()
                    .grid_row(2)
                    .grid_column_span(2)
                    .content(component::<Status>("status", input.shared.clone())),
            ))
            .into()
    }
}

#[derive(Clone, PartialEq)]
struct ListInput {
    tasks: Rc<Vec<Rc<Task>>>,
    key_revision: u64,
    selected: usize,
    shared: Shared,
}

struct TaskList;

impl Component for TaskList {
    type Input = ListInput;
    type Message = ();

    fn create(_: &ListInput, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &ListInput, _: &mut ViewContext<Self>) -> View {
        let _work = input.shared.app_work();
        input.shared.0.borrow_mut().views[1] += 1;
        let keys = Rc::clone(&input.tasks);
        let input = input.clone();
        ScrollViewer::new()
            .content(ItemsRepeater::new().virtual_source(VirtualSource::new(
                input.key_revision,
                input.tasks.len(),
                move |index| keys[index].id,
                move |index| {
                    View::component::<TaskRow>((
                        Rc::clone(&input.tasks[index]),
                        input.tasks[index].id == input.selected,
                        input.shared.clone(),
                    ))
                },
            )))
            .into()
    }
}

struct TaskRow;

impl Component for TaskRow {
    type Input = (Rc<Task>, bool, Shared);
    type Message = ();

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, (task, selected, shared): &Self::Input, _: &mut ViewContext<Self>) -> View {
        let _work = shared.app_work();
        shared.0.borrow_mut().views[4] += 1;
        StackPanel::new()
            .spacing(2.0)
            .margin(6.0)
            .children((
                TextBlock::new().text(task.title.clone()),
                format!(
                    "#{} - {}{}",
                    task.id,
                    if task.done { "done" } else { "open" },
                    if *selected { " - selected" } else { "" },
                ),
            ))
            .into()
    }
}

struct Detail {
    id: usize,
    draft: String,
    revision: usize,
}

impl Component for Detail {
    type Input = (Rc<Task>, Shared);
    type Message = String;

    fn create((task, shared): &Self::Input, context: &ComponentContext<Self>) -> Self {
        let _work = shared.app_work();
        let mut state = shared.0.borrow_mut();
        state.detail = Some(context.sender());
        state.detail_mounts += 1;
        Self {
            id: task.id,
            draft: task.title.clone(),
            revision: 0,
        }
    }

    fn input_changed(&mut self, (task, shared): &Self::Input, _: &ComponentContext<Self>) {
        let _work = shared.app_work();
        if self.id != task.id {
            self.id = task.id;
            self.draft.clone_from(&task.title);
        }
    }

    fn update(&mut self, draft: String, context: &ComponentContext<Self>) {
        let _work = context.input().1.app_work();
        self.draft = draft;
        self.revision += 1;
    }

    fn view(&self, (task, shared): &Self::Input, context: &mut ViewContext<Self>) -> View {
        let _work = shared.app_work();
        shared.0.borrow_mut().views[2] += 1;
        let record = shared.clone();
        context.use_effect("commit", self.revision, move || {
            record.commit(2);
            None
        });
        StackPanel::new()
            .spacing(12.0)
            .children((
                format!("Details for #{}", task.id),
                TextBox::new()
                    .header("Draft title")
                    .text(self.draft.clone())
                    .on_text_changed(context.callback(|text: Rc<str>| text.to_string())),
                TextBlock::new().text("Draft edits stay local until the selection changes."),
                Button::new().content("Save").is_enabled(false),
            ))
            .into()
    }
}

struct Status(usize);

impl Component for Status {
    type Input = Shared;
    type Message = ();

    fn create(shared: &Shared, context: &ComponentContext<Self>) -> Self {
        shared.0.borrow_mut().status = Some(context.sender());
        Self(0)
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        let _work = context.input().app_work();
        self.0 += 1;
    }

    fn view(&self, shared: &Shared, context: &mut ViewContext<Self>) -> View {
        let _work = shared.app_work();
        shared.0.borrow_mut().views[3] += 1;
        let shared = shared.clone();
        context.use_effect("commit", self.0, move || {
            shared.commit(3);
            None
        });
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(12.0)
            .children((
                format!("Background sync: {}", self.0),
                ProgressBar::new()
                    .width(200.0)
                    .maximum(100.0)
                    .value((self.0 % 100) as f64),
            ))
            .into()
    }
}

pub(super) fn run(options: Options) {
    assert!(
        !options.json,
        "--json is only supported by the grid workload"
    );
    App::run_component_with_policy::<Workspace>(
        Input {
            options,
            live: true,
            shared: Shared::default(),
        },
        WindowPolicy::new()
            .title("Reactor task viewer benchmark")
            .client_size(1100.0, 750.0),
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> Options {
        Options {
            tasks: true,
            count: 100,
            cycles: 2,
            duration: Duration::from_secs(1),
            headless: false,
            json: false,
            percent: 10.0,
        }
    }

    #[test]
    fn model_revisions_follow_keys_not_row_values() {
        let mut model = Model::new(100);
        let original = Rc::clone(&model.visible);
        model.apply(Action::UpdateRow);
        assert_eq!(model.key_revision, 0);
        assert!(!original[0].done);
        assert!(model.visible[0].done);
        assert!(Rc::ptr_eq(&original[1], &model.visible[1]));
        model.apply(Action::Filter);
        assert_eq!(model.key_revision, 1);
        assert_eq!(model.visible.len(), 11);
        model.apply(Action::ClearFilter);
        model.apply(Action::Reverse);
        assert_eq!(model.visible[0].id, 99);
        model.apply(Action::Reverse);
        assert_eq!(model.key_revision, 4);
        assert_eq!(model.visible[0].id, 0);
    }

    #[test]
    fn replay_is_local_and_remounts_only_the_closed_detail() {
        let shared = Shared::default();
        let options = options();
        let mut host = ComponentHost::mount(
            RecordingAdapter::new(),
            [component::<Workspace>(
                "workspace",
                Input {
                    options,
                    live: false,
                    shared: shared.clone(),
                },
            )],
        );
        let detail_path = [Key::from("workspace"), Key::from("detail")];
        let list = host
            .reference_at(&[Key::from("workspace"), Key::from("list")])
            .unwrap();
        let list_object = list.get().unwrap();
        let mut draft = None;
        for step in 0..(options.cycles + 1) * SCRIPT.len() {
            let action = SCRIPT[step % SCRIPT.len()];
            let detail = host.reference_at(&detail_path);
            let detail_object = detail.as_ref().and_then(ElementRef::get);
            shared.begin(action, step < SCRIPT.len());
            shared.dispatch(action, step);
            host.drain(100);
            assert!(shared.settle());
            assert!(shared.0.borrow().pending.is_none());
            assert_eq!(list.get(), Some(list_object));
            match action {
                Action::Close => {
                    assert!(host.reference_at(&detail_path).is_none());
                    assert!(detail.unwrap().get().is_none());
                    draft = None;
                }
                Action::Open => {
                    assert!(host.reference_at(&detail_path).unwrap().get().is_some());
                }
                _ => {
                    assert_eq!(
                        host.reference_at(&detail_path).unwrap().get(),
                        detail_object,
                    );
                    if action == Action::Type {
                        draft = Some(format!("Draft {step}"));
                    } else if action == Action::Select {
                        draft = None;
                    }
                }
            }
            if let Some(expected) = &draft {
                assert!(host.runtime().graph().objects().any(|object| {
                    host.runtime().graph().properties(object).unwrap().iter().any(|property| {
                        matches!(&property.value, PropertyValue::String(text) if text.as_ref() == expected)
                    })
                }));
            }
        }
        let state = shared.0.borrow();
        assert_eq!(state.detail_mounts, 4);
        for sample in &state.samples {
            match sample.action {
                Action::Type => assert_eq!(sample.views, [0, 0, 1, 0, 0]),
                Action::Status => assert_eq!(sample.views, [0, 0, 0, 1, 0]),
                _ => assert_eq!(sample.views[0], 1),
            }
        }
        drop(state);
        assert!(shared.report(options).contains("24 actions"));
    }
}
