use std::cell::Cell;
use std::env;
use std::time::{Duration, Instant};

use windows::Win32::{FILETIME, GetCurrentProcess, GetProcessTimes};
use windows::Win32::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX};
use windows_reactor::*;

mod allocator;

const COLUMNS: usize = 70;
const ROWS: usize = 70;
const TOTAL_CELLS: usize = COLUMNS * ROWS;
const CELL_WIDTH: f64 = 64.0;
const CELL_HEIGHT: f64 = 18.0;
const UPDATE_INTERVAL: Duration = Duration::from_millis(33);

#[derive(Clone, Debug, PartialEq)]
struct Options {
    percent: f64,
    duration: Duration,
    churn_count: usize,
    headless: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            percent: 10.0,
            duration: Duration::from_secs(10),
            churn_count: 0,
            headless: false,
        }
    }
}

impl Options {
    fn parse() -> Result<Option<Self>, String> {
        let mut options = Self::default();
        let mut args = env::args().skip(1);

        while let Some(argument) = args.next() {
            match argument.as_str() {
                "--percent" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--percent requires a value".to_string())?;
                    options.percent = value
                        .parse()
                        .map_err(|_| format!("invalid --percent value: {value}"))?;
                }
                "--duration" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--duration requires a value".to_string())?;
                    let seconds = value
                        .parse::<u64>()
                        .map_err(|_| format!("invalid --duration value: {value}"))?;
                    options.duration = Duration::from_secs(seconds);
                }
                "--churn-count" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--churn-count requires a value".to_string())?;
                    options.churn_count = value
                        .parse()
                        .map_err(|_| format!("invalid --churn-count value: {value}"))?;
                }
                "--headless" => options.headless = true,
                "--help" | "-h" => {
                    print_help();
                    return Ok(None);
                }
                _ => return Err(format!("unknown argument: {argument}")),
            }
        }

        if !options.percent.is_finite() || !(0.0..=100.0).contains(&options.percent) {
            return Err("--percent must be between 0 and 100".to_string());
        }
        if options.duration.is_zero() {
            return Err("--duration must be at least 1 second".to_string());
        }
        if options.churn_count > TOTAL_CELLS {
            return Err(format!("--churn-count must not exceed {TOTAL_CELLS}"));
        }

        Ok(Some(options))
    }
}

fn print_help() {
    println!(
        "reactor-live-grid\n\
         \n\
         Options:\n\
           --percent N       Dirty cells per update (default: 10)\n\
           --churn-count N   Trailing cells removed and restored per update (default: 0)\n\
           --duration N      Measured seconds (default: 10)\n\
           --headless        Start automatically for unattended runs\n\
           -h, --help        Print this help"
    );
}

#[derive(Clone, Debug, PartialEq)]
struct StockItem {
    symbol: String,
    current_price: f64,
    is_up: bool,
}

struct StockDataSource {
    items: Vec<StockItem>,
    random: SeededRandom,
}

impl StockDataSource {
    fn new() -> Self {
        let mut random = SeededRandom::new(42);
        let items = (0..TOTAL_CELLS)
            .map(|index| {
                let row = index / COLUMNS;
                let column = index % COLUMNS;
                let symbol = format!(
                    "{}{}{}",
                    char::from(b'A' + (row % 26) as u8),
                    char::from(b'A' + ((column / 3) % 26) as u8),
                    char::from(b'A' + (column % 26) as u8),
                );
                StockItem {
                    symbol,
                    current_price: round_price(10.0 + random.next_f64() * 990.0),
                    is_up: true,
                }
            })
            .collect();
        Self { items, random }
    }

    fn update(&mut self, percent: f64) -> Vec<usize> {
        let count = ((TOTAL_CELLS as f64 * percent / 100.0).ceil() as usize).min(TOTAL_CELLS);
        let mut dirty = Vec::with_capacity(count);
        for _ in 0..count {
            let index = self.random.next_usize(TOTAL_CELLS);
            let item = &mut self.items[index];
            let delta = (self.random.next_f64() - 0.48) * 2.0 * item.current_price * 0.02;
            let previous = item.current_price;
            item.current_price = round_price((previous + delta).max(0.01));
            item.is_up = item.current_price >= previous;
            dirty.push(index);
        }
        dirty.sort_unstable();
        dirty.dedup();
        dirty
    }
}

struct SeededRandom(u64);

impl SeededRandom {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        let mut value = self.0;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.0 = value;
        value
    }

    fn next_f64(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
    }

    fn next_usize(&mut self, maximum: usize) -> usize {
        (self.next_u64() % maximum as u64) as usize
    }
}

