use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use windows_reactor::*;

mod allocator;

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
struct LeafInput {
    sender: Rc<RefCell<Option<LocalSender<bool>>>>,
}

impl PartialEq for LeafInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct BenchLeaf {
    active: bool,
}

enum BackgroundMessage {
    Complete,
    Start,
}

#[derive(Clone)]
struct BackgroundInput {
    sender: Rc<RefCell<Option<LocalSender<BackgroundMessage>>>>,
}

impl PartialEq for BackgroundInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct BackgroundLeaf(bool);

impl Component for BackgroundLeaf {
    type Input = BackgroundInput;
    type Message = BackgroundMessage;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self(false)
    }

    fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self>) {}

    fn update(&mut self, message: BackgroundMessage, context: &ComponentContext<Self>) {
        match message {
            BackgroundMessage::Complete => self.0 = !self.0,
            BackgroundMessage::Start => {
                context.spawn_background(|_| BackgroundMessage::Complete);
            }
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(self.0.to_string()).into()
    }
}

#[derive(Clone)]
struct BackgroundRootInput {
    count: usize,
    sender: Rc<RefCell<Option<LocalSender<BackgroundMessage>>>>,
}

impl PartialEq for BackgroundRootInput {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct BackgroundRoot(BackgroundRootInput);

impl Component for BackgroundRoot {
    type Input = BackgroundRootInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().keyed_children((0..self.0.count).map(|index| {
            let view = if index == self.0.count / 2 {
                View::component::<BackgroundLeaf>(BackgroundInput {
                    sender: Rc::clone(&self.0.sender),
                })
            } else {
                TextBlock::new().text("static").into()
            };
            KeyedView::new(index, view)
        }))
    }
}

impl Component for BenchLeaf {
    type Input = LeafInput;
    type Message = bool;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self { active: false }
    }

    fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self>) {}

    fn update(&mut self, toggle: Self::Message, _context: &ComponentContext<Self>) {
        if toggle {
            self.active = !self.active;
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new()
            .text(if self.active { "on" } else { "off" })
            .into()
    }
}

struct BenchRoot {
    senders: Rc<Vec<Rc<RefCell<Option<LocalSender<bool>>>>>>,
}

#[derive(Clone)]
struct RootInput(Rc<Vec<Rc<RefCell<Option<LocalSender<bool>>>>>>);

impl PartialEq for RootInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Component for BenchRoot {
    type Input = RootInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            senders: Rc::clone(&input.0),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.senders = Rc::clone(&input.0);
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().keyed_children(self.senders.iter().enumerate().map(|(index, sender)| {
            KeyedView::new(
                index,
                View::component::<BenchLeaf>(LeafInput {
                    sender: Rc::clone(sender),
                }),
            )
        }))
    }
}

struct BenchFragmentRoot(RootInput);

impl Component for BenchFragmentRoot {
    type Input = RootInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let fragment = self.0.0.iter().enumerate().map(|(index, sender)| {
            KeyedView::new(
                index,
                View::component::<BenchLeaf>(LeafInput {
                    sender: Rc::clone(sender),
                }),
            )
        });
        StackPanel::new().children((View::keyed_fragment(fragment),))
    }
}

#[derive(Clone)]
struct ContextOwnerInput {
    context: Rc<Context<bool>>,
    sender: Rc<RefCell<Option<LocalSender<bool>>>>,
}

impl PartialEq for ContextOwnerInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context) && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct ContextConsumer(Rc<Context<bool>>);

impl Component for ContextConsumer {
    type Input = Rc<Context<bool>>;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(Rc::clone(input))
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = Rc::clone(input);
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        TextBlock::new()
            .text(context.use_context(&self.0).to_string())
            .into()
    }
}

struct ContextOwner {
    input: ContextOwnerInput,
    value: bool,
}

impl Component for ContextOwner {
    type Input = ContextOwnerInput;
    type Message = bool;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self {
            input: input.clone(),
            value: false,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, value: bool, _context: &ComponentContext<Self>) {
        self.value = value;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::provide(
            &self.input.context,
            self.value,
            View::component::<ContextConsumer>(Rc::clone(&self.input.context)),
        )
    }
}

