use std::cell::{Cell, RefCell};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use windows::core::Result;
use windows::processthreadsapi::GetCurrentProcess;
use windows::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};

use windows_reactor::*;

const APP_NAME: &str = "StressPerf.Reactor";
const COLUMNS: usize = 80;
const ROWS: usize = 60;
const TOTAL: usize = COLUMNS * ROWS;
const CELL_WIDTH: f64 = 64.0;
const CELL_HEIGHT: f64 = 18.0;
const CELL_PAD: Thickness = Thickness {
    left: 2.0,
    top: 1.0,
    right: 2.0,
    bottom: 1.0,
};

#[derive(Clone, Debug)]
struct CliOptions {
    percent: f64,
    duration_seconds: u64,
    headless: bool,
}

impl Default for CliOptions {
    fn default() -> Self {
        Self {
            percent: 10.0,
            duration_seconds: 10,
            headless: false,
        }
    }
}

fn parse_cli() -> CliOptions {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut o = CliOptions::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--percent" if i + 1 < args.len() => {
                o.percent = args[i + 1].parse().unwrap_or(o.percent);
                i += 2;
            }
            "--duration" if i + 1 < args.len() => {
                o.duration_seconds = args[i + 1].parse().unwrap_or(o.duration_seconds);
                i += 2;
            }
            "--headless" => {
                o.headless = true;
                i += 1;
            }
            _ => i += 1,
        }
    }
    o
}

static CLI: OnceLock<CliOptions> = OnceLock::new();

#[derive(Clone, Debug, PartialEq)]
struct StockItem {
    symbol: String,
    prev_price: f64,
    current_price: f64,
    is_up: bool,
}

struct StockDataSource {
    items: Vec<StockItem>,
    rng: NetRandom,
}

impl StockDataSource {
    fn new() -> Self {
        let mut rng = NetRandom::new(42);
        let mut items = Vec::with_capacity(TOTAL);
        for i in 0..TOTAL {
            let row = i / COLUMNS;
            let col = i % COLUMNS;
            let c1 = (b'A' + (row % 26) as u8) as char;
            let c2 = (b'A' + ((col / 3) % 26) as u8) as char;
            let c3 = (b'A' + (col % 26) as u8) as char;
            let symbol = format!("{c1}{c2}{c3}");
            let price = round2(10.0 + rng.next_double() * 990.0);
            items.push(StockItem {
                symbol,
                prev_price: price,
                current_price: price,
                is_up: true,
            });
        }
        Self { items, rng }
    }

    fn update(&mut self, percent: f64) -> Vec<usize> {
        let count = (((TOTAL as f64) * percent / 100.0) as i32).max(1) as usize;
        let mut dirty = Vec::with_capacity(count);
        for _ in 0..count {
            let idx = self.rng.next_max(TOTAL as i32) as usize;
            let item = &mut self.items[idx];

            // +/- up to 2%, biased slightly upward.
            let delta = (self.rng.next_double() - 0.48) * 2.0 * item.current_price * 0.02;
            let new_price = round2((item.current_price + delta).max(0.01));
            let was = item.current_price;
            item.prev_price = was;
            item.current_price = new_price;
            item.is_up = new_price >= was;
            dirty.push(idx);
        }
        dirty.sort_unstable();
        dirty.dedup();
        dirty
    }

    fn item_at(&self, idx: usize) -> &StockItem {
        &self.items[idx]
    }
}

fn round2(v: f64) -> f64 {
    // Banker's rounding (round half to even).
    let scaled = v * 100.0;
    let floor = scaled.floor();
    let diff = scaled - floor;
    let rounded = if diff > 0.5 {
        floor + 1.0
    } else if diff < 0.5 {
        floor
    } else {
        // exact 0.5 — round half to even
        if (floor as i64) & 1 == 0 {
            floor
        } else {
            floor + 1.0
        }
    };
    rounded / 100.0
}

fn format_cell(item: &StockItem) -> String {
    format!("{} {:.2}", item.symbol, item.current_price)
}

