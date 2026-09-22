#![windows_subsystem = "console"]

use std::cell::{Cell, RefCell};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::time::Instant;

use windows::Win32::{FILETIME, GetCurrentProcess, GetProcessTimes};
use windows::Win32::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX};
use windows_core::EventRevoker;
use windows_reactor::test::{
    LiveWindowActivationProbe, clear_live_performance_times, subscribe_live_rendering,
    subscribe_live_window_activation, take_live_performance_times,
};
use windows_reactor::{
    App, AppProxy, ChildrenControl as _, Component, ComponentContext, Grid, KeyedView, ListView,
    TextBlock, View, ViewContext, WindowVisuals,
};
use windows_reactor2 as reactor2;

mod allocator;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Frontend {
    Reactor,
    Reactor2,
}

impl Frontend {
    fn name(self) -> &'static str {
        match self {
            Self::Reactor => "reactor",
            Self::Reactor2 => "reactor2",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Workload {
    Text,
    Rotate,
    Reverse,
    Churn,
}

impl Workload {
    fn name(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Rotate => "rotate",
            Self::Reverse => "reverse",
            Self::Churn => "churn",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Surface {
    Grid,
    List,
}

impl Surface {
    fn name(self) -> &'static str {
        match self {
            Self::Grid => "grid",
            Self::List => "list",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Options {
    frontend: Frontend,
    surface: Surface,
    workload: Workload,
    count: usize,
    updates: usize,
    churn_count: usize,
}

impl Options {
    fn parse() -> Result<Option<Self>, String> {
        let mut frontend = None;
        let mut surface = Surface::Grid;
        let mut workload = Workload::Text;
        let mut count = 512;
        let mut updates = 120;
        let mut churn_count = 64;
        let mut args = env::args().skip(1);

        while let Some(argument) = args.next() {
            match argument.as_str() {
                "--frontend" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--frontend requires a value".to_string())?;
                    frontend = Some(match value.as_str() {
                        "reactor" => Frontend::Reactor,
                        "reactor2" => Frontend::Reactor2,
                        _ => return Err(format!("invalid --frontend value: {value}")),
                    });
                }
                "--workload" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--workload requires a value".to_string())?;
                    workload = match value.as_str() {
                        "text" => Workload::Text,
                        "rotate" => Workload::Rotate,
                        "reverse" => Workload::Reverse,
                        "churn" => Workload::Churn,
                        _ => return Err(format!("invalid --workload value: {value}")),
                    };
                }
                "--surface" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--surface requires a value".to_string())?;
                    surface = match value.as_str() {
                        "grid" => Surface::Grid,
                        "list" => Surface::List,
                        _ => return Err(format!("invalid --surface value: {value}")),
                    };
                }
                "--count" => count = parse_value(&mut args, "--count")?,
                "--updates" => updates = parse_value(&mut args, "--updates")?,
                "--churn-count" => churn_count = parse_value(&mut args, "--churn-count")?,
                "--help" | "-h" => {
                    print_help();
                    return Ok(None);
                }
                _ => return Err(format!("unknown argument: {argument}")),
            }
        }

        if count == 0 {
            return Err("--count must be at least 1".to_string());
        }
        if updates == 0 {
            return Err("--updates must be at least 1".to_string());
        }
        if churn_count > count {
            return Err("--churn-count must not exceed --count".to_string());
        }

        Ok(Some(Self {
            frontend: frontend.ok_or_else(|| "--frontend is required".to_string())?,
            surface,
            workload,
            count,
            updates,
            churn_count,
        }))
    }
}

fn parse_value(args: &mut impl Iterator<Item = String>, name: &str) -> Result<usize, String> {
    let value = args
        .next()
        .ok_or_else(|| format!("{name} requires a value"))?;
    value
        .parse()
        .map_err(|_| format!("invalid {name} value: {value}"))
}

fn print_help() {
    println!(
        "reactor-live-compare\n\
         \n\
         Options:\n\
           --frontend reactor|reactor2  Frontend to measure (required)\n\
           --surface grid|list          Native surface (default: grid)\n\
           --workload text|rotate|reverse|churn\n\
                                        Update shape (default: text)\n\
           --count N                    Logical item count (default: 512)\n\
           --updates N                  Render-paced updates (default: 120)\n\
           --churn-count N              Trailing children toggled by churn (default: 64)\n\
           --gate-reverse               Compare 512/1024 reverse native apply ratios\n\
           --gate-startup               Compare fresh-process startup phases\n\
           --startup-runs N             Runs per frontend (default: 12, minimum: 10)\n\
           --items-memory               Measure live 10k ItemsRepeater memory\n\
           -h, --help                   Print this help"
    );
}

#[derive(Clone)]
struct Model {
    values: Vec<Rc<str>>,
    order: Vec<usize>,
    visible: usize,
    revision: usize,
}

impl Model {
    fn new(count: usize) -> Self {
        Self {
            values: (0..count)
                .map(|index| Rc::from(format!("Cell {index}")))
                .collect(),
            order: (0..count).collect(),
            visible: count,
            revision: 0,
        }
    }

    fn advance(&mut self, options: &Options) {
        self.revision += 1;
        match options.workload {
            Workload::Text => {
                let index = self.revision.wrapping_mul(2_654_435_761) % self.values.len();
                self.values[index] = Rc::from(format!("Cell {index} revision {}", self.revision));
            }
            Workload::Rotate => self.order.rotate_left(1),
            Workload::Reverse => self.order.reverse(),
            Workload::Churn => {
                self.visible = if self.visible == self.values.len() {
                    self.values.len() - options.churn_count
                } else {
                    self.values.len()
                };
            }
        }
    }

    fn reactor_view(&self, surface: Surface) -> View {
        let children = self.order[..self.visible].iter().map(|index| {
            KeyedView::new(*index, TextBlock::new().text(self.values[*index].as_ref()))
        });
        match surface {
            Surface::Grid => Grid::new().keyed_children(children),
            Surface::List => ListView::new().items(children).into(),
        }
    }

    fn reactor2_view(&self, surface: Surface) -> reactor2::Visual {
        match surface {
            Surface::Grid => reactor2::Grid::new()
                .children(self.order[..self.visible].iter().map(|index| {
                    reactor2::keyed(
                        *index,
                        reactor2::TextBlock::new(Rc::clone(&self.values[*index])),
                    )
                }))
                .into(),
            Surface::List => {
                reactor2::ListView::new()
                    .items(self.order[..self.visible].iter().map(|index| {
                        reactor2::DataItem::new(*index, Rc::clone(&self.values[*index]))
                    }))
                    .into()
            }
        }
    }
}

#[derive(Default)]
struct MemoryStats {
    samples: u64,
    working_set_sum: u64,
    working_set_peak: u64,
    private_sum: u64,
    private_peak: u64,
}

impl MemoryStats {
    fn sample(&mut self) {
        let (working_set, private) = process_memory_bytes().unwrap();
        self.samples += 1;
        self.working_set_sum = self.working_set_sum.saturating_add(working_set);
        self.working_set_peak = self.working_set_peak.max(working_set);
        self.private_sum = self.private_sum.saturating_add(private);
        self.private_peak = self.private_peak.max(private);
    }

    fn working_set_average(&self) -> u64 {
        self.working_set_sum / self.samples.max(1)
    }

    fn private_average(&self) -> u64 {
        self.private_sum / self.samples.max(1)
    }
}

struct Measurement {
    started: Instant,
    allocations: u64,
    allocated_bytes: u64,
    live_bytes: u64,
    cpu_time_100ns: u64,
    memory: MemoryStats,
}

impl Measurement {
    fn start() -> Self {
        let mut memory = MemoryStats::default();
        memory.sample();
        Self {
            started: Instant::now(),
            allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed),
            allocated_bytes: allocator::allocated_bytes(),
            live_bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed),
            cpu_time_100ns: process_cpu_time_100ns().unwrap(),
            memory,
        }
    }

    fn report(
        mut self,
        options: &Options,
        updates: usize,
        update_times: &mut [f64],
        native_times: &mut [f64],
    ) {
        self.memory.sample();
        let elapsed = self.started.elapsed();
        let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - self.allocations;
        let allocated_bytes = allocator::allocated_bytes() - self.allocated_bytes;
        let live_bytes = allocator::CURRENT_BYTES
            .load(Ordering::Relaxed)
            .saturating_sub(self.live_bytes);
        let cpu_time_100ns = process_cpu_time_100ns()
            .unwrap()
            .saturating_sub(self.cpu_time_100ns);
        let update = distribution(update_times);
        let native = distribution(native_times);
        let divisor = updates.max(1) as f64;
        let cpu_time_ms = cpu_time_100ns as f64 / 10_000.0;
        let cpu_core_percent = cpu_time_100ns as f64 / 10_000_000.0 / elapsed.as_secs_f64() * 100.0;

        println!(
            "{{\"benchmark\":\"reactor-live-compare\",\"frontend\":\"{}\",\
             \"surface\":\"{}\",\"workload\":\"{}\",\"objects\":{},\"updates\":{},\
             \"churn_count\":{},\
             \"elapsed_ms\":{:.3},\"updates_per_second\":{:.3},\
             \"rust_allocations\":{},\"rust_allocations_per_update\":{:.3},\
             \"rust_alloc_bytes\":{},\"rust_alloc_bytes_per_update\":{:.3},\
             \"rust_live_delta_bytes\":{},\"cpu_time_ms\":{:.3},\
             \"cpu_core_percent\":{:.3},\"working_set_avg_bytes\":{},\
             \"working_set_peak_bytes\":{},\"private_avg_bytes\":{},\
             \"private_peak_bytes\":{},\"update_samples\":{},\
             \"update_avg_us\":{:.3},\"update_p95_us\":{:.3},\
             \"native_apply_samples\":{},\"native_apply_avg_us\":{:.3},\
             \"native_apply_p95_us\":{:.3}}}",
            options.frontend.name(),
            options.surface.name(),
            options.workload.name(),
            options.count,
            updates,
            options.churn_count,
            elapsed.as_secs_f64() * 1_000.0,
            updates as f64 / elapsed.as_secs_f64(),
            allocations,
            allocations as f64 / divisor,
            allocated_bytes,
            allocated_bytes as f64 / divisor,
            live_bytes,
            cpu_time_ms,
            cpu_core_percent,
            self.memory.working_set_average(),
            self.memory.working_set_peak,
            self.memory.private_average(),
            self.memory.private_peak,
            update.count,
            update.average,
            update.p95,
            native.count,
            native.average,
            native.p95,
        );
    }
}

struct Distribution {
    count: usize,
    average: f64,
    median: f64,
    p95: f64,
}

fn distribution(samples: &mut [f64]) -> Distribution {
    if samples.is_empty() {
        return Distribution {
            count: 0,
            average: 0.0,
            median: 0.0,
            p95: 0.0,
        };
    }
    samples.sort_by(f64::total_cmp);
    let average = samples.iter().sum::<f64>() / samples.len() as f64;
    let median = if samples.len().is_multiple_of(2) {
        let right = samples.len() / 2;
        (samples[right - 1] + samples[right]) / 2.0
    } else {
        samples[samples.len() / 2]
    };
    let p95 = samples[((samples.len() - 1) as f64 * 0.95).ceil() as usize];
    Distribution {
        count: samples.len(),
        average,
        median,
        p95,
    }
}

fn process_memory_bytes() -> windows::core::Result<(u64, u64)> {
    unsafe {
        let mut counters = PROCESS_MEMORY_COUNTERS_EX::default();
        let size = size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        let base = std::ptr::from_mut(&mut counters).cast::<PROCESS_MEMORY_COUNTERS>();
        if GetProcessMemoryInfo(GetCurrentProcess(), base, size).as_bool() {
            Ok((counters.WorkingSetSize as u64, counters.PrivateUsage as u64))
        } else {
            Err(windows::core::Error::from_thread())
        }
    }
}

fn process_cpu_time_100ns() -> windows::core::Result<u64> {
    unsafe {
        let mut creation = FILETIME::default();
        let mut exit = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();
        GetProcessTimes(
            GetCurrentProcess(),
            &mut creation,
            &mut exit,
            &mut kernel,
            &mut user,
        )
        .ok()?;
        let ticks = |value: FILETIME| {
            (u64::from(value.dwHighDateTime) << 32) | u64::from(value.dwLowDateTime)
        };
        Ok(ticks(kernel) + ticks(user))
    }
}

enum ReactorMessage {
    Begin,
    Tick,
    Finish {
        update_times: Vec<f64>,
        native_times: Vec<f64>,
    },
}

struct ReactorFixture {
    options: Options,
    model: Model,
    updates: usize,
    measurement: Option<Measurement>,
    completed: Rc<Cell<usize>>,
    tick_pending: Rc<Cell<bool>>,
    _rendering: EventRevoker,
}

impl Component for ReactorFixture {
    type Input = Options;
    type Message = ReactorMessage;