#[derive(Clone)]
struct ContextSubtreeInput {
    all_consumers: bool,
    context: Rc<Context<bool>>,
    count: usize,
}

impl PartialEq for ContextSubtreeInput {
    fn eq(&self, other: &Self) -> bool {
        self.all_consumers == other.all_consumers
            && self.count == other.count
            && Rc::ptr_eq(&self.context, &other.context)
    }
}

struct ContextSubtree(ContextSubtreeInput);

impl Component for ContextSubtree {
    type Input = ContextSubtreeInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().keyed_children((0..self.0.count).map(|index| {
            let view = if self.0.all_consumers || index == self.0.count / 2 {
                View::component::<ContextConsumer>(Rc::clone(&self.0.context))
            } else {
                TextBlock::new().text("static").into()
            };
            KeyedView::new(index, view)
        }))
    }
}

#[derive(Clone)]
struct ContextBroadOwnerInput {
    context: Rc<Context<bool>>,
    sender: Rc<RefCell<Option<LocalSender<bool>>>>,
    subtree: ContextSubtreeInput,
}

impl PartialEq for ContextBroadOwnerInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.sender, &other.sender)
            && self.subtree == other.subtree
    }
}

struct ContextBroadOwner {
    input: ContextBroadOwnerInput,
    value: bool,
}

impl Component for ContextBroadOwner {
    type Input = ContextBroadOwnerInput;
    type Message = bool;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self {
            input: input.clone(),
            value: false,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, value: bool, _context: &ComponentContext<Self>) {
        self.value = value;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::provide(
            &self.input.context,
            self.value,
            View::component::<ContextSubtree>(self.input.subtree.clone()),
        )
    }
}

#[derive(Clone)]
struct ContextRootInput {
    context: Rc<Context<bool>>,
    owner: Rc<RefCell<Option<LocalSender<bool>>>>,
    senders: Rc<Vec<Rc<RefCell<Option<LocalSender<bool>>>>>>,
}

impl PartialEq for ContextRootInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.owner, &other.owner)
            && Rc::ptr_eq(&self.senders, &other.senders)
    }
}

struct ContextRoot(ContextRootInput);

impl Component for ContextRoot {
    type Input = ContextRootInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let middle = self.0.senders.len() / 2;
        StackPanel::new().keyed_children(self.0.senders.iter().enumerate().map(
            |(index, sender)| {
                let view = if index == middle {
                    View::component::<ContextOwner>(ContextOwnerInput {
                        context: Rc::clone(&self.0.context),
                        sender: Rc::clone(&self.0.owner),
                    })
                } else {
                    View::component::<BenchLeaf>(LeafInput {
                        sender: Rc::clone(sender),
                    })
                };
                KeyedView::new(index, view)
            },
        ))
    }
}

#[derive(Clone)]
struct ManyProviderRootInput {
    context: Rc<Context<bool>>,
    owners: Rc<Vec<Rc<RefCell<Option<LocalSender<bool>>>>>>,
}

impl PartialEq for ManyProviderRootInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context) && Rc::ptr_eq(&self.owners, &other.owners)
    }
}

struct ManyProviderRoot(ManyProviderRootInput);

impl Component for ManyProviderRoot {
    type Input = ManyProviderRootInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().keyed_children(self.0.owners.iter().enumerate().map(|(index, owner)| {
            KeyedView::new(
                index,
                View::component::<ContextOwner>(ContextOwnerInput {
                    context: Rc::clone(&self.0.context),
                    sender: Rc::clone(owner),
                }),
            )
        }))
    }
}

#[derive(Clone)]
struct KeyedRootInput(Rc<Vec<u64>>);

impl PartialEq for KeyedRootInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

struct KeyedRoot(KeyedRootInput);

impl Component for KeyedRoot {
    type Input = KeyedRootInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().keyed_children(
            self.0
                .0
                .iter()
                .map(|key| KeyedView::new(*key, View::component::<KeyedLeaf>(*key))),
        )
    }
}

struct KeyedLeaf(u64);

