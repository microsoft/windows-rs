//! Headless coverage for the pointer-handler surface (`on_pointer_*` /
//! `on_tapped`). These exercise the reconciler's attach / detach / dedup
//! behaviour for the `PointerHandlers` modifier bundle through the
//! `RecordingBackend`, which now records every `set_pointer_handlers` call as
//! an [`Op::SetPointerHandlers`].
//!
//! The pointer dispatch itself lives in the WinUI backend (behind real
//! `PointerRoutedEventArgs`) and can only be driven by the `reactor_selftest`
//! integration harness; these tests pin down the *reactor-side* contract that
//! the editor sample and phases 1/2 rely on: a memoized callback is not
//! re-attached on re-render, and a removed handler is cleared (no token leak).

use std::cell::Cell;
use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::{
    Border, Callback, ControlId, Element, ElementExt, IntoCallback, PointerEventInfo,
    PointerHandlers, Reconciler,
};

fn noop_rr() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount(el: &Element) -> (Reconciler<RecordingBackend>, Option<ControlId>) {
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, el, None, noop_rr());
    (r, id)
}

fn update_ops(old: Element, new: Element) -> Vec<Op> {
    let (mut r, id) = mount(&old);
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&old), &new, id, noop_rr());
    r.backend.ops.clone()
}

/// Every `set_pointer_handlers` call recorded, in order. `None` means the
/// handlers were cleared (Some→None transition); `Some` means attach/update.
fn pointer_ops(ops: &[Op]) -> Vec<&Option<PointerHandlers>> {
    ops.iter()
        .filter_map(|o| match o {
            Op::SetPointerHandlers { handlers, .. } => Some(handlers),
            _ => None,
        })
        .collect()
}

#[test]
fn mount_attaches_pointer_handler() {
    let el: Element = Border::new(Element::Empty).on_pointer_moved(|_| {}).into();
    let (r, _) = mount(&el);

    let ph = pointer_ops(&r.backend.ops);
    assert_eq!(ph.len(), 1, "exactly one attach on mount, got {ph:?}");
    assert!(ph[0].as_ref().unwrap().on_pointer_moved.is_some());
}

#[test]
fn all_handlers_attach_as_a_single_bundle() {
    let el: Element = Border::new(Element::Empty)
        .on_pointer_pressed(|_| {})
        .on_pointer_released(|_| {})
        .on_pointer_moved(|_| {})
        .on_pointer_entered(|_| {})
        .on_pointer_exited(|| {})
        .into();
    let (r, _) = mount(&el);

    let ph = pointer_ops(&r.backend.ops);
    assert_eq!(ph.len(), 1, "handlers attach as one bundle, not per-slot");
    let h = ph[0].as_ref().unwrap();
    assert!(h.on_pointer_pressed.is_some());
    assert!(h.on_pointer_released.is_some());
    assert!(h.on_pointer_moved.is_some());
    assert!(h.on_pointer_entered.is_some());
    assert!(h.on_pointer_exited.is_some());
}

#[test]
fn stable_callback_is_not_reattached_on_rerender() {
    // A memoized callback (same `Rc`, as produced by `cx.use_callback`) must
    // compare equal across renders so the reconciler skips re-attaching — the
    // perf-critical behaviour for drag/hover handlers that fire continuously.
    let cb: Callback<PointerEventInfo> = (|_: PointerEventInfo| {}).into_callback();
    let old: Element = Border::new(Element::Empty)
        .on_pointer_moved(cb.clone())
        .into();
    let new: Element = Border::new(Element::Empty).on_pointer_moved(cb).into();

    let ops = update_ops(old, new);
    assert!(
        pointer_ops(&ops).is_empty(),
        "stable callback should skip re-attach, got {ops:?}"
    );
}

#[test]
fn fresh_callback_reattaches_on_rerender() {
    // Two distinct closures (no memoization) are not equal, so the handler is
    // re-applied — this is what `cx.use_callback` exists to avoid.
    let old: Element = Border::new(Element::Empty).on_pointer_moved(|_| {}).into();
    let new: Element = Border::new(Element::Empty).on_pointer_moved(|_| {}).into();

    let ops = update_ops(old, new);
    let ph = pointer_ops(&ops);
    assert_eq!(ph.len(), 1, "fresh closure should re-attach, got {ops:?}");
    assert!(ph[0].as_ref().unwrap().on_pointer_moved.is_some());
}

#[test]
fn removing_handler_clears_to_none() {
    // Some→None must emit a clear so the previously-attached token is released
    // (no leak), rather than silently leaving the old handler bound.
    let old: Element = Border::new(Element::Empty).on_pointer_moved(|_| {}).into();
    let new: Element = Border::new(Element::Empty).into();

    let ops = update_ops(old, new);
    let ph = pointer_ops(&ops);
    assert_eq!(ph.len(), 1, "removal should emit a clear, got {ops:?}");
    assert!(ph[0].is_none(), "Some→None should clear to None");
}

#[test]
fn changing_handler_slot_reapplies_with_new_set() {
    let old: Element = Border::new(Element::Empty).on_pointer_moved(|_| {}).into();
    let new: Element = Border::new(Element::Empty)
        .on_pointer_pressed(|_| {})
        .into();

    let ops = update_ops(old, new);
    let ph = pointer_ops(&ops);
    assert_eq!(ph.len(), 1);
    let h = ph[0].as_ref().unwrap();
    assert!(h.on_pointer_pressed.is_some(), "new slot present");
    assert!(h.on_pointer_moved.is_none(), "old slot gone");
}

#[test]
fn recorded_handler_invokes_with_pointer_info() {
    // The bundle records the live callbacks, so we can drive one headlessly and
    // confirm the wired closure runs and receives the `PointerEventInfo`.
    let seen: Rc<Cell<Option<PointerEventInfo>>> = Rc::new(Cell::new(None));
    let el: Element = {
        let seen = seen.clone();
        Border::new(Element::Empty)
            .on_pointer_moved(move |info| seen.set(Some(info)))
            .into()
    };
    let (r, _) = mount(&el);

    let bundle = pointer_ops(&r.backend.ops)[0]
        .clone()
        .expect("handlers attached");
    let info = PointerEventInfo {
        x: 12.0,
        y: 34.0,
        is_left_button_pressed: true,
        is_right_button_pressed: false,
        is_middle_button_pressed: false,
    };
    bundle.on_pointer_moved.unwrap().invoke(info);

    let got = seen.get().expect("callback fired");
    assert_eq!(got.x, 12.0);
    assert_eq!(got.y, 34.0);
    assert!(got.is_left_button_pressed);
    assert!(!got.is_right_button_pressed);
}
