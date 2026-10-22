use std::hint::black_box;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use windows_reactor as reactor;

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
    memory: reactor::RetainedMemory,
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

#[derive(Clone, PartialEq)]
struct BranchInput {
    depth: usize,
    fanout: usize,
}

struct Branch(usize);

impl reactor::Component for Branch {
    type Input = BranchInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self(0)
    }

    fn update(&mut self, (): Self::Message, _context: &reactor::ComponentContext<Self>) {
        self.0 += 1;
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        if input.depth == 0 {
            return reactor::TextBlock::new().text(self.0.to_string()).into();
        }
        reactor::StackPanel::new()
            .children(
                (0..input.fanout)
                    .map(|index| {
                        reactor::component::<Self>(
                            index.to_string(),
                            BranchInput {
                                depth: input.depth - 1,
                                fanout: input.fanout,
                            },
                        )
                        .into()
                    })
                    .collect::<Vec<reactor::View>>(),
            )
            .into()
    }
}

#[derive(Clone)]
struct ContextInput {
    context: Rc<reactor::Context<bool>>,
    subscribe: bool,
}

impl PartialEq for ContextInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context) && self.subscribe == other.subscribe
    }
}

struct ContextComponent;

impl reactor::Component for ContextComponent {
    type Input = ContextInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut reactor::ViewContext<Self>) -> reactor::View {
        let value = if input.subscribe {
            context.use_context(&input.context)
        } else {
            false
        };
        reactor::TextBlock::new()
            .text(if value { "on" } else { "off" })
            .into()
    }
}

impl reactor::Component for BenchComponent {
    type Input = bool;
    type Message = bool;

    fn create(effect: &bool, _context: &reactor::ComponentContext<Self>) -> Self {
        Self {
            active: false,
            effect: *effect,
        }
    }

    fn update(&mut self, toggle: bool, _context: &reactor::ComponentContext<Self>) {
        if toggle {
            self.active = !self.active;
        }
    }

    fn view(&self, _input: &bool, context: &mut reactor::ViewContext<Self>) -> reactor::View {
        if self.effect {
            context.use_effect("bench", self.active, || None);
        }
        reactor::TextBlock::new()
            .text(if self.active { "on" } else { "off" })
            .into()
    }
}

struct RootSwitch(bool);

impl reactor::Component for RootSwitch {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &reactor::ComponentContext<Self>) {
        self.0 = !self.0;
    }

    fn view(
        &self,
        _input: &Self::Input,
        _context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        if self.0 {
            reactor::Border::new().into()
        } else {
            reactor::TextBlock::new().text("root").into()
        }
    }
}

#[derive(Default)]
struct NullAdapter;

