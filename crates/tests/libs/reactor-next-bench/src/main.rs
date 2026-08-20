use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use windows_reactor_next::*;

static BYTES: AtomicU64 = AtomicU64::new(0);
static ALLOCS: AtomicU64 = AtomicU64::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() && size > layout.size() {
            BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        pointer
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

struct Perf {
    ns: f64,
    bytes: f64,
    allocs: f64,
}

struct Row {
    name: &'static str,
    n: usize,
    perf: Perf,
}

fn measure(iters: u64, reps: u32, mut op: impl FnMut()) -> Perf {
    for _ in 0..2 {
        for _ in 0..iters {
            op();
        }
    }

    let mut best = Perf {
        ns: f64::MAX,
        bytes: 0.0,
        allocs: 0.0,
    };
    for _ in 0..reps {
        let bytes = BYTES.load(Ordering::Relaxed);
        let allocs = ALLOCS.load(Ordering::Relaxed);
        let start = Instant::now();
        for _ in 0..iters {
            op();
        }
        let ns = start.elapsed().as_nanos() as f64 / iters as f64;
        if ns < best.ns {
            best.ns = ns;
            best.bytes = (BYTES.load(Ordering::Relaxed) - bytes) as f64 / iters as f64;
            best.allocs = (ALLOCS.load(Ordering::Relaxed) - allocs) as f64 / iters as f64;
        }
    }
    best
}

fn runtime() -> RecordingRuntime {
    let mut runtime = RecordingRuntime::default();
    runtime.record_commands(false);
    runtime
}

fn stack(labels: &[String]) -> Element {
    StackPanel::new()
        .children(
            labels
                .iter()
                .enumerate()
                .map(|(index, text)| KeyedElement::new(index as u64, TextBlock::new().text(text))),
        )
        .into()
}

fn keyed_stack(keys: &[String]) -> Element {
    StackPanel::new()
        .children(
            keys.iter()
                .map(|key| KeyedElement::new(key.clone(), TextBlock::new().text(key.clone()))),
        )
        .into()
}

fn virtual_list(key_prefix: &str, text_prefix: &str, count: usize) -> Element {
    ItemsRepeater::new()
        .items((0..count).map(|index| {
            KeyedElement::new(
                format!("{key_prefix}{index}"),
                TextBlock::new().text(format!("{text_prefix}{index}")),
            )
        }))
        .into()
}

fn bench_update(
    name: &'static str,
    n: usize,
    a: Element,
    b: Element,
    iters: u64,
    reps: u32,
) -> Row {
    let mut pump = Pump::new(runtime());
    pump.mount(a.clone()).unwrap();
    let mut flip = false;
    let perf = measure(iters, reps, || {
        pump.update(if flip { a.clone() } else { b.clone() })
            .unwrap();
        flip = !flip;
    });
    Row { name, n, perf }
}

fn queue_realize(pump: &mut Pump<RecordingRuntime>, count: usize) {
    let collection = pump.root().unwrap();
    for index in 0..count {
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(index as u64),
                index,
            });
    }
    pump.process_realizations().unwrap();
}

fn queue_recycle(pump: &mut Pump<RecordingRuntime>, count: usize) {
    let collection = pump.root().unwrap();
    for index in 0..count {
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(index as u64),
            });
    }
    pump.process_realizations().unwrap();
}

fn bench_virtual_payload(count: usize, realized: usize, iters: u64, reps: u32) -> Row {
    let a = virtual_list("key-", "a-", count);
    let b = virtual_list("key-", "b-", count);
    let mut pump = Pump::new(runtime());
    pump.mount(a.clone()).unwrap();
    queue_realize(&mut pump, realized);
    let mut flip = false;
    let perf = measure(iters, reps, || {
        pump.update(if flip { a.clone() } else { b.clone() })
            .unwrap();
        flip = !flip;
    });
    Row {
        name: "virtual_payload",
        n: count,
        perf,
    }
}

fn bench_virtual_reset(count: usize, realized: usize, iters: u64, reps: u32) -> Row {
    let a = virtual_list("a-", "row-", count);
    let b = virtual_list("b-", "row-", count);
    let mut pump = Pump::new(runtime());
    pump.mount(a.clone()).unwrap();
    queue_realize(&mut pump, realized);
    let mut flip = false;
    let perf = measure(iters, reps, || {
        pump.update(if flip { a.clone() } else { b.clone() })
            .unwrap();
        queue_realize(&mut pump, realized);
        flip = !flip;
    });
    Row {
        name: "virtual_reset",
        n: count,
        perf,
    }
}

fn bench_realize_cycle(count: usize, realized: usize, iters: u64, reps: u32) -> Row {
    let mut pump = Pump::new(runtime());
    pump.mount(virtual_list("key-", "row-", count)).unwrap();
    let perf = measure(iters, reps, || {
        queue_realize(&mut pump, realized);
        queue_recycle(&mut pump, realized);
    });
    Row {
        name: "realize_recycle",
        n: realized,
        perf,
    }
}

fn parse_arg(name: &str, default: u64) -> u64 {
    let args: Vec<String> = std::env::args().collect();
    args.windows(2)
        .find(|pair| pair[0] == name)
        .and_then(|pair| pair[1].parse().ok())
        .unwrap_or(default)
}

fn main() {
    let iters = parse_arg("--iters", 500);
    let reps = parse_arg("--reps", 6) as u32;
    let labels: Vec<_> = (0..512).map(|index| format!("cell-{index}")).collect();
    let mut changed = labels.clone();
    changed[0] = "changed".to_string();
    let mut reversed = labels.clone();
    reversed.reverse();

    let rows = [
        bench_update(
            "update_no_change",
            512,
            stack(&labels),
            stack(&labels),
            iters,
            reps,
        ),
        bench_update(
            "update_1_changed",
            512,
            stack(&labels),
            stack(&changed),
            iters,
            reps,
        ),
        bench_update(
            "keyed_reverse",
            512,
            keyed_stack(&labels),
            keyed_stack(&reversed),
            (iters / 4).max(1),
            reps,
        ),
        bench_update(
            "root_replace",
            1,
            TextBlock::new().text("text").into(),
            Button::new()
                .content(TextBlock::new().text("button"))
                .into(),
            iters,
            reps,
        ),
        bench_update(
            "content_replace",
            2,
            Button::new().content(TextBlock::new().text("text")).into(),
            Button::new()
                .content(StackPanel::new().child("row", TextBlock::new().text("row")))
                .into(),
            iters,
            reps,
        ),
        bench_update(
            "virtual_no_change",
            10_000,
            virtual_list("key-", "row-", 10_000),
            virtual_list("key-", "row-", 10_000),
            iters,
            reps,
        ),
        bench_virtual_payload(10_000, 32, (iters / 4).max(1), reps),
        bench_virtual_reset(10_000, 32, (iters / 10).max(1), reps),
        bench_realize_cycle(10_000, 32, (iters / 4).max(1), reps),
    ];

    println!("windows-reactor-next headless reconciler micro-benchmarks");
    println!("(RecordingRuntime; best-of-reps, native command history disabled)\n");
    println!(
        "{:<22} {:>8} {:>14} {:>14} {:>12}",
        "bench", "N", "ns/op", "bytes/op", "allocs/op"
    );
    println!("{}", "-".repeat(76));
    for row in rows {
        println!(
            "{:<22} {:>8} {:>14.1} {:>14.1} {:>12.2}",
            row.name, row.n, row.perf.ns, row.perf.bytes, row.perf.allocs
        );
    }
}
