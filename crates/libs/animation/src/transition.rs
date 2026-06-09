use super::*;

/// A transition that interpolates a variable's value over time.
pub struct Transition(pub(crate) IUIAnimationTransition2);

/// Provides factory methods for creating common transition types.
pub struct TransitionLibrary(IUIAnimationTransitionLibrary2);

impl TransitionLibrary {
    /// Creates a new transition library.
    pub fn new() -> Result<Self> {
        unsafe {
            let mut ptr = core::ptr::null_mut();
            CoCreateInstance(
                &UIAnimationTransitionLibrary2,
                core::ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                &IUIAnimationTransitionLibrary2::IID,
                &mut ptr,
            )
            .ok()?;
            Ok(Self(windows_core::Type::from_abi(ptr)?))
        }
    }

    /// Creates a transition that accelerates then decelerates.
    ///
    /// The sum of `acceleration_ratio` and `deceleration_ratio` must be ≤ 1.0.
    pub fn accelerate_decelerate(
        &self,
        duration: f64,
        final_value: f64,
        acceleration_ratio: f64,
        deceleration_ratio: f64,
    ) -> Result<Transition> {
        unsafe {
            Ok(Transition(self.0.CreateAccelerateDecelerateTransition(
                duration,
                final_value,
                acceleration_ratio,
                deceleration_ratio,
            )?))
        }
    }

    /// Creates a linear transition to the final value.
    pub fn linear(&self, duration: f64, final_value: f64) -> Result<Transition> {
        unsafe {
            Ok(Transition(
                self.0.CreateLinearTransition(duration, final_value)?,
            ))
        }
    }

    /// Creates an instantaneous transition that jumps to the final value.
    pub fn instantaneous(&self, final_value: f64) -> Result<Transition> {
        unsafe {
            Ok(Transition(
                self.0.CreateInstantaneousTransition(final_value)?,
            ))
        }
    }
}
