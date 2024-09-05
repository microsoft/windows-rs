// This test validates the transformation of functions returning handles and are wrapped in a Result<Handle>,
// returning the last Win32 error code on failure.
#[test]
fn test() {
    unsafe {
        use windows::{core::*, Win32::Foundation::*, Win32::System::Threading::*};

        // This should succeed, creating a mutex with the given name.
        let result = CreateMutexA(None, false, s!("test"));
        assert!(result.is_ok());

        // This should fail, since the event name collides with that of the mutex.
        let result = CreateEventA(None, true, false, s!("test"));
        let error = result.unwrap_err();
        assert_eq!(error.code(), ERROR_INVALID_HANDLE.into());
    }
}
