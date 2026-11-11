#![windows_subsystem = "windows"]

mod bindings {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        non_upper_case_globals,
        dead_code,
        clippy::all
    )]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod model;
mod performance;
mod process;
mod source;

use model::{Model, Process, ProcessId, Row, Snapshot, Sort};
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
            return "Process monitor: no snapshots committed\n".into();
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
            "Process monitor: {} snapshots committed\n\
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
    Sort(Sort),
    Tree(bool),
    Select(ProcessId),
    Collapse(ProcessId),
    Pause,
    Navigate(Option<String>),
    Details,
    Metric(performance::Metric),
    CpuView(performance::CpuView),
    ColorScheme(ColorScheme),
    Finish,
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Processes,
    Performance,
}

struct Monitor {
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
    page: Page,
    show_details: bool,
    metric: performance::Metric,
    cpu_view: performance::CpuView,
    color_scheme: ColorScheme,
}

impl Monitor {
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

impl Component for Monitor {
    type Input = Options;
    type Message = Message;

    fn create(_: &Options, context: &ComponentContext<Self>) -> Self {
        let mut monitor = Self {
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
            page: Page::Processes,
            show_details: false,
            metric: performance::Metric::default(),
            cpu_view: performance::CpuView::default(),
            color_scheme: ColorScheme::default(),
        };
        monitor.acquire(context);
        monitor
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
            Message::Pause => {
                self.paused = !self.paused;
                self.timer = None;
                if !self.paused {
                    self.acquire(context);
                }
            }
            Message::Navigate(tag) => match tag.as_deref() {
                Some("processes") => self.page = Page::Processes,
                Some("performance") => self.page = Page::Performance,
                None => {}
                Some(_) => unreachable!(),
            },
            Message::Details => self.show_details = !self.show_details,
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
        context.window_title("Process monitor");
        context.on_color_scheme(context.callback(Message::ColorScheme));
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
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
        let content = match self.page {
            Page::Processes => self.processes_page(options, context),
            Page::Performance => performance::page(
                &self.model.history,
                self.metric,
                self.cpu_view,
                self.color_scheme,
                context.callback(Message::Metric),
                context.callback(Message::CpuView),
            ),
        };
        let mut actions = Vec::new();
        if self.page == Page::Processes {
            actions.extend([
                CheckBox::new()
                    .content("Process tree")
                    .is_checked(self.model.tree)
                    .on_is_checked_changed(
                        context.callback(|value: Option<bool>| Message::Tree(value == Some(true))),
                    )
                    .into(),
                Button::new()
                    .automation_id("details-toggle")
                    .style(ButtonStyle::Subtle)
                    .is_enabled(self.show_details || self.model.selected.is_some())
                    .content(if self.show_details {
                        "Hide details"
                    } else {
                        "Show details"
                    })
                    .on_click(context.callback(|()| Message::Details))
                    .into(),
            ]);
        }
        actions.push(self.pause_button(context).into());
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
                                Page::Performance => "Performance",
                            })
                            .font_size(24.0)
                            .font_weight(FontWeight::SEMI_BOLD)
                            .vertical_alignment(VerticalAlignment::Center),
                        StackPanel::new()
                            .grid_column(1)
                            .orientation(Orientation::Horizontal)
                            .spacing(12.0)
                            .children(actions),
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
                            Page::Performance => "performance",
                        },
                        content,
                    )]),
            ),
        ];
        if let Some(error) = &self.error {
            page.push(keyed(
                "error",
                InfoBar::new()
                    .automation_id("sampling-error")
                    .grid_row(1)
                    .margin(Thickness::new(0.0, 0.0, 0.0, 16.0))
                    .is_open(true)
                    .is_closable(false)
                    .severity(InfoBarSeverity::Error)
                    .title("Updates stopped - displayed data may be stale")
                    .message(error),
            ));
        }
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                TitleBar::new()
                    .title("Process monitor")
                    .subtitle(if options.demo {
                        "Demo"
                    } else if options.live() {
                        "Live"
                    } else {
                        "Replay"
                    })
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
            .into()
    }
}

