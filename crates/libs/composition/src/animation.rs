use super::*;
use std::time::Duration;
use windows_time::TimeSpan;

fn to_time_span(duration: Duration) -> TimeSpan {
    // Durations too large for a WinRT `TimeSpan` saturate rather than wrap.
    TimeSpan::try_from(duration).unwrap_or(TimeSpan::MAX)
}

/// The base type shared by every composition animation. An [`Animation`] can be
/// turned into one via [`Animation::as_animation`] to start it on a visual with
/// [`Visual::start_animation`](crate::Visual::start_animation).
#[derive(Clone)]
pub struct CompositionAnimation(pub(crate) bindings::CompositionAnimation);

/// An animation that can be started on a visual property.
///
/// This trait is sealed: only the animation types in this crate implement it.
pub trait Animation: Sealed {
    /// Returns this animation as the shared [`CompositionAnimation`] base type.
    fn as_animation(&self) -> CompositionAnimation;
}

/// An easing function that shapes a key frame's interpolation curve.
///
/// Create one with
/// [`Compositor::create_linear_easing_function`](crate::Compositor::create_linear_easing_function)
/// or
/// [`Compositor::create_cubic_bezier_easing_function`](crate::Compositor::create_cubic_bezier_easing_function)
/// and pass it to a key frame with `insert_key_frame_with_easing`.
#[derive(Clone)]
pub struct CompositionEasingFunction(pub(crate) bindings::CompositionEasingFunction);

/// A key-frame animation that interpolates a scalar (`f32`) property (such as a
/// visual's `Opacity`) through a series of key frames.
#[derive(Clone)]
pub struct ScalarKeyFrameAnimation(pub(crate) bindings::ScalarKeyFrameAnimation);

impl ScalarKeyFrameAnimation {
    /// Inserts a key frame at `progress` (in `0.0..=1.0`) that eases to `value`
    /// along `easing`.
    pub fn insert_key_frame_with_easing(
        &self,
        progress: f32,
        value: f32,
        easing: &CompositionEasingFunction,
    ) {
        let animation: bindings::IScalarKeyFrameAnimation = self.0.cast().unwrap();
        animation
            .InsertKeyFrameWithEasingFunction(progress, value, &easing.0)
            .unwrap();
    }

    /// Inserts a key frame at `progress` whose value is the composition
    /// `expression` (for example `"this.FinalValue"`), eased along `easing`.
    pub fn insert_expression_key_frame_with_easing(
        &self,
        progress: f32,
        expression: &str,
        easing: &CompositionEasingFunction,
    ) {
        let animation: bindings::IKeyFrameAnimation = self.0.cast().unwrap();
        animation
            .InsertExpressionKeyFrameWithEasingFunction(progress, expression, &easing.0)
            .unwrap();
    }

    /// Sets how long one iteration of the animation takes.
    pub fn set_duration(&self, duration: Duration) {
        let animation: bindings::IKeyFrameAnimation = self.0.cast().unwrap();
        animation.SetDuration(to_time_span(duration)).unwrap();
    }

    /// Sets the property this animation targets when used as an implicit
    /// animation (for example `"Opacity"`).
    pub fn set_target(&self, target: &str) {
        let animation: bindings::ICompositionAnimation2 = self.0.cast().unwrap();
        animation.SetTarget(target).unwrap();
    }
}

impl Sealed for ScalarKeyFrameAnimation {}

impl Animation for ScalarKeyFrameAnimation {
    fn as_animation(&self) -> CompositionAnimation {
        CompositionAnimation(self.0.cast().unwrap())
    }
}

/// A key-frame animation that interpolates a `Vector3` property (such as a
/// visual's `Scale`) through a series of key frames.
#[derive(Clone)]
pub struct Vector3KeyFrameAnimation(pub(crate) bindings::Vector3KeyFrameAnimation);

impl Vector3KeyFrameAnimation {
    /// Inserts a key frame at `progress` (in `0.0..=1.0`) with the given value.
    pub fn insert_key_frame(&self, progress: f32, value: Vector3) {
        self.0.InsertKeyFrame(progress, value).unwrap();
    }

