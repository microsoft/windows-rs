//! Integration tests for windows-animation.
//!
//! These tests exercise the Windows Animation Manager API.

#[cfg(test)]
mod tests {
    use windows_animation::*;

    fn ensure_com_initialized() {
        unsafe {
            windows_core::link!("ole32.dll" "system" fn CoIncrementMTAUsage(pcookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
            let mut cookie = core::ptr::null_mut();
            let _ = CoIncrementMTAUsage(&mut cookie);
        }
    }

    #[test]
    fn create_manager() {
        ensure_com_initialized();
        let _manager = Manager::new().expect("Failed to create Manager");
    }

    #[test]
    fn create_variable() {
        ensure_com_initialized();
        let manager = Manager::new().unwrap();
        let variable = manager.create_variable(42.0).unwrap();
        assert_eq!(variable.value().unwrap(), 42.0);
    }

    #[test]
    fn create_transition_library() {
        ensure_com_initialized();
        let _library = TransitionLibrary::new().expect("Failed to create TransitionLibrary");
    }

    #[test]
    fn accelerate_decelerate_transition() {
        ensure_com_initialized();
        let library = TransitionLibrary::new().unwrap();
        let _transition = library.accelerate_decelerate(1.0, 100.0, 0.2, 0.8).unwrap();
    }

    #[test]
    fn linear_transition() {
        ensure_com_initialized();
        let library = TransitionLibrary::new().unwrap();
        let _transition = library.linear(2.0, 50.0).unwrap();
    }

    #[test]
    fn instantaneous_transition() {
        ensure_com_initialized();
        let library = TransitionLibrary::new().unwrap();
        let _transition = library.instantaneous(99.0).unwrap();
    }

    #[test]
    fn schedule_transition_and_update() {
        ensure_com_initialized();
        let manager = Manager::new().unwrap();
        let variable = manager.create_variable(0.0).unwrap();
        let library = TransitionLibrary::new().unwrap();
        let transition = library.accelerate_decelerate(1.0, 1.0, 0.2, 0.8).unwrap();

        manager
            .schedule_transition(&variable, &transition, 0.0)
            .unwrap();
        manager.update(0.0).unwrap();

        // At t=0 the value should still be at (or very near) the initial value.
        let val = variable.value().unwrap();
        assert!((0.0..=1.0).contains(&val));
    }

    #[test]
    fn storyboard_workflow() {
        ensure_com_initialized();
        let manager = Manager::new().unwrap();
        let variable = manager.create_variable(0.0).unwrap();
        let library = TransitionLibrary::new().unwrap();

        let storyboard = manager.create_storyboard().unwrap();
        let transition = library.accelerate_decelerate(1.0, 180.0, 0.2, 0.8).unwrap();
        let keyframe = storyboard.add_transition(&variable, &transition).unwrap();

        // Add a second transition starting at the keyframe
        let transition2 = library.accelerate_decelerate(1.0, 0.0, 0.2, 0.8).unwrap();
        storyboard
            .add_transition_at_keyframe(&variable, &transition2, keyframe)
            .unwrap();

        storyboard.schedule(0.0).unwrap();
        manager.update(0.5).unwrap();

        let val = variable.value().unwrap();
        assert!((0.0..=180.0).contains(&val));
    }
}
