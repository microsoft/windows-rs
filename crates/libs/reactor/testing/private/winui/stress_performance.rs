use std::cell::{Cell, RefCell};
use std::mem::size_of;
use std::rc::Rc;
use std::time::{Duration, Instant};

use windows::Win32::{
    GetCurrentProcess, GetProcessHandleCount, GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS,
    PROCESS_MEMORY_COUNTERS_EX,
};

use super::*;
use crate::performance::{
    DispatcherTimer, HostOptions, RenderMetrics, RenderingSubscription, request_exit, run_host,
};

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_STRESS_PERFORMANCE_FIXTURE";
const DURATION_ENV: &str = "WINDOWS_REACTOR_STRESS_PERFORMANCE_SECONDS";
const COLUMNS: usize = 70;
const ROWS: usize = 70;
const TOTAL: usize = COLUMNS * ROWS;
const CELL_WIDTH: f64 = 64.0;
const CELL_HEIGHT: f64 = 18.0;
const GREEN: Color = Color::rgb(0, 128, 0);
const RED: Color = Color::rgb(255, 0, 0);
const CELL_PADDING: Thickness = Thickness {
    left: 2.0,
    top: 1.0,
    right: 2.0,
    bottom: 1.0,
};

#[derive(Clone, PartialEq)]
struct CellView {
    text: String,
    color: Color,
}

struct StockItem {
    symbol: String,
    price: f64,
    is_up: bool,
}

struct StockDataSource {
    items: Vec<StockItem>,
    random: NetRandom,
}

impl StockDataSource {
    fn new() -> Self {
        let mut random = NetRandom::new(42);
        let items = (0..TOTAL)
            .map(|index| {
                let row = index / COLUMNS;
                let column = index % COLUMNS;
                let symbol = format!(
                    "{}{}{}",
                    (b'A' + (row % 26) as u8) as char,
                    (b'A' + ((column / 3) % 26) as u8) as char,
                    (b'A' + (column % 26) as u8) as char,
                );
                StockItem {
                    symbol,
                    price: round2(10.0 + random.next_double() * 990.0),
                    is_up: true,
                }
            })
            .collect();
        Self { items, random }
    }

    fn update(&mut self, percent: f64) -> Vec<usize> {
        let count = ((TOTAL as f64 * percent / 100.0) as usize).max(1);
        let mut dirty = Vec::with_capacity(count);
        for _ in 0..count {
            let index = self.random.next_max(TOTAL as i32) as usize;
            let item = &mut self.items[index];
            let previous = item.price;
            let delta = (self.random.next_double() - 0.48) * 2.0 * previous * 0.02;
            item.price = round2((previous + delta).max(0.01));
            item.is_up = item.price >= previous;
            dirty.push(index);
        }
        dirty.sort_unstable();
        dirty.dedup();
        dirty
    }

    fn view(&self, index: usize) -> CellView {
        let item = &self.items[index];
        CellView {
            text: format!("{} {:.2}", item.symbol, item.price),
            color: if item.is_up { GREEN } else { RED },
        }
    }
}

struct Harness {
    churn: bool,
    duration: Duration,
    source: RefCell<StockDataSource>,
    cells: RefCell<Vec<Option<State<CellView>>>>,
    visible: RefCell<Option<State<usize>>>,
    updates: Cell<u64>,
    frames: Cell<u64>,
    metrics: RefCell<Vec<RenderMetrics>>,
    process_metrics: RefCell<ProcessMetrics>,
    allocation_start: Cell<Option<u64>>,
    startup_started: Instant,
    startup_ms: Cell<Option<f64>>,
    started: Cell<Option<Instant>>,
    exit_requested: Cell<bool>,
}

impl Harness {
    fn new(churn: bool, duration: Duration) -> Self {
        Self {
            churn,
            duration,
            source: RefCell::new(StockDataSource::new()),
            cells: RefCell::new(vec![None; TOTAL]),
            visible: RefCell::new(None),
            updates: Cell::new(0),
            frames: Cell::new(0),
            metrics: RefCell::new(Vec::new()),
            process_metrics: RefCell::new(ProcessMetrics::default()),
            allocation_start: Cell::new(None),
            startup_started: Instant::now(),
            startup_ms: Cell::new(None),
            started: Cell::new(None),
            exit_requested: Cell::new(false),
        }
    }

