mod allocator;

use std::hint::black_box;
use std::sync::atomic::Ordering;
use std::time::Instant;
use windows_reactor2 as reactor2;

#[derive(Clone, PartialEq)]
struct BranchInput {
    depth: usize,
    fanout: usize,
}

struct Branch {
    value: usize,
}

impl reactor2::Component for Branch {
    type Input = BranchInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, (): Self::Message, _context: &reactor2::ComponentContext<Self::Message>) {
        self.value += 1;
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        if input.depth == 0 {
            return reactor2::TextBlock::new(self.value.to_string()).into();
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

struct ResultRow {
    allocations_per_update: f64,
    bytes_per_scope: f64,
    bytes_per_update: f64,
    depth: usize,
    fanout: usize,
    median_ns: f64,
    mutations_per_update: f64,
    p95_ns: f64,
    retained_bytes: u64,
    scopes: usize,
}

fn scope_count(depth: usize, fanout: usize) -> usize {
    (0..=depth).map(|level| fanout.pow(level as u32)).sum()
}

fn percentile(values: &[u64], percentile: usize) -> u64 {
    values[(values.len() - 1) * percentile / 100]
}

fn measure(depth: usize, fanout: usize, samples: usize) -> ResultRow {
    let before = allocator::CURRENT_BYTES.load(Ordering::Relaxed);
    let mut host = reactor2::ComponentHost::mount(
        reactor2::RecordingAdapter::default(),
        [reactor2::component::<Branch>(
            "root",
            BranchInput { depth, fanout },
        )],
    )
    .unwrap();
    let retained_bytes = allocator::CURRENT_BYTES.load(Ordering::Relaxed) - before;
    let scopes = scope_count(depth, fanout);
    host.runtime_mut().adapter_mut().validate_batches(false);
    let mut path = Vec::with_capacity(depth + 1);
    path.push(reactor2::Key::from("root"));
    path.extend((0..depth).map(|_| reactor2::Key::from("0")));
    let sender = host.sender_at::<Branch>(&path).unwrap();

    for _ in 0..32 {
        assert!(sender.send(()));
        black_box(host.drain(usize::MAX).unwrap());
    }

    let mut timings = Vec::with_capacity(samples);
    let bytes = allocator::allocated_bytes();
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let mut mutations = 0;
    for _ in 0..samples {
        assert!(sender.send(()));
        let start = Instant::now();
        mutations += host.drain(usize::MAX).unwrap().mutations;
        timings.push(start.elapsed().as_nanos() as u64);
    }
    let bytes = allocator::allocated_bytes() - bytes;
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations;
    timings.sort_unstable();

    ResultRow {
        allocations_per_update: allocations as f64 / samples as f64,
        bytes_per_scope: retained_bytes as f64 / scopes as f64,
        bytes_per_update: bytes as f64 / samples as f64,
        depth,
        fanout,
        median_ns: percentile(&timings, 50) as f64,
        mutations_per_update: mutations as f64 / samples as f64,
        p95_ns: percentile(&timings, 95) as f64,
        retained_bytes,
        scopes,
    }
}

fn parse_samples() -> usize {
    let args = std::env::args().collect::<Vec<_>>();
    args.windows(2)
        .find(|pair| pair[0] == "--samples")
        .and_then(|pair| pair[1].parse().ok())
        .unwrap_or(1_000)
}

fn main() {
    let samples = parse_samples();
    let rows = [
        measure(3, 8, samples),
        measure(4, 8, samples),
        measure(7, 4, samples),
    ];

    println!("reactor2-component-benchmark-format: 1");
    println!(
        "{:>5} {:>6} {:>8} {:>15} {:>13} {:>12} {:>12} {:>11} {:>12}",
        "depth",
        "fanout",
        "scopes",
        "retained bytes",
        "bytes/scope",
        "median ns",
        "p95 ns",
        "allocs/op",
        "mutations/op"
    );
    println!("{}", "-".repeat(108));
    for row in rows {
        println!(
            "{:>5} {:>6} {:>8} {:>15} {:>13.1} {:>12.1} {:>12.1} {:>11.2} {:>12.2}",
            row.depth,
            row.fanout,
            row.scopes,
            row.retained_bytes,
            row.bytes_per_scope,
            row.median_ns,
            row.p95_ns,
            row.allocations_per_update,
            row.mutations_per_update
        );
        println!("  allocated bytes/update: {:.1}", row.bytes_per_update);
    }
}
