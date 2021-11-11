#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ConditionForceEffect();
    fn ConditionForceEffectKind();
    fn ConstantForceEffect();
    fn ForceFeedbackEffectAxes();
    fn ForceFeedbackEffectState();
    fn ForceFeedbackLoadEffectResult();
    fn ForceFeedbackMotor();
    fn IConditionForceEffect();
    fn IConditionForceEffectFactory();
    fn IConstantForceEffect();
    fn IForceFeedbackEffect();
    fn IForceFeedbackMotor();
    fn IPeriodicForceEffect();
    fn IPeriodicForceEffectFactory();
    fn IRampForceEffect();
    fn PeriodicForceEffect();
    fn PeriodicForceEffectKind();
    fn RampForceEffect();
}