impl reactor::Adapter for NullAdapter {
    fn apply(&mut self, _mutations: &[reactor::Mutation]) {}
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

fn reactor_runtime() -> reactor::Runtime<reactor::RecordingAdapter> {
    let mut adapter = reactor::RecordingAdapter::default();
    adapter.record_batches(false);
    reactor::Runtime::new(adapter)
}

fn reactor_grid(order: &[usize], changed: Option<usize>) -> reactor::View {
    reactor::Grid::new()
        .keyed_children(order.iter().map(|index| {
            reactor::keyed(
                *index,
                reactor::TextBlock::new().text(if Some(*index) == changed {
                    format!("Changed {index}")
                } else {
                    format!("Cell {index}")
                }),
            )
        }))
        .into()
}

fn reactor_tree(order: &[usize], changed: Option<usize>) -> reactor::View {
    reactor::TreeView::new()
        .nodes(order.iter().map(|index| {
            reactor::TreeNode::new(
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

fn bench_reactor(
    workload: &'static str,
    objects: usize,
    first: reactor::View,
    second: reactor::View,
    samples: usize,
    batch: usize,
) -> Row {
    let mut runtime = reactor_runtime();
    runtime.update(first.clone());
    let mut flip = false;
    let perf = measure(samples, batch, || {
        runtime.update(if flip { first.clone() } else { second.clone() });
        flip = !flip;
    });
    Row {
        frontend: "reactor",
        workload,
        objects,
        perf,
    }
}

fn bench_reactor_100k(one_changed: bool, samples: usize) -> Row {
    let count = 50_000;
    let order = (0..count).collect::<Vec<_>>();
    let first = reactor_grid(&order, None);
    let changed = reactor_grid(&order, one_changed.then_some(0));
    let mut runtimes = [reactor_runtime(), reactor_runtime()];
    for runtime in &mut runtimes {
        runtime.update(first.clone());
    }
    let mut flip = false;
    let perf = measure(samples, 1, || {
        runtimes[0].update(if flip { first.clone() } else { changed.clone() });
        runtimes[1].update(first.clone());
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

fn retirement_view(visible: bool) -> reactor::View {
    reactor::Grid::new()
        .keyed_children(visible.then(|| {
            reactor::keyed(
                "retiring",
                reactor::Button::new()
                    .exit_fade(Duration::from_secs(1))
                    .content(reactor::TextBlock::new().text("retiring")),
            )
        }))
        .into()
}

fn retirement_runtime() -> (
    reactor::Runtime<reactor::RecordingAdapter>,
    reactor::ObjectId,
) {
    let mut runtime = reactor_runtime();
    runtime.update(retirement_view(true));
    let root = runtime.graph().root().unwrap();
    let retiring = runtime
        .graph()
        .children(root, reactor::RelationId::Children)
        .unwrap()[0];
    (runtime, retiring)
}

fn bench_reactor_retirement_initiation(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(samples, batch, retirement_runtime, |(mut runtime, _)| {
        runtime.update(retirement_view(false));
    });
    Row {
        frontend: "reactor",
        workload: "retirement_start",
        objects: 3,
        perf,
    }
}

fn bench_reactor_retirement_completion(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let (mut runtime, retiring) = retirement_runtime();
            runtime.update(retirement_view(false));
            assert!(runtime.complete_retirement(retiring));
            runtime
        },
        |mut runtime| {
            runtime.dispatch_native_events();
        },
    );
    Row {
        frontend: "reactor",
        workload: "retirement_finish",
        objects: 3,
        perf,
    }
}

fn bench_reactor_retirement_remount(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let (mut runtime, retiring) = retirement_runtime();
            runtime.update(retirement_view(false));
            assert!(runtime.complete_retirement(retiring));
            runtime.dispatch_native_events();
            runtime
        },
        |mut runtime| {
            runtime.update(retirement_view(true));
        },
    );
    Row {
        frontend: "reactor",
        workload: "retirement_remount",
        objects: 3,
        perf,
    }
}

fn virtual_items_view(revision: u64) -> reactor::View {
    reactor::ItemsRepeater::new()
        .virtual_source(reactor::VirtualSource::new(
            1,
            10_000,
            reactor::Key::from,
            move |index| -> reactor::View {
                reactor::TextBlock::new()
                    .text(format!("{revision}:{index}"))
                    .into()
            },
        ))
        .into()
}

fn virtual_items_runtime() -> (
    reactor::Runtime<reactor::RecordingAdapter>,
    reactor::ObjectId,
) {
    let mut runtime = reactor_runtime();
    runtime.update(virtual_items_view(0));
    let collection = runtime.graph().root().unwrap();
    (runtime, collection)
}

fn virtual_items_realized_runtime(
    realized: usize,
) -> (
    reactor::Runtime<reactor::RecordingAdapter>,
    reactor::ObjectId,
) {
    let (mut runtime, collection) = virtual_items_runtime();
    for index in 0..realized {
        runtime.queue_realization(reactor::RealizationRequest::Realize {
            collection,
            container: reactor::RealizedContainer(index as u64),
            index,
            source_revision: 0,
        });
    }
    runtime.dispatch_native_events();
    (runtime, collection)
}

fn bench_reactor_virtual_create(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(samples, batch, reactor_runtime, |mut runtime| {
        black_box(runtime.update(virtual_items_view(0)));
    });
    Row {
        frontend: "reactor",
        workload: "virtual_create_10k",
        objects: 10_000,
        perf,
    }
}

fn bench_reactor_virtual_source_replace(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let mut runtime = reactor_runtime();
            runtime.update(reactor::ItemsRepeater::new());
            runtime
        },
        |mut runtime| {
            black_box(runtime.update(virtual_items_view(0)));
        },
    );
    Row {
        frontend: "reactor",
        workload: "virtual_source_replace_10k",
        objects: 10_000,
        perf,
    }
}

fn bench_reactor_virtual_realize(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let (mut runtime, collection) = virtual_items_runtime();
            runtime.queue_realization(reactor::RealizationRequest::Realize {
                collection,
                container: reactor::RealizedContainer(1),
                index: 9_999,
                source_revision: 0,
            });
            runtime
        },
        |mut runtime| {
            black_box(runtime.dispatch_native_events());
        },
    );
    Row {
        frontend: "reactor",
        workload: "virtual_realize_10k",
        objects: 10_000,
        perf,
    }
}

fn bench_reactor_virtual_update(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || virtual_items_realized_runtime(8),
        |(mut runtime, _)| {
            black_box(runtime.update(virtual_items_view(1)));
            black_box(runtime.dispatch_native_events());
        },
    );
    Row {
        frontend: "reactor",
        workload: "virtual_update_10k",
        objects: 10_000,
        perf,
    }
}