/// Bit-identical port of .NET's seeded `System.Random` (CompatPrng — Knuth's
/// subtractive generator from Numerical Recipes via Mitchell-Moore). This
/// matches `new Random(42)` byte-for-byte across `NextDouble()` and
/// `Next(maxValue)` so the StressPerf.Reactor data sequence is identical
/// to the C# variant.
///
/// Reference: dotnet/runtime CompatPrng.cs (used by Random(int) on
/// .NET Framework, .NET Core/5+ when seeded).
struct NetRandom {
    seed_array: [i32; 56],
    inext: i32,
    inextp: i32,
}

const M_BIG: i32 = i32::MAX; // 2_147_483_647
const M_SEED: i32 = 161_803_398;

impl NetRandom {
    fn new(seed: i32) -> Self {
        let mut seed_array = [0i32; 56];
        let subtraction = if seed == i32::MIN {
            i32::MAX
        } else {
            seed.abs()
        };
        let mut mj = M_SEED - subtraction;
        seed_array[55] = mj;
        let mut mk: i32 = 1;
        let mut ii: usize = 0;
        for _i in 1..55 {
            ii += 21;
            if ii >= 55 {
                ii -= 55;
            }
            seed_array[ii] = mk;
            mk = mj.wrapping_sub(mk);
            if mk < 0 {
                mk = mk.wrapping_add(M_BIG);
            }
            mj = seed_array[ii];
        }
        for _k in 1..5 {
            for i in 1..56 {
                let mut n = i + 30;
                if n >= 55 {
                    n -= 55;
                }
                seed_array[i] = seed_array[i].wrapping_sub(seed_array[1 + n]);
                if seed_array[i] < 0 {
                    seed_array[i] = seed_array[i].wrapping_add(M_BIG);
                }
            }
        }
        Self {
            seed_array,
            inext: 0,
            inextp: 21,
        }
    }

    fn internal_sample(&mut self) -> i32 {
        let mut loc_inext = self.inext;
        let mut loc_inextp = self.inextp;
        loc_inext += 1;
        if loc_inext >= 56 {
            loc_inext = 1;
        }
        loc_inextp += 1;
        if loc_inextp >= 56 {
            loc_inextp = 1;
        }
        let mut ret =
            self.seed_array[loc_inext as usize].wrapping_sub(self.seed_array[loc_inextp as usize]);
        if ret == M_BIG {
            ret -= 1;
        }
        if ret < 0 {
            ret = ret.wrapping_add(M_BIG);
        }
        self.seed_array[loc_inext as usize] = ret;
        self.inext = loc_inext;
        self.inextp = loc_inextp;
        ret
    }

    fn sample(&mut self) -> f64 {
        self.internal_sample() as f64 * (1.0 / M_BIG as f64)
    }

    /// Equivalent to C# `Random.NextDouble()`.
    fn next_double(&mut self) -> f64 {
        self.sample()
    }

    /// Equivalent to C# `Random.Next(int maxValue)` for `maxValue > 0`.
    fn next_max(&mut self, max_value: i32) -> i32 {
        (self.sample() * max_value as f64) as i32
    }
}

struct PerfTracker {
    wall_clock: Instant,
    update_sw: Instant,
    last_update_ms: f64,
    frame_count: u32,
    last_sample_time: f64,
    current_fps: f64,
    fps_samples: Vec<f64>,
    memory_samples: Vec<u64>,
    update_time_samples: Vec<f64>,
    reconcile_time_samples: Vec<f64>,
    tree_build_samples: Vec<f64>,
    diff_patch_samples: Vec<f64>,
    effects_samples: Vec<f64>,
    /// Reconciler element counters (last render pass).
    last_elements_diffed: u64,
    last_elements_skipped: u64,
    last_elements_created: u64,
    /// Accumulated totals for averaging.
    total_elements_diffed: u64,
    total_elements_skipped: u64,
    total_elements_created: u64,
    /// Cross-variant render counter. See `tests/stress_perf/METHODOLOGY.md`
    /// on the C# branch — Reactor increments this when the reconcile
    /// completes (via `record_phases`). Emitted as `Total Renders: N`.
    render_count: u32,
}

