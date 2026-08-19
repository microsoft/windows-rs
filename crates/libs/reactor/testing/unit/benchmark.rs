//! Crate-internal headless model benchmarks for `windows-reactor`.
//!
//! The runtime counts commands without storing or cloning them. Timings and heap traffic therefore
//! cover element construction, reconciliation, arena mutation, and protocol generation, but not
//! WinUI work or recording-runtime assertions.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use super::*;

static BYTES: AtomicU64 = AtomicU64::new(0);
static ALLOCS: AtomicU64 = AtomicU64::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() && size > layout.size() {
            BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        pointer
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

pub(crate) fn allocated_bytes() -> u64 {
    BYTES.load(Ordering::Relaxed)
}

#[derive(Default)]
struct Metrics {
    commands: Cell<u64>,
    list: Cell<Option<NodeId>>,
}

#[derive(Default)]
struct BenchRuntime {
    metrics: Rc<Metrics>,
    events: RefCell<VecDeque<NativeEvent>>,
}

impl BenchRuntime {
    fn new(metrics: Rc<Metrics>) -> Self {
        Self {
            metrics,
            events: RefCell::new(VecDeque::new()),
        }
    }

    fn queue_event(&self, event: NativeEvent) {
        self.events.borrow_mut().push_back(event);
    }
}

impl NativeRuntime for BenchRuntime {
    fn apply(&mut self, commands: &[Command]) {
        self.metrics
            .commands
            .set(self.metrics.commands.get() + commands.len() as u64);
        for command in commands {
            if let Command::Create {
                id,
                kind: NativeKind::ListView,
            } = command
            {
                self.metrics.list.set(Some(*id));
            }
        }
    }

    fn drain_events(&mut self) -> Vec<NativeEvent> {
        self.events.borrow_mut().drain(..).collect()
    }
}

struct Perf {
    ns: f64,
    bytes: f64,
    allocs: f64,
    commands: f64,
}

fn measure(iters: u64, reps: u32, metrics: &Metrics, mut operation: impl FnMut()) -> Perf {
    for _ in 0..2 {
        for _ in 0..iters {
            operation();
        }
    }

    let mut best = Perf {
        ns: f64::MAX,
        bytes: 0.0,
        allocs: 0.0,
        commands: 0.0,
    };
    for _ in 0..reps {
        let bytes = BYTES.load(Ordering::Relaxed);
        let allocs = ALLOCS.load(Ordering::Relaxed);
        let commands = metrics.commands.get();
        let start = Instant::now();
        for _ in 0..iters {
            operation();
        }
        let ns = start.elapsed().as_nanos() as f64 / iters as f64;
        if ns < best.ns {
            best = Perf {
                ns,
                bytes: (BYTES.load(Ordering::Relaxed) - bytes) as f64 / iters as f64,
                allocs: (ALLOCS.load(Ordering::Relaxed) - allocs) as f64 / iters as f64,
                commands: (metrics.commands.get() - commands) as f64 / iters as f64,
            };
        }
    }
    best
}

struct Row {
    name: &'static str,
    n: usize,
    perf: Perf,
}

fn mount_tree(n: usize) -> Element {
    stack_panel((0..n).map(|index| text_block(format!("item {index}"))))
}

fn bench_mount_drop(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let perf = measure(iters, reps, &metrics, || {
        let runtime = BenchRuntime::new(Rc::clone(&metrics));
        let mut reactor = Reactor::new(runtime, mount_tree(64));
        reactor.pump();
        drop(reactor);
    });
    Row {
        name: "mount_drop",
        n: 64,
        perf,
    }
}

fn bench_no_change(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value);
        text_block("same")
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let perf = measure(iters, reps, &metrics, || {
        assert!(state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
    });
    Row {
        name: "no_change",
        n: 1,
        perf,
    }
}

fn bench_one_change(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        text_block(if value.get().unwrap() { "a" } else { "b" })
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "one_change",
        n: 1,
        perf,
    }
}

fn bench_framework_unchanged(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value);
        TextBlock::new("same").height(24.0).build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let perf = measure(iters, reps, &metrics, || {
        assert!(state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
    });
    Row {
        name: "framework_unchanged",
        n: 1,
        perf,
    }
}

fn bench_framework_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .height(if value.get().unwrap() { 48.0 } else { 24.0 })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "framework_changed",
        n: 1,
        perf,
    }
}