    fn create(options: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let sender = context.sender();
        let completed = Rc::new(Cell::new(0));
        let completed_for_rendering = Rc::clone(&completed);
        let tick_pending = Rc::new(Cell::new(false));
        let tick_pending_for_rendering = Rc::clone(&tick_pending);
        let started = Cell::new(false);
        let finish_pending = Cell::new(false);
        let updates = options.updates;
        let rendering = subscribe_live_rendering(move || {
            if !started.replace(true) {
                clear_live_performance_times();
                _ = sender.send(ReactorMessage::Begin);
            } else if completed_for_rendering.get() >= updates {
                if !finish_pending.replace(true) {
                    let (update_times, native_times) = take_live_performance_times();
                    _ = sender.send(ReactorMessage::Finish {
                        update_times,
                        native_times,
                    });
                }
            } else if !tick_pending_for_rendering.replace(true) {
                _ = sender.send(ReactorMessage::Tick);
            }
        })
        .unwrap();

        Self {
            options: options.clone(),
            model: Model::new(options.count),
            updates: 0,
            measurement: None,
            completed,
            tick_pending,
            _rendering: rendering,
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            ReactorMessage::Begin => self.measurement = Some(Measurement::start()),
            ReactorMessage::Tick => {
                self.tick_pending.set(false);
                self.model.advance(&self.options);
                self.updates += 1;
                self.completed.set(self.updates);
                if self.updates.is_multiple_of(10) {
                    self.measurement.as_mut().unwrap().memory.sample();
                }
            }
            ReactorMessage::Finish {
                mut update_times,
                mut native_times,
            } => {
                if update_times.len() > self.updates {
                    update_times =
                        update_times.split_off(update_times.len().saturating_sub(self.updates));
                }
                if native_times.len() > self.updates {
                    native_times =
                        native_times.split_off(native_times.len().saturating_sub(self.updates));
                }
                self.measurement.take().unwrap().report(
                    &self.options,
                    self.updates,
                    &mut update_times,
                    &mut native_times,
                );
                assert!(context.window().request_close());
            }
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        self.model.reactor_view(self.options.surface)
    }
}

struct Reactor2State {
    options: Options,
    app: AppProxy,
    runtime: reactor2::Runtime<MeasuredWinUiAdapter>,
    window: reactor2::native::NativeWindow,
    model: Model,
    measurement: Option<Measurement>,
    update_times: Vec<f64>,
    build_times: Vec<f64>,
    runtime_times: Vec<f64>,
    build_allocations: u64,
    build_bytes: u64,
    runtime_allocations: u64,
    runtime_bytes: u64,
    mutations: usize,
    updates: usize,
    finished: bool,
}

impl Reactor2State {
    fn render(&mut self) -> Result<(), String> {
        if self.finished {
            return Ok(());
        }
        if self.measurement.is_none() {
            *self.runtime.adapter().metrics.borrow_mut() = AdapterMetrics::default();
            self.measurement = Some(Measurement::start());
            return Ok(());
        }
        if self.updates < self.options.updates {
            self.model.advance(&self.options);
            let total_started = Instant::now();
            let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
            let bytes = allocator::allocated_bytes();
            let build_started = Instant::now();
            let declaration = self.model.reactor2_view(self.options.surface);
            self.build_times
                .push(build_started.elapsed().as_secs_f64() * 1_000_000.0);
            self.build_allocations += allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations;
            self.build_bytes += allocator::allocated_bytes() - bytes;

            let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
            let bytes = allocator::allocated_bytes();
            let runtime_started = Instant::now();
            let mutations = self
                .runtime
                .update(declaration)
                .map_err(|error| format!("Reactor2 update failed: {error:?}"))?;
            self.runtime_times
                .push(runtime_started.elapsed().as_secs_f64() * 1_000_000.0);
            self.runtime_allocations +=
                allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations;
            self.runtime_bytes += allocator::allocated_bytes() - bytes;
            self.mutations += mutations.len();
            self.update_times
                .push(total_started.elapsed().as_secs_f64() * 1_000_000.0);
            self.updates += 1;
            if self.updates.is_multiple_of(10) {
                self.measurement.as_mut().unwrap().memory.sample();
            }
            return Ok(());
        }

        self.finished = true;
        self.measurement.take().unwrap().report(
            &self.options,
            self.updates,
            &mut self.update_times,
            &mut Vec::new(),
        );
        self.runtime
            .adapter()
            .validate_graph(self.runtime.graph())
            .map_err(|error| format!("Reactor2 native graph mismatch: {error:?}"))?;
        self.report_phases();
        self.window
            .close()
            .map_err(|error| format!("Reactor2 window close failed: {error:?}"))?;
        self.app
            .exit()
            .map_err(|error| format!("application exit failed: {error}"))
    }

