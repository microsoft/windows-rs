use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::backend::{ControlKind, Op, RecordingBackend};
use windows_reactor::core::element::Element;
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::dsl::{ElementExt, swap_chain_panel};

fn noop_request_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount(
    el: Element,
) -> (
    Reconciler<RecordingBackend>,
    Option<windows_reactor::core::backend::ControlId>,
) {
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
fn swap_chain_panel_on_ready_stores_callback() {
    let el: Element = swap_chain_panel().on_ready(|_| {}).into();
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
fn on_mounted_not_called_on_recording_backend() {
    // RecordingBackend.get_native_element returns None, so callback is skipped.
    let called = Rc::new(Cell::new(false));
    let called2 = called.clone();
    let el: Element = swap_chain_panel()
        .on_ready(move |_| {
            called2.set(true);
        })
        .into();
    mount(el);
    assert!(
        !called.get(),
        "callback should not fire without a native element"
    );
}

#[test]
fn swap_chain_panel_element_equality() {
    let a: Element = swap_chain_panel().width(100.0).into();
    let b: Element = swap_chain_panel().width(100.0).into();
    let c: Element = swap_chain_panel().width(200.0).into();
    assert_eq!(a, b);
    assert_ne!(a, c);
}
