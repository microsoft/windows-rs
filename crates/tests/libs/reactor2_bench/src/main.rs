use std::hint::black_box;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use windows_reactor as reactor1;
use windows_reactor::{ChildrenControl as _, TreeViewExt as _};
use windows_reactor2 as reactor2;

mod allocator;

struct Perf {
    median_ns: f64,
    p95_ns: f64,
    bytes: f64,
    allocations: f64,
}

struct Row {
    frontend: &'static str,
    workload: &'static str,
    objects: usize,
    perf: Perf,
}

struct MemoryRow {
    frontend: &'static str,
    workload: &'static str,
    objects: usize,
    bytes: u64,
    allocations: u64,
}

struct GraphMemoryRow {
    fixture: &'static str,
    bytes: u64,
    allocations: u64,
    memory: reactor2::RetainedMemory,
}

struct RecursiveRow {
    allocations: f64,
    bytes: f64,
    bytes_per_scope: f64,
    depth: usize,
    fanout: usize,
    median_ns: f64,
    mutations: f64,
    p95_ns: f64,
    retained_bytes: u64,
    scopes: usize,
}

struct BenchComponent {
    active: bool,
    effect: bool,
}

#[derive(Default)]
struct LifecycleServices {
    background: Mutex<Vec<Box<dyn FnOnce() + Send>>>,
    cancelled_timers: Arc<AtomicUsize>,
}

impl LifecycleServices {
    fn run_background(&self) {
        for work in std::mem::take(&mut *self.background.lock().unwrap()) {
            work();
        }
    }
}

struct LifecycleTimerRegistration {
    active: AtomicBool,
    cancelled: Arc<AtomicUsize>,
}

impl reactor2::ComponentTimerRegistration for LifecycleTimerRegistration {
    fn cancel(&self) {
        if self.active.swap(false, Ordering::AcqRel) {
            self.cancelled.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl reactor2::ComponentServices for LifecycleServices {
    fn spawn_background(&self, work: Box<dyn FnOnce() + Send>) {
        self.background.lock().unwrap().push(work);
    }

    fn set_timeout(
        &self,
        _delay: Duration,
        _callback: Box<dyn FnOnce() + Send>,
    ) -> Arc<dyn reactor2::ComponentTimerRegistration> {
        Arc::new(LifecycleTimerRegistration {
            active: AtomicBool::new(true),
            cancelled: Arc::clone(&self.cancelled_timers),
        })
    }
}

#[derive(Clone)]
struct LifecycleInput {
    cleanups: Arc<AtomicUsize>,
    context: Rc<reactor2::Context<usize>>,
    task: Arc<Mutex<Option<reactor2::ComponentTask>>>,
}

impl PartialEq for LifecycleInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.cleanups, &other.cleanups) && Rc::ptr_eq(&self.context, &other.context)
    }
}

struct LifecycleComponent {
    _timer: reactor2::ComponentTimer,
}

impl reactor2::Component for LifecycleComponent {
    type Input = LifecycleInput;
    type Message = usize;

    fn create(input: &Self::Input, context: &reactor2::ComponentContext<Self::Message>) -> Self {
        let task = context.spawn_background(|cancel| usize::from(cancel.is_cancelled()));
        *input.task.lock().unwrap() = Some(task);
        Self {
            _timer: context.set_timeout(Duration::from_secs(60), 1),
        }
    }

    fn view(
        &self,
        input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<Self::Message>,
    ) -> reactor2::Visual {
        let value = context.use_context(&input.context);
        let cleanups = Arc::clone(&input.cleanups);
        context.use_effect("lifecycle", value, move || {
            Some(Box::new(move || {
                cleanups.fetch_add(1, Ordering::Relaxed);
            }))
        });
        let sender = context.sender();
        reactor2::Button::new()
            .automation_name(format!("lifecycle-{value}"))
            .element_ref(&context.root())
            .on_click(move || {
                let _ = sender.send(value);
            })
            .into()
    }
}

#[derive(Clone, PartialEq)]
struct BranchInput {
    depth: usize,
    fanout: usize,
}

struct Branch(usize);

impl reactor2::Component for Branch {
    type Input = BranchInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(0)
    }

    fn update(&mut self, (): Self::Message, _context: &reactor2::ComponentContext<Self::Message>) {
        self.0 += 1;
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<Self::Message>,
    ) -> reactor2::Visual {
        if input.depth == 0 {
            return reactor2::TextBlock::new(self.0.to_string()).into();
        }
        reactor2::StackPanel::new()
            .children((0..input.fanout).map(|index| {
                reactor2::component::<Self>(
                    index.to_string(),
                    BranchInput {
                        depth: input.depth - 1,
                        fanout: input.fanout,
                    },
                )
                .into()
            }))
            .into()
    }
}

#[derive(Clone)]
struct ContextInput {
    context: Rc<reactor2::Context<bool>>,
    subscribe: bool,
}

impl PartialEq for ContextInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context) && self.subscribe == other.subscribe
    }
}

struct ContextComponent;

impl reactor2::Component for ContextComponent {
    type Input = ContextInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self
    }

    fn view(
        &self,
        input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<Self::Message>,
    ) -> reactor2::Visual {
        let value = if input.subscribe {
            context.use_context(&input.context)
        } else {
            false
        };
        reactor2::TextBlock::new(if value { "on" } else { "off" }).into()
    }
}

impl reactor2::Component for BenchComponent {
    type Input = bool;
    type Message = bool;

    fn create(effect: &bool, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self {
            active: false,
            effect: *effect,
        }
    }

    fn update(&mut self, toggle: bool, _context: &reactor2::ComponentContext<Self::Message>) {
        if toggle {
            self.active = !self.active;
        }
    }

    fn view(
        &self,
        _input: &bool,
        context: &mut reactor2::ComponentViewContext<Self::Message>,
    ) -> reactor2::Visual {
        if self.effect {
            context.use_effect("bench", self.active, || None);
        }
        reactor2::TextBlock::new(if self.active { "on" } else { "off" }).into()
    }
}

struct RootSwitch(bool);

impl reactor2::Component for RootSwitch {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &reactor2::ComponentContext<Self::Message>) {
        self.0 = !self.0;
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<Self::Message>,
    ) -> reactor2::Visual {
        if self.0 {
            reactor2::Border::new().into()
        } else {
            reactor2::TextBlock::new("root").into()
        }
    }
}

#[derive(Default)]
struct NullAdapter;

impl reactor2::Adapter for NullAdapter {
    type Error = std::convert::Infallible;