    fn report_phases(&mut self) {
        let build = distribution(&mut self.build_times);
        let runtime = distribution(&mut self.runtime_times);
        let metrics = self.runtime.adapter().metrics.borrow();
        assert_eq!(metrics.apply_times.len(), self.updates);
        let mut apply_times = metrics.apply_times.clone();
        let apply = distribution(&mut apply_times);
        let divisor = self.updates.max(1) as f64;
        println!(
            "{{\"benchmark\":\"reactor2-live-phases\",\"surface\":\"{}\",\"workload\":\"{}\",\
             \"objects\":{},\"updates\":{},\"declaration_avg_us\":{:.3},\
             \"declaration_p95_us\":{:.3},\"declaration_allocations_per_update\":{:.3},\
             \"declaration_bytes_per_update\":{:.3},\"runtime_avg_us\":{:.3},\
             \"runtime_p95_us\":{:.3},\"runtime_allocations_per_update\":{:.3},\
             \"runtime_bytes_per_update\":{:.3},\"native_apply_avg_us\":{:.3},\
             \"native_apply_p95_us\":{:.3},\"native_allocations_per_update\":{:.3},\
             \"native_bytes_per_update\":{:.3},\"mutations_per_update\":{:.3}}}",
            self.options.surface.name(),
            self.options.workload.name(),
            self.options.count,
            self.updates,
            build.average,
            build.p95,
            self.build_allocations as f64 / divisor,
            self.build_bytes as f64 / divisor,
            runtime.average,
            runtime.p95,
            self.runtime_allocations as f64 / divisor,
            self.runtime_bytes as f64 / divisor,
            apply.average,
            apply.p95,
            metrics.apply_allocations as f64 / divisor,
            metrics.apply_bytes as f64 / divisor,
            self.mutations as f64 / divisor,
        );
    }
}

#[derive(Default)]
struct AdapterMetrics {
    apply_times: Vec<f64>,
    apply_allocations: u64,
    apply_bytes: u64,
}

struct MeasuredWinUiAdapter {
    inner: reactor2::native::WinUiAdapter,
    metrics: Rc<RefCell<AdapterMetrics>>,
}

impl MeasuredWinUiAdapter {
    fn new(metrics: Rc<RefCell<AdapterMetrics>>) -> Self {
        Self {
            inner: reactor2::native::WinUiAdapter::default(),
            metrics,
        }
    }

    fn open_window(
        &self,
        root: reactor2::ObjectId,
    ) -> Result<reactor2::native::NativeWindow, reactor2::native::WinUiError> {
        self.inner.open_window(root)
    }

    fn validate_graph(
        &self,
        graph: &reactor2::RetainedGraph,
    ) -> Result<(), reactor2::native::WinUiError> {
        self.inner.validate_graph(graph)
    }
}

impl reactor2::Adapter for MeasuredWinUiAdapter {
    type Error = reactor2::native::WinUiError;

    fn validate(&self, mutations: &[reactor2::Mutation]) -> Result<(), Self::Error> {
        self.inner.validate(mutations)
    }

    fn apply(&mut self, mutations: &[reactor2::Mutation]) -> Result<(), Self::Error> {
        let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
        let bytes = allocator::allocated_bytes();
        let started = Instant::now();
        let result = self.inner.apply(mutations);
        let mut metrics = self.metrics.borrow_mut();
        metrics
            .apply_times
            .push(started.elapsed().as_secs_f64() * 1_000_000.0);
        metrics.apply_allocations += allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations;
        metrics.apply_bytes += allocator::allocated_bytes() - bytes;
        result
    }

