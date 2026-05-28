use std::rc::Rc;

use windows_reactor::core::backend::{ControlKind, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::Element;
use windows_reactor::core::element::Expander;
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::dsl::text_block;

fn mount(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, el, None, Rc::new(|| {}));
    r
}

#[test]
fn expander_mounts_with_header_and_collapses_by_default() {
    let el: Element = Expander::new(text_block("hidden")).header("Details").into();
    let r = mount(&el);

    let creates: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::Create {
                    kind: ControlKind::Expander,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(creates.len(), 1);

    let header_set = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp { prop: Prop::Header, value: PropValue::Str(s), .. } if s == "Details")
    });
    assert!(header_set);

    let collapsed_set = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::IsExpanded,
                value: PropValue::Bool(false),
                ..
            }
        )
    });
    assert!(collapsed_set);
}

#[test]
fn expander_update_diffs_is_expanded() {
    let old: Element = Expander::new(text_block("x")).into();
    let new: Element = Expander::new(text_block("x")).expanded(true).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &old, None, Rc::new(|| {})).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let changes: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::IsExpanded,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(changes.len(), 1);
}

#[test]
fn expander_child_is_mounted_as_inner_control() {
    let el: Element = Expander::new(text_block("inner")).into();
    let r = mount(&el);

    let creates: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::Create { .. }))
        .collect();
    assert_eq!(creates.len(), 2, "expected Expander + its child");
}