impl PerfTracker {
    fn new() -> Self {
        Self {
            wall_clock: Instant::now(),
            update_sw: Instant::now(),
            last_update_ms: 0.0,
            frame_count: 0,
            last_sample_time: 0.0,
            current_fps: 0.0,
            fps_samples: Vec::new(),
            memory_samples: Vec::new(),
            update_time_samples: Vec::new(),
            reconcile_time_samples: Vec::new(),
            tree_build_samples: Vec::new(),
            diff_patch_samples: Vec::new(),
            effects_samples: Vec::new(),
            last_elements_diffed: 0,
            last_elements_skipped: 0,
            last_elements_created: 0,
            total_elements_diffed: 0,
            total_elements_skipped: 0,
            total_elements_created: 0,
            render_count: 0,
        }
    }

    fn frame_rendered(&mut self) {
        self.frame_count += 1;
        let now = self.wall_clock.elapsed().as_secs_f64();
        let elapsed = now - self.last_sample_time;
        if elapsed >= 1.0 {
            self.current_fps = self.frame_count as f64 / elapsed;
            self.fps_samples.push(self.current_fps);
            self.memory_samples.push(working_set_bytes());
            self.frame_count = 0;
            self.last_sample_time = now;
        }
    }

    fn begin_update(&mut self) {
        self.update_sw = Instant::now();
    }

    fn end_update(&mut self) {
        self.last_update_ms = self.update_sw.elapsed().as_secs_f64() * 1000.0;
        self.update_time_samples.push(self.last_update_ms);
    }

    fn record_phases(
        &mut self,
        tree_build_ms: f64,
        diff_patch_ms: f64,
        effects_ms: f64,
        elements_diffed: u64,
        elements_skipped: u64,
        elements_created: u64,
    ) {
        self.tree_build_samples.push(tree_build_ms);
        self.diff_patch_samples.push(diff_patch_ms);
        self.effects_samples.push(effects_ms);
        self.reconcile_time_samples
            .push(tree_build_ms + diff_patch_ms + effects_ms);
        self.last_elements_diffed = elements_diffed;
        self.last_elements_skipped = elements_skipped;
        self.last_elements_created = elements_created;
        self.total_elements_diffed += elements_diffed;
        self.total_elements_skipped += elements_skipped;
        self.total_elements_created += elements_created;
        // Reactor counts a render once the reconcile completes.
        self.render_count += 1;
    }

    fn current_memory_mb() -> u64 {
        working_set_bytes() / (1024 * 1024)
    }

    fn elapsed_seconds(&self) -> f64 {
        self.wall_clock.elapsed().as_secs_f64()
    }