    fn tick(&self) {
        let visible = self
            .visible
            .borrow()
            .as_ref()
            .and_then(State::try_value)
            .unwrap();
        let dirty = self.source.borrow_mut().update(10.0);
        let source = self.source.borrow();
        let cells = self.cells.borrow();
        for index in dirty {
            if index < visible
                && let Some(state) = cells[index].as_ref()
            {
                state.set(source.view(index));
            }
        }
        drop(cells);
        drop(source);

        if self.churn {
            assert!(
                self.visible
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .try_set(if visible == TOTAL { TOTAL - 400 } else { TOTAL })
            );
        }
        self.process_metrics.borrow_mut().sample();
        self.updates.set(self.updates.get() + 1);
    }

    fn record_metrics(&self, metrics: &RenderMetrics) {
        let mut samples = self.metrics.borrow_mut();
        if samples.is_empty() {
            self.startup_ms
                .set(Some(self.startup_started.elapsed().as_secs_f64() * 1_000.0));
            self.allocation_start
                .set(Some(crate::tests::benchmark::allocated_bytes()));
        }
        samples.push(*metrics);
    }
}

#[derive(Default)]
struct ProcessMetrics {
    samples: u64,
    working_set_sum: u128,
    working_set_peak: u64,
    private_bytes_sum: u128,
    private_bytes_peak: u64,
    handle_sum: u64,
    handle_peak: u32,
}

impl ProcessMetrics {
    fn sample(&mut self) {
        let (working_set, private_bytes, handles) = process_metrics();
        self.samples += 1;
        self.working_set_sum += u128::from(working_set);
        self.working_set_peak = self.working_set_peak.max(working_set);
        self.private_bytes_sum += u128::from(private_bytes);
        self.private_bytes_peak = self.private_bytes_peak.max(private_bytes);
        self.handle_sum += u64::from(handles);
        self.handle_peak = self.handle_peak.max(handles);
    }

    fn average_working_set(&self) -> u64 {
        (self.working_set_sum / u128::from(self.samples.max(1))) as u64
    }

    fn average_private_bytes(&self) -> u64 {
        (self.private_bytes_sum / u128::from(self.samples.max(1))) as u64
    }

    fn average_handles(&self) -> f64 {
        self.handle_sum as f64 / self.samples.max(1) as f64
    }
}

fn run_case(churn: bool) {
    let mode = if churn { "churn" } else { "update" };
    let output = test_reactor_support::run_test_process(
        "winui::tests::stress_performance::stress_performance_fixture",
        &[(FIXTURE_ENV, mode)],
        Duration::from_secs(60),
    )
    .unwrap();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Some(index) = line.find("REACTOR_PERF_JSON ") {
            println!("{}", &line[index..]);
        }
    }
    test_reactor_support::assert_success(output);
}

#[test]
#[ignore = "requires the Windows App Runtime and a release build"]
fn matched_stock_updates() {
    run_case(false);
}

#[test]
#[ignore = "requires the Windows App Runtime and a release build"]
fn matched_stock_churn() {
    run_case(true);
}

#[test]
fn stress_performance_fixture() {
    let Some(mode) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };
    run_fixture(mode == "churn");
}

