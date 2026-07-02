use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::ElementExt;
use windows_reactor::Reconciler;
use windows_reactor::{BrushBinding, ThemeRef, tokens};
use windows_reactor::{Color, Element};
use windows_reactor::{ControlKind, Prop};
use windows_reactor::{button, text_block, text_box};

use windows_reactor::vstack;
fn fresh() -> Reconciler<RecordingBackend> {
    Reconciler::new(RecordingBackend::new())
}

fn no_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

#[test]
fn button_with_theme_background_emits_apply_theme_bindings_op() {
    let mut r = fresh();
    let el: Element = button("Go").background(ThemeRef::Accent).into();

    let id = r.reconcile(None, &el, None, no_rerender()).unwrap();

    let theme_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::ApplyThemeBindings {
                id: oid,
                kind,
                bindings,
                cache_hit,
            } if *oid == id => Some((kind, bindings.clone(), *cache_hit)),
            _ => None,
        })
        .collect();
    assert_eq!(theme_ops.len(), 1, "expected exactly one theme-binding op");
    let (kind, bindings, cache_hit) = &theme_ops[0];
    assert_eq!(**kind, ControlKind::Button);
    assert_eq!(bindings, &vec![(Prop::Background, ThemeRef::Accent)]);
    assert!(!cache_hit, "first observation must miss the cache");
}

#[test]
fn no_theme_bindings_means_no_apply_theme_bindings_op() {
    let mut r = fresh();
    let el: Element = button("Go").background(Color::rgb(255, 0, 0)).into();
    let _ = r.reconcile(None, &el, None, no_rerender());

    let theme_op_count = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::ApplyThemeBindings { .. }))
        .count();
    assert_eq!(theme_op_count, 0);
}

#[test]
fn theme_binding_overrides_direct_brush_when_both_are_set_at_construction() {
    let mut r = fresh();
    let el: Element = button("Go")
        .background(Color::rgb(255, 0, 0))
        .background(ThemeRef::Accent)
        .into();
    let id = r.reconcile(None, &el, None, no_rerender()).unwrap();

    let direct_brush_ops = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::Background,
                    ..
                }
            )
        })
        .count();
    assert_eq!(direct_brush_ops, 0);

    let bindings = r.backend.theme_bindings_of(id);
    assert_eq!(bindings, vec![(Prop::Background, ThemeRef::Accent)]);
}

#[test]
fn direct_brush_overrides_theme_binding_when_set_last() {
    let mut r = fresh();
    let el: Element = button("Go")
        .background(ThemeRef::Accent)
        .background(Color::rgb(0, 255, 0))
        .into();
    let id = r.reconcile(None, &el, None, no_rerender()).unwrap();

    let direct_brush_ops = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::Background,
                    ..
                }
            )
        })
        .count();
    assert_eq!(direct_brush_ops, 1);
    let bindings = r.backend.theme_bindings_of(id);
    assert!(bindings.is_empty());
}

#[test]
fn text_box_direct_border_brush_overrides_theme_binding_when_set_last() {
    let mut r = fresh();
    let el: Element = text_box("x")
        .border_brush(ThemeRef::Accent)
        .border_brush(Color::rgb(0, 255, 0))
        .into();
    let id = r.reconcile(None, &el, None, no_rerender()).unwrap();

    let border_brush_ops = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::BorderBrush,
                    ..
                }
            )
        })
        .count();
    assert_eq!(border_brush_ops, 1);
    let bindings = r.backend.theme_bindings_of(id);
    assert!(bindings.is_empty());
}

#[test]
fn twenty_buttons_with_same_theme_binding_share_one_cache_entry() {
    let mut r = fresh();

    let buttons: Vec<Element> = (0..20)
        .map(|i| button(format!("b{i}")).background(ThemeRef::Accent).into())
        .collect();
    let tree: Element = vstack(buttons).into();

    let _ = r.reconcile(None, &tree, None, no_rerender());

    assert_eq!(
        r.backend.theme_cache_size(),
        1,
        "expected 1 cache entry; got {}",
        r.backend.theme_cache_size()
    );

    let theme_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::ApplyThemeBindings { cache_hit, .. } => Some(*cache_hit),
            _ => None,
        })
        .collect();
    assert_eq!(theme_ops.len(), 20);
    assert!(!theme_ops[0]);
    assert!(
        theme_ops[1..].iter().all(|h| *h),
        "expected every subsequent application to hit the cache"
    );
}

