use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::backend::RecordingBackend;
use windows_reactor::core::component::Component;
use windows_reactor::core::component_element::component;
use windows_reactor::core::element::{Element, TextBlock};
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::core::render_context::RenderCx;
use windows_reactor::core::templated_list::list_view;

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
