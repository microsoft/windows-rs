use test_bindgen::bool_event::*;
use windows::Win32::Foundation::{STATUS_SUCCESS, STATUS_TIMEOUT, WAIT_OBJECT_0, WAIT_TIMEOUT};

#[test]
fn test() {
    unsafe {
        // These bool parameters are actually BOOL.
        let event = CreateEventW(None, true, false, None).unwrap();

        // This bool parameter is actually BOOL.
        assert_eq!(WAIT_TIMEOUT, WaitForSingleObjectEx(event, 0, false));

        // This bool parameter is actually BOOLEAN.
        assert_eq!(STATUS_TIMEOUT, NtWaitForSingleObject(event, false, &mut 0));

        let event = CreateEventW(None, true, true, None).unwrap();

        assert_eq!(WAIT_OBJECT_0, WaitForSingleObjectEx(event, 0, false));
        assert_eq!(STATUS_SUCCESS, NtWaitForSingleObject(event, false, &mut 0));
    }
}
