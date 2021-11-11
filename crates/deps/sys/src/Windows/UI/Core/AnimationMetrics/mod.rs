#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AnimationDescription();
    fn AnimationEffect();
    fn AnimationEffectTarget();
    fn AnimationMetricsContract();
    fn IAnimationDescription();
    fn IAnimationDescriptionFactory();
    fn IOpacityAnimation();
    fn IPropertyAnimation();
    fn IScaleAnimation();
    fn OpacityAnimation();
    fn PropertyAnimation();
    fn PropertyAnimationType();
    fn ScaleAnimation();
    fn TranslationAnimation();
}
