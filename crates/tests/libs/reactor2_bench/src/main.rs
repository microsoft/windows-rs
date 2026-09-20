use std::hint::black_box;
use std::sync::atomic::Ordering;
use std::time::Instant;

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
    context: std::rc::Rc<reactor2::Context<bool>>,
    subscribe: bool,
}

impl PartialEq for ContextInput {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.context, &other.context) && self.subscribe == other.subscribe
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
    let context = std::rc::Rc::new(reactor2::Context::new(false));
    let mut components = reactor2::ComponentHost::mount(
        adapter,
        (0..count).map(|index| {
            reactor2::component::<ContextComponent>(
                index,
                ContextInput {
                    context: std::rc::Rc::clone(&context),
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
    components
        .runtime_mut()
        .adapter_mut()
        .validate_batches(false);
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

fn main() {
    let samples = argument("--samples", 80);
    let batch = argument("--batch", 8);
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
    ];

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

    println!();
    println!(
        "{:<9} {:<22} {:>7} {:>16} {:>14} {:>16}",
        "frontend", "workload", "objects", "retained bytes", "bytes/object", "allocations"
    );
    for row in [
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
    ] {
        println!(
            "{:<9} {:<22} {:>7} {:>16} {:>14.1} {:>16}",
            row.frontend,
            row.workload,
            row.objects,
            row.bytes,
            row.bytes as f64 / row.objects as f64,
            row.allocations
        );
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
