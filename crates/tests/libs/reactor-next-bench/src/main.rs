use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use windows_reactor_next::*;

static BYTES: AtomicU64 = AtomicU64::new(0);
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static CURRENT_BYTES: AtomicU64 = AtomicU64::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
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
            BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            CURRENT_BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
        } else if !pointer.is_null() {
            CURRENT_BYTES.fetch_sub((layout.size() - size) as u64, Ordering::Relaxed);
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

struct FrontendRow {
    frontend: &'static str,
    name: &'static str,
    n: usize,
    median_ns: f64,
    p95_ns: f64,
    p99_ns: f64,
    bytes: f64,
    allocs: f64,
}

struct MemoryRow {
    n: usize,
    bytes: u64,
    bytes_per_scope: f64,
}

#[derive(Clone)]
struct LeafProps {
    sender: Rc<RefCell<Option<LocalSender<bool>>>>,
}

impl PartialEq for LeafProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct BenchLeaf {
    active: bool,
}

impl Component for BenchLeaf {
    type Props = LeafProps;
    type Message = bool;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        *props.sender.borrow_mut() = Some(context.sender());
        Self { active: false }
    }

    fn changed(&mut self, _props: &Self::Props, _context: &mut ComponentContext<Self>) {}

    fn update(&mut self, toggle: Self::Message, _context: &mut ComponentContext<Self>) {
        if toggle {
            self.active = !self.active;
        }
    }

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(if self.active { "on" } else { "off" }))
    }
}

struct BenchRoot {
    senders: Rc<Vec<Rc<RefCell<Option<LocalSender<bool>>>>>>,
}

#[derive(Clone)]
struct RootProps(Rc<Vec<Rc<RefCell<Option<LocalSender<bool>>>>>>);

impl PartialEq for RootProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Component for BenchRoot {
    type Props = RootProps;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self {
            senders: Rc::clone(&props.0),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.senders = Rc::clone(&props.0);
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::children(
            StackPanel::new(),
            self.senders.iter().enumerate().map(|(index, sender)| {
                KeyedView::new(
                    index,
                    View::component::<BenchLeaf>(LeafProps {
                        sender: Rc::clone(sender),
                    }),
                )
            }),
        )
    }
}

struct BenchFragmentRoot(RootProps);

impl Component for BenchFragmentRoot {
    type Props = RootProps;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(props.clone())
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.0 = props.clone();
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        let fragment = self.0.0.iter().enumerate().map(|(index, sender)| {
            KeyedView::new(
                index,
                View::component::<BenchLeaf>(LeafProps {
                    sender: Rc::clone(sender),
                }),
            )
        });
        View::children(
            StackPanel::new(),
            [KeyedView::new("fragment", View::fragment(fragment))],
        )
    }
}

#[derive(Clone)]
struct KeyedRootProps(Rc<Vec<u64>>);

impl PartialEq for KeyedRootProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

struct KeyedRoot(KeyedRootProps);

impl Component for KeyedRoot {
    type Props = KeyedRootProps;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(props.clone())
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.0 = props.clone();
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::children(
            StackPanel::new(),
            self.0
                .0
                .iter()
                .map(|key| KeyedView::new(*key, View::component::<KeyedLeaf>(*key))),
        )
    }
}

struct KeyedLeaf(u64);

impl Component for KeyedLeaf {
    type Props = u64;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(*props)
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.0 = *props;
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(self.0.to_string()))
    }
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

