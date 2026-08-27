use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use sample_reactor_virtual::performance::Scenario;

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() && size > layout.size() {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
        }
        pointer
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

struct Row {
    allocs: f64,
    bytes: f64,
    median_us: f64,
    name: String,
    p95_us: f64,
    p99_us: f64,
}

fn measure(
    name: impl Into<String>,
    samples: usize,
    warmups: usize,
    mut scenario: Scenario,
    mut operation: impl FnMut(&mut Scenario),
) -> Row {
    for _ in 0..warmups {
        operation(&mut scenario);
    }

    let mut timings = Vec::with_capacity(samples);
    let allocs = ALLOCS.load(Ordering::Relaxed);
    let bytes = BYTES.load(Ordering::Relaxed);
    for _ in 0..samples {
        let start = Instant::now();
        operation(&mut scenario);
        timings.push(start.elapsed().as_nanos() as f64 / 1_000.0);
    }
    let allocs = ALLOCS.load(Ordering::Relaxed) - allocs;
    let bytes = BYTES.load(Ordering::Relaxed) - bytes;
    timings.sort_by(f64::total_cmp);

    let percentile = |value: f64| {
        let index = ((timings.len() - 1) as f64 * value).ceil() as usize;
        timings[index]
    };
    Row {
        allocs: allocs as f64 / samples as f64,
        bytes: bytes as f64 / samples as f64,
        median_us: percentile(0.50),
        name: name.into(),
        p95_us: percentile(0.95),
        p99_us: percentile(0.99),
    }
}

fn main() {
    let samples = std::env::args()
        .skip_while(|argument| argument != "--samples")
        .nth(1)
        .map_or(500, |value| value.parse::<usize>().unwrap());
    assert!(samples > 0);
    let scaling_samples = std::env::args()
        .skip_while(|argument| argument != "--scaling-samples")
        .nth(1)
        .map_or(20, |value| value.parse::<usize>().unwrap());
    assert!(scaling_samples > 0);

    let rows = [
        measure(
            "local controlled edit",
            samples,
            16,
            Scenario::new(),
            |scenario| {
                scenario.local_edit();
            },
        ),
        measure(
            "broad selection change",
            samples,
            16,
            Scenario::new(),
            |scenario| {
                scenario.broad_selection_change();
            },
        ),
        measure(
            "redundant parent message",
            samples,
            16,
            Scenario::new(),
            |scenario| {
                scenario.redundant_parent_message();
            },
        ),
        measure(
            "unchanged root component memo hit",
            samples,
            16,
            Scenario::new(),
            |scenario| {
                scenario.unchanged_root_component_memo_hit();
            },
        ),
        measure(
            "value-equal root recomposition",
            samples,
            16,
            Scenario::new(),
            |scenario| {
                scenario.value_equal_root_recomposition();
            },
        ),
        measure(
            "32-row recycle/realize",
            samples,
            16,
            Scenario::new(),
            |scenario| {
                scenario.realize_recycle_batch();
            },
        ),
        measure(
            "background completion",
            samples,
            16,
            Scenario::new(),
            |scenario| {
                scenario.background_completion();
            },
        ),
        measure(
            "mixed virtual cycle",
            samples,
            16,
            Scenario::new(),
            |scenario| {
                scenario.mixed_virtual_cycle();
            },
        ),
    ];

    println!("windows-reactor virtual editor ({samples} samples after 16 warmups)");
    println!(
        "{:<28} {:>10} {:>10} {:>10} {:>12} {:>10}",
        "workload", "median us", "p95 us", "p99 us", "bytes/op", "allocs/op"
    );
    for row in rows {
        println!(
            "{:<28} {:>10.1} {:>10.1} {:>10.1} {:>12.0} {:>10.1}",
            row.name, row.median_us, row.p95_us, row.p99_us, row.bytes, row.allocs
        );
    }

    println!();
    println!("eager virtual-source scaling ({scaling_samples} samples after 2 warmups)");
    println!(
        "{:<28} {:>10} {:>10} {:>10} {:>12} {:>10}",
        "workload", "median us", "p95 us", "p99 us", "bytes/op", "allocs/op"
    );
    for task_count in [1_000, 10_000, 100_000] {
        let row = measure(
            format!("value-equal {task_count}"),
            scaling_samples,
            2,
            Scenario::with_task_count(task_count),
            Scenario::value_equal_root_recomposition,
        );
        println!(
            "{:<28} {:>10.1} {:>10.1} {:>10.1} {:>12.0} {:>10.1}",
            row.name, row.median_us, row.p95_us, row.p99_us, row.bytes, row.allocs
        );
    }
}
