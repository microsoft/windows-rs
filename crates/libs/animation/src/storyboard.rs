use super::*;

/// Opaque handle to a point in a storyboard timeline.
#[derive(Clone, Copy)]
pub struct Keyframe(UI_ANIMATION_KEYFRAME);

/// Sequences one or more transitions on variables.
pub struct Storyboard(pub(crate) IUIAnimationStoryboard2);

impl Storyboard {
    /// Adds a transition on a variable and returns a keyframe marking its end.
    pub fn add_transition(&self, variable: &Variable, transition: &Transition) -> Result<Keyframe> {
        unsafe {
            self.0.AddTransition(&variable.0, &transition.0).ok()?;
            let kf = self.0.AddKeyframeAfterTransition(&transition.0)?;
            Ok(Keyframe(kf))
        }
    }

    /// Adds a transition that starts at the specified keyframe.
    pub fn add_transition_at_keyframe(
        &self,
        variable: &Variable,
        transition: &Transition,
        keyframe: Keyframe,
    ) -> Result<()> {
        unsafe {
            self.0
                .AddTransitionAtKeyframe(&variable.0, &transition.0, keyframe.0)
                .ok()
        }
    }

    /// Schedules the storyboard to begin at the specified time.
    pub fn schedule(&self, time: f64) -> Result<()> {
        unsafe { self.0.Schedule(time, None) }.ok()?;
        Ok(())
    }
}
