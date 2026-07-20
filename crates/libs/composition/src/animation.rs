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

/// A key-frame animation that interpolates a `Vector3` property (such as a
/// visual's `Scale`) through a series of key frames.
#[derive(Clone)]
pub struct Vector3KeyFrameAnimation(pub(crate) bindings::Vector3KeyFrameAnimation);

impl Vector3KeyFrameAnimation {
    /// Inserts a key frame at `progress` (in `0.0..=1.0`) with the given value.
    pub fn insert_key_frame(&self, progress: f32, value: Vector3) {
        self.0.InsertKeyFrame(progress, value).unwrap();
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