    fn validate(&self, _mutations: &[reactor2::Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, _mutations: &[reactor2::Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn focus(&mut self, _object: reactor2::ObjectId) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

fn measure(samples: usize, batch: usize, mut operation: impl FnMut()) -> Perf {
    for _ in 0..8 {
        for _ in 0..batch {
            operation();
        }
    }
    let mut timings = Vec::with_capacity(samples);
    let bytes = allocator::allocated_bytes();
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    for _ in 0..samples {
        let start = Instant::now();
        for _ in 0..batch {
            operation();
        }
        timings.push(start.elapsed().as_nanos() as f64 / batch as f64);
    }
    timings.sort_by(f64::total_cmp);
    let percentile = |fraction: f64| {
        let index = ((timings.len() - 1) as f64 * fraction).ceil() as usize;
        timings[index]
    };
    Perf {
        median_ns: percentile(0.50),
        p95_ns: percentile(0.95),
        bytes: (allocator::allocated_bytes() - bytes) as f64 / (samples * batch) as f64,
        allocations: (allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations) as f64
            / (samples * batch) as f64,
    }
}

fn measure_prepared<S>(
    samples: usize,
    batch: usize,
    mut prepare: impl FnMut() -> S,
    mut operation: impl FnMut(S),
) -> Perf {
    for _ in 0..8 {
        operation(prepare());
    }
    let mut timings = Vec::with_capacity(samples * batch);
    let mut bytes = 0;
    let mut allocations = 0;
    for _ in 0..samples {
        for _ in 0..batch {
            let state = prepare();
            let before_bytes = allocator::allocated_bytes();
            let before_allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
            let start = Instant::now();
            operation(state);
            timings.push(start.elapsed().as_nanos() as f64);
            bytes += allocator::allocated_bytes() - before_bytes;
            allocations += allocator::ALLOCATIONS.load(Ordering::Relaxed) - before_allocations;
        }
    }
    timings.sort_by(f64::total_cmp);
    let percentile = |value: f64| {
        let index = ((timings.len() - 1) as f64 * value).round() as usize;
        timings[index]
    };
    Perf {
        median_ns: percentile(0.50),
        p95_ns: percentile(0.95),
        bytes: bytes as f64 / timings.len() as f64,
        allocations: allocations as f64 / timings.len() as f64,
    }
}

fn reactor1_runtime() -> reactor1::test::RecordingRuntime {
    let mut runtime = reactor1::test::RecordingRuntime::default();
    runtime.record_commands(false);
    runtime
}

fn reactor2_runtime() -> reactor2::Runtime<reactor2::RecordingAdapter> {
    let mut adapter = reactor2::RecordingAdapter::default();
    adapter.record_batches(false);
    adapter.validate_batches(false);
    reactor2::Runtime::new(adapter)
}

fn reactor1_grid(order: &[usize], changed: Option<usize>) -> reactor1::View {
    reactor1::StackPanel::new().keyed_children(order.iter().map(|index| {
        reactor1::KeyedView::new(
            *index,
            reactor1::TextBlock::new().text(if Some(*index) == changed {
                format!("Changed {index}")
            } else {
                format!("Cell {index}")
            }),
        )
    }))
}

fn reactor2_grid(order: &[usize], changed: Option<usize>) -> reactor2::Visual {
    reactor2::Grid::new()
        .children(order.iter().map(|index| {
            reactor2::keyed(
                *index,
                reactor2::TextBlock::new(if Some(*index) == changed {
                    format!("Changed {index}")
                } else {
                    format!("Cell {index}")
                }),
            )
        }))
        .into()
}

fn reactor1_tree(order: &[usize], changed: Option<usize>) -> reactor1::View {
    reactor1::TreeView::new().nodes(order.iter().map(|index| {
        reactor1::TreeNode::new(
            index.to_string(),
            if Some(*index) == changed {
                format!("Changed {index}")
            } else {
                format!("Node {index}")
            },
        )
    }))
}

fn reactor2_tree(order: &[usize], changed: Option<usize>) -> reactor2::Visual {
    reactor2::TreeView::new()
        .nodes(order.iter().map(|index| {
            reactor2::TreeNode::new(
                *index,
                if Some(*index) == changed {
                    format!("Changed {index}")
                } else {
                    format!("Node {index}")
                },
            )
        }))
        .into()
}

fn bench_reactor1(
    workload: &'static str,
    objects: usize,
    first: reactor1::View,
    second: reactor1::View,
    samples: usize,
    batch: usize,
) -> Row {
    let mut pump = reactor1::test::Pump::new(reactor1_runtime());
    pump.mount_view(first.clone()).unwrap();
    let mut flip = false;
    let perf = measure(samples, batch, || {
        pump.update_view(if flip { first.clone() } else { second.clone() })
            .unwrap();
        flip = !flip;
    });
    Row {
        frontend: "reactor",
        workload,
        objects,
        perf,
    }
}

fn bench_reactor2(
    workload: &'static str,
    objects: usize,
    first: reactor2::Visual,
    second: reactor2::Visual,
    samples: usize,
    batch: usize,
) -> Row {
    let mut runtime = reactor2_runtime();
    runtime.update(first.clone()).unwrap();
    let mut flip = false;
    let perf = measure(samples, batch, || {
        runtime
            .update(if flip { first.clone() } else { second.clone() })
            .unwrap();
        flip = !flip;
    });
    Row {
        frontend: "reactor2",
        workload,
        objects,
        perf,
    }
}

fn bench_reactor1_100k(one_changed: bool, samples: usize) -> Row {
    let count = 50_000;
    let order = (0..count).collect::<Vec<_>>();
    let first = reactor1_grid(&order, None);
    let changed = reactor1_grid(&order, one_changed.then_some(0));
    let mut pumps = [
        reactor1::test::Pump::new(reactor1_runtime()),
        reactor1::test::Pump::new(reactor1_runtime()),
    ];
    for pump in &mut pumps {
        pump.mount_view(first.clone()).unwrap();
    }
    let mut flip = false;
    let perf = measure(samples, 1, || {
        pumps[0]
            .update_view(if flip { first.clone() } else { changed.clone() })
            .unwrap();
        pumps[1].update_view(first.clone()).unwrap();
        flip = !flip;
    });
    Row {
        frontend: "reactor",
        workload: if one_changed {
            "keyed_one_changed"
        } else {
            "keyed_no_change"
        },
        objects: count * 2,
        perf,
    }
}

fn bench_reactor2_100k(one_changed: bool, samples: usize) -> Row {
    let count = 50_000;
    let order = (0..count).collect::<Vec<_>>();
    let first = reactor2_grid(&order, None);
    let changed = reactor2_grid(&order, one_changed.then_some(0));
    let mut runtimes = [reactor2_runtime(), reactor2_runtime()];
    for runtime in &mut runtimes {
        runtime.update(first.clone()).unwrap();
    }
    let mut flip = false;
    let perf = measure(samples, 1, || {
        runtimes[0]
            .update(if flip { first.clone() } else { changed.clone() })
            .unwrap();
        runtimes[1].update(first.clone()).unwrap();
        flip = !flip;
    });
    Row {
        frontend: "reactor2",
        workload: if one_changed {
            "keyed_one_changed"
        } else {
            "keyed_no_change"
        },
        objects: count * 2,
        perf,
    }
}

fn retirement_view(visible: bool) -> reactor2::Visual {
    reactor2::Grid::new()
        .children(visible.then(|| {
            reactor2::keyed(
                "retiring",
                reactor2::Button::new()
                    .exit_fade(Duration::from_secs(1))
                    .content(reactor2::TextBlock::new("retiring")),
            )
        }))
        .into()
}

fn retirement_runtime() -> (
    reactor2::Runtime<reactor2::RecordingAdapter>,
    reactor2::ObjectId,
) {
    let mut runtime = reactor2_runtime();
    runtime.update(retirement_view(true)).unwrap();
    let root = runtime.graph().root().unwrap();
    let retiring = runtime
        .graph()
        .children(root, reactor2::RelationId::Children)
        .unwrap()[0];
    (runtime, retiring)
}

fn bench_reactor2_retirement_initiation(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(samples, batch, retirement_runtime, |(mut runtime, _)| {
        runtime.update(retirement_view(false)).unwrap();
    });
    Row {
        frontend: "reactor2",
        workload: "retirement_start",
        objects: 3,
        perf,
    }
}

fn bench_reactor2_retirement_completion(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let (mut runtime, retiring) = retirement_runtime();
            runtime.update(retirement_view(false)).unwrap();
            assert!(runtime.complete_retirement(retiring));
            runtime
        },
        |mut runtime| {
            runtime.dispatch_native_events().unwrap();
        },
    );
    Row {
        frontend: "reactor2",
        workload: "retirement_finish",
        objects: 3,
        perf,
    }
}

fn bench_reactor2_retirement_remount(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let (mut runtime, retiring) = retirement_runtime();
            runtime.update(retirement_view(false)).unwrap();
            assert!(runtime.complete_retirement(retiring));
            runtime.dispatch_native_events().unwrap();
            runtime
        },
        |mut runtime| {
            runtime.update(retirement_view(true)).unwrap();
        },
    );
    Row {
        frontend: "reactor2",
        workload: "retirement_remount",
        objects: 3,
        perf,
    }
}

fn virtual_items_view(revision: u64) -> reactor2::Visual {
    reactor2::ItemsRepeater::new()
        .virtual_source(reactor2::VirtualSource::new(
            1,
            10_000,
            reactor2::Key::from,
            move |index| -> reactor2::Visual {
                reactor2::TextBlock::new(format!("{revision}:{index}")).into()
            },
        ))
        .into()
}