    fn focus(&mut self, object: reactor2::ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object)
    }
}

struct Reactor2Host {
    _state: Rc<RefCell<Reactor2State>>,
    _rendering: EventRevoker,
}

fn run_reactor2(options: Options) -> windows_core::Result<()> {
    App::run_with(move |app| {
        let model = Model::new(options.count);
        let metrics = Rc::new(RefCell::new(AdapterMetrics::default()));
        let mut runtime = reactor2::Runtime::new(MeasuredWinUiAdapter::new(metrics));
        runtime
            .update(model.reactor2_view(options.surface))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let window = runtime.adapter().open_window(root).unwrap();
        let state = Rc::new(RefCell::new(Reactor2State {
            options,
            app: app.proxy(),
            runtime,
            window,
            model,
            measurement: None,
            update_times: Vec::new(),
            build_times: Vec::new(),
            runtime_times: Vec::new(),
            build_allocations: 0,
            build_bytes: 0,
            runtime_allocations: 0,
            runtime_bytes: 0,
            mutations: 0,
            updates: 0,
            finished: false,
        }));
        let weak = Rc::downgrade(&state);
        let rendering = subscribe_live_rendering(move || {
            let Some(state) = weak.upgrade() else {
                return;
            };
            if let Err(error) = state.borrow_mut().render() {
                eprintln!("reactor-live-compare: {error}");
                std::process::exit(1);
            }
        })?;
        Ok(Reactor2Host {
            _state: state,
            _rendering: rendering,
        })
    })
}

struct ItemsMemoryPoint {
    stage: &'static str,
    working_set: u64,
    private: u64,
    rust_live: u64,
    graph_objects: usize,
    realized_rows: usize,
}

fn items_memory_point(
    stage: &'static str,
    runtime: Option<&reactor2::Runtime<reactor2::native::WinUiAdapter>>,
    repeater: Option<reactor2::ObjectId>,
) -> ItemsMemoryPoint {
    let (working_set, private) = process_memory_bytes().unwrap();
    ItemsMemoryPoint {
        stage,
        working_set,
        private,
        rust_live: allocator::CURRENT_BYTES.load(Ordering::Relaxed),
        graph_objects: runtime.map_or(0, |runtime| runtime.graph().object_count()),
        realized_rows: runtime.zip(repeater).map_or(0, |(runtime, repeater)| {
            runtime
                .graph()
                .children(repeater, reactor2::RelationId::Items)
                .unwrap()
                .len()
        }),
    }
}

fn items_memory_view(revision: usize, count: usize) -> reactor2::Visual {
    reactor2::ItemsRepeater::new()
        .virtual_source(reactor2::VirtualSource::new(
            1,
            count,
            reactor2::Key::from,
            move |index| -> reactor2::Visual {
                reactor2::TextBlock::new(format!("{revision}:{index}")).into()
            },
        ))
        .into()
}

fn run_items_memory() -> windows_core::Result<()> {
    App::run_with(|app| {
        let mut points = Vec::with_capacity(7);
        points.push(items_memory_point("startup", None, None));
        {
            let mut runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
            runtime.update(items_memory_view(0, 0)).unwrap();
            let repeater = runtime.graph().root().unwrap();
            let window = runtime.adapter().create_window(repeater).unwrap();
            points.push(items_memory_point(
                "mounted_empty",
                Some(&runtime),
                Some(repeater),
            ));

            runtime.update(items_memory_view(1, 10_000)).unwrap();
            points.push(items_memory_point(
                "source_10k",
                Some(&runtime),
                Some(repeater),
            ));
            for index in 0..8 {
                runtime
                    .adapter()
                    .realize_virtual_item(repeater, index)
                    .unwrap();
            }
            runtime.dispatch_native_events().unwrap();
            assert_eq!(
                runtime
                    .graph()
                    .children(repeater, reactor2::RelationId::Items)
                    .unwrap()
                    .len(),
                8
            );
            points.push(items_memory_point(
                "source_10k_realized_8",
                Some(&runtime),
                Some(repeater),
            ));

            runtime.update(items_memory_view(2, 10_000)).unwrap();
            runtime.dispatch_native_events().unwrap();
            points.push(items_memory_point(
                "source_10k_replaced",
                Some(&runtime),
                Some(repeater),
            ));

            runtime.update(reactor2::ItemsRepeater::new()).unwrap();
            runtime.dispatch_native_events().unwrap();
            points.push(items_memory_point(
                "source_cleared",
                Some(&runtime),
                Some(repeater),
            ));
            window.close().unwrap();
        }
        points.push(items_memory_point("teardown", None, None));

        let baseline = &points[1];
        for point in &points {
            println!(
                "{{\"benchmark\":\"reactor2-live-items-memory\",\"stage\":\"{}\",\
                 \"working_set_bytes\":{},\"private_bytes\":{},\"rust_live_bytes\":{},\
                 \"working_set_delta_from_mount\":{},\"private_delta_from_mount\":{},\
                 \"rust_live_delta_from_mount\":{},\"graph_objects\":{},\"realized_rows\":{}}}",
                point.stage,
                point.working_set,
                point.private,
                point.rust_live,
                point.working_set as i64 - baseline.working_set as i64,
                point.private as i64 - baseline.private as i64,
                point.rust_live as i64 - baseline.rust_live as i64,
                point.graph_objects,
                point.realized_rows,
            );
        }
        app.exit()
    })
}

#[derive(Clone, Copy)]
struct LifecycleMemorySample {
    working_set: u64,
    private: u64,
    rust_live: u64,
}

struct NativeLifecycleState {
    cycle: usize,
    cycles: usize,
    items: usize,
    realized: usize,
    closing: bool,
    mounted: Vec<LifecycleMemorySample>,
    closed: Vec<LifecycleMemorySample>,
    runtime: Option<reactor2::Runtime<reactor2::native::WinUiAdapter>>,
    window: Option<reactor2::native::NativeWindow>,
    proxy: reactor2::AppProxy,
}

struct NativeLifecycleHost {
    _state: Rc<RefCell<NativeLifecycleState>>,
    _rendering: EventRevoker,
}

fn lifecycle_memory_sample() -> LifecycleMemorySample {
    let (working_set, private) = process_memory_bytes().unwrap();
    LifecycleMemorySample {
        working_set,
        private,
        rust_live: allocator::CURRENT_BYTES.load(Ordering::Relaxed),
    }
}

fn lifecycle_slope(
    samples: &[LifecycleMemorySample],
    select: impl Fn(&LifecycleMemorySample) -> u64,
) -> f64 {
    let start = samples.len() / 2;
    let samples = &samples[start..];
    if samples.len() < 2 {
        return 0.0;
    }
    let first = select(&samples[0]) as f64;
    let last = select(samples.last().unwrap()) as f64;
    (last - first) / (samples.len() - 1) as f64
}

fn mount_native_lifecycle_cycle(
    state: &Rc<RefCell<NativeLifecycleState>>,
) -> windows_core::Result<()> {
    let state_ref = state.borrow();
    let cycle = state_ref.cycle;
    let items = state_ref.items;
    let realized = state_ref.realized;
    drop(state_ref);
    let mut runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
    runtime.update(items_memory_view(cycle, items)).unwrap();
    let repeater = runtime.graph().root().unwrap();
    let mut window = runtime
        .adapter()
        .create_window_with_policy(
            repeater,
            &reactor2::native::WindowPolicy::new()
                .title("Reactor2 lifecycle")
                .client_size(320.0, 240.0),
        )
        .unwrap();
    let weak_state = Rc::downgrade(state);
    window.set_closed(move || {
        let Some(state) = weak_state.upgrade() else {
            return Ok(());
        };
        let mut state_ref = state.borrow_mut();
        state_ref.window.take();
        state_ref.runtime.take();
        state_ref.closed.push(lifecycle_memory_sample());
        state_ref.cycle += 1;
        if state_ref.cycle == state_ref.cycles {
            emit_native_lifecycle_sample(&state_ref);
            state_ref.proxy.exit().unwrap();
            return Ok(());
        }
        state_ref.closing = false;
        drop(state_ref);
        mount_native_lifecycle_cycle(&state)
    })?;
    runtime.dispatch_native_events().unwrap();
    runtime
        .update(items_memory_view(cycle.wrapping_add(1), items))
        .unwrap();
    runtime.dispatch_native_events().unwrap();
    for index in 0..realized {
        runtime
            .adapter()
            .realize_virtual_item(repeater, index)
            .unwrap();
    }
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.graph().object_count(), realized + 1);
    assert_eq!(runtime.adapter().total_virtual_shell_count(), realized);
    assert_eq!(runtime.adapter().queued_event_count(), 0);
    window.activate().unwrap();
    let mut state = state.borrow_mut();
    state.runtime = Some(runtime);
    state.window = Some(window);
    Ok(())
}

