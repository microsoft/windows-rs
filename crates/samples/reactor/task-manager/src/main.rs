#![windows_subsystem = "windows"]

mod bindings {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        non_upper_case_globals,
        clippy::all
    )]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod details;
mod inspect;
mod model;
mod performance;
mod process;
mod source;

use details::{DetailInput, Details};
use model::{Column, Model, Process, ProcessId, Row, Snapshot};
use source::Source;
use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use windows_reactor::*;

#[derive(Clone, Default)]
struct Outcome(Arc<Mutex<Option<String>>>);

impl PartialEq for Outcome {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, PartialEq)]
struct Options {
    outcome: Outcome,
    demo: bool,
    replay: Option<PathBuf>,
    record: Option<PathBuf>,
    report: Option<PathBuf>,
    interval: Duration,
    frames: Option<usize>,
    exit: bool,
}

impl Options {
    fn parse(mut args: impl Iterator<Item = String>) -> Self {
        let mut options = Self {
            outcome: Outcome::default(),
            demo: false,
            replay: None,
            record: None,
            report: None,
            interval: Duration::from_secs(1),
            frames: None,
            exit: false,
        };
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--demo" => options.demo = true,
                "--replay" => options.replay = Some(args.next().unwrap().into()),
                "--record" => options.record = Some(args.next().unwrap().into()),
                "--report" => options.report = Some(args.next().unwrap().into()),
                "--interval-ms" => {
                    options.interval = Duration::from_millis(args.next().unwrap().parse().unwrap());
                }
                "--frames" => options.frames = Some(args.next().unwrap().parse().unwrap()),
                "--exit" => options.exit = true,
                _ => panic!("unknown option: {arg}"),
            }
        }
        assert!(
            !(options.demo && options.replay.is_some()),
            "choose demo or replay"
        );
        assert!(
            !(options.record.is_some() && options.replay.is_some()),
            "cannot record a replay"
        );
        assert!(!options.interval.is_zero(), "interval must be positive");
        assert!(options.frames != Some(0), "frames must be positive");
        let finite = !options.live() || options.frames.is_some();
        assert!(
            !options.exit || finite,
            "--exit requires a finite source or --frames"
        );
        assert!(
            options.report.is_none() || finite,
            "--report requires a finite source or --frames"
        );
        options
    }

    fn live(&self) -> bool {
        !self.demo && self.replay.is_none()
    }
}

#[derive(Default)]
struct Measurements {
    pending: Option<Instant>,
    commit_ms: Vec<f64>,
    acquisition_ms: Vec<f64>,
    model_ms: Vec<f64>,
}

impl Measurements {
    fn complete(&mut self) {
        if let Some(started) = self.pending.take() {
            self.commit_ms
                .push(started.elapsed().as_secs_f64() * 1000.0);
        }
    }

    fn report(&self) -> String {
        let mut times = self.commit_ms.clone();
        times.sort_by(f64::total_cmp);
        if times.is_empty() {
            return "Task Manager: no snapshots committed\n".into();
        }
        let mut model = self.model_ms.clone();
        model.sort_by(f64::total_cmp);
        let mut remaining: Vec<_> = self
            .commit_ms
            .iter()
            .zip(&self.model_ms)
            .map(|(commit, model)| commit - model)
            .collect();
        remaining.sort_by(f64::total_cmp);
        format!(
            "Task Manager: {} snapshots committed\n\
             Update-to-effect median: {:.4} ms\n\
             Update-to-effect p95: {:.4} ms\n\
             Model/projection median: {:.4} ms\n\
             Other update-to-effect median: {:.4} ms\n\
             Mean acquisition/recording: {:.4} ms\n\
             Includes model projection and native commit; excludes later layout and presentation.\n\
             Acquisition includes sampling or replay parsing, plus recording when enabled.\n\
             Startup is included. This is not a frame-rate or input-latency measurement.\n",
            times.len(),
            times[(times.len() * 50).div_ceil(100) - 1],
            times[(times.len() * 95).div_ceil(100) - 1],
            model[(model.len() * 50).div_ceil(100) - 1],
            remaining[(remaining.len() * 50).div_ceil(100) - 1],
            self.acquisition_ms.iter().sum::<f64>() / self.acquisition_ms.len() as f64,
        )
    }
}

struct Batch {
    source: Source,
    snapshot: Option<Snapshot>,
    elapsed: Duration,
}

