#![windows_subsystem = "console"]

use std::cell::{Cell, RefCell};
use std::env;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::time::Instant;

use windows::Win32::{FILETIME, GetCurrentProcess, GetProcessTimes};
use windows::Win32::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX};
use windows_core::EventRevoker;
use windows_reactor::test::{
    clear_live_performance_times, subscribe_live_rendering, take_live_performance_times,
};
use windows_reactor::{
    App, AppProxy, ChildrenControl as _, Component, ComponentContext, Grid, KeyedView, ListView,
    TextBlock, View, ViewContext,
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
    p95: f64,
}

fn distribution(samples: &mut [f64]) -> Distribution {
    if samples.is_empty() {
        return Distribution {
            count: 0,
            average: 0.0,
            p95: 0.0,
        };
    }
    samples.sort_by(f64::total_cmp);
    let average = samples.iter().sum::<f64>() / samples.len() as f64;
    let p95 = samples[((samples.len() - 1) as f64 * 0.95).ceil() as usize];
    Distribution {
        count: samples.len(),
        average,
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

fn main() {
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
