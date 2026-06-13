use std::rc::Rc;
use std::time::Duration;

use windows_reactor::Element;
use windows_reactor::ElementExt;
use windows_reactor::button;
use windows_reactor::imp::Reconciler;
use windows_reactor::imp::{Op, RecordingBackend};
use windows_reactor::vstack;
use windows_reactor::{
    AnimationConfig, Easing, ImplicitTransitions, LayoutAnimationConfig, ScalarTransition,
    Vector3Axes, Vector3Transition,
};

fn fresh() -> Reconciler<RecordingBackend> {
    Reconciler::new(RecordingBackend::new())
}

fn no_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn implicit_op(ops: &[Op]) -> Option<&ImplicitTransitions> {
    ops.iter().rev().find_map(|op| match op {
        Op::SetImplicitTransitions {
            transitions: Some(t),
            ..
        } => Some(t),
        _ => None,
    })
}

fn layout_op(ops: &[Op]) -> Option<&LayoutAnimationConfig> {
    ops.iter().rev().find_map(|op| match op {
        Op::SetLayoutAnimation {
            config: Some(c), ..
        } => Some(c),
        _ => None,
    })
}

fn property_op(ops: &[Op]) -> Option<&AnimationConfig> {
    ops.iter().rev().find_map(|op| match op {
        Op::RunPropertyAnimation {
            config: Some(c), ..
        } => Some(c),
        _ => None,
    })
}

#[test]
fn with_opacity_transition_emits_set_implicit_transitions() {
    let mut r = fresh();
    let el: Element = button("hi")
        .with_opacity_transition(Duration::from_millis(150))
        .into();
    let _ = r.reconcile(None, &el, None, no_rerender());

    let it = implicit_op(&r.backend.ops).expect("expected SetImplicitTransitions op");
    assert_eq!(
        it.opacity,
        Some(ScalarTransition::new(Duration::from_millis(150)))
    );
    assert!(it.scale.is_none());
    assert!(it.translation.is_none());
}

#[test]
fn with_scale_and_translation_emit_combined_transitions() {
    let mut r = fresh();
    let el: Element = button("hi")
        .with_scale_transition(Duration::from_millis(200))
        .with_translation_transition(Duration::from_millis(100))
        .into();
    let _ = r.reconcile(None, &el, None, no_rerender());

    let it = implicit_op(&r.backend.ops).unwrap();
    assert_eq!(
        it.scale,
        Some(Vector3Transition {
            duration: Duration::from_millis(200),
            components: Vector3Axes::All,
        })
    );
    assert_eq!(
        it.translation,
        Some(Vector3Transition {
            duration: Duration::from_millis(100),
            components: Vector3Axes::All,
        })
    );
}

#[test]
fn no_animation_means_no_set_implicit_transitions_op() {
    let mut r = fresh();
    let el: Element = button("hi").into();
    let _ = r.reconcile(None, &el, None, no_rerender());
    assert!(implicit_op(&r.backend.ops).is_none());
}

#[test]
fn changing_transition_duration_emits_one_update_op() {
    let mut r = fresh();
    let v1: Element = button("hi")
        .with_opacity_transition(Duration::from_millis(100))
        .into();
    let id = r.reconcile(None, &v1, None, no_rerender()).unwrap();
    r.backend.clear_ops();

    let v2: Element = button("hi")
        .with_opacity_transition(Duration::from_millis(250))
        .into();
    let _ = r.reconcile(Some(&v1), &v2, Some(id), no_rerender());

    let count = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::SetImplicitTransitions { .. }))
        .count();
    assert_eq!(count, 1);
    let it = implicit_op(&r.backend.ops).unwrap();
    assert_eq!(it.opacity.unwrap().duration, Duration::from_millis(250));
}

#[test]
fn equal_transitions_in_old_and_new_emit_no_update_op() {
    let mut r = fresh();
    let v1: Element = button("hi")
        .with_opacity_transition(Duration::from_millis(100))
        .into();
    let id = r.reconcile(None, &v1, None, no_rerender()).unwrap();
    r.backend.clear_ops();

    let v2: Element = button("hi")
        .with_opacity_transition(Duration::from_millis(100))
        .into();
    let _ = r.reconcile(Some(&v1), &v2, Some(id), no_rerender());

    let count = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::SetImplicitTransitions { .. }))
        .count();
    assert_eq!(count, 0, "skip path must not re-emit equal transitions");
}

#[test]
fn dropping_transition_clears_with_none() {
    let mut r = fresh();
    let v1: Element = button("hi")
        .with_opacity_transition(Duration::from_millis(100))
        .into();
    let id = r.reconcile(None, &v1, None, no_rerender()).unwrap();
    r.backend.clear_ops();

    let v2: Element = button("hi").into();
    let _ = r.reconcile(Some(&v1), &v2, Some(id), no_rerender());

    let cleared = r
        .backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::SetImplicitTransitions { transitions, .. } => Some(*transitions),
            _ => None,
        })
        .expect("expected a clearing op");
    assert!(cleared.is_none());
}

