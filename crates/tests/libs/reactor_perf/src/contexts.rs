use super::*;

struct Pending {
    target: Option<usize>,
    start: Instant,
    bytes: u64,
    calls: u64,
    views: usize,
    expected_views: usize,
    committed: Option<(f64, u64, u64)>,
}

struct Sample {
    broad: bool,
    ms: f64,
    bytes: u64,
    calls: u64,
    live: u64,
    private: usize,
    working: usize,
}

struct Measurements {
    contexts: Vec<Context<usize>>,
    root: Option<LocalSender<()>>,
    leaves: Vec<Option<LocalSender<()>>>,
    pending: Option<Pending>,
    samples: Vec<Sample>,
    views: usize,
}

#[derive(Clone)]
struct Shared(Rc<RefCell<Measurements>>);

impl PartialEq for Shared {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Shared {
    fn commit(&self, target: Option<usize>) {
        let mut state = self.0.borrow_mut();
        let views = state.views;
        if let Some(pending) = &mut state.pending
            && pending.target == target
            && pending.committed.is_none()
        {
            assert_eq!(
                views - pending.views,
                pending.expected_views,
                "unexpected leaf renders"
            );
            pending.committed = Some((
                pending.start.elapsed().as_secs_f64() * 1000.0,
                ALLOCATED_BYTES.load(Ordering::Relaxed) - pending.bytes,
                ALLOCATIONS.load(Ordering::Relaxed) - pending.calls,
            ));
        }
    }

    fn report(&self, options: Options) -> String {
        let state = self.0.borrow();
        assert_eq!(state.samples.len(), options.cycles * 2);
        let mut report = format!(
            "Context workload: {} leaves, {} contexts, {} provider depth, {} cycles\n\
             One broad/local warmup cycle excluded. Commit excludes later layout/presentation.\n\
             Live = requested live Rust bytes; private/working = process bytes at next timer tick.\n\
             action samples median_ms p95_ms mean_bytes mean_calls median_live median_private median_working live_first live_last\n",
            options.count, options.context_count, options.provider_depth, options.cycles,
        );
        for broad in [true, false] {
            let samples: Vec<_> = state.samples.iter().filter(|s| s.broad == broad).collect();
            let mut times: Vec<_> = samples.iter().map(|s| s.ms).collect();
            let mut live: Vec<_> = samples.iter().map(|s| s.live).collect();
            let mut private: Vec<_> = samples.iter().map(|s| s.private).collect();
            let mut working: Vec<_> = samples.iter().map(|s| s.working).collect();
            times.sort_by(f64::total_cmp);
            live.sort_unstable();
            private.sort_unstable();
            working.sort_unstable();
            let n = samples.len();
            writeln!(
                report,
                "{} {n} {:.4} {:.4} {:.0} {:.1} {} {} {} {} {}",
                if broad { "broad" } else { "local" },
                times[n / 2],
                times[(n * 95).div_ceil(100) - 1],
                samples.iter().map(|s| s.bytes).sum::<u64>() as f64 / n as f64,
                samples.iter().map(|s| s.calls).sum::<u64>() as f64 / n as f64,
                live[n / 2],
                private[n / 2],
                working[n / 2],
                samples[0].live,
                samples[n - 1].live,
            )
            .unwrap();
        }
        report
    }
}

#[derive(Clone, PartialEq)]
struct Input {
    options: Options,
    shared: Shared,
}

struct Root(usize);

impl Component for Root {
    type Input = Input;
    type Message = ();

    fn create(input: &Input, context: &ComponentContext<Self>) -> Self {
        input.shared.0.borrow_mut().root = Some(context.sender());
        Self(0)
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.0 += 1;
    }

    fn view(&self, input: &Input, context: &mut ViewContext<Self>) -> View {
        context
            .window_visuals(WindowVisuals::new().initial_position(ScreenPoint { x: 100, y: 100 }));
        let shared = input.shared.clone();
        context.use_effect("commit", self.0, move || {
            shared.commit(None);
            None
        });
        let mut forest = View::component::<Forest>(input.clone());
        for key in &input.shared.0.borrow().contexts {
            forest = provide(key, self.0, forest);
        }
        Grid::new()
            .children((forest, View::component::<Driver>(input.clone())))
            .into()
    }
}

struct Forest;

impl Component for Forest {
    type Input = Input;
    type Message = ();