fn measure_frontend(
    frontend: &'static str,
    name: &'static str,
    n: usize,
    samples: usize,
    batch: usize,
    mut op: impl FnMut(),
) -> FrontendRow {
    for _ in 0..16 {
        for _ in 0..batch {
            op();
        }
    }
    let mut timings = Vec::with_capacity(samples);
    let bytes = BYTES.load(Ordering::Relaxed);
    let allocs = ALLOCS.load(Ordering::Relaxed);
    for _ in 0..samples {
        let start = Instant::now();
        for _ in 0..batch {
            op();
        }
        timings.push(start.elapsed().as_nanos() as f64 / batch as f64);
    }
    timings.sort_by(f64::total_cmp);
    let percentile = |value: f64| {
        let index = ((timings.len() - 1) as f64 * value).ceil() as usize;
        timings[index]
    };
    FrontendRow {
        frontend,
        name,
        n,
        median_ns: percentile(0.50),
        p95_ns: percentile(0.95),
        p99_ns: percentile(0.99),
        bytes: (BYTES.load(Ordering::Relaxed) - bytes) as f64 / (samples * batch) as f64,
        allocs: (ALLOCS.load(Ordering::Relaxed) - allocs) as f64 / (samples * batch) as f64,
    }
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

fn bench_component_keyed(
    name: &'static str,
    n: usize,
    a: Vec<u64>,
    b: Vec<u64>,
    iters: u64,
    reps: u32,
) -> Row {
    let a = KeyedRootProps(Rc::new(a));
    let b = KeyedRootProps(Rc::new(b));
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<KeyedRoot>(a.clone()))
        .unwrap();
    let mut flip = false;
    let perf = measure(iters, reps, || {
        let props = if flip { a.clone() } else { b.clone() };
        pump.update_view(View::component::<KeyedRoot>(props))
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

fn bench_component_leaf(count: usize, iters: u64, reps: u32) -> Row {
    let senders = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<BenchRoot>(RootProps(Rc::clone(&senders))))
        .unwrap();
    let sender = senders[count / 2].borrow().as_ref().unwrap().clone();
    let perf = measure(iters, reps, || {
        sender.send(true);
        pump.dispatch_components(1).unwrap();
    });
    Row {
        name: "component_leaf",
        n: count,
        perf,
    }
}

fn bench_hook_no_change(count: usize, samples: usize) -> FrontendRow {
    let state = Rc::new(RefCell::new(None));
    let state_capture = Rc::clone(&state);
    let labels = Rc::new(
        (0..count)
            .map(|index| format!("cell-{index}"))
            .collect::<Vec<_>>(),
    );
    let mut app = RenderLoop::new(runtime(), move |hooks| {
        let active = hooks.use_state(|| false);
        *state_capture.borrow_mut() = Some(active);
        stack(&labels)
    });
    app.run().unwrap();
    let state = state.borrow().as_ref().unwrap().clone();
    measure_frontend("hooks", "no_change", count, samples, 1, || {
        state.set(state.get());
        app.run().unwrap();
    })
}

fn bench_component_no_change(count: usize, samples: usize) -> FrontendRow {
    let senders = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<BenchRoot>(RootProps(Rc::clone(&senders))))
        .unwrap();
    let sender = senders[count / 2].borrow().as_ref().unwrap().clone();
    measure_frontend("components", "no_change", count, samples, 1, || {
        _ = sender.send(false);
        pump.dispatch_components(1).unwrap();
    })
}

fn bench_hook_isolated_leaf(count: usize, samples: usize) -> FrontendRow {
    let state = Rc::new(RefCell::new(None));
    let state_capture = Rc::clone(&state);
    let labels = Rc::new(
        (0..count)
            .map(|index| format!("cell-{index}"))
            .collect::<Vec<_>>(),
    );
    let mut app = RenderLoop::new(runtime(), move |hooks| {
        let active = hooks.use_state(|| false);
        *state_capture.borrow_mut() = Some(active.clone());
        StackPanel::new()
            .children(labels.iter().enumerate().map(|(index, label)| {
                KeyedElement::new(
                    index,
                    TextBlock::new().text(if index == count / 2 && active.get() {
                        "active".to_string()
                    } else {
                        label.clone()
                    }),
                )
            }))
            .into()
    });
    app.run().unwrap();
    let state = state.borrow().as_ref().unwrap().clone();
    measure_frontend("hooks", "isolated_leaf", count, samples, 1, || {
        state.update(|active| *active = !*active);
        app.run().unwrap();
    })
}

fn bench_component_isolated_leaf(count: usize, samples: usize) -> FrontendRow {
    let senders = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<BenchRoot>(RootProps(Rc::clone(&senders))))
        .unwrap();
    let sender = senders[count / 2].borrow().as_ref().unwrap().clone();
    measure_frontend("components", "isolated_leaf", count, samples, 1, || {
        _ = sender.send(true);
        pump.dispatch_components(1).unwrap();
    })
}

fn bench_component_fragment_leaf(count: usize, samples: usize) -> FrontendRow {
    let senders = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<BenchFragmentRoot>(RootProps(Rc::clone(
        &senders,
    ))))
    .unwrap();
    let sender = senders[count / 2].borrow().as_ref().unwrap().clone();
    measure_frontend("components", "fragment_leaf", count, samples, 1, || {
        _ = sender.send(true);
        pump.dispatch_components(1).unwrap();
    })
}