#[test]
fn with_layout_animation_emits_set_layout_animation() {
    let mut r = fresh();
    let el: Element = button("hi")
        .with_layout_animation(LayoutAnimationConfig::linear(Duration::from_millis(400)))
        .into();
    let _ = r.reconcile(None, &el, None, no_rerender());

    let la = layout_op(&r.backend.ops).expect("expected SetLayoutAnimation");
    assert_eq!(la.duration, Duration::from_millis(400));
    assert!(!la.use_spring);
    assert!(la.animate_offset);
    assert!(!la.animate_size);
}

#[test]
fn layout_animation_spring_preset_sets_flag() {
    let mut r = fresh();
    let cfg = LayoutAnimationConfig::spring().animate_size(true);
    let el: Element = button("hi").with_layout_animation(cfg).into();
    let _ = r.reconcile(None, &el, None, no_rerender());
    let la = layout_op(&r.backend.ops).unwrap();
    assert!(la.use_spring);
    assert!(la.animate_size);
}

#[test]
fn animate_emits_run_property_animation_on_mount() {
    let mut r = fresh();
    let el: Element = button("hi")
        .animate(AnimationConfig::fade_in(Duration::from_millis(200)))
        .into();
    let _ = r.reconcile(None, &el, None, no_rerender());

    let p = property_op(&r.backend.ops).expect("expected RunPropertyAnimation");
    assert_eq!(p.opacity, Some(1.0));
    assert_eq!(p.duration, Duration::from_millis(200));
    assert_eq!(p.easing, Easing::EaseOut);
}

#[test]
fn changing_animation_config_re_runs_on_update() {
    let mut r = fresh();
    let v1: Element = button("hi")
        .animate(AnimationConfig::fade_in(Duration::from_millis(100)))
        .into();
    let id = r.reconcile(None, &v1, None, no_rerender()).unwrap();
    r.backend.clear_ops();

    let v2: Element = button("hi")
        .animate(AnimationConfig::fade_out(Duration::from_millis(100)))
        .into();
    let _ = r.reconcile(Some(&v1), &v2, Some(id), no_rerender());

    let count = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::RunPropertyAnimation { .. }))
        .count();
    assert_eq!(count, 1);
    let p = property_op(&r.backend.ops).unwrap();
    assert_eq!(p.opacity, Some(0.0));
    assert_eq!(p.easing, Easing::EaseIn);
}

#[test]
fn equal_animation_config_does_not_re_run() {
    let mut r = fresh();
    let v1: Element = button("hi")
        .animate(AnimationConfig::fade_in(Duration::from_millis(100)))
        .into();
    let id = r.reconcile(None, &v1, None, no_rerender()).unwrap();
    r.backend.clear_ops();

    let v2: Element = button("hi")
        .animate(AnimationConfig::fade_in(Duration::from_millis(100)))
        .into();
    let _ = r.reconcile(Some(&v1), &v2, Some(id), no_rerender());

    let count = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::RunPropertyAnimation { .. }))
        .count();
    assert_eq!(count, 0);
}

#[test]
fn enter_transition_fires_at_mount_time() {
    let mut r = fresh();
    let el: Element = button("hi")
        .transition(
            Some(AnimationConfig::fade_in(Duration::from_millis(200))),
            None,
        )
        .into();
    let _ = r.reconcile(None, &el, None, no_rerender());

    let p = property_op(&r.backend.ops).expect("enter should produce a run-property-animation op");
    assert_eq!(p.opacity, Some(1.0));
}

#[test]
fn animation_does_not_disturb_theme_bindings_or_layout_props() {
    use windows_reactor::ThemeRef;

    let mut r = fresh();
    let el: Element = button("hi")
        .background(ThemeRef::Accent)
        .with_opacity_transition(Duration::from_millis(120))
        .margin(8.0)
        .into();
    let _ = r.reconcile(None, &el, None, no_rerender());

    let theme = r
        .backend
        .ops
        .iter()
        .any(|op| matches!(op, Op::ApplyThemeBindings { .. }));
    let anim = r
        .backend
        .ops
        .iter()
        .any(|op| matches!(op, Op::SetImplicitTransitions { .. }));
    let margin = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: windows_reactor::Prop::Margin,
                ..
            }
        )
    });
    assert!(theme);
    assert!(anim);
    assert!(margin);
}

#[test]
fn parent_transition_does_not_propagate_to_children() {
    let mut r = fresh();
    let tree: Element = vstack((
        button("a").with_opacity_transition(Duration::from_millis(100)),
        button("b"),
    ))
    .into();
    let _ = r.reconcile(None, &tree, None, no_rerender());

    let count = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::SetImplicitTransitions { .. }))
        .count();
    assert_eq!(
        count, 1,
        "only the explicitly-animated child should get a transition"
    );
}