fn emit_native_lifecycle_sample(state: &NativeLifecycleState) {
    let mounted_private_slope = lifecycle_slope(&state.mounted, |sample| sample.private);
    let closed_private_slope = lifecycle_slope(&state.closed, |sample| sample.private);
    let mounted_working_set_slope = lifecycle_slope(&state.mounted, |sample| sample.working_set);
    let closed_working_set_slope = lifecycle_slope(&state.closed, |sample| sample.working_set);
    let closed_rust_slope = lifecycle_slope(&state.closed, |sample| sample.rust_live);
    let first_mounted = state.mounted.first().unwrap();
    let last_mounted = state.mounted.last().unwrap();
    let last_closed = state.closed.last().unwrap();
    println!(
        "{{\"benchmark\":\"reactor2-native-lifecycle\",\"cycles\":{},\
         \"items\":{},\"realized\":{},\
         \"first_mounted_private_bytes\":{},\"last_mounted_private_bytes\":{},\
         \"last_closed_private_bytes\":{},\"first_mounted_working_set_bytes\":{},\
         \"last_mounted_working_set_bytes\":{},\"last_closed_working_set_bytes\":{},\
         \"last_closed_rust_live_bytes\":{},\"mounted_private_slope\":{:.3},\
         \"closed_private_slope\":{:.3},\"mounted_working_set_slope\":{:.3},\
         \"closed_working_set_slope\":{:.3},\"closed_rust_slope\":{:.3}}}",
        state.cycles,
        state.items,
        state.realized,
        first_mounted.private,
        last_mounted.private,
        last_closed.private,
        first_mounted.working_set,
        last_mounted.working_set,
        last_closed.working_set,
        last_closed.rust_live,
        mounted_private_slope,
        closed_private_slope,
        mounted_working_set_slope,
        closed_working_set_slope,
        closed_rust_slope,
    );
}

fn run_native_lifecycle_child(
    cycles: usize,
    items: usize,
    realized: usize,
) -> windows_core::Result<()> {
    reactor2::App::run_with(move |app| {
        let state = Rc::new(RefCell::new(NativeLifecycleState {
            cycle: 0,
            cycles,
            items,
            realized,
            closing: false,
            mounted: Vec::with_capacity(cycles),
            closed: Vec::with_capacity(cycles),
            runtime: None,
            window: None,
            proxy: app.proxy(),
        }));
        let rendering_state = Rc::clone(&state);
        let rendering = subscribe_live_rendering(move || {
            let mut state = rendering_state.borrow_mut();
            if state.window.is_none() || state.closing {
                return;
            }
            state.mounted.push(lifecycle_memory_sample());
            state.closing = true;
            let window = state.window.as_ref().unwrap().clone();
            drop(state);
            window.close().unwrap();
        })?;
        mount_native_lifecycle_cycle(&state)?;
        Ok(NativeLifecycleHost {
            _state: state,
            _rendering: rendering,
        })
    })
}

#[derive(Clone, Copy)]
struct NativeLifecycleSummary {
    closed_private_slope: f64,
    closed_working_set_slope: f64,
    closed_rust_slope: f64,
}

fn run_native_lifecycle_process(
    cycles: usize,
    items: usize,
    realized: usize,
) -> Result<NativeLifecycleSummary, String> {
    let output = Command::new(env::current_exe().map_err(|error| error.to_string())?)
        .args([
            "--native-lifecycle-child",
            "--lifecycle-cycles",
            &cycles.to_string(),
            "--lifecycle-items",
            &items.to_string(),
            "--lifecycle-realized",
            &realized.to_string(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| format!("native lifecycle child failed to start: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "native lifecycle child failed with {}",
            output.status
        ));
    }
    let output = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    let line = output
        .lines()
        .find(|line| line.contains("\"benchmark\":\"reactor2-native-lifecycle\""))
        .ok_or_else(|| "native lifecycle child did not report a sample".to_string())?;
    println!("{line}");
    Ok(NativeLifecycleSummary {
        closed_private_slope: json_number(line, "closed_private_slope")?,
        closed_working_set_slope: json_number(line, "closed_working_set_slope")?,
        closed_rust_slope: json_number(line, "closed_rust_slope")?,
    })
}

fn lifecycle_median(
    summaries: &[NativeLifecycleSummary],
    select: impl Fn(&NativeLifecycleSummary) -> f64,
) -> f64 {
    let mut values = summaries.iter().map(select).collect::<Vec<_>>();
    values.sort_by(f64::total_cmp);
    values[values.len() / 2]
}

fn run_native_lifecycle_gate() -> Result<(), String> {
    let runs = startup_argument("--lifecycle-runs", 10)?;
    let cycles = startup_argument("--lifecycle-cycles", 40)?;
    let items = startup_argument("--lifecycle-items", 10_000)?;
    let realized = startup_argument("--lifecycle-realized", 8)?;
    if runs < 5 || cycles < 10 {
        return Err("native lifecycle gate requires at least 5 runs and 10 cycles".to_string());
    }
    if realized > items {
        return Err("--lifecycle-realized cannot exceed --lifecycle-items".to_string());
    }
    let mut baseline = Vec::with_capacity(runs);
    let mut loaded = Vec::with_capacity(runs);
    for run in 0..runs {
        if run.is_multiple_of(2) {
            baseline.push(run_native_lifecycle_process(cycles, 0, 0)?);
            loaded.push(run_native_lifecycle_process(cycles, items, realized)?);
        } else {
            loaded.push(run_native_lifecycle_process(cycles, items, realized)?);
            baseline.push(run_native_lifecycle_process(cycles, 0, 0)?);
        }
    }
    let baseline_private = lifecycle_median(&baseline, |sample| sample.closed_private_slope);
    let loaded_private = lifecycle_median(&loaded, |sample| sample.closed_private_slope);
    let baseline_working_set =
        lifecycle_median(&baseline, |sample| sample.closed_working_set_slope);
    let loaded_working_set = lifecycle_median(&loaded, |sample| sample.closed_working_set_slope);
    let baseline_rust = lifecycle_median(&baseline, |sample| sample.closed_rust_slope);
    let loaded_rust = lifecycle_median(&loaded, |sample| sample.closed_rust_slope);
    let private_excess = loaded_private - baseline_private;
    let working_set_excess = loaded_working_set - baseline_working_set;
    println!(
        "{{\"benchmark\":\"reactor2-native-lifecycle-summary\",\"runs\":{runs},\
         \"cycles\":{cycles},\"items\":{items},\"realized\":{realized},\
         \"baseline_closed_private_slope_median\":{baseline_private:.3},\
         \"loaded_closed_private_slope_median\":{loaded_private:.3},\
         \"loaded_private_excess_slope\":{private_excess:.3},\
         \"baseline_closed_working_set_slope_median\":{baseline_working_set:.3},\
         \"loaded_closed_working_set_slope_median\":{loaded_working_set:.3},\
         \"loaded_working_set_excess_slope\":{working_set_excess:.3},\
         \"baseline_closed_rust_slope_median\":{baseline_rust:.3},\
         \"loaded_closed_rust_slope_median\":{loaded_rust:.3}}}"
    );
    const MAX_NATIVE_EXCESS_PER_CYCLE: f64 = 128.0 * 1024.0;
    if baseline_rust != 0.0 || loaded_rust != 0.0 {
        return Err("native lifecycle retained Rust allocations".to_string());
    }
    if private_excess > MAX_NATIVE_EXCESS_PER_CYCLE
        || working_set_excess > MAX_NATIVE_EXCESS_PER_CYCLE
    {
        return Err("native lifecycle exceeded empty-window process-memory growth".to_string());
    }
    Ok(())
}

struct StartupReactor;

impl Component for StartupReactor {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor startup");
        context.window_visuals(WindowVisuals::new().client_size(320.0, 200.0));
        TextBlock::new().text("startup").into()
    }
}