enum Message {
    Tick,
    Sampled(Result<Box<Batch>, String>),
    Filter(String),
    Sort(Column),
    ShowColumn(Column, bool),
    ResizeStart(Column, PointerEventInfo),
    ResizeMove(PointerEventInfo),
    ResizeEnd,
    Tree(bool),
    Select(ProcessId),
    Collapse(ProcessId),
    Pause,
    Navigate(Option<String>),
    Open(ProcessId),
    End(ProcessId, bool),
    Confirmed(ContentDialogResult),
    DismissActionError,
    Metric(performance::Metric),
    CpuView(performance::CpuView),
    ColorScheme(ColorScheme),
    Finish,
}

struct Pending {
    name: String,
    tree: bool,
    targets: Vec<ProcessId>,
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Processes,
    Details,
    Performance,
}

struct TaskManager {
    model: Model,
    source: Option<Source>,
    task: Option<ComponentTask>,
    timer: Option<ComponentTimer>,
    paused: bool,
    finished: bool,
    finalized: bool,
    error: Option<String>,
    frames: usize,
    measurements: Rc<RefCell<Measurements>>,
    on_select: Callback<ProcessId>,
    on_collapse: Callback<ProcessId>,
    on_open: Callback<ProcessId>,
    on_end: Callback<(ProcessId, bool)>,
    pending: Option<Pending>,
    action_error: Option<String>,
    page: Page,
    resizing: Option<(Column, f64, f64)>,
    metric: performance::Metric,
    cpu_view: performance::CpuView,
    color_scheme: ColorScheme,
}

impl TaskManager {
    fn acquire(&mut self, context: &ComponentContext<Self>) {
        if self.paused || self.finished || self.task.is_some() {
            return;
        }
        let source = self.source.take();
        let options = context.input().clone();
        self.task = Some(context.spawn_background(move |cancel| {
            Message::Sampled((|| {
                let mut source = match source {
                    Some(source) => source,
                    None => Source::open(&options)?,
                };
                let started = Instant::now();
                let snapshot = source.next(&cancel)?;
                Ok(Box::new(Batch {
                    source,
                    snapshot,
                    elapsed: started.elapsed(),
                }))
            })())
        }));
    }
}

impl Component for TaskManager {
    type Input = Options;
    type Message = Message;

