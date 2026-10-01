#![windows_subsystem = "windows"]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::{Cell, RefCell};
use std::fmt::Write;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use windows_reactor::*;
use windows_sys::Win32::{GetCurrentProcess, GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};

const APP_NAME: &str = "StressPerf.Reactor";
const COLUMNS: usize = 70;
const ROWS: usize = 70;
const TOTAL_ITEMS: usize = COLUMNS * ROWS;

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() && size > layout.size() {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            ALLOCATED_BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
        }
        pointer
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

#[derive(Clone, Copy, PartialEq)]
struct Options {
    duration: Duration,
    headless: bool,
    json: bool,
    percent: f64,
}

impl Options {
    fn parse() -> Self {
        let mut options = Self {
            duration: Duration::from_secs(10),
            headless: false,
            json: false,
            percent: 10.0,
        };
        let mut arguments = std::env::args().skip(1);
        while let Some(argument) = arguments.next() {
            match argument.as_str() {
                "--duration" => {
                    options.duration =
                        Duration::from_secs(arguments.next().unwrap().parse().unwrap());
                }
                "--headless" => options.headless = true,
                "--json" => options.json = true,
                "--percent" => options.percent = arguments.next().unwrap().parse().unwrap(),
                _ => panic!("unknown argument: {argument}"),
            }
        }
        assert!(
            options.percent.is_finite() && (0.0..=100.0).contains(&options.percent),
            "percent must be between 0 and 100"
        );
        assert!(!options.duration.is_zero(), "duration must be positive");
        options
    }
}

#[derive(Clone, Copy)]
struct StockItem {
    current_price: f64,
    is_up: bool,
    previous_price: f64,
    symbol: [u8; 3],
}

struct DotNetRandom {
    inext: usize,
    inextp: usize,
    seed_array: [i32; 56],
}

impl DotNetRandom {
    fn new(seed: i32) -> Self {
        const BIG: i32 = i32::MAX;
        const SEED: i32 = 161_803_398;

        let subtraction = if seed == i32::MIN {
            i32::MAX
        } else {
            seed.abs()
        };
        let mut mj = SEED - subtraction;
        let mut seed_array = [0; 56];
        seed_array[55] = mj;
        let mut mk = 1;
        for i in 1..55 {
            let index = (21 * i) % 55;
            seed_array[index] = mk;
            mk = mj - mk;
            if mk < 0 {
                mk += BIG;
            }
            mj = seed_array[index];
        }
        for _ in 0..4 {
            for i in 1..56 {
                seed_array[i] -= seed_array[1 + (i + 30) % 55];
                if seed_array[i] < 0 {
                    seed_array[i] += BIG;
                }
            }
        }
        Self {
            inext: 0,
            inextp: 21,
            seed_array,
        }
    }

    fn sample(&mut self) -> f64 {
        const BIG: i32 = i32::MAX;

        self.inext += 1;
        if self.inext >= 56 {
            self.inext = 1;
        }
        self.inextp += 1;
        if self.inextp >= 56 {
            self.inextp = 1;
        }
        let mut value = self.seed_array[self.inext] - self.seed_array[self.inextp];
        if value == BIG {
            value -= 1;
        }
        if value < 0 {
            value += BIG;
        }
        self.seed_array[self.inext] = value;
        f64::from(value) * (1.0 / f64::from(BIG))
    }

    fn next(&mut self, maximum: usize) -> usize {
        (self.sample() * maximum as f64) as usize
    }
}

struct StockDataSource {
    items: Vec<StockItem>,
    random: DotNetRandom,
}

impl StockDataSource {
    fn new() -> Self {
        let mut random = DotNetRandom::new(42);
        let items = (0..TOTAL_ITEMS)
            .map(|index| {
                let row = index / COLUMNS;
                let column = index % COLUMNS;
                let symbol = [
                    b'A' + (row % 26) as u8,
                    b'A' + ((column / 3) % 26) as u8,
                    b'A' + (column % 26) as u8,
                ];
                let price = round_cents(10.0 + random.sample() * 990.0);
                StockItem {
                    current_price: price,
                    is_up: true,
                    previous_price: price,
                    symbol,
                }
            })
            .collect();
        Self { items, random }
    }