fn bench_reactor_virtual_recycle(samples: usize, batch: usize) -> Row {
    let perf = measure_prepared(
        samples,
        batch,
        || {
            let (mut runtime, collection) = virtual_items_realized_runtime(8);
            runtime.queue_realization(reactor::RealizationRequest::Recycle {
                collection,
                container: reactor::RealizedContainer(7),
                source_revision: 0,
            });
            runtime
        },
        |mut runtime| {
            black_box(runtime.dispatch_native_events());
        },
    );
    Row {
        frontend: "reactor",
        workload: "virtual_recycle_10k",
        objects: 10_000,
        perf,
    }
}

fn reactor_virtual_memory() -> MemoryRow {
    let before_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let before_allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let (runtime, _) = virtual_items_realized_runtime(8);
    let bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before_bytes;
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - before_allocations;
    assert_eq!(runtime.graph().object_count(), 9);
    black_box(runtime);
    MemoryRow {
        frontend: "reactor",
        workload: "virtual_10k_eight_rows",
        objects: 10_000,
        bytes,
        allocations,
    }
}

fn reactor_virtual_mutation_counts() -> [(&'static str, usize, usize); 4] {
    let mut create = reactor_runtime();
    create.record_batches(true);
    create.update(virtual_items_view(0));

    let (mut realize, collection) = virtual_items_runtime();
    realize.record_batches(true);
    realize.queue_realization(reactor::RealizationRequest::Realize {
        collection,
        container: reactor::RealizedContainer(0),
        index: 0,
        source_revision: 0,
    });
    realize.dispatch_native_events();

    let (mut update, _) = virtual_items_realized_runtime(8);
    update.record_batches(true);
    update.update(virtual_items_view(1));
    update.dispatch_native_events();

    let (mut recycle, collection) = virtual_items_realized_runtime(8);
    recycle.record_batches(true);
    recycle.queue_realization(reactor::RealizationRequest::Recycle {
        collection,
        container: reactor::RealizedContainer(7),
        source_revision: 0,
    });
    recycle.dispatch_native_events();

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

fn reactor_memory(count: usize) -> MemoryRow {
    let order = (0..count).collect::<Vec<_>>();
    let mut runtime = reactor_runtime();
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(reactor_grid(&order, None));
    black_box(runtime.graph());
    MemoryRow {
        frontend: "reactor",
        workload: "keyed_grid_retained",
        objects: count + 1,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor_graph_memory(count: usize) -> MemoryRow {
    let order = (0..count).collect::<Vec<_>>();
    let mut runtime = reactor::Runtime::new(NullAdapter);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(reactor_grid(&order, None));
    black_box(runtime.graph());
    MemoryRow {
        frontend: "reactor",
        workload: "keyed_grid_graph",
        objects: count + 1,
        bytes: allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn reactor_tree_memory(count: usize, custom_content: bool) -> MemoryRow {
    let mut runtime = reactor_runtime();
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(reactor::TreeView::new().nodes((0..count).map(|index| {
        let node = reactor::TreeNode::new(index, format!("Node {index}"));
        if custom_content {
            node.content(reactor::TextBlock::new().text(format!("Content {index}")))
        } else {
            node
        }
    })));
    black_box(runtime.graph());
    MemoryRow {
        frontend: "reactor",
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

fn reactor_tree_graph_memory(count: usize, custom_content: bool) -> MemoryRow {
    let mut runtime = reactor::Runtime::new(NullAdapter);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(reactor::TreeView::new().nodes((0..count).map(|index| {
        let node = reactor::TreeNode::new(index, format!("Node {index}"));
        if custom_content {
            node.content(reactor::TextBlock::new().text(format!("Content {index}")))
        } else {
            node
        }
    })));
    black_box(runtime.graph());
    MemoryRow {
        frontend: "reactor",
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

fn reactor_border_fixture(count: usize, text: bool, event: bool) -> reactor::View {
    reactor::Grid::new()
        .keyed_children((0..count).map(|index| {
            let border = reactor::Border::new();
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
            reactor::keyed(index, border)
        }))
        .into()
}

fn reactor_unkeyed_border_fixture(count: usize) -> reactor::View {
    reactor::StackPanel::new()
        .children(
            (0..count)
                .map(|_| reactor::Border::new().into())
                .collect::<Vec<reactor::View>>(),
        )
        .into()
}

fn reactor_deep_chain(count: usize) -> reactor::View {
    let mut current: reactor::View = reactor::Border::new().into();
    for _ in 1..count {
        current = reactor::Border::new().content(current).into();
    }
    current
}

fn reactor_empty_navigation_fixture(count: usize) -> reactor::View {
    reactor::Grid::new()
        .keyed_children(
            (0..count).map(|index| reactor::keyed(index, reactor::NavigationView::new())),
        )
        .into()
}

fn reactor_filled_navigation_fixture(count: usize) -> reactor::View {
    reactor::Grid::new()
        .keyed_children((0..count).map(|index| {
            reactor::keyed(
                index,
                reactor::NavigationView::new()
                    .content(reactor::Border::new())
                    .header(reactor::Border::new())
                    .pane_custom_content(reactor::Border::new())
                    .pane_footer(reactor::Border::new()),
            )
        }))
        .into()
}

fn reactor_graph_shape(
    fixture: &'static str,
    visual: impl FnOnce() -> reactor::View,
) -> GraphMemoryRow {
    let mut runtime = reactor::Runtime::new(NullAdapter);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(visual());
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

fn reactor_retirement_graph_shape() -> GraphMemoryRow {
    let mut runtime = reactor::Runtime::new(NullAdapter);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(retirement_view(true));
    runtime.update(retirement_view(false));
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

fn reactor_retained_memory_shapes(count: usize) -> Vec<GraphMemoryRow> {
    vec![
        reactor_graph_shape("flat_keyed_empty", || {
            reactor_border_fixture(count, false, false)
        }),
        reactor_graph_shape("flat_unkeyed_empty", || {
            reactor_unkeyed_border_fixture(count)
        }),
        reactor_graph_shape("flat_string_property", || {
            reactor_border_fixture(count, true, false)
        }),
        reactor_graph_shape("flat_event", || reactor_border_fixture(count, false, true)),
        reactor_graph_shape("deep_one_child", || reactor_deep_chain(count.min(128))),
        reactor_graph_shape("many_empty_relations", || {
            reactor_empty_navigation_fixture(count)
        }),
        reactor_graph_shape("many_filled_relations", || {
            reactor_filled_navigation_fixture(count / 4)
        }),
        reactor_graph_shape("tree_structural", || {
            reactor_tree(&(0..count).collect::<Vec<_>>(), None)
        }),
        reactor_graph_shape("virtual_10k_zero_rows", || virtual_items_view(0)),
        reactor_retirement_graph_shape(),
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

fn reactor_fixture_memory<A>(
    frontend: &'static str,
    workload: &'static str,
    objects: usize,
    mut runtime: reactor::Runtime<A>,
    visual: impl FnOnce() -> reactor::View,
) -> MemoryRow
where
    A: reactor::Adapter,
{
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    runtime.update(visual());
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

fn reactor_tree_declaration_memory(count: usize) -> MemoryRow {
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let declaration = reactor_tree(&(0..count).collect::<Vec<_>>(), None);
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

fn reactor_memory_attribution(count: usize) -> [MemoryRow; 6] {
    [
        reactor_tree_declaration_memory(count),
        reactor_fixture_memory(
            "graph",
            "flat_border_base",
            count + 1,
            reactor::Runtime::new(NullAdapter),
            || reactor_border_fixture(count, false, false),
        ),
        reactor_fixture_memory(
            "graph",
            "flat_border_string",
            count + 1,
            reactor::Runtime::new(NullAdapter),
            || reactor_border_fixture(count, true, false),
        ),
        reactor_fixture_memory(
            "graph",
            "flat_border_event",
            count + 1,
            reactor::Runtime::new(NullAdapter),
            || reactor_border_fixture(count, false, true),
        ),
        reactor_fixture_memory(
            "graph",
            "tree_graph",
            count + 1,
            reactor::Runtime::new(NullAdapter),
            || reactor_tree(&(0..count).collect::<Vec<_>>(), None),
        ),
        reactor_fixture_memory(
            "recorded",
            "tree_recorded",
            count + 1,
            reactor_runtime(),
            || reactor_tree(&(0..count).collect::<Vec<_>>(), None),
        ),
    ]
}

fn reactor_component(count: usize, effect: bool, samples: usize, batch: usize) -> Row {
    let mut adapter = reactor::RecordingAdapter::default();
    adapter.record_batches(false);
    let mut components = reactor::ComponentHost::mount(
        adapter,
        (0..count).map(|index| reactor::component::<BenchComponent>(index, effect)),
    );
    let sender = components
        .sender::<BenchComponent>(&reactor::Key::from(count / 2))
        .unwrap();
    let perf = measure(samples, batch, || {
        assert!(sender.send(true));
        black_box(components.drain(1));
    });
    Row {
        frontend: "reactor",
        workload: if effect {
            "component_effect"
        } else {
            "component_isolated"
        },
        objects: count,
        perf,
    }
}

fn reactor_component_root_replace(count: usize, samples: usize, batch: usize) -> Row {
    let mut adapter = reactor::RecordingAdapter::default();
    adapter.record_batches(false);
    let mut components = reactor::ComponentHost::mount(
        adapter,
        (0..count).map(|index| reactor::component::<RootSwitch>(index, ())),
    );
    let sender = components
        .sender::<RootSwitch>(&reactor::Key::from(count / 2))
        .unwrap();
    let perf = measure(samples, batch, || {
        assert!(sender.send(()));
        black_box(components.drain(1));
    });
    Row {
        frontend: "reactor",
        workload: "component_replace",
        objects: count,
        perf,
    }
}

fn reactor_component_remove(count: usize, samples: usize) -> Row {
    let perf = measure_prepared(
        samples,
        1,
        || {
            let mut adapter = reactor::RecordingAdapter::default();
            adapter.record_batches(false);
            reactor::ComponentHost::mount(
                adapter,
                (0..count).map(|index| reactor::component::<BenchComponent>(index, false)),
            )
        },
        |mut components| {
            components.remove(&reactor::Key::from(count / 2));
            black_box(components);
        },
    );
    Row {
        frontend: "reactor",
        workload: "component_remove",
        objects: count,
        perf,
    }
}

fn reactor_component_memory(count: usize, effect: bool) -> MemoryRow {
    let mut adapter = reactor::RecordingAdapter::default();
    adapter.record_batches(false);
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let components = reactor::ComponentHost::mount(
        adapter,
        (0..count).map(|index| reactor::component::<BenchComponent>(index, effect)),
    );
    black_box(&components);
    MemoryRow {
        frontend: "reactor",
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

fn reactor_context(count: usize, broad: bool, samples: usize) -> Row {
    let mut adapter = reactor::RecordingAdapter::default();
    adapter.record_batches(false);
    let context = Rc::new(reactor::Context::new(false));
    let mut components = reactor::ComponentHost::mount(
        adapter,
        (0..count).map(|index| {
            reactor::component::<ContextComponent>(
                index,
                ContextInput {
                    context: Rc::clone(&context),
                    subscribe: broad || index == count / 2,
                },
            )
        }),
    );
    let mut value = false;
    let perf = measure(samples, 1, || {
        value = !value;
        black_box(components.set_context(&context, value));
    });
    Row {
        frontend: "reactor",
        workload: if broad {
            "context_broad"
        } else {
            "context_isolated"
        },
        objects: count,
        perf,
    }
}

fn reactor_recursive(depth: usize, fanout: usize, samples: usize) -> RecursiveRow {
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let mut components = reactor::ComponentHost::mount(
        reactor::RecordingAdapter::default(),
        [reactor::component::<Branch>(
            "root",
            BranchInput { depth, fanout },
        )],
    );
    let retained_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before;
    let mut path = Vec::with_capacity(depth + 1);
    path.push(reactor::Key::from("root"));
    path.extend((0..depth).map(|_| reactor::Key::from("0")));
    let sender = components.sender_at::<Branch>(&path).unwrap();
    let mut mutations = 0;
    let perf = measure(samples, 1, || {
        assert!(sender.send(()));
        mutations += components.drain(usize::MAX).mutations;
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

#[derive(Debug, PartialEq)]
enum Mode {
    All,
    Architecture,
    RetainedMemory,
    VirtualMemory,
    VirtualItems,
    KeyedScaling,
    Profile(String),
}

#[derive(Debug)]
struct Options {
    mode: Mode,
    samples: usize,
    batch: usize,
    count: usize,
    component_count: usize,
    profile_iterations: Option<usize>,
}

impl Options {
    fn parse(arguments: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut options = Self {
            mode: Mode::All,
            samples: 80,
            batch: 8,
            count: 512,
            component_count: 16_384,
            profile_iterations: None,
        };
        let mut samples = None;
        let mut arguments = arguments.into_iter();
        while let Some(argument) = arguments.next() {
            match argument.as_str() {
                "--samples" => samples = Some(positive_argument(&mut arguments, &argument)?),
                "--batch" => options.batch = positive_argument(&mut arguments, &argument)?,
                "--count" => options.count = positive_argument(&mut arguments, &argument)?,
                "--component-count" => {
                    options.component_count = positive_argument(&mut arguments, &argument)?;
                }
                "--profile-iterations" => {
                    options.profile_iterations =
                        Some(positive_argument(&mut arguments, &argument)?);
                }
                _ => {
                    let mode = match argument.as_str() {
                        "--architecture-report" => Mode::Architecture,
                        "--retained-memory" => Mode::RetainedMemory,
                        "--virtual-memory" => Mode::VirtualMemory,
                        "--virtual-items" => Mode::VirtualItems,
                        "--keyed-scaling" => Mode::KeyedScaling,
                        "--profile-workload" => Mode::Profile(
                            arguments
                                .next()
                                .ok_or("--profile-workload requires a workload name")?,
                        ),
                        _ => return Err(format!("unknown argument: {argument}")),
                    };
                    if options.mode != Mode::All {
                        return Err("select only one benchmark mode".to_string());
                    }
                    options.mode = mode;
                }
            }
        }
        options.samples = samples.unwrap_or(if options.mode == Mode::Architecture {
            6
        } else {
            80
        });
        Ok(options)
    }
}

fn positive_argument(
    arguments: &mut impl Iterator<Item = String>,
    name: &str,
) -> Result<usize, String> {
    arguments
        .next()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .ok_or_else(|| format!("{name} requires a positive integer"))
}

fn profile_virtual_source_replace(iterations: usize) {
    let (mut runtime, _) = virtual_items_realized_runtime(8);
    let bytes = allocator::allocated_bytes();
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let started = Instant::now();
    let mut mutations = 0;
    for revision in 1..=iterations {
        mutations += runtime.update(virtual_items_view(revision as u64)).len();
        runtime.dispatch_native_events();
    }
    let elapsed = started.elapsed();
    assert_eq!(runtime.graph().object_count(), 9);
    println!(
        "{{\"benchmark\":\"reactor-profile\",\
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

fn run_profile_workload(
    workload: &str,
    iterations: usize,
    component_count: usize,
) -> Result<(), String> {
    match workload {
        "reactor-100k-no-change" => {
            print_perf_rows([bench_reactor_100k(false, iterations)]);
        }
        "reactor-100k-one-changed" => {
            print_perf_rows([bench_reactor_100k(true, iterations)]);
        }
        "reactor-context-broad" => {
            print_perf_rows([reactor_context(component_count, true, iterations)]);
        }
        "reactor-virtual-source-replace" => profile_virtual_source_replace(iterations),
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
    println!("\n513-node Reactor additive attribution");
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
    if let Err(error) = Options::parse(std::env::args().skip(1)).and_then(run) {
        eprintln!("test-reactor-bench: {error}");
        std::process::exit(2);
    }
}

fn run(options: Options) -> Result<(), String> {
    let samples = options.samples;
    let batch = options.batch;
    if let Mode::Profile(workload) = &options.mode {
        return run_profile_workload(
            workload,
            options.profile_iterations.unwrap_or(samples),
            options.component_count,
        );
    }
    if options.mode == Mode::VirtualMemory {
        print_memory_rows(&[reactor_virtual_memory()]);
        return Ok(());
    }
    if options.mode == Mode::RetainedMemory {
        print_retained_memory_shapes(&reactor_retained_memory_shapes(options.count));
        return Ok(());
    }
    if options.mode == Mode::VirtualItems {
        print_perf_rows([
            bench_reactor_virtual_create(samples, batch),
            bench_reactor_virtual_source_replace(samples, batch),
            bench_reactor_virtual_realize(samples, batch),
            bench_reactor_virtual_update(samples, batch),
            bench_reactor_virtual_recycle(samples, batch),
        ]);
        println!();
        print_memory_rows(&[reactor_virtual_memory()]);
        println!("\nvirtual mutation trace");
        for (workload, mutations, realized) in reactor_virtual_mutation_counts() {
            println!("{workload:<24} {mutations:>12} {realized:>15}");
        }
        return Ok(());
    }
    if options.mode == Mode::Architecture {
        println!("architecture report: 100k keyed updates (2 x 50k graphs)");
        print_perf_rows([
            bench_reactor_100k(false, samples),
            bench_reactor_100k(true, samples),
        ]);

        println!("\narchitecture report: 513-node retained memory attribution");
        let memory_rows = reactor_memory_attribution(512);
        print_memory_rows(&memory_rows);
        print_memory_attribution(&memory_rows);

        let component_count = options.component_count;
        println!("\narchitecture report: large component graph operations");
        print_perf_rows([
            reactor_component_root_replace(component_count, samples, 1),
            reactor_component_remove(component_count, samples),
        ]);
        return Ok(());
    }
    if options.mode == Mode::KeyedScaling {
        let mut rows = Vec::new();
        for count in [512, 1_024, 10_000] {
            let order = (0..count).collect::<Vec<_>>();
            rows.push(bench_reactor(
                "keyed_no_change",
                count,
                reactor_grid(&order, None),
                reactor_grid(&order, None),
                samples,
                batch,
            ));
            rows.push(bench_reactor(
                "keyed_one_changed",
                count,
                reactor_grid(&order, None),
                reactor_grid(&order, Some(0)),
                samples,
                batch,
            ));
        }
        print_perf_rows(rows);
        return Ok(());
    }
    let count = options.count;
    let order = (0..count).collect::<Vec<_>>();
    let mut rotated = order.clone();
    rotated.rotate_left(1);
    let mut reversed = order.clone();
    reversed.reverse();

    let rows = [
        bench_reactor(
            "keyed_no_change",
            count,
            reactor_grid(&order, None),
            reactor_grid(&order, None),
            samples,
            batch,
        ),
        bench_reactor(
            "keyed_one_changed",
            count,
            reactor_grid(&order, None),
            reactor_grid(&order, Some(0)),
            samples,
            batch,
        ),
        bench_reactor(
            "keyed_rotate_one",
            count,
            reactor_grid(&order, None),
            reactor_grid(&rotated, None),
            samples,
            batch,
        ),
        bench_reactor(
            "keyed_reverse",
            count,
            reactor_grid(&order, None),
            reactor_grid(&reversed, None),
            samples,
            batch,
        ),
        bench_reactor(
            "tree_rotate_one",
            count,
            reactor_tree(&order, None),
            reactor_tree(&rotated, None),
            samples,
            batch,
        ),
        reactor_component(count, false, samples, batch),
        reactor_component(count, true, samples, batch),
        reactor_component_root_replace(count, samples, batch),
        reactor_context(count, false, samples),
        reactor_context(count, true, samples),
        bench_reactor_retirement_initiation(samples, batch),
        bench_reactor_retirement_completion(samples, batch),
        bench_reactor_retirement_remount(samples, batch),
        bench_reactor_virtual_create(samples, batch),
        bench_reactor_virtual_source_replace(samples, batch),
        bench_reactor_virtual_realize(samples, batch),
        bench_reactor_virtual_update(samples, batch),
        bench_reactor_virtual_recycle(samples, batch),
    ];

    print_perf_rows(rows);

    println!();
    let memory_rows = [
        reactor_graph_memory(count),
        reactor_memory(count),
        reactor_tree_graph_memory(count, false),
        reactor_tree_memory(count, false),
        reactor_tree_graph_memory(count, true),
        reactor_tree_memory(count, true),
        reactor_component_memory(count, false),
        reactor_component_memory(count, true),
        reactor_virtual_memory(),
    ];
    print_memory_rows(&memory_rows);

    println!("\nvirtual mutation trace");
    println!(
        "{:<24} {:>12} {:>15}",
        "workload", "mutations", "realized rows"
    );
    for (workload, mutations, realized) in reactor_virtual_mutation_counts() {
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
        reactor_recursive(3, 8, samples),
        reactor_recursive(4, 8, samples),
        reactor_recursive(7, 4, samples),
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(arguments: &[&str]) -> Result<Options, String> {
        Options::parse(arguments.iter().map(|value| (*value).to_string()))
    }

    #[test]
    fn sample_defaults_and_overrides() {
        assert_eq!(parse(&[]).unwrap().samples, 80);
        assert_eq!(parse(&["--architecture-report"]).unwrap().samples, 6);
        for arguments in [
            ["--architecture-report", "--samples", "3"],
            ["--samples", "3", "--architecture-report"],
        ] {
            let options = parse(&arguments).unwrap();
            assert_eq!(options.mode, Mode::Architecture);
            assert_eq!(options.samples, 3);
        }
    }

    #[test]
    fn invalid_arguments_do_not_run_default_benchmarks() {
        for arguments in [
            vec!["--lifecycle-stress"],
            vec!["--architecture-gate"],
            vec!["--gate-samples", "6"],
            vec!["--unknown"],
            vec!["--virtual-memory", "--retained-memory"],
            vec!["--profile-workload"],
        ] {
            assert!(parse(&arguments).is_err(), "{arguments:?}");
        }
        for name in [
            "--count",
            "--samples",
            "--batch",
            "--component-count",
            "--profile-iterations",
        ] {
            assert!(parse(&[name]).is_err(), "{name}");
            for value in ["0", "-1", "invalid", "--virtual-memory"] {
                assert!(parse(&[name, value]).is_err(), "{name} {value}");
            }
        }
    }
}
