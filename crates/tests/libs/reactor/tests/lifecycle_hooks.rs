use std::cell::Cell;
use std::rc::Rc;

use test_reactor::RecordingBackend;
use windows_reactor::Component;
use windows_reactor::Reconciler;
use windows_reactor::RenderCx;
use windows_reactor::component;
use windows_reactor::list_view;
use windows_reactor::{Element, TextBlock};

#[derive(Clone)]
struct Hooked {
    appeared: Rc<Cell<i32>>,
    disappeared: Rc<Cell<i32>>,
    label: String,
}

impl PartialEq for Hooked {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.appeared, &other.appeared)
            && Rc::ptr_eq(&self.disappeared, &other.disappeared)
            && self.label == other.label
    }
}

struct HookView;

impl Component<Hooked> for HookView {
    fn render(&self, props: &Hooked, _cx: &mut RenderCx) -> Element {
        TextBlock::new(props.label.clone()).into()
    }
    fn has_on_appeared(&self) -> bool {
        true
    }
    fn has_on_disappeared(&self) -> bool {
        true
    }
    fn on_appeared(&self, props: &Hooked, _cx: &mut RenderCx) {
        props.appeared.set(props.appeared.get() + 1);
    }
    fn on_disappeared(&self, props: &Hooked, _cx: &mut RenderCx) {
        props.disappeared.set(props.disappeared.get() + 1);
    }
}

#[derive(Clone)]
struct Recycled {
    appeared: Rc<Cell<i32>>,
    disappeared: Rc<Cell<i32>>,
    setups: Rc<Cell<i32>>,
    cleanups: Rc<Cell<i32>>,
}

impl PartialEq for Recycled {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.appeared, &other.appeared)
            && Rc::ptr_eq(&self.disappeared, &other.disappeared)
            && Rc::ptr_eq(&self.setups, &other.setups)
            && Rc::ptr_eq(&self.cleanups, &other.cleanups)
    }
}

struct RecycledView;

impl Component<Recycled> for RecycledView {
    fn render(&self, props: &Recycled, cx: &mut RenderCx) -> Element {
        let setups = Rc::clone(&props.setups);
        let cleanups = Rc::clone(&props.cleanups);
        cx.use_effect_with_cleanup((), move || {
            setups.set(setups.get() + 1);
            Some(move || cleanups.set(cleanups.get() + 1))
        });
        TextBlock::new("recycled").into()
    }

    fn has_on_appeared(&self) -> bool {
        true
    }

    fn has_on_disappeared(&self) -> bool {
        true
    }

    fn on_appeared(&self, props: &Recycled, _cx: &mut RenderCx) {
        props.appeared.set(props.appeared.get() + 1);
    }

    fn on_disappeared(&self, props: &Recycled, _cx: &mut RenderCx) {
        props.disappeared.set(props.disappeared.get() + 1);
    }
}

fn noop() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

#[test]
fn on_appeared_fires_on_realization() {
    let appeared = Rc::new(Cell::new(0));
    let disappeared = Rc::new(Cell::new(0));
    let a1 = Rc::clone(&appeared);
    let d1 = Rc::clone(&disappeared);

    let el = list_view(vec![0u32, 1, 2, 3], move |n, _| {
        component(
            HookView,
            Hooked {
                appeared: Rc::clone(&a1),
                disappeared: Rc::clone(&d1),
                label: format!("row-{n}"),
            },
        )
    })
    .build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();

    assert_eq!(appeared.get(), 0, "on_appeared must not fire at mount time");
    assert_eq!(disappeared.get(), 0);

    r.backend.simulate_prepare_row(list_id, 2);
    r.drain_realizations();
    assert_eq!(appeared.get(), 1, "on_appeared should fire once on realize");
    assert_eq!(disappeared.get(), 0);
}

#[test]
fn on_disappeared_fires_on_recycle() {
    let appeared = Rc::new(Cell::new(0));
    let disappeared = Rc::new(Cell::new(0));
    let a1 = Rc::clone(&appeared);
    let d1 = Rc::clone(&disappeared);

    let el = list_view(vec![0u32, 1, 2], move |n, _| {
        component(
            HookView,
            Hooked {
                appeared: Rc::clone(&a1),
                disappeared: Rc::clone(&d1),
                label: format!("row-{n}"),
            },
        )
    })
    .build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();
    r.backend.simulate_prepare_row(list_id, 0);
    r.drain_realizations();
    assert_eq!(appeared.get(), 1);

    r.backend.simulate_clear_row(list_id, 0);
    r.drain_realizations();
    assert_eq!(
        disappeared.get(),
        1,
        "on_disappeared should fire on recycle"
    );
}

#[test]
fn on_appeared_does_not_fire_during_initial_mount_of_offscreen_rows() {
    let appeared = Rc::new(Cell::new(0));
    let disappeared = Rc::new(Cell::new(0));
    let a1 = Rc::clone(&appeared);
    let d1 = Rc::clone(&disappeared);

    let el = list_view((0..1000u32).collect::<Vec<_>>(), move |n, _| {
        component(
            HookView,
            Hooked {
                appeared: Rc::clone(&a1),
                disappeared: Rc::clone(&d1),
                label: format!("row-{n}"),
            },
        )
    })
    .build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();

    assert_eq!(appeared.get(), 0);
    assert_eq!(disappeared.get(), 0);
}

