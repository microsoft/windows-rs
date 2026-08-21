//! Headless reconciler micro-benchmarks for `windows-reactor`.
//!
//! Ported in spirit from Microsoft.UI.Reactor's `PerfBench.ControlModel`
//! (spec-047 M1-M13). Unlike `test_reactor_perf`, which drives a live WinUI
//! window and is render-bound (so per-reconcile deltas are diluted by the
//! render pipeline), this crate brackets only the reconcile body against the
//! headless `RecordingBackend`. That makes it ns-resolution and free of WinUI
//! noise - the right instrument for evaluating Rust-side reconciler changes
//! (structural-skip / diff hints, keyed-diff scratch allocation).
//!
//! It measures two things per benchmark:
//!
//! - ns/op: wall-clock nanoseconds per reconcile, best-of-`reps` to shed
//!   scheduler noise.
//! - bytes/op and allocs/op: heap traffic per reconcile, captured by a
//!   counting global allocator. This is deterministic for identical code and
//!   is the sensitive signal for allocation-reduction work (for example
//!   reusing the keyed-diff scratch buffers).
//!
//! `RecordingBackend::create` is a trivial id bump, so these numbers are the
//! Rust-side reconciler cost only. The native WinUI control create/destroy
//! cost that control pooling targets is not modeled here - that question
//! belongs to `test_reactor_perf`'s churn scenario, which drives real XAML
//! controls.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use test_reactor::RecordingBackend;
use windows_reactor::{
    Component, Context, Element, KeyExt, ProvideExt, Reconciler, RenderCx, SetState, component,
    grid, list_view, memo, swap_chain_panel, text_block, vstack,
};

static BYTES: AtomicU64 = AtomicU64::new(0);
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static CURRENT_BYTES: AtomicU64 = AtomicU64::new(0);
static BENCH_CONTEXT: LazyLock<Context<u8>> = LazyLock::new(|| Context::new(0));

/// Global allocator that counts bytes and allocation calls. Wraps `System`;
/// tracks only growth (alloc plus grow-realloc), which is enough for a
/// per-op relative signal.
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            CURRENT_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        CURRENT_BYTES.fetch_sub(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let p = unsafe { System.realloc(ptr, layout, new_size) };
        if !p.is_null() && new_size > layout.size() {
            BYTES.fetch_add((new_size - layout.size()) as u64, Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            CURRENT_BYTES.fetch_add((new_size - layout.size()) as u64, Ordering::Relaxed);
        } else if !p.is_null() {
            CURRENT_BYTES.fetch_sub((layout.size() - new_size) as u64, Ordering::Relaxed);
        }
        p
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

struct Perf {
    ns: f64,
    bytes: f64,
    allocs: f64,
}

/// Runs `op` for `warmup` discarded passes, then `reps` timed passes of
/// `iters` each. Reports the fastest pass (ns) with its heap traffic.
fn measure(iters: u64, reps: u32, warmup: u32, mut op: impl FnMut()) -> Perf {
    for _ in 0..warmup {
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
        let b0 = BYTES.load(Ordering::Relaxed);
        let a0 = ALLOCS.load(Ordering::Relaxed);
        let start = Instant::now();
        for _ in 0..iters {
            op();
        }
        let ns = start.elapsed().as_nanos() as f64 / iters as f64;
        if ns < best.ns {
            best.ns = ns;
            best.bytes = (BYTES.load(Ordering::Relaxed) - b0) as f64 / iters as f64;
            best.allocs = (ALLOCS.load(Ordering::Relaxed) - a0) as f64 / iters as f64;
        }
    }
    best
}

struct Row {
    name: String,
    n: usize,
    perf: Perf,
    skipped: u64,
    diffed: u64,
    created: u64,
}

fn no_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn labels(n: usize, prefix: &str) -> Vec<String> {
    (0..n).map(|i| format!("{prefix}{i}")).collect()
}

fn plain_stack(labels: &[String]) -> Element {
    let children: Vec<Element> = labels
        .iter()
        .map(|s| text_block(s.clone()).into())
        .collect();
    vstack(children).into()
}

fn keyed_stack(keys: &[String]) -> Element {
    let children: Vec<Element> = keys
        .iter()
        .map(|k| text_block(k.clone()).with_key(k.clone()).into())
        .collect();
    let mut s = vstack(children);
    s.key = Some("root".to_string());
    s.into()
}

fn component_leaf(_props: &(), _cx: &mut RenderCx) -> Element {
    text_block("component").into()
}

fn pass_through_component(_props: &(), _cx: &mut RenderCx) -> Element {
    component(component_leaf, ())
}

struct DirtyLeaf {
    setter: Rc<RefCell<Option<SetState<u64>>>>,
}

impl Component for DirtyLeaf {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (value, setter) = cx.use_state(0_u64);
        *self.setter.borrow_mut() = Some(setter);
        text_block(format!("dirty-{value}")).into()
    }
}

struct DirtyWidgetRoot {
    setter: Rc<RefCell<Option<SetState<u64>>>>,
}

impl Component for DirtyWidgetRoot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        grid(vec![component(
            DirtyLeaf {
                setter: Rc::clone(&self.setter),
            },
            (),
        )])
        .into()
    }
}