    fn update(&mut self, percent: f64) -> Vec<usize> {
        let count = ((TOTAL_ITEMS as f64 * percent / 100.0) as usize).max(1);
        let mut changed = Vec::with_capacity(count);
        for _ in 0..count {
            let index = self.random.next(TOTAL_ITEMS);
            let item = &mut self.items[index];
            let delta = ((self.random.sample() - 0.48) * 2.0) * item.current_price * 0.02;
            let new_price = round_cents((item.current_price + delta).max(0.01));
            item.previous_price = item.current_price;
            item.current_price = new_price;
            item.is_up = new_price >= item.previous_price;
            changed.push(index);
        }
        changed.sort_unstable();
        changed.dedup();
        changed
    }

    fn snapshot(&self) -> Vec<StockItem> {
        self.items.clone()
    }
}

fn round_cents(value: f64) -> f64 {
    (value * 100.0).round_ties_even() / 100.0
}

struct PendingRender {
    allocation_start: u64,
    started: Instant,
    tree_allocated: u64,
    tree_ms: f64,
    update_allocated: u64,
    update_ms: f64,
}

#[derive(Default)]
struct Metrics {
    allocation_baseline: Option<(u64, u64, usize)>,
    diff_samples: Vec<f64>,
    diff_allocation_samples: Vec<u64>,
    fps_samples: Vec<f64>,
    frame_count: usize,
    total_frame_count: usize,
    last_frame_sample: Option<Instant>,
    memory_samples: Vec<u64>,
    pending: Option<PendingRender>,
    reconcile_samples: Vec<f64>,
    render_count: usize,
    started: Option<Instant>,
    tree_allocation_samples: Vec<u64>,
    tree_samples: Vec<f64>,
    update_allocation_samples: Vec<u64>,
    update_samples: Vec<f64>,
}

impl Metrics {
    fn start(&mut self) {
        let now = Instant::now();
        self.started = Some(now);
        self.last_frame_sample = Some(now);
    }

    fn frame_rendered(&mut self) {
        self.frame_count += 1;
        self.total_frame_count += 1;
        let now = Instant::now();
        let elapsed = now
            .duration_since(self.last_frame_sample.unwrap())
            .as_secs_f64();
        if elapsed >= 1.0 {
            self.fps_samples.push(self.frame_count as f64 / elapsed);
            self.memory_samples.push(working_set());
            self.frame_count = 0;
            self.last_frame_sample = Some(now);
        }
    }

    fn begin_render(
        &mut self,
        allocation_start: u64,
        started: Instant,
        update_allocated: u64,
        update_ms: f64,
    ) {
        self.pending = Some(PendingRender {
            allocation_start,
            started,
            tree_allocated: 0,
            tree_ms: 0.0,
            update_allocated,
            update_ms,
        });
    }

    fn tree_built(&mut self, tree_allocated: u64, tree_ms: f64) {
        if let Some(pending) = &mut self.pending {
            pending.tree_allocated = tree_allocated;
            pending.tree_ms = tree_ms;
        }
    }

    fn render_complete(&mut self) {
        let Some(pending) = self.pending.take() else {
            return;
        };
        let reconcile_ms = pending.started.elapsed().as_secs_f64() * 1_000.0 - pending.update_ms;
        let diff_ms = (reconcile_ms - pending.tree_ms).max(0.0);
        let total_allocated = ALLOCATED_BYTES.load(Ordering::Relaxed) - pending.allocation_start;
        let diff_allocated = total_allocated
            .saturating_sub(pending.update_allocated)
            .saturating_sub(pending.tree_allocated);
        self.update_allocation_samples
            .push(pending.update_allocated);
        self.tree_allocation_samples.push(pending.tree_allocated);
        self.diff_allocation_samples.push(diff_allocated);
        self.update_samples.push(pending.update_ms);
        self.tree_samples.push(pending.tree_ms);
        self.diff_samples.push(diff_ms);
        self.reconcile_samples.push(reconcile_ms);
        if self.allocation_baseline.is_none() {
            self.allocation_baseline = Some((
                ALLOCATED_BYTES.load(Ordering::Relaxed),
                ALLOCATIONS.load(Ordering::Relaxed),
                self.render_count,
            ));
        }
        self.render_count += 1;
    }