    fn create(_: &Options, context: &ComponentContext<Self>) -> Self {
        let mut manager = Self {
            model: Model::default(),
            source: None,
            task: None,
            timer: None,
            paused: false,
            finished: false,
            finalized: false,
            error: None,
            frames: 0,
            measurements: Rc::default(),
            on_select: context.callback(Message::Select),
            on_collapse: context.callback(Message::Collapse),
            on_open: context.callback(Message::Open),
            on_end: context.callback(|(id, tree)| Message::End(id, tree)),
            pending: None,
            action_error: None,
            page: Page::Processes,
            resizing: None,
            metric: performance::Metric::default(),
            cpu_view: performance::CpuView::default(),
            color_scheme: ColorScheme::default(),
        };
        manager.acquire(context);
        manager
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Tick => {
                self.timer = None;
                self.acquire(context);
            }
            Message::Sampled(result) => {
                self.task = None;
                match result {
                    Ok(batch) => {
                        self.source = Some(batch.source);
                        if let Some(snapshot) = batch.snapshot {
                            let started = Instant::now();
                            match self.model.apply(snapshot) {
                                Ok(()) => {
                                    let model_ms = started.elapsed().as_secs_f64() * 1000.0;
                                    self.frames += 1;
                                    if context.input().report.is_some() {
                                        let mut measurements = self.measurements.borrow_mut();
                                        measurements.pending = Some(started);
                                        measurements.model_ms.push(model_ms);
                                        measurements
                                            .acquisition_ms
                                            .push(batch.elapsed.as_secs_f64() * 1000.0);
                                    }
                                    self.finished = context
                                        .input()
                                        .frames
                                        .is_some_and(|limit| self.frames >= limit);
                                }
                                Err(error) => {
                                    self.error = Some(error);
                                    self.finished = true;
                                }
                            }
                        } else {
                            self.finished = true;
                        }
                    }
                    Err(error) => {
                        self.error = Some(error);
                        self.finished = true;
                    }
                }
                if !self.finished && !self.paused {
                    self.timer = Some(context.set_timeout(context.input().interval, Message::Tick));
                }
            }
            Message::Filter(filter) => {
                self.model.filter = filter;
                self.model.rebuild();
            }
            Message::Sort(sort) => {
                self.model.sort_by(sort);
            }
            Message::ShowColumn(column, visible) => {
                self.model.show_column(column, visible);
            }
            Message::ResizeStart(column, info) => {
                if info.is_left_button_pressed && info.capture_succeeded == Some(true) {
                    self.resizing = Some((column, info.window_x, self.model.width(column)));
                }
            }
            Message::ResizeMove(info) => {
                if !info.is_left_button_pressed {
                    self.resizing = None;
                } else if let Some((column, start_x, start_width)) = self.resizing {
                    self.model
                        .resize_column(column, start_width + info.window_x - start_x);
                }
            }
            Message::ResizeEnd => self.resizing = None,
            Message::Tree(tree) => {
                self.model.tree = tree;
                self.model.rebuild();
            }
            Message::Select(id) => {
                if self.model.processes.contains_key(&id) {
                    self.model.selected = Some(id);
                }
            }
            Message::Collapse(id) => {
                if !self.model.collapsed.remove(&id) {
                    self.model.collapsed.insert(id);
                }
                self.model.rebuild();
            }
            Message::Pause if !self.finished => {
                self.paused = !self.paused;
                self.timer = None;
                if !self.paused {
                    self.acquire(context);
                }
            }
            Message::Pause => {}
            Message::Navigate(tag) => match tag.as_deref() {
                Some("processes") => self.page = Page::Processes,
                Some("details") => self.page = Page::Details,
                Some("performance") => self.page = Page::Performance,
                None => {}
                Some(_) => unreachable!(),
            },
            Message::Open(id) => {
                if self.model.processes.contains_key(&id) {
                    self.model.selected = Some(id);
                    self.page = Page::Details;
                }
            }
            Message::End(id, tree) => {
                if let Some(process) = self.model.processes.get(&id)
                    && context.input().live()
                {
                    self.pending = Some(Pending {
                        name: process.name.clone(),
                        tree,
                        targets: if tree {
                            self.model.descendants(id)
                        } else {
                            vec![id]
                        },
                    });
                }
            }
            Message::Confirmed(result) => {
                if let Some(pending) = self.pending.take()
                    && result == ContentDialogResult::Primary
                {
                    if pending
                        .targets
                        .iter()
                        .any(|id| id.pid == std::process::id())
                    {
                        self.action_error = Some("Task Manager cannot end itself.".into());
                        return;
                    }
                    // Best effort, like Process.Kill(entireProcessTree): each process is ended
                    // before its descendants, and a failure does not stop the rest of the tree.
                    let failures: Vec<_> = pending
                        .targets
                        .into_iter()
                        .filter_map(|id| {
                            let error = inspect::terminate(id).err()?;
                            let name = self.model.processes.get(&id).map_or("", |p| &p.name);
                            Some(format!("{name} (PID {}): {error}", id.pid))
                        })
                        .collect();
                    self.action_error = (!failures.is_empty()).then(|| failures.join("\n"));
                }
            }
            Message::DismissActionError => self.action_error = None,
            Message::Metric(metric) => self.metric = metric,
            Message::CpuView(view) => self.cpu_view = view,
            Message::ColorScheme(scheme) => self.color_scheme = scheme,
            Message::Finish if !self.finalized => {
                self.finalized = true;
                if let Some(path) = &context.input().report {
                    let mut report = self.measurements.borrow().report();
                    if let Some(error) = &self.error {
                        report.push_str(&format!("FAILED: {error}\n"));
                    }
                    if let Err(error) = std::fs::File::create_new(path)
                        .and_then(|mut file| file.write_all(report.as_bytes()))
                    {
                        self.error = Some(format!("Report write failed: {error}"));
                    }
                }
                context
                    .input()
                    .outcome
                    .0
                    .lock()
                    .unwrap()
                    .clone_from(&self.error);
                if context.input().exit {
                    assert!(context.close_window());
                }
            }
            Message::Finish => {}
        }
    }

    fn view(&self, options: &Options, context: &mut ViewContext<Self>) -> View {
        context.window_title("Task Manager");
        context.on_color_scheme(context.callback(Message::ColorScheme));
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(SystemBackdrop::Mica)
                .constraints(WindowConstraints {
                    min_width: Some(900.0),
                    min_height: Some(400.0),
                    max_width: None,
                    max_height: None,
                }),
        );
        let measurements = Rc::clone(&self.measurements);
        let finished = self.finished;
        let sender = context.sender();
        context.use_effect("snapshot", (self.frames, finished), move || {
            measurements.borrow_mut().complete();
            if finished {
                _ = sender.send(Message::Finish);
            }
            None
        });
        let can_end = options.live();
        let content = match self.page {
            Page::Processes => self.processes_page(can_end, context),
            Page::Details => component::<Details>(
                "details",
                DetailInput {
                    process: self
                        .model
                        .selected
                        .and_then(|id| self.model.processes.get(&id).cloned()),
                    live: options.live(),
                    snapshot: self.frames,
                },
            )
            .into(),
            Page::Performance => performance::page(
                &self.model.history,
                self.metric,
                self.cpu_view,
                self.color_scheme,
                context.callback(Message::Metric),
            ),
        };
        let mut actions = Vec::new();
        if self.page == Page::Processes {
            actions.push(keyed(
                "tree",
                CheckBox::new()
                    .content("Process tree")
                    .is_checked(self.model.tree)
                    .on_is_checked_changed(
                        context.callback(|value: Option<bool>| Message::Tree(value == Some(true))),
                    ),
            ));
            actions.push(keyed("columns", self.columns_button(context)));
            actions.push(keyed(
                "end",
                Button::new()
                    .automation_id("end-task")
                    .vertical_alignment(VerticalAlignment::Center)
                    .content("End task")
                    .is_enabled(
                        can_end
                            && self
                                .model
                                .selected
                                .is_some_and(|id| self.model.processes.contains_key(&id)),
                    )
                    .on_click({
                        let on_end = self.on_end.clone();
                        let selected = self.model.selected;
                        move || {
                            if let Some(id) = selected {
                                on_end.call((id, false));
                            }
                        }
                    }),
            ));
        }
        if self.page == Page::Performance && self.metric == performance::Metric::Cpu {
            actions.push(keyed(
                "graph",
                performance::graph_button(self.cpu_view, context.callback(Message::CpuView)),
            ));
        }
        let mut page = vec![
            keyed(
                "heading",
                Grid::new()
                    .margin(Thickness::new(0.0, 0.0, 0.0, 20.0))
                    .columns([GridLength::STAR, GridLength::Auto])
                    .children((
                        TextBlock::new()
                            .automation_id("page-title")
                            .text(match self.page {
                                Page::Processes => "Processes",
                                Page::Details => "Details",
                                Page::Performance => "Performance",
                            })
                            .font_size(24.0)
                            .font_weight(FontWeight::SEMI_BOLD)
                            .vertical_alignment(VerticalAlignment::Center),
                        StackPanel::new()
                            .grid_column(1)
                            .orientation(Orientation::Horizontal)
                            .spacing(12.0)
                            .keyed_children(actions),
                    )),
            ),
            keyed(
                "body",
                Grid::new()
                    .automation_id("page-content")
                    .automation_name("Page content")
                    .grid_row(2)
                    .keyed_children([keyed(
                        match self.page {
                            Page::Processes => "processes",
                            Page::Details => "details",
                            Page::Performance => "performance",
                        },
                        content,
                    )]),
            ),
        ];
        let mut notices = Vec::new();
        if let Some(error) = &self.error {
            notices.push(keyed(
                "error",
                InfoBar::new()
                    .automation_id("sampling-error")
                    .is_open(true)
                    .is_closable(false)
                    .severity(InfoBarSeverity::Error)
                    .title("Updates stopped - displayed data may be stale")
                    .message(error),
            ));
        }
        if let Some(error) = &self.action_error {
            notices.push(keyed(
                "action-error",
                InfoBar::new()
                    .automation_id("action-error")
                    .is_open(true)
                    .severity(InfoBarSeverity::Error)
                    .title("Some processes could not be ended")
                    .message(error)
                    .on_closed(context.callback(|()| Message::DismissActionError)),
            ));
        }
        if !notices.is_empty() {
            page.push(keyed(
                "notices",
                StackPanel::new()
                    .grid_row(1)
                    .spacing(8.0)
                    .margin(Thickness::new(0.0, 0.0, 0.0, 16.0))
                    .keyed_children(notices),
            ));
        }
        let dialog = match &self.pending {
            Some(pending) if pending.tree => ContentDialog::new()
                .title(format!("End {} and its descendants?", pending.name))
                .content(format!(
                    "{} processes will be ended. Unsaved data will be lost.",
                    pending.targets.len()
                ))
                .primary_button_text("End process tree"),
            Some(pending) => ContentDialog::new()
                .title(format!("End {}?", pending.name))
                .content("Unsaved data will be lost.")
                .primary_button_text("End process"),
            None => ContentDialog::new(),
        };
        let pause = context.callback(|()| Message::Pause);
        let root = Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                TitleBar::new()
                    .title("Task Manager")
                    .subtitle(format!(
                        "{}{}",
                        if options.demo {
                            "Demo"
                        } else if options.live() {
                            "Live"
                        } else {
                            "Replay"
                        },
                        if self.error.is_some() {
                            " - stopped"
                        } else if self.finished {
                            " - complete"
                        } else if self.paused {
                            " - paused (Ctrl+Space to resume)"
                        } else {
                            ""
                        }
                    ))
                    .preferred_height(WindowTitleBarHeight::Tall)
                    .content(
                        TextBox::new()
                            .automation_id("filter")
                            .automation_name("Filter processes")
                            .automation_help_text("Filters the Processes page by name or PID.")
                            .is_enabled(self.page == Page::Processes)
                            .width(420.0)
                            .placeholder_text("Search by name or PID")
                            .text(self.model.filter.clone())
                            .on_text_changed(
                                context.callback(|text: Rc<str>| Message::Filter(text.to_string())),
                            ),
                    ),
                NavigationView::new()
                    .grid_row(1)
                    .pane_display_mode(NavigationViewPaneDisplayMode::Left)
                    .open_pane_length(220.0)
                    .is_back_button_visible(NavigationViewBackButtonVisible::Collapsed)
                    .is_settings_visible(false)
                    .always_show_header(false)
                    .menu_items((
                        NavigationViewItem::new()
                            .tag("processes")
                            .automation_id("nav-processes")
                            .icon(Symbol::List)
                            .content("Processes")
                            .is_selected(self.page == Page::Processes),
                        NavigationViewItem::new()
                            .tag("details")
                            .automation_id("nav-details")
                            .icon(Symbol::ContactInfo)
                            .content("Details")
                            .is_selected(self.page == Page::Details),
                        NavigationViewItem::new()
                            .tag("performance")
                            .automation_id("nav-performance")
                            .icon(Icon::font("\u{e9d9}"))
                            .content("Performance")
                            .is_selected(self.page == Page::Performance),
                    ))
                    .on_selected_tag_changed(context.callback(|tag: Option<Rc<str>>| {
                        Message::Navigate(tag.map(|tag| tag.to_string()))
                    }))
                    .content(
                        Border::new()
                            .automation_id("page")
                            .automation_name(match self.page {
                                Page::Processes => "Processes page",
                                Page::Details => "Details page",
                                Page::Performance => "Performance page",
                            })
                            .automation_help_text(self.status(options))
                            .background(ThemeBrush::SolidBackground)
                            .corner_radius(8.0)
                            .padding(24.0)
                            .content(
                                Grid::new()
                                    .rows([GridLength::Auto, GridLength::Auto, GridLength::STAR])
                                    .keyed_children(page),
                            ),
                    ),
            ))
            .content_dialog(
                dialog
                    .close_button_text("Cancel")
                    .default_button(ContentDialogButton::Close)
                    .is_open(self.pending.is_some())
                    .on_closed(context.callback(Message::Confirmed)),
            );
        // Ctrl+Space pauses wherever focus is. A preview handler runs before the focused control,
        // which would otherwise treat it as Space and activate a row or button.
        Border::new()
            .on_preview_key_down(RoutedCallback::new(move |info: KeyEventInfo| {
                let toggle =
                    info.key == VirtualKey::SPACE && info.modifiers == InputModifiers::CONTROL;
                if toggle && !info.status.was_down {
                    pause.call(());
                }
                toggle
            }))
            .content(root)
            .into()
    }
}