fn run_fixture(churn: bool) {
    let duration = std::env::var(DURATION_ENV).map_or(Duration::from_secs(2), |value| {
        Duration::from_secs(value.parse().unwrap())
    });
    let harness = Rc::new(Harness::new(churn, duration));
    bootstrap().unwrap();
    let root = component({
        let harness = Rc::clone(&harness);
        move |cx| render_app(cx, &harness)
    });
    let metrics_harness = Rc::clone(&harness);

    run_host(
        "windows-reactor - stress_perf",
        root,
        HostOptions { fullscreen: true },
        move |metrics| metrics_harness.record_metrics(metrics),
    )
    .unwrap();

    let elapsed = harness.started.get().unwrap().elapsed().as_secs_f64();
    let metrics = harness.metrics.borrow();
    let steady_metrics = metrics.iter().skip(1).collect::<Vec<_>>();
    let updates = harness.updates.get();
    let renders = steady_metrics.len() as u64;
    let frames = harness.frames.get();
    let created = steady_metrics
        .iter()
        .map(|metrics| metrics.elements_created)
        .sum::<u64>();
    let allocated = crate::tests::benchmark::allocated_bytes()
        .saturating_sub(harness.allocation_start.get().unwrap());
    let process_metrics = harness.process_metrics.borrow();
    let average = |values: Vec<f64>| {
        if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        }
    };
    let average_reconcile = average(
        steady_metrics
            .iter()
            .map(|metrics| metrics.tree_build_ms + metrics.reconcile_ms + metrics.effects_ms)
            .collect(),
    );
    let average_diff = average(
        steady_metrics
            .iter()
            .map(|metrics| metrics.reconcile_ms)
            .collect(),
    );
    let allocated_per_render = allocated as f64 / renders.max(1) as f64;
    let average_memory_mb = process_metrics.average_working_set() as f64 / (1024.0 * 1024.0);
    let peak_memory_mb = process_metrics.working_set_peak as f64 / (1024.0 * 1024.0);
    let average_private_mb = process_metrics.average_private_bytes() as f64 / (1024.0 * 1024.0);
    let peak_private_mb = process_metrics.private_bytes_peak as f64 / (1024.0 * 1024.0);
    println!(
        "REACTOR_PERF_JSON {{\"app\":\"StressPerf.Reactor\",\"percent\":10,\
         \"startupMs\":{},\"durationSeconds\":{elapsed},\"rendersPerSec\":{},\
         \"totalRenders\":{renders},\
         \"avgReconcileMs\":{average_reconcile},\"avgDiffMs\":{average_diff},\
         \"avgMemoryMB\":{average_memory_mb},\"peakMemoryMB\":{peak_memory_mb},\
         \"avgPrivateMemoryMB\":{average_private_mb},\
         \"peakPrivateMemoryMB\":{peak_private_mb},\
         \"avgHandles\":{},\"peakHandles\":{},\
         \"allocBytesPerRender\":{allocated_per_render},\"gen0\":0,\"gen1\":0,\"gen2\":0,\
         \"gen0PerKRenders\":0,\"avgFps\":{},\"sampleCount\":1,\"churn\":{churn},\
         \"updates\":{updates},\"elementsCreated\":{created}}}",
        harness.startup_ms.get().unwrap(),
        renders as f64 / elapsed,
        process_metrics.average_handles(),
        process_metrics.handle_peak,
        frames as f64 / elapsed,
    );

    assert!(updates >= 10, "only {updates} update ticks completed");
    assert!(renders >= 10, "only {renders} renders completed");
    assert!(allocated > 0);
    assert!(process_metrics.working_set_peak > 0);
    assert!(process_metrics.private_bytes_peak > 0);
    assert!(process_metrics.handle_peak > 0);
    if churn {
        assert!(created >= 400, "churn created only {created} elements");
    }
}

fn render_app(cx: &mut RenderCx, harness: &Rc<Harness>) -> Element {
    let visible = cx.use_state(|| TOTAL);
    harness.visible.borrow_mut().replace(visible.clone());
    let visible_count = visible.try_value().unwrap();

    let update_timer = cx.use_ref(|| None::<DispatcherTimer>);
    let rendering = cx.use_ref(|| None::<RenderingSubscription>);
    let tick_harness = Rc::clone(harness);
    let frame_harness = Rc::clone(harness);
    let started_harness = Rc::clone(harness);
    cx.use_effect((), move || {
        started_harness.started.set(Some(Instant::now()));
        rendering.set(Some(
            RenderingSubscription::new(move || {
                frame_harness.frames.set(frame_harness.frames.get() + 1);
            })
            .unwrap(),
        ));
        update_timer.set(Some(
            DispatcherTimer::repeating(Duration::from_millis(33), move || {
                tick_harness.tick();
                if tick_harness.started.get().unwrap().elapsed() >= tick_harness.duration
                    && tick_harness.updates.get() >= 10
                    && !tick_harness.exit_requested.replace(true)
                {
                    request_exit().unwrap();
                }
            })
            .unwrap(),
        ));
    });

    let children = (0..visible_count).map(|index| {
        grid_child(stock_cell(Rc::clone(harness), index))
            .row((index / COLUMNS) as i32)
            .column((index % COLUMNS) as i32)
    });
    Grid::new(children)
        .columns(std::iter::repeat_n(GridLength::Pixel(CELL_WIDTH), COLUMNS))
        .rows(std::iter::repeat_n(GridLength::Pixel(CELL_HEIGHT), ROWS))
        .build()
}