    fn report(&self, percent: f64) -> String {
        let duration = self.started.unwrap().elapsed().as_secs_f64();
        let average = |values: &[f64]| {
            if values.is_empty() {
                0.0
            } else {
                values.iter().sum::<f64>() / values.len() as f64
            }
        };
        let maximum = |values: &[f64]| values.iter().copied().fold(0.0, f64::max);
        let average_allocation = |values: &[u64]| {
            if values.is_empty() {
                0.0
            } else {
                values.iter().sum::<u64>() as f64 / values.len() as f64
            }
        };
        let fallback_fps = self.total_frame_count as f64 / duration;
        let fps_average = if self.fps_samples.is_empty() {
            fallback_fps
        } else {
            average(&self.fps_samples)
        };
        let fps_minimum = self
            .fps_samples
            .iter()
            .copied()
            .reduce(f64::min)
            .unwrap_or(fallback_fps);
        let fps_maximum = if self.fps_samples.is_empty() {
            fallback_fps
        } else {
            maximum(&self.fps_samples)
        };
        let memory_fallback = working_set();
        let memory_average = if self.memory_samples.is_empty() {
            memory_fallback as f64
        } else {
            self.memory_samples.iter().sum::<u64>() as f64 / self.memory_samples.len() as f64
        } / (1024.0 * 1024.0);
        let memory_peak = self
            .memory_samples
            .iter()
            .copied()
            .max()
            .unwrap_or(memory_fallback) as f64
            / (1024.0 * 1024.0);
        let (allocated_bytes, allocations) =
            if let Some((bytes, allocations, baseline_renders)) = self.allocation_baseline {
                let renders = self.render_count.saturating_sub(baseline_renders + 1);
                if renders == 0 {
                    (0.0, 0.0)
                } else {
                    (
                        (ALLOCATED_BYTES.load(Ordering::Relaxed) - bytes) as f64 / renders as f64,
                        (ALLOCATIONS.load(Ordering::Relaxed) - allocations) as f64 / renders as f64,
                    )
                }
            } else {
                (0.0, 0.0)
            };
        let combined = (self.update_samples.iter().sum::<f64>()
            + self.reconcile_samples.iter().sum::<f64>())
            / self.update_samples.len() as f64;

        format!(
            "=== {APP_NAME} ===\n\
             Duration:    {duration:.1}s\n\
             Percent:     {percent:.0}%\n\
             Avg FPS:     {:.1}\n\
             Min FPS:     {:.1}\n\
             Max FPS:     {:.1}\n\
             Avg Update:  {:.1} ms\n\
             Max Update:  {:.1} ms\n\
             Total Renders: {}\n\
             Renders/sec: {:.1}\n\
             Avg Reconcile: {:.1} ms\n\
             Max Reconcile: {:.1} ms\n\
               Avg Tree:    {:.1} ms\n\
               Avg Diff:    {:.1} ms\n\
             Avg Combined:  {:.1} ms  (renders/tick: 1.00)\n\
             Avg Memory:  {memory_average:.1} MB\n\
             Peak Memory: {memory_peak:.1} MB\n\
             Alloc/render: {allocated_bytes:.0} bytes\n\
             Allocs/render: {allocations:.1}\n\
             Alloc/update: {:.0} bytes\n\
             Alloc/tree: {:.0} bytes\n\
             Alloc/diff: {:.0} bytes\n\
             GC Gen0/1/2: 0 / 0 / 0\n\
             Gen0/Krender: 0.00\n",
            fps_average,
            fps_minimum,
            fps_maximum,
            average(&self.update_samples),
            maximum(&self.update_samples),
            self.render_count,
            self.render_count as f64 / duration,
            average(&self.reconcile_samples),
            maximum(&self.reconcile_samples),
            average(&self.tree_samples),
            average(&self.diff_samples),
            combined,
            average_allocation(&self.update_allocation_samples),
            average_allocation(&self.tree_allocation_samples),
            average_allocation(&self.diff_allocation_samples),
        )
    }

