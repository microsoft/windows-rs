#[cfg(feature = "implement_exclusive")]
pub trait IAnimationDescriptionImpl: Sized {
    fn Animations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPropertyAnimation>>;
    fn StaggerDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn StaggerDelayFactor(&self) -> ::windows::core::Result<f32>;
    fn DelayLimit(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn ZOrder(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnimationDescriptionFactoryImpl: Sized {
    fn CreateInstance(&self, effect: AnimationEffect, target: AnimationEffectTarget) -> ::windows::core::Result<AnimationDescription>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOpacityAnimationImpl: Sized + IPropertyAnimationImpl {
    fn InitialOpacity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn FinalOpacity(&self) -> ::windows::core::Result<f32>;
}
pub trait IPropertyAnimationImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<PropertyAnimationType>;
    fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScaleAnimationImpl: Sized + IPropertyAnimationImpl {
    fn InitialScaleX(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn InitialScaleY(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn FinalScaleX(&self) -> ::windows::core::Result<f32>;
    fn FinalScaleY(&self) -> ::windows::core::Result<f32>;
    fn NormalizedOrigin(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