fn virtual_items_runtime() -> (
    reactor2::Runtime<reactor2::RecordingAdapter>,
    reactor2::ObjectId,
) {
    let mut runtime = reactor2_runtime();
    runtime.update(virtual_items_view(0)).unwrap();
    let collection = runtime.graph().root().unwrap();
    (runtime, collection)
}

fn virtual_items_realized_runtime(
    realized: usize,
) -> (
    reactor2::Runtime<reactor2::RecordingAdapter>,
    reactor2::ObjectId,
) {
    let (mut runtime, collection) = virtual_items_runtime();
    for index in 0..realized {
        runtime.queue_realization(reactor2::RealizationRequest::Realize {
            collection,
            container: reactor2::RealizedContainer(index as u64),
            index,
            source_revision: 0,
        });
    }
    runtime.dispatch_native_events().unwrap();
    (runtime, collection)
}

fn bench_reactor2_virtual_create(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(samples, batch, reactor2_runtime, |mut runtime| {
        black_box(runtime.update(virtual_items_view(0)).unwrap());
    });
    Row {
        frontend: "reactor2",
        workload: "virtual_create_10k",
        objects: 10_000,
        perf,
    }
}

fn bench_reactor2_virtual_source_replace(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let mut runtime = reactor2_runtime();
            runtime.update(reactor2::ItemsRepeater::new()).unwrap();
            runtime
        },
        |mut runtime| {
            black_box(runtime.update(virtual_items_view(0)).unwrap());
        },
    );
    Row {
        frontend: "reactor2",
        workload: "virtual_source_replace_10k",
        objects: 10_000,
        perf,
    }
}

fn bench_reactor2_virtual_realize(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let (mut runtime, collection) = virtual_items_runtime();
            runtime.queue_realization(reactor2::RealizationRequest::Realize {
                collection,
                container: reactor2::RealizedContainer(1),
                index: 9_999,
                source_revision: 0,
            });
            runtime
        },
        |mut runtime| {
            black_box(runtime.dispatch_native_events().unwrap());
        },
    );
    Row {
        frontend: "reactor2",
        workload: "virtual_realize_10k",
        objects: 10_000,
        perf,
    }
}

fn bench_reactor2_virtual_update(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || virtual_items_realized_runtime(8),
        |(mut runtime, _)| {
            black_box(runtime.update(virtual_items_view(1)).unwrap());
            black_box(runtime.dispatch_native_events().unwrap());
        },
    );
    Row {
        frontend: "reactor2",
        workload: "virtual_update_10k",
        objects: 10_000,
        perf,
    }
}

fn bench_reactor2_virtual_recycle(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let (mut runtime, collection) = virtual_items_realized_runtime(8);
            runtime.queue_realization(reactor2::RealizationRequest::Recycle {
                collection,
                container: reactor2::RealizedContainer(7),
                source_revision: 0,
            });
            runtime
        },
        |mut runtime| {
            black_box(runtime.dispatch_native_events().unwrap());
        },
    );
    Row {
        frontend: "reactor2",
        workload: "virtual_recycle_10k",
        objects: 10_000,
        perf,
    }
}

fn reactor2_virtual_memory() -> MemoryRow {
    let before_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let before_allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let (runtime, _) = virtual_items_realized_runtime(8);
    let bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before_bytes;
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - before_allocations;
    assert_eq!(runtime.graph().object_count(), 9);
    black_box(runtime);
    MemoryRow {
        frontend: "reactor2",
        workload: "virtual_10k_eight_rows",
        objects: 10_000,
        bytes,
        allocations,
    }
}