impl TaskManager {
    fn columns_button(&self, context: &ViewContext<Self>) -> View {
        let choices: Vec<View> = Column::ALL
            .into_iter()
            .map(|column| {
                CheckBox::new()
                    .automation_id(format!("column-{}", column.id()))
                    .content(column.label())
                    .is_checked(self.model.columns.contains(&column))
                    .is_enabled(column != Column::Name)
                    .on_is_checked_changed(context.callback(move |value: Option<bool>| {
                        Message::ShowColumn(column, value == Some(true))
                    }))
                    .into()
            })
            .collect();
        DropDownButton::new()
            .automation_id("columns")
            .vertical_alignment(VerticalAlignment::Center)
            .content("Columns")
            .flyout_with(
                Flyout::rich(StackPanel::new().children(choices))
                    .placement(FlyoutPlacement::BottomEdgeAlignedRight),
            )
    }

    fn status(&self, options: &Options) -> String {
        match &self.error {
            Some(error) => format!("Stopped - displayed data is stale: {error}"),
            None => format!(
                "{} | {} processes, {} shown | snapshot {}{}",
                if options.demo {
                    "Demo / no process queries"
                } else if options.live() {
                    "Live"
                } else {
                    "Replay / no process queries"
                },
                self.model.processes.len(),
                self.model.visible.len(),
                self.frames,
                if self.finished {
                    " | complete"
                } else if self.paused {
                    " | paused"
                } else {
                    ""
                },
            ),
        }
    }