fn bench_visibility_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .visibility(if value.get().unwrap() {
                Visibility::Collapsed
            } else {
                Visibility::Visible
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "visibility_changed",
        n: 1,
        perf,
    }
}

fn bench_opacity_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .opacity(if value.get().unwrap() { 0.25 } else { 0.75 })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "opacity_changed",
        n: 1,
        perf,
    }
}

fn bench_font_size_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .font_size(if value.get().unwrap() { 18.0 } else { 14.0 })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "font_size_changed",
        n: 1,
        perf,
    }
}

fn bench_font_weight_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .font_weight(if value.get().unwrap() {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "font_weight_changed",
        n: 1,
        perf,
    }
}

fn bench_font_variants_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let changed = value.get().unwrap();
        TextBlock::new("same")
            .font_style(if changed {
                FontStyle::Italic
            } else {
                FontStyle::Normal
            })
            .font_stretch(if changed {
                FontStretch::Expanded
            } else {
                FontStretch::Normal
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "font_variants_changed",
        n: 1,
        perf,
    }
}

fn bench_character_spacing_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .character_spacing(if value.get().unwrap() { 100 } else { 0 })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "character_spacing_changed",
        n: 1,
        perf,
    }
}

fn bench_text_flow_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let changed = value.get().unwrap();
        TextBlock::new("same")
            .text_wrapping(if changed {
                TextWrapping::WrapWholeWords
            } else {
                TextWrapping::Wrap
            })
            .text_trimming(if changed {
                TextTrimming::Clip
            } else {
                TextTrimming::WordEllipsis
            })
            .text_selection_enabled(changed)
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "text_flow_changed",
        n: 1,
        perf,
    }
}

fn bench_enabled_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBox::new("same", |_| {})
            .enabled(value.get().unwrap())
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "enabled_changed",
        n: 1,
        perf,
    }
}

fn bench_grid_placement_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        Grid::new([grid_child(text_block("same")).row(if value.get().unwrap() { 2 } else { 1 })])
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "grid_placement_changed",
        n: 1,
        perf,
    }
}

fn bench_canvas_placement_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        Canvas::new([
            canvas_child(text_block("same")).left(if value.get().unwrap() { 2.0 } else { 1.0 }),
        ])
        .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "canvas_placement_changed",
        n: 1,
        perf,
    }
}

fn bench_relative_panel_placement_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        RelativePanel::new([
            relative_panel_child(text_block("same")).align_left(value.get().unwrap())
        ])
        .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "relative_panel_placement_changed",
        n: 1,
        perf,
    }
}

fn bench_keyboard_accelerator_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let modifiers = if value.get().unwrap() {
            VirtualKeyModifiers::CONTROL | VirtualKeyModifiers::SHIFT
        } else {
            VirtualKeyModifiers::CONTROL
        };
        TextBlock::new("same")
            .keyboard_accelerator(KeyboardAccelerator::new(VirtualKey::S, modifiers, || {}))
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "keyboard_accelerator_changed",
        n: 1,
        perf,
    }
}

fn bench_keyboard_accelerator_handler_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let captured = value.get().unwrap();
        TextBlock::new("same")
            .keyboard_accelerator(KeyboardAccelerator::new(
                VirtualKey::S,
                VirtualKeyModifiers::CONTROL,
                move || {
                    std::hint::black_box(captured);
                },
            ))
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "keyboard_handler_changed",
        n: 1,
        perf,
    }
}

fn bench_pointer_subscription_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let target = TextBlock::new("same").on_pointer_pressed(|_| {});
        if value.get().unwrap() {
            target.capture_pointer_on_press().build()
        } else {
            target.build()
        }
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "pointer_subscription",
        n: 1,
        perf,
    }
}

fn bench_pointer_handler_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let captured = value.get().unwrap();
        TextBlock::new("same")
            .on_pointer_pressed(move |_| {
                std::hint::black_box(captured);
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "pointer_handler",
        n: 1,
        perf,
    }
}

fn bench_drop_target_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let target = if value.get().unwrap() {
            DropTarget::new(DropOperation::Move, DropFormats::STORAGE_ITEMS)
        } else {
            DropTarget::new(DropOperation::Copy, DropFormats::TEXT)
        };
        TextBlock::new("same").on_drop(target, |_| {}).build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "drop_target",
        n: 1,
        perf,
    }
}

