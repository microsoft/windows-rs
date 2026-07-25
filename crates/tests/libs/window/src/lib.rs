//! Shared Win32 declarations for driving the window under test.
//!
//! The tests avoid depending on the `windows` projection; a couple of one-off
//! `windows-link` declarations are all that is needed to send messages and
//! check handle validity.

pub const WM_SIZE: u32 = 0x0005;
pub const WM_USER: u32 = 0x0400;

windows_link::link!("user32.dll" "system" fn SendMessageW(hwnd: *mut core::ffi::c_void, msg: u32, wparam: usize, lparam: isize) -> isize);
windows_link::link!("user32.dll" "system" fn IsWindow(hwnd: *mut core::ffi::c_void) -> i32);