struct StartupReactorHost {
    _activation: LiveWindowActivationProbe,
    _rendering: EventRevoker,
}

struct StartupReactor2Host {
    _runtime: reactor2::Runtime<reactor2::native::WinUiAdapter>,
    _window: Rc<RefCell<Option<reactor2::native::NativeWindow>>>,
    _rendering: EventRevoker,
}

fn elapsed_us(start: Instant, end: Instant) -> f64 {
    end.duration_since(start).as_secs_f64() * 1_000_000.0
}

fn cpu_elapsed_us(start: u64, end: u64) -> f64 {
    end.saturating_sub(start) as f64 / 10.0
}

struct StartupTiming {
    entry: Instant,
    app_initialized: Instant,
    activated: Instant,
    first_frame: Instant,
    entry_cpu: u64,
    app_cpu: u64,
    activation_cpu: u64,
    frame_cpu: u64,
}

fn emit_startup_sample(frontend: Frontend, timing: StartupTiming) -> Result<(), String> {
    let StartupTiming {
        entry,
        app_initialized,
        activated,
        first_frame,
        entry_cpu,
        app_cpu,
        activation_cpu,
        frame_cpu,
    } = timing;
    let (working_set, private) =
        process_memory_bytes().map_err(|error| format!("memory sample failed: {error}"))?;
    println!(
        "{{\"benchmark\":\"reactor-live-startup-sample\",\"frontend\":\"{}\",\
         \"entry_to_app_us\":{:.3},\"app_to_activation_us\":{:.3},\
         \"activation_to_frame_us\":{:.3},\"entry_to_frame_us\":{:.3},\
         \"entry_to_app_cpu_us\":{:.3},\"app_to_activation_cpu_us\":{:.3},\
         \"activation_to_frame_cpu_us\":{:.3},\"entry_to_frame_cpu_us\":{:.3},\
         \"working_set_bytes\":{},\"private_bytes\":{}}}",
        frontend.name(),
        elapsed_us(entry, app_initialized),
        elapsed_us(app_initialized, activated),
        elapsed_us(activated, first_frame),
        elapsed_us(entry, first_frame),
        cpu_elapsed_us(entry_cpu, app_cpu),
        cpu_elapsed_us(app_cpu, activation_cpu),
        cpu_elapsed_us(activation_cpu, frame_cpu),
        cpu_elapsed_us(entry_cpu, frame_cpu),
        working_set,
        private,
    );
    std::io::stdout()
        .flush()
        .map_err(|error| format!("startup sample flush failed: {error}"))
}

fn run_reactor_startup(entry: Instant, entry_cpu: u64) -> windows_core::Result<()> {
    App::run_with(move |app| {
        let app_initialized = Instant::now();
        let app_cpu = process_cpu_time_100ns().unwrap();
        let activated = Rc::new(Cell::new(None::<Instant>));
        let activation_cpu = Rc::new(Cell::new(None::<u64>));
        let activated_for_probe = Rc::clone(&activated);
        let activation_cpu_for_probe = Rc::clone(&activation_cpu);
        let activation = subscribe_live_window_activation(move || {
            if activated_for_probe.get().is_none() {
                activated_for_probe.set(Some(Instant::now()));
                activation_cpu_for_probe.set(Some(process_cpu_time_100ns().unwrap()));
            }
        });
        let reported = Rc::new(Cell::new(false));
        let reported_for_rendering = Rc::clone(&reported);
        let activated_for_rendering = Rc::clone(&activated);
        let activation_cpu_for_rendering = Rc::clone(&activation_cpu);
        let proxy = app.proxy();
        let rendering = subscribe_live_rendering(move || {
            let Some(activated) = activated_for_rendering.get() else {
                return;
            };
            let Some(activation_cpu) = activation_cpu_for_rendering.get() else {
                return;
            };
            if reported_for_rendering.replace(true) {
                return;
            }
            let first_frame = Instant::now();
            let frame_cpu = process_cpu_time_100ns().unwrap();
            if let Err(error) = emit_startup_sample(
                Frontend::Reactor,
                StartupTiming {
                    entry,
                    app_initialized,
                    activated,
                    first_frame,
                    entry_cpu,
                    app_cpu,
                    activation_cpu,
                    frame_cpu,
                },
            ) {
                eprintln!("reactor-live-compare: {error}");
                std::process::exit(1);
            }
            if let Err(error) = proxy.exit() {
                eprintln!("reactor-live-compare: application exit failed: {error}");
                std::process::exit(1);
            }
        })?;
        app.open_window(View::component::<StartupReactor>(()))?;
        Ok(StartupReactorHost {
            _activation: activation,
            _rendering: rendering,
        })
    })
}

fn run_reactor2_startup(entry: Instant, entry_cpu: u64) -> windows_core::Result<()> {
    App::run_with(move |app| {
        let app_initialized = Instant::now();
        let app_cpu = process_cpu_time_100ns().unwrap();
        let activated = Rc::new(Cell::new(None::<Instant>));
        let activation_cpu = Rc::new(Cell::new(None::<u64>));
        let window = Rc::new(RefCell::new(None::<reactor2::native::NativeWindow>));
        let reported = Rc::new(Cell::new(false));
        let reported_for_rendering = Rc::clone(&reported);
        let activated_for_rendering = Rc::clone(&activated);
        let activation_cpu_for_rendering = Rc::clone(&activation_cpu);
        let window_for_rendering = Rc::clone(&window);
        let proxy = app.proxy();
        let rendering = subscribe_live_rendering(move || {
            let Some(activated) = activated_for_rendering.get() else {
                return;
            };
            let Some(activation_cpu) = activation_cpu_for_rendering.get() else {
                return;
            };
            if reported_for_rendering.replace(true) {
                return;
            }
            let first_frame = Instant::now();
            let frame_cpu = process_cpu_time_100ns().unwrap();
            if let Err(error) = emit_startup_sample(
                Frontend::Reactor2,
                StartupTiming {
                    entry,
                    app_initialized,
                    activated,
                    first_frame,
                    entry_cpu,
                    app_cpu,
                    activation_cpu,
                    frame_cpu,
                },
            ) {
                eprintln!("reactor-live-compare: {error}");
                std::process::exit(1);
            }
            if let Some(window) = window_for_rendering.borrow_mut().take()
                && let Err(error) = window.close()
            {
                eprintln!("reactor-live-compare: window close failed: {error:?}");
                std::process::exit(1);
            }
            if let Err(error) = proxy.exit() {
                eprintln!("reactor-live-compare: application exit failed: {error}");
                std::process::exit(1);
            }
        })?;

        let mut runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        runtime.update(reactor2::TextBlock::new("startup")).unwrap();
        let root = runtime.graph().root().unwrap();
        let native_window = runtime
            .adapter()
            .create_window_with_policy(
                root,
                &reactor2::native::WindowPolicy::new()
                    .title("Reactor startup")
                    .client_size(320.0, 200.0),
            )
            .unwrap();
        native_window.activate().unwrap();
        activated.set(Some(Instant::now()));
        activation_cpu.set(Some(process_cpu_time_100ns().unwrap()));
        *window.borrow_mut() = Some(native_window);
        Ok(StartupReactor2Host {
            _runtime: runtime,
            _window: window,
            _rendering: rendering,
        })
    })
}