    fn create(_: &Input, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Input, _: &mut ViewContext<Self>) -> View {
        let state = input.shared.0.borrow();
        ScrollViewer::new()
            .content(
                StackPanel::new().children(
                    (0..input.options.count)
                        .map(|index| {
                            let mut leaf: View =
                                component::<Leaf>(index, (index, input.shared.clone())).into();
                            for key in state.contexts.iter().take(input.options.provider_depth) {
                                leaf = provide(key, index, leaf);
                            }
                            leaf
                        })
                        .collect::<Vec<_>>(),
                ),
            )
            .into()
    }
}

struct Leaf(usize);

impl Component for Leaf {
    type Input = (usize, Shared);
    type Message = ();

    fn create((index, shared): &Self::Input, context: &ComponentContext<Self>) -> Self {
        shared.0.borrow_mut().leaves[*index] = Some(context.sender());
        Self(0)
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.0 += 1;
    }

    fn view(&self, (index, shared): &Self::Input, context: &mut ViewContext<Self>) -> View {
        shared.0.borrow_mut().views += 1;
        let sum: usize = shared
            .0
            .borrow()
            .contexts
            .iter()
            .map(|key| context.use_context(key))
            .sum();
        let record = shared.clone();
        let index = *index;
        context.use_effect("commit", self.0, move || {
            record.commit(Some(index));
            None
        });
        TextBlock::new()
            .text(format!("{index}: {sum}/{}", self.0))
            .into()
    }
}

struct Driver {
    step: usize,
    timer: ComponentTimer,
}

impl Component for Driver {
    type Input = Input;
    type Message = ();

    fn create(_: &Input, context: &ComponentContext<Self>) -> Self {
        Self {
            step: 0,
            timer: context.set_timeout(TICK, ()),
        }
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        let input = context.input();
        let mut state = input.shared.0.borrow_mut();
        if let Some(pending) = &state.pending {
            let Some((ms, bytes, calls)) = pending.committed else {
                assert!(
                    pending.start.elapsed() < Duration::from_secs(10),
                    "context update did not commit"
                );
                self.timer = context.set_timeout(TICK, ());
                return;
            };
            if self.step > 2 {
                let live = LIVE_BYTES.load(Ordering::Relaxed);
                let mut memory = PROCESS_MEMORY_COUNTERS_EX {
                    cb: size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
                    ..Default::default()
                };
                assert_ne!(
                    unsafe {
                        GetProcessMemoryInfo(
                            GetCurrentProcess(),
                            (&mut memory as *mut PROCESS_MEMORY_COUNTERS_EX).cast(),
                            memory.cb,
                        )
                    },
                    0
                );
                let broad = pending.target.is_none();
                state.samples.push(Sample {
                    broad,
                    ms,
                    bytes,
                    calls,
                    live,
                    private: memory.PrivateUsage,
                    working: memory.WorkingSetSize,
                });
            }
            state.pending = None;
        }
        if self.step == (input.options.cycles + 1) * 2 {
            drop(state);
            let path = std::env::current_exe()
                .unwrap()
                .with_file_name("Contexts.Reactor.report.txt");
            std::fs::write(path, input.shared.report(input.options)).unwrap();
            assert!(context.close_window());
            return;
        }
        let target = (self.step % 2 == 1).then_some((self.step / 2) % input.options.count);
        let sender = match target {
            Some(index) => state.leaves[index].as_ref().unwrap(),
            None => state.root.as_ref().unwrap(),
        }
        .clone();
        state.pending = Some(Pending {
            target,
            start: Instant::now(),
            bytes: ALLOCATED_BYTES.load(Ordering::Relaxed),
            calls: ALLOCATIONS.load(Ordering::Relaxed),
            views: state.views,
            expected_views: if target.is_some() {
                1
            } else if input.options.context_count > input.options.provider_depth {
                input.options.count
            } else {
                0
            },
            committed: None,
        });
        drop(state);
        assert!(sender.send(()));
        self.step += 1;
        self.timer = context.set_timeout(TICK, ());
    }

    fn view(&self, _: &Input, _: &mut ViewContext<Self>) -> View {
        Grid::new().width(0.0).height(0.0).into()
    }
}

pub(super) fn run(options: Options) {
    assert!(
        !options.json,
        "--json is only supported by the grid workload"
    );
    App::run_component_with_policy::<Root>(
        Input {
            options,
            shared: Shared(Rc::new(RefCell::new(Measurements {
                contexts: (0..options.context_count)
                    .map(|_| Context::new(0))
                    .collect(),
                root: None,
                leaves: vec![None; options.count],
                pending: None,
                samples: Vec::with_capacity(options.cycles * 2),
                views: 0,
            }))),
        },
        WindowPolicy::new()
            .title("Reactor context benchmark")
            .client_size(700.0, 500.0),
    )
    .unwrap();
}
