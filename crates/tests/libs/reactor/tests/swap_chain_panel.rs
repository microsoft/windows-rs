use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::Element;
use windows_reactor::Reconciler;
use windows_reactor::{Backend, ControlId, ControlKind, Op, RecordingBackend};
use windows_reactor::{ElementExt, swap_chain_panel, text_block};

fn noop_request_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

/// Returns the id of the single `SwapChainPanel` control created so far.
fn swap_chain_panel_id(r: &Reconciler<RecordingBackend>) -> ControlId {
    r.backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::Create {
                id,
                kind: ControlKind::SwapChainPanel,
            } => Some(*id),
            _ => None,
        })
        .expect("a SwapChainPanel was created")
}

fn was_destroyed(r: &Reconciler<RecordingBackend>, id: ControlId) -> bool {
    r.backend
        .ops
        .iter()
        .any(|op| matches!(op, Op::Destroy { id: did } if *did == id))
}

fn mount(el: Element) -> (Reconciler<RecordingBackend>, Option<ControlId>) {
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &el, None, noop_request_rerender());
    (r, id)
}

#[test]
fn swap_chain_panel_factory_defaults() {
    let w = swap_chain_panel();
    assert!(w.key.is_none());
    assert!(w.modifiers.is_empty());
}

#[test]
fn swap_chain_panel_on_mounted_stores_callback() {
    let el: Element = swap_chain_panel().on_mounted(|_| {}).into();
    // If callback is stored, the element differs from one without it.
    let el_no_cb: Element = swap_chain_panel().into();
    assert_ne!(el, el_no_cb);
}

#[test]
fn swap_chain_panel_modifiers_chain() {
    let w = swap_chain_panel()
        .width(400.0)
        .height(300.0)
        .with_key("render");
    assert_eq!(w.modifiers.width, Some(400.0));
    assert_eq!(w.modifiers.height, Some(300.0));
    assert_eq!(w.key.as_deref(), Some("render"));
}

#[test]
fn mount_creates_swap_chain_panel_control() {
    let el: Element = swap_chain_panel().width(200.0).into();
    let (r, id) = mount(el);
    let id = id.expect("mount produces an id");

    let creates: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Create { id: cid, kind } => Some((*cid, *kind)),
            _ => None,
        })
        .collect();

    assert_eq!(creates.len(), 1);
    assert_eq!(creates[0], (id, ControlKind::SwapChainPanel));
}

#[test]
fn swap_chain_panel_element_equality() {
    let a: Element = swap_chain_panel().width(100.0).into();
    let b: Element = swap_chain_panel().width(100.0).into();
    let c: Element = swap_chain_panel().width(200.0).into();
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn on_unmount_stores_callback() {
    // An on_unmounted callback makes the element differ from one without it.
    let with_cb: Element = swap_chain_panel().on_unmounted(|_| {}).into();
    let without_cb: Element = swap_chain_panel().into();
    assert_ne!(with_cb, without_cb);
}

#[test]
fn on_unmount_fires_before_destroy() {
    // With native elements enabled, the pre-unmount callback fires. Because the
    // backend drops a control's native element in `destroy`, the callback can
    // only run while the element still exists — i.e. *before* destroy. If the
    // reconciler invoked it after `backend.destroy`, `get_native_element` would
    // return `None` and the callback would not fire, failing this test.
    let called = Rc::new(Cell::new(0));
    let called2 = called.clone();
    let panel: Element = swap_chain_panel()
        .on_unmounted(move |_| called2.set(called2.get() + 1))
        .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &panel, None, noop_request_rerender())
        .expect("panel mounts");
    let panel_id = swap_chain_panel_id(&r);
    assert_eq!(called.get(), 0, "must not fire before unmount");

    // Replacing the panel with a different control kind unmounts the panel.
    let replacement: Element = text_block("x").into();
    r.reconcile(
        Some(&panel),
        &replacement,
        Some(id),
        noop_request_rerender(),
    );

    assert_eq!(called.get(), 1, "on_unmount fires exactly once on teardown");
    assert!(was_destroyed(&r, panel_id), "panel was destroyed");
    assert!(
        r.backend.get_native_element(panel_id).is_none(),
        "native element is gone after destroy"
    );
}

#[test]
fn on_mounted_and_on_unmount_both_fire_across_lifecycle() {
    let ready = Rc::new(Cell::new(0));
    let unmounted = Rc::new(Cell::new(0));
    let ready2 = ready.clone();
    let unmounted2 = unmounted.clone();
    let panel: Element = swap_chain_panel()
        .on_mounted(move |_| ready2.set(ready2.get() + 1))
        .on_unmounted(move |_| unmounted2.set(unmounted2.get() + 1))
        .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &panel, None, noop_request_rerender())
        .expect("panel mounts");
    assert_eq!(ready.get(), 1, "on_mounted fires once at mount");
    assert_eq!(unmounted.get(), 0, "on_unmount has not fired yet");

    let replacement: Element = text_block("x").into();
    r.reconcile(
        Some(&panel),
        &replacement,
        Some(id),
        noop_request_rerender(),
    );
    assert_eq!(ready.get(), 1, "on_mounted does not fire again");
    assert_eq!(unmounted.get(), 1, "on_unmount fires once at unmount");
}

#[test]
fn on_unmount_callback_replaced_on_update() {
    // Re-rendering the panel with a new on_unmounted callback must replace the
    // registered one; only the latest callback fires at teardown.
    let first = Rc::new(Cell::new(0));
    let second = Rc::new(Cell::new(0));
    let first2 = first.clone();
    let second2 = second.clone();

    let panel_a: Element = swap_chain_panel()
        .width(10.0)
        .on_unmounted(move |_| first2.set(first2.get() + 1))
        .into();
    let panel_b: Element = swap_chain_panel()
        .width(20.0)
        .on_unmounted(move |_| second2.set(second2.get() + 1))
        .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &panel_a, None, noop_request_rerender())
        .expect("panel mounts");
    // In-place update swaps the callback (same control kind).
    r.reconcile(Some(&panel_a), &panel_b, Some(id), noop_request_rerender());

    let replacement: Element = text_block("x").into();
    r.reconcile(
        Some(&panel_b),
        &replacement,
        Some(id),
        noop_request_rerender(),
    );

    assert_eq!(first.get(), 0, "the replaced callback must not fire");
    assert_eq!(second.get(), 1, "the latest callback fires once");
}

#[test]
fn on_unmount_callback_removed_on_update() {
    // Re-rendering the panel without an on_unmounted callback must clear the
    // previously registered one, so nothing fires at teardown.
    let called = Rc::new(Cell::new(0));
    let called2 = called.clone();
    let panel_a: Element = swap_chain_panel()
        .width(10.0)
        .on_unmounted(move |_| called2.set(called2.get() + 1))
        .into();
    let panel_b: Element = swap_chain_panel().width(20.0).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &panel_a, None, noop_request_rerender())
        .expect("panel mounts");
    r.reconcile(Some(&panel_a), &panel_b, Some(id), noop_request_rerender());

    let replacement: Element = text_block("x").into();
    r.reconcile(
        Some(&panel_b),
        &replacement,
        Some(id),
        noop_request_rerender(),
    );

    assert_eq!(called.get(), 0, "removed callback must not fire");
}