fn bench_drop_handler_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let captured = value.get().unwrap();
        TextBlock::new("same")
            .on_drop(
                DropTarget::new(DropOperation::Copy, DropFormats::TEXT),
                move |_| {
                    std::hint::black_box(captured);
                },
            )
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "drop_handler",
        n: 1,
        perf,
    }
}

fn bench_viewbox_stretch_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        Viewbox::new(text_block("same"))
            .stretch(if value.get().unwrap() {
                Stretch::UniformToFill
            } else {
                Stretch::Uniform
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "viewbox_stretch",
        n: 1,
        perf,
    }
}

fn bench_scroll_viewer_configuration_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let changed = value.get().unwrap();
        ScrollViewer::new(text_block("same"))
            .horizontal_scroll_bar_visibility(if changed {
                ScrollBarVisibility::Visible
            } else {
                ScrollBarVisibility::Disabled
            })
            .vertical_scroll_bar_visibility(if changed {
                ScrollBarVisibility::Hidden
            } else {
                ScrollBarVisibility::Auto
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "scrollviewer_config",
        n: 1,
        perf,
    }
}

fn bench_scroll_view_configuration_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let changed = value.get().unwrap();
        ScrollView::new(text_block("same"))
            .horizontal_scroll_bar_visibility(if changed {
                ScrollViewBarVisibility::Visible
            } else {
                ScrollViewBarVisibility::Auto
            })
            .vertical_scroll_bar_visibility(if changed {
                ScrollViewBarVisibility::Hidden
            } else {
                ScrollViewBarVisibility::Auto
            })
            .content_orientation(if changed {
                ScrollOrientation::Both
            } else {
                ScrollOrientation::Vertical
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "scrollview_config",
        n: 1,
        perf,
    }
}

fn bench_scroll_handler_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let captured = value.get().unwrap();
        ScrollView::new(text_block("same"))
            .on_view_changed(move |_| {
                std::hint::black_box(captured);
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "scroll_handler",
        n: 1,
        perf,
    }
}

fn bench_split_view_configuration_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let changed = value.get().unwrap();
        SplitView::display(text_block("content"), text_block("pane"))
            .display_mode(if changed {
                SplitViewDisplayMode::CompactOverlay
            } else {
                SplitViewDisplayMode::Inline
            })
            .is_pane_open(!changed)
            .open_pane_length(if changed { 280.0 } else { 320.0 })
            .compact_pane_length(if changed { 40.0 } else { 48.0 })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "splitview_config",
        n: 1,
        perf,
    }
}

fn bench_expander_state_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        Expander::display(text_block("header"), text_block("content"))
            .expanded(value.get().unwrap())
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "expander_state",
        n: 1,
        perf,
    }
}

fn bench_tooltip_content_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        text_block("owner").tooltip(text_block(if value.get().unwrap() {
            "updated tooltip"
        } else {
            "tooltip"
        }))
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "tooltip_content",
        n: 1,
        perf,
    }
}

fn bench_font_family_unchanged(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value);
        TextBlock::new("same")
            .font_family(Some("Arial".to_string()))
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let perf = measure(iters, reps, &metrics, || {
        assert!(state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
    });
    Row {
        name: "font_family_same",
        n: 1,
        perf,
    }
}

fn bench_font_family_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .font_family(Some(if value.get().unwrap() {
                "Consolas".to_string()
            } else {
                "Arial".to_string()
            }))
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "font_family_changed",
        n: 1,
        perf,
    }
}

fn bench_foreground_unchanged(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value);
        TextBlock::new("same")
            .foreground(Color::rgb(10, 20, 30))
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let perf = measure(iters, reps, &metrics, || {
        assert!(state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
    });
    Row {
        name: "foreground_same",
        n: 1,
        perf,
    }
}

fn bench_foreground_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .foreground(if value.get().unwrap() {
                Color::rgb(40, 50, 60)
            } else {
                Color::rgb(10, 20, 30)
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "foreground_changed",
        n: 1,
        perf,
    }
}

fn bench_accessibility_unchanged(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value);
        TextBlock::new("same").automation_name("same").build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let perf = measure(iters, reps, &metrics, || {
        assert!(state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
    });
    Row {
        name: "accessibility_same",
        n: 1,
        perf,
    }
}

fn bench_accessibility_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBlock::new("same")
            .automation_name(if value.get().unwrap() { "a" } else { "b" })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "accessibility_changed",
        n: 1,
        perf,
    }
}

