use windows_sys::{Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*};

#[test]
fn simple() {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null_mut());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);
    }
}

#[test]
fn types() {
    // Unscoped enums
    let _: WNDCLASS_STYLES = CS_HREDRAW | CS_VREDRAW;
    let _: u32 = CS_HREDRAW | CS_VREDRAW;

    // Constant
    let _: windows_sys::core::HRESULT = E_FAIL;

    // Constant
    let _: NTSTATUS = DBG_APP_NOT_IDLE;
    let _: i32 = DBG_APP_NOT_IDLE;

    // Handles
    let _: HANDLE = 0;
    let _: PSTR = b"hello\0".as_ptr() as _;
}

#[test]
fn callback() {
    unsafe {
        extern "system" fn enum_window(_: isize, _: isize) -> i32 {
            0
        }

        EnumWindows(Some(enum_window), 0);

        extern "system" fn wndproc(_: isize, _: u32, _: usize, _: isize) -> isize {
            0
        }

        let mut wc: WNDCLASSA = std::mem::zeroed();
        wc.lpfnWndProc = None;
        wc.lpfnWndProc = Some(wndproc);
    }
}
