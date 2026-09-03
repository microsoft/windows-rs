//! Shared Win32 declarations for driving the window under test.
//!
//! The tests avoid depending on the `windows` projection; a couple of one-off
//! `windows-link` declarations are all that is needed to send messages and
//! check handle validity.

pub const WM_SIZE: u32 = 0x0005;
pub const WM_USER: u32 = 0x0400;
pub const GWL_EXSTYLE: i32 = -20;
pub const WS_EX_NOREDIRECTIONBITMAP: isize = 0x0020_0000;

windows_link::link!("user32.dll" "system" fn SendMessageW(hwnd: *mut core::ffi::c_void, msg: u32, wparam: usize, lparam: isize) -> isize);
windows_link::link!("user32.dll" "system" fn IsWindow(hwnd: *mut core::ffi::c_void) -> i32);

#[cfg(target_pointer_width = "64")]
windows_link::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd: *mut core::ffi::c_void, index: i32) -> isize);
#[cfg(target_pointer_width = "32")]
windows_link::link!("user32.dll" "system" fn GetWindowLongW(hwnd: *mut core::ffi::c_void, index: i32) -> i32);

pub fn get_window_ex_style(window: &windows_window::Window) -> isize {
    #[cfg(target_pointer_width = "64")]
    unsafe {
        GetWindowLongPtrW(window.hwnd(), GWL_EXSTYLE)
    }
    #[cfg(target_pointer_width = "32")]
    unsafe {
        GetWindowLongW(window.hwnd(), GWL_EXSTYLE) as isize
    }
}