fn bench_dirty_component(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_root = Rc::clone(&state);
    let root = component(move |_| {
        let state = Rc::clone(&state_for_root);
        stack_panel([
            text_block("static"),
            component(move |_| {
                let state = Rc::clone(&state);
                component(move |cx| {
                    let value = cx.use_state(|| false);
                    *state.borrow_mut() = Some(value.clone());
                    text_block(if value.get().unwrap() { "a" } else { "b" })
                })
            }),
        ])
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "dirty_component",
        n: 3,
        perf,
    }
}

fn bench_keyed_reorder(n: usize, iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let shifted = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(shifted.clone());
        let mut keys = (0..n as u64).collect::<Vec<_>>();
        if shifted.get().unwrap() {
            keys.rotate_left(1);
        }
        stack_panel(keys.into_iter().map(|key| text_block("item").key(key)))
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut shifted = false;
    let perf = measure(iters, reps, &metrics, || {
        shifted = !shifted;
        assert!(state.borrow().as_ref().unwrap().try_set(shifted));
        reactor.pump();
    });
    Row {
        name: "keyed_rotate1",
        n,
        perf,
    }
}

fn bench_keyed_transition(
    name: &'static str,
    first: Vec<u64>,
    second: Vec<u64>,
    iters: u64,
    reps: u32,
) -> Row {
    let n = first.len().max(second.len());
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let first = Rc::new(first);
    let second = Rc::new(second);
    let root = component(move |cx| {
        let alternate = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(alternate.clone());
        let keys = if alternate.get().unwrap() {
            &second
        } else {
            &first
        };
        stack_panel(keys.iter().map(|key| text_block("item").key(*key)))
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut alternate = false;
    let perf = measure(iters, reps, &metrics, || {
        alternate = !alternate;
        assert!(state.borrow().as_ref().unwrap().try_set(alternate));
        reactor.pump();
    });
    Row { name, n, perf }
}

fn bench_application_validation(n: usize, iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let mut engine = Engine::new(BenchRuntime::new(Rc::clone(&metrics)));
    let application = engine.create_application().unwrap();
    let window = engine
        .create_window(WindowCreate {
            title: "benchmark".to_string(),
        })
        .unwrap();
    engine.attach(application, window).unwrap();
    let content = engine.create_logical().unwrap();
    engine.attach(window, content).unwrap();
    for _ in 0..n {
        let child = engine.create_logical().unwrap();
        engine.attach(content, child).unwrap();
    }
    let native = engine.create_native(NativeKind::TextBlock).unwrap();
    engine.attach(content, native).unwrap();
    let owned = engine.create_logical().unwrap();
    engine.attach(window, owned).unwrap();

    let perf = measure(iters, reps, &metrics, || {
        std::hint::black_box(engine.validate_application_root(application).unwrap());
    });
    Row {
        name: "application_validate",
        n,
        perf,
    }
}

fn bench_virtual_events(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let root =
        component(|_| virtual_list(1_000, 300.0, |index| text_block(format!("row {index}"))));
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let host = metrics.list.get().unwrap();
    let mut realized = false;
    let mut lease = 1;
    let perf = measure(iters, reps, &metrics, || {
        let event = if realized {
            NativeEvent::Recycle {
                host,
                index: 0,
                lease,
            }
        } else {
            NativeEvent::Realize {
                host,
                index: 0,
                lease,
            }
        };
        reactor.engine().runtime().queue_event(event);
        reactor.pump();
        realized = !realized;
        if !realized {
            lease += 1;
        }
    });
    Row {
        name: "virtual_event",
        n: 1,
        perf,
    }
}

fn bench_virtual_size_events(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let root = component(|_| {
        virtual_list(1_000, 300.0, |index| {
            TextBlock::new(format!("row {index}")).height(24.0).build()
        })
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let host = metrics.list.get().unwrap();
    let mut realized = false;
    let mut lease = 1;
    let perf = measure(iters, reps, &metrics, || {
        let event = if realized {
            NativeEvent::Recycle {
                host,
                index: 0,
                lease,
            }
        } else {
            NativeEvent::Realize {
                host,
                index: 0,
                lease,
            }
        };
        reactor.engine().runtime().queue_event(event);
        reactor.pump();
        realized = !realized;
        if !realized {
            lease += 1;
        }
    });
    Row {
        name: "virtual_size",
        n: 1,
        perf,
    }
}

fn bench_keyed_virtual_unchanged(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let keys = VirtualItemKeys::new(0..1_000);
    let root = component(move |cx| {
        let changed = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(changed.clone());
        _ = changed.get().unwrap();
        VirtualList::new(keys.len(), 300.0, |index| {
            text_block(format!("row {index}"))
        })
        .item_keys(keys.clone())
        .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut changed = false;
    let perf = measure(iters, reps, &metrics, || {
        changed = !changed;
        assert!(state.borrow().as_ref().unwrap().try_set(changed));
        reactor.pump();
    });
    Row {
        name: "virtual_keyed_same",
        n: 1_000,
        perf,
    }
}

fn bench_keyed_virtual_mutation(iters: u64, reps: u32) -> Row {
    const COUNT: usize = 1_000;
    const REALIZED: usize = 16;

    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<VirtualItemKeys>>));
    let state_for_render = Rc::clone(&state);
    let first = VirtualItemKeys::new(0..COUNT as u64);
    let mut shifted = (0..COUNT as u64).collect::<Vec<_>>();
    shifted.rotate_left(1);
    let second = VirtualItemKeys::new(shifted);
    let first_for_render = first.clone();
    let root = component(move |cx| {
        let keys = cx.use_state(|| first_for_render.clone());
        *state_for_render.borrow_mut() = Some(keys.clone());
        let keys = keys.get().unwrap();
        VirtualList::new(keys.len(), 300.0, |index| {
            text_block(format!("row {index}"))
        })
        .item_keys(keys)
        .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let host = metrics.list.get().unwrap();
    for index in 0..REALIZED {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Realize {
                host,
                index,
                lease: index as u64 + 1,
            });
    }
    reactor.pump();
    let mut use_second = false;
    let mut lease = REALIZED as u64 + 1;
    let perf = measure(iters, reps, &metrics, || {
        use_second = !use_second;
        assert!(state.borrow().as_ref().unwrap().try_set(if use_second {
            second.clone()
        } else {
            first.clone()
        }));
        reactor.pump();
        for index in 0..REALIZED {
            reactor
                .engine()
                .runtime()
                .queue_event(NativeEvent::Realize { host, index, lease });
            lease += 1;
        }
        reactor.pump();
    });
    Row {
        name: "virtual_keyed_move",
        n: COUNT,
        perf,
    }
}

fn bench_keyed_virtual_selection(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<CollectionSelection>>));
    let state_for_render = Rc::clone(&state);
    let keys = VirtualItemKeys::new(0..1_000);
    let root = component(move |cx| {
        let selection = cx.use_state(CollectionSelection::default);
        *state_for_render.borrow_mut() = Some(selection.clone());
        VirtualList::new(keys.len(), 300.0, |index| {
            text_block(format!("row {index}"))
        })
        .item_keys(keys.clone())
        .selection_mode(SelectionMode::Multiple)
        .selection(selection.get().unwrap(), |_| {})
        .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut selected = false;
    let perf = measure(iters, reps, &metrics, || {
        selected = !selected;
        assert!(
            state
                .borrow()
                .as_ref()
                .unwrap()
                .try_set(CollectionSelection::new(selected.then_some(500)))
        );
        reactor.pump();
    });
    Row {
        name: "virtual_selection",
        n: 1_000,
        perf,
    }
}

fn bench_effect_unchanged(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let starts = Rc::new(Cell::new(0_u64));
    let starts_for_render = Rc::clone(&starts);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value);
        let starts = Rc::clone(&starts_for_render);
        cx.use_effect((), move || starts.set(starts.get() + 1));
        text_block("same")
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let perf = measure(iters, reps, &metrics, || {
        assert!(state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
    });
    Row {
        name: "effect_unchanged",
        n: 1,
        perf,
    }
}

fn bench_effect_changed(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let starts = Rc::new(Cell::new(0_u64));
    let cleanups = Rc::new(Cell::new(0_u64));
    let starts_for_render = Rc::clone(&starts);
    let cleanups_for_render = Rc::clone(&cleanups);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let starts = Rc::clone(&starts_for_render);
        let cleanups = Rc::clone(&cleanups_for_render);
        let dependency = value.get().unwrap();
        cx.use_effect_with_cleanup(dependency, move || {
            starts.set(starts.get() + 1);
            move || cleanups.set(cleanups.get() + 1)
        });
        text_block("same")
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "effect_changed",
        n: 1,
        perf,
    }
}

fn bench_effect_mount_drop(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let perf = measure(iters, reps, &metrics, || {
        let root = component(|cx| {
            cx.use_effect_with_cleanup((), || || {});
            text_block("effect")
        });
        let runtime = BenchRuntime::new(Rc::clone(&metrics));
        let mut reactor = Reactor::new(runtime, root);
        reactor.pump();
        drop(reactor);
    });
    Row {
        name: "effect_mount_drop",
        n: 1,
        perf,
    }
}

fn context_tree(context: &Context<usize>, depth: usize, mut child: Element) -> Element {
    for value in 0..depth {
        child = provide_context(context, value, child);
    }
    child
}

fn bench_context_depth(depth: usize, iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let context = Context::new(0_usize);
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let context_for_render = context.clone();
    let child = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value);
        let _ = cx.use_context(&context_for_render);
        text_block("same")
    });
    let root = context_tree(&context, depth, child);
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let perf = measure(iters, reps, &metrics, || {
        assert!(state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
    });
    Row {
        name: "context_depth",
        n: depth,
        perf,
    }
}

fn bench_context_fanout(n: usize, iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let context = Context::new(0_usize);
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        let children = (0..n).map(|_| {
            let context = context.clone();
            component(move |cx| {
                let _ = cx.use_context(&context);
                text_block("same")
            })
        });
        provide_context(
            &context,
            usize::from(value.get().unwrap()),
            stack_panel(children),
        )
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "context_fanout",
        n,
        perf,
    }
}

fn bench_virtual_context_effect(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let context = Context::new(0_usize);
    let root = component(move |_| {
        let row_context = context.clone();
        provide_context(
            &context,
            1,
            virtual_list(1_000, 300.0, move |index| {
                let context = row_context.clone();
                component(move |cx| {
                    let value = cx.use_context(&context);
                    cx.use_effect_with_cleanup((), || || {});
                    text_block(format!("row {index}: {value}"))
                })
            }),
        )
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let host = metrics.list.get().unwrap();
    let mut realized = false;
    let mut lease = 1;
    let perf = measure(iters, reps, &metrics, || {
        let event = if realized {
            NativeEvent::Recycle {
                host,
                index: 0,
                lease,
            }
        } else {
            NativeEvent::Realize {
                host,
                index: 0,
                lease,
            }
        };
        reactor.engine().runtime().queue_event(event);
        reactor.pump();
        realized = !realized;
        if !realized {
            lease += 1;
        }
    });
    Row {
        name: "virtual_context_fx",
        n: 1,
        perf,
    }
}

fn bench_reference_unchanged(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        let reference = cx.use_element_ref::<TextBox>();
        *state_for_render.borrow_mut() = Some(value);
        TextBox::new("same", |_| {}).reference(&reference).build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let perf = measure(iters, reps, &metrics, || {
        assert!(state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
    });
    Row {
        name: "reference_unchanged",
        n: 1,
        perf,
    }
}

fn bench_reference_swap(iters: u64, reps: u32) -> Row {
    let metrics = Rc::new(Metrics::default());
    let state = Rc::new(RefCell::new(None::<State<bool>>));
    let first = ElementRef::<TextBox>::new();
    let second = ElementRef::<TextBox>::new();
    let state_for_render = Rc::clone(&state);
    let root = component(move |cx| {
        let value = cx.use_state(|| false);
        *state_for_render.borrow_mut() = Some(value.clone());
        TextBox::new("same", |_| {})
            .reference(if value.get().unwrap() {
                &second
            } else {
                &first
            })
            .build()
    });
    let mut reactor = Reactor::new(BenchRuntime::new(Rc::clone(&metrics)), root);
    reactor.pump();
    let mut value = false;
    let perf = measure(iters, reps, &metrics, || {
        value = !value;
        assert!(state.borrow().as_ref().unwrap().try_set(value));
        reactor.pump();
    });
    Row {
        name: "reference_swap",
        n: 1,
        perf,
    }
}

fn parse_env(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

#[test]
#[ignore = "manual release benchmark"]
fn benchmark() {
    let iters = parse_env("WINDOWS_REACTOR_BENCH_ITERS", 2_000);
    let reps = parse_env("WINDOWS_REACTOR_BENCH_REPS", 9) as u32;
    let rows = [
        bench_mount_drop((iters / 16).max(1), reps),
        bench_no_change(iters, reps),
        bench_one_change(iters, reps),
        bench_framework_unchanged(iters, reps),
        bench_framework_changed(iters, reps),
        bench_visibility_changed(iters, reps),
        bench_opacity_changed(iters, reps),
        bench_font_size_changed(iters, reps),
        bench_font_weight_changed(iters, reps),
        bench_font_variants_changed(iters, reps),
        bench_character_spacing_changed(iters, reps),
        bench_text_flow_changed(iters, reps),
        bench_font_family_unchanged(iters, reps),
        bench_font_family_changed(iters, reps),
        bench_foreground_unchanged(iters, reps),
        bench_foreground_changed(iters, reps),
        bench_grid_placement_changed(iters, reps),
        bench_canvas_placement_changed(iters, reps),
        bench_relative_panel_placement_changed(iters, reps),
        bench_keyboard_accelerator_changed(iters, reps),
        bench_keyboard_accelerator_handler_changed(iters, reps),
        bench_pointer_subscription_changed(iters, reps),
        bench_pointer_handler_changed(iters, reps),
        bench_drop_target_changed(iters, reps),
        bench_drop_handler_changed(iters, reps),
        bench_viewbox_stretch_changed(iters, reps),
        bench_scroll_viewer_configuration_changed(iters, reps),
        bench_scroll_view_configuration_changed(iters, reps),
        bench_scroll_handler_changed(iters, reps),
        bench_split_view_configuration_changed(iters, reps),
        bench_expander_state_changed(iters, reps),
        bench_tooltip_content_changed(iters, reps),
        bench_enabled_changed(iters, reps),
        bench_accessibility_unchanged(iters, reps),
        bench_accessibility_changed(iters, reps),
        bench_dirty_component(iters, reps),
        bench_keyed_reorder(64, (iters / 8).max(1), reps),
        bench_keyed_reorder(512, (iters / 64).max(1), reps),
        bench_keyed_transition(
            "keyed_same",
            (0..512).collect(),
            (0..512).collect(),
            (iters / 64).max(1),
            reps,
        ),
        bench_keyed_transition(
            "keyed_append",
            (0..512).collect(),
            (0..520).collect(),
            (iters / 64).max(1),
            reps,
        ),
        bench_keyed_transition(
            "keyed_sparse",
            (0..512).collect(),
            (0..512)
                .filter(|key| key % 64 != 0)
                .chain(512..520)
                .collect(),
            (iters / 64).max(1),
            reps,
        ),
        bench_keyed_transition(
            "keyed_reverse",
            (0..512).collect(),
            (0..512).rev().collect(),
            (iters / 64).max(1),
            reps,
        ),
        bench_application_validation(512, (iters / 64).max(1), reps),
        bench_application_validation(4096, (iters / 512).max(1), reps),
        bench_virtual_events(iters, reps),
        bench_virtual_size_events(iters, reps),
        bench_keyed_virtual_unchanged(iters, reps),
        bench_keyed_virtual_mutation((iters / 16).max(1), reps),
        bench_keyed_virtual_selection(iters, reps),
        bench_effect_unchanged(iters, reps),
        bench_effect_changed(iters, reps),
        bench_effect_mount_drop((iters / 4).max(1), reps),
        bench_context_depth(1, iters, reps),
        bench_context_depth(16, iters, reps),
        bench_context_fanout(64, (iters / 16).max(1), reps),
        bench_virtual_context_effect(iters, reps),
        bench_reference_unchanged(iters, reps),
        bench_reference_swap(iters, reps),
    ];

    println!("windows-reactor headless model benchmarks");
    println!("(BenchRuntime; best-of-reps, Rust-side cost only)\n");
    println!(
        "{:<20} {:>6} {:>12} {:>11} {:>10} {:>12}",
        "bench", "N", "ns/op", "bytes/op", "allocs/op", "commands/op"
    );
    println!("{}", "-".repeat(76));
    for row in rows {
        println!(
            "{:<20} {:>6} {:>12.1} {:>11.1} {:>10.2} {:>12.2}",
            row.name, row.n, row.perf.ns, row.perf.bytes, row.perf.allocs, row.perf.commands,
        );
    }
}
