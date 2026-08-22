use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt::Write;
use std::mem::size_of;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use test_reactor_matched_bench::{OPERATIONS, Operation};
use windows::Win32::GetCurrentProcess;
use windows::Win32::{GetActiveWindow, GetClientRect, RECT};
use windows::Win32::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX};
use windows::Win32::{SW_MAXIMIZE, ShowWindow};

const WARMUP_UPDATES: usize = 16;
const SETTLE_FRAMES: usize = 8;

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
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
            let growth = (size - layout.size()) as u64;
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            ALLOCATED_BYTES.fetch_add(growth, Ordering::Relaxed);
        }
        pointer
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

pub enum FrameAction {
    Apply {
        iteration: usize,
        operation: Operation,
        start_measurement: bool,
    },
    Finish,
    Settle,
}

pub struct LiveTracker {
    samples: usize,
    iteration: usize,
    settle_frames: usize,
    last_update_frame: Option<Instant>,
    frame_ms: Vec<f64>,
    allocation_start: u64,
    byte_start: u64,
    working_start: u64,
    private_start: u64,
    working_peak: u64,
    private_peak: u64,
    client_start: (i32, i32),
    tree_build_us: Vec<f64>,
    reconcile_us: Vec<f64>,
    effects_us: Vec<f64>,
}

impl LiveTracker {
    pub fn new(samples: usize) -> Self {
        assert!(samples > 0);
        Self {
            samples,
            iteration: 0,
            settle_frames: 0,
            last_update_frame: None,
            frame_ms: Vec::with_capacity(samples),
            allocation_start: 0,
            byte_start: 0,
            working_start: 0,
            private_start: 0,
            working_peak: 0,
            private_peak: 0,
            client_start: (0, 0),
            tree_build_us: Vec::with_capacity(samples),
            reconcile_us: Vec::with_capacity(samples),
            effects_us: Vec::with_capacity(samples),
        }
    }

    pub fn on_frame(&mut self) -> FrameAction {
        let now = Instant::now();
        if let Some(last) = self.last_update_frame.take()
            && self.iteration > WARMUP_UPDATES
        {
            self.frame_ms
                .push(now.duration_since(last).as_secs_f64() * 1_000.0);
        }

        let total = WARMUP_UPDATES + self.samples;
        if self.iteration < total {
            let start_measurement = self.iteration == WARMUP_UPDATES;
            if self.iteration >= WARMUP_UPDATES {
                let (working, private) = process_memory_bytes();
                self.working_peak = self.working_peak.max(working);
                self.private_peak = self.private_peak.max(private);
            }
            let logical_iteration = self.iteration;
            let operation = OPERATIONS[logical_iteration % OPERATIONS.len()].1;
            self.iteration += 1;
            self.last_update_frame = Some(now);
            FrameAction::Apply {
                iteration: logical_iteration,
                operation,
                start_measurement,
            }
        } else if self.settle_frames < SETTLE_FRAMES {
            self.settle_frames += 1;
            FrameAction::Settle
        } else {
            FrameAction::Finish
        }
    }

    pub fn begin_measurement(&mut self) {
        self.allocation_start = ALLOCATIONS.load(Ordering::Relaxed);
        self.byte_start = ALLOCATED_BYTES.load(Ordering::Relaxed);
        (self.working_start, self.private_start) = process_memory_bytes();
        self.working_peak = self.working_start;
        self.private_peak = self.private_start;
        self.client_start = active_client_size();
    }

    #[allow(dead_code)]
    pub fn measurement_active(&self) -> bool {
        self.iteration > WARMUP_UPDATES
            && self.iteration <= WARMUP_UPDATES + self.samples
            && self.settle_frames == 0
    }

    #[allow(dead_code)]
    pub fn record_incumbent_phases(
        &mut self,
        tree_build_ms: f64,
        reconcile_ms: f64,
        effects_ms: f64,
    ) {
        if self.measurement_active() {
            self.tree_build_us.push(tree_build_ms * 1_000.0);
            self.reconcile_us.push(reconcile_ms * 1_000.0);
            self.effects_us.push(effects_ms * 1_000.0);
        }
    }