fn dirty_widget_tree(setter: Rc<RefCell<Option<SetState<u64>>>>) -> Element {
    memo(DirtyWidgetRoot { setter }, ())
}

struct DirtyPassThrough {
    setter: Rc<RefCell<Option<SetState<u64>>>>,
}

impl Component<u8> for DirtyPassThrough {
    fn render(&self, depth: &u8, cx: &mut RenderCx) -> Element {
        if *depth == 0 {
            let (value, setter) = cx.use_state(0_u64);
            *self.setter.borrow_mut() = Some(setter);
            text_block(format!("deep-dirty-{value}")).into()
        } else {
            component(
                Self {
                    setter: Rc::clone(&self.setter),
                },
                depth - 1,
            )
        }
    }
}

fn deep_dirty_tree(setter: Rc<RefCell<Option<SetState<u64>>>>) -> Element {
    memo(DirtyPassThrough { setter }, 3)
}

/// One reconcile A -> B per op, alternating direction so the live tree stays
/// consistent and each op is a real diff in one direction or the other.
fn bench_update(name: &str, n: usize, a: Element, b: Element, iters: u64, reps: u32) -> Row {
    let rr = no_rerender();

    let (skipped, diffed, created) = {
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r.reconcile(None, &a, None, Rc::clone(&rr)).unwrap();
        r.reset_stats();
        r.reconcile(Some(&a), &b, Some(id), Rc::clone(&rr));
        let stats = r.stats();
        (
            stats.elements_skipped,
            stats.elements_diffed,
            stats.ui_elements_created,
        )
    };

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &a, None, Rc::clone(&rr)).unwrap();
    let mut flip = false;
    let perf = measure(iters, reps, 2, || {
        if flip {
            r.reconcile(Some(&b), &a, Some(id), Rc::clone(&rr));
        } else {
            r.reconcile(Some(&a), &b, Some(id), Rc::clone(&rr));
        }
        flip = !flip;
    });

    Row {
        name: name.to_string(),
        n,
        perf,
        skipped,
        diffed,
        created,
    }
}

/// One mount + unmount of the whole subtree per op. This is the Rust-side
/// create/destroy cost that a control pool would try to avoid (the native
/// side is not modeled by `RecordingBackend`).
fn bench_mount_unmount(name: &str, n: usize, tree: Element, iters: u64, reps: u32) -> Row {
    let rr = no_rerender();

    let (skipped, diffed, created) = {
        let mut r = Reconciler::new(RecordingBackend::new());
        r.reset_stats();
        let id = r.reconcile(None, &tree, None, Rc::clone(&rr)).unwrap();
        let stats = r.stats();
        let counts = (
            stats.elements_skipped,
            stats.elements_diffed,
            stats.ui_elements_created,
        );
        r.unmount(id);
        counts
    };

    let mut r = Reconciler::new(RecordingBackend::new());
    let perf = measure(iters, reps, 2, || {
        let id = r.reconcile(None, &tree, None, Rc::clone(&rr)).unwrap();
        r.unmount(id);
    });

    Row {
        name: name.to_string(),
        n,
        perf,
        skipped,
        diffed,
        created,
    }
}

