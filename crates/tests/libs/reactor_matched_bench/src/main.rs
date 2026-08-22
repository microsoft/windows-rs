use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use test_reactor_matched_bench::{Model, OPERATIONS, Operation};

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
static CURRENT_BYTES: AtomicU64 = AtomicU64::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            CURRENT_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        CURRENT_BYTES.fetch_sub(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() && size > layout.size() {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
            CURRENT_BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
        } else if !pointer.is_null() {
            CURRENT_BYTES.fetch_sub((layout.size() - size) as u64, Ordering::Relaxed);
        }
        pointer
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

trait Scenario {
    type Declaration;

    fn declare(&mut self, operation: Operation, iteration: usize) -> Self::Declaration;
    fn publish(&mut self, declaration: Self::Declaration);
}

#[cfg(feature = "next")]
mod next {
    use super::*;
    use windows_reactor_next::*;

    pub(super) struct Next {
        model: Model,
        pump: Pump<RecordingRuntime>,
    }

    impl Next {
        pub(super) fn new() -> Self {
            let model = Model::default();
            let mut pump = Pump::new(RecordingRuntime::default());
            pump.mount_view(test_reactor_matched_bench::next::view(&model))
                .unwrap();
            Self { model, pump }
        }
    }

    impl Scenario for Next {
        type Declaration = View;

        fn declare(&mut self, operation: Operation, iteration: usize) -> Self::Declaration {
            self.model.apply(operation, iteration);
            test_reactor_matched_bench::next::view(&self.model)
        }

        fn publish(&mut self, declaration: Self::Declaration) {
            self.pump.update_view(declaration).unwrap();
        }
    }
}

#[cfg(feature = "incumbent")]
mod incumbent {
    use super::*;
    use std::rc::Rc;
    use test_reactor::RecordingBackend;
    use windows_reactor::*;

    pub(super) struct Incumbent {
        model: Model,
        previous: Element,
        reconciler: Reconciler<RecordingBackend>,
        rerender: Rc<dyn Fn()>,
        root: ControlId,
    }

    impl Incumbent {
        pub(super) fn new() -> Self {
            let model = Model::default();
            let previous = test_reactor_matched_bench::incumbent::view(&model);
            let mut reconciler = Reconciler::new(RecordingBackend::new());
            let rerender: Rc<dyn Fn()> = Rc::new(|| {});
            let root = reconciler
                .reconcile(None, &previous, None, Rc::clone(&rerender))
                .unwrap();
            Self {
                model,
                previous,
                reconciler,
                rerender,
                root,
            }
        }
    }

    impl Scenario for Incumbent {
        type Declaration = Element;

        fn declare(&mut self, operation: Operation, iteration: usize) -> Self::Declaration {
            self.model.apply(operation, iteration);
            test_reactor_matched_bench::incumbent::view(&self.model)
        }

        fn publish(&mut self, declaration: Self::Declaration) {
            self.reconciler
                .reconcile(
                    Some(&self.previous),
                    &declaration,
                    Some(self.root),
                    Rc::clone(&self.rerender),
                )
                .unwrap();
            self.previous = declaration;
        }
    }
}

struct Row {
    allocations: f64,
    bytes: f64,
    declaration_allocations: f64,
    declaration_bytes: f64,
    declaration_median_us: f64,
    frontend: &'static str,
    median_us: f64,
    operation: &'static str,
    p95_us: f64,
    p99_us: f64,
    publication_allocations: f64,
    publication_bytes: f64,
    publication_median_us: f64,
}

fn measure<S: Scenario>(
    frontend: &'static str,
    operation_name: &'static str,
    operation: Operation,
    samples: usize,
    mut scenario: S,
) -> Row {
    for iteration in 0..16 {
        let declaration = scenario.declare(operation, iteration);
        scenario.publish(declaration);
    }
    let mut timings = Vec::with_capacity(samples);
    let mut declaration_timings = Vec::with_capacity(samples);
    let mut publication_timings = Vec::with_capacity(samples);
    let allocations = ALLOCS.load(Ordering::Relaxed);
    let bytes = BYTES.load(Ordering::Relaxed);
    let mut declaration_allocations = 0;
    let mut declaration_bytes = 0;
    for iteration in 0..samples {
        let allocation_start = ALLOCS.load(Ordering::Relaxed);
        let byte_start = BYTES.load(Ordering::Relaxed);
        let start = Instant::now();
        let declaration = scenario.declare(operation, iteration);
        let declaration_end = Instant::now();
        let allocation_middle = ALLOCS.load(Ordering::Relaxed);
        let byte_middle = BYTES.load(Ordering::Relaxed);
        scenario.publish(declaration);
        let end = Instant::now();
        declaration_allocations += allocation_middle - allocation_start;
        declaration_bytes += byte_middle - byte_start;
        declaration_timings.push((declaration_end - start).as_nanos() as f64 / 1_000.0);
        publication_timings.push((end - declaration_end).as_nanos() as f64 / 1_000.0);
        timings.push((end - start).as_nanos() as f64 / 1_000.0);
    }
    let allocations = ALLOCS.load(Ordering::Relaxed) - allocations;
    let bytes = BYTES.load(Ordering::Relaxed) - bytes;
    timings.sort_by(f64::total_cmp);
    declaration_timings.sort_by(f64::total_cmp);
    publication_timings.sort_by(f64::total_cmp);
    let percentile = |values: &[f64], value: f64| {
        let index = ((values.len() - 1) as f64 * value).ceil() as usize;
        values[index]
    };
    Row {
        allocations: allocations as f64 / samples as f64,
        bytes: bytes as f64 / samples as f64,
        declaration_allocations: declaration_allocations as f64 / samples as f64,
        declaration_bytes: declaration_bytes as f64 / samples as f64,
        declaration_median_us: percentile(&declaration_timings, 0.5),
        frontend,
        median_us: percentile(&timings, 0.5),
        operation: operation_name,
        p95_us: percentile(&timings, 0.95),
        p99_us: percentile(&timings, 0.99),
        publication_allocations: (allocations - declaration_allocations) as f64 / samples as f64,
        publication_bytes: (bytes - declaration_bytes) as f64 / samples as f64,
        publication_median_us: percentile(&publication_timings, 0.5),
    }
}

fn retained_bytes<S>(create: impl FnOnce() -> S) -> u64 {
    let before = CURRENT_BYTES.load(Ordering::Relaxed);
    let scenario = create();
    let retained = CURRENT_BYTES.load(Ordering::Relaxed) - before;
    std::hint::black_box(&scenario);
    retained
}

fn main() {
    let samples = std::env::args()
        .skip_while(|argument| argument != "--samples")
        .nth(1)
        .map_or(500, |value| value.parse::<usize>().unwrap());
    assert!(samples > 0);

    let mut rows = Vec::new();
    for (name, operation) in OPERATIONS {
        #[cfg(feature = "incumbent")]
        rows.push(measure(
            "windows-reactor",
            name,
            operation,
            samples,
            incumbent::Incumbent::new(),
        ));
        #[cfg(feature = "next")]
        rows.push(measure(
            "windows-reactor-next",
            name,
            operation,
            samples,
            next::Next::new(),
        ));
    }

    println!("matched 32-task shell ({samples} samples after 16 warmups)");
    println!(
        "{:<21} {:<14} {:>10} {:>10} {:>10} {:>12} {:>10}",
        "frontend", "operation", "median us", "p95 us", "p99 us", "bytes/op", "allocs/op"
    );
    for row in &rows {
        println!(
            "{:<21} {:<14} {:>10.1} {:>10.1} {:>10.1} {:>12.0} {:>10.1}",
            row.frontend,
            row.operation,
            row.median_us,
            row.p95_us,
            row.p99_us,
            row.bytes,
            row.allocations
        );
    }
    println!();
    println!(
        "{:<21} {:<14} {:>11} {:>12} {:>11} {:>12} {:>11} {:>12}",
        "frontend",
        "operation",
        "declare us",
        "declare B",
        "declare A",
        "publish us",
        "publish B",
        "publish A"
    );
    for row in rows {
        println!(
            "{:<21} {:<14} {:>11.1} {:>12.0} {:>11.1} {:>12.1} {:>11.0} {:>12.1}",
            row.frontend,
            row.operation,
            row.declaration_median_us,
            row.declaration_bytes,
            row.declaration_allocations,
            row.publication_median_us,
            row.publication_bytes,
            row.publication_allocations
        );
    }
    println!();
    #[cfg(feature = "incumbent")]
    println!(
        "retained bytes: windows-reactor={}",
        retained_bytes(incumbent::Incumbent::new)
    );
    #[cfg(feature = "next")]
    println!(
        "retained bytes: windows-reactor-next={}",
        retained_bytes(next::Next::new)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measured_transitions_change_and_restore_the_model() {
        for operation in [
            Operation::BroadToggle,
            Operation::LocalEdit,
            Operation::Reverse,
            Operation::Selection,
        ] {
            let original = Model::default();
            let mut model = original.clone();
            model.apply(operation, 0);
            assert!(model != original);
            model.apply(operation, 0);
            assert!(model == original);
        }
    }

    #[test]
    fn value_equal_transition_preserves_the_model() {
        let original = Model::default();
        let mut model = original.clone();
        model.apply(Operation::ValueEqual, 0);
        assert!(model == original);
    }
}