    fn processes_page(&self, can_end: bool, context: &ViewContext<Self>) -> View {
        let rows = Rc::clone(&self.model.visible);
        let keys = Rc::clone(&rows);
        let selected = self.model.selected;
        let on_select = self.on_select.clone();
        let on_collapse = self.on_collapse.clone();
        let on_open = self.on_open.clone();
        let on_end = self.on_end.clone();
        let columns = Rc::clone(&self.model.columns);
        let column_widths = Rc::clone(&self.model.widths);
        let source = VirtualSource::new(
            self.model.key_revision,
            rows.len(),
            move |index| keys[index].process.key.clone(),
            move |index| {
                View::component::<ProcessRow>(RowInput {
                    row: rows[index].clone(),
                    selected: selected == Some(rows[index].process.id),
                    on_select: on_select.clone(),
                    on_collapse: on_collapse.clone(),
                    on_open: on_open.clone(),
                    on_end: on_end.clone(),
                    can_end,
                    columns: Rc::clone(&columns),
                    widths: Rc::clone(&column_widths),
                })
            },
        );
        let resize_grips = self
            .model
            .columns
            .iter()
            .enumerate()
            .map(|(index, &column)| {
                // A transparent strip over the right edge of each header cell. Pointer capture keeps
                // moves flowing while the grip itself moves with the column edge.
                Border::new()
                    .grid_column(index as i32)
                    .automation_id(format!("resize-{}", column.id()))
                    .width(8.0)
                    .horizontal_alignment(HorizontalAlignment::Right)
                    .background(Color::transparent())
                    .capture_pointer_on_press(true)
                    .pointer_cursor(InputSystemCursorShape::SizeWestEast)
                    .on_pointer_pressed(
                        context.callback(move |info| Message::ResizeStart(column, info)),
                    )
                    .on_pointer_moved(context.callback(Message::ResizeMove))
                    .on_pointer_released(context.callback(|_| Message::ResizeEnd))
                    .on_pointer_capture_lost(context.callback(|_| Message::ResizeEnd))
                    .on_pointer_canceled(context.callback(|_| Message::ResizeEnd))
                    .content(
                        Border::new()
                            .width(1.0)
                            .margin(Thickness::xy(0.0, 10.0))
                            .horizontal_alignment(HorizontalAlignment::Right)
                            .background(ThemeBrush::CardStroke),
                    )
                    .into()
            });
        let header = Grid::new()
            .columns(widths(&self.model.columns, &self.model.widths))
            .children(
                self.model
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(index, &sort)| {
                        let label = sort.label();
                        let active = self.model.sort == sort;
                        let mut content: Vec<View> = vec![TextBlock::new().text(label).into()];
                        if active {
                            content.push(
                                FontIcon::new()
                                    .glyph(if self.model.descending {
                                        "\u{e70d}"
                                    } else {
                                        "\u{e70e}"
                                    })
                                    .font_size(10.0)
                                    .into(),
                            );
                        }
                        Button::new()
                            .grid_column(index as i32)
                            .automation_id(format!("sort-{}", sort.id()))
                            .automation_name(if active {
                                format!(
                                    "{label}, sorted {}",
                                    if self.model.descending {
                                        "descending"
                                    } else {
                                        "ascending"
                                    }
                                )
                            } else {
                                format!("Sort by {label}")
                            })
                            .height(44.0)
                            .style(ButtonStyle::Subtle)
                            .horizontal_alignment(HorizontalAlignment::Stretch)
                            .horizontal_content_alignment(if sort == Column::Name {
                                HorizontalAlignment::Left
                            } else {
                                HorizontalAlignment::Right
                            })
                            .content(
                                StackPanel::new()
                                    .orientation(Orientation::Horizontal)
                                    .spacing(6.0)
                                    .children(content),
                            )
                            .on_click(context.callback(move |()| Message::Sort(sort)))
                            .into()
                    })
                    .chain(resize_grips)
                    .collect::<Vec<View>>(),
            );
        let mut children = vec![
            keyed(
                "header",
                Border::new()
                    .background(ThemeBrush::CardBackground)
                    .border_brush(ThemeBrush::CardStroke)
                    .border_thickness(Thickness::new(0.0, 1.0, 0.0, 1.0))
                    .padding(Thickness::xy(12.0, 0.0))
                    .content(header),
            ),
            keyed(
                "rows",
                ScrollViewer::new()
                    .automation_id("processes")
                    .grid_row(1)
                    .content(ItemsRepeater::new().virtual_source(source)),
            ),
        ];
        if self.model.visible.is_empty() {
            children.push(keyed(
                "empty",
                TextBlock::new()
                    .grid_row(1)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center)
                    .text(if self.frames == 0 {
                        "Waiting for a snapshot..."
                    } else {
                        "No matching processes"
                    }),
            ));
        }
        // The table keeps its column widths and scrolls horizontally when the window is narrower,
        // rather than letting the grid clip columns. Disabling vertical scrolling here keeps the
        // height constrained so the inner rows scroller still virtualizes.
        let table_width = self
            .model
            .columns
            .iter()
            .map(|&column| self.model.width(column))
            .sum::<f64>()
            + 24.0;
        ScrollViewer::new()
            .automation_id("table")
            .horizontal_scroll_bar_visibility(ScrollBarVisibility::Auto)
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Disabled)
            .content(
                Grid::new()
                    .min_width(table_width)
                    .rows([GridLength::Auto, GridLength::STAR])
                    .keyed_children(children),
            )
            .into()
    }
}