    fn report(&self, app_name: &str, percent: f64) -> String {
        if self.fps_samples.is_empty() {
            return "No data collected.".into();
        }
        let avg = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
        let max = |v: &[f64]| v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let min = |v: &[f64]| v.iter().copied().fold(f64::INFINITY, f64::min);

        let elapsed = self.elapsed_seconds();
        let renders_per_sec = if elapsed > 0.0 {
            self.render_count as f64 / elapsed
        } else {
            0.0
        };

        let mut s = String::new();
        s.push_str(&format!("=== {app_name} ===\n"));
        s.push_str(&format!("Duration:    {elapsed:.1}s\n"));
        s.push_str(&format!("Percent:     {percent:.0}%\n"));
        s.push_str(&format!("Avg FPS:     {:.1}\n", avg(&self.fps_samples)));
        s.push_str(&format!("Min FPS:     {:.1}\n", min(&self.fps_samples)));
        s.push_str(&format!("Max FPS:     {:.1}\n", max(&self.fps_samples)));
        if !self.update_time_samples.is_empty() {
            s.push_str(&format!(
                "Avg Update:  {:.1} ms\n",
                avg(&self.update_time_samples)
            ));
            s.push_str(&format!(
                "Max Update:  {:.1} ms\n",
                max(&self.update_time_samples)
            ));
        }
        s.push_str(&format!("Total Renders: {}\n", self.render_count));
        s.push_str(&format!("Renders/sec: {renders_per_sec:.1}\n"));
        if !self.reconcile_time_samples.is_empty() {
            s.push_str(&format!(
                "Avg Reconcile: {:.1} ms\n",
                avg(&self.reconcile_time_samples)
            ));
            s.push_str(&format!(
                "Max Reconcile: {:.1} ms\n",
                max(&self.reconcile_time_samples)
            ));
        }
        if !self.tree_build_samples.is_empty() {
            s.push_str(&format!(
                "  Avg Tree:    {:.1} ms\n",
                avg(&self.tree_build_samples)
            ));
            s.push_str(&format!(
                "  Avg Diff:    {:.1} ms\n",
                avg(&self.diff_patch_samples)
            ));
            s.push_str(&format!(
                "  Avg Effects: {:.1} ms\n",
                avg(&self.effects_samples)
            ));
        }
        if !self.update_time_samples.is_empty() && !self.reconcile_time_samples.is_empty() {
            let ticks = self.update_time_samples.len() as f64;
            let combined = (self.update_time_samples.iter().sum::<f64>()
                + self.reconcile_time_samples.iter().sum::<f64>())
                / ticks;
            let renders_per_tick = self.reconcile_time_samples.len() as f64 / ticks;
            s.push_str(&format!(
                "Avg Combined:  {combined:.1} ms  (renders/tick: {renders_per_tick:.2})\n"
            ));
        }
        if self.render_count > 0 {
            let avg_diffed = self.total_elements_diffed as f64 / self.render_count as f64;
            let avg_skipped = self.total_elements_skipped as f64 / self.render_count as f64;
            let avg_created = self.total_elements_created as f64 / self.render_count as f64;
            s.push_str(&format!("Avg Elements Diffed:  {avg_diffed:.0}\n"));
            s.push_str(&format!("Avg Elements Skipped: {avg_skipped:.0}\n"));
            s.push_str(&format!("Avg Elements Created: {avg_created:.0}\n"));
        }
        let mem_avg = self.memory_samples.iter().copied().sum::<u64>() as f64
            / self.memory_samples.len() as f64;
        let mem_max = *self.memory_samples.iter().max().unwrap_or(&0) as f64;
        s.push_str(&format!(
            "Avg Memory:  {:.1} MB\n",
            mem_avg / (1024.0 * 1024.0)
        ));
        s.push_str(&format!(
            "Peak Memory: {:.1} MB\n",
            mem_max / (1024.0 * 1024.0)
        ));
        s
    }