fn bench_dirty_component(
    name: &str,
    n: usize,
    tree: Element,
    setter: Rc<RefCell<Option<SetState<u64>>>>,
    iters: u64,
    reps: u32,
) -> Row {
    let rr = no_rerender();
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &tree, None, Rc::clone(&rr)).unwrap();
    r.reset_stats();

    setter.borrow().as_ref().unwrap().call(1);
    r.reconcile(Some(&tree), &tree, Some(id), Rc::clone(&rr));
    let stats = r.stats();
    let counts = (
        stats.elements_skipped,
        stats.elements_diffed,
        stats.ui_elements_created,
    );
    r.reset_stats();

    let mut value = 1_u64;
    let perf = measure(iters, reps, 2, || {
        value ^= 1;
        setter.borrow().as_ref().unwrap().call(value);
        r.reconcile(Some(&tree), &tree, Some(id), Rc::clone(&rr));
    });

    Row {
        name: name.to_string(),
        n,
        perf,
        skipped: counts.0,
        diffed: counts.1,
        created: counts.2,
    }
}

fn measure_idle_component_memory(count: usize) -> u64 {
    let tree = vstack(
        (0..count)
            .map(|_| component(component_leaf, ()))
            .collect::<Vec<_>>(),
    )
    .into();
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let before = CURRENT_BYTES.load(Ordering::Relaxed);
    let id = reconciler
        .reconcile(None, &tree, None, no_rerender())
        .unwrap();
    let bytes = CURRENT_BYTES.load(Ordering::Relaxed) - before;
    reconciler.unmount(id);
    bytes
}

fn parse_arg(name: &str, default: u64) -> u64 {
    let args: Vec<String> = std::env::args().collect();
    for w in args.windows(2) {
        if w[0] == name {
            return w[1].parse().unwrap_or(default);
        }
    }
    default
}