    fn json(&self, percent: f64) -> String {
        let average = |values: &[f64]| {
            if values.is_empty() {
                0.0
            } else {
                values.iter().sum::<f64>() / values.len() as f64
            }
        };
        let duration = self.started.unwrap().elapsed().as_secs_f64();
        let memory = if self.memory_samples.is_empty() {
            working_set() as f64
        } else {
            self.memory_samples.iter().sum::<u64>() as f64 / self.memory_samples.len() as f64
        } / (1024.0 * 1024.0);
        let fps = if self.fps_samples.is_empty() {
            self.total_frame_count as f64 / duration
        } else {
            average(&self.fps_samples)
        };
        format!(
            "{{\"app\":\"{APP_NAME}\",\"percent\":{percent:.4},\
             \"durationSeconds\":{duration:.4},\"rendersPerSec\":{:.4},\
             \"totalRenders\":{},\"avgReconcileMs\":{:.4},\
             \"avgDiffMs\":{:.4},\"avgMemoryMB\":{memory:.4},\
             \"avgFps\":{:.4},\"sampleCount\":{}}}",
            self.render_count as f64 / duration,
            self.render_count,
            average(&self.reconcile_samples),
            average(&self.diff_samples),
            fps,
            self.fps_samples.len(),
        )
    }
}

struct Message;

struct StockGrid {
    cells: Vec<View>,
    data: Vec<StockItem>,
    metrics: Rc<RefCell<Metrics>>,
    options: Options,
    _rendering: LiveRenderingSubscription,
    revision: u64,
    source: StockDataSource,
    _timer: LiveTickSubscription,
}

impl Component for StockGrid {
    type Input = Options;
    type Message = Message;

    fn create(options: &Options, context: &ComponentContext<Self>) -> Self {
        let source = StockDataSource::new();
        let metrics = Rc::new(RefCell::new(Metrics::default()));
        metrics.borrow_mut().start();
        let rendering = subscribe_live_rendering({
            let metrics = Rc::clone(&metrics);
            move || metrics.borrow_mut().frame_rendered()
        })
        .unwrap();
        let sender = context.sender();
        let started = Instant::now();
        let finished = Rc::new(Cell::new(false));
        let timer = subscribe_live_interval(Duration::from_millis(33), {
            let finished = Rc::clone(&finished);
            let duration = options.duration;
            let json = options.json;
            let metrics = Rc::clone(&metrics);
            let percent = options.percent;
            move || {
                if started.elapsed() >= duration {
                    if finished.replace(true) {
                        return;
                    }
                    let metrics = metrics.borrow();
                    std::fs::write(report_path("report.txt"), metrics.report(percent)).unwrap();
                    if json {
                        std::fs::write(report_path("metrics.json"), metrics.json(percent)).unwrap();
                    }
                    schedule_live_test_exit(true).unwrap();
                } else {
                    _ = sender.send(Message);
                }
            }
        })
        .unwrap();
        let data = source.snapshot();
        Self {
            cells: data.iter().enumerate().map(build_cell).collect(),
            data,
            metrics,
            options: *options,
            _rendering: rendering,
            revision: 0,
            source,
            _timer: timer,
        }
    }

    fn update(&mut self, _message: Message, _context: &ComponentContext<Self>) {
        let allocation_start = ALLOCATED_BYTES.load(Ordering::Relaxed);
        let started = Instant::now();
        let changed = self.source.update(self.options.percent);
        self.data = self.source.snapshot();
        for index in changed {
            self.cells[index] = build_cell((index, &self.data[index]));
        }
        self.revision = self.revision.checked_add(1).unwrap();
        let update_ms = started.elapsed().as_secs_f64() * 1_000.0;
        let update_allocated = ALLOCATED_BYTES.load(Ordering::Relaxed) - allocation_start;
        self.metrics.borrow_mut().begin_render(
            allocation_start,
            started,
            update_allocated,
            update_ms,
        );
    }