    pub fn report(
        &self,
        frontend: &str,
        host_dispatch_us: &mut [f64],
        native_apply_us: &mut [f64],
    ) -> String {
        let allocations = ALLOCATIONS.load(Ordering::Relaxed) - self.allocation_start;
        let bytes = ALLOCATED_BYTES.load(Ordering::Relaxed) - self.byte_start;
        let (working_end, private_end) = process_memory_bytes();
        let client_end = active_client_size();
        let mut output = format!(
            "matched 32-task live WinUI\n\
             frontend: {frontend}\n\
             measured updates: {}\n\
             allocations/update: {:.1}\n\
             bytes/update: {:.0}\n\
             working set: start {} end {} peak {} bytes\n\
             private bytes: start {} end {} peak {} bytes\n",
            self.samples,
            allocations as f64 / self.samples as f64,
            bytes as f64 / self.samples as f64,
            self.working_start,
            working_end,
            self.working_peak.max(working_end),
            self.private_start,
            private_end,
            self.private_peak.max(private_end),
        );
        writeln!(
            output,
            "client size: start {}x{} end {}x{} pixels",
            self.client_start.0, self.client_start.1, client_end.0, client_end.1,
        )
        .unwrap();
        distribution(&mut output, "frame interval", &self.frame_ms, "ms");
        distribution(&mut output, "tree build", &self.tree_build_us, "us");
        distribution(&mut output, "reconcile", &self.reconcile_us, "us");
        distribution(&mut output, "effects", &self.effects_us, "us");
        distribution(&mut output, "host dispatch", host_dispatch_us, "us");
        distribution(&mut output, "native apply", native_apply_us, "us");
        let over_25 = self.frame_ms.iter().filter(|value| **value > 25.0).count();
        let over_two_frames = self.frame_ms.iter().filter(|value| **value > 33.4).count();
        writeln!(
            output,
            "frame misses: >25 ms {over_25}, >33.4 ms {over_two_frames}"
        )
        .unwrap();
        output
    }
}

fn distribution(output: &mut String, name: &str, values: &[f64], unit: &str) {
    if values.is_empty() {
        writeln!(output, "{name}: no samples").unwrap();
        return;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let percentile = |fraction: f64| {
        let index = ((sorted.len() - 1) as f64 * fraction).ceil() as usize;
        sorted[index]
    };
    writeln!(
        output,
        "{name}: {} samples, median {:.2} {unit}, p95 {:.2} {unit}, \
         p99 {:.2} {unit}, max {:.2} {unit}",
        sorted.len(),
        percentile(0.50),
        percentile(0.95),
        percentile(0.99),
        sorted.last().unwrap(),
    )
    .unwrap();
}

pub fn maximize_active_window() {
    unsafe {
        _ = ShowWindow(GetActiveWindow(), SW_MAXIMIZE);
    }
}

fn active_client_size() -> (i32, i32) {
    unsafe {
        let mut rect = RECT::default();
        if GetClientRect(GetActiveWindow(), &mut rect).as_bool() {
            (rect.right - rect.left, rect.bottom - rect.top)
        } else {
            (0, 0)
        }
    }
}

fn process_memory_bytes() -> (u64, u64) {
    unsafe {
        let mut counters = PROCESS_MEMORY_COUNTERS_EX::default();
        let size = size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        let base = std::ptr::from_mut(&mut counters).cast::<PROCESS_MEMORY_COUNTERS>();
        if GetProcessMemoryInfo(GetCurrentProcess(), base, size).as_bool() {
            (counters.WorkingSetSize as u64, counters.PrivateUsage as u64)
        } else {
            (0, 0)
        }
    }
}

pub fn samples() -> usize {
    std::env::args()
        .skip_while(|argument| argument != "--samples")
        .nth(1)
        .map_or(500, |value| value.parse::<usize>().unwrap())
}

pub fn write_report(frontend: &str, report: &str) {
    print!("{report}");
    let path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("reactor-matched-live-{frontend}.txt"));
    std::fs::write(path, report).unwrap();
}