fn stock_cell(harness: Rc<Harness>, index: usize) -> Element {
    memo_component(index, move |cx| {
        let state = cx.use_state(|| harness.source.borrow().view(index));
        harness.cells.borrow_mut()[index] = Some(state.clone());
        let view = state.try_value().unwrap();
        TextBlock::new(view.text)
            .padding(CELL_PADDING)
            .font_size(8.0)
            .foreground(view.color)
            .build()
    })
    .key(index as u64)
}

fn round2(value: f64) -> f64 {
    let scaled = value * 100.0;
    let floor = scaled.floor();
    let difference = scaled - floor;
    let rounded = if difference > 0.5 {
        floor + 1.0
    } else if difference < 0.5 || floor as i64 & 1 == 0 {
        floor
    } else {
        floor + 1.0
    };
    rounded / 100.0
}

struct NetRandom {
    seed: [i32; 56],
    next: i32,
    next_pair: i32,
}

impl NetRandom {
    fn new(value: i32) -> Self {
        let mut seed = [0; 56];
        let mut current = 161_803_398 - value.abs();
        seed[55] = current;
        let mut previous = 1;
        let mut index = 0;
        for _ in 1..55 {
            index = (index + 21) % 55;
            seed[index] = previous;
            previous = current.wrapping_sub(previous);
            if previous < 0 {
                previous = previous.wrapping_add(i32::MAX);
            }
            current = seed[index];
        }
        for _ in 1..5 {
            for index in 1..56 {
                let paired = 1 + (index + 30) % 55;
                seed[index] = seed[index].wrapping_sub(seed[paired]);
                if seed[index] < 0 {
                    seed[index] = seed[index].wrapping_add(i32::MAX);
                }
            }
        }
        Self {
            seed,
            next: 0,
            next_pair: 21,
        }
    }

    fn sample(&mut self) -> f64 {
        self.next += 1;
        if self.next >= 56 {
            self.next = 1;
        }
        self.next_pair += 1;
        if self.next_pair >= 56 {
            self.next_pair = 1;
        }
        let mut value =
            self.seed[self.next as usize].wrapping_sub(self.seed[self.next_pair as usize]);
        if value == i32::MAX {
            value -= 1;
        }
        if value < 0 {
            value = value.wrapping_add(i32::MAX);
        }
        self.seed[self.next as usize] = value;
        value as f64 / i32::MAX as f64
    }

    fn next_double(&mut self) -> f64 {
        self.sample()
    }

    fn next_max(&mut self, maximum: i32) -> i32 {
        (self.sample() * maximum as f64) as i32
    }
}

fn process_metrics() -> (u64, u64, u32) {
    unsafe {
        let mut counters = PROCESS_MEMORY_COUNTERS_EX::default();
        let mut handles = 0;
        let size = size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        let counters_base = std::ptr::from_mut(&mut counters).cast::<PROCESS_MEMORY_COUNTERS>();
        let process = GetCurrentProcess();
        let memory_ok = GetProcessMemoryInfo(process, counters_base, size).as_bool();
        let handles_ok = GetProcessHandleCount(process, &mut handles).as_bool();
        (
            if memory_ok {
                counters.WorkingSetSize as u64
            } else {
                0
            },
            if memory_ok {
                counters.PrivateUsage as u64
            } else {
                0
            },
            if handles_ok { handles } else { 0 },
        )
    }
}