#[test]
fn multiple_realize_recycle_cycles_are_counted_independently() {
    let appeared = Rc::new(Cell::new(0));
    let disappeared = Rc::new(Cell::new(0));
    let a1 = Rc::clone(&appeared);
    let d1 = Rc::clone(&disappeared);

    let el = list_view(vec![0u32, 1, 2, 3, 4], move |n, _| {
        component(
            HookView,
            Hooked {
                appeared: Rc::clone(&a1),
                disappeared: Rc::clone(&d1),
                label: format!("row-{n}"),
            },
        )
    })
    .build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();

    for i in 0..3 {
        r.backend.simulate_prepare_row(list_id, i);
    }
    r.drain_realizations();
    assert_eq!(appeared.get(), 3);

    r.backend.simulate_clear_row(list_id, 0);
    r.backend.simulate_prepare_row(list_id, 3);
    r.drain_realizations();
    assert_eq!(appeared.get(), 4);
    assert_eq!(disappeared.get(), 1);
}

#[test]
fn repeated_recycle_cycles_balance_lifecycle_and_effect_ownership() {
    const CYCLES: i32 = 64;

    let appeared = Rc::new(Cell::new(0));
    let disappeared = Rc::new(Cell::new(0));
    let setups = Rc::new(Cell::new(0));
    let cleanups = Rc::new(Cell::new(0));
    let props = Recycled {
        appeared: Rc::clone(&appeared),
        disappeared: Rc::clone(&disappeared),
        setups: Rc::clone(&setups),
        cleanups: Rc::clone(&cleanups),
    };
    let el = list_view(vec![0u32], move |_, _| {
        component(RecycledView, props.clone())
    })
    .build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();

    for cycle in 1..=CYCLES {
        r.backend.simulate_prepare_row(list_id, 0);
        r.drain_realizations();
        assert_eq!(appeared.get(), cycle);
        assert_eq!(setups.get(), cycle);
        assert_eq!(r.debug_appeared_listener_count(), 1);
        assert_eq!(r.debug_disappeared_listener_count(), 1);

        r.backend.simulate_clear_row(list_id, 0);
        r.drain_realizations();
        assert_eq!(disappeared.get(), cycle);
        assert_eq!(cleanups.get(), cycle);
        assert_eq!(r.debug_appeared_listener_count(), 0);
        assert_eq!(r.debug_disappeared_listener_count(), 0);
        assert!(r.backend.row_contents_of(list_id).is_empty());
    }
}

#[test]
fn lifecycle_hooks_default_to_noop_on_plain_component() {
    #[derive(Clone, PartialEq)]
    struct Plain {
        label: String,
    }
    struct PlainView;
    impl Component<Plain> for PlainView {
        fn render(&self, props: &Plain, _cx: &mut RenderCx) -> Element {
            TextBlock::new(props.label.clone()).into()
        }
    }

    let el = list_view(vec![1u32, 2, 3], |n, _| {
        component(
            PlainView,
            Plain {
                label: format!("{n}"),
            },
        )
    })
    .build();
    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();
    r.backend.simulate_prepare_row(list_id, 0);
    r.backend.simulate_clear_row(list_id, 0);
    r.drain_realizations();
}

#[test]
fn listener_counters_track_mounted_components() {
    let appeared = Rc::new(Cell::new(0));
    let disappeared = Rc::new(Cell::new(0));

    let el = list_view(vec![0u32, 1, 2], {
        let a1 = Rc::clone(&appeared);
        let d1 = Rc::clone(&disappeared);
        move |n, _| {
            component(
                HookView,
                Hooked {
                    appeared: Rc::clone(&a1),
                    disappeared: Rc::clone(&d1),
                    label: format!("row-{n}"),
                },
            )
        }
    })
    .build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();
    assert_eq!(r.debug_appeared_listener_count(), 0);
    assert_eq!(r.debug_disappeared_listener_count(), 0);

    r.backend.simulate_prepare_row(list_id, 1);
    r.drain_realizations();
    assert_eq!(r.debug_appeared_listener_count(), 1);
    assert_eq!(r.debug_disappeared_listener_count(), 1);
    assert_eq!(appeared.get(), 1);

    r.backend.simulate_prepare_row(list_id, 2);
    r.drain_realizations();
    assert_eq!(r.debug_appeared_listener_count(), 2);
    assert_eq!(r.debug_disappeared_listener_count(), 2);

    r.backend.simulate_clear_row(list_id, 1);
    r.drain_realizations();
    assert_eq!(r.debug_appeared_listener_count(), 1);
    assert_eq!(r.debug_disappeared_listener_count(), 1);
    assert_eq!(disappeared.get(), 1);

    r.backend.simulate_clear_row(list_id, 2);
    r.drain_realizations();
    assert_eq!(r.debug_appeared_listener_count(), 0);
    assert_eq!(r.debug_disappeared_listener_count(), 0);
}

#[test]
fn dispatch_walks_short_circuit_when_no_component_opts_in() {
    #[derive(Clone, PartialEq)]
    struct Plain {
        label: String,
    }
    struct PlainView;
    impl Component<Plain> for PlainView {
        fn render(&self, props: &Plain, _cx: &mut RenderCx) -> Element {
            TextBlock::new(props.label.clone()).into()
        }
        fn on_appeared(&self, _props: &Plain, _cx: &mut RenderCx) {
            panic!("on_appeared must not fire without has_on_appeared opt-in");
        }
    }

    let el = list_view(vec![0u32, 1, 2], |n, _| {
        component(
            PlainView,
            Plain {
                label: format!("{n}"),
            },
        )
    })
    .build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let list_id = r.reconcile(None, &el, None, noop()).unwrap();
    r.drain_realizations();
    r.backend.simulate_prepare_row(list_id, 0);
    r.backend.simulate_prepare_row(list_id, 1);
    r.drain_realizations();
    assert_eq!(r.debug_appeared_listener_count(), 0);
    assert_eq!(r.debug_disappeared_listener_count(), 0);

    r.backend.simulate_clear_row(list_id, 0);
    r.drain_realizations();
}
