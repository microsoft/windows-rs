use super::*;

/// Coordinates variables, transitions, and storyboards.
pub struct Manager(pub(crate) IUIAnimationManager2);

impl Manager {
    /// Creates a new animation manager.
    pub fn new() -> windows_core::Result<Self> {
        unsafe {
            let mut ptr = core::ptr::null_mut();
            CoCreateInstance(
                &UIAnimationManager2,
                core::ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                &<IUIAnimationManager2 as windows_core::Interface>::IID,
                &mut ptr,
            )
            .ok()?;
            Ok(Self(windows_core::Type::from_abi(ptr)?))
        }
    }

    /// Creates a new animation variable with the given initial value.
    pub fn create_variable(&self, initial_value: f64) -> windows_core::Result<Variable> {
        unsafe { Ok(Variable(self.0.CreateAnimationVariable(initial_value)?)) }
    }

    /// Creates a new storyboard for sequencing transitions.
    pub fn create_storyboard(&self) -> windows_core::Result<Storyboard> {
        unsafe { Ok(Storyboard(self.0.CreateStoryboard()?)) }
    }

    /// Advances the animation manager to the specified time.
    pub fn update(&self, time: f64) -> windows_core::Result<()> {
        unsafe { self.0.Update(time, None) }?;
        Ok(())
    }

    /// Schedules a transition on a variable, starting at the specified time.
    pub fn schedule_transition(
        &self,
        variable: &Variable,
        transition: &Transition,
        time: f64,
    ) -> windows_core::Result<()> {
        unsafe { self.0.ScheduleTransition(&variable.0, &transition.0, time) }
    }
}