impl Monitor {
    fn pause_button(&self, context: &ViewContext<Self>) -> Button {
        Button::new()
            .automation_id("pause")
            .min_width(88.0)
            .vertical_alignment(VerticalAlignment::Center)
            .content(if self.error.is_some() {
                "Stopped"
            } else if self.finished {
                "Complete"
            } else if self.paused {
                "Resume"
            } else {
                "Pause"
            })
            .is_enabled(!self.finished)
            .on_click(context.callback(|()| Message::Pause))
    }

    fn status(&self, options: &Options) -> String {
        match &self.error {
            Some(error) => format!("Stopped - displayed data is stale: {error}"),
            None => format!(
                "{} | {} processes, {} shown | snapshot {}{}",
                if options.demo {
                    "Demo / no process queries"
                } else if options.live() {
                    "Live / read-only"
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

    fn processes_page(&self, options: &Options, context: &ViewContext<Self>) -> View {
        let rows = Rc::clone(&self.model.visible);
        let keys = Rc::clone(&rows);
        let selected = self.model.selected;
        let on_select = self.on_select.clone();
        let on_collapse = self.on_collapse.clone();
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
                })
            },
        );
        let header = Grid::new().columns(columns()).children(
            [
                (Sort::Name, "Name", "sort-name"),
                (Sort::Pid, "PID", "sort-pid"),
                (Sort::Cpu, "CPU", "sort-cpu"),
                (Sort::Memory, "Memory (MB)", "sort-memory"),
                (Sort::Private, "Private (MB)", "sort-private"),
            ]
            .into_iter()
            .enumerate()
            .map(|(column, (sort, label, id))| {
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
                    .grid_column(column as i32)
                    .automation_id(id)
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
                    .horizontal_content_alignment(if column == 0 {
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
        if self.show_details {
            children.push(keyed(
                "details",
                Border::new()
                    .grid_row(2)
                    .border_brush(ThemeBrush::CardStroke)
                    .border_thickness(Thickness::new(0.0, 1.0, 0.0, 0.0))
                    .padding(Thickness::xy(12.0, 16.0))
                    .content(
                        ScrollViewer::new()
                            .max_height(190.0)
                            .content(component::<Details>(
                                "details",
                                DetailInput {
                                    process: self
                                        .model
                                        .selected
                                        .and_then(|id| self.model.processes.get(&id).cloned()),
                                    live: options.live(),
                                },
                            )),
                    ),
            ));
        }
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR, GridLength::Auto])
            .keyed_children(children)
            .into()
    }
}

fn columns() -> [GridLength; 5] {
    [
        GridLength::STAR,
        GridLength::Pixel(70.0),
        GridLength::Pixel(85.0),
        GridLength::Pixel(140.0),
        GridLength::Pixel(140.0),
    ]
}

fn memory(value: Option<u64>) -> String {
    value.map_or_else(|| "--".into(), |v| format!("{:.1}", v as f64 / 1_048_576.0))
}

fn cpu(process: &Process) -> String {
    process
        .cpu_tenths
        .map_or_else(|| "--".into(), |v| format!("{:.1}", f64::from(v) / 10.0))
}

#[derive(Clone, PartialEq)]
struct RowInput {
    row: Row,
    selected: bool,
    on_select: Callback<ProcessId>,
    on_collapse: Callback<ProcessId>,
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
        let indent = (row.depth.min(12) * 12) as f64;
        let mut cells: Vec<View> = vec![
            TextBlock::new()
                .text(&process.name)
                .text_trimming(TextTrimming::CharacterEllipsis)
                .vertical_alignment(VerticalAlignment::Center)
                .margin(Thickness::new(indent + 36.0, 0.0, 12.0, 0.0))
                .into(),
        ];
        for (index, value) in [
            process.id.pid.to_string(),
            process
                .cpu_tenths
                .map_or_else(|| "--".into(), |_| format!("{}%", cpu(process))),
            memory(process.working_set),
            memory(process.private_bytes),
        ]
        .into_iter()
        .enumerate()
        {
            cells.push(
                TextBlock::new()
                    .grid_column(index as i32 + 1)
                    .automation_id(format!("cell-{}-{index}", id.pid))
                    .vertical_alignment(VerticalAlignment::Center)
                    .text_alignment(TextAlignment::Right)
                    .margin(Thickness::xy(12.0, 0.0))
                    .text(value)
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
        let mut children: Vec<View> = vec![
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
                .background(if input.selected {
                    ThemeBrush::CardBackground
                } else {
                    ThemeBrush::SolidBackground
                })
                .height(32.0)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .horizontal_content_alignment(HorizontalAlignment::Stretch)
                .content(Grid::new().columns(columns()).children(cells))
                .on_click(move || select.call(id))
                .into(),
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

#[derive(Clone, PartialEq)]
struct DetailInput {
    process: Option<Rc<Process>>,
    live: bool,
}

struct Details {
    id: Option<ProcessId>,
    path: String,
    task: Option<ComponentTask>,
}

impl Details {
    fn load(&mut self, input: &DetailInput, context: &ComponentContext<Self>) {
        if let Some(task) = self.task.take() {
            task.cancel();
        }
        self.id = input.process.as_ref().map(|p| p.id);
        self.path = match self.id {
            None => "Select a process.".into(),
            Some(_) if !input.live => {
                "Executable paths are not recorded or queried during replay.".into()
            }
            Some(id) if id.created.is_none() => {
                "Process identity unavailable; details cannot be verified.".into()
            }
            Some(id) => {
                self.task = Some(context.spawn_background(move |_| (id, process::image_path(id))));
                "Loading executable path...".into()
            }
        };
    }
}

impl Component for Details {
    type Input = DetailInput;
    type Message = (ProcessId, Result<String, String>);

    fn create(input: &DetailInput, context: &ComponentContext<Self>) -> Self {
        let mut details = Self {
            id: None,
            path: String::new(),
            task: None,
        };
        details.load(input, context);
        details
    }

    fn input_changed(&mut self, input: &DetailInput, context: &ComponentContext<Self>) {
        if self.id != input.process.as_ref().map(|p| p.id) {
            self.load(input, context);
        }
    }

    fn update(&mut self, (id, result): Self::Message, _: &ComponentContext<Self>) {
        if self.id == Some(id) {
            self.task = None;
            self.path = result.unwrap_or_else(|error| format!("Unavailable: {error}"));
        }
    }

    fn view(&self, input: &DetailInput, _: &mut ViewContext<Self>) -> View {
        let mut children: Vec<View> = vec![
            TextBlock::new()
                .text(
                    input
                        .process
                        .as_ref()
                        .map_or("Process details", |p| p.name.as_str()),
                )
                .font_size(18.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .grid_column_span(2)
                .into(),
            TextBlock::new()
                .grid_row(1)
                .grid_column_span(2)
                .margin(Thickness::xy(0.0, 8.0))
                .text(self.path.clone())
                .text_wrapping(TextWrapping::Wrap)
                .into(),
        ];
        if let Some(process) = &input.process {
            children.extend([
                TextBlock::new()
                    .grid_row(2)
                    .text(format!(
                        "PID: {}  Parent PID: {}",
                        process.id.pid, process.parent_pid
                    ))
                    .into(),
                TextBlock::new()
                    .grid_row(2)
                    .grid_column(1)
                    .text(format!("Threads: {}", process.threads))
                    .into(),
                TextBlock::new()
                    .grid_row(3)
                    .text(format!("Working set: {} MB", memory(process.working_set)))
                    .into(),
                TextBlock::new()
                    .grid_row(3)
                    .grid_column(1)
                    .text(format!(
                        "Private commit: {} MB",
                        memory(process.private_bytes)
                    ))
                    .into(),
                TextBlock::new()
                    .grid_row(4)
                    .grid_column_span(2)
                    .margin(Thickness::new(0.0, 8.0, 0.0, 0.0))
                    .font_size(12.0)
                    .text(process.unavailable.as_deref().unwrap_or(
                        "CPU requires two samples. Working set includes shared resident pages.",
                    ))
                    .text_wrapping(TextWrapping::Wrap)
                    .into(),
            ]);
        }
        Grid::new()
            .columns([GridLength::STAR, GridLength::STAR])
            .rows([GridLength::Auto; 5])
            .children(children)
            .into()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse(std::env::args().skip(1));
    let outcome = options.outcome.clone();
    App::run_component_with_policy::<Monitor>(
        options,
        WindowPolicy::new()
            .title("Process monitor")
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