impl Component for KeyedLeaf {
    type Input = u64;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(*input)
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = *input;
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(self.0.to_string()).into()
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
        let bytes = allocator::allocated_bytes();
        let allocs = allocator::allocations();
        let start = Instant::now();
        for _ in 0..iters {
            op();
        }
        let ns = start.elapsed().as_nanos() as f64 / iters as f64;
        if ns < best.ns {
            best.ns = ns;
            best.bytes = (allocator::allocated_bytes() - bytes) as f64 / iters as f64;
            best.allocs = (allocator::allocations() - allocs) as f64 / iters as f64;
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
    let bytes = allocator::allocated_bytes();
    let allocs = allocator::allocations();
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
        bytes: (allocator::allocated_bytes() - bytes) as f64 / (samples * batch) as f64,
        allocs: (allocator::allocations() - allocs) as f64 / (samples * batch) as f64,
    }
}

fn runtime() -> RecordingRuntime {
    let mut runtime = RecordingRuntime::default();
    runtime.record_commands(false);
    runtime
}

fn indexed_stack(labels: &[String]) -> View {
    StackPanel::new().keyed_children(
        labels
            .iter()
            .enumerate()
            .map(|(index, text)| KeyedView::new(index, TextBlock::new().text(text))),
    )
}

fn keyed_stack(keys: &[String]) -> View {
    StackPanel::new().keyed_children(
        keys.iter()
            .map(|key| KeyedView::new(key.clone(), TextBlock::new().text(key.clone()))),
    )
}

fn virtual_list(key_prefix: &str, text_prefix: &str, count: usize) -> View {
    ItemsRepeater::new()
        .items((0..count).map(|index| {
            KeyedView::new(
                format!("{key_prefix}{index}"),
                TextBlock::new().text(format!("{text_prefix}{index}")),
            )
        }))
        .into()
}

fn bench_update(name: &'static str, n: usize, a: View, b: View, iters: u64, reps: u32) -> Row {
    let mut pump = Pump::new(runtime());
    pump.mount_view(a.clone()).unwrap();
    let mut flip = false;
    let perf = measure(iters, reps, || {
        pump.update_view(if flip { a.clone() } else { b.clone() })
            .unwrap();
        flip = !flip;
    });
    Row { name, n, perf }
}

fn bench_mount_shutdown(name: &'static str, n: usize, view: View, iters: u64, reps: u32) -> Row {
    let perf = measure(iters, reps, || {
        let mut pump = Pump::new(runtime());
        pump.mount_view(view.clone()).unwrap();
        pump.shutdown();
    });
    Row { name, n, perf }
}

fn bench_reference_mount(n: usize, iters: u64, reps: u32) -> Row {
    let references = (0..n).map(|_| ElementRef::new()).collect::<Vec<_>>();
    let view =
        StackPanel::new().keyed_children(references.iter().enumerate().map(
            |(index, reference)| KeyedView::new(index, TextBox::new().element_ref(reference)),
        ));
    bench_mount_shutdown("reference_mount", n, view, iters, reps)
}

fn bench_textbox_mount(n: usize, iters: u64, reps: u32) -> Row {
    let view =
        StackPanel::new().keyed_children((0..n).map(|index| KeyedView::new(index, TextBox::new())));
    bench_mount_shutdown("textbox_mount", n, view, iters, reps)
}

fn bench_component_keyed(
    name: &'static str,
    n: usize,
    a: Vec<u64>,
    b: Vec<u64>,
    iters: u64,
    reps: u32,
) -> Row {
    let a = KeyedRootInput(Rc::new(a));
    let b = KeyedRootInput(Rc::new(b));
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<KeyedRoot>(a.clone()))
        .unwrap();
    let mut flip = false;
    let perf = measure(iters, reps, || {
        let input = if flip { a.clone() } else { b.clone() };
        pump.update_view(View::component::<KeyedRoot>(input))
            .unwrap();
        flip = !flip;
    });
    Row { name, n, perf }
}

fn queue_realize(pump: &mut Pump<RecordingRuntime>, count: usize) {
    let collection = pump.root().unwrap();
    for index in 0..count {
        pump.runtime_mut()
            .queue_realize(collection, RealizedContainer(index as u64), index);
    }
    pump.process_realizations().unwrap();
}