    fn write_report_file(&self, app_name: &str, percent: f64) {
        let report = self.report(app_name, percent);

        eprint!("{report}");

        let dir = exe_dir();
        let report_path = dir.join(format!("{app_name}.report.txt"));
        if let Err(e) = fs::write(&report_path, &report) {
            eprintln!("WARNING: failed to write {}: {e}", report_path.display());
        }

        let mut csv = String::from("Second,FPS,Memory_MB\n");
        let n = self.fps_samples.len().min(self.memory_samples.len());
        for i in 0..n {
            let mb = self.memory_samples[i] as f64 / (1024.0 * 1024.0);
            csv.push_str(&format!("{},{:.2},{:.1}\n", i + 1, self.fps_samples[i], mb));
        }
        let csv_path = dir.join(format!("{app_name}.samples.csv"));
        if let Err(e) = fs::write(&csv_path, csv) {
            eprintln!("WARNING: failed to write {}: {e}", csv_path.display());
        }

        // Summary CSV — one row, machine-parseable, compatible with the C#
        // stress_perf benchmark_results.csv format for cross-comparison.
        let avg = |v: &[f64]| {
            if v.is_empty() {
                f64::NAN
            } else {
                v.iter().sum::<f64>() / v.len() as f64
            }
        };
        let elapsed = self.elapsed_seconds();
        let renders_per_sec = if elapsed > 0.0 {
            self.render_count as f64 / elapsed
        } else {
            0.0
        };
        let mem_avg = if self.memory_samples.is_empty() {
            0.0
        } else {
            self.memory_samples.iter().copied().sum::<u64>() as f64
                / self.memory_samples.len() as f64
                / (1024.0 * 1024.0)
        };
        let mem_peak = *self.memory_samples.iter().max().unwrap_or(&0) as f64 / (1024.0 * 1024.0);
        let summary = format!(
            "App,Percent,Duration_s,Avg_FPS,Min_FPS,Max_FPS,Avg_Update_ms,Max_Update_ms,\
             Avg_Reconcile_ms,Renders,Renders_per_s,Avg_Elements_Diffed,Avg_Elements_Skipped,\
             Avg_Elements_Created,Avg_Memory_MB,Peak_Memory_MB\n\
             {app_name},{percent:.0},{elapsed:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{},{renders_per_sec:.1},\
             {:.0},{:.0},{:.0},{mem_avg:.1},{mem_peak:.1}\n",
            avg(&self.fps_samples),
            self.fps_samples
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min),
            self.fps_samples
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max),
            avg(&self.update_time_samples),
            if self.update_time_samples.is_empty() {
                f64::NAN
            } else {
                self.update_time_samples
                    .iter()
                    .copied()
                    .fold(f64::NEG_INFINITY, f64::max)
            },
            avg(&self.reconcile_time_samples),
            self.render_count,
            if self.render_count > 0 {
                self.total_elements_diffed as f64 / self.render_count as f64
            } else {
                0.0
            },
            if self.render_count > 0 {
                self.total_elements_skipped as f64 / self.render_count as f64
            } else {
                0.0
            },
            if self.render_count > 0 {
                self.total_elements_created as f64 / self.render_count as f64
            } else {
                0.0
            },
        );
        let summary_path = dir.join(format!("{app_name}.summary.csv"));
        if let Err(e) = fs::write(&summary_path, summary) {
            eprintln!("WARNING: failed to write {}: {e}", summary_path.display());
        }
    }
}

fn working_set_bytes() -> u64 {
    unsafe {
        let mut counters = PROCESS_MEMORY_COUNTERS::default();
        let size = size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
        if GetProcessMemoryInfo(GetCurrentProcess(), &mut counters, size).as_bool() {
            counters.WorkingSetSize as u64
        } else {
            0
        }
    }
}