fn round_price(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn stock_cell(index: usize, item: &StockItem) -> TextBlock {
    TextBlock::new()
        .text(format!("{} {:.2}", item.symbol, item.current_price))
        .font_size(8.0)
        .foreground(Brush::Solid(if item.is_up {
            Color::rgb(0, 128, 0)
        } else {
            Color::rgb(255, 0, 0)
        }))
        .width(CELL_WIDTH)
        .height(CELL_HEIGHT)
        .grid_row((index / COLUMNS) as i32)
        .grid_column((index % COLUMNS) as i32)
}

#[derive(Default)]
struct MemoryStats {
    samples: u64,
    working_set_sum: u64,
    working_set_peak: u64,
    private_sum: u64,
    private_peak: u64,
    last_working_set: u64,
}

impl MemoryStats {
    fn sample(&mut self) {
        let (working_set, private) = process_memory_bytes().unwrap();
        self.samples += 1;
        self.working_set_sum = self.working_set_sum.saturating_add(working_set);
        self.working_set_peak = self.working_set_peak.max(working_set);
        self.private_sum = self.private_sum.saturating_add(private);
        self.private_peak = self.private_peak.max(private);
        self.last_working_set = working_set;
    }

    fn working_set_average(&self) -> u64 {
        self.working_set_sum / self.samples.max(1)
    }

    fn private_average(&self) -> u64 {
        self.private_sum / self.samples.max(1)
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

#[derive(Clone)]
enum Message {
    Start,
    Begin,
    Tick,
    Finish,
    Report {
        dispatch_times: Vec<f64>,
        native_times: Vec<f64>,
    },
}

struct PendingReport {
    elapsed: Duration,
    allocated_bytes: u64,
    cpu_time_100ns: u64,
}

struct LiveGrid {
    options: Options,
    source: StockDataSource,
    cells: Vec<TextBlock>,
    visible: usize,
    started: Option<Instant>,
    allocation_start: u64,
    cpu_start_100ns: u64,
    updates: u64,
    next_memory_sample: Duration,
    memory: MemoryStats,
    complete: bool,
    pending_report: Option<PendingReport>,
    begin_pending: std::rc::Rc<Cell<bool>>,
    report_pending: std::rc::Rc<Cell<bool>>,
    _rendering: LiveRendering,
}

impl LiveGrid {
    fn begin(&mut self, context: &ComponentContext<Self>) {
        if self.started.is_some() {
            return;
        }
        self.memory.sample();
        self.allocation_start = allocator::allocated_bytes();
        self.cpu_start_100ns = process_cpu_time_100ns().unwrap();
        self.started = Some(Instant::now());
        let duration = self.options.duration;
        context.spawn_background(move |_| {
            std::thread::sleep(duration);
            Message::Finish
        });
    }

    fn tick(&mut self) {
        let Some(started) = self.started else {
            return;
        };
        let dirty = self.source.update(self.options.percent);
        for index in dirty {
            self.cells[index] = stock_cell(index, &self.source.items[index]);
        }
        if self.options.churn_count != 0 {
            let reduced = TOTAL_CELLS - self.options.churn_count;
            self.visible = if self.visible == TOTAL_CELLS {
                reduced
            } else {
                TOTAL_CELLS
            };
        }
        self.updates += 1;

        let elapsed = started.elapsed();
        if elapsed >= self.next_memory_sample {
            self.memory.sample();
            self.next_memory_sample += Duration::from_secs(1);
        }
    }

    fn finish(&mut self) {
        if self.complete {
            return;
        }
        self.complete = true;
        self.memory.sample();
        let allocation_end = allocator::allocated_bytes();
        let allocated_bytes = allocation_end.saturating_sub(self.allocation_start);
        let elapsed = self.started.map_or(Duration::ZERO, |start| start.elapsed());
        let cpu_time_100ns = process_cpu_time_100ns()
            .unwrap()
            .saturating_sub(self.cpu_start_100ns);
        self.pending_report = Some(PendingReport {
            elapsed,
            allocated_bytes,
            cpu_time_100ns,
        });
        self.report_pending.set(true);
    }

    fn report(
        &mut self,
        context: &ComponentContext<Self>,
        mut dispatch_times: Vec<f64>,
        mut native_times: Vec<f64>,
    ) {
        let pending = self.pending_report.take().unwrap();
        let dispatch = distribution(&mut dispatch_times);
        let native = distribution(&mut native_times);
        let allocated_per_update = if self.updates == 0 {
            0.0
        } else {
            pending.allocated_bytes as f64 / self.updates as f64
        };
        let cpu_time_ms = pending.cpu_time_100ns as f64 / 10_000.0;
        let cpu_core_percent =
            pending.cpu_time_100ns as f64 / 10_000_000.0 / pending.elapsed.as_secs_f64() * 100.0;

        println!(
            "{{\"benchmark\":\"reactor-live-grid\",\"headless\":{},\
             \"dirty_percent\":{:.3},\"churn_count\":{},\"duration_ms\":{:.3},\
             \"updates\":{},\"rust_alloc_bytes\":{},\"rust_alloc_bytes_per_update\":{:.3},\
             \"cpu_time_ms\":{:.3},\"cpu_core_percent\":{:.3},\
             \"working_set_avg_bytes\":{},\"working_set_peak_bytes\":{},\
             \"private_avg_bytes\":{},\"private_peak_bytes\":{},\
             \"host_dispatch_samples\":{},\"host_dispatch_avg_us\":{:.3},\
             \"host_dispatch_p95_us\":{:.3},\"native_apply_samples\":{},\
             \"native_apply_avg_us\":{:.3},\"native_apply_p95_us\":{:.3}}}",
            self.options.headless,
            self.options.percent,
            self.options.churn_count,
            pending.elapsed.as_secs_f64() * 1_000.0,
            self.updates,
            pending.allocated_bytes,
            allocated_per_update,
            cpu_time_ms,
            cpu_core_percent,
            self.memory.working_set_average(),
            self.memory.working_set_peak,
            self.memory.private_average(),
            self.memory.private_peak,
            dispatch.count,
            dispatch.average,
            dispatch.p95,
            native.count,
            native.average,
            native.p95,
        );
        assert!(
            context.window().request_close(),
            "benchmark window close request was rejected"
        );
    }
}

impl Component for LiveGrid {
    type Input = Options;
    type Message = Message;

    fn create(options: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let source = StockDataSource::new();
        let cells = source
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| stock_cell(index, item))
            .collect();
        let sender = context.sender();
        let headless = options.headless;
        let begin_pending = std::rc::Rc::new(Cell::new(false));
        let begin_pending_for_rendering = std::rc::Rc::clone(&begin_pending);
        let report_pending = std::rc::Rc::new(Cell::new(false));
        let report_pending_for_rendering = std::rc::Rc::clone(&report_pending);
        let auto_started = Cell::new(false);
        let last_update = Cell::new(Instant::now());
        let rendering = subscribe_live_rendering(move || {
            if report_pending_for_rendering.replace(false) {
                let (dispatch_times, native_times) = take_live_performance_times();
                _ = sender.send(Message::Report {
                    dispatch_times,
                    native_times,
                });
                return;
            }
            if begin_pending_for_rendering.replace(false) {
                clear_live_performance_times();
                _ = sender.send(Message::Begin);
                return;
            }
            if headless && !auto_started.replace(true) {
                _ = sender.send(Message::Start);
                return;
            }
            let now = Instant::now();
            if now.duration_since(last_update.get()) >= UPDATE_INTERVAL {
                last_update.set(now);
                _ = sender.send(Message::Tick);
            }
        })
        .unwrap();

        Self {
            options: options.clone(),
            source,
            cells,
            visible: TOTAL_CELLS,
            started: None,
            allocation_start: 0,
            cpu_start_100ns: 0,
            updates: 0,
            next_memory_sample: Duration::from_secs(1),
            memory: MemoryStats::default(),
            complete: false,
            pending_report: None,
            begin_pending,
            report_pending,
            _rendering: rendering,
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            Message::Start if self.started.is_none() => self.begin_pending.set(true),
            Message::Begin => self.begin(context),
            Message::Tick if !self.complete => self.tick(),
            Message::Finish if self.started.is_some() => self.finish(),
            Message::Report {
                dispatch_times,
                native_times,
            } => self.report(context, dispatch_times, native_times),
            Message::Start | Message::Tick | Message::Finish => {}
        }
    }

    fn view(&self, _options: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let status = if self.started.is_some() {
            format!(
                "{} updates, {} MB working set",
                self.updates,
                self.memory.last_working_set / (1024 * 1024)
            )
        } else {
            "Ready".to_string()
        };
        let header = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(12.0)
            .children((
                Button::new()
                    .on_click(context.message(Message::Start))
                    .content(TextBlock::new().text("Start")),
                TextBlock::new().text(format!(
                    "70x70 stocks, {:.1}% dirty, {} churn",
                    self.options.percent, self.options.churn_count
                )),
                TextBlock::new().text(status),
            ));
        let cells = self.cells[..self.visible]
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, cell)| KeyedView::new(index, cell));
        let grid = Grid::new()
            .rows(std::iter::repeat_n(GridLength::Pixel(CELL_HEIGHT), ROWS))
            .columns(std::iter::repeat_n(GridLength::Pixel(CELL_WIDTH), COLUMNS))
            .keyed_children(cells);

        StackPanel::new()
            .spacing(8.0)
            .children((header, ScrollViewer::new().content(grid)))
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
    let p95_index = ((samples.len() - 1) as f64 * 0.95).ceil() as usize;
    Distribution {
        count: samples.len(),
        average,
        p95: samples[p95_index],
    }
}

fn main() {
    let options = match Options::parse() {
        Ok(Some(options)) => options,
        Ok(None) => return,
        Err(error) => {
            eprintln!("reactor-live-grid: {error}");
            std::process::exit(2);
        }
    };

    if let Err(error) = App::run_component::<LiveGrid>(options) {
        eprintln!("reactor-live-grid: {error}");
        std::process::exit(1);
    }
}