#[test]
fn different_kinds_with_same_binding_are_different_cache_entries() {
    let mut r = fresh();
    let tree: Element = vstack((
        button("b").background(ThemeRef::Accent),
        text_block("t").background(ThemeRef::Accent),
    ))
    .into();
    let _ = r.reconcile(None, &tree, None, no_rerender());
    assert_eq!(r.backend.theme_cache_size(), 2);
}

#[test]
fn binding_set_canonicalized_so_order_doesnt_split_cache() {
    let mut r = fresh();
    let a = button("a")
        .background(ThemeRef::Accent)
        .foreground(ThemeRef::AccentText);
    let b = button("b")
        .foreground(ThemeRef::AccentText)
        .background(ThemeRef::Accent);
    let tree: Element = vstack((a, b)).into();
    let _ = r.reconcile(None, &tree, None, no_rerender());
    assert_eq!(r.backend.theme_cache_size(), 1);
}

#[test]
fn theme_change_clears_cache_and_next_render_misses() {
    let mut r = fresh();
    let tree1: Element = button("a").background(ThemeRef::Accent).into();
    let id = r.reconcile(None, &tree1, None, no_rerender()).unwrap();
    assert_eq!(r.backend.theme_cache_size(), 1);

    r.notify_theme_changed();
    assert_eq!(r.backend.theme_cache_size(), 0);

    let tree2: Element = button("a").background(ThemeRef::Accent).into();
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&tree1), &tree2, Some(id), no_rerender());

    r.backend.clear_ops();
    let new_button: Element = button("c").background(ThemeRef::Accent).into();
    let _ = r.reconcile(None, &new_button, None, no_rerender());
    let theme_op = r
        .backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::ApplyThemeBindings { cache_hit, .. } => Some(*cache_hit),
            _ => None,
        })
        .expect("expected a theme binding op");
    assert!(
        !theme_op,
        "post-theme-change application must miss the cache"
    );
}

#[test]
fn changing_theme_binding_emits_fresh_apply_op() {
    let mut r = fresh();
    let v1: Element = button("a").background(ThemeRef::Accent).into();
    let id = r.reconcile(None, &v1, None, no_rerender()).unwrap();
    r.backend.clear_ops();

    let v2: Element = button("a").background(ThemeRef::CardBackground).into();
    let _ = r.reconcile(Some(&v1), &v2, Some(id), no_rerender());

    let theme_op = r
        .backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::ApplyThemeBindings { bindings, .. } => Some(bindings.clone()),
            _ => None,
        })
        .expect("expected a theme binding op on update");
    assert_eq!(theme_op, vec![(Prop::Background, ThemeRef::CardBackground)]);
}

#[test]
fn dropping_theme_binding_clears_with_empty_bindings_op() {
    let mut r = fresh();
    let v1: Element = button("a").background(ThemeRef::Accent).into();
    let id = r.reconcile(None, &v1, None, no_rerender()).unwrap();
    r.backend.clear_ops();

    let v2: Element = button("a").background(Color::rgb(255, 255, 255)).into();
    let _ = r.reconcile(Some(&v1), &v2, Some(id), no_rerender());

    let theme_op = r
        .backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::ApplyThemeBindings { bindings, .. } => Some(bindings.clone()),
            _ => None,
        })
        .expect("expected a clearing apply-theme-bindings op");
    assert!(
        theme_op.is_empty(),
        "transition theme→brush clears bindings"
    );
}

#[test]
fn tokens_module_drives_dsl() {
    use tokens::*;

    let mut r = fresh();
    let el: Element = text_block("hi").foreground(SecondaryText).into();
    let _ = r.reconcile(None, &el, None, no_rerender());

    let theme_op = r
        .backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::ApplyThemeBindings { bindings, .. } => Some(bindings.clone()),
            _ => None,
        })
        .unwrap();
    assert_eq!(theme_op, vec![(Prop::Foreground, ThemeRef::SecondaryText)]);
}

#[test]
fn background_accepts_brush_directly_without_wrapping() {
    let _: Element = button("hi").background(Color::rgb(1, 2, 3)).into();
}

#[test]
fn background_accepts_theme_ref_directly_without_wrapping() {
    let _: Element = button("hi").background(ThemeRef::Accent).into();
}

#[test]
fn background_accepts_brush_binding_explicitly() {
    let _: Element = button("hi")
        .background(BrushBinding::Theme(ThemeRef::SubtleFill))
        .into();
}