fn queue_recycle(pump: &mut Pump<RecordingRuntime>, count: usize) {
    let collection = pump.root().unwrap();
    for index in 0..count {
        pump.runtime_mut()
            .queue_recycle(collection, RealizedContainer(index as u64));
    }
    pump.process_realizations().unwrap();
}

fn bench_virtual_payload(count: usize, realized: usize, iters: u64, reps: u32) -> Row {
    let a = virtual_list("key-", "a-", count);
    let b = virtual_list("key-", "b-", count);
    let mut pump = Pump::new(runtime());
    pump.mount_view(a.clone()).unwrap();
    queue_realize(&mut pump, realized);
    let mut flip = false;
    let perf = measure(iters, reps, || {
        pump.update_view(if flip { a.clone() } else { b.clone() })
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
    pump.mount_view(a.clone()).unwrap();
    queue_realize(&mut pump, realized);
    let mut flip = false;
    let perf = measure(iters, reps, || {
        pump.update_view(if flip { a.clone() } else { b.clone() })
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
    pump.mount_view(virtual_list("key-", "row-", count))
        .unwrap();
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
    pump.mount_view(View::component::<BenchRoot>(RootInput(Rc::clone(&senders))))
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

fn bench_component_no_change(count: usize, samples: usize) -> FrontendRow {
    let senders = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<BenchRoot>(RootInput(Rc::clone(&senders))))
        .unwrap();
    let sender = senders[count / 2].borrow().as_ref().unwrap().clone();
    measure_frontend("components", "no_change", count, samples, 1, || {
        _ = sender.send(false);
        pump.dispatch_components(1).unwrap();
    })
}

fn bench_component_isolated_leaf(count: usize, samples: usize) -> FrontendRow {
    let senders = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<BenchRoot>(RootInput(Rc::clone(&senders))))
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
    pump.mount_view(View::component::<BenchFragmentRoot>(RootInput(Rc::clone(
        &senders,
    ))))
    .unwrap();
    let sender = senders[count / 2].borrow().as_ref().unwrap().clone();
    measure_frontend("components", "fragment_leaf", count, samples, 1, || {
        _ = sender.send(true);
        pump.dispatch_components(1).unwrap();
    })
}

fn bench_context_isolated_provider(count: usize, samples: usize) -> FrontendRow {
    let senders = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let owner = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<ContextRoot>(ContextRootInput {
        context: Rc::new(Context::new(false)),
        owner: Rc::clone(&owner),
        senders,
    }))
    .unwrap();
    let sender = owner.borrow().as_ref().unwrap().clone();
    let mut value = false;
    measure_frontend("components", "context_provider", count, samples, 1, || {
        value = !value;
        _ = sender.send(value);
        pump.dispatch_components(1).unwrap();
    })
}

fn bench_context_broad_provider(count: usize, all_consumers: bool, samples: usize) -> FrontendRow {
    let context = Rc::new(Context::new(false));
    let owner = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<ContextBroadOwner>(
        ContextBroadOwnerInput {
            context: Rc::clone(&context),
            sender: Rc::clone(&owner),
            subtree: ContextSubtreeInput {
                all_consumers,
                context,
                count,
            },
        },
    ))
    .unwrap();
    let sender = owner.borrow().as_ref().unwrap().clone();
    let mut value = false;
    measure_frontend(
        "components",
        if all_consumers {
            "context_all"
        } else {
            "context_broad"
        },
        count,
        samples,
        1,
        || {
            value = !value;
            _ = sender.send(value);
            pump.dispatch_components(1).unwrap();
        },
    )
}

fn bench_context_many_providers(count: usize, samples: usize) -> FrontendRow {
    let owners = Rc::new(
        (0..count)
            .map(|_| Rc::new(RefCell::new(None)))
            .collect::<Vec<_>>(),
    );
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<ManyProviderRoot>(ManyProviderRootInput {
        context: Rc::new(Context::new(false)),
        owners: Rc::clone(&owners),
    }))
    .unwrap();
    let sender = owners[count / 2].borrow().as_ref().unwrap().clone();
    let mut value = false;
    measure_frontend("components", "context_many", count, samples, 1, || {
        value = !value;
        _ = sender.send(value);
        pump.dispatch_components(1).unwrap();
    })
}