fn main() {
    let iters = parse_arg("--iters", 2_000);
    let reps = parse_arg("--reps", 6) as u32;

    let mut rows: Vec<Row> = Vec::new();

    // Mount + unmount: Rust-side create/destroy cost (control-pooling relevance).
    rows.push(bench_mount_unmount(
        "component_mount",
        1,
        component(component_leaf, ()),
        iters,
        reps,
    ));
    rows.push(bench_mount_unmount(
        "pass_through_mount",
        2,
        memo(pass_through_component, ()),
        iters,
        reps,
    ));
    rows.push(bench_mount_unmount(
        "provider_mount",
        2,
        component(component_leaf, ()).provide(&BENCH_CONTEXT, 1),
        iters,
        reps,
    ));
    rows.push(bench_mount_unmount(
        "lifecycle_mount",
        1,
        swap_chain_panel().on_unmounted(|_| {}).into(),
        iters,
        reps,
    ));
    for n in [64, 4096] {
        rows.push(bench_mount_unmount(
            "templated_mount",
            n,
            list_view((0..n).map(|n| n as i32).collect::<Vec<_>>(), |n, _| {
                text_block(n.to_string())
            })
            .on_selection_changed(|_| {})
            .on_reorder(|_| {})
            .build(),
            iters,
            reps,
        ));
    }
    rows.push(bench_mount_unmount(
        "deep_pass_mount",
        4,
        deep_dirty_tree(Rc::new(RefCell::new(None))),
        iters,
        reps,
    ));
    {
        let setter = Rc::new(RefCell::new(None));
        rows.push(bench_dirty_component(
            "dirty_component",
            1,
            component(
                DirtyLeaf {
                    setter: Rc::clone(&setter),
                },
                (),
            ),
            setter,
            iters,
            reps,
        ));
    }
    {
        let setter = Rc::new(RefCell::new(None));
        rows.push(bench_dirty_component(
            "dirty_deep_pass",
            4,
            deep_dirty_tree(Rc::clone(&setter)),
            setter,
            iters,
            reps,
        ));
    }
    {
        let setter = Rc::new(RefCell::new(None));
        rows.push(bench_dirty_component(
            "dirty_memo_widget",
            3,
            dirty_widget_tree(Rc::clone(&setter)),
            setter,
            iters,
            reps,
        ));
    }
    for &n in &[64usize, 512] {
        let tree = plain_stack(&labels(n, "cell"));
        rows.push(bench_mount_unmount(
            "mount_unmount",
            n,
            tree,
            iters / (n as u64 / 32).max(1),
            reps,
        ));
    }

    // Update with one leaf changed: forces the per-child skip-walk of N-1
    // untouched siblings. This is the diff-hints target - the (N-1) skips are
    // exactly what a changed-index hint would eliminate. ns scaling with N
    // gives the per-skip cost.
    for &n in &[64usize, 512, 4096] {
        let base = labels(n, "cell");
        let mut changed = base.clone();
        changed[0] = "CHANGED".to_string();
        rows.push(bench_update(
            "update_1_changed",
            n,
            plain_stack(&base),
            plain_stack(&changed),
            iters,
            reps,
        ));
    }

    // Update with every leaf changed: full per-child diff, the upper bound of
    // real reconcile work at this size (context for the skip cost above).
    {
        let n = 512;
        let a = labels(n, "a");
        let b = labels(n, "b");
        rows.push(bench_update(
            "update_all_changed",
            n,
            plain_stack(&a),
            plain_stack(&b),
            iters,
            reps,
        ));
    }

    // Identical tree: the whole subtree is structurally equal, so the root
    // `can_skip_update` short-circuits in O(1) (1 skip for the whole tree).
    {
        let n = 512;
        let a = plain_stack(&labels(n, "cell"));
        let b = a.clone();
        rows.push(bench_update("update_no_change", n, a, b, iters, reps));
    }

    // Keyed full reversal: takes the keyed arm (prefix/suffix strip, key map,
    // LIS) every op. bytes/op here is the keyed-diff scratch allocation - the
    // scratch-reuse target.
    for &n in &[64usize, 512] {
        let keys = labels(n, "k");
        let mut rev = keys.clone();
        rev.reverse();
        rows.push(bench_update(
            "keyed_reverse",
            n,
            keyed_stack(&keys),
            keyed_stack(&rev),
            iters / (n as u64 / 64).max(1),
            reps,
        ));
    }

    // Keyed rotate-by-one: a common insert/remove-shaped edit; the prefix and
    // suffix strip cover nothing, so the whole middle takes the key map + LIS.
    {
        let n = 512;
        let keys = labels(n, "k");
        let mut rot = keys.clone();
        rot.rotate_left(1);
        rows.push(bench_update(
            "keyed_rotate1",
            n,
            keyed_stack(&keys),
            keyed_stack(&rot),
            iters,
            reps,
        ));
    }

    print_table(&rows);

    println!("\nidle component memory");
    println!(
        "{:>8} {:>16} {:>18}",
        "scopes", "retained bytes", "bytes/scope"
    );
    println!("{}", "-".repeat(46));
    for count in [512, 4_096, 16_384] {
        let bytes = measure_idle_component_memory(count);
        println!(
            "{count:>8} {bytes:>16} {:>18.1}",
            bytes as f64 / count as f64
        );
    }
}

fn print_table(rows: &[Row]) {
    println!("windows-reactor headless reconciler micro-benchmarks");
    println!("(RecordingBackend; ns/op is best-of-reps, Rust-side cost only)\n");
    println!(
        "{:<20} {:>6} {:>12} {:>11} {:>10} {:>7} {:>6} {:>6}",
        "bench", "N", "ns/op", "bytes/op", "allocs/op", "skip", "diff", "crt"
    );
    println!("{}", "-".repeat(86));
    for r in rows {
        println!(
            "{:<20} {:>6} {:>12.1} {:>11.1} {:>10.2} {:>7} {:>6} {:>6}",
            r.name, r.n, r.perf.ns, r.perf.bytes, r.perf.allocs, r.skipped, r.diffed, r.created,
        );
    }
}