fn exe_dir() -> PathBuf {
    env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

struct State {
    source: StockDataSource,
    perf: PerfTracker,
    percent: Cell<f64>,
    pending_phases: Cell<bool>,
    /// Cached cell elements — only dirty indices are rebuilt each tick.
    cells: RefCell<Vec<Element>>,
    set_generation: RefCell<Option<SetState<u64>>>,
    set_fps: RefCell<Option<SetState<String>>>,
    set_upd: RefCell<Option<SetState<String>>>,
    set_mem: RefCell<Option<SetState<String>>>,
    generation: Cell<u64>,
}

impl State {
    fn new(initial_percent: f64) -> Self {
        let source = StockDataSource::new();
        let cells = build_all_cells(&source.items);
        Self {
            percent: Cell::new(initial_percent),
            source,
            perf: PerfTracker::new(),
            pending_phases: Cell::new(false),
            cells: RefCell::new(cells),
            set_generation: RefCell::new(None),
            set_fps: RefCell::new(None),
            set_upd: RefCell::new(None),
            set_mem: RefCell::new(None),
            generation: Cell::new(0),
        }
    }
}

fn data_tick(state: &Rc<RefCell<State>>) {
    let (set_generation, set_fps, set_upd, set_mem) = {
        let s = state.borrow();
        let g = s.set_generation.borrow().clone();
        let f = s.set_fps.borrow().clone();
        let u = s.set_upd.borrow().clone();
        let m = s.set_mem.borrow().clone();
        (g, f, u, m)
    };

    {
        let mut s = state.borrow_mut();
        s.perf.begin_update();
        let p = s.percent.get();
        let dirty = s.source.update(p);

        let green = Color::rgb(0, 128, 0);
        let red = Color::rgb(255, 0, 0);
        let mut cells = s.cells.borrow_mut();
        for idx in dirty {
            let item = s.source.item_at(idx);
            let r = (idx / COLUMNS) as i32;
            let c = (idx % COLUMNS) as i32;
            let brush = if item.is_up { green } else { red };
            cells[idx] = text_block(format_cell(item))
                .font_size(8.0)
                .foreground(brush)
                .padding(CELL_PAD)
                .grid_row(r)
                .grid_column(c)
                .into();
        }
        s.pending_phases.set(true);
        let generation = s.generation.get() + 1;
        s.generation.set(generation);
    };

    if let Some(setter) = set_generation {
        let generation = state.borrow().generation.get();
        setter.call(generation);
    }

    let (fps_label, upd_label, mem_label) = {
        let mut s = state.borrow_mut();
        s.perf.end_update();
        (
            format!("FPS: {:.0}", s.perf.current_fps),
            format!("Update: {:.1} ms", s.perf.last_update_ms),
            format!("Mem: {} MB", PerfTracker::current_memory_mb()),
        )
    };

    if let Some(setter) = set_fps {
        setter.call(fps_label);
    }
    if let Some(setter) = set_upd {
        setter.call(upd_label);
    }
    if let Some(setter) = set_mem {
        setter.call(mem_label);
    }
}

struct StockGridApp;

impl Component for StockGridApp {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let cli = CLI.get().cloned().unwrap_or_default();

        let (_generation, set_generation) = cx.use_state::<u64>(0);
        let (percent, set_percent) = cx.use_state(cli.percent);
        let (running, set_running) = cx.use_state(false);
        let (fps_lbl, set_fps_lbl) = cx.use_state(String::from("FPS: --"));
        let (upd_lbl, set_upd_lbl) = cx.use_state(String::from("Update: -- ms"));
        let (mem_lbl, set_mem_lbl) = cx.use_state(String::from("Mem: -- MB"));

        let state_ref = cx.use_ref::<Option<Rc<RefCell<State>>>>(None);
        if state_ref.borrow().is_none() {
            let s = State::new(cli.percent);
            state_ref.set(Some(Rc::new(RefCell::new(s))));
        }
        let state: Rc<RefCell<State>> = state_ref.borrow().clone().unwrap();

        {
            let s = state.borrow();
            *s.set_generation.borrow_mut() = Some(set_generation);
            *s.set_fps.borrow_mut() = Some(set_fps_lbl);
            *s.set_upd.borrow_mut() = Some(set_upd_lbl);
            *s.set_mem.borrow_mut() = Some(set_mem_lbl);
            s.percent.set(percent);
        }

        let rendering_ref = cx.use_ref::<Option<Rendering>>(None);
        if rendering_ref.borrow().is_none() {
            let state_for_rendering = state.clone();
            let hook = on_rendering(move || {
                state_for_rendering.borrow_mut().perf.frame_rendered();
            })
            .expect("CompositionTarget.Rendering subscribe");
            rendering_ref.set(Some(hook));
        }

        let phases_installed = cx.use_ref::<bool>(false);
        if !*phases_installed.borrow() {
            phases_installed.set(true);
            let state_for_phases = state.clone();
            with_active_host(|host| {
                host.set_render_complete(move |info: &RenderCompleteInfo| {
                    let mut s = state_for_phases.borrow_mut();
                    if s.pending_phases.get() {
                        s.pending_phases.set(false);
                        s.perf.record_phases(
                            info.tree_build_ms,
                            info.reconcile_ms,
                            info.effects_ms,
                            info.elements_diffed,
                            info.elements_skipped,
                            info.elements_created,
                        );
                    }
                });
            });
        }

        let timer_ref = cx.use_ref::<Option<DispatcherTimer>>(None);
        let timer_ref_for_effect = timer_ref;
        let state_for_effect = state.clone();
        cx.use_effect((running,), move || {
            if running {
                let state_for_tick = state_for_effect.clone();
                let timer = DispatcherTimer::new(Duration::from_millis(33), move || {
                    data_tick(&state_for_tick);
                })
                .expect("DispatcherQueue.CreateTimer");
                timer_ref_for_effect.set(Some(timer));
            } else {
                timer_ref_for_effect.set(None);
            }
        });

        let shutdown_ref = cx.use_ref::<Option<DispatcherTimer>>(None);
        let cli_for_autostart = cli;
        let set_running_for_autostart = set_running.clone();
        let state_for_autostart = state.clone();
        let shutdown_ref_for_autostart = shutdown_ref;
        cx.use_effect((), move || {
            if cli_for_autostart.headless {
                set_running_for_autostart.call(true);
                let state_for_shutdown = state_for_autostart.clone();
                let app_name = APP_NAME;
                let percent = cli_for_autostart.percent;
                let timer = DispatcherTimer::new_one_shot(
                    Duration::from_secs(cli_for_autostart.duration_seconds),
                    move || {
                        state_for_shutdown
                            .borrow()
                            .perf
                            .write_report_file(app_name, percent);
                        // Flush stderr so the report printed by
                        // write_report_file is visible even though
                        // process::exit skips destructors/drop.
                        let _ = std::io::stderr().flush();
                        std::process::exit(0);
                    },
                )
                .expect("DispatcherQueue.CreateTimer (one-shot)");
                shutdown_ref_for_autostart.set(Some(timer));
            }
        });

        let toggle_running = {
            let cur = running;
            let set = set_running;
            move || set.call(!cur)
        };

        let header = hstack((
            button(if running { "Stop" } else { "Start" }).on_click(toggle_running),
            text_block("Update %:").vertical_alignment(VerticalAlignment::Center),
            Slider::new(percent)
                .range(0.0, 100.0)
                .on_value_changed(move |v| set_percent.call(v))
                .width(200.0),
            text_block(fps_lbl)
                .vertical_alignment(VerticalAlignment::Center)
                .width(100.0),
            text_block(upd_lbl)
                .vertical_alignment(VerticalAlignment::Center)
                .width(120.0),
            text_block(mem_lbl)
                .vertical_alignment(VerticalAlignment::Center)
                .width(120.0),
        ))
        .spacing(12.0)
        .padding(Thickness::uniform(8.0));

        let cells = state.borrow().cells.borrow().clone();
        let game_grid = grid(cells)
            .columns(std::iter::repeat_n(GridLength::Pixel(CELL_WIDTH), COLUMNS))
            .rows(std::iter::repeat_n(GridLength::Pixel(CELL_HEIGHT), ROWS));

        vstack((header, scroll_viewer(game_grid))).into()
    }
}

fn build_all_cells(data: &[StockItem]) -> Vec<Element> {
    let green = Color::rgb(0, 128, 0);
    let red = Color::rgb(255, 0, 0);

    if data.is_empty() {
        return Vec::new();
    }

    let mut cells: Vec<Element> = Vec::with_capacity(TOTAL);
    for (i, item) in data.iter().enumerate().take(TOTAL) {
        let r = (i / COLUMNS) as i32;
        let c = (i % COLUMNS) as i32;
        let brush = if item.is_up { green } else { red };
        cells.push(
            text_block(format_cell(item))
                .font_size(8.0)
                .foreground(brush)
                .padding(CELL_PAD)
                .grid_row(r)
                .grid_column(c)
                .into(),
        );
    }
    cells
}

fn main() -> Result<()> {
    bootstrap()?;
    let cli = parse_cli();
    let _ = CLI.set(cli);
    App::new()
        .title("windows_reactor — stress_perf")
        .fullscreen(true)
        .run(|| StockGridApp)
}