fn measure_idle_component_memory(count: usize) -> MemoryRow {
    let senders = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let mut pump = Pump::new(runtime());
    let before = CURRENT_BYTES.load(Ordering::Relaxed);
    pump.mount_view(View::component::<BenchRoot>(RootProps(senders)))
        .unwrap();
    let bytes = CURRENT_BYTES.load(Ordering::Relaxed) - before;
    pump.shutdown();
    MemoryRow {
        n: count + 1,
        bytes,
        bytes_per_scope: bytes as f64 / (count + 1) as f64,
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
    let labels_4k: Vec<_> = (0..4_096).map(|index| format!("cell-{index}")).collect();
    let mut reversed_4k = labels_4k.clone();
    reversed_4k.reverse();
    let component_keys = (0..512_u64).collect::<Vec<_>>();
    let mut component_reversed = component_keys.clone();
    component_reversed.reverse();
    let mut component_rotated = component_keys.clone();
    component_rotated.rotate_left(1);
    let mut component_inserted = component_keys.clone();
    component_inserted.push(512);
    let mut component_removed = component_keys.clone();
    component_removed.pop();
    let component_keys_4k = (0..4_096_u64).collect::<Vec<_>>();
    let mut component_reversed_4k = component_keys_4k.clone();
    component_reversed_4k.reverse();
    let mut component_rotated_4k = component_keys_4k.clone();
    component_rotated_4k.rotate_left(1);
    let mut component_inserted_4k = component_keys_4k.clone();
    component_inserted_4k.push(4_096);
    let mut component_removed_4k = component_keys_4k.clone();
    component_removed_4k.pop();

    let rows = vec![
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
            "keyed_reverse",
            4_096,
            keyed_stack(&labels_4k),
            keyed_stack(&reversed_4k),
            (iters / 16).max(1),
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
        bench_component_leaf(512, iters, reps),
        bench_component_leaf(4_096, (iters / 4).max(1), reps),
        bench_component_leaf(16_384, (iters / 16).max(1), reps),
        bench_component_keyed(
            "component_same_order",
            512,
            component_keys.clone(),
            component_keys.clone(),
            (iters / 4).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_reverse",
            512,
            component_keys.clone(),
            component_reversed,
            (iters / 4).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_rotate",
            512,
            component_keys.clone(),
            component_rotated,
            (iters / 4).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_insert",
            512,
            component_keys.clone(),
            component_inserted,
            (iters / 4).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_remove",
            512,
            component_keys.clone(),
            component_removed,
            (iters / 4).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_same_order",
            4_096,
            component_keys_4k.clone(),
            component_keys_4k.clone(),
            (iters / 32).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_reverse",
            4_096,
            component_keys_4k.clone(),
            component_reversed_4k,
            (iters / 32).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_rotate",
            4_096,
            component_keys_4k.clone(),
            component_rotated_4k,
            (iters / 32).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_insert",
            4_096,
            component_keys_4k.clone(),
            component_inserted_4k,
            (iters / 32).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_remove",
            4_096,
            component_keys_4k,
            component_removed_4k,
            (iters / 32).max(1),
            reps,
        ),
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

    let samples = usize::try_from(iters).unwrap().max(128);
    let frontend_rows = [
        bench_hook_no_change(512, samples),
        bench_component_no_change(512, samples),
        bench_hook_isolated_leaf(512, samples),
        bench_component_isolated_leaf(512, samples),
        bench_hook_isolated_leaf(4_096, samples),
        bench_component_isolated_leaf(4_096, samples),
        bench_hook_isolated_leaf(16_384, samples),
        bench_component_isolated_leaf(16_384, samples),
        bench_component_fragment_leaf(512, samples),
        bench_component_fragment_leaf(16_384, samples),
    ];
    println!("\nfrontend comparison");
    println!(
        "{:<12} {:<16} {:>8} {:>12} {:>12} {:>12} {:>12} {:>10}",
        "frontend", "bench", "N", "median ns", "p95 ns", "p99 ns", "bytes/op", "allocs/op"
    );
    println!("{}", "-".repeat(104));
    for row in frontend_rows {
        println!(
            "{:<12} {:<16} {:>8} {:>12.1} {:>12.1} {:>12.1} {:>12.1} {:>10.2}",
            row.frontend,
            row.name,
            row.n,
            row.median_ns,
            row.p95_ns,
            row.p99_ns,
            row.bytes,
            row.allocs
        );
    }

    println!("\nidle component memory");
    println!(
        "{:>8} {:>16} {:>18}",
        "scopes", "retained bytes", "bytes/scope"
    );
    println!("{}", "-".repeat(46));
    for row in [512, 4_096, 16_384].map(measure_idle_component_memory) {
        println!(
            "{:>8} {:>16} {:>18.1}",
            row.n, row.bytes, row.bytes_per_scope
        );
    }
}