    fn view(&self, _options: &Options, context: &mut ViewContext<Self>) -> View {
        let allocation_start = ALLOCATED_BYTES.load(Ordering::Relaxed);
        let started = Instant::now();
        context.window_title("windows-reactor StocksGrid");
        context.window_visuals(WindowVisuals::new().client_size(1600.0, 900.0));

        let metrics = Rc::clone(&self.metrics);
        context.use_effect("record", self.revision, move || {
            metrics.borrow_mut().render_complete();
            None
        });

        let grid = Grid::new()
            .columns([GridLength::Pixel(64.0); COLUMNS])
            .rows([GridLength::Pixel(18.0); ROWS])
            .children(self.cells.clone());
        let view = Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(12.0)
                    .margin(8.0)
                    .children((
                        if self.options.headless {
                            "Headless"
                        } else {
                            "Interactive"
                        },
                        format!("Update: {:.0}%", self.options.percent),
                        format!("Renders: {}", self.metrics.borrow().render_count),
                    )),
                ScrollViewer::new().grid_row(1).content(grid),
            ))
            .into();
        self.metrics.borrow_mut().tree_built(
            ALLOCATED_BYTES.load(Ordering::Relaxed) - allocation_start,
            started.elapsed().as_secs_f64() * 1_000.0,
        );
        view
    }
}

fn build_cell((index, item): (usize, &StockItem)) -> View {
    let mut text = String::with_capacity(12);
    text.push(item.symbol[0] as char);
    text.push(item.symbol[1] as char);
    text.push(item.symbol[2] as char);
    write!(text, " {:.2}", item.current_price).unwrap();
    TextBlock::new()
        .text(text)
        .font_size(8.0)
        .foreground(if item.is_up {
            Color::rgb(0, 128, 0)
        } else {
            Color::rgb(255, 0, 0)
        })
        .padding(Thickness::new(2.0, 1.0, 2.0, 1.0))
        .grid_row((index / COLUMNS) as i32)
        .grid_column((index % COLUMNS) as i32)
        .into()
}

fn working_set() -> u64 {
    let mut counters = PROCESS_MEMORY_COUNTERS {
        cb: size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        ..Default::default()
    };
    let success = unsafe {
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut counters,
            size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
    };
    assert_ne!(success, 0, "GetProcessMemoryInfo failed");
    counters.WorkingSetSize as u64
}

fn report_path(extension: &str) -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("{APP_NAME}.{extension}"))
}

fn main() {
    let options = Options::parse();
    App::run_component_with_policy::<StockGrid>(
        options,
        WindowPolicy::new()
            .title("windows-reactor StocksGrid")
            .client_size(1600.0, 900.0),
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dotnet_random_seed_42_matches_reference_sequence() {
        let mut random = DotNetRandom::new(42);
        let actual = std::array::from_fn::<_, 5, _>(|_| random.next(1000));
        assert_eq!(actual, [668, 140, 125, 522, 168]);
    }

    #[test]
    fn zero_percent_still_changes_one_item() {
        let mut source = StockDataSource::new();
        let before = source.snapshot();
        let changed_indices = source.update(0.0);
        let changed = before
            .iter()
            .zip(&source.items)
            .filter(|(before, after)| before.current_price != after.current_price)
            .count();
        assert_eq!(changed_indices.len(), 1);
        assert_eq!(changed, 1);
    }

    #[test]
    fn short_run_report_does_not_require_periodic_samples() {
        let metrics = Metrics {
            diff_allocation_samples: vec![0],
            diff_samples: vec![0.0],
            reconcile_samples: vec![0.0],
            render_count: 1,
            started: Some(Instant::now()),
            total_frame_count: 1,
            tree_allocation_samples: vec![0],
            tree_samples: vec![0.0],
            update_allocation_samples: vec![0],
            update_samples: vec![0.0],
            ..Default::default()
        };
        assert!(metrics.report(0.0).contains("Avg FPS:"));
        assert!(metrics.json(0.0).contains("\"avgFps\":"));
    }
}