    /// Inserts a key frame at `progress` that eases to `value` along `easing`.
    pub fn insert_key_frame_with_easing(
        &self,
        progress: f32,
        value: Vector3,
        easing: &CompositionEasingFunction,
    ) {
        let animation: bindings::IVector3KeyFrameAnimation = self.0.cast().unwrap();
        animation
            .InsertKeyFrameWithEasingFunction(progress, value, &easing.0)
            .unwrap();
    }

    /// Inserts a key frame at `progress` whose value is the composition
    /// `expression` (for example `"this.FinalValue"`), eased along `easing`.
    pub fn insert_expression_key_frame_with_easing(
        &self,
        progress: f32,
        expression: &str,
        easing: &CompositionEasingFunction,
    ) {
        let animation: bindings::IKeyFrameAnimation = self.0.cast().unwrap();
        animation
            .InsertExpressionKeyFrameWithEasingFunction(progress, expression, &easing.0)
            .unwrap();
    }

    /// Sets how long one iteration of the animation takes.
    pub fn set_duration(&self, duration: Duration) {
        let animation: bindings::IKeyFrameAnimation = self.0.cast().unwrap();
        animation.SetDuration(to_time_span(duration)).unwrap();
    }

    /// Sets how long to wait before the animation starts.
    pub fn set_delay(&self, delay: Duration) {
        let animation: bindings::IKeyFrameAnimation = self.0.cast().unwrap();
        animation.SetDelayTime(to_time_span(delay)).unwrap();
    }

    /// Sets the property this animation targets when used as an implicit
    /// animation (for example `"Scale"`).
    pub fn set_target(&self, target: &str) {
        let animation: bindings::ICompositionAnimation2 = self.0.cast().unwrap();
        animation.SetTarget(target).unwrap();
    }

    /// Sets the animation to run for a fixed number of iterations.
    pub fn set_iteration_count(&self, count: i32) {
        let animation: bindings::IKeyFrameAnimation = self.0.cast().unwrap();
        animation
            .SetIterationBehavior(bindings::AnimationIterationBehavior::Count)
            .unwrap();
        animation.SetIterationCount(count).unwrap();
    }

    /// Sets the animation to repeat forever.
    pub fn set_iterate_forever(&self) {
        let animation: bindings::IKeyFrameAnimation = self.0.cast().unwrap();
        animation
            .SetIterationBehavior(bindings::AnimationIterationBehavior::Forever)
            .unwrap();
    }
}

impl Sealed for Vector3KeyFrameAnimation {}

impl Animation for Vector3KeyFrameAnimation {
    fn as_animation(&self) -> CompositionAnimation {
        CompositionAnimation(self.0.cast().unwrap())
    }
}

/// A map of property-name -> animation applied to a visual so that changes to
/// those properties animate automatically.
///
/// Create one with
/// [`Compositor::create_implicit_animation_collection`](crate::Compositor::create_implicit_animation_collection),
/// populate it with [`insert`](Self::insert), then attach it via
/// [`Visual::set_implicit_animations`](crate::Visual::set_implicit_animations).
#[derive(Clone)]
pub struct ImplicitAnimationCollection(pub(crate) bindings::ImplicitAnimationCollection);

impl ImplicitAnimationCollection {
    /// Associates `animation` with the property named `target` (for example
    /// `"Opacity"`). The animation should
    /// [target](ScalarKeyFrameAnimation::set_target) the same property.
    pub fn insert(&self, target: &str, animation: &impl Animation) {
        let map: windows_collections::IMap<
            windows_core::HSTRING,
            bindings::ICompositionAnimationBase,
        > = self.0.cast().unwrap();
        let base: bindings::ICompositionAnimationBase = animation.as_animation().0.cast().unwrap();
        map.Insert(&windows_core::HSTRING::from(target), &base)
            .unwrap();
    }
}