fn widths(columns: &[Column], widths: &[f64; Column::ALL.len()]) -> Vec<GridLength> {
    columns
        .iter()
        .map(|&column| GridLength::Pixel(widths[column as usize]))
        .collect()
}

fn cell(column: Column, process: &Process) -> String {
    let Some(value) = column.value(process) else {
        return "--".into();
    };
    match column {
        Column::Cpu => format!("{:.1}%", value as f64 / 10.0),
        Column::WorkingSet | Column::PeakWorkingSet | Column::Private | Column::Virtual => {
            format!("{:.1}", value as f64 / 1_048_576.0)
        }
        Column::PagedPool | Column::NonPagedPool => format!("{:.0}", value as f64 / 1024.0),
        _ => value.to_string(),
    }
}

#[derive(Clone, PartialEq)]
struct RowInput {
    row: Row,
    selected: bool,
    on_select: Callback<ProcessId>,
    on_collapse: Callback<ProcessId>,
    on_open: Callback<ProcessId>,
    on_end: Callback<(ProcessId, bool)>,
    can_end: bool,
    columns: Rc<[Column]>,
    widths: Rc<[f64; Column::ALL.len()]>,
}

struct ProcessRow;

impl Component for ProcessRow {
    type Input = RowInput;
    type Message = ();

