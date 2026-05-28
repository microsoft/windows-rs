use std::time::Duration;

use windows_reactor::core::animation::{
    AnimationConfig, Easing, ImplicitTransitions, LayoutAnimationConfig, ScalarTransition,
    Vector3Axes, Vector3Transition,
};

#[test]
fn implicit_transitions_default_is_empty() {
    let it = ImplicitTransitions::default();
    assert!(it.is_empty());
}

#[test]
fn implicit_transitions_with_opacity_is_not_empty() {
    let it = ImplicitTransitions {
        opacity: Some(ScalarTransition::new(Duration::from_millis(100))),
        ..Default::default()
    };
    assert!(!it.is_empty());
}

#[test]
fn vector3_transition_default_axes_is_all() {
    let v = Vector3Transition::new(Duration::from_millis(200));
    assert_eq!(v.components, Vector3Axes::All);
}

#[test]
fn layout_animation_config_default_is_linear_offset_only() {
    let c = LayoutAnimationConfig::default();
    assert!(!c.use_spring);
    assert!(c.animate_offset);
    assert!(!c.animate_size);
}

#[test]
fn layout_animation_spring_preset_flips_use_spring() {
    let c = LayoutAnimationConfig::spring();
    assert!(c.use_spring);
}

#[test]
fn animation_config_fade_in_targets_opacity_1() {
    let c = AnimationConfig::fade_in(Duration::from_millis(150));
    assert_eq!(c.opacity, Some(1.0));
    assert_eq!(c.easing, Easing::EaseOut);
}
