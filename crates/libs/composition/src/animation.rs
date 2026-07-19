use crate::bindings;
use crate::sealed::Sealed;
use std::time::Duration;
use windows_core::{Interface, Result};
use windows_numerics::Vector3;

fn to_time_span(duration: Duration) -> bindings::TimeSpan {
    // WinRT TimeSpan is measured in 100-nanosecond ticks.
    bindings::TimeSpan {
        duration: (duration.as_nanos() / 100) as i64,
    }
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
    fn as_animation(&self) -> Result<CompositionAnimation>;
}

/// A key-frame animation that interpolates a `Vector3` property (such as a
/// visual's `Scale`) through a series of key frames.
#[derive(Clone)]
pub struct Vector3KeyFrameAnimation(pub(crate) bindings::Vector3KeyFrameAnimation);

impl Vector3KeyFrameAnimation {
    /// Inserts a key frame at `progress` (in `0.0..=1.0`) with the given value.
    pub fn insert_key_frame(&self, progress: f32, value: Vector3) -> Result<()> {
        self.0.InsertKeyFrame(progress, value)
    }

    /// Sets how long one iteration of the animation takes.
    pub fn set_duration(&self, duration: Duration) -> Result<()> {
        let animation: bindings::IKeyFrameAnimation = self.0.cast()?;
        animation.SetDuration(to_time_span(duration))
    }

    /// Sets how long to wait before the animation starts.
    pub fn set_delay(&self, delay: Duration) -> Result<()> {
        let animation: bindings::IKeyFrameAnimation = self.0.cast()?;
        animation.SetDelayTime(to_time_span(delay))
    }

    /// Sets the animation to run for a fixed number of iterations.
    pub fn set_iteration_count(&self, count: i32) -> Result<()> {
        let animation: bindings::IKeyFrameAnimation = self.0.cast()?;
        animation.SetIterationBehavior(bindings::AnimationIterationBehavior::Count)?;
        animation.SetIterationCount(count)
    }

    /// Sets the animation to repeat forever.
    pub fn set_iterate_forever(&self) -> Result<()> {
        let animation: bindings::IKeyFrameAnimation = self.0.cast()?;
        animation.SetIterationBehavior(bindings::AnimationIterationBehavior::Forever)
    }
}

impl Sealed for Vector3KeyFrameAnimation {}

impl Animation for Vector3KeyFrameAnimation {
    fn as_animation(&self) -> Result<CompositionAnimation> {
        Ok(CompositionAnimation(self.0.cast()?))
    }
}