#[derive(Clone)]
struct StartupSample {
    frontend: Frontend,
    entry_to_app_us: f64,
    app_to_activation_us: f64,
    activation_to_frame_us: f64,
    entry_to_frame_us: f64,
    entry_to_app_cpu_us: f64,
    app_to_activation_cpu_us: f64,
    activation_to_frame_cpu_us: f64,
    entry_to_frame_cpu_us: f64,
    working_set_bytes: f64,
    private_bytes: f64,
    process_to_report_us: f64,
}

fn json_number(line: &str, field: &str) -> Result<f64, String> {
    let field = format!("\"{field}\":");
    let start = line
        .find(&field)
        .map(|index| index + field.len())
        .ok_or_else(|| format!("missing {field}"))?;
    let end = line[start..]
        .find([',', '}'])
        .map_or(line.len(), |index| start + index);
    line[start..end]
        .parse()
        .map_err(|error| format!("invalid {field}: {error}"))
}

fn run_startup_process(frontend: Frontend) -> Result<StartupSample, String> {
    let executable =
        env::current_exe().map_err(|error| format!("failed to locate benchmark: {error}"))?;
    let started = Instant::now();
    let mut child = Command::new(executable)
        .args(["--startup-child", "--frontend", frontend.name()])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| format!("failed to start {frontend:?}: {error}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| format!("{frontend:?} stdout was not captured"))?;
    let mut sample_line = None;
    for line in BufReader::new(stdout).lines() {
        let line = line.map_err(|error| format!("{frontend:?} output failed: {error}"))?;
        if line.contains("\"benchmark\":\"reactor-live-startup-sample\"") {
            sample_line = Some((line, started.elapsed().as_secs_f64() * 1_000_000.0));
            break;
        }
    }
    let status = child
        .wait()
        .map_err(|error| format!("failed to wait for {frontend:?}: {error}"))?;
    if !status.success() {
        return Err(format!("{frontend:?} startup child failed with {status}"));
    }
    let (line, process_to_report_us) =
        sample_line.ok_or_else(|| format!("{frontend:?} startup sample was not reported"))?;
    Ok(StartupSample {
        frontend,
        entry_to_app_us: json_number(&line, "entry_to_app_us")?,
        app_to_activation_us: json_number(&line, "app_to_activation_us")?,
        activation_to_frame_us: json_number(&line, "activation_to_frame_us")?,
        entry_to_frame_us: json_number(&line, "entry_to_frame_us")?,
        entry_to_app_cpu_us: json_number(&line, "entry_to_app_cpu_us")?,
        app_to_activation_cpu_us: json_number(&line, "app_to_activation_cpu_us")?,
        activation_to_frame_cpu_us: json_number(&line, "activation_to_frame_cpu_us")?,
        entry_to_frame_cpu_us: json_number(&line, "entry_to_frame_cpu_us")?,
        working_set_bytes: json_number(&line, "working_set_bytes")?,
        private_bytes: json_number(&line, "private_bytes")?,
        process_to_report_us,
    })
}

fn startup_argument(name: &str, default: usize) -> Result<usize, String> {
    let args = env::args().collect::<Vec<_>>();
    let Some(index) = args.iter().position(|argument| argument == name) else {
        return Ok(default);
    };
    let value = args
        .get(index + 1)
        .ok_or_else(|| format!("{name} requires a value"))?;
    value
        .parse()
        .map_err(|_| format!("invalid {name} value: {value}"))
}

fn startup_distribution(
    samples: &[StartupSample],
    select: impl Fn(&StartupSample) -> f64,
) -> Distribution {
    let mut values = samples.iter().map(select).collect::<Vec<_>>();
    distribution(&mut values)
}

fn report_startup_summary(frontend: Frontend, samples: &[StartupSample]) {
    let entry_to_app = startup_distribution(samples, |sample| sample.entry_to_app_us);
    let app_to_activation = startup_distribution(samples, |sample| sample.app_to_activation_us);
    let activation_to_frame = startup_distribution(samples, |sample| sample.activation_to_frame_us);
    let entry_to_frame = startup_distribution(samples, |sample| sample.entry_to_frame_us);
    let entry_to_frame_cpu = startup_distribution(samples, |sample| sample.entry_to_frame_cpu_us);
    let process_to_report = startup_distribution(samples, |sample| sample.process_to_report_us);
    let working_set = startup_distribution(samples, |sample| sample.working_set_bytes);
    let private = startup_distribution(samples, |sample| sample.private_bytes);
    println!(
        "{{\"benchmark\":\"reactor-live-startup-summary\",\"frontend\":\"{}\",\
         \"samples\":{},\"entry_to_app_median_us\":{:.3},\"entry_to_app_p95_us\":{:.3},\
         \"app_to_activation_median_us\":{:.3},\"app_to_activation_p95_us\":{:.3},\
         \"activation_to_frame_median_us\":{:.3},\"activation_to_frame_p95_us\":{:.3},\
         \"entry_to_frame_median_us\":{:.3},\"entry_to_frame_p95_us\":{:.3},\
         \"entry_to_frame_cpu_median_us\":{:.3},\"entry_to_frame_cpu_p95_us\":{:.3},\
         \"process_to_report_median_us\":{:.3},\"process_to_report_p95_us\":{:.3},\
         \"working_set_median_bytes\":{:.0},\"working_set_p95_bytes\":{:.0},\
         \"private_median_bytes\":{:.0},\"private_p95_bytes\":{:.0}}}",
        frontend.name(),
        samples.len(),
        entry_to_app.median,
        entry_to_app.p95,
        app_to_activation.median,
        app_to_activation.p95,
        activation_to_frame.median,
        activation_to_frame.p95,
        entry_to_frame.median,
        entry_to_frame.p95,
        entry_to_frame_cpu.median,
        entry_to_frame_cpu.p95,
        process_to_report.median,
        process_to_report.p95,
        working_set.median,
        working_set.p95,
        private.median,
        private.p95,
    );
}

fn run_startup_gate() -> Result<(), String> {
    let runs = startup_argument("--startup-runs", 12)?;
    if runs < 10 {
        return Err("--startup-runs must be at least 10".to_string());
    }
    let mut reactor = Vec::with_capacity(runs);
    let mut reactor2 = Vec::with_capacity(runs);
    for run in 0..runs {
        let order = if run.is_multiple_of(2) {
            [Frontend::Reactor, Frontend::Reactor2]
        } else {
            [Frontend::Reactor2, Frontend::Reactor]
        };
        for frontend in order {
            let sample = run_startup_process(frontend)?;
            println!(
                "{{\"benchmark\":\"reactor-live-startup-raw\",\"run\":{},\
                 \"frontend\":\"{}\",\"entry_to_app_us\":{:.3},\
                 \"app_to_activation_us\":{:.3},\"activation_to_frame_us\":{:.3},\
                 \"entry_to_frame_us\":{:.3},\"entry_to_app_cpu_us\":{:.3},\
                 \"app_to_activation_cpu_us\":{:.3},\
                 \"activation_to_frame_cpu_us\":{:.3},\
                 \"entry_to_frame_cpu_us\":{:.3},\"process_to_report_us\":{:.3},\
                 \"working_set_bytes\":{:.0},\"private_bytes\":{:.0}}}",
                run + 1,
                sample.frontend.name(),
                sample.entry_to_app_us,
                sample.app_to_activation_us,
                sample.activation_to_frame_us,
                sample.entry_to_frame_us,
                sample.entry_to_app_cpu_us,
                sample.app_to_activation_cpu_us,
                sample.activation_to_frame_cpu_us,
                sample.entry_to_frame_cpu_us,
                sample.process_to_report_us,
                sample.working_set_bytes,
                sample.private_bytes,
            );
            match frontend {
                Frontend::Reactor => reactor.push(sample),
                Frontend::Reactor2 => reactor2.push(sample),
            }
        }
    }
    report_startup_summary(Frontend::Reactor, &reactor);
    report_startup_summary(Frontend::Reactor2, &reactor2);
    Ok(())
}

fn reverse_gate_metric(frontend: Frontend, count: usize) -> Result<f64, String> {
    let executable =
        env::current_exe().map_err(|error| format!("failed to locate benchmark: {error}"))?;
    let output = Command::new(executable)
        .args([
            "--frontend",
            frontend.name(),
            "--surface",
            "grid",
            "--workload",
            "reverse",
            "--count",
            &count.to_string(),
            "--updates",
            "60",
        ])
        .output()
        .map_err(|error| format!("failed to run {frontend:?} {count}: {error}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !output.status.success() {
        return Err(format!(
            "{frontend:?} {count} failed:\n{stdout}\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let benchmark = match frontend {
        Frontend::Reactor => "\"benchmark\":\"reactor-live-compare\"",
        Frontend::Reactor2 => "\"benchmark\":\"reactor2-live-phases\"",
    };
    let line = stdout
        .lines()
        .find(|line| line.contains(benchmark))
        .ok_or_else(|| format!("{frontend:?} {count} did not report native apply timing"))?;
    let field = "\"native_apply_avg_us\":";
    let start = line
        .find(field)
        .map(|index| index + field.len())
        .ok_or_else(|| format!("{frontend:?} {count} timing field is missing"))?;
    let end = line[start..]
        .find([',', '}'])
        .map_or(line.len(), |index| start + index);
    line[start..end]
        .parse()
        .map_err(|error| format!("{frontend:?} {count} timing is invalid: {error}"))
}

fn run_reverse_gate() -> Result<(), String> {
    let reactor_512 = reverse_gate_metric(Frontend::Reactor, 512)?;
    let reactor2_512 = reverse_gate_metric(Frontend::Reactor2, 512)?;
    let reactor_1024 = reverse_gate_metric(Frontend::Reactor, 1024)?;
    let reactor2_1024 = reverse_gate_metric(Frontend::Reactor2, 1024)?;
    let ratio_512 = reactor2_512 / reactor_512;
    let ratio_1024 = reactor2_1024 / reactor_1024;
    let scaling = reactor2_1024 / reactor2_512;
    println!(
        "{{\"benchmark\":\"reactor-live-reverse-gate\",\
         \"reactor_512_native_apply_avg_us\":{reactor_512:.3},\
         \"reactor2_512_native_apply_avg_us\":{reactor2_512:.3},\
         \"reactor_1024_native_apply_avg_us\":{reactor_1024:.3},\
         \"reactor2_1024_native_apply_avg_us\":{reactor2_1024:.3},\
         \"reactor2_to_reactor_512\":{ratio_512:.3},\
         \"reactor2_to_reactor_1024\":{ratio_1024:.3},\
         \"reactor2_scaling_1024_over_512\":{scaling:.3}}}"
    );
    if ratio_512 > 1.25 || ratio_1024 > 1.25 || scaling > 2.5 {
        return Err("reverse native apply ratio or scaling gate failed".to_string());
    }
    Ok(())
}

fn main() {
    let entry = Instant::now();
    let entry_cpu = process_cpu_time_100ns().unwrap();
    if env::args().any(|argument| argument == "--native-lifecycle-child") {
        let cycles = startup_argument("--lifecycle-cycles", 40).unwrap();
        let items = startup_argument("--lifecycle-items", 10_000).unwrap();
        let realized = startup_argument("--lifecycle-realized", 8).unwrap();
        if realized > items {
            eprintln!("reactor-live-compare: --lifecycle-realized cannot exceed --lifecycle-items");
            std::process::exit(2);
        }
        if let Err(error) = run_native_lifecycle_child(cycles, items, realized) {
            eprintln!("reactor-live-compare: {error}");
            std::process::exit(1);
        }
        return;
    }
    if env::args().any(|argument| argument == "--gate-native-lifecycle") {
        if let Err(error) = run_native_lifecycle_gate() {
            eprintln!("reactor-live-compare: {error}");
            std::process::exit(1);
        }
        return;
    }
    if env::args().any(|argument| argument == "--startup-child") {
        let args = env::args().collect::<Vec<_>>();
        let frontend = args
            .iter()
            .position(|argument| argument == "--frontend")
            .and_then(|index| args.get(index + 1))
            .and_then(|frontend| match frontend.as_str() {
                "reactor" => Some(Frontend::Reactor),
                "reactor2" => Some(Frontend::Reactor2),
                _ => None,
            });
        let Some(frontend) = frontend else {
            eprintln!("reactor-live-compare: startup child requires a valid --frontend");
            std::process::exit(2);
        };
        let result = match frontend {
            Frontend::Reactor => run_reactor_startup(entry, entry_cpu),
            Frontend::Reactor2 => run_reactor2_startup(entry, entry_cpu),
        };
        if let Err(error) = result {
            eprintln!("reactor-live-compare: {error}");
            std::process::exit(1);
        }
        return;
    }
    if env::args().any(|argument| argument == "--gate-startup") {
        if let Err(error) = run_startup_gate() {
            eprintln!("reactor-live-compare: {error}");
            std::process::exit(1);
        }
        return;
    }
    if env::args().any(|argument| argument == "--items-memory") {
        if let Err(error) = run_items_memory() {
            eprintln!("reactor-live-compare: {error}");
            std::process::exit(1);
        }
        return;
    }
    if env::args().any(|argument| argument == "--gate-reverse") {
        if let Err(error) = run_reverse_gate() {
            eprintln!("reactor-live-compare: {error}");
            std::process::exit(1);
        }
        return;
    }
    let options = match Options::parse() {
        Ok(Some(options)) => options,
        Ok(None) => return,
        Err(error) => {
            eprintln!("reactor-live-compare: {error}");
            std::process::exit(2);
        }
    };

    let result = match options.frontend {
        Frontend::Reactor => App::run_component::<ReactorFixture>(options),
        Frontend::Reactor2 => run_reactor2(options),
    };
    if let Err(error) = result {
        eprintln!("reactor-live-compare: {error}");
        std::process::exit(1);
    }
}