    fn create(_: &RowInput, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &RowInput, _: &mut ViewContext<Self>) -> View {
        let row = &input.row;
        let process = &row.process;
        let select = input.on_select.clone();
        let collapse = input.on_collapse.clone();
        let id = process.id;
        // Keep the expander (12 DIP offset, 36 DIP wide) inside the Name cell when it is narrow.
        let indent = ((row.depth.min(12) * 12) as f64)
            .min((input.widths[Column::Name as usize] - 48.0).max(0.0));
        let mut cells: Vec<View> = vec![
            TextBlock::new()
                .text(&process.name)
                .foreground(ThemeBrush::PrimaryText)
                .text_trimming(TextTrimming::CharacterEllipsis)
                .vertical_alignment(VerticalAlignment::Center)
                .margin(Thickness::new(indent + 36.0, 0.0, 12.0, 0.0))
                .into(),
        ];
        for (index, &column) in input.columns.iter().enumerate().skip(1) {
            cells.push(
                TextBlock::new()
                    .grid_column(index as i32)
                    .automation_id(format!("cell-{}-{}", id.pid, column.id()))
                    .vertical_alignment(VerticalAlignment::Center)
                    .text_alignment(TextAlignment::Right)
                    .margin(Thickness::xy(12.0, 0.0))
                    .text(cell(column, process))
                    .foreground(ThemeBrush::PrimaryText)
                    .into(),
            );
        }
        if input.selected {
            cells.push(
                Border::new()
                    .width(3.0)
                    .height(18.0)
                    .corner_radius(1.5)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Center)
                    .background(ThemeBrush::Accent)
                    .into(),
            );
        }
        let open = input.on_open.clone();
        let end = input.on_end.clone();
        let action = |key, label| {
            if input.can_end {
                MenuItem::item(key, label)
            } else {
                MenuItem::disabled(key, label)
            }
        };
        let menu = Menu::new(
            [
                MenuItem::item("details", "Details"),
                MenuItem::separator("separator"),
                action("end", "End task"),
                action("tree", "End process tree"),
            ],
            {
                let open = open.clone();
                move |key: Key| match key.as_str() {
                    Some("details") => open.call(id),
                    Some("end") => end.call((id, false)),
                    Some("tree") => end.call((id, true)),
                    _ => unreachable!(),
                }
            },
        );
        let right_select = input.on_select.clone();
        let mut children: Vec<View> = vec![
            // The selection background is drawn here rather than on the button, whose hover and
            // pressed states are made transparent, so moving or clicking does not change colors.
            Border::new()
                .corner_radius(4.0)
                .background(if input.selected {
                    Brush::from(ThemeBrush::CardBackground)
                } else {
                    Color::transparent().into()
                })
                .on_double_tapped(move || open.call(id))
                // Like Task Manager and Process Explorer, right-clicking a row selects it.
                .on_pointer_pressed(move |info: PointerEventInfo| {
                    if info.is_right_button_pressed {
                        right_select.call(id);
                    }
                })
                .content(
                    Button::new()
                        .automation_id(format!("process-{}", id.pid))
                        .automation_name(&process.name)
                        .automation_help_text(format!(
                            "PID {}{}. {}",
                            id.pid,
                            if input.selected { ", selected" } else { "" },
                            process
                                .unavailable
                                .as_deref()
                                .unwrap_or("Select to inspect process details.")
                        ))
                        .style(ButtonStyle::Subtle)
                        .resource_overrides(
                            ResourceOverrides::new()
                                .set("SubtleButtonBackgroundPointerOver", Color::transparent())
                                .set("SubtleButtonBackgroundPressed", Color::transparent())
                                .set("SubtleButtonBorderBrushPointerOver", Color::transparent())
                                .set("SubtleButtonBorderBrushPressed", Color::transparent()),
                        )
                        .height(32.0)
                        .horizontal_alignment(HorizontalAlignment::Stretch)
                        .horizontal_content_alignment(HorizontalAlignment::Stretch)
                        .content(
                            Grid::new()
                                .columns(widths(&input.columns, &input.widths))
                                .children(cells),
                        )
                        .on_click(move || select.call(id)),
                )
                .menu(menu),
        ];
        if row.has_children {
            children.push(
                Button::new()
                    .automation_id(format!("expand-{}", id.pid))
                    .automation_name(format!(
                        "{} {}",
                        if row.expanded { "Collapse" } else { "Expand" },
                        process.name
                    ))
                    .style(ButtonStyle::Subtle)
                    .width(36.0)
                    .height(32.0)
                    .min_height(0.0)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Center)
                    .margin(Thickness::new(indent + 12.0, 0.0, 0.0, 0.0))
                    .content(FontIcon::new().font_size(10.0).glyph(if row.expanded {
                        "\u{e70d}"
                    } else {
                        "\u{e76c}"
                    }))
                    .on_click(move || collapse.call(id))
                    .into(),
            );
        }
        Grid::new().children(children).into()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse(std::env::args().skip(1));
    let outcome = options.outcome.clone();
    App::run_component_with_policy::<TaskManager>(
        options,
        WindowPolicy::new()
            .title("Task Manager")
            .client_size(1250.0, 800.0),
    )?;
    if let Some(error) = outcome.0.lock().unwrap().take() {
        return Err(error.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "--report requires a finite source or --frames")]
    fn live_reporting_requires_a_frame_limit() {
        Options::parse(["--report", "report.txt"].map(String::from).into_iter());
    }

    #[test]
    fn reporting_accepts_finite_sources() {
        for source in [
            &["--demo"][..],
            &["--replay", "recording.jsonl"],
            &["--frames", "60"],
        ] {
            Options::parse(
                ["--report", "report.txt"]
                    .into_iter()
                    .chain(source.iter().copied())
                    .map(String::from),
            );
        }
    }
}
