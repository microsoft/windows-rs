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
    let bytes = allocator::allocated_bytes();
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let mut timings = Vec::with_capacity(samples);
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
}