fn bench_background_task(count: usize, samples: usize) -> FrontendRow {
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(runtime());
    pump.mount_view(View::component::<BackgroundRoot>(BackgroundRootInput {
        count,
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    let sender = sender.borrow().as_ref().unwrap().clone();
    measure_frontend("components", "background_task", count, samples, 1, || {
        _ = sender.send(BackgroundMessage::Start);
        pump.dispatch_components(1).unwrap();
        while !pump.native_work_pending() {
            std::thread::yield_now();
        }
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
    let before = allocator::current_bytes();
    pump.mount_view(View::component::<BenchRoot>(RootInput(senders)))
        .unwrap();
    let bytes = allocator::current_bytes() - before;
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
    let mut rotated = labels.clone();
    rotated.rotate_left(1);
    let all_changed: Vec<_> = (0..512).map(|index| format!("changed-{index}")).collect();
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
    let mut component_moved_10_4k = component_keys_4k.clone();
    component_moved_10_4k.rotate_left(410);
    let mut component_moved_20_4k = component_keys_4k.clone();
    component_moved_20_4k.rotate_left(819);
    let mut component_moved_25_4k = component_keys_4k.clone();
    component_moved_25_4k.rotate_left(1_024);

    let rows = vec![
        bench_mount_shutdown(
            "mount_shutdown",
            512,
            indexed_stack(&labels),
            (iters / 16).max(1),
            reps,
        ),
        bench_textbox_mount(512, (iters / 16).max(1), reps),
        bench_reference_mount(512, (iters / 16).max(1), reps),
        bench_update(
            "update_no_change",
            512,
            indexed_stack(&labels),
            indexed_stack(&labels),
            iters,
            reps,
        ),
        bench_update(
            "update_1_changed",
            512,
            indexed_stack(&labels),
            indexed_stack(&changed),
            iters,
            reps,
        ),
        bench_update(
            "update_all_changed",
            512,
            indexed_stack(&labels),
            indexed_stack(&all_changed),
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
            "keyed_rotate1",
            512,
            keyed_stack(&labels),
            keyed_stack(&rotated),
            iters,
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
            Button::new().content(TextBlock::new().text("button")),
            iters,
            reps,
        ),
        bench_update(
            "content_replace",
            2,
            Button::new().content(TextBlock::new().text("text")),
            Button::new().content(StackPanel::new().children([TextBlock::new().text("row")])),
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
            component_keys,
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
            component_keys_4k.clone(),
            component_removed_4k,
            (iters / 32).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_move_10pct",
            4_096,
            component_keys_4k.clone(),
            component_moved_10_4k,
            (iters / 32).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_move_20pct",
            4_096,
            component_keys_4k.clone(),
            component_moved_20_4k,
            (iters / 32).max(1),
            reps,
        ),
        bench_component_keyed(
            "component_move_25pct",
            4_096,
            component_keys_4k,
            component_moved_25_4k,
            (iters / 32).max(1),
            reps,
        ),
    ];

    println!("reactor-benchmark-format: 1");
    println!("windows-reactor headless reconciler micro-benchmarks");
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
        bench_component_no_change(512, samples),
        bench_component_isolated_leaf(512, samples),
        bench_component_isolated_leaf(4_096, samples),
        bench_component_isolated_leaf(16_384, samples),
        bench_component_fragment_leaf(512, samples),
        bench_component_fragment_leaf(16_384, samples),
        bench_context_isolated_provider(512, samples),
        bench_context_isolated_provider(16_384, samples),
        bench_context_broad_provider(512, false, samples),
        bench_context_broad_provider(16_384, false, samples),
        bench_context_broad_provider(512, true, samples),
        bench_context_broad_provider(16_384, true, (samples / 32).max(8)),
        bench_context_many_providers(512, samples),
        bench_context_many_providers(16_384, samples),
        bench_background_task(512, samples),
        bench_background_task(16_384, samples),
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
