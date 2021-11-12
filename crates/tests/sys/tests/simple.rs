use windows_sys::{Win32::Foundation::*, Win32::System::Threading::*};

#[test]
fn simple() {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), BOOL(1), BOOL(0), PWSTR(std::ptr::null_mut()));
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);
    }
}