fn reactor2_virtual_mutation_counts() -> [(&'static str, usize, usize); 4] {
    let mut create = reactor2_runtime();
    create.record_batches(true);
    create.update(virtual_items_view(0)).unwrap();

    let (mut realize, collection) = virtual_items_runtime();
    realize.record_batches(true);
    realize.queue_realization(reactor2::RealizationRequest::Realize {
        collection,
        container: reactor2::RealizedContainer(0),
        index: 0,
        source_revision: 0,
    });
    realize.dispatch_native_events().unwrap();

    let (mut update, _) = virtual_items_realized_runtime(8);
    update.record_batches(true);
    update.update(virtual_items_view(1)).unwrap();
    update.dispatch_native_events().unwrap();

    let (mut recycle, collection) = virtual_items_realized_runtime(8);
    recycle.record_batches(true);
    recycle.queue_realization(reactor2::RealizationRequest::Recycle {
        collection,
        container: reactor2::RealizedContainer(7),
        source_revision: 0,
    });
    recycle.dispatch_native_events().unwrap();

    [
        (
            "virtual_create_10k",
            create.adapter().batches().iter().map(Vec::len).sum(),
            create.graph().object_count() - 1,
        ),
        (
            "virtual_realize_10k",
            realize.adapter().batches().iter().map(Vec::len).sum(),
            realize.graph().object_count() - 1,
        ),
        (
            "virtual_update_10k",
            update.adapter().batches().iter().map(Vec::len).sum(),
            update.graph().object_count() - 1,
        ),
        (
            "virtual_recycle_10k",
            recycle.adapter().batches().iter().map(Vec::len).sum(),
            recycle.graph().object_count() - 1,
        ),
    ]
}

fn reactor1_memory(count: usize) -> MemoryRow {
    let order = (0..count).collect::<Vec<_>>();
    let mut pump = reactor1::test::Pump::new(reactor1_runtime());
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    pump.mount_view(reactor1_grid(&order, None)).unwrap();
    MemoryRow {
        frontend: "reactor",
        workload: "keyed_grid_retained",
        objects: count + 1,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor2_memory(count: usize) -> MemoryRow {
    let order = (0..count).collect::<Vec<_>>();
    let mut runtime = reactor2_runtime();
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(reactor2_grid(&order, None)).unwrap();
    black_box(runtime.graph());
    MemoryRow {
        frontend: "reactor2",
        workload: "keyed_grid_retained",
        objects: count + 1,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor2_graph_memory(count: usize) -> MemoryRow {
    let order = (0..count).collect::<Vec<_>>();
    let mut runtime = reactor2::Runtime::new(NullAdapter);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(reactor2_grid(&order, None)).unwrap();
    black_box(runtime.graph());
    MemoryRow {
        frontend: "reactor2",
        workload: "keyed_grid_graph",
        objects: count + 1,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor1_tree_memory(count: usize) -> MemoryRow {
    let order = (0..count).collect::<Vec<_>>();
    let mut pump = reactor1::test::Pump::new(reactor1_runtime());
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    pump.mount_view(reactor1_tree(&order, None)).unwrap();
    MemoryRow {
        frontend: "reactor",
        workload: "tree_retained",
        objects: count + 1,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor1_tree_memory_detail(count: usize) -> (MemoryRow, usize, usize, usize) {
    let order = (0..count).collect::<Vec<_>>();
    let mut pump = reactor1::test::Pump::new(reactor1_runtime());
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    pump.mount_view(reactor1_tree(&order, None)).unwrap();
    let row = MemoryRow {
        frontend: "reactor",
        workload: "tree_retained",
        objects: count + 1,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    };
    (
        row,
        pump.retained_node_count(),
        pump.retained_node_size(),
        pump.retained_arena_slot_size(),
    )
}

fn reactor2_tree_memory(count: usize, custom_content: bool) -> MemoryRow {
    let mut runtime = reactor2_runtime();
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime
        .update(reactor2::TreeView::new().nodes((0..count).map(|index| {
            let node = reactor2::TreeNode::new(index, format!("Node {index}"));
            if custom_content {
                node.content(reactor2::TextBlock::new(format!("Content {index}")))
            } else {
                node
            }
        })))
        .unwrap();
    black_box(runtime.graph());
    MemoryRow {
        frontend: "reactor2",
        workload: if custom_content {
            "tree_content_retained"
        } else {
            "tree_retained"
        },
        objects: count + 1 + usize::from(custom_content) * count,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor2_tree_graph_memory(count: usize, custom_content: bool) -> MemoryRow {
    let mut runtime = reactor2::Runtime::new(NullAdapter);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime
        .update(reactor2::TreeView::new().nodes((0..count).map(|index| {
            let node = reactor2::TreeNode::new(index, format!("Node {index}"));
            if custom_content {
                node.content(reactor2::TextBlock::new(format!("Content {index}")))
            } else {
                node
            }
        })))
        .unwrap();
    black_box(runtime.graph());
    MemoryRow {
        frontend: "reactor2",
        workload: if custom_content {
            "tree_content_graph"
        } else {
            "tree_graph"
        },
        objects: count + 1 + usize::from(custom_content) * count,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor2_border_fixture(count: usize, text: bool, event: bool) -> reactor2::Visual {
    reactor2::Grid::new()
        .children((0..count).map(|index| {
            let border = reactor2::Border::new();
            let border = if text {
                border.automation_name(format!("Node {index}"))
            } else {
                border
            };
            let border = if event {
                border.on_pointer_released(|_| {})
            } else {
                border
            };
            reactor2::keyed(index, border)
        }))
        .into()
}

fn reactor2_unkeyed_border_fixture(count: usize) -> reactor2::Visual {
    reactor2::StackPanel::new()
        .children((0..count).map(|_| reactor2::Border::new().into()))
        .into()
}

fn reactor2_deep_chain(count: usize) -> reactor2::Visual {
    let mut current: reactor2::Visual = reactor2::Border::new().into();
    for _ in 1..count {
        current = reactor2::Border::new().content(current).into();
    }
    current
}

fn reactor2_empty_navigation_fixture(count: usize) -> reactor2::Visual {
    reactor2::Grid::new()
        .children((0..count).map(|index| reactor2::keyed(index, reactor2::NavigationView::new())))
        .into()
}

fn reactor2_filled_navigation_fixture(count: usize) -> reactor2::Visual {
    reactor2::Grid::new()
        .children((0..count).map(|index| {
            reactor2::keyed(
                index,
                reactor2::NavigationView::new()
                    .content(reactor2::Border::new())
                    .header(reactor2::Border::new())
                    .pane_custom_content(reactor2::Border::new())
                    .pane_footer(reactor2::Border::new()),
            )
        }))
        .into()
}

fn reactor2_graph_shape(
    fixture: &'static str,
    visual: impl FnOnce() -> reactor2::Visual,
) -> GraphMemoryRow {
    let mut runtime = reactor2::Runtime::new(NullAdapter);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(visual()).unwrap();
    runtime.release_test_scratch();
    let memory = runtime.graph().retained_memory();
    let bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before;
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations;
    black_box(runtime.graph());
    GraphMemoryRow {
        fixture,
        bytes,
        allocations,
        memory,
    }
}

fn reactor2_retirement_graph_shape() -> GraphMemoryRow {
    let mut runtime = reactor2::Runtime::new(NullAdapter);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(retirement_view(true)).unwrap();
    runtime.update(retirement_view(false)).unwrap();
    runtime.release_test_scratch();
    let memory = runtime.graph().retained_memory();
    let bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before;
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations;
    black_box(runtime.graph());
    GraphMemoryRow {
        fixture: "active_retirement",
        bytes,
        allocations,
        memory,
    }
}

fn reactor2_retained_memory_shapes(count: usize) -> Vec<GraphMemoryRow> {
    vec![
        reactor2_graph_shape("flat_keyed_empty", || {
            reactor2_border_fixture(count, false, false)
        }),
        reactor2_graph_shape("flat_unkeyed_empty", || {
            reactor2_unkeyed_border_fixture(count)
        }),
        reactor2_graph_shape("flat_string_property", || {
            reactor2_border_fixture(count, true, false)
        }),
        reactor2_graph_shape("flat_event", || reactor2_border_fixture(count, false, true)),
        reactor2_graph_shape("deep_one_child", || reactor2_deep_chain(count.min(128))),
        reactor2_graph_shape("many_empty_relations", || {
            reactor2_empty_navigation_fixture(count)
        }),
        reactor2_graph_shape("many_filled_relations", || {
            reactor2_filled_navigation_fixture(count / 4)
        }),
        reactor2_graph_shape("tree_structural", || {
            reactor2_tree(&(0..count).collect::<Vec<_>>(), None)
        }),
        reactor2_graph_shape("virtual_10k_zero_rows", || virtual_items_view(0)),
        reactor2_retirement_graph_shape(),
    ]
}

fn print_retained_memory_shapes(rows: &[GraphMemoryRow]) {
    let layout = rows.first().unwrap().memory;
    println!("release retained layout sizes");
    println!("{:<28} {:>10}", "RetainedGraph", layout.graph_size);
    println!("{:<28} {:>10}", "RetainedSlot", layout.slot_size);
    println!("{:<28} {:>10}", "RetainedObject", layout.object_size);
    println!("{:<28} {:>10}", "ObjectType", layout.object_type_size);
    println!("{:<28} {:>10}", "Option<Key>", layout.optional_key_size);
    println!(
        "{:<28} {:>10}",
        "Option<ElementRef>", layout.optional_reference_size
    );
    println!(
        "{:<28} {:>10}",
        "Option<ExitTransition>", layout.optional_transition_size
    );
    println!(
        "{:<28} {:>10}",
        "retained properties", layout.property_list_size
    );
    println!(
        "{:<28} {:>10}",
        "retained events", layout.retained_event_list_size
    );
    println!("{:<28} {:>10}", "relation Vec", layout.relation_list_size);
    println!(
        "{:<28} {:>10}",
        "optional virtual pointer", layout.optional_virtual_items_size
    );
    println!("{:<28} {:>10}", "RetainedRelation", layout.relation_size);
    println!(
        "{:<28} {:>10}",
        "RetainedRelationValue", layout.relation_value_size
    );
    println!("{:<28} {:>10}", "ObjectId", layout.object_id_size);
    println!("{:<28} {:>10}", "Key", layout.key_size);
    println!("{:<28} {:>10}", "Property", layout.property_size);
    println!("{:<28} {:>10}", "Event", layout.event_size);
    println!(
        "{:<28} {:>10}",
        "SharedList<Property>", layout.property_list_size
    );
    println!("{:<28} {:>10}", "SharedList<Event>", layout.event_list_size);
    println!(
        "{:<28} {:>10}",
        "RetainedVirtualItems", layout.virtual_items_size
    );
    println!(
        "{:<28} {:>10}",
        "RetainedRetirement", layout.retirement_size
    );
    println!("{:<28} {:>10}", "HashMap header", layout.hash_map_size);

    println!("\nretained graph allocation attribution");
    println!(
        "{:<24} {:>7} {:>10} {:>12} {:>12} {:>12} {:>12} {:>10}",
        "fixture", "objects", "slot cap", "allocator", "slots", "relations", "children", "residual"
    );
    for row in rows {
        let memory = row.memory;
        let known = memory.slot_bytes
            + memory.relation_bytes
            + memory.child_bytes
            + memory.free_bytes
            + memory.virtual_bytes
            + memory.retirement_node_bytes;
        println!(
            "{:<24} {:>7} {:>10} {:>12} {:>12} {:>12} {:>12} {:>10}",
            row.fixture,
            memory.live_objects,
            memory.slot_capacity,
            row.bytes,
            memory.slot_bytes,
            memory.relation_bytes,
            memory.child_bytes,
            row.bytes.saturating_sub(known as u64)
        );
    }

    println!("\nretained graph shape details");
    println!(
        "{:<24} {:>10} {:>10} {:>10} {:>10} {:>8} {:>8} {:>10}",
        "fixture", "bytes/obj", "relations", "rel cap", "child cap", "keys", "props", "events"
    );
    for row in rows {
        let memory = row.memory;
        println!(
            "{:<24} {:>10.1} {:>10} {:>10} {:>10} {:>8} {:>8} {:>10}",
            row.fixture,
            row.bytes as f64 / memory.live_objects as f64,
            memory.relation_entries,
            memory.relation_capacity,
            memory.child_capacity,
            memory.keyed_objects,
            memory.property_entries,
            memory.event_entries
        );
    }
    println!("\nmount allocation counts");
    for row in rows {
        println!("{:<24} {:>12}", row.fixture, row.allocations);
    }

    println!("\nvirtual and retirement state");
    println!(
        "{:<24} {:>10} {:>14} {:>12} {:>16}",
        "fixture", "virtuals", "virtual bytes", "retirements", "retired node cap"
    );
    for row in rows
        .iter()
        .filter(|row| row.memory.virtual_items != 0 || row.memory.retirement_entries != 0)
    {
        println!(
            "{:<24} {:>10} {:>14} {:>12} {:>16}",
            row.fixture,
            row.memory.virtual_items,
            row.memory.virtual_bytes,
            row.memory.retirement_entries,
            row.memory.retirement_node_capacity
        );
    }
}

fn lifecycle_root_view(reference: &reactor2::ElementRef, cycle: usize) -> reactor2::Visual {
    reactor2::Grid::new()
        .children([reactor2::keyed(
            "child",
            reactor2::Button::new()
                .automation_name(format!("cycle-{cycle}"))
                .element_ref(reference)
                .on_click(|| {})
                .content(reactor2::TextBlock::new(format!("value-{cycle}"))),
        )])
        .into()
}

fn lifecycle_retirement_view(visible: bool) -> reactor2::Visual {
    reactor2::Grid::new()
        .children(
            visible
                .then(|| {
                    [
                        reactor2::keyed(
                            "first",
                            reactor2::Button::new().exit_fade(Duration::from_secs(1)),
                        ),
                        reactor2::keyed(
                            "second",
                            reactor2::Button::new().exit_fade(Duration::from_secs(1)),
                        ),
                    ]
                })
                .into_iter()
                .flatten(),
        )
        .into()
}

fn print_lifecycle_checkpoint(
    suite: &str,
    iteration: usize,
    baseline_bytes: u64,
    baseline_allocations: u64,
    state: reactor2::ComponentHostState,
) {
    println!(
        "{{\"benchmark\":\"reactor2-lifecycle\",\"suite\":\"{suite}\",\
         \"iteration\":{iteration},\"rust_live_delta\":{},\"allocations\":{},\
         \"graph_objects\":{},\"scopes\":{},\"scope_slots\":{},\"free_scopes\":{},\
         \"effects\":{},\"tasks\":{},\"contexts\":{},\"context_consumers\":{},\
         \"virtual_rows\":{},\"queued_messages\":{},\"queue_closed\":{},\
         \"retirements\":{}}}",
        allocator::CURRENT_BYTES
            .load(Ordering::Relaxed)
            .saturating_sub(baseline_bytes),
        allocator::ALLOCATIONS
            .load(Ordering::Relaxed)
            .saturating_sub(baseline_allocations),
        state.graph_objects,
        state.live_scopes,
        state.scope_slots,
        state.free_scopes,
        state.effects,
        state.tasks,
        state.contexts,
        state.context_consumers,
        state.virtual_rows,
        state.queued_messages,
        state.queue_closed,
        state.retirements,
    );
}

fn run_recording_lifecycle_stress(iterations: usize, checkpoint: usize) {
    let baseline_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let baseline_allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let mut runtime = reactor2_runtime();
    runtime.update(reactor2::Grid::new()).unwrap();
    for iteration in 1..=iterations {
        let reference = reactor2::ElementRef::default();
        runtime
            .update(lifecycle_root_view(&reference, iteration))
            .unwrap();
        assert!(reference.get().is_some());
        runtime.update(reactor2::Grid::new()).unwrap();
        assert_eq!(reference.get(), None);
        assert_eq!(runtime.graph().object_count(), 1);
        assert_eq!(runtime.graph().retired_count(), 0);
        assert_eq!(runtime.adapter().object_count(), 1);
        assert_eq!(runtime.adapter().retirement_count(), 0);
        assert_eq!(runtime.adapter().realization_count(), 0);
        assert_eq!(runtime.adapter().queued_native_event_count(), 0);
        if iteration % checkpoint == 0 || iteration == iterations {
            let memory = runtime.graph().retained_memory();
            println!(
                "{{\"benchmark\":\"reactor2-lifecycle\",\"suite\":\"root\",\
                 \"iteration\":{iteration},\"rust_live_delta\":{},\"allocations\":{},\
                 \"graph_objects\":1,\"slot_len\":{},\"slot_capacity\":{},\
                 \"free_slots\":{},\"retirements\":0,\"realized_rows\":0,\
                 \"queued_events\":0}}",
                allocator::CURRENT_BYTES
                    .load(Ordering::Relaxed)
                    .saturating_sub(baseline_bytes),
                allocator::ALLOCATIONS
                    .load(Ordering::Relaxed)
                    .saturating_sub(baseline_allocations),
                memory.slot_len,
                memory.slot_capacity,
                memory.free_len,
            );
        }
    }
    drop(runtime);
    println!(
        "{{\"benchmark\":\"reactor2-lifecycle\",\"suite\":\"root_teardown\",\
         \"iteration\":{iterations},\"rust_live_delta\":{}}}",
        allocator::CURRENT_BYTES
            .load(Ordering::Relaxed)
            .saturating_sub(baseline_bytes)
    );

    let baseline_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let baseline_allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let services = Arc::new(LifecycleServices::default());
    let cleanups = Arc::new(AtomicUsize::new(0));
    for iteration in 1..=iterations {
        {
            let context = Rc::new(reactor2::Context::new(0usize));
            let task = Arc::new(Mutex::new(None));
            let input = LifecycleInput {
                cleanups: Arc::clone(&cleanups),
                context: Rc::clone(&context),
                task: Arc::clone(&task),
            };
            let mut host = reactor2::ComponentHost::mount_with_services(
                reactor2::RecordingAdapter::default(),
                services.clone(),
                [reactor2::component::<LifecycleComponent>(
                    "lifecycle",
                    input,
                )],
            )
            .unwrap();
            let sender = host
                .sender::<LifecycleComponent>(&reactor2::Key::from("lifecycle"))
                .unwrap();
            let completion = sender.completion();
            let reference = host.reference(&reactor2::Key::from("lifecycle")).unwrap();
            let object = reference.get().unwrap();
            let event = host.runtime().graph().events(object).unwrap()[0].clone();
            assert!(sender.send(iteration));
            host.drain(1).unwrap();
            host.set_context(&context, iteration).unwrap();
            let state = host.test_state();
            assert_eq!(state.live_scopes, 1);
            assert_eq!(state.effects, 1);
            assert_eq!(state.tasks, 2);
            assert_eq!(state.contexts, 1);
            assert_eq!(state.context_consumers, 1);
            assert_eq!(state.queued_messages, 0);
            drop(host);
            assert_eq!(reference.get(), None);
            assert!(!sender.send(iteration));
            assert!(!completion.complete(iteration));
            let reactor2::EventValue::Unit(callback) = event.value else {
                unreachable!()
            };
            callback.call(());
            assert_eq!(
                task.lock().unwrap().as_ref().unwrap().status(),
                reactor2::ComponentTaskStatus::Cancelled
            );
        }
        services.run_background();
        if iteration % checkpoint == 0 || iteration == iterations {
            print_lifecycle_checkpoint(
                "component_teardown",
                iteration,
                baseline_bytes,
                baseline_allocations,
                reactor2::ComponentHostState {
                    queue_closed: true,
                    ..Default::default()
                },
            );
        }
    }
    assert_eq!(cleanups.load(Ordering::Relaxed), iterations * 2);
    assert_eq!(
        services.cancelled_timers.load(Ordering::Relaxed),
        iterations
    );

    let baseline_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let baseline_allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let mut runtime = reactor2_runtime();
    runtime.update(lifecycle_retirement_view(true)).unwrap();
    for iteration in 1..=iterations {
        let root = runtime.graph().root().unwrap();
        let retiring = runtime
            .graph()
            .children(root, reactor2::RelationId::Children)
            .unwrap()
            .to_vec();
        runtime.update(lifecycle_retirement_view(false)).unwrap();
        assert_eq!(runtime.graph().retired_count(), 2);
        for object in retiring.iter().rev() {
            assert!(runtime.complete_retirement(*object));
        }
        runtime.dispatch_native_events().unwrap();
        for object in &retiring {
            assert!(!runtime.complete_retirement(*object));
        }
        assert_eq!(runtime.graph().retired_count(), 0);
        assert_eq!(runtime.adapter().retirement_count(), 0);
        runtime.update(lifecycle_retirement_view(true)).unwrap();
        if iteration % checkpoint == 0 || iteration == iterations {
            let memory = runtime.graph().retained_memory();
            println!(
                "{{\"benchmark\":\"reactor2-lifecycle\",\"suite\":\"retirement\",\
                 \"iteration\":{iteration},\"rust_live_delta\":{},\"allocations\":{},\
                 \"graph_objects\":{},\"slot_len\":{},\"slot_capacity\":{},\
                 \"free_slots\":{},\"retirements\":0,\"queued_events\":0}}",
                allocator::CURRENT_BYTES
                    .load(Ordering::Relaxed)
                    .saturating_sub(baseline_bytes),
                allocator::ALLOCATIONS
                    .load(Ordering::Relaxed)
                    .saturating_sub(baseline_allocations),
                runtime.graph().object_count(),
                memory.slot_len,
                memory.slot_capacity,
                memory.free_len,
            );
        }
    }
    drop(runtime);

    let baseline_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let baseline_allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let mut runtime = reactor2_runtime();
    runtime.update(virtual_items_view(0)).unwrap();
    let collection = runtime.graph().root().unwrap();
    for iteration in 1..=iterations {
        let source_revision = runtime
            .update(virtual_items_view(u64::try_from(iteration).unwrap()))
            .unwrap()
            .into_iter()
            .find_map(|mutation| match mutation {
                reactor2::Mutation::SetVirtualSource {
                    source_revision, ..
                } => Some(source_revision),
                _ => None,
            })
            .unwrap();
        runtime.queue_realization(reactor2::RealizationRequest::Realize {
            collection,
            container: reactor2::RealizedContainer(1),
            index: iteration % 10_000,
            source_revision,
        });
        runtime.dispatch_native_events().unwrap();
        runtime.queue_realization(reactor2::RealizationRequest::Recycle {
            collection,
            container: reactor2::RealizedContainer(1),
            source_revision,
        });
        runtime.dispatch_native_events().unwrap();
        runtime.queue_realization(reactor2::RealizationRequest::Realize {
            collection,
            container: reactor2::RealizedContainer(1),
            index: (iteration + 1) % 10_000,
            source_revision,
        });
        runtime.queue_realization(reactor2::RealizationRequest::Cancel {
            collection,
            container: reactor2::RealizedContainer(1),
            source_revision,
        });
        runtime.dispatch_native_events().unwrap();
        runtime.update(reactor2::ItemsRepeater::new()).unwrap();
        runtime.dispatch_native_events().unwrap();
        assert_eq!(runtime.graph().object_count(), 1);
        assert_eq!(runtime.graph().retired_count(), 0);
        assert_eq!(runtime.adapter().object_count(), 1);
        assert_eq!(runtime.adapter().realization_count(), 0);
        assert_eq!(runtime.adapter().queued_native_event_count(), 0);
        runtime
            .update(virtual_items_view(u64::try_from(iteration).unwrap()))
            .unwrap();
        if iteration % checkpoint == 0 || iteration == iterations {
            println!(
                "{{\"benchmark\":\"reactor2-lifecycle\",\"suite\":\"virtual\",\
                 \"iteration\":{iteration},\"rust_live_delta\":{},\"allocations\":{},\
                 \"graph_objects\":1,\"retirements\":0,\"realized_rows\":0,\
                 \"queued_events\":0}}",
                allocator::CURRENT_BYTES
                    .load(Ordering::Relaxed)
                    .saturating_sub(baseline_bytes),
                allocator::ALLOCATIONS
                    .load(Ordering::Relaxed)
                    .saturating_sub(baseline_allocations),
            );
        }
    }
}

fn reactor2_fixture_memory<A>(
    frontend: &'static str,
    workload: &'static str,
    objects: usize,
    mut runtime: reactor2::Runtime<A>,
    visual: impl FnOnce() -> reactor2::Visual,
) -> MemoryRow
where
    A: reactor2::Adapter,
    A::Error: std::fmt::Debug,
{
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(visual()).unwrap();
    runtime.release_test_scratch();
    black_box(runtime.graph());
    MemoryRow {
        frontend,
        workload,
        objects,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor2_tree_declaration_memory(count: usize) -> MemoryRow {
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let declaration = reactor2_tree(&(0..count).collect::<Vec<_>>(), None);
    let bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before;
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations;
    black_box(declaration);
    MemoryRow {
        frontend: "declaration",
        workload: "tree_declarations",
        objects: count + 1,
        bytes,
        allocations,
    }
}

fn reactor2_memory_attribution(count: usize) -> [MemoryRow; 7] {
    [
        reactor1_tree_memory(count),
        reactor2_tree_declaration_memory(count),
        reactor2_fixture_memory(
            "graph",
            "flat_border_base",
            count + 1,
            reactor2::Runtime::new(NullAdapter),
            || reactor2_border_fixture(count, false, false),
        ),
        reactor2_fixture_memory(
            "graph",
            "flat_border_string",
            count + 1,
            reactor2::Runtime::new(NullAdapter),
            || reactor2_border_fixture(count, true, false),
        ),
        reactor2_fixture_memory(
            "graph",
            "flat_border_event",
            count + 1,
            reactor2::Runtime::new(NullAdapter),
            || reactor2_border_fixture(count, false, true),
        ),
        reactor2_fixture_memory(
            "graph",
            "tree_graph",
            count + 1,
            reactor2::Runtime::new(NullAdapter),
            || reactor2_tree(&(0..count).collect::<Vec<_>>(), None),
        ),
        reactor2_fixture_memory(
            "recorded",
            "tree_recorded",
            count + 1,
            reactor2_runtime(),
            || reactor2_tree(&(0..count).collect::<Vec<_>>(), None),
        ),
    ]
}

fn reactor2_component(count: usize, effect: bool, samples: usize, batch: usize) -> Row {
    let mut adapter = reactor2::RecordingAdapter::default();
    adapter.record_batches(false);
    adapter.validate_batches(false);
    let mut components = reactor2::ComponentHost::mount(
        adapter,
        (0..count).map(|index| reactor2::component::<BenchComponent>(index, effect)),
    )
    .unwrap();
    let sender = components
        .sender::<BenchComponent>(&reactor2::Key::from(count / 2))
        .unwrap();
    let perf = measure(samples, batch, || {
        assert!(sender.send(true));
        black_box(components.drain(1).unwrap());
    });
    Row {
        frontend: "reactor2",
        workload: if effect {
            "component_effect"
        } else {
            "component_isolated"
        },
        objects: count,
        perf,
    }
}

fn reactor2_component_root_replace(count: usize, samples: usize, batch: usize) -> Row {
    let mut adapter = reactor2::RecordingAdapter::default();
    adapter.record_batches(false);
    adapter.validate_batches(false);
    let mut components = reactor2::ComponentHost::mount(
        adapter,
        (0..count).map(|index| reactor2::component::<RootSwitch>(index, ())),
    )
    .unwrap();
    let sender = components
        .sender::<RootSwitch>(&reactor2::Key::from(count / 2))
        .unwrap();
    let perf = measure(samples, batch, || {
        assert!(sender.send(()));
        black_box(components.drain(1).unwrap());
    });
    Row {
        frontend: "reactor2",
        workload: "component_replace",
        objects: count,
        perf,
    }
}

fn reactor2_component_remove(count: usize, samples: usize) -> Row {
    let perf = measure_prepared(
        samples,
        1,
        || {
            let mut adapter = reactor2::RecordingAdapter::default();
            adapter.record_batches(false);
            adapter.validate_batches(false);
            reactor2::ComponentHost::mount(
                adapter,
                (0..count).map(|index| reactor2::component::<BenchComponent>(index, false)),
            )
            .unwrap()
        },
        |mut components| {
            components.remove(&reactor2::Key::from(count / 2)).unwrap();
            black_box(components);
        },
    );
    Row {
        frontend: "reactor2",
        workload: "component_remove",
        objects: count,
        perf,
    }
}

fn reactor2_component_memory(count: usize, effect: bool) -> MemoryRow {
    let mut adapter = reactor2::RecordingAdapter::default();
    adapter.record_batches(false);
    adapter.validate_batches(false);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let components = reactor2::ComponentHost::mount(
        adapter,
        (0..count).map(|index| reactor2::component::<BenchComponent>(index, effect)),
    )
    .unwrap();
    black_box(&components);
    MemoryRow {
        frontend: "reactor2",
        workload: if effect {
            "component_effect"
        } else {
            "component_idle"
        },
        objects: count,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor2_context(count: usize, broad: bool, samples: usize) -> Row {
    let mut adapter = reactor2::RecordingAdapter::default();
    adapter.record_batches(false);
    adapter.validate_batches(false);
    let context = Rc::new(reactor2::Context::new(false));
    let mut components = reactor2::ComponentHost::mount(
        adapter,
        (0..count).map(|index| {
            reactor2::component::<ContextComponent>(
                index,
                ContextInput {
                    context: Rc::clone(&context),
                    subscribe: broad || index == count / 2,
                },
            )
        }),
    )
    .unwrap();
    let mut value = false;
    let perf = measure(samples, 1, || {
        value = !value;
        black_box(components.set_context(&context, value).unwrap());
    });
    Row {
        frontend: "reactor2",
        workload: if broad {
            "context_broad"
        } else {
            "context_isolated"
        },
        objects: count,
        perf,
    }
}

fn reactor2_recursive(depth: usize, fanout: usize, samples: usize) -> RecursiveRow {
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let mut components = reactor2::ComponentHost::mount(
        reactor2::RecordingAdapter::default(),
        [reactor2::component::<Branch>(
            "root",
            BranchInput { depth, fanout },
        )],
    )
    .unwrap();
    let retained_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before;
    components.validate_batches(false);
    let mut path = Vec::with_capacity(depth + 1);
    path.push(reactor2::Key::from("root"));
    path.extend((0..depth).map(|_| reactor2::Key::from("0")));
    let sender = components.sender_at::<Branch>(&path).unwrap();
    let mut mutations = 0;
    let perf = measure(samples, 1, || {
        assert!(sender.send(()));
        mutations += components.drain(usize::MAX).unwrap().mutations;
    });
    let scopes = (0..=depth)
        .map(|level| fanout.pow(level as u32))
        .sum::<usize>();
    RecursiveRow {
        allocations: perf.allocations,
        bytes: perf.bytes,
        bytes_per_scope: retained_bytes as f64 / scopes as f64,
        depth,
        fanout,
        median_ns: perf.median_ns,
        mutations: mutations as f64 / (samples + 8) as f64,
        p95_ns: perf.p95_ns,
        retained_bytes,
        scopes,
    }
}

fn argument(name: &str, default: usize) -> usize {
    let arguments = std::env::args().collect::<Vec<_>>();
    arguments
        .windows(2)
        .find(|pair| pair[0] == name)
        .and_then(|pair| pair[1].parse().ok())
        .unwrap_or(default)
}

fn has_argument(name: &str) -> bool {
    std::env::args().any(|argument| argument == name)
}

fn argument_value(name: &str) -> Option<String> {
    let arguments = std::env::args().collect::<Vec<_>>();
    arguments
        .windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn profile_virtual_source_replace(iterations: usize) {
    let (mut runtime, _) = virtual_items_realized_runtime(8);
    let bytes = allocator::allocated_bytes();
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let started = Instant::now();
    let mut mutations = 0;
    for revision in 1..=iterations {
        mutations += runtime
            .update(virtual_items_view(revision as u64))
            .unwrap()
            .len();
        runtime.dispatch_native_events().unwrap();
    }
    let elapsed = started.elapsed();
    assert_eq!(runtime.graph().object_count(), 9);
    println!(
        "{{\"benchmark\":\"reactor2-profile\",\
         \"workload\":\"virtual_source_replace_10k_eight_rows\",\
         \"iterations\":{iterations},\"elapsed_us\":{:.3},\
         \"ns_per_iteration\":{:.3},\"mutations_per_iteration\":{:.3},\
         \"bytes_per_iteration\":{:.3},\"allocations_per_iteration\":{:.3}}}",
        elapsed.as_secs_f64() * 1_000_000.0,
        elapsed.as_nanos() as f64 / iterations as f64,
        mutations as f64 / iterations as f64,
        (allocator::allocated_bytes() - bytes) as f64 / iterations as f64,
        (allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations) as f64 / iterations as f64,
    );
}

fn run_profile_workload(workload: &str, iterations: usize) -> Result<(), String> {
    if iterations == 0 {
        return Err("--profile-iterations must be greater than zero".to_string());
    }
    match workload {
        "reactor-100k-no-change" => {
            print_perf_rows([bench_reactor1_100k(false, iterations)]);
        }
        "reactor2-100k-no-change" => {
            print_perf_rows([bench_reactor2_100k(false, iterations)]);
        }
        "reactor-100k-one-changed" => {
            print_perf_rows([bench_reactor1_100k(true, iterations)]);
        }
        "reactor2-100k-one-changed" => {
            print_perf_rows([bench_reactor2_100k(true, iterations)]);
        }
        "reactor2-context-broad" => {
            print_perf_rows([reactor2_context(
                argument("--component-count", 16_384),
                true,
                iterations,
            )]);
        }
        "reactor2-virtual-source-replace" => profile_virtual_source_replace(iterations),
        _ => return Err(format!("unknown --profile-workload value: {workload}")),
    }
    Ok(())
}

fn print_perf_rows(rows: impl IntoIterator<Item = Row>) {
    println!(
        "{:<9} {:<20} {:>7} {:>14} {:>14} {:>14} {:>12}",
        "frontend", "workload", "objects", "median ns", "p95 ns", "bytes/op", "allocs/op"
    );
    for row in rows {
        println!(
            "{:<9} {:<20} {:>7} {:>14.1} {:>14.1} {:>14.1} {:>12.2}",
            row.frontend,
            row.workload,
            row.objects,
            row.perf.median_ns,
            row.perf.p95_ns,
            row.perf.bytes,
            row.perf.allocations
        );
    }
}

fn print_memory_rows(rows: &[MemoryRow]) {
    println!(
        "{:<12} {:<22} {:>7} {:>16} {:>14} {:>16}",
        "layer", "fixture", "objects", "retained bytes", "bytes/object", "allocations"
    );
    for row in rows {
        println!(
            "{:<12} {:<22} {:>7} {:>16} {:>14.1} {:>16}",
            row.frontend,
            row.workload,
            row.objects,
            row.bytes,
            row.bytes as f64 / row.objects as f64,
            row.allocations
        );
    }
}

fn print_memory_attribution(rows: &[MemoryRow]) {
    let bytes = |workload| {
        rows.iter()
            .find(|row| row.workload == workload)
            .unwrap()
            .bytes
    };
    let declarations = bytes("tree_declarations");
    let graph = bytes("tree_graph");
    let recorded = bytes("tree_recorded");
    println!("\n513-node Reactor2 additive attribution");
    println!("{:<30} {:>16}", "category", "bytes");
    println!(
        "{:<30} {:>16}",
        "declarations held by fixture", declarations
    );
    println!("{:<30} {:>16}", "retained graph", graph);
    println!(
        "{:<30} {:>16}",
        "recording adapter overhead",
        recorded - graph
    );
    println!("{:<30} {:>16}", "recorded runtime total", recorded);
    println!(
        "{:<30} {:>16}",
        "fixture + recorded total",
        declarations + recorded
    );
}

fn main() {
    let samples = argument("--samples", 80);
    let batch = argument("--batch", 8);
    if let Some(workload) = argument_value("--profile-workload") {
        if let Err(error) =
            run_profile_workload(&workload, argument("--profile-iterations", samples))
        {
            eprintln!("test-reactor2-bench: {error}");
            std::process::exit(2);
        }
        return;
    }
    if has_argument("--virtual-memory") {
        print_memory_rows(&[reactor2_virtual_memory()]);
        return;
    }
    if has_argument("--retained-memory") {
        let count = argument("--count", 512);
        let (reactor, retained_nodes, node_size, slot_size) = reactor1_tree_memory_detail(count);
        println!(
            "Reactor Tree fixture: {} logical entries, {} retained graph nodes, {} bytes, \
             Node {} bytes, arena slot {} bytes\n",
            count + 1,
            retained_nodes,
            reactor.bytes,
            node_size,
            slot_size
        );
        print_retained_memory_shapes(&reactor2_retained_memory_shapes(count));
        return;
    }
    if has_argument("--lifecycle-stress") {
        run_recording_lifecycle_stress(
            argument("--iterations", 1_000),
            argument("--checkpoint", 100).max(1),
        );
        return;
    }
    if has_argument("--virtual-items") {
        print_perf_rows([
            bench_reactor2_virtual_create(samples, batch),
            bench_reactor2_virtual_source_replace(samples, batch),
            bench_reactor2_virtual_realize(samples, batch),
            bench_reactor2_virtual_update(samples, batch),
            bench_reactor2_virtual_recycle(samples, batch),
        ]);
        println!();
        print_memory_rows(&[reactor2_virtual_memory()]);
        println!("\nvirtual mutation trace");
        for (workload, mutations, realized) in reactor2_virtual_mutation_counts() {
            println!("{workload:<24} {mutations:>12} {realized:>15}");
        }
        return;
    }
    if has_argument("--architecture-gate") {
        let gate_samples = argument("--gate-samples", 6);
        println!("architecture decision gate: 100k keyed updates (2 x 50k graphs)");
        print_perf_rows([
            bench_reactor1_100k(false, gate_samples),
            bench_reactor2_100k(false, gate_samples),
            bench_reactor1_100k(true, gate_samples),
            bench_reactor2_100k(true, gate_samples),
        ]);

        println!("\narchitecture decision gate: 513-node retained memory attribution");
        let memory_rows = reactor2_memory_attribution(512);
        print_memory_rows(&memory_rows);
        print_memory_attribution(&memory_rows);

        let component_count = argument("--component-count", 16_384);
        println!("\narchitecture decision gate: large component graph operations");
        print_perf_rows([
            reactor2_component_root_replace(component_count, gate_samples, 1),
            reactor2_component_remove(component_count, gate_samples),
        ]);
        return;
    }
    if has_argument("--keyed-scaling") {
        let mut rows = Vec::new();
        for count in [512, 1_024, 10_000] {
            let order = (0..count).collect::<Vec<_>>();
            rows.push(bench_reactor2(
                "keyed_no_change",
                count,
                reactor2_grid(&order, None),
                reactor2_grid(&order, None),
                samples,
                batch,
            ));
            rows.push(bench_reactor2(
                "keyed_one_changed",
                count,
                reactor2_grid(&order, None),
                reactor2_grid(&order, Some(0)),
                samples,
                batch,
            ));
        }
        print_perf_rows(rows);
        return;
    }
    let count = argument("--count", 512);
    let order = (0..count).collect::<Vec<_>>();
    let mut rotated = order.clone();
    rotated.rotate_left(1);
    let mut reversed = order.clone();
    reversed.reverse();

    let rows = [
        bench_reactor1(
            "keyed_no_change",
            count,
            reactor1_grid(&order, None),
            reactor1_grid(&order, None),
            samples,
            batch,
        ),
        bench_reactor2(
            "keyed_no_change",
            count,
            reactor2_grid(&order, None),
            reactor2_grid(&order, None),
            samples,
            batch,
        ),
        bench_reactor1(
            "keyed_one_changed",
            count,
            reactor1_grid(&order, None),
            reactor1_grid(&order, Some(0)),
            samples,
            batch,
        ),
        bench_reactor2(
            "keyed_one_changed",
            count,
            reactor2_grid(&order, None),
            reactor2_grid(&order, Some(0)),
            samples,
            batch,
        ),
        bench_reactor1(
            "keyed_rotate_one",
            count,
            reactor1_grid(&order, None),
            reactor1_grid(&rotated, None),
            samples,
            batch,
        ),
        bench_reactor2(
            "keyed_rotate_one",
            count,
            reactor2_grid(&order, None),
            reactor2_grid(&rotated, None),
            samples,
            batch,
        ),
        bench_reactor1(
            "keyed_reverse",
            count,
            reactor1_grid(&order, None),
            reactor1_grid(&reversed, None),
            samples,
            batch,
        ),
        bench_reactor2(
            "keyed_reverse",
            count,
            reactor2_grid(&order, None),
            reactor2_grid(&reversed, None),
            samples,
            batch,
        ),
        bench_reactor1(
            "tree_rotate_one",
            count,
            reactor1_tree(&order, None),
            reactor1_tree(&rotated, None),
            samples,
            batch,
        ),
        bench_reactor2(
            "tree_rotate_one",
            count,
            reactor2_tree(&order, None),
            reactor2_tree(&rotated, None),
            samples,
            batch,
        ),
        reactor2_component(count, false, samples, batch),
        reactor2_component(count, true, samples, batch),
        reactor2_component_root_replace(count, samples, batch),
        reactor2_context(count, false, samples),
        reactor2_context(count, true, samples),
        bench_reactor2_retirement_initiation(samples, batch),
        bench_reactor2_retirement_completion(samples, batch),
        bench_reactor2_retirement_remount(samples, batch),
        bench_reactor2_virtual_create(samples, batch),
        bench_reactor2_virtual_source_replace(samples, batch),
        bench_reactor2_virtual_realize(samples, batch),
        bench_reactor2_virtual_update(samples, batch),
        bench_reactor2_virtual_recycle(samples, batch),
    ];

    print_perf_rows(rows);

    println!();
    let memory_rows = [
        reactor1_memory(count),
        reactor2_graph_memory(count),
        reactor2_memory(count),
        reactor1_tree_memory(count),
        reactor2_tree_graph_memory(count, false),
        reactor2_tree_memory(count, false),
        reactor2_tree_graph_memory(count, true),
        reactor2_tree_memory(count, true),
        reactor2_component_memory(count, false),
        reactor2_component_memory(count, true),
        reactor2_virtual_memory(),
    ];
    print_memory_rows(&memory_rows);

    println!("\nvirtual mutation trace");
    println!(
        "{:<24} {:>12} {:>15}",
        "workload", "mutations", "realized rows"
    );
    for (workload, mutations, realized) in reactor2_virtual_mutation_counts() {
        println!("{workload:<24} {mutations:>12} {realized:>15}");
    }

    println!("\nrecursive component scaling");
    println!(
        "{:>5} {:>6} {:>8} {:>15} {:>13} {:>12} {:>12} {:>11} {:>12} {:>12}",
        "depth",
        "fanout",
        "scopes",
        "retained bytes",
        "bytes/scope",
        "median ns",
        "p95 ns",
        "allocs/op",
        "bytes/op",
        "mutations/op"
    );
    for row in [
        reactor2_recursive(3, 8, samples),
        reactor2_recursive(4, 8, samples),
        reactor2_recursive(7, 4, samples),
    ] {
        println!(
            "{:>5} {:>6} {:>8} {:>15} {:>13.1} {:>12.1} {:>12.1} {:>11.2} {:>12.1} {:>12.2}",
            row.depth,
            row.fanout,
            row.scopes,
            row.retained_bytes,
            row.bytes_per_scope,
            row.median_ns,
            row.p95_ns,
            row.allocations,
            row.bytes,
            row.mutations
        );
    }
}
