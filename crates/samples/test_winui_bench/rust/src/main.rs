#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

mod bindings;

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use bindings::*;
use windows::Win32::{
    DispatchMessageW, FILETIME, GetCurrentProcess, GetMessageW, GetProcessMemoryInfo,
    GetProcessTimes, GetSystemTimePreciseAsFileTime, MSG, PROCESS_MEMORY_COUNTERS, PostQuitMessage,
    TranslateMessage,
};
use windows_collections::IVector;
use windows_core::{HSTRING, Interface, Result};
use windows_reactor::{App, DispatcherTimer, bootstrap, on_rendering};

static ALLOCATED: AtomicU64 = AtomicU64::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let result = unsafe { System.alloc(layout) };
        if !result.is_null() {
            ALLOCATED.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        result
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let result = unsafe { System.realloc(pointer, layout, size) };
        if !result.is_null() && size > layout.size() {
            ALLOCATED.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
        }
        result
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[derive(Clone, Copy)]
struct Metric {
    ns: f64,
    bytes: f64,
}

struct Sustained {
    start: Instant,
    allocation_start: Option<u64>,
    update_ns: u128,
    ticks: u64,
    frames: u64,
}

const STRESS_SIZE: u64 = 4_900;
const STRESS_COLUMNS: usize = 70;
const STRESS_CELL_WIDTH: f64 = 64.0;
const STRESS_CELL_HEIGHT: f64 = 18.0;

fn main() -> Result<()> {
    let startup = Instant::now();
    let main_ms = process_elapsed_ms(startup);
    let (
        iterations,
        create_iterations,
        tree_iterations,
        tree_size,
        stress_iterations,
        sustained_seconds,
        sustained_percent,
        sustained_churn,
        headless,
        settle_ms,
    ) = parse();
    bootstrap()?;
    let host_start_ms = process_elapsed_ms(startup);

    App::new().run_custom(move |_| {
        let startup_ms = process_elapsed_ms(startup);
        let text = TextBlock::new()?;
        text.SetText(&HSTRING::from("windows-rs"))?;
        let mut retained = TreeFixture::create(tree_size)?;
        let content: UIElement = retained.root.cast()?;
        let window = Window::new()?;
        window.SetTitle(&HSTRING::from("windows-rs WinUI benchmark"))?;
        window.SetContent(&content)?;
        if !headless {
            window.Activate()?;
        }
        let window_ms = process_elapsed_ms(startup);

        let create = measure(create_iterations, |_| {
            let item = TextBlock::new().unwrap();
            item.SetText(&HSTRING::from("item")).unwrap();
        });
        let even = HSTRING::from("even");
        let odd = HSTRING::from("odd");
        let update = measure(iterations, |i| {
            text.SetText(if i & 1 == 0 { &even } else { &odd }).unwrap();
        });
        let cast = measure(create_iterations, |_| {
            let item: UIElement = text.cast().unwrap();
            std::hint::black_box(item);
        });
        let tree = measure_tree(tree_iterations, tree_size);
        let batch_update = measure_batch_update(&retained, iterations);
        let churn = measure_churn(&mut retained, create_iterations);
        let teardown = measure_teardown(tree_iterations, tree_size);
        let button = Button::new()?;
        let button_base: ButtonBase = button.cast()?;
        let event_add_remove = measure(create_iterations, |_| {
            let revoker = button_base.Click(|_, _| {}).unwrap();
            drop(revoker);
        });
        let control: Control = button.cast()?;
        let boolean = measure(iterations, |i| {
            control.SetIsEnabled(i & 1 == 0).unwrap();
        });
        let base_working_set = working_set();
        drop(StressFixture::create(1)?);
        let stress_before = ALLOCATED.load(Ordering::Relaxed);
        let stress_start = Instant::now();
        let stress = StressFixture::create(STRESS_SIZE as usize)?;
        let stress_build = Metric {
            ns: stress_start.elapsed().as_nanos() as f64,
            bytes: (ALLOCATED.load(Ordering::Relaxed) - stress_before) as f64,
        };
        let stress_content: UIElement = stress.root.cast()?;
        window.SetContent(&stress_content)?;
        let stress0 = measure_stress(&stress, 0, stress_iterations);
        let stress10 = measure_stress(&stress, 10, stress_iterations);
        let stress50 = measure_stress(&stress, 50, stress_iterations);
        let stress100 = measure_stress(&stress, 100, stress_iterations);
        let stress_working_set = working_set();
        let sustained_stress = Rc::new(StressFixture::create(STRESS_SIZE as usize)?);
        let sustained_content: UIElement = sustained_stress.root.cast()?;
        window.SetContent(&sustained_content)?;
        drop(stress_content);
        drop(stress);

        let sustained = Rc::new(RefCell::new(Sustained {
            start: Instant::now(),
            allocation_start: None,
            update_ns: 0,
            ticks: 0,
            frames: 0,
        }));
        let sustained_running = Rc::new(Cell::new(true));
        let sustained_for_tick = sustained.clone();
        let running_for_tick = sustained_running.clone();
        let stress_for_tick = sustained_stress.clone();
        let even = HSTRING::from("even");
        let odd = HSTRING::from("odd");
        let sustained_count =
            (sustained_stress.text.len() * sustained_percent as usize / 100).max(1);
        let _tick = DispatcherTimer::new(Duration::from_millis(33), move || {
            if !running_for_tick.get() {
                return;
            }
            let render = {
                let mut state = sustained_for_tick.borrow_mut();
                if state.allocation_start.is_none() {
                    state.allocation_start = Some(ALLOCATED.load(Ordering::Relaxed));
                }
                state.ticks
            };
            let start = Instant::now();
            stress_for_tick.update_sustained(render, sustained_count, &even, &odd);
            stress_for_tick.reattach_tail(sustained_churn as usize);
            let elapsed = start.elapsed().as_nanos();
            let mut state = sustained_for_tick.borrow_mut();
            state.update_ns += elapsed;
            state.ticks += 1;
        })?;
        let sustained_for_frame = sustained.clone();
        let running_for_frame = sustained_running.clone();
        let _rendering = on_rendering(move || {
            if running_for_frame.get() {
                sustained_for_frame.borrow_mut().frames += 1;
            }
        })?;
        let sustained_for_finish = sustained;
        let running_for_finish = sustained_running;
        let _finish =
            DispatcherTimer::new_one_shot(Duration::from_secs(sustained_seconds), move || {
                running_for_finish.set(false);
                let allocation_end = ALLOCATED.load(Ordering::Relaxed);
                let state = sustained_for_finish.borrow();
                let seconds = state.start.elapsed().as_secs_f64();
                let update_ms = if state.ticks == 0 {
                    0.0
                } else {
                    state.update_ns as f64 / 1_000_000.0 / state.ticks as f64
                };
                let update_bytes = match (state.ticks, state.allocation_start) {
                    (0, _) | (_, None) => 0.0,
                    (ticks, Some(start)) => (allocation_end - start) as f64 / ticks as f64,
                };
                let fps = if seconds == 0.0 {
                    0.0
                } else {
                    state.frames as f64 / seconds
                };
                let sustained_working_set = working_set();
                println!(
                    "WINUI_BENCH_JSON {{\"consumer\":\"windows-rs\",\"mainMs\":{main_ms:.3},\
                     \"hostStartMs\":{host_start_ms:.3},\"startupMs\":{startup_ms:.3},\
                     \"windowMs\":{window_ms:.3},\"workingSet\":{},\"createNs\":{:.3},\
                     \"createBytes\":{:.3},\"updateNs\":{:.3},\"updateBytes\":{:.3},\
                     \"castNs\":{:.3},\"castBytes\":{:.3},\"treeNs\":{:.3},\
                     \"treeBytes\":{:.3},\"batchUpdateNs\":{:.3},\
                     \"batchUpdateBytes\":{:.3},\"churnNs\":{:.3},\"churnBytes\":{:.3},\
                     \"teardownNs\":{:.3},\"teardownBytes\":{:.3},\
                     \"eventNs\":{:.3},\"eventBytes\":{:.3},\
                     \"booleanNs\":{:.3},\"booleanBytes\":{:.3},\
                     \"stressBuildNs\":{:.3},\"stressBuildBytes\":{:.3},\
                     \"stress0Ms\":{:.3},\"stress0Bytes\":{:.3},\
                     \"stress10Ms\":{:.3},\"stress10Bytes\":{:.3},\
                     \"stress50Ms\":{:.3},\"stress50Bytes\":{:.3},\
                     \"stress100Ms\":{:.3},\"stress100Bytes\":{:.3},\
                     \"stressWorkingSet\":{stress_working_set},\
                     \"sustainedTicks\":{},\"sustainedUpdateMs\":{update_ms:.3},\
                     \"sustainedUpdateBytes\":{update_bytes:.3},\"sustainedFrames\":{},\
                     \"sustainedFps\":{fps:.3},\"sustainedChurn\":{sustained_churn},\
                     \"sustainedWorkingSet\":{sustained_working_set}}}",
                    base_working_set,
                    create.ns,
                    create.bytes,
                    update.ns,
                    update.bytes,
                    cast.ns,
                    cast.bytes,
                    tree.ns,
                    tree.bytes,
                    batch_update.ns,
                    batch_update.bytes,
                    churn.ns,
                    churn.bytes,
                    teardown.ns,
                    teardown.bytes,
                    event_add_remove.ns,
                    event_add_remove.bytes,
                    boolean.ns,
                    boolean.bytes,
                    stress_build.ns,
                    stress_build.bytes,
                    stress0.ns / 1_000_000.0,
                    stress0.bytes,
                    stress10.ns / 1_000_000.0,
                    stress10.bytes,
                    stress50.ns / 1_000_000.0,
                    stress50.bytes,
                    stress100.ns / 1_000_000.0,
                    stress100.bytes,
                    state.ticks,
                    state.frames,
                );
            })?;
        let quit_after = Duration::from_secs(sustained_seconds)
            + Duration::from_millis(if headless { 1 } else { settle_ms.max(1) });
        let _quit = DispatcherTimer::new_one_shot(quit_after, || unsafe { PostQuitMessage(0) })?;
        unsafe {
            let mut message = MSG::default();
            while GetMessageW(&mut message, None, 0, 0).0 > 0 {
                _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
        window.SetContent(&content)?;
        Ok(())
    })
}

fn measure(iterations: u64, mut action: impl FnMut(u64)) -> Metric {
    for i in 0..iterations.min(100) {
        action(i);
    }
    let before = ALLOCATED.load(Ordering::Relaxed);
    let start = Instant::now();
    for i in 0..iterations {
        action(i);
    }
    Metric {
        ns: start.elapsed().as_nanos() as f64 / iterations as f64,
        bytes: (ALLOCATED.load(Ordering::Relaxed) - before) as f64 / iterations as f64,
    }
}

fn working_set() -> u64 {
    unsafe {
        let mut counters = PROCESS_MEMORY_COUNTERS::default();
        if GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut counters,
            size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
        .as_bool()
        {
            counters.WorkingSetSize as u64
        } else {
            0
        }
    }
}

struct TreeFixture {
    root: StackPanel,
    children: IVector<UIElement>,
    text: Vec<TextBlock>,
    views: Vec<UIElement>,
}

impl TreeFixture {
    fn create(count: u64) -> Result<Self> {
        let root = StackPanel::new()?;
        let children: IVector<UIElement> = root.Children()?.cast()?;
        let mut text = Vec::with_capacity(count as usize);
        let mut views = Vec::with_capacity(count as usize);
        for i in 0..count {
            let item = TextBlock::new()?;
            item.SetText(&HSTRING::from(format!("Item {i}")))?;
            let child: UIElement = item.cast()?;
            children.Append(&child)?;
            text.push(item);
            views.push(child);
        }
        Ok(Self {
            root,
            children,
            text,
            views,
        })
    }

    fn update(&self, value: &HSTRING) {
        for item in &self.text {
            item.SetText(value).unwrap();
        }
    }

    fn reattach(&self) {
        self.children.Clear().unwrap();
        for child in &self.views {
            self.children.Append(child).unwrap();
        }
    }

    fn update_indices(&self, render: u64, count: usize, even: &HSTRING, odd: &HSTRING) {
        let value = if render & 1 == 0 { even } else { odd };
        let start = render as usize * 97 % self.text.len();
        for i in 0..count {
            self.text[(start + i * 17) % self.text.len()]
                .SetText(value)
                .unwrap();
        }
    }
}

impl Drop for TreeFixture {
    fn drop(&mut self) {
        let _ = self.children.Clear();
    }
}

struct StressFixture {
    root: ScrollViewer,
    _canvas: Canvas,
    children: IVector<UIElement>,
    text: Vec<TextBlock>,
    views: Vec<UIElement>,
    red: Brush,
    green: Brush,
}

impl StressFixture {
    fn create(count: usize) -> Result<Self> {
        let root = ScrollViewer::new()?;
        let canvas = Canvas::new()?;
        canvas.SetWidth(STRESS_COLUMNS as f64 * STRESS_CELL_WIDTH)?;
        let rows = count.div_ceil(STRESS_COLUMNS);
        canvas.SetHeight(rows as f64 * STRESS_CELL_HEIGHT)?;
        let children: IVector<UIElement> = canvas.Children()?.cast()?;
        let red = SolidColorBrush::new()?;
        red.SetColor(Color {
            A: 255,
            R: 220,
            G: 60,
            B: 60,
        })?;
        let red: Brush = red.cast()?;
        let green = SolidColorBrush::new()?;
        green.SetColor(Color {
            A: 255,
            R: 70,
            G: 210,
            B: 100,
        })?;
        let green: Brush = green.cast()?;
        let mut text = Vec::with_capacity(count);
        let mut views = Vec::with_capacity(count);

        for i in 0..count {
            let item = TextBlock::new()?;
            item.SetText(&HSTRING::from(format!("Item {i}")))?;
            item.SetWidth(STRESS_CELL_WIDTH)?;
            item.SetHeight(STRESS_CELL_HEIGHT)?;
            item.SetFontSize(12.0)?;
            item.SetForeground(if i & 1 == 0 { &green } else { &red })?;
            let child: UIElement = item.cast()?;
            Canvas::SetLeft(&child, (i % STRESS_COLUMNS) as f64 * STRESS_CELL_WIDTH)?;
            Canvas::SetTop(&child, (i / STRESS_COLUMNS) as f64 * STRESS_CELL_HEIGHT)?;
            children.Append(&child)?;
            text.push(item);
            views.push(child);
        }

        root.SetContent(&canvas)?;
        Ok(Self {
            root,
            _canvas: canvas,
            children,
            text,
            views,
            red,
            green,
        })
    }

    fn update_indices(&self, render: u64, count: usize, even: &HSTRING, odd: &HSTRING) {
        let value = if render & 1 == 0 { even } else { odd };
        let start = render as usize * 97 % self.text.len();
        for i in 0..count {
            self.text[(start + i * 17) % self.text.len()]
                .SetText(value)
                .unwrap();
        }
    }

    fn update_sustained(&self, render: u64, count: usize, even: &HSTRING, odd: &HSTRING) {
        let value = if render & 1 == 0 { even } else { odd };
        let start = render as usize * 97 % self.text.len();
        for i in 0..count {
            let item = &self.text[(start + i * 17) % self.text.len()];
            item.SetText(value).unwrap();
            item.SetForeground(if (render as usize + i) & 1 == 0 {
                &self.green
            } else {
                &self.red
            })
            .unwrap();
        }
    }

    fn reattach_tail(&self, count: usize) {
        let count = count.min(self.views.len());
        for _ in 0..count {
            self.children.RemoveAtEnd().unwrap();
        }
        for child in &self.views[self.views.len() - count..] {
            self.children.Append(child).unwrap();
        }
    }
}

impl Drop for StressFixture {
    fn drop(&mut self) {
        let _ = self.children.Clear();
    }
}

fn measure_tree(iterations: u64, size: u64) -> Metric {
    for _ in 0..iterations.min(10) {
        std::hint::black_box(TreeFixture::create(size).unwrap());
    }

    let mut trees = Vec::with_capacity(iterations as usize);
    let before = ALLOCATED.load(Ordering::Relaxed);
    let start = Instant::now();
    for _ in 0..iterations {
        trees.push(TreeFixture::create(size).unwrap());
    }
    let metric = Metric {
        ns: start.elapsed().as_nanos() as f64 / iterations as f64,
        bytes: (ALLOCATED.load(Ordering::Relaxed) - before) as f64 / iterations as f64,
    };
    drop(trees);
    metric
}

fn measure_batch_update(tree: &TreeFixture, target_updates: u64) -> Metric {
    let iterations = (target_updates / tree.text.len() as u64).max(1);
    let even = HSTRING::from("even");
    let odd = HSTRING::from("odd");
    for i in 0..iterations.min(10) {
        tree.update(if i & 1 == 0 { &even } else { &odd });
    }

    let before = ALLOCATED.load(Ordering::Relaxed);
    let start = Instant::now();
    for i in 0..iterations {
        tree.update(if i & 1 == 0 { &even } else { &odd });
    }
    let operations = iterations * tree.text.len() as u64;
    Metric {
        ns: start.elapsed().as_nanos() as f64 / operations as f64,
        bytes: (ALLOCATED.load(Ordering::Relaxed) - before) as f64 / operations as f64,
    }
}

fn measure_churn(tree: &mut TreeFixture, target_operations: u64) -> Metric {
    let iterations = (target_operations / tree.views.len() as u64).max(1);
    for _ in 0..iterations.min(10) {
        tree.reattach();
    }

    let before = ALLOCATED.load(Ordering::Relaxed);
    let start = Instant::now();
    for _ in 0..iterations {
        tree.reattach();
    }
    let operations = iterations * tree.views.len() as u64;
    Metric {
        ns: start.elapsed().as_nanos() as f64 / operations as f64,
        bytes: (ALLOCATED.load(Ordering::Relaxed) - before) as f64 / operations as f64,
    }
}

fn measure_teardown(iterations: u64, size: u64) -> Metric {
    let mut trees = Vec::with_capacity(iterations as usize);
    for _ in 0..iterations {
        trees.push(TreeFixture::create(size).unwrap());
    }

    let before = ALLOCATED.load(Ordering::Relaxed);
    let start = Instant::now();
    drop(trees);
    Metric {
        ns: start.elapsed().as_nanos() as f64 / iterations as f64,
        bytes: (ALLOCATED.load(Ordering::Relaxed) - before) as f64 / iterations as f64,
    }
}

fn measure_stress(tree: &StressFixture, percent: usize, iterations: u64) -> Metric {
    let count = (tree.text.len() * percent / 100).max(1);
    let even = HSTRING::from("even");
    let odd = HSTRING::from("odd");
    for render in 0..iterations.min(3) {
        tree.update_indices(render, count, &even, &odd);
    }

    let before = ALLOCATED.load(Ordering::Relaxed);
    let start = Instant::now();
    for render in 0..iterations {
        tree.update_indices(render, count, &even, &odd);
    }
    Metric {
        ns: start.elapsed().as_nanos() as f64 / iterations as f64,
        bytes: (ALLOCATED.load(Ordering::Relaxed) - before) as f64 / iterations as f64,
    }
}

fn parse() -> (u64, u64, u64, u64, u64, u64, u64, u64, bool, u64) {
    let args: Vec<String> = std::env::args().collect();
    let mut iterations = 100_000;
    let mut create_iterations = 1_000;
    let mut tree_iterations = 20;
    let mut tree_size = 100;
    let mut stress_iterations = 100;
    let mut sustained_seconds = 3;
    let mut sustained_percent = 10;
    let mut sustained_churn = 0;
    let mut settle_ms = 750;
    let headless = args.iter().any(|arg| arg == "--headless");
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--iterations" => iterations = pair[1].parse().unwrap_or(iterations),
            "--create-iterations" => {
                create_iterations = pair[1].parse().unwrap_or(create_iterations);
            }
            "--tree-iterations" => {
                tree_iterations = pair[1].parse().unwrap_or(tree_iterations);
            }
            "--tree-size" => tree_size = pair[1].parse().unwrap_or(tree_size),
            "--stress-iterations" => {
                if let Ok(value) = pair[1].parse::<u64>()
                    && value > 0
                {
                    stress_iterations = value;
                }
            }
            "--sustained-seconds" => {
                if let Ok(value) = pair[1].parse::<u64>()
                    && value > 0
                {
                    sustained_seconds = value;
                }
            }
            "--sustained-percent" => {
                sustained_percent = pair[1].parse().unwrap_or(sustained_percent).min(100);
            }
            "--sustained-churn" => {
                sustained_churn = pair[1].parse().unwrap_or(sustained_churn).min(STRESS_SIZE);
            }
            "--settle-ms" => settle_ms = pair[1].parse().unwrap_or(settle_ms),
            _ => {}
        }
    }
    (
        iterations,
        create_iterations,
        tree_iterations,
        tree_size,
        stress_iterations,
        sustained_seconds,
        sustained_percent,
        sustained_churn,
        headless,
        settle_ms,
    )
}

fn process_elapsed_ms(startup: Instant) -> f64 {
    unsafe {
        let mut created = FILETIME::default();
        let mut exited = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();
        if GetProcessTimes(
            GetCurrentProcess(),
            &mut created,
            &mut exited,
            &mut kernel,
            &mut user,
        )
        .as_bool()
        {
            let now = GetSystemTimePreciseAsFileTime();
            return (filetime_value(now) - filetime_value(created)) as f64 / 10_000.0;
        }
        startup.elapsed().as_secs_f64() * 1000.0
    }
}

fn filetime_value(value: FILETIME) -> u64 {
    u64::from(value.dwLowDateTime) | (u64::from(value.dwHighDateTime) << 32)
}
