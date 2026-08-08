use std::cell::Cell;
use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Component;
use windows_reactor::Element;
use windows_reactor::Reconciler;
use windows_reactor::RenderCx;
use windows_reactor::component;
use windows_reactor::error_boundary;
use windows_reactor::text_block;

struct Boom {
    boom: Rc<Cell<bool>>,
}
impl Component for Boom {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        assert!(!self.boom.get(), "simulated render failure");
        text_block("healthy").into()
    }
}

fn reconcile(
    r: &mut Reconciler<RecordingBackend>,
    old: Option<&Element>,
    new: &Element,
    existing: Option<windows_reactor::ControlId>,
) -> Option<windows_reactor::ControlId> {
    r.reconcile(old, new, existing, Rc::new(|| {}))
}

#[test]
fn panicking_child_on_mount_substitutes_fallback() {
    let boom = Rc::new(Cell::new(true));
    let child = component(
        Boom {
            boom: Rc::clone(&boom),
        },
        (),
    );
    let tree = error_boundary(child, |msg| text_block(format!("fallback: {msg}")).into());

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &tree, None);
    assert!(id.is_some(), "error boundary must mount a fallback");

    let set_texts: Vec<&Op> = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: windows_reactor::Prop::Text,
                    ..
                }
            )
        })
        .collect();
    assert!(
        set_texts.iter().any(|op| matches!(
            op,
            Op::SetProp {
                value: windows_reactor::PropValue::Str(s),
                ..
            } if s.contains("fallback: simulated render failure")
        )),
        "expected fallback text, got {set_texts:?}"
    );
}

#[test]
fn recovery_after_fix_mounts_healthy_child() {
    let boom = Rc::new(Cell::new(true));
    let child_a = component(
        Boom {
            boom: Rc::clone(&boom),
        },
        (),
    );
    let tree_a = error_boundary(child_a, |_| text_block("fallback").into());

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &tree_a, None).unwrap();
    assert_eq!(
        r.debug_logical_node_count(),
        1,
        "fallback state belongs to the mounted boundary node"
    );

    boom.set(false);
    let child_b = component(
        Boom {
            boom: Rc::clone(&boom),
        },
        (),
    );
    let tree_b = error_boundary(child_b, |_| text_block("fallback").into());
    let id = reconcile(&mut r, Some(&tree_a), &tree_b, Some(id)).unwrap();
    assert_eq!(
        r.debug_logical_node_count(),
        2,
        "recovery mounts the healthy component beneath the existing boundary"
    );

    let saw_healthy = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: windows_reactor::Prop::Text,
                value: windows_reactor::PropValue::Str(s),
                ..
            } if s == "healthy"
        )
    });
    assert!(saw_healthy, "expected healthy mount after recovery");

    r.unmount(id);
    assert_eq!(r.debug_logical_node_count(), 0);
}

#[test]
fn nested_boundaries_catch_at_the_nearest_one() {
    let boom = Rc::new(Cell::new(true));
    let child = component(
        Boom {
            boom: Rc::clone(&boom),
        },
        (),
    );

    let inner = error_boundary(child, |_| text_block("inner-fallback").into());
    let outer = error_boundary(inner, |_| text_block("outer-fallback").into());

    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = reconcile(&mut r, None, &outer, None);
    assert_eq!(
        r.debug_logical_node_count(),
        2,
        "both nested boundaries retain independent logical identity"
    );

    let saw_inner = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: windows_reactor::Prop::Text,
                value: windows_reactor::PropValue::Str(s),
                ..
            } if s == "inner-fallback"
        )
    });
    let saw_outer = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: windows_reactor::Prop::Text,
                value: windows_reactor::PropValue::Str(s),
                ..
            } if s == "outer-fallback"
        )
    });
    assert!(saw_inner, "inner boundary must catch");
    assert!(!saw_outer, "outer boundary must not fire");
}
